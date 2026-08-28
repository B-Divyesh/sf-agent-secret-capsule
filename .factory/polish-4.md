# Polish 4 — cumulative zero-finding closure

Implementation commit: `49c494c492fa18e8b60e6400fa16838b81d782ab`  
Deployment: `https://agent-secret-capsule.sociobot.in/` (static work order `agent-secret-capsule-polish-4`)

This document maps every finding in `review-1.md` through `review-4.md` to
the current implementation and evidence. I reread all four reviews and all
three earlier polish records before making the repair. “Removed” means absent
from current site copy, metadata, controls, and README—not hidden.

## Round-four findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-4-1 / M2 | Added shared route-focus behavior. Home now focuses its h1 and announces its title on `load` and persisted history restores; Demo → Home has a dedicated regression test. The next Tab after a programmatic heading focus moves to the skip link. | `Demo → Home focuses and announces the new route`; live `.factory/evidence/polish-4-live/review4-checks.json` records focused Home h1 and announcement; live Home screenshot `home-after-demo-mobile.png`. |
| F-4-2 / B3 / U46 / F-2-2 / F-3-2 | Removed README test-composition/manifest sentences. Added the `build-output` manifest claim and an observable artifact test for the deployment root, route files, service worker, and SWA configuration. | `@claim:build-output`; all 14 manifest commands pass independently in clean clone `/tmp/asc-round4-clean.u4edhs/repo`. |
| F-4-3 / M3 | Replaced the landing caption with “The selected process and its children receive one credential.” Replaced the 404 h1 with “This page does not exist.” | `.factory/copy-audit.md`; live `home-cold-desktop.png`; live `/404.html` h1 in `cumulative-live-checks.json`. |
| F-4-4 | Renamed the install reference and every footer source link to name GitHub and mark it external. | `external GitHub links name their destination`; live route crawl in `cumulative-live-checks.json` and `review4-checks.json`. |
| F-4-5 | Replaced the root-relative README demo text with `[Try the web sample](https://agent-secret-capsule.sociobot.in/demo/)`. | Live GitHub raw README check in `review4-checks.json`; live `/demo/` returns 200. |

## Severity, routing, and presentation findings from earlier reviews

| Finding | Change made | Evidence |
| --- | --- | --- |
| B1 / F-2-1 | Kept the job-first headline, named developer audience, one-click sample action, outcome note, and tested fact strip within both first viewports. | Live `home-cold-desktop.png` and `home-cold-mobile.png`; `cumulative-live-checks.json` records desktop 657/736px and mobile 471/543/627/650/673px bottoms. |
| B2 / F-3-1 | Kept `/demo/` and `?demo=1`, the banner, Reset, Start for real, `demo:asc:` sandbox key, immediate realistic deployment-status result, CLI `asc demo`, fixture, and documentation. | `@claim:demo-isolation`, `@claim:cli-demo`, `@claim:demo-parity`, `@claim:offline-reload`; live `demo-query-mobile.png`, `demo-cold-desktop.png`, `demo-cold-mobile.png`. |
| B3 | Maintained a complete 14-entry claim manifest with exactly one tagged command per retained observable promise. | Every command in `.factory/claims.json` passed independently from clean clone. |
| B4 | Public scope remains “selected process and its children”; it no longer claims an impossible single-process boundary. | `@claim:process-tree`; current landing, README, and caption. |
| B5 | Unsupported checkout, price, licensing, entitlement, and billing controls remain absent. | Live link crawl in `cumulative-live-checks.json`; no billing URL in built site. |
| B6 | Maintained multi-page routes, designed 404 response, discovery files, and SWA response override. | Live `/not-a-real-route` is HTTP 404; live status check and `real routes load, unknown routes return 404, and discovery assets exist`. |
| M1 / F-2-4 | Retained route-specific title, description, canonical, OG/Twitter card, favicon, touch icon, social image, robots, sitemap, and 404 metadata. | Five-route metadata/Axe browser tests; live `verify.json` reports in `polish-4-live*`. |
| M4 / F-3-3 | Retained 44px controls and no 390px horizontal overflow; compact fact strip now remains above the first viewport. | `mobile controls meet the touch target and never overflow`; live first-screen report. |
| F-2-3 | Visitor wording continues to use “time limit”; the authorized-command warning is two short sentences. | `.factory/copy-audit.md`; `@claim:process-tree`; live Home. |

## Copy findings from review 1

