# Review 7: give one agent command a temporary credential — FAIL

**Verdict: FAIL.** One major finding remains. There is one untested public
claim, so this review cannot declare a release pass.

- Implementation reviewed: `f018b7d8f15a1be374575d642f4e25c158fa74d0`
- Documentation baseline: `97da8082e5dd599f3d6cc76ee5563192b56ab49c`
- Report-only base: `927586c3efb68e2839de9c0d3ce0c17b9681bc53`
- Live URL: <https://agent-secret-capsule.sociobot.in/>
- Date: 2026-09-06 UTC

## Job, audience, and first action

Fresh Chromium desktop (1440×900) and phone (390×844) contexts showed before
scrolling:

| Check | Live evidence |
| --- | --- |
| Job | “Give one agent command a temporary credential.” |
| Audience | “For developers running coding agents…” |
| First action | **Try it with sample data** |
| Desktop placement | Audience ends at y=657; primary action ends at y=736. |
| Phone placement | Audience ends at y=471; primary action ends at y=543. |

## Finding

### F-7-1 — MAJOR — OS-keychain storage is a public claim without a matching claim test

The changelog publicly says **“OS-keychain-backed named secrets.”** The
installed `asc --help` also says that Agent Secret Capsule “stores named
credentials in your OS keychain,” and the `put` command describes storage in
that keychain.

No entry in `.factory/claims.json` declares that release-binary claim. The
closest entry, `credential-lifecycle`, claims only a local-alias lifecycle. Its
declared test runs the binary with `--features test-keyring` and
`ASC_TEST_KEYRING_DIR`; the test source explicitly says this is a private,
file-backed test store that is not compiled into release binaries. It therefore
does not prove storage, retrieval, or removal through a native OS keychain.

The clean installed consumer artifact recovered safely when its unavailable
Linux Secret Service backend rejected a fake `put`: it exited 3, returned JSON,
and did not echo the input. That is useful recovery evidence, not proof of the
claimed successful native storage path.

**Required repair:** either remove the OS-keychain claim from public text until
it is proven, or add a distinct `os-keychain-storage` claim with a clean,
native-keychain integration test on each supported target. The test must run
the release configuration, perform `put → list → run → remove`, and assert no
value appears in aliases, output, or receipts. It must be listed in
`.factory/claims.json` and run from the documented clean setup.

## Demo, recovery, and browser results

The one-click browser sample passed. It opened `/demo/` and immediately showed
the realistic `api-gateway` production result, redacted stdout and stderr,
time-limit output, and a no-value receipt. The persistent label read “Demo —
sample data, nothing is saved.” On the phone, running it added only
`sessionStorage["demo:asc:run-count"]`; Reset restored `READY` and removed that
key. Separate `real:sentinel` local and session values survived Reset and
Start for real.

A fresh service-worker-controlled phone context reloaded `/demo/` offline and
kept the sample visible. The active worker had no waiting update. Reduced
motion set the redaction transition to `0.00001s`. The deliberate unknown URL
returned HTTP 404 with “This page does not exist,” which is the expected
designed 404 rather than a defect.

The installed clean consumer artifact reported `asc 0.1.0`; its help, JSON
demo, empty JSON list/receipt states, short-input recovery, and 61-minute
time-limit rejection behaved as documented. `asc --json demo` created a new
temporary sample directory, returned redacted streams, and supplied two
no-value receipts. The native keychain could not be unlocked in this container;
the observed operational recovery is described above.

## Declared claims from a clean checkout

A new clone at the implementation SHA completed `npm ci` (60 packages, zero
reported vulnerabilities). Each declared command was run independently and
passed:

| Claim | Result |
| --- | --- |
| `demo-isolation` | PASS |
| `offline-reload` | PASS |
| `cli-demo` | PASS |
| `redaction-forms` | PASS |
| `process-tree` | PASS |
| `captured-output-receipt` | PASS |
| `credential-lifecycle` | PASS — test-only keychain, not native-keychain proof |
| `receipt-commands` | PASS |
| `receipt-storage-schema` | PASS |
| `cli-interface` | PASS |
| `demo-parity` | PASS |
| `license-package` | PASS |
| `site-privacy` | PASS |
| `build-output` | PASS |

`npm test`, `npm run build`, `cargo fmt --check`, strict Clippy, and
`cargo package -p agent-secret-capsule --allow-dirty --list` also passed.
The package listed 11 expected files and the clean consumer install succeeded.

## Live routes, accessibility, privacy, and deployment

Fresh desktop and phone scans found zero console errors, page errors, or Axe
serious/critical violations on Home, Demo, Privacy, Terms, and 404. Every page
had its expected route title, `lang="en"`, one `h1`, and one `main`. The skip
link was first in keyboard order and moved focus to `#main`. No horizontal
overflow occurred at 390 px. Every visible header/footer link measured at least
44×44 CSS pixels.

`verify-url.sh` passed on all five routes. All observed live browser requests
were same-origin; fresh contexts had no cookies. Every internal and linked
GitHub URL returned HTTP 200. Security headers include CSP, HSTS, `nosniff`,
strict-origin referrer policy, and camera/microphone/geolocation denial.

The local production build and live deployment matched byte-for-byte for
`index.html`, main JavaScript, stylesheet, and mobile hero image. The checked
hashes were identical. The initial JavaScript gzip size is 0.46 KiB, CSS gzip
size is 3.85 KiB, and the 960 px hero image is 84.7 KiB, within the stated
static budgets. A Lighthouse rerun was attempted but its launcher crashed in
this disposable container; no Lighthouse score is claimed in this review.

## Earlier finding disposition

| Earlier finding | Current disposition |
| --- | --- |
| Verification 1 parser panic and cache policy | Fixed; clean compiled/package checks and current immutable asset caching pass. |
| Review 1 B1–B6, M1–M4, CW01–CW26, U01–U57 | Fixed or removed; current live demo, routing, copy, touch targets, and listed claims pass. |
| Review 2 F-2-1 through F-2-4 | Fixed; first-screen placement, claim ledger, direct copy, and 404 metadata pass. |
| Review 3 F-3-1 through F-3-3 | Fixed; populated sample, compiled CLI coverage, and phone fact placement pass. |
| Review 4 F-4-1 through F-4-5 | Fixed; route focus, build output, literal headings, named GitHub links, and README demo link pass. |
| Review 6 F-6-1 through F-6-3 | Fixed; the false paid-license changelog text is absent, Terms sentences meet the limit, and shared links are at least 44×44. |
| New F-7-1 | Open; native OS-keychain storage is still unlisted and untested as a release claim. |

## Evidence

Live browser JSON, route screenshots, `verify-url.sh` output, and command
evidence are under `/work/.evidence/agent-secret-capsule-review-7/`.

**Finding count: 1. Untested claim count: 1. Verdict: FAIL.**
