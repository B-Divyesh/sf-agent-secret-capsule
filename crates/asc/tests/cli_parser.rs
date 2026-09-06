//! Black-box regression tests for the installed command-line surface.
//!
//! These invoke Cargo's compiled `asc` binary rather than calling command
//! functions directly. A target without an unlocked OS keychain is expected to
//! return an operational error, but valid arguments must always get that far
//! without Clap panicking.

use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(target_os = "linux")]
use std::{io, os::unix::process::CommandExt};

fn temporary_root(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after the Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("asc-cli-{label}-{}-{nonce}", std::process::id()))
}

fn asc(root: &PathBuf) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_asc"));
    command
        .env("ASC_HOME", root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

fn output_text(output: &Output) -> String {
    format!(
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn assert_not_panic(output: &Output) {
    let text = output_text(output);
    assert!(
        !text.contains("panicked at") && !text.contains("Mismatch between definition and access"),
        "CLI parser panicked:\n{text}"
    );
}

fn assert_json_result(output: &Output) {
    let text = String::from_utf8_lossy(&output.stdout);
    let value: serde_json::Value = serde_json::from_str(text.trim())
        .unwrap_or_else(|error| panic!("expected JSON output ({error}): {}", output_text(output)));
    assert!(
        value
            .get("ok")
            .and_then(serde_json::Value::as_bool)
            .is_some()
    );
}

#[cfg(target_os = "linux")]
fn stop_process_if_it_opens_a_socket(command: &mut Command) {
    // A Secret Service lookup and an outbound telemetry request both need to
    // create a socket on Linux. This seccomp filter turns either attempt into
    // SIGSYS, so a successful command is observable evidence that neither path
    // ran. The filter is installed immediately before exec and remains active
    // in the release-profile binary.
    unsafe {
        command.pre_exec(|| {
            const BPF_LD_W_ABS: u16 = 0x20;
            const BPF_JMP_JEQ_K: u16 = 0x15;
            const BPF_RET_K: u16 = 0x06;
            const SECCOMP_RET_TRAP: u32 = 0x0003_0000;
            const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;

            let mut filter = [
                libc::sock_filter {
                    code: BPF_LD_W_ABS,
                    jt: 0,
                    jf: 0,
                    k: 0,
                },
                libc::sock_filter {
                    code: BPF_JMP_JEQ_K,
                    jt: 0,
                    jf: 1,
                    k: libc::SYS_socket as u32,
                },
                libc::sock_filter {
                    code: BPF_RET_K,
                    jt: 0,
                    jf: 0,
                    k: SECCOMP_RET_TRAP,
                },
                libc::sock_filter {
                    code: BPF_RET_K,
                    jt: 0,
                    jf: 0,
                    k: SECCOMP_RET_ALLOW,
                },
            ];
            let program = libc::sock_fprog {
                len: filter.len() as u16,
                filter: filter.as_mut_ptr(),
            };

            // SAFETY: prctl receives the documented scalar values and a valid
            // pointer to `program`, which remains alive for the syscall.
            if libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) != 0 {
                return Err(io::Error::last_os_error());
            }
            if libc::prctl(
                libc::PR_SET_SECCOMP,
                libc::SECCOMP_MODE_FILTER,
                &program as *const libc::sock_fprog,
            ) != 0
            {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        });
    }
}

#[cfg(not(target_os = "linux"))]
fn stop_process_if_it_opens_a_socket(_command: &mut Command) {}

#[test]
fn valid_alias_commands_reach_their_operational_json_paths_without_panicking() {
    let root = temporary_root("valid");
    let alias = format!("smoke-{}", std::process::id());
    let secret = "valid-secret-123";

    let mut put = asc(&root);
    let mut child = put
        .args(["--json", "put", &alias, "--stdin"])
        .stdin(Stdio::piped())
        .spawn()
        .expect("put process should start");
    child
        .stdin
        .take()
        .expect("put stdin should be available")
        .write_all(secret.as_bytes())
        .expect("secret should be written to stdin");
    let put_output = child.wait_with_output().expect("put should finish");
    assert_not_panic(&put_output);
    assert_json_result(&put_output);
    assert!(matches!(put_output.status.code(), Some(0 | 3)));

    let run_output = asc(&root)
        .args([
            "--json",
            "run",
            &alias,
            "--env",
            "TOKEN",
            "--",
            "asc-command-that-does-not-exist",
        ])
        .output()
        .expect("run process should finish");
    assert_not_panic(&run_output);
    assert_json_result(&run_output);
    assert!(matches!(run_output.status.code(), Some(0 | 3 | 4)));
    assert!(!output_text(&run_output).contains(secret));

    let remove_output = asc(&root)
        .args(["--json", "remove", &alias])
        .output()
        .expect("remove process should finish");
    assert_not_panic(&remove_output);
    assert_json_result(&remove_output);
    assert!(matches!(remove_output.status.code(), Some(0 | 3)));

    let _ = fs::remove_dir_all(root);
}

#[test]
fn invalid_alias_and_environment_errors_are_usage_errors_not_panics() {
    let root = temporary_root("invalid");
    let cases: &[&[&str]] = &[
        &["put", "bad/name", "--stdin"],
        &["run", "bad/name", "--env", "TOKEN", "--", "true"],
        &["remove", "bad/name"],
        &["run", "valid-name", "--env", "NOT-VALID", "--", "true"],
    ];

    for arguments in cases {
        let output = asc(&root)
            .args(*arguments)
            .output()
            .expect("CLI process should finish");
        assert_not_panic(&output);
        assert_eq!(output.status.code(), Some(2), "{}", output_text(&output));
        assert!(output_text(&output).contains("error:"));
    }

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(target_os = "linux")]
fn release_binary_fails_closed_when_the_platform_credential_store_is_unavailable() {
    let root = temporary_root("unavailable-platform-store");
    let test_store = root.join("test-store-must-not-exist");
    let alias = format!("release-store-{}", std::process::id());
    let secret = "release-only-secret-7391";

    let mut command = asc(&root);
    let mut child = command
        .env(
            "DBUS_SESSION_BUS_ADDRESS",
            "unix:path=/tmp/asc-deliberately-missing-secret-service",
        )
        .env("ASC_TEST_KEYRING_DIR", &test_store)
        .args(["--json", "put", &alias, "--stdin"])
        .stdin(Stdio::piped())
        .spawn()
        .expect("release-configured put should start");
    child
        .stdin
        .take()
        .expect("put stdin should be available")
        .write_all(secret.as_bytes())
        .expect("test credential should be written");
    let output = child.wait_with_output().expect("put should finish");

    assert_eq!(output.status.code(), Some(3), "{}", output_text(&output));
    assert_json_result(&output);
    assert_not_panic(&output);
    assert!(!output_text(&output).contains(secret));
    assert!(
        !test_store.exists(),
        "a default-feature binary must not activate the test-only file store"
    );
    assert!(
        !root.join("secrets.json").exists(),
        "a failed platform-store write must not leave alias metadata"
    );

    let _ = fs::remove_dir_all(root);
}

/// @claim:cli-doctor-privacy
#[test]
fn claim_release_doctor_reads_no_credential_and_opens_no_network_socket() {
    let root = temporary_root("doctor-privacy");
    let credential_backend = root.join("credential-backend-must-not-exist");
    let secret = "doctor-must-not-read-this-secret-4982";
    fs::create_dir_all(&root).expect("doctor sandbox should be created");
    fs::write(root.join("secret-canary"), secret).expect("secret canary should be written");

    let mut command = asc(&root);
    command
        .env(
            "DBUS_SESSION_BUS_ADDRESS",
            format!("unix:path={}", credential_backend.display()),
        )
        .env("ASC_TEST_KEYRING_DIR", &credential_backend)
        .args(["--json", "doctor"]);
    stop_process_if_it_opens_a_socket(&mut command);
    let output = command.output().expect("release doctor should finish");

    assert!(output.status.success(), "{}", output_text(&output));
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
            panic!(
                "doctor should return JSON ({error}): {}",
                output_text(&output)
            )
        });
    assert_eq!(report["ok"], true);
    assert_eq!(report["telemetry"], false);
    assert_eq!(report["data_dir"], root.to_string_lossy().as_ref());
    assert!(!output_text(&output).contains(secret));

    let mut human_command = asc(&root);
    human_command
        .env(
            "DBUS_SESSION_BUS_ADDRESS",
            format!("unix:path={}", credential_backend.display()),
        )
        .env("ASC_TEST_KEYRING_DIR", &credential_backend)
        .arg("doctor");
    stop_process_if_it_opens_a_socket(&mut human_command);
    let human = human_command
        .output()
        .expect("human release doctor should finish");
    assert!(human.status.success(), "{}", output_text(&human));
    assert!(String::from_utf8_lossy(&human.stdout).contains("telemetry: off"));
    assert!(!output_text(&human).contains(secret));

    assert!(
        !credential_backend.exists(),
        "doctor must not access the credential backend"
    );
    assert_eq!(
        fs::read_to_string(root.join("secret-canary"))
            .expect("secret canary should remain readable"),
        secret
    );

    let _ = fs::remove_dir_all(root);
}
