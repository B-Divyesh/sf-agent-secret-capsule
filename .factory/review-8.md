# Give one agent command a temporary credential — review 8

**Verdict: FAIL.** One major finding remains. Two public privacy claims have no
entry in `.factory/claims.json` and no tagged claim test.

- Implementation reviewed: `6ce85df2f4a76d674fa95a08bda005918257cfcb`
- Documentation baseline: `80a7442b06e5a5ce5def4ed21a61564e1901aa95`
- Live URL: <https://agent-secret-capsule.sociobot.in/>
- Review date: 2026-09-06 UTC
- Finding count: **1**
- Untested claim count: **2**

The commits after the implementation candidate change only
`.factory/handoff.md` and `.factory/verification-4.md`. Home, Demo, Privacy,
Terms, the 404 document, all emitted JavaScript and CSS, and the phone hero
matched a clean build of the implementation candidate byte for byte.

The work order named
`factory-evidence/agent-secret-capsule-verify-4/qa-report.md` as authoritative
evidence. That file was absent from the checkout and `/work/.evidence`. The full
repository report `.factory/verification-4.md` was read, and every required
check was repeated independently. This missing attachment did not leave a
product path untested.

## Job, audience, and first action

Fresh Chromium contexts at 1440×900 and 390×844 showed the required information
before scrolling:

| Check | Live result |
| --- | --- |
| Job | “Give one agent command a temporary credential.” |
| Audience | “For developers running coding agents…” |
| First action | **Try it with sample data** |
| Next result | “See a fake credential redacted and a no-value receipt.” |
| Desktop bounds | Audience ends at y=657; action at y=736; all facts at y=803. |
| Phone bounds | Audience ends at y=471; action at y=543; all facts at y=673. |

The page title is “Agent Secret Capsule — give one command a credential.” The
headline names the job. The first screen has no metaphor or mood heading.

## Finding

### F-8-1 — MAJOR — `doctor` makes two unlisted privacy claims

The installed release artifact exposes these statements:

```text
asc --help
  doctor  Report local capability and storage paths without reading any secret

asc --json doctor
  ... "telemetry": false ...
```

A user can rely on both statements: `doctor` does not read a stored credential,
and the CLI performs no telemetry. Neither statement appears in
`.factory/claims.json`, so neither has the required tagged test.

The nearest entries do not cover them:

- `cli-interface` checks that help contains examples and that non-terminal
  `put` requires `--stdin`. Its test only checks that `doctor --help` exits and
  contains `EXAMPLE`; it never runs `doctor`.
- `site-privacy` observes browser requests and cookies. It does not cover the
  installed CLI.
- `cli-demo` proves only that the bundled demo does not use the keychain or
  `ASC_HOME`.

`rg` found no tagged test that invokes `doctor` or verifies CLI network
silence. Source inspection found no HTTP client, and the command succeeded in
this review, so the statements are not shown false. They are still unlisted and
untested under the claims contract.

Required repair: add a `cli-doctor-privacy` manifest entry and one tagged
release-binary test that verifies both promises in a fresh sandbox. It should
prove `doctor` does not access the credential backend and that the CLI makes no
outbound request. Otherwise remove the two statements. Then rerun every claim
command from a clean checkout.

## Demo and real-data isolation

- Home opened `/demo/` in one click. `/?demo=1` also opened the same route.
- The initial view showed `api-gateway` in production, healthy status, redacted
  stdout and stderr, a 30 ms expiry result, and a no-value receipt.
- “Demo — sample data, nothing is saved” remained visible after rerun and
  Reset.
- Rerun created only `sessionStorage["demo:asc:run-count"]`. Reset removed that
  key and restored `READY`.
- Start for real removed the demo key and returned Home. Separate
  `real:sentinel` values in local and session storage survived Reset and Start
  for real.
- The browser sample contained no configured fake value. All observed browser
  requests were same-origin, and fresh contexts had no cookies.