| Finding | Change made | Evidence |
| --- | --- | --- |
| CW01 | Uses the seven-word job-first credential headline. | Copy audit; live Home. |
| CW02 | Names developers running coding agents and the command result. | Copy audit; first-screen checks. |
| CW03 | Removed the abstract scope slogan. | Copy audit; live Home. |
| CW04 | Uses the process-and-children boundary. | `@claim:process-tree`. |
| CW05 | Names supported redaction forms. | `@claim:redaction-forms`. |
| CW06 | Uses the precise no-value receipt wording. | `@claim:captured-output-receipt`. |
| CW07 | Uses “time limit,” not lease jargon. | Copy audit; `@claim:process-tree`. |
| CW08 | Uses “alias,” not capability jargon. | Copy audit. |
| CW09 | Uses selected process and children plus time-limit wording. | `@claim:process-tree`. |
| CW10 | The demo heading names the deployment-check result. | Live `/demo/`; `demo-cold-mobile.png`. |
| CW11 | Replaced the scripted illustration with the isolated fake-data demo. | `@claim:demo-isolation`; `@claim:cli-demo`. |
| CW12 | Browser and CLI sample result parity is tested rather than merely asserted. | `@claim:demo-parity`. |
| CW13 | All sample controls name their result: try, reset, start, rerun. | Live `/demo/`; browser suite. |
| CW14 | Install control says “Copy install command.” | Browser suite; live Home. |
| CW15 | Boundary warning is split below the word limit. | Copy audit; live Home and README. |
| CW16 | Removed vague “safety layer” marketing. | Copy audit. |
| CW17 | README opening uses three short job/result sentences. | Copy audit. |
| CW18 | README audience/instruction is short and direct. | Copy audit. |
| CW19 | README separates scope, streams, forms, and receipt behavior. | CLI claim suite. |
| CW20 | README authorized-process warning is split. | Copy audit. |
| CW21 | Uses “percent-encoded,” not ambiguous “URL.” | `@claim:redaction-forms`. |
| CW22 | Says redaction limits leaks and explicitly is not a sandbox. | Live security-limits section. |
| CW23 | Unsupported license form remains absent. | Live link crawl. |
| CW24 | Keeps the direct non-sandbox limitation. | Live Home and README. |
| CW25 | Uses selected process tree and time limit. | `@claim:process-tree`. |
| CW26 | Names raw, percent, Base64, Base64url, and hex forms. | `@claim:redaction-forms`. |

## Claim-ledger findings from review 1

| Finding | Change made | Evidence |
| --- | --- | --- |
| U01 | Removed prompt/tool-trace guarantee. | Copy audit and current metadata. |
| U02 | Removed compressed platform/backend/telemetry bundle. | Current hero facts. |
| U03 | Removed unconditional OS-keychain-at-rest promise. | Copy audit; scoped CLI demo claim only. |
| U04 | Corrected public scope to process and children. | `@claim:process-tree`. |
| U05 | Retained only named supported redaction forms. | `@claim:redaction-forms`. |
| U06 | Retained only precise no-value receipt behavior. | `@claim:captured-output-receipt`. |
| U07 | Removed capability/prompt-content promise. | Copy audit. |
| U08 | Removed alias/keychain-resolution promise. | Copy audit. |
| U09 | Removed prompt-content promise. | Copy audit and metadata. |
| U10 | Corrected tree scope and time-limit wording. | `@claim:process-tree`. |
| U11 | Captured stdout and stderr remain tested. | `@claim:captured-output-receipt`. |
| U12 | All five named forms are tested on both streams. | `@claim:redaction-forms`. |
| U13 | Receipt metadata schema is listed and exact-field tested. | `@claim:receipt-storage-schema`. |
| U14 | Persisted receipts omit the credential. | `@claim:captured-output-receipt`. |
| U15 | Browser fake-data storage, reset, request scope, and real-data isolation are tested. | `@claim:demo-isolation`; live `review4-checks.json`. |
| U16 | Browser result is compared with the actual CLI demo fixture. | `@claim:demo-parity`. |
| U17 | Removed daemon/account/cloud-vault/telemetry bundle. | Copy audit. |
| U18 | Removed visitor-facing compiler-version promise. | README and copy audit. |
| U19 | Retained honest non-sandbox limitation. | Live security-limits section. |
| U20 | Retained short authorized-command boundary warning. | Copy audit; process-tree coverage. |
| U21 | Removed vague free-tier claim. | Current Home. |
| U22 | Removed ungated-feature marketing. | Current Home. |
| U23 | Removed price, kit, signed-release, and update promises. | Live link crawl. |
| U24 | Removed merchant-of-record copy. | Live Home and Terms. |
| U25 | Removed refund/revocation promise. | Live Home and Terms. |
| U26 | Removed free-core status. | Current Home. |
| U27 | Removed license-storage status/code. | Demo isolation storage assertions. |
| U28 | Retained tested no-value receipt behavior. | `@claim:captured-output-receipt`. |
| U29 | Split broad README promise into process/output/receipt claims. | `process-tree`, `redaction-forms`, `captured-output-receipt`. |
| U30 | Removed broad OS-keychain-at-rest sentence. | README security limits. |
| U31 | Removed hosted-store/CLI-telemetry sentence. | README. |
| U32 | Retained honest non-sandbox limitation. | README and live Home. |
| U33 | Removed compiler-version promise. | README. |
| U34 | Removed future binary-release promise. | README. |
| U35 | Kept package instruction and verified package contents. | `@claim:license-package`; clean-clone `cargo package --list`. |
| U36 | Added successful isolated put/list/run/remove lifecycle coverage. | `@claim:credential-lifecycle`. |
| U37 | Corrected to selected process tree and time limit. | `@claim:process-tree`. |
| U38 | Tests output forms, streams, receipt omission, and exit behavior through CLI. | `redaction-forms`; `captured-output-receipt`. |
| U39 | Added human and JSON receipt-command coverage. | `@claim:receipt-commands`. |
| U40 | Added successful remove behavior. | `@claim:credential-lifecycle`. |
| U41 | Added all documented help paths/examples. | `@claim:cli-interface`. |
| U42 | Added non-TTY rejection coverage. | `@claim:cli-interface`. |
| U43 | Splits tree, forms, streams, and receipt behavior into observable claims. | CLI claim suite. |
| U44 | Inspects persisted receipt log for omission. | `@claim:captured-output-receipt`. |
| U45 | Keeps explicit authorized-process warning. | Live security limits. |
| U46 | Removed the unlisted public test-suite-composition sentence. | README Develop and verify; `@claim:build-output` covers retained output statement. |
| U47 | Browser claim records same-origin requests, no cookies, and no third-party scripts. | `@claim:site-privacy`; live `cumulative-live-checks.json`. |
| U48 | License storage/verification code remains absent. | Storage checks and source audit. |
| U49 | Source and packaged CLI use exact MIT License. | `@claim:license-package`. |
| U50 | Removed lifetime-update promise. | Copy audit. |
| U51 | Uses `STORE / STDIN INPUT`; no history promise. | Live Home. |
| U52 | Uses `RECEIPTS / JSON`. | `@claim:receipt-commands`. |
| U53 | Removed team-kit entitlement. | Live link crawl. |
| U54 | Removed receipt-policy entitlement. | Live link crawl. |
| U55 | Metadata uses credential/time-limit wording. | Route metadata browser test. |
| U56 | Visitor copy uses “redact,” not “scrub.” | Copy audit. |
| U57 | Keeps precise receipt omission claim. | `@claim:captured-output-receipt`. |

