# Review 6: give one agent command a temporary credential — FAIL

**Product:** Agent Secret Capsule

**Live URL:** <https://agent-secret-capsule.sociobot.in/>

**Implementation candidate:** `49c494c492fa18e8b60e6400fa16838b81d782ab`

**Documentation baseline:** `8362d492a20e765824e9ed8042d6942892a2766b`

**Review date:** 2026-09-06 UTC

**Viewports:** fresh Chromium contexts at 390×844 and 1440×900

## Verdict

**FAIL.** Three findings remain: one major false and unlisted public claim, one
minor plain-words failure, and one minor touch-target failure. One public claim
has no declared automated test. The product therefore does not meet the required
zero-finding and zero-untested-claim threshold.

The live site is the reviewed implementation. Home, Demo, Privacy, Terms, the
designed 404, and every built JavaScript and CSS file matched the clean build by
SHA-256. Commits after `49c494c` contain reports and evidence only.

## Job, audience, and first action

Before scrolling, both fresh viewports state:

| Question | Answer shown |
| --- | --- |
| Job | Give one agent command a temporary credential, redact its output, and save a receipt without the credential. |
| Audience | Developers running coding agents that need an authorized command. |
| First action | **Try it with sample data.** The adjacent text says a fake credential and no-value receipt will appear. |

The action ends at y=736 on desktop and y=543 on phone. All three facts end at
y=803 on desktop and y=673 on phone. Nothing requires scrolling to understand
the job, audience, action, result, privacy fact, offline fact, or price fact.

## Findings

### F-6-1 — MAJOR — the changelog claims a paid feature that does not exist

`CHANGELOG.md:16` says version 0.1.0 added “paid license restore.” The live site,
README, installed CLI help, source, and package contain no purchase, license
entry, license storage, verification, or restore flow. The statement is false.

The statement is also absent from `.factory/claims.json`, so no declared test
can catch it. This is the review's one untested public claim.

**Required change:** remove “paid license restore” from the 0.1.0 release notes,
or implement the complete approved paid-license flow and add an observable claim
test. Removing the false text is the smaller change and matches the current free,
MIT-licensed product.

### F-6-2 — MINOR — one Terms sentence exceeds the plain-words limit

The final sentence under **No warranty** has 23 whitespace-delimited words:

> To the maximum extent allowed by law, the authors are not liable for
> credential exposure, service interruption, or indirect damages arising from
> use.

The plain-words contract sets a hard cap of 22 words for every public sentence.
All other live page and README sentences passed the same count and banned-word
check.

**Required change:** split this sentence into two short sentences without
changing its meaning.

### F-6-3 — MINOR — two repeated links are narrower than 44 pixels

At the 390×844 phone viewport, the header **Demo** link is 32×44 CSS pixels and
the footer **Terms** link is 40×44. The same widths occur on desktop. Both are
below the required 44×44 touch target. They repeat on Home, Demo, Privacy,
Terms, and the 404 page.

The existing test checks target height only. Axe reports no serious or critical
issue because this rule is outside that scan.

**Required change:** add enough horizontal padding or `min-width: 44px`, then
assert both dimensions for every visible interactive target at 390 pixels.

## Demo and real-data isolation

| Check | Result |
| --- | --- |
| One-click entry | PASS — Home opens `/demo/` with one action. |
| Direct entry | PASS — `/?demo=1` redirects to `/demo/`. |
| Realistic first result | PASS — `api-gateway`, production, healthy, two redactions, expiry, alias, outcome, count, and omitted value are populated. |
| Initial viewport | PASS — the phone's last receipt field ends at y=841 of 844; desktop ends at y=616 or earlier. |
| Persistent label | PASS — “Demo — sample data, nothing is saved” remains after rerun and Reset. |
| Separate namespace | PASS — rerun creates only `sessionStorage["demo:asc:run-count"]`. |
| Reset | PASS — Reset removes the demo key and restores `READY`. |
| Start for real | PASS — it removes the demo key and returns Home. |
| Real-data sentinel | PASS — `real:sentinel` in local and session storage survived Reset and Start for real. |
| Requests and cookies | PASS — all observed requests were same-origin; no cookies appeared. |
| Offline | PASS — a separately owned context reloaded `/demo/` offline after service-worker control and kept the sample output. |
| CLI sample | PASS — the installed release binary made a new `/tmp/asc-demo-*` directory with two no-value receipts and did not touch sentinel `ASC_HOME`. |

The demo directory was mode 0700 and `receipts.jsonl` was mode 0600. The fake
credential was absent from captured streams and files.

## Declared claims from a clean checkout

Clean clone: `/tmp/asc-review6-clean.iKocoa/repo` at documentation baseline
`8362d492…`. `npm ci` installed the documented prerequisites with zero reported
vulnerabilities. Every command in `.factory/claims.json` ran independently.

