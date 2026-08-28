//! Core primitives for Agent Secret Capsule.
//!
//! The public surface is deliberately small: [`redact`] removes a configured
//! secret and its common exact encodings, while [`run_lease`] executes one
//! child process with an expiring environment lease and produces a no-value
//! [`Receipt`].

use base64::{Engine, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    io::{self, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

pub const SERVICE: &str = "in.sociobot.agent-secret-capsule";
pub const REDACTION: &str = "[REDACTED:ASC]";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Receipt {
    pub id: String,
    pub started_at: String,
    pub duration_ms: u128,
    pub secret_name: String,
    pub env_name: String,
    pub command: String,
    pub outcome: String,
    pub exit_code: Option<i32>,
    pub lease_ms: u64,
    pub redactions: usize,
}

#[derive(Debug, Clone)]
pub struct LeaseRequest {
    pub secret_name: String,
    pub env_name: String,
    pub command: Vec<OsString>,
    pub ttl: Duration,
}

#[derive(Debug)]
pub struct LeaseResult {
    pub stdout: String,
    pub stderr: String,
    pub receipt: Receipt,
    pub timed_out: bool,
}

pub fn validate_secret_name(value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > 64 {
        return Err("secret name must contain 1–64 characters".into());
    }
    if !value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err("secret name may contain only letters, numbers, '.', '_' and '-'".into());
    }
    Ok(())
}

pub fn validate_env_name(value: &str) -> Result<(), String> {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return Err("environment variable name cannot be empty".into());
    };
    if !(first == '_' || first.is_ascii_alphabetic())
        || !chars.all(|character| character == '_' || character.is_ascii_alphanumeric())
    {
        return Err("environment variable must match [A-Za-z_][A-Za-z0-9_]*".into());
    }
    Ok(())
}

pub fn parse_ttl(value: &str) -> Result<Duration, String> {
    let split = value
        .find(|character: char| !character.is_ascii_digit())
        .ok_or_else(|| "TTL needs a unit: ms, s, or m".to_string())?;
    let (number, unit) = value.split_at(split);
    let amount: u64 = number
        .parse()
        .map_err(|_| "TTL must start with a positive integer".to_string())?;
    if amount == 0 {
        return Err("TTL must be greater than zero".into());
    }
    let duration = match unit {
        "ms" => Duration::from_millis(amount),
        "s" => Duration::from_secs(amount),
        "m" => Duration::from_secs(amount.saturating_mul(60)),
        _ => return Err("TTL unit must be ms, s, or m".into()),
    };
    if duration > Duration::from_secs(3600) {
        return Err("TTL cannot exceed 60m".into());
    }
    Ok(duration)
}

fn percent_encode(secret: &[u8]) -> String {
    let mut encoded = String::with_capacity(secret.len() * 3);
    for byte in secret {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(*byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

fn variants(secret: &str) -> Vec<String> {
    let bytes = secret.as_bytes();
    let encoded = percent_encode(bytes);
    let mut values = vec![
        secret.to_owned(),
        encoded.clone(),
        encoded.to_ascii_lowercase(),
        general_purpose::STANDARD.encode(bytes),
        general_purpose::STANDARD_NO_PAD.encode(bytes),
        general_purpose::URL_SAFE.encode(bytes),
        general_purpose::URL_SAFE_NO_PAD.encode(bytes),
        hex::encode(bytes),
        hex::encode_upper(bytes),
    ];
    values.sort_by_key(|value| std::cmp::Reverse(value.len()));
    values.dedup();
    values
}

/// Replace the raw secret and common exact encodings with a fixed marker.
///
/// The replacement count is returned for receipt metadata. Very short secrets
/// are rejected by the CLI because broad replacement would corrupt ordinary
/// output.
pub fn redact(input: &str, secret: &str) -> (String, usize) {
    let mut output = input.to_owned();
    let mut count = 0;
    for variant in variants(secret) {
        if variant.is_empty() {
            continue;
        }
        count += output.matches(&variant).count();
        output = output.replace(&variant, REDACTION);
    }
    (output, count)
}

fn read_all<T: Read>(mut stream: T) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn now_rfc3339() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "unknown".into())
}

fn receipt_id() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("asc-{millis}-{}", std::process::id())
}