- The installed `asc --json demo` created a new mode-0700 temporary directory,
  wrote two mode-0600 no-value receipts, redacted both streams, printed its
  directory, and left a fresh sentinel `ASC_HOME` absent.

The first combined CLI harness had already created its `ASC_HOME` by running
the empty-state commands before its demo assertion. Its resulting failure was a
harness ordering error. A new, never-created sentinel path passed; that
correction is recorded separately in the evidence directory.

## Live routes, accessibility, privacy, and offline behavior

- Home, Demo, Privacy, Terms, and `/404.html` passed `verify-url.sh`. Each had
  its route title, `lang=en`, one h1, a main landmark, image alternatives,
  labeled buttons, and no console or page errors.
- Fresh desktop and phone Axe scans found zero serious or critical violations
  on all five documents.
- An unknown route returned HTTP 404 and the exact designed 404 document with
  “This page does not exist.” Its expected failed-navigation console message is
  not a defect.
- Tab reached the skip link first and Enter moved focus to `main`. Space
  activated Reset. Demo → Home and Privacy → Terms → Back focused and announced
  the destination h1.
- All visible controls on every checked phone route were at least 44×44 CSS
  pixels. No route overflowed at 390 px. A 200% text-size smoke test retained
  the heading and action without horizontal overflow.
- Reduced motion set transition and animation duration to `0.00001s`. Nothing
  flashed or looped.
- A fresh service-worker-controlled context had an active worker, no waiting or
  installing update, and reloaded the populated Demo offline.
- Every internal link, both GitHub links, robots, sitemap, favicon, touch icon,
  social image, and service worker returned the expected successful response.
- The Privacy page covers local receipt data, demo storage, deletion, network
  limits, and a privacy-request path through the public source repository.
- Live headers include CSP, HSTS, `nosniff`, strict-origin referrer policy, and
  camera, microphone, and geolocation denial. Hashed assets are immutable for
  one year; the service worker is `no-cache`.

The dark concrete-and-moss presentation, self-hosted serif and mono fonts,
square controls, and restrained state motion match `.factory/design.md`.

## Declared claims from a clean checkout

A fresh remote clone was pinned to the implementation SHA. `npm ci` installed
60 packages with zero reported vulnerabilities. Every command in
`.factory/claims.json` ran independently and passed. Every declared claim tag
occurs exactly once.

| Claim | Result |
| --- | --- |
| `demo-isolation` | PASS, 2 projects |
| `offline-reload` | PASS, 2 projects |
| `cli-demo` | PASS, 1 pass / 1 intentional project skip |
| `redaction-forms` | PASS |
| `process-tree` | PASS |
| `captured-output-receipt` | PASS |
| `credential-lifecycle` | PASS |
| `receipt-commands` | PASS |
| `receipt-storage-schema` | PASS |
| `cli-interface` | PASS |
| `demo-parity` | PASS, 1 pass / 1 intentional project skip |
| `license-package` | PASS, 1 pass / 1 intentional project skip |
| `site-privacy` | PASS, 2 projects |
| `build-output` | PASS, 2 projects |

Passing the listed commands does not close F-8-1 because the two `doctor`
privacy statements are absent from the manifest.

## Installed CLI, normal, invalid, boundary, and recovery paths

| Check | Result |
| --- | --- |
| `npm test` | PASS: 11 Rust tests, 2 Vitest tests, 40 Playwright passes, 6 intentional duplicate-project skips |
| `npm run build` | PASS; produced `target/release/asc` and `dist/site` |
| `cargo fmt --check` | PASS |
| Strict Clippy | PASS with workspace, all targets, all features, locked dependencies, and denied warnings |
| `cargo package -p agent-secret-capsule --allow-dirty` | PASS; verified 11 files |
| Clean package consumer | PASS; installed `asc 0.1.0` into a new prefix |

The installed artifact returned useful root and subcommand help. JSON demo,
empty alias and receipt states, invalid aliases and environment names, zero and
61-minute time limits, receipt limits 0 and 1001, and short input were exercised.
Usage boundaries returned exit 2; storage failures returned exit 3. No supplied
input appeared in error output.