| Claim | Declared command result |
| --- | --- |
| `demo-isolation` | PASS, 2 projects |
| `offline-reload` | PASS, 2 projects |
| `cli-demo` | PASS, 1 project and 1 intentional viewport skip |
| `redaction-forms` | PASS, compiled CLI test |
| `process-tree` | PASS, compiled CLI test |
| `captured-output-receipt` | PASS, compiled CLI test |
| `credential-lifecycle` | PASS, compiled CLI test |
| `receipt-commands` | PASS, compiled CLI test |
| `receipt-storage-schema` | PASS, compiled CLI test |
| `cli-interface` | PASS, compiled CLI test |
| `demo-parity` | PASS, 1 project and 1 intentional viewport skip |
| `license-package` | PASS, 1 project and 1 intentional viewport skip |
| `site-privacy` | PASS, 2 projects |
| `build-output` | PASS, 2 projects |

Each claim ID occurs in exactly one test definition. The changelog statement in
F-6-1 remains outside the manifest, so the untested claim count is **1**.

## Installed CLI exercise

The release package was installed with `cargo install --path crates/asc --root
<new-temp-dir> --locked`. The installed artifact reported `asc 0.1.0`.

- Installed `asc --help` returned useful commands and exit codes. The compiled
  claim test also checked every subcommand's example.
- In an isolated D-Bus and GNOME Secret Service session, the release binary
  completed `put → list → run → receipts → remove → list`.
- The selected child wrote the synthetic value to both streams. The installed
  binary returned two `[REDACTED:ASC]` markers and a no-value receipt.
- Invalid alias, environment name, zero receipt limit, limit 1001, and time
  limit 61m returned usage exit 2 with specific guidance.
- A five-byte credential returned operational exit 3 and explained the
  eight-byte minimum.
- Without a Secret Service session, `put` returned documented exit 3 with a
  clear keychain error. The synthetic value did not appear in output.
- Empty `list` and `receipts` returned valid JSON empty states.

This product has no backend. Tenant isolation, server restart persistence,
health endpoints, and HTTP 429/`Retry-After` checks do not apply.

## Live routes, accessibility, privacy, and performance

- `/`, `/demo/`, `/privacy/`, `/terms/`, and the designed 404 have distinct
  titles, descriptions, canonicals, social metadata, `lang=en`, one h1, one
  main landmark, ordered headings, and labeled controls.
- `/not-a-real-route` correctly returns HTTP 404 with the designed page. The
  deliberate status is expected and is not a defect.
- Every normal internal and GitHub link returned 200. Discovery files, favicon,
  touch icon, social image, and service worker returned correct content types.
- Direct routes focus and announce their h1. Privacy → Terms → Back restores
  Privacy focus. Demo → Home focuses and announces the Home h1.
- Tab reaches the skip link first; Enter moves focus to main. Space activates
  Reset. Enter activates Start for real. Clipboard denial selects the install
  command and changes the button label.
- Fresh phone and desktop runs had no console errors, page errors, horizontal
  overflow, third-party requests, analytics, cookies, or external fonts.
- Reduced motion matched the media query and reduced transitions and animations
  to 0.01ms. No looping motion or flashing exists.
- Playwright Axe found zero serious or critical violations on five routes at
  both viewports. F-6-3 remains because Axe does not enforce the attached
  44×44 rule.
- Mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
  practices, and 100 SEO. LCP was 1.5s, CLS was 0, and total blocking time was
  0ms.
- Built JavaScript files are at most 1.22 KB uncompressed. CSS is 13.46 KB;
  fonts total 36.06 KB; the mobile hero is 84.74 KB.
- Hashed assets return `Cache-Control: public, max-age=31536000, immutable`.
- `verify-url.sh` passed Home, Demo, Privacy, Terms, and `/404.html` with no
  console errors, missing alt text, or unlabeled buttons.

The concrete-and-moss visual system, self-hosted serif and mono fonts, square
controls, restrained motion, and custom art still match `.factory/design.md`.

## Earlier finding disposition

Every earlier review, verification, polish report, and handoff was read. The
following status comes from current live or clean-build evidence.

### Severity and round findings