/// Run one subprocess with a secret present in exactly one added environment
/// variable. Output is held until the process exits so secrets split across IO
/// chunks are still redacted before anything reaches the parent terminal.
pub fn run_lease(request: &LeaseRequest, secret: &str) -> Result<LeaseResult, String> {
    if request.command.is_empty() {
        return Err("a command is required after '--'".into());
    }
    let started_at = now_rfc3339();
    let started = Instant::now();
    let mut command = Command::new(&request.command[0]);
    command
        .args(&request.command[1..])
        .env(&request.env_name, secret)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        // Isolate the lease in a process group so expiry also stops descendants
        // that inherited the environment and captured output pipes.
        command.process_group(0);
    }
    let mut child = command
        .spawn()
        .map_err(|error| format!("could not start command: {error}"))?;

    let stdout = child.stdout.take().ok_or("could not capture stdout")?;
    let stderr = child.stderr.take().ok_or("could not capture stderr")?;
    let stdout_reader = thread::spawn(move || read_all(stdout));
    let stderr_reader = thread::spawn(move || read_all(stderr));
    let mut timed_out = false;

    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("could not inspect command: {error}"))?
        {
            break status;
        }
        if started.elapsed() >= request.ttl {
            timed_out = true;
            #[cfg(unix)]
            // SAFETY: the PID comes from the live child. A negative PID targets
            // only the isolated process group created above.
            unsafe {
                libc::kill(-(child.id() as i32), libc::SIGKILL);
            }
            child.kill().map_err(|error| {
                format!("lease expired but command could not be stopped: {error}")
            })?;
            break child
                .wait()
                .map_err(|error| format!("could not reap expired command: {error}"))?;
        }
        thread::sleep(Duration::from_millis(10));
    };

    let stdout_bytes = stdout_reader
        .join()
        .map_err(|_| "stdout capture thread failed")?
        .map_err(|error| format!("could not read stdout: {error}"))?;
    let stderr_bytes = stderr_reader
        .join()
        .map_err(|_| "stderr capture thread failed")?
        .map_err(|error| format!("could not read stderr: {error}"))?;
    let (stdout, stdout_redactions) = redact(&String::from_utf8_lossy(&stdout_bytes), secret);
    let (stderr, stderr_redactions) = redact(&String::from_utf8_lossy(&stderr_bytes), secret);
    let exit_code = status.code();
    let outcome = if timed_out {
        "expired"
    } else if status.success() {
        "succeeded"
    } else {
        "failed"
    };
    let command = request.command[0].to_string_lossy().into_owned();

    Ok(LeaseResult {
        stdout,
        stderr,
        timed_out,
        receipt: Receipt {
            id: receipt_id(),
            started_at,
            duration_ms: started.elapsed().as_millis(),
            secret_name: request.secret_name.clone(),
            env_name: request.env_name.clone(),
            command,
            outcome: outcome.into(),
            exit_code,
            lease_ms: request.ttl.as_millis().min(u64::MAX as u128) as u64,
            redactions: stdout_redactions + stderr_redactions,
        },
    })
}

pub fn data_dir() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("ASC_HOME") {
        return Ok(PathBuf::from(path));
    }
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
    #[cfg(target_os = "macos")]
    let base = std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join("Library/Application Support"));
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")));
    base.map(|path| path.join("agent-secret-capsule"))
        .ok_or_else(|| "could not locate a user data directory; set ASC_HOME".into())
}

pub fn ensure_private_dir(path: &Path) -> Result<(), String> {
    std::fs::create_dir_all(path)
        .map_err(|error| format!("could not create {}: {error}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
            .map_err(|error| format!("could not secure {}: {error}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_raw_and_encoded_forms() {
        let secret = "tok_en+value/42";
        let text = format!(
            "raw={secret} url={} b64={} hex={}",
            percent_encode(secret.as_bytes()),
            general_purpose::STANDARD.encode(secret),
            hex::encode(secret)
        );
        let (clean, count) = redact(&text, secret);
        assert!(!clean.contains(secret));
        assert_eq!(count, 4);
        assert_eq!(clean.matches(REDACTION).count(), 4);
    }

    #[test]
    fn parses_bounded_ttls() {
        assert_eq!(parse_ttl("250ms").unwrap(), Duration::from_millis(250));
        assert_eq!(parse_ttl("30s").unwrap(), Duration::from_secs(30));
        assert_eq!(parse_ttl("2m").unwrap(), Duration::from_secs(120));
        assert!(parse_ttl("0s").is_err());
        assert!(parse_ttl("61m").is_err());
    }

    #[test]
    #[cfg(unix)]
    fn documented_run_scrubs_output_and_records_no_value() {
        let secret = "capsule-example-987";
        let request = LeaseRequest {
            secret_name: "example".into(),
            env_name: "TOKEN".into(),
            command: vec!["sh".into(), "-c".into(), "printf '%s' \"$TOKEN\"".into()],
            ttl: Duration::from_secs(2),
        };
        let result = run_lease(&request, secret).unwrap();
        assert_eq!(result.stdout, REDACTION);
        assert_eq!(result.receipt.redactions, 1);
        assert!(
            !serde_json::to_string(&result.receipt)
                .unwrap()
                .contains(secret)
        );
    }

    #[test]
    #[cfg(unix)]
    fn expired_lease_stops_command() {
        let request = LeaseRequest {
            secret_name: "example".into(),
            env_name: "TOKEN".into(),
            command: vec!["sh".into(), "-c".into(), "sleep 1".into()],
            ttl: Duration::from_millis(30),
        };
        let started = Instant::now();
        let result = run_lease(&request, "capsule-example-987").unwrap();
        assert!(result.timed_out);
        assert_eq!(result.receipt.outcome, "expired");
        assert!(started.elapsed() < Duration::from_millis(500));
    }

    #[test]
    #[cfg(unix)]
    fn one_hundred_controlled_invocations_do_not_disclose() {
        let secret = "controlled-secret-4a9fe2";
        for _ in 0..100 {
            let request = LeaseRequest {
                secret_name: "controlled".into(),
                env_name: "TOKEN".into(),
                command: vec![
                    "sh".into(),
                    "-c".into(),
                    "printf '%s\\n' \"$TOKEN\"; printf '%s' \"$TOKEN\" >&2".into(),
                ],
                ttl: Duration::from_secs(2),
            };
            let result = run_lease(&request, secret).unwrap();
            assert!(!result.stdout.contains(secret));
            assert!(!result.stderr.contains(secret));
            assert_eq!(result.receipt.outcome, "succeeded");
        }
    }
}