The release-configured unavailable-store check ignored the test-store variable,
wrote no test-store file or alias metadata, did not disclose input, and exited
3. This worker has no unlocked Secret Service session. Successful lifecycle
behavior was therefore exercised through the explicitly feature-gated isolated
claim store. The product no longer claims successful native-keychain storage.

This product has no backend, so tenant isolation, server restart persistence,
health, and HTTP 429/`Retry-After` checks do not apply.

## Performance

Fresh mobile Lighthouse scored **100 performance, 100 accessibility, 100 best
practices, and 100 SEO**. FCP was 0.95 seconds, LCP 1.37 seconds, CLS 0, total
blocking time 0 ms, and transfer size 129,529 bytes.

The build emitted 3,298 bytes of JavaScript across all chunks, 13,549 bytes of
CSS, 36,056 bytes of self-hosted fonts, and an 84,742-byte phone hero. These are
inside the declared budgets.

## Earlier finding disposition

Every earlier review, verification, polish report, and handoff was inspected.
Disposition below comes from current live, clean-build, or installed-artifact
evidence rather than closure notes alone.

| Earlier finding | Current disposition |
| --- | --- |
| Verification 1 parser panic | Fixed. Valid commands reach operational results; invalid parser paths return exit 2 without panic. |
| Verification 1 cache gap | Fixed. Live hashed assets are immutable for one year and `sw.js` is `no-cache`. |
| Review 1 B1 / Review 2 F-2-1 | Fixed. Job, audience, action, outcome, and three facts are before scrolling at both required sizes. |
| Review 1 B2 / Review 3 F-3-1 | Fixed. Browser and CLI demos open populated, remain labeled, reset, and preserve unrelated data. |
| Review 1 B3 and U01–U57 / Reviews 2–4 claim findings | **Reopened in part by F-8-1.** All 14 listed claims pass, but the `doctor` no-read and CLI no-telemetry claims were missed. |
| Review 1 B4 | Fixed. Copy and tests state the selected process and its children. |
| Review 1 B5 | Fixed. No paid tier, checkout, restore, price, or billing claim remains. |
| Review 1 B6 / M1 / Review 2 F-2-4 | Fixed. Route metadata and discovery assets load; unknown URLs use the designed HTTP 404. |
| Review 1 M2 / Review 4 F-4-1 | Fixed. Shared structure, route focus, announcements, and Back behavior pass. |
| Review 1 M3 and CW01–CW26 / Reviews 2–4 copy findings | Fixed. Current site, legal, README, and catalog copy is literal, short, and consistent. |
| Review 1 M4 / Review 3 F-3-3 | Fixed. Phone targets meet 44 px, facts fit, and routes do not overflow. |
| Review 4 F-4-4 / F-4-5 | Fixed. GitHub links identify the destination; README has an absolute demo link. |
| Review 6 F-6-1 | Fixed. The false paid-license restore changelog line is absent. |
| Review 6 F-6-2 | Fixed. Terms warranty copy is split into sentences within 22 words. |
| Review 6 F-6-3 | Fixed. Repeated Demo and Terms targets are 44×44 or larger. |
| Review 7 F-7-1 | Fixed. Native-keychain success wording is absent, and the release binary fails closed without activating test storage. |
| Verification 2 environment note | Still bounded. This worker lacks an unlocked Secret Service session, and the product makes no native-success claim. |
| Reviews 5 and verifications 3–4 | Their behavioral checks remain green, but their zero-untested-claim conclusion is superseded by F-8-1. |

The brief does not benefit from runtime AI. Adding it would widen the credential
boundary. No missed-leverage finding applies.

## Evidence

Review evidence is under
`/work/.evidence/agent-secret-capsule-review-8/`, including live browser JSON,
screenshots, route checks, claim logs, build/package output, installed-CLI
checks, candidate/live comparisons, and Lighthouse JSON.

**Final verdict: FAIL — 1 finding and 2 untested public claims.**