| Earlier ID | Current disposition |
| --- | --- |
| B1 | Fixed — both first screens show job, audience, action, result, and three facts. |
| B2 | Fixed — browser and CLI samples, labels, Reset, exit, namespace, and sentinel isolation pass. |
| B3 | Regressed in part — 14 listed claims pass, but F-6-1 is a public claim outside the manifest. |
| B4 | Fixed — copy and compiled tests cover the selected process and its children. |
| B5 | Regressed in documentation — no dead paid flow remains, but F-6-1 falsely says paid license restore shipped. |
| B6 | Fixed — real routes work and unknown paths return the designed HTTP 404. |
| M1 | Fixed — route metadata and discovery assets are complete. |
| M2 | Fixed — shared shell, order, route focus, announcements, and Back behavior pass. |
| M3 | Open in part — earlier landing and README copy is fixed; F-6-2 finds one 23-word Terms sentence. |
| M4 | Open — F-6-3 proves the Demo and Terms targets are narrower than 44 pixels. |
| F-2-1 | Fixed — desktop audience and action end before y=900. |
| F-2-2 | Regressed in part through F-6-1; all 14 declared claims still pass. |
| F-2-3 | Fixed — README sentences remain within the limit and visitor copy uses “time limit.” |
| F-2-4 | Fixed — 404 social metadata is complete. |
| F-3-1 | Fixed — both demo first screens show the populated result and receipt. |
| F-3-2 | Fixed — lifecycle, receipt, schema, interface, and parity tests pass. |
| F-3-3 | Fixed — all three phone facts end at y=673. |
| F-4-1 | Fixed — Demo → Home focuses and announces the Home h1. |
| F-4-2 | Fixed for README and build output; F-6-1 is a separate changelog failure. |
| F-4-3 | Fixed — literal process and 404 headings remain. |
| F-4-4 | Fixed — GitHub links identify the external destination. |
| F-4-5 | Fixed — README contains the absolute demo link. |

Review 5 assigned no finding IDs. Its PASS is superseded because it did not
include the stale changelog claim, the 23-word legal sentence, or target width.

### Review 1 copy findings

| Earlier IDs | Current evidence |
| --- | --- |
| CW01–CW06 | Fixed — job, audience, process scope, redaction, and receipt wording are direct and tested. |
| CW07–CW09 | Fixed — “time limit,” “alias,” and process-and-children wording are consistent. |
| CW10–CW14 | Fixed — sample and copy actions name their result; browser and CLI parity passes. |
| CW15–CW16 | Fixed — authorized-command warning is split and the vague safety phrase is absent. |
| CW17–CW22 | Fixed — README opening, audience, scope, streams, encodings, and limits are short and literal. |
| CW23–CW26 | Fixed — no license form is advertised; sandbox, scope, and encoding limits are explicit. |

F-6-2 is a newly found legal-page sentence, not a return of the exact earlier
landing or README sentences.

### Review 1 claim findings

| Earlier IDs | Current evidence |
| --- | --- |
| U01–U03 | Fixed — broad prompt, platform, telemetry, and unconditional storage promises remain absent from the site and README. |
| U04–U06 | Fixed — process scope, named encodings, and receipt omission tests pass. |
| U07–U09 | Fixed — capability, prompt, and keychain-resolution promises remain absent. |
| U10–U14 | Fixed — time limit, both streams, encodings, exact receipt schema, and persisted omission pass. |
| U15–U16 | Fixed — demo isolation and browser/CLI parity pass. |
| U17–U18 | Fixed — bundled daemon/account/cloud-vault and compiler promises remain absent. |
| U19–U20 | Fixed — these remain explicit limitations, not guarantees. |
| U21–U27 | Fixed — free-tier, price, release, update, merchant, refund, and license-status marketing remains absent from live and README copy. |
| U28–U31 | Fixed — receipt behavior passes; broad keychain and telemetry copy remains removed. |
| U32–U34 | Fixed — limitation retained; compiler and binary-release promises remain absent. |
| U35–U42 | Fixed — package, lifecycle, process, output, receipts, removal, help, and non-TTY tests pass. |
| U43–U47 | Fixed — compiled security paths, stored omission, warnings, suite, and site privacy pass. |
| U48 | Regressed through F-6-1 — implementation has no license flow, but the changelog says paid license restore shipped. |
| U49–U50 | Fixed — MIT source/package test passes and lifetime-update wording is absent. |
| U51–U54 | Fixed — history guarantees and team/receipt-policy entitlements remain absent; receipt JSON passes. |
| U55–U57 | Fixed — metadata uses credential wording; copy uses redact and explicit omission. |

The independent-verification parser panic remains fixed: valid alias commands
reach normal operational paths and invalid parsers return exit 2 without a
panic. The static-cache issue remains fixed by the live immutable asset header.

## Quality commands

```text
npm ci                 PASS (60 packages, 0 vulnerabilities)
14 claim commands      PASS independently
npm test               PASS (10 Rust, 2 Vitest, 38 Playwright; 6 intentional skips)
npm run build          PASS (release CLI and dist/site)
cargo fmt --check      PASS
cargo clippy ...       PASS with -D warnings
cargo package ...      PASS (11 files; package verification succeeded)
verify-url.sh           PASS on five documents
live Axe               PASS on five routes × two viewports
mobile Lighthouse      100 / 100 / 100 / 100
```

## Missed leverage

No additional product feature is needed. This tool needs a narrow local
credential boundary. Sending credential context to an AI service or adding sync
would widen that boundary. Human and JSON receipts already provide the useful
export path.

## Required next steps

1. Remove the false “paid license restore” changelog text or ship and test that
   feature.
2. Split the 23-word Terms sentence.
3. Make every repeated navigation and footer target at least 44×44 pixels and
   extend the mobile test to assert width and height.

Re-run the complete claim list, installed release-binary flow, live browser
checks, copy count, and target-size audit before changing the verdict.
