//! End-to-end claim tests for the compiled `asc` binary.
//!
//! The opt-in feature gives each test a private, disposable credential store.
//! Release builds do not contain that test-only path; normal runs still use the
//! operating system keychain.

#![cfg(feature = "test-keyring")]

use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const SECRET: &str = "scope+value/42";

struct Sandbox {
    root: PathBuf,
    data: PathBuf,
    keychain: PathBuf,
}

impl Sandbox {
    fn new(label: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after the Unix epoch")
            .as_nanos();
        let root =
            std::env::temp_dir().join(format!("asc-claim-{label}-{}-{nonce}", std::process::id()));
        let data = root.join("data");
        let keychain = root.join("isolated-keychain");
        fs::create_dir_all(&root).expect("claim sandbox should be created");
        Self {
            root,
            data,
            keychain,
        }
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_asc"));
        command
            .env("ASC_HOME", &self.data)
            .env("ASC_TEST_KEYRING_DIR", &self.keychain)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command
    }

    fn clean(self) {
        let _ = fs::remove_dir_all(self.root);
    }
}

fn output_text(output: &Output) -> String {
    format!(
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn json(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("expected JSON output ({error}): {}", output_text(output)))
}

fn put(sandbox: &Sandbox, alias: &str, secret: &str) {
    let mut child = sandbox
        .command()
        .args(["--json", "put", alias, "--stdin"])
        .stdin(Stdio::piped())
        .spawn()
        .expect("put command should start");
    child
        .stdin
        .take()
        .expect("put stdin should be available")
        .write_all(secret.as_bytes())
        .expect("test credential should be written");
    let output = child.wait_with_output().expect("put command should finish");
    assert!(output.status.success(), "{}", output_text(&output));
    assert_eq!(json(&output)["stored"], true);
}

fn run_success(sandbox: &Sandbox, alias: &str, script: &str) -> Output {
    let output = sandbox
        .command()
        .args([
            "--json",
            "run",
            alias,
            "--env",
            "DEPLOY_STATUS_TOKEN",
            "--",
            "sh",
            "-c",
            script,
        ])
        .output()
        .expect("run command should finish");
    assert!(output.status.success(), "{}", output_text(&output));
    output
}

/// @claim:credential-lifecycle
#[test]
fn claim_credential_lifecycle_stores_lists_runs_and_removes_an_alias() {
    let sandbox = Sandbox::new("credential-lifecycle");
    let alias = "deploy-status";
    put(&sandbox, alias, SECRET);

    let listed = sandbox
        .command()
        .args(["--json", "list"])
        .output()
        .expect("list command should finish");
    assert!(listed.status.success(), "{}", output_text(&listed));
    assert_eq!(json(&listed)["secrets"], serde_json::json!([alias]));
    assert!(!output_text(&listed).contains(SECRET));

    let run = run_success(
        &sandbox,
        alias,
        "printf 'status token=%s\\n' \"$DEPLOY_STATUS_TOKEN\"; printf 'trace token=%s\\n' \"$DEPLOY_STATUS_TOKEN\" >&2",
    );
    let result = json(&run);
    assert_eq!(result["receipt"]["outcome"], "succeeded");
    assert!(
        result["stdout"]
            .as_str()
            .unwrap_or_default()
            .contains("[REDACTED:ASC]")
    );
    assert!(
        result["stderr"]
            .as_str()
            .unwrap_or_default()
            .contains("[REDACTED:ASC]")
    );

    let removed = sandbox
        .command()
        .args(["--json", "remove", alias])
        .output()
        .expect("remove command should finish");
    assert!(removed.status.success(), "{}", output_text(&removed));
    assert_eq!(json(&removed)["removed"], true);

    let after = sandbox
        .command()
        .args(["--json", "list"])
        .output()
        .expect("list command should finish");
    assert_eq!(json(&after)["secrets"], serde_json::json!([]));
    assert!(
        !fs::read_to_string(sandbox.data.join("secrets.json"))
            .expect("alias metadata should exist")
            .contains(SECRET)
    );
    sandbox.clean();
}

/// @claim:redaction-forms
#[test]
fn claim_redaction_forms_removes_every_named_form_from_compiled_cli_output() {
    let sandbox = Sandbox::new("redaction-forms");
    let alias = "forms";
    put(&sandbox, alias, SECRET);
    let percent = "scope%2Bvalue%2F42";
    let base64 = general_purpose::STANDARD.encode(SECRET);
    let base64url = general_purpose::URL_SAFE_NO_PAD.encode(SECRET);
    let hex = hex::encode(SECRET);
    let forms = [SECRET.to_owned(), percent.into(), base64, base64url, hex];
    let stdout = forms
        .iter()
        .map(|form| format!("printf '%s\\n' 'stdout={form}'"))
        .collect::<Vec<_>>()
        .join("; ");
    let stderr = forms
        .iter()
        .map(|form| format!("printf '%s\\n' 'stderr={form}' >&2"))
        .collect::<Vec<_>>()
        .join("; ");
    let run = run_success(&sandbox, alias, &format!("{stdout}; {stderr}"));
    let result = json(&run);
    let rendered = output_text(&run);
    for form in forms {
        assert!(!rendered.contains(&form), "credential form escaped: {form}");
    }
    assert_eq!(result["receipt"]["redactions"], 10);
    assert_eq!(
        result["stdout"]
            .as_str()
            .unwrap_or_default()
            .matches("[REDACTED:ASC]")
            .count(),
        5
    );
    assert_eq!(
        result["stderr"]
            .as_str()
            .unwrap_or_default()
            .matches("[REDACTED:ASC]")
            .count(),
        5
    );
    sandbox.clean();
}

/// @claim:process-tree
#[test]
#[cfg(unix)]
fn claim_process_tree_uses_the_documented_cli_and_stops_at_its_time_limit() {
    let sandbox = Sandbox::new("process-tree");
    let alias = "tree";
    put(&sandbox, alias, SECRET);
    let started = Instant::now();
    let output = sandbox
        .command()
        .args([
            "--json",
            "run",
            alias,
            "--env",
            "DEPLOY_STATUS_TOKEN",
            "--ttl",
            "80ms",
            "--",
            "sh",
            "-c",
            "(printf 'child=%s\\n' \"$DEPLOY_STATUS_TOKEN\"; sleep 1) & wait",
        ])
        .output()
        .expect("time-limited run should finish");
    assert_eq!(output.status.code(), Some(124), "{}", output_text(&output));
    assert!(started.elapsed() < Duration::from_millis(500));
    let result = json(&output);
    assert_eq!(result["receipt"]["outcome"], "expired");
    assert_eq!(result["receipt"]["lease_ms"], 80);
    assert!(
        result["stdout"]
            .as_str()
            .unwrap_or_default()
            .contains("child=[REDACTED:ASC]")
    );
    assert!(!output_text(&output).contains(SECRET));
    sandbox.clean();
}

/// @claim:captured-output-receipt
#[test]
fn claim_captured_output_and_receipt_omit_the_credential() {
    let sandbox = Sandbox::new("captured-output");
    let alias = "receipt-output";
    put(&sandbox, alias, SECRET);
    let output = run_success(
        &sandbox,
        alias,
        "printf 'stdout=%s\\n' \"$DEPLOY_STATUS_TOKEN\"; printf 'stderr=%s\\n' \"$DEPLOY_STATUS_TOKEN\" >&2",
    );
    let result = json(&output);
    assert_eq!(result["receipt"]["redactions"], 2);
    assert_eq!(result["receipt"]["outcome"], "succeeded");
    let receipt_log = fs::read_to_string(sandbox.data.join("receipts.jsonl"))
        .expect("run should append a receipt");
    assert!(!receipt_log.contains(SECRET));
    assert!(!output_text(&output).contains(SECRET));
    sandbox.clean();
}

/// @claim:receipt-commands
#[test]
fn claim_receipt_commands_return_newest_first_human_and_json_no_value_results() {
    let sandbox = Sandbox::new("receipt-commands");
    let alias = "receipt-command";
    put(&sandbox, alias, SECRET);
    run_success(
        &sandbox,
        alias,
        "printf 'first=%s\\n' \"$DEPLOY_STATUS_TOKEN\"",
    );
    std::thread::sleep(Duration::from_millis(12));
    let failed = sandbox
        .command()
        .args([
            "--json",
            "run",
            alias,
            "--env",
            "DEPLOY_STATUS_TOKEN",
            "--",
            "sh",
            "-c",
            "printf 'second=%s\\n' \"$DEPLOY_STATUS_TOKEN\"; exit 7",
        ])
        .output()
        .expect("failing run should finish");
    assert_eq!(failed.status.code(), Some(7));

    let human = sandbox
        .command()
        .args(["receipts"])
        .output()
        .expect("human receipt command should finish");
    assert!(human.status.success(), "{}", output_text(&human));
    let human_text = output_text(&human);
    assert!(human_text.contains("TIME") && human_text.contains("OUTCOME"));
    assert!(!human_text.contains(SECRET));

    let machine = sandbox
        .command()
        .args(["--json", "receipts"])
        .output()
        .expect("JSON receipt command should finish");
    assert!(machine.status.success(), "{}", output_text(&machine));
    let receipts = json(&machine)["receipts"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    assert_eq!(receipts.len(), 2);
    assert_eq!(receipts[0]["outcome"], "failed");
    assert_eq!(receipts[1]["outcome"], "succeeded");
    assert!(!output_text(&machine).contains(SECRET));
    sandbox.clean();
}

/// @claim:receipt-storage-schema
#[test]
fn claim_receipt_storage_schema_is_private_and_contains_only_declared_metadata() {
    let sandbox = Sandbox::new("receipt-storage-schema");
    let alias = "schema";
    put(&sandbox, alias, SECRET);
    run_success(
        &sandbox,
        alias,
        "printf 'schema=%s\\n' \"$DEPLOY_STATUS_TOKEN\"",
    );

    let receipt_path = sandbox.data.join("receipts.jsonl");
    let record: Value = serde_json::from_str(
        fs::read_to_string(&receipt_path)
            .expect("receipt should be in the configured local data directory")
            .trim(),
    )
    .expect("receipt line should be JSON");
    let mut fields = record
        .as_object()
        .expect("receipt should be an object")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    fields.sort();
    assert_eq!(
        fields,
        [
            "command",
            "duration_ms",
            "env_name",
            "exit_code",
            "id",
            "lease_ms",
            "outcome",
            "redactions",
            "secret_name",
            "started_at"
        ]
    );
    assert_eq!(record["secret_name"], alias);
    assert_eq!(record["env_name"], "DEPLOY_STATUS_TOKEN");
    assert_eq!(record["outcome"], "succeeded");
    assert!(!record.to_string().contains(SECRET));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(&sandbox.data).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            fs::metadata(&receipt_path).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
    sandbox.clean();
}

/// @claim:cli-interface
#[test]
fn claim_cli_interface_help_and_non_tty_input_behave_as_documented() {
    let sandbox = Sandbox::new("cli-interface");
    let root_help = sandbox
        .command()
        .arg("--help")
        .output()
        .expect("root help should run");
    assert!(root_help.status.success());
    assert!(output_text(&root_help).contains("EXIT CODES"));
    for command in ["put", "run", "list", "remove", "receipts", "doctor", "demo"] {
        let output = sandbox
            .command()
            .args([command, "--help"])
            .output()
            .expect("subcommand help should run");
        assert!(
            output.status.success(),
            "{command}: {}",
            output_text(&output)
        );
        assert!(
            output_text(&output).contains("EXAMPLE"),
            "{command} lacks its documented example"
        );
    }
    let non_tty = sandbox
        .command()
        .args(["--json", "put", "non-tty"])
        .output()
        .expect("non-TTY put should finish");
    assert_eq!(non_tty.status.code(), Some(3));
    assert_eq!(json(&non_tty)["ok"], false);
    assert!(
        json(&non_tty)["error"]
            .as_str()
            .unwrap_or_default()
            .contains("pass --stdin")
    );
    sandbox.clean();
}
