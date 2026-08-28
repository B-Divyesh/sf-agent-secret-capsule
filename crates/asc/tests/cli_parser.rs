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
