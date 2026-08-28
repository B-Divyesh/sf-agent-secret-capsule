# Independent verification — FAIL

**Candidate:** `fa3cfbe38edf1d9c02a118272fed2b39d53cbebf`  
**Repository/branch:** `B-Divyesh/sf-agent-secret-capsule`, `main`  
**Live URL tested:** https://agent-secret-capsule.sociobot.in/  
**Verification date:** 2026-08-28 UTC

## Decision

**FAIL — release blocker.** The publishable `asc` binary panics (exit 101) for
every command that accepts a secret alias, including the documented `put`,
`run`, and `remove` flows. A developer therefore cannot store a credential or
lease it to a subprocess, so the brief's smallest useful product does not work
end to end.

## Blocking defect

### Critical — alias-bearing CLI commands panic before doing useful work

Fresh package-consumer reproduction after `cargo package` and a clean
`cargo install`:

```text
$ printf 'valid-secret-123' | asc put smoke --stdin
thread 'main' panicked at .../clap_builder-4.6.6/src/parser/error.rs:32:9:
Mismatch between definition and access of `name`. Could not downcast ...
exit=101

$ asc run smoke --env TOKEN -- true
... same panic ...
exit=101

$ asc remove smoke
... same panic ...
exit=101
```

The problem is present in the candidate, not deployment-only: the `name:
String` Clap fields use `validate_secret_name`, which returns `Result<(),
String>`, at `crates/asc/src/main.rs:39`, `:51`, and `:67`; the validator is
defined at `crates/asc/src/lib.rs:53`. `env: String` uses the same
`Result<(), String>` validator pattern at `main.rs:54`, so a valid `run` would
also be unsafe after the alias parser is corrected. The parser's stored type
does not match the derive target and Clap panics rather than issuing a normal
usage error. Existing tests call command/library functions directly and do not
exercise argument parsing for these paths.

Needed before release: make the argument parsers preserve/return the parsed
`String` while validating it, then add black-box tests for valid `put`, `run`,
`remove`, and their error/JSON paths using the built binary.

## Commands and package evidence

All commands were run from the clean candidate checkout.

| Check | Result |
| --- | --- |
| `npm ci` | Passed; 60 packages audited, 0 vulnerabilities |
| `npm test` | Passed: 7 Rust tests, 3 Vitest tests, 12 Playwright tests (desktop + 390x844) |
| `cargo fmt --check` | Passed |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed |
| `npm run build` | Passed; `target/release/asc` and `dist/site` produced |
| `cargo package -p agent-secret-capsule --allow-dirty` | Passed and verified; 22.7 KiB compressed crate |
| Clean-consumer install | `cargo install --path target/package/agent-secret-capsule-0.1.0 --root /tmp/asc-consumer.bIJdPm --locked` passed; installed binary reports `asc 0.1.0` |

The installed package's non-alias paths work: `doctor --json`, `list --json`,
and `receipts --json` returned valid no-value JSON. Boundary validation also
returned normal exit 2 errors for invalid env names, TTL `0s`/`61m`, and receipt
limits `0`/`1001`. The core alias-bearing normal, invalid, and recovery paths
cannot be completed because of the panic above. The headless container also has
no unlocked Linux Secret Service session, but that is not the cause of this
failure: parsing panics before any keychain call.

## Site, privacy, and deployment evidence

The live deployment **matches the candidate**. SHA-256 values matched for
`index.html` (`0708ad00...`), `assets/main-mO-9Whj3.js`
(`f296cc71...`), `assets/style-Dr0HkjDk.css` (`8e09117f...`), `sw.js`
(`2e96e90a...`), and the 960px WebP hero (`104225da...`). The live index imports
the candidate's hashed JS/CSS assets and the live response was HTTP 200.

Live Playwright checks at desktop and 390x844 mobile/reduced-motion found:

- correct title, one `h1`, and one `main`;
- demo replaces the fake credential with `[REDACTED:ASC]` and never displays
  `capsule_fake_token`;
- no console errors or page errors;
- keyboard skip link reaches `#main`; desktop computed focus ring is a 3px moss
  outline;
- zero axe serious/critical findings;
- normal first-load requests go only to the product origin (no analytics or
  third-party script/font request);
- service worker is active, `registration.update()` completes with no waiting
  worker, and an offline reload succeeds after the shell is cached.

Privacy and policy checks: production CSP restricts scripts/styles/fonts/images
to self and permits connections/forms only to the Sociobot license APIs; HSTS,
`nosniff`, strict-origin referrer policy, and camera/microphone/geolocation
permissions denial are present. An invalid license API response was HTTP 200
with `{ "valid": false, "reason": "invalid" }`, CORS allowed the production
origin, and `Cache-Control: no-store` was present. Source review found no CLI
telemetry or tracking; license data is the documented localStorage exception.

### Medium — static assets are not long-lived immutable in production

The live HTML, JS, CSS, WebP, service worker, and legal pages all return
`Cache-Control: public, must-revalidate, max-age=30`, including hash-named
assets. This misses the factory caching requirement for long-lived immutable
hashed assets and creates avoidable repeat-request overhead. It is not the
reason for the FAIL, but deployment caching should be configured before release.

## Follow-up verification required

After the parser fix, repeat the package-consumer flow on an unlocked target-OS
keychain: `put` a representative credential, `run` a command that writes raw,
percent/base64/base64url/hex forms to both streams, check no-value receipts and
exit propagation, force TTL expiry including a child process, then `remove`.
Retest the advertised 100-command success/no-disclosure measure through the
actual CLI binary rather than only the library test.
