# Give one agent command a temporary credential — review 9

**Verdict: PASS.** There are **0 findings** and **0 untested public claims**.

- Implementation reviewed: `3559a6eeb6451347700b9309e040419a48135a94`
- Documentation revision: `88211c1f88d95090242d151bf3fcaeea4f231ecf`
- Live URL: <https://agent-secret-capsule.sociobot.in/>
- Review date: 2026-09-06 UTC
- Finding count: **0**
- Untested claim count: **0**

The documentation revision changes only `.factory/handoff.md` and
`.factory/verification-5.md`. A clean build of the implementation candidate
matched live Home, Demo, Privacy, Terms, 404, main JavaScript, demo JavaScript,
route-focus JavaScript, and CSS byte for byte.

The named external QA attachment was not present in this checkout or under
`/work/.evidence`; `.factory/verification-5.md` was available and read in full.
This review independently reran every declared command and the live checks
below, so the absent duplicate attachment leaves no claim untested.

## Job, audience, and first action

Fresh Chromium contexts opened the live page before scrolling at desktop and
phone sizes.

| Check | Result |
| --- | --- |
| Job | “Give one agent command a temporary credential.” |
| Audience | “For developers running coding agents…” |
| First action | **Try it with sample data** |
| Result after clicking | A populated fake `api-gateway` run with redacted stdout/stderr, an expiry result, and a no-value receipt |
| Desktop action / facts bottom | 736 / 803 px of a 900 px viewport |
| Phone action / facts bottom | 543 / 673 px of an 844 px viewport |

The home title is “Agent Secret Capsule — give one command a credential.” The
headline names the job and uses plain operational language.

## Demo, privacy, and installed artifact

- The one-click sample and `/demo/` began populated. Its persistent label said
  “Demo — sample data, nothing is saved.”
- Keyboard Space activated **Reset demo**, restored `READY`, and removed the
  `demo:asc:` key. Separate `real:sentinel` values in local and session storage
  survived Reset and **Start for real**.
- A fresh service-worker-controlled phone context reloaded the populated Demo
  offline after its first visit.
- Fresh Home contexts set no cookies, made only same-origin requests, and
  logged no console or page errors.
- A clean consumer prefix installed `asc 0.1.0`. Its version, root help, JSON
  doctor, JSON demo, invalid alias, zero time limit, and receipt limit 1001
  paths behaved as documented. Invalid/boundary inputs exited 2 with useful
  errors.
- The installed release `doctor` and `demo` each succeeded under a Linux
  seccomp wrapper that traps every `socket(2)` call. `doctor` returned
  `telemetry: false` and did not create or access an unavailable credential
  backend path. The demo used only bundled fake data, emitted redacted stdout
  and stderr, wrote no-value sample receipts, and touched neither `ASC_HOME`
  nor the credential-backend path. This observes no outbound CLI request
  without using a real credential.

## Accessibility, routes, and visual checks

- `verify-url.sh` passed Home, Demo, Privacy, Terms, and `/404.html`: every
  document had its own title, `lang=en`, one h1, main landmark, image
  alternatives, labeled controls, and no console error.
- Fresh desktop and phone Axe scans over all five documents had zero serious or
  critical violations.
- Skip link, keyboard Reset, route focus, Back focus, 44×44 demo targets,
  200% text without horizontal overflow, and reduced motion (maximum computed
  duration `0.00001s`) passed.
- Home, Demo, Privacy, Terms, 404, discovery assets, touch icon, social image,
  and GitHub source link loaded. An unknown route returned the designed HTTP
  404; the direct static `/404.html` route correctly returned 200.
- Live responses include CSP, HSTS, `nosniff`, and a strict-origin referrer
  policy. The service worker is `no-cache` and the page has no tracking cookie.
- Visual inspection of fresh desktop and phone screenshots found the concrete
  and moss treatment clear, legible, and consistent with `.factory/design.md`.

## Declared claims from a clean checkout

A separate clean checkout at the implementation SHA completed `npm ci` with
zero reported vulnerabilities. Each declaration in `.factory/claims.json` ran
as its exact command. All 15 passed, and each `@claim:` tag occurred exactly
once in the browser or Rust test suite.

| Claim | Result |
| --- | --- |
| `demo-isolation` | PASS |
| `offline-reload` | PASS |
| `cli-demo` | PASS |
| `redaction-forms` | PASS |
| `process-tree` | PASS |
| `captured-output-receipt` | PASS |
| `credential-lifecycle` | PASS |
| `receipt-commands` | PASS |
| `receipt-storage-schema` | PASS |
| `cli-interface` | PASS |
| `cli-doctor-privacy` | PASS; release binary under socket-denying seccomp |
| `demo-parity` | PASS |
| `license-package` | PASS |
| `site-privacy` | PASS |
| `build-output` | PASS |

`npm test` passed: 12 Rust tests, 2 Vitest tests, 40 Playwright passes, and 6
intentional duplicate-project skips. `npm run build`, `cargo fmt --check`,
strict Clippy, and `cargo package -p agent-secret-capsule --allow-dirty` also
passed. The built phone hero is 84,742 bytes; fonts total 36,056 bytes; CSS is
13,549 bytes; JavaScript totals 3,298 bytes.

## Earlier finding disposition

| Earlier finding | Current disposition |
| --- | --- |
| Verification 1 parser safety and cache gap | Fixed: valid and invalid installed CLI paths return operational results without panic; live service-worker cache policy is correct. |
| Review 1 B1–B6, M1–M4, and U01–U57 | Fixed or removed: the first screen, isolated demo, process-tree wording, no billing flow, routes/metadata, focus, plain copy, targets, and all retained observable claims pass above. |
| Review 2 F-2-1 through F-2-4 | Fixed: desktop comprehension/action, claim inventory, copy, and 404 metadata pass. |
| Review 3 F-3-1 through F-3-3 | Fixed: the demo is immediately populated, core claims are inventoried, and phone facts fit before scrolling. |
| Review 4 F-4-1 through F-4-5 | Fixed: Home focus/announcement, README claims/copy, external link labels, and usable demo link pass. |
| Review 6 F-6-1 through F-6-3 | Fixed: no false paid feature, Terms sentences stay within the limit, and repeated controls meet target size. |
| Review 7 F-7-1 and Verification 2 environment note | Fixed/bounded: no native-store success promise remains; unavailable-store behavior fails closed and test-store lifecycle coverage is explicit. |
| Review 8 F-8-1 | Fixed: `cli-doctor-privacy` is declared and passes for the release binary. This review additionally observed the installed binary under socket denial. |

This is a local CLI with no backend, tenant state, paid tier, or runtime AI
feature. Tenant isolation, server restart persistence, health, and 429 checks
do not apply.

Evidence: `/tmp/agent-secret-capsule-review-9-live/` and
`/tmp/agent-secret-capsule-review-9-logs/` in the review worker.

**Final verdict: PASS — 0 findings and 0 untested public claims.**
