use agent_secret_capsule::{
    LeaseRequest, Receipt, SERVICE, data_dir, ensure_private_dir, parse_env_name,
    parse_secret_name, parse_ttl, run_lease,
};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{self, IsTerminal, Read, Write},
    path::{Path, PathBuf},
    process::ExitCode,
    time::Duration,
};

#[derive(Parser, Debug)]
#[command(
    name = "asc",
    version,
    about = "Give one command a temporary credential with redacted output",
    long_about = "Agent Secret Capsule stores named credentials in your OS keychain. It gives one credential to a selected process and its children until exit or the time limit. It captures and redacts their output, then writes a no-value receipt.\n\nSecurity limit: an authorized command can send the credential over the network or write it to disk. It can also transform the credential or pass it to a child. Use a sandbox for hostile code.",
    after_help = "EXIT CODES:\n  0 success\n  2 usage error\n  3 keychain or local-data error\n  4 command could not start\n  124 lease expired\n  otherwise the child command's exit code"
)]
struct Cli {
    /// Emit machine-readable JSON. For `run`, captured output is included in the object.
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Store or replace a named secret in the OS keychain
    #[command(after_help = "EXAMPLE:\n  printf '%s' \"$TOKEN\" | asc put cloudflare --stdin")]
    Put {
        /// Local alias used by `asc run`
        #[arg(value_parser = parse_secret_name)]
        name: String,
        /// Read the value from stdin. Required when stdin is not a terminal.
        #[arg(long)]
        stdin: bool,
    },
    /// Run one command with an expiring secret environment variable
    #[command(
        after_help = "EXAMPLE:\n  asc run cloudflare --env CLOUDFLARE_API_TOKEN --ttl 30s -- curl --fail https://example.test"
    )]
    Run {
        /// Stored secret alias
        #[arg(value_parser = parse_secret_name)]
        name: String,
        /// Environment variable exposed to the selected subprocess
        #[arg(long, value_parser = parse_env_name)]
        env: String,
        /// Time limit (for example 500ms, 30s, 2m; maximum 60m)
        #[arg(long, default_value = "30s", value_parser = parse_ttl)]
        ttl: Duration,
        /// Program and arguments. Prefix with `--` so program flags pass through unchanged.
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<OsString>,
    },
    /// List local aliases (never values)
    List,
    /// Delete a named secret and its local alias
    Remove {
        #[arg(value_parser = parse_secret_name)]
        name: String,
    },
    /// Show recent no-value command receipts
    Receipts {
        /// Maximum number of newest receipts to return
        #[arg(long, default_value_t = 20, value_parser = parse_limit)]
        limit: usize,
    },
    /// Report local capability and storage paths without reading any secret
    Doctor,
    /// Run bundled sample data in a new temporary directory (never uses your keychain)
    #[command(
        after_help = "EXAMPLE:\n  asc demo\n\nThe demo creates a new temporary directory for its receipts and removes no real ASC data."
    )]
    Demo,
}

fn parse_limit(value: &str) -> Result<usize, String> {
    let limit = value
        .parse::<usize>()
        .map_err(|_| "limit must be a whole number".to_string())?;
    if !(1..=1000).contains(&limit) {
        return Err("limit must be between 1 and 1000".into());
    }
    Ok(limit)
}

#[derive(Serialize)]
struct ErrorOutput<'a> {
    ok: bool,
    error: &'a str,
}

#[derive(Serialize)]
struct RunOutput<'a> {
    ok: bool,
    stdout: &'a str,
    stderr: &'a str,
    receipt: &'a Receipt,
}

fn print_error(json: bool, message: &str) {
    if json {
        println!(
            "{}",
            serde_json::to_string(&ErrorOutput {
                ok: false,
                error: message
            })
            .unwrap_or_else(|_| "{\"ok\":false,\"error\":\"serialization failure\"}".into())
        );
    } else {
        eprintln!("asc: {message}");
    }
}

fn metadata_path(root: &Path) -> PathBuf {
    root.join("secrets.json")
}

fn receipts_path(root: &Path) -> PathBuf {
    root.join("receipts.jsonl")
}

