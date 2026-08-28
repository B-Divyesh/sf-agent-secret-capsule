# Independent verification 2 — PASS

**Candidate:** `5a4b212ebdc2f82569b998c98ecfc9386ba5a3b3`
**Repository/branch:** `B-Divyesh/sf-agent-secret-capsule`, `main`
**Live URL:** https://agent-secret-capsule.sociobot.in/
**Verification date:** 2026-08-28 UTC

## Decision

**PASS.** The prior release-blocking Clap parser failure is repaired in the candidate, package consumer, and live deployment. No release-blocking defects were found.

## Clean-checkout gates

A fresh detached worktree at the candidate SHA was used (`/tmp/asc-qa-SFXTEu`).

| Check | Result |
| --- | --- |
| `npm ci` | Passed; 60 packages audited, 0 vulnerabilities |
| `npm test` | Passed: 9 Rust tests, 5 Vitest tests, 14 Playwright tests across desktop and 390×844 mobile |
| `cargo fmt --check` | Passed |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed |
| `npm run build` | Passed; produced `target/release/asc` and `dist/site` |
| `cargo package -p agent-secret-capsule --allow-dirty` | Passed and verified: 8 files, 84.8 KiB unpacked / 24.0 KiB compressed |
| Clean package consumer | `cargo install --path target/package/agent-secret-capsule-0.1.0 --root /tmp/asc-consumer-Aaf4DI --locked` passed; installed `asc --version` is `0.1.0` |

There is no separate JavaScript lint/type-check script or TypeScript project configuration in the repository; the available strict static check is Clippy.

## CLI and security exercise

The clean consumer's `--help`, `--version`, `doctor --json`, `list --json`, and `receipts --json` completed normally. `doctor` reports telemetry off. Valid aliases and invalid aliases, invalid environment names, TTL `0s`/`61m`, and receipt limits `0`/`1001` were exercised. Invalid cases returned Clap usage errors with exit 2; no output contained a panic or the previous `Mismatch between definition and access` failure.

For the operational paths, `put smoke --stdin`, `run smoke --env TOKEN -- true`, and `remove smoke` reached normal JSON keychain errors with exit 3 in this headless Linux container. A synthetic credential was absent from all captured output. The container has no unlocked Secret Service/D-Bus session, so a real OS-keychain `put → run → remove` smoke test could not be completed here. This is an environment limitation, not a candidate failure: the release binary's black-box parser tests passed, and the Rust suite completed the documented subprocess normal case, raw/percent/base64/base64url/hex redaction coverage, lease expiry/process-group shutdown, no-value receipts, and 100 controlled successful secret-bearing invocations without disclosure.

Source review found no CLI network client or telemetry endpoint. Secrets are only passed via the selected child environment; captured stdout/stderr is held until redaction, local data uses a private directory and no-value receipt log, and the README warns that an authorized command can exfiltrate a credential by other means.

## Live deployment, privacy, and browser QA

The live deployment is the candidate, not merely a similarly functioning build. SHA-256 matched between fresh local production artifacts and production:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `0708ad00ea557fc7387d88f7c65c55f1426eb7727c600b31602a687406577a56` |
| `assets/main-mO-9Whj3.js` | `f296cc719013838357ca8974c52f3a58dd83d58eea6183201302146cc773be80` |
| `assets/style-Dr0HkjDk.css` | `8e09117f8dafa24aab63bfae2fb4f646b8a23a653f90c857a7a9be0e132162df` |
| `assets/generated/capsule-concrete-960.webp` | `104225da2bc0275c49d3048444e3bf17f62128c3ad6a38cc9947b0d40e3d00f2` |

Fresh live Chromium checks covered desktop 1366×900 and mobile 390×844. `/`, `/privacy/`, and `/terms/` have the expected titles, `lang=en`, one `h1`, and one `main`; axe found zero serious/critical violations on all three. There were no console errors or page errors. Normal first-load requests went only to the product origin; no analytics, third-party scripts, or CDN fonts were requested.

Keyboard use reached the skip link and moved focus to `#main`; the primary action was operable with Space. Its visible focus outline is the specified moss `rgb(184, 217, 87)` at 3 px. The 390 px view has no horizontal overflow and the tested primary target is 200×51 px. The demo output replaced the fake credential with `[REDACTED:ASC]`. Under reduced motion it redacted immediately (computed transition `0.00001s`). The service worker was active with no waiting update; an offline reload from its cached shell kept `main` usable.

An invalid license recovery returned the clear quiet error state and made only the expected Sociobot verification request. The verification API returned `200 {"valid":false,"reason":"invalid","expires_at":null}`, exact-origin CORS, and `Cache-Control: no-store`.

Production HTTP policies are present: HSTS, `nosniff`, strict-origin referrer policy, camera/microphone/geolocation denial, and a self-only CSP with only the documented Sociobot license APIs in `connect-src`/`form-action`. Documents use `public, max-age=0, must-revalidate`; hash-named `/assets/*` use `public, max-age=31536000, immutable`; and `sw.js` uses `no-cache`.

## Performance

The fresh build's main JS is 3.91 KiB, CSS 12.02 KiB, total self-hosted fonts 36.1 KiB, and the mobile hero is 84.7 KiB (1440 asset 190.6 KiB), all within the stated budgets. Live mobile Lighthouse (Chromium, 2026-08-28 UTC) scored **100 performance, 100 accessibility, 100 best practices, and 92 SEO**: FCP 0.8 s, LCP 1.5 s, speed index 0.9 s, TBT 40 ms, CLS 0, 132 KiB transfer, and 8 requests.

## Defects by severity

No critical, high, medium, or low candidate defects found.

### Environment limitation (not a product defect)

This disposable Linux verifier has no unlocked Secret Service session, so the native-keychain persistence portion of `put → run → remove` could only be observed through its documented operational error path. Repeat that smoke test on an unlocked Linux/macOS/Windows target before distributing binaries.
