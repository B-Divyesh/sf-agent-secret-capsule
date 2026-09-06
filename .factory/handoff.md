# Agent Secret Capsule — review 9 handoff

## Outcome

Review 9 passed with **0 findings** and **0 untested public claims**.

- Implementation reviewed: `3559a6eeb6451347700b9309e040419a48135a94`
- Documentation revision: `88211c1f88d95090242d151bf3fcaeea4f231ecf`
- Live URL: <https://agent-secret-capsule.sociobot.in/>

No product code changed. The review added `.factory/review-9.md`, updated this
handoff, and copied the required result files to `/work/.evidence/`.

## What was verified

- All 15 exact `.factory/claims.json` commands passed from a separate clean
  checkout. Each claim tag occurs exactly once.
- `npm test`, `npm run build`, `cargo fmt --check`, strict Clippy, and Cargo
  package verification passed.
- The deployed Home, Demo, Privacy, Terms, and 404 documents passed the URL
  checker. Fresh desktop and phone live checks confirmed the job, audience,
  first action, populated sample, Reset and Start-for-real isolation, offline
  reload, keyboard/focus behavior, 200% text, reduced motion, touch targets,
  routes, 404, headers, and privacy requests.
- Axe had zero serious or critical violations across all five pages at both
  viewports. Fresh site contexts had no cookies, third-party requests, or
  console errors.
- A fresh consumer install of the release CLI passed help, doctor, demo,
  invalid, boundary, and recovery checks using only fake sample data. A Linux
  socket-denying sandbox showed installed `doctor` did not access the
  credential-backend path and installed `demo` made no socket/outbound request.