fn load_names(root: &Path) -> Result<Vec<String>, String> {
    let path = metadata_path(root);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let bytes =
        fs::read(&path).map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let mut names: Vec<String> = serde_json::from_slice(&bytes)
        .map_err(|error| format!("could not parse {}: {error}", path.display()))?;
    names.sort();
    names.dedup();
    Ok(names)
}

fn secure_file(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|error| format!("could not secure {}: {error}", path.display()))?;
    }
    Ok(())
}

fn save_names(root: &Path, names: &[String]) -> Result<(), String> {
    ensure_private_dir(root)?;
    let path = metadata_path(root);
    let temporary = root.join("secrets.json.new");
    let bytes = serde_json::to_vec_pretty(names)
        .map_err(|error| format!("could not serialize aliases: {error}"))?;
    fs::write(&temporary, bytes)
        .map_err(|error| format!("could not write {}: {error}", temporary.display()))?;
    secure_file(&temporary)?;
    fs::rename(&temporary, &path)
        .map_err(|error| format!("could not replace {}: {error}", path.display()))?;
    Ok(())
}

fn append_receipt(root: &Path, receipt: &Receipt) -> Result<(), String> {
    ensure_private_dir(root)?;
    let path = receipts_path(root);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| format!("could not open {}: {error}", path.display()))?;
    secure_file(&path)?;
    serde_json::to_writer(&mut file, receipt)
        .map_err(|error| format!("could not write receipt: {error}"))?;
    file.write_all(b"\n")
        .map_err(|error| format!("could not finish receipt: {error}"))
}

fn load_receipts(root: &Path, limit: usize) -> Result<Vec<Receipt>, String> {
    let path = receipts_path(root);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let mut receipts = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<Receipt>(line)
                .map_err(|error| format!("could not parse receipt log: {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let keep_from = receipts.len().saturating_sub(limit);
    receipts.drain(0..keep_from);
    receipts.reverse();
    Ok(receipts)
}

fn keychain_entry(name: &str) -> Result<keyring::Entry, String> {
    keyring::Entry::new(SERVICE, name)
        .map_err(|error| format!("OS keychain is unavailable: {error}"))
}

fn read_secret(name: &str) -> Result<String, String> {
    keychain_entry(name)?
        .get_password()
        .map_err(|error| format!("could not read '{name}' from the OS keychain: {error}"))
}

fn command_put(name: String, from_stdin: bool, root: &Path, json: bool) -> Result<(), String> {
    let value = if from_stdin {
        let mut value = String::new();
        io::stdin()
            .read_to_string(&mut value)
            .map_err(|error| format!("could not read stdin: {error}"))?;
        while value.ends_with(['\n', '\r']) {
            value.pop();
        }
        value
    } else {
        if !io::stdin().is_terminal() {
            return Err("stdin is not a terminal; pass --stdin to confirm secret input".into());
        }
        rpassword::prompt_password(format!("Secret value for '{name}': "))
            .map_err(|error| format!("could not read secret: {error}"))?
    };
    if value.len() < 8 {
        return Err("secret must be at least 8 bytes to avoid unsafe over-redaction".into());
    }
    if value.contains('\0') {
        return Err(
            "secret cannot contain a NUL byte because subprocess environments reject it".into(),
        );
    }
    keychain_entry(&name)?
        .set_password(&value)
        .map_err(|error| format!("could not store '{name}' in the OS keychain: {error}"))?;
    let mut names = load_names(root)?;
    if !names.contains(&name) {
        names.push(name.clone());
        names.sort();
    }
    if let Err(error) = save_names(root, &names) {
        let _ = keychain_entry(&name).and_then(|entry| {
            entry
                .delete_credential()
                .map_err(|delete_error| delete_error.to_string())
        });
        return Err(format!(
            "secret was rolled back after metadata failed: {error}"
        ));
    }
    if json {
        println!(
            "{}",
            serde_json::json!({"ok": true, "name": name, "stored": true})
        );
    } else {
        println!("Stored '{name}' in the OS keychain. No value was written locally.");
    }
    Ok(())
}

fn command_remove(name: String, root: &Path, json: bool) -> Result<(), String> {
    keychain_entry(&name)?
        .delete_credential()
        .map_err(|error| format!("could not remove '{name}' from the OS keychain: {error}"))?;
    let mut names = load_names(root)?;
    names.retain(|saved| saved != &name);
    save_names(root, &names)?;
    if json {
        println!(
            "{}",
            serde_json::json!({"ok": true, "name": name, "removed": true})
        );
    } else {
        println!("Removed '{name}' from the OS keychain.");
    }
    Ok(())
}

fn command_list(root: &Path, json: bool) -> Result<(), String> {
    let names = load_names(root)?;
    if json {
        println!("{}", serde_json::json!({"ok": true, "secrets": names}));
    } else if names.is_empty() {
        println!("No secret aliases yet. Add one with: asc put <name>");
    } else {
        println!("Stored aliases (values remain in the OS keychain):");
        for name in names {
            println!("  {name}");
        }
    }
    Ok(())
}

fn command_receipts(root: &Path, limit: usize, json: bool) -> Result<(), String> {
    let receipts = load_receipts(root, limit)?;
    if json {
        println!("{}", serde_json::json!({"ok": true, "receipts": receipts}));
    } else if receipts.is_empty() {
        println!("No receipts yet. A receipt appears after asc run completes or expires.");
    } else {
        println!(
            "TIME                       OUTCOME    SECRET            COMMAND          REDACTED"
        );
        for receipt in receipts {
            println!(
                "{:<26} {:<10} {:<17} {:<16} {}",
                receipt.started_at,
                receipt.outcome,
                receipt.secret_name,
                receipt.command,
                receipt.redactions
            );
        }
    }
    Ok(())
}

fn command_doctor(root: &Path, json: bool) -> Result<(), String> {
    ensure_private_dir(root)?;
    let platform = std::env::consts::OS;
    let backend = match platform {
        "linux" => "Secret Service (D-Bus)",
        "macos" => "macOS Keychain",
        "windows" => "Windows Credential Manager",
        _ => "unsupported platform",
    };
    let supported = matches!(platform, "linux" | "macos" | "windows");
    if json {
        println!(
            "{}",
            serde_json::json!({
                "ok": supported,
                "platform": platform,
                "keychain_backend": backend,
                "data_dir": root,
                "telemetry": false
            })
        );
    } else {
        println!("Agent Secret Capsule doctor");
        println!("  platform:  {platform}");
        println!("  keychain:  {backend}");
        println!("  receipts:  {}", root.display());
        println!("  telemetry: off");
        if platform == "linux" {
            println!("  note:      an unlocked Secret Service session is required");
        }
    }
    if supported {
        Ok(())
    } else {
        Err("this OS has no supported keychain backend".into())
    }
}

fn demo_root() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("asc-demo-{}-{nonce}", std::process::id()))
}