## Independent-verification follow-ups

| Finding | Change made | Evidence |
| --- | --- | --- |
| Verification 1 critical parser panic | Corrected Clap parser return types and added compiled-binary parser/lifecycle coverage. | `valid_alias_commands_reach_their_operational_json_paths_without_panicking`; lifecycle claim. |
| Verification 1 immutable-cache gap | Retained immutable `/assets/*` and no-cache service-worker policy. | Site Vitest cache-config checks; live response headers. |
| Verification 2 unlocked-keychain environment note | Uses the feature-gated isolated test keychain for successful lifecycle claims; normal `asc demo` remains keychain-free. | `@claim:credential-lifecycle`; `@claim:cli-demo`. |

## Exact verification evidence

- Fresh clone: `/tmp/asc-round4-clean.u4edhs/repo` at
  `49c494c492fa18e8b60e6400fa16838b81d782ab`; `npm ci` passed with zero
  vulnerabilities.
- All 14 `.factory/claims.json` commands passed independently. Browser claims
  included two viewport projects; intentional desktop-only claim duplicates
  reported one skip on mobile.
- `npm test` passed: 10 Rust tests, 2 Vitest tests, 38 Playwright passes, and
  6 intentional duplicate-project skips. `npm run build`, `cargo fmt --check`,
  `cargo clippy --workspace --all-targets --locked -- -D warnings`, and
  `cargo package -p agent-secret-capsule --allow-dirty --list` passed.
- Local `verify-url.sh` reports and screenshots: `.factory/evidence/polish-4-local*/`.
  Local mobile Lighthouse: performance 100, accessibility 100, best practices
  100, SEO 100; LCP 1,843 ms, CLS 0, TBT 0. Report:
  `.factory/evidence/polish-4-local/lighthouse.json`.
- Cold live `verify-url.sh` reports and screenshots: `.factory/evidence/polish-4-live*/`.
  Live Axe: zero serious/critical violations for `/`, `/demo/`, `/privacy/`,
  `/terms/`, and `/404.html` at both 1440×900 and 390×844.
- Live cumulative report: `.factory/evidence/polish-4-live/cumulative-live-checks.json`;
  every crawled link was 200 and `/not-a-real-route` was HTTP 404.
- Live review-four report: `.factory/evidence/polish-4-live/review4-checks.json`;
  it confirms direct `?demo=1`, storage isolation/reset, Demo → Home h1 focus
  and announcement, literal copy, external link names, README web-demo link,
  and no console errors.

No known finding or deferred work remains.