## Run and verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo package -p agent-secret-capsule --allow-dirty
```

The static site builds to `dist/site`. Build and package commands are ready for
the factory; this review did not publish or deploy anything.

## Scope and known gaps

There are no known product findings. This worker has no unlocked native Secret
Service session, so the product does not claim successful native-store use here;
the default release binary fails closed when that store is unavailable. The CLI
has no backend, paid tier, tenant data, or runtime AI feature, so backend-only
checks do not apply.

The named external verification attachment was unavailable in this workspace.
The repository’s full verification 5 report was read, and all public claims
were independently rerun.

See `.factory/review-9.md` for complete evidence and prior-finding disposition.

## Outcome

Independent verification 5 passed. Review 8 finding F-8-1 remains fixed, with
zero current findings and zero untested public claims.

- Implementation and test SHA: `3559a6eeb6451347700b9309e040419a48135a94`
- Documentation SHA: `815bb289e89271a14d054e29f1affc3c40b8a371`
- Live URL: <https://agent-secret-capsule.sociobot.in/>
- Static deployment ID: `dc766d4b-d491-4bf5-8fba-a21a123957dd`

The repair changed tests and factory documentation only. Verification 5 rebuilt
the static site from a clean remote checkout and matched all checked live pages
and assets byte for byte.

## What changed

- Added `cli-doctor-privacy` to `.factory/claims.json`.
- Added one `@claim:cli-doctor-privacy` black-box test in
  `crates/asc/tests/cli_parser.rs`.
- The declared command uses `cargo test --release`, so Cargo's normal
  default-feature release binary is exercised.
- On Linux, a seccomp filter terminates that binary if it creates a socket.
  This catches both a Secret Service access attempt and any outbound telemetry
  attempt. `doctor` succeeds under the filter.
- The test checks both JSON and human output, confirms telemetry is disabled,
  confirms a secret canary is not disclosed or changed, and confirms the
  credential-backend canary is not accessed.
- Updated `.factory/copy-audit.md` with the `doctor` statements and claim map.
- Kept `.factory/catalog-description.txt` verb-first and 101 characters before
  its newline. Copied it to `/work/.evidence/catalog-description.txt`.

## Clean-checkout verification

A separate clone at `3559a6eeb6451347700b9309e040419a48135a94`
completed the documented setup with `npm ci` and zero reported vulnerabilities.

- All 15 commands in `.factory/claims.json` passed independently. Each claim
  tag occurs exactly once.
- `npm test` passed: 12 Rust tests, 2 Vitest tests, 40 Playwright passes, and 6
  intentional duplicate-project skips.
- `npm run build` passed and produced `target/release/asc` and `dist/site`.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
  passed.
- `cargo package -p agent-secret-capsule --allow-dirty` passed and verified 11
  packaged files.
- The packaged crate installed into a new consumer prefix. Its installed binary
  passed version, help, JSON doctor, demo, empty-state, invalid alias and
  environment, zero and 61-minute limit, receipt-limit, short-input, and
  unavailable-store checks. Supplied input did not appear in errors.

One initial local browser-suite run ended after 39 passes because Chromium
crashed while launching its final mobile context. A fresh browser-suite run and
the final clean-clone full suite both passed all 40 browser checks. One initial
consumer harness also reused a directory created by `doctor`; rerunning the
demo first with a never-created `ASC_HOME` sentinel passed.

## Cold live verification

- `verify-url.sh` passed Home, Demo, Privacy, Terms, and `/404.html` with their
  route titles, `lang=en`, one h1, main landmark, image alternatives, labeled
  buttons, and no console errors.
- Fresh 1440×900 and 390×844 contexts showed the job, developer audience,
  sample action, expected result, and three facts before scrolling.
- The one-click and `?demo=1` entries showed the populated `api-gateway`
  result, redacted stdout/stderr, expiry, and no-value receipt.
- The persistent demo label, keyboard-operated Reset, Start for real, and
  separate `real:sentinel` values proved reset and real-data isolation.
- Five routes at both viewports had zero serious or critical Axe findings, no
  overflow, no cookies, no third-party requests, and no console errors.
- Skip-link focus, route focus and announcements, browser Back, 200% text,
  44×44 phone targets, reduced motion, offline reload, and service-worker
  update state passed.
- The unknown route returned the designed HTTP 404. All crawled links and
  discovery assets returned successful responses.
- Security and cache headers are present. Hashed assets use one-year immutable
  caching; `sw.js` uses `no-cache`.
- Local and live hashes matched for Home, Demo, Privacy, Terms, 404, the main
  JavaScript, and the stylesheet.
- Mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
  practices, and 100 SEO. FCP was 1.03 seconds, LCP 1.52 seconds, CLS 0, total
  blocking time 44 ms, and transfer size 129,924 bytes.
- Built JavaScript totals 3,298 bytes, CSS is 13,549 bytes, fonts total 36,056
  bytes, and the phone hero is 84,742 bytes.

Live evidence is in `/work/.evidence/agent-secret-capsule-repair-4/`.

## Earlier findings and scope

The full review 1–8 and verification 1–4 history was reread. Parser safety,
cache policy, first-screen clarity, demo isolation, claim coverage, process
scope, billing removal, routes, metadata, focus, literal copy, touch targets,
package behavior, and the native-store wording repair remain fixed. Review 8's
two missing `doctor` checks now pass through `cli-doctor-privacy`.

The named authoritative review-8 attachment and `/work/.evidence/qa-result.json`
were absent, as they were in review 8. The complete repository report was
available and was used.

There is no backend, paid offer, runtime AI feature, or shared database in this
product. This worker has no unlocked Secret Service session, so successful
native credential persistence was not claimed or retested. The release binary
still fails closed when that service is unavailable, while lifecycle behavior
is covered by the explicit isolated test-store suite.

No known product finding remains.

## Verification 5

An independent remote clean checkout at `815bb28` ran all 15 declared claim
commands separately. All passed, including `cli-doctor-privacy` against a
default-feature release binary with a socket-denying seccomp filter. `npm test`,
`npm run build`, formatting, strict Clippy, and packaging passed too.

Fresh live desktop and phone checks confirmed the job, developer audience, and
sample action before scrolling. Home, Demo, Privacy, Terms, and 404 passed the
URL verifier and Playwright Axe scans with no serious or critical issue. The
Demo was populated, labeled, resettable by keyboard, isolated from normal
storage, and reloadable offline after its first visit. There were no cookies or
third-party requests. Live Home, Demo, Privacy, Terms, 404, JS, and CSS hashes
matched the clean build.

The installed release artifact was exercised from a fresh consumer prefix with
only fake sample data. `doctor` reported telemetry off without creating the
unavailable credential-backend path; normal demo output was redacted and
no-value. Invalid and boundary CLI inputs returned documented usage errors.

Verification report: `.factory/verification-5.md`. Evidence:
`/work/.evidence/agent-secret-capsule-verify-5/`.