/// Exercise the same lease/redaction/receipt code as `asc run`, but with a
/// bundled fake credential and a new temporary receipt directory.  It never
/// reads the OS keychain or ASC_HOME.
fn command_demo(json: bool) -> Result<(), String> {
    let root = demo_root();
    ensure_private_dir(&root)?;
    let secret = "demo_credential_7Kp9mQ2x";
    let success = LeaseRequest {
        secret_name: "demo-api".into(),
        env_name: "ASC_DEMO_TOKEN".into(),
        command: vec![
            "sh".into(),
            "-c".into(),
            "printf 'stdout credential=%s\\n' \"$ASC_DEMO_TOKEN\"; printf 'stderr credential=%s\\n' \"$ASC_DEMO_TOKEN\" >&2".into(),
        ],
        ttl: Duration::from_secs(2),
    };
    let success_result = run_lease(&success, secret)?;
    append_receipt(&root, &success_result.receipt)?;
    let expiry = LeaseRequest {
        secret_name: "demo-api".into(),
        env_name: "ASC_DEMO_TOKEN".into(),
        command: vec!["sh".into(), "-c".into(), "sleep 1".into()],
        ttl: Duration::from_millis(30),
    };
    let expiry_result = run_lease(&expiry, secret)?;
    append_receipt(&root, &expiry_result.receipt)?;
    fs::write(
        root.join("README.txt"),
        "Agent Secret Capsule demo data. This directory contains only fake sample receipts. Delete this directory to reset the command-line demo.\n",
    )
    .map_err(|error| format!("could not write demo note: {error}"))?;

    if json {
        println!(
            "{}",
            serde_json::json!({
                "ok": true,
                "demo": true,
                "directory": root,
                "stdout": success_result.stdout,
                "stderr": success_result.stderr,
                "receipts": [success_result.receipt, expiry_result.receipt]
            })
        );
    } else {
        print!("{}", success_result.stdout);
        eprint!("{}", success_result.stderr);
        println!("Demo complete. Sample receipts: {}", root.display());
        println!(
            "The fake credential was redacted before output. A second sample reached its 30ms time limit."
        );
        println!(
            "Delete that directory to reset this command-line demo. Your keychain and ASC_HOME were not used."
        );
    }
    Ok(())
}

