# Give one agent command a temporary credential — verification 5

**Verdict: PASS.** There are **0 findings** and **0 untested public claims**.

- Implementation candidate reviewed: `3559a6eeb6451347700b9309e040419a48135a94`
- Documentation baseline reviewed: `815bb289e89271a14d054e29f1affc3c40b8a371`
- Live URL: <https://agent-secret-capsule.sociobot.in/>
- Verification date: 2026-09-06 UTC
- Finding count: **0**
- Untested claim count: **0**

The only change from the implementation candidate to the documentation baseline
is `.factory/handoff.md`. A clean build at the baseline produced byte-for-byte
matches with live Home, Demo, Privacy, Terms, 404, main JavaScript, and CSS.

## Job, audience, and first action

Fresh browser contexts were opened at 1440×900 and 390×844 before scrolling.
The job is “Give one agent command a temporary credential.” The audience is
developers running coding agents. The first action is **Try it with sample
data**; its adjacent result says it will show a fake credential redacted and a
no-value receipt.

| Viewport | Audience bottom | Action bottom | Facts bottom | Viewport height |
| --- | ---: | ---: | ---: | ---: |
| Desktop | 657 px | 736 px | 803 px | 900 px |
| Phone | 471 px | 543 px | 673 px | 844 px |

The title is “Agent Secret Capsule — give one command a credential.” The
headline names the job and the visible first screen uses plain operational
language.

## Review 8 repair and installed CLI check

`cli-doctor-privacy` is now declared in `.factory/claims.json` and has exactly
one tagged release-binary test. The declared command passed in a clean checkout:

```sh
cargo test --release --locked --test cli_parser \
  claim_release_doctor_reads_no_credential_and_opens_no_network_socket
```

The test runs the normal default-feature release executable in a new `ASC_HOME`
directory. On Linux it installs a seccomp rule immediately before execution
that traps `socket(2)`. Both JSON and human `doctor` output succeeded, reporting
telemetry disabled. The unavailable D-Bus credential-backend path stayed absent,
and the fixture canary was neither disclosed nor changed. A Secret Service lookup
or an outbound request would need socket creation in this fresh process and
would have terminated it.

A separate consumer prefix installed the release artifact with `cargo install
--path crates/asc --root <fresh-prefix> --locked`. Its `asc --json doctor`
reported `telemetry: false`; its bundled `asc --json demo` returned the realistic
healthy `api-gateway` sample, redacted both streams, and wrote only no-value
receipts in a new temporary directory. The consumer run used no real credential.
With a fresh home and an impossible D-Bus/backend path, `doctor` left that
backend path absent. Invalid aliases, 0-second and 61-minute time limits, and a
receipt limit of 1001 returned usage exit code 2 and useful errors.

## Claims from a clean checkout

Remote clone `815bb28` was detached, clean, and installed with `npm ci` (60
packages; zero reported vulnerabilities). Every declared command was run
independently. All 15 claim IDs occur exactly once as tags.

| Claim | Result |
| --- | --- |
| `demo-isolation` | PASS, 2 browser projects |
| `offline-reload` | PASS, 2 browser projects |
| `cli-demo` | PASS, 1 browser project; 1 intentional duplicate-project skip |
| `redaction-forms` | PASS |
| `process-tree` | PASS |
| `captured-output-receipt` | PASS |
| `credential-lifecycle` | PASS |
| `receipt-commands` | PASS |
| `receipt-storage-schema` | PASS |
| `cli-interface` | PASS |
| `cli-doctor-privacy` | PASS, release binary under socket-denying seccomp |
| `demo-parity` | PASS, 1 browser project; 1 intentional duplicate-project skip |
| `license-package` | PASS, 1 browser project; 1 intentional duplicate-project skip |
| `site-privacy` | PASS, 2 browser projects |
| `build-output` | PASS, 2 browser projects |

`npm test` passed from that checkout: 12 Rust tests, 2 Vitest tests, 40
Playwright passes, and 6 intentional duplicate-project skips. `npm run build`
produced `target/release/asc` and `dist/site`. `cargo fmt --check`, strict
Clippy, and `cargo package -p agent-secret-capsule --allow-dirty` also passed;
the package verification compiled the packaged 11-file crate.

## Live site, demo, accessibility, privacy, and routes

`verify-url.sh` passed Home, Demo, Privacy, Terms, and `/404.html`. Each had its
route title, `lang=en`, one h1, main landmark, image alternatives, labeled
controls, and no page or console error. Playwright Axe scans on all five routes
at desktop and phone sizes found zero serious or critical violations. No route
overflowed at 390 px and all visible interactive controls measured at least
44×44 CSS px.

The live Demo started populated with `api-gateway`, redacted stdout and stderr,
an expiry result, and a no-value receipt. “Demo — sample data, nothing is
saved” was visible before and after Reset. Space activated Reset and restored
`READY`; Start for real returned Home and left no `demo:` session-storage key.
A fresh browser context loaded Demo, waited for the service worker, went offline,
and reloaded the populated sample successfully. It had an active worker.

The first keyboard focus was the skip link. Reduced-motion media settings made
hero animation and transition durations `0.00001s`. Fresh visits to Home, Demo,
and Privacy had zero cookies and zero third-party requests. Five crawled link
targets succeeded. The unknown route returned HTTP 404 with the designed
“This page does not exist.” page; that expected status is not a defect.

Live hashed assets have one-year immutable caching and `sw.js` has `no-cache`.
Live response headers include CSP, HSTS, `nosniff`, strict-origin referrer
policy, and camera/microphone/geolocation denial.

## Earlier finding disposition

All earlier review, verification, and polish reports were read. Current
evidence, rather than a closure note alone, supports each disposition.

| Earlier finding group | Current disposition and evidence |
| --- | --- |
| Verification 1 parser safety and cache gap | Fixed: valid and invalid installed-CLI paths reached expected results without panic; hashed assets are immutable and `sw.js` is revalidated. |
| Review 1 B1, B2, B4, B5, B6 and minor M1–M4 | Fixed: first-screen coordinates, populated isolated demo, process-tree wording, no billing, complete metadata/404, focus, literal copy, and phone targets all pass above. |
| Review 1 U01–U57 and Reviews 2–4 claim/copy findings | Fixed or removed: all retained observable promises map to the 15 passing claims; unsupported prompt, platform-success, billing, update, and broad telemetry promises remain absent. |
| Review 3 demo and claim coverage findings | Fixed: browser and CLI samples agree; the sample remains labeled, resettable, isolated, and populated. |
| Review 4 route-focus, build-output, copy, and external-link findings | Fixed: route documents, focused keyboard entry, build artifact claim, plain 404 wording, and explicit GitHub links passed. |
| Review 6 terms and license-copy findings | Fixed: current Terms wording is plain, the MIT source/package claim passes, and no paid-license copy remains. |
| Review 7 native-store wording | Fixed: no native-store success claim remains; unavailable-store behavior fails closed and demo behavior is isolated. |
| Review 8 F-8-1 | Fixed: the new `cli-doctor-privacy` claim covers no credential-backend access and no socket creation for the release binary. |

This CLI has no backend, tenant state, paid tier, or runtime AI feature. Tenant
isolation, server restart persistence, health, and HTTP 429 checks do not apply.

## Evidence

Evidence is in `/work/.evidence/agent-secret-capsule-verify-5/`, including
desktop and phone screenshots, route checks, browser/Axe results, offline and
privacy observations, and live/local hash comparisons.

**Final verdict: PASS — 0 findings and 0 untested public claims.**