fn command_run(
    name: String,
    env: String,
    ttl: Duration,
    command: Vec<OsString>,
    root: &Path,
    json: bool,
) -> Result<u8, String> {
    let secret = read_secret(&name)?;
    if secret.len() < 8 {
        return Err("stored secret is shorter than the safe redaction minimum; replace it".into());
    }
    let request = LeaseRequest {
        secret_name: name,
        env_name: env,
        command,
        ttl,
    };
    let result = run_lease(&request, &secret)?;
    append_receipt(root, &result.receipt)?;
    if json {
        println!(
            "{}",
            serde_json::to_string(&RunOutput {
                ok: result.receipt.outcome == "succeeded",
                stdout: &result.stdout,
                stderr: &result.stderr,
                receipt: &result.receipt,
            })
            .map_err(|error| format!("could not serialize result: {error}"))?
        );
    } else {
        io::stdout()
            .write_all(result.stdout.as_bytes())
            .map_err(|error| format!("could not write scrubbed stdout: {error}"))?;
        io::stderr()
            .write_all(result.stderr.as_bytes())
            .map_err(|error| format!("could not write scrubbed stderr: {error}"))?;
        eprintln!(
            "\nasc: {} · {} redaction(s) · receipt {}",
            result.receipt.outcome, result.receipt.redactions, result.receipt.id
        );
    }
    if result.timed_out {
        Ok(124)
    } else {
        Ok(result.receipt.exit_code.unwrap_or(1).clamp(0, 255) as u8)
    }
}

fn run(cli: Cli) -> Result<u8, String> {
    let root = data_dir()?;
    match cli.command {
        Commands::Put { name, stdin } => command_put(name, stdin, &root, cli.json).map(|_| 0),
        Commands::Run {
            name,
            env,
            ttl,
            command,
        } => command_run(name, env, ttl, command, &root, cli.json),
        Commands::List => command_list(&root, cli.json).map(|_| 0),
        Commands::Remove { name } => command_remove(name, &root, cli.json).map(|_| 0),
        Commands::Receipts { limit } => command_receipts(&root, limit, cli.json).map(|_| 0),
        Commands::Doctor => command_doctor(&root, cli.json).map(|_| 0),
        Commands::Demo => command_demo(cli.json).map(|_| 0),
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let json = cli.json;
    match run(cli) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            print_error(json, &error);
            let code = if error.starts_with("could not start command") {
                4
            } else {
                3
            };
            ExitCode::from(code)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temporary_root(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("asc-test-{label}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        path
    }

    #[test]
    fn names_round_trip_without_values() {
        let root = temporary_root("names");
        save_names(&root, &["cloudflare".into(), "github".into()]).unwrap();
        assert_eq!(load_names(&root).unwrap(), ["cloudflare", "github"]);
        let content = fs::read_to_string(metadata_path(&root)).unwrap();
        assert!(!content.contains("token"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn receipt_log_returns_newest_first() {
        let root = temporary_root("receipts");
        for index in 0..3 {
            append_receipt(
                &root,
                &Receipt {
                    id: format!("id-{index}"),
                    started_at: "2026-08-28T00:00:00Z".into(),
                    duration_ms: 1,
                    secret_name: "demo".into(),
                    env_name: "TOKEN".into(),
                    command: "true".into(),
                    outcome: "succeeded".into(),
                    exit_code: Some(0),
                    lease_ms: 1000,
                    redactions: 0,
                },
            )
            .unwrap();
        }
        let loaded = load_receipts(&root, 2).unwrap();
        assert_eq!(loaded[0].id, "id-2");
        assert_eq!(loaded[1].id, "id-1");
        fs::remove_dir_all(root).unwrap();
    }
}
