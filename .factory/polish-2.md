# Polish 2 — cumulative finding closure

Implementation commit: `3a36e49eded9f501730f90f2aa1c38a02883cd54`  
Deployment: `13603aa0-fc26-4a4b-9f13-23e6cecd15b8`  
Live site: <https://agent-secret-capsule.sociobot.in/>

This ledger includes every finding from `review-2.md` and every earlier finding
from `review-1.md`. A removal means the unsupported public claim or control is
absent from the live product, not hidden behind a different phrase.

## Review 2

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1 | Reduced the wide-screen display size and hero spacing while retaining the concrete-and-moss composition. The audience and sample action now end at 671 px and 750 px in the 900 px viewport. | `desktop first screen shows its audience and sample action`; `.factory/evidence/polish-2-live/first-screen-1440.png`; live `cold-browser-report.json` |
| F-2-2 | Removed the prompt and Rust-version promises; expanded `cli-demo` to cover unavailable-keychain and `ASC_HOME` isolation; added `license-package` and `site-privacy` claims. The packaged crate now contains the exact MIT license. | All eight `.factory/claims.json` commands passed from clean clone; `@claim:cli-demo`, `@claim:license-package`, `@claim:site-privacy` |
| F-2-3 | Split the 23-word README warning and replaced “secret leasing” with “credential time limit” in the landing label and accessible flow name. | `.factory/copy-audit.md`; `rg 'SECRET LEASING|Build with Rust 1.85'` has no visitor-copy match; live `/` |
| F-2-4 | Added route-specific Open Graph and Twitter metadata to the designed 404. | `the 404 route has complete route-specific social metadata`; live `/not-a-real-route` returned 404 with all fields in `.factory/evidence/polish-2-live/cold-browser-report.json` |

## Review 1 — severity findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| B1 | Job-first headline, named developer audience, sample CTA, outcome note, and three facts are present in both tested first screens. | First-screen Playwright tests; live desktop/mobile screenshots |
| B2 | `/demo/` and `?demo=1` enter isolated sample data; the banner persists; Reset and Start for real both clear the `demo:asc:` namespace. `asc demo` uses a new temporary directory. | `@claim:demo-isolation`; `@claim:cli-demo`; live demo screenshot |
| B3 | Added and completed the claims manifest. Unsupported claims were removed and eight observable claim commands now pass from a clean clone. | `.factory/claims.json`; clean-clone claim run recorded in handoff |
| B4 | Public scope says “selected process and its children”; expiry stops the process group. | `@claim:process-tree` |
| B5 | Removed the unavailable supporter checkout, license UI, price, and entitlement promises. | Live link crawl contains no billing URL; `live-accessibility-links.json` |
| B6 | Kept the Vite multi-page build, SWA 404 override, real `/demo/`, legal routes, robots, sitemap, and designed HTTP 404. | `real routes load, unknown routes return 404, and discovery assets exist`; live status crawl |
| M1 | Every page, now including 404, has its own title, description, canonical, favicon, touch icon, Open Graph, and Twitter metadata. | Per-route metadata/Axe tests; 404 metadata test; live cold report |
| M2 | Shared header/footer, Demo and legal links, build attribution, correct section order, route announcements, and Back-focus are retained. | Route test now asserts Privacy → Terms → Back focuses the Privacy h1; live `backFocus: true` |
| M3 | Re-audited landing and README copy; all sentences are at most 22 words and use the shared credential/alias/redact/receipt/time-limit terms. | `.factory/copy-audit.md` |
| M4 | Demo, footer, and legal controls retain at least 44 px height and the 390 px layout has no horizontal overflow. | `mobile controls meet the touch target and never overflow`; live targets are 169×44 px |

## Review 1 — copy findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| CW01 | Replaced the metaphor/banned adjective with “Give one agent command a temporary credential.” | Copy audit; live hero |
| CW02 | Added the developer/coding-agent audience and concrete result. | Copy audit; first-screen tests |
| CW03 | Removed the abstract “scope is the safety feature” sentence. | Live `/`; copy audit |
| CW04 | Corrected scope to the selected process and its children. | `@claim:process-tree` |
| CW05 | Uses “redact” and names supported forms in README. | `@claim:redaction-forms` |
| CW06 | Replaced “everything” with the precise no-value receipt claim. | `@claim:captured-output-receipt` |
| CW07 | Removed “lease” from visitor copy and now says “credential time limit.” | Copy audit; live `/` |
| CW08 | Removed capability jargon; the instructions consistently use “alias.” | Copy audit |
| CW09 | Replaced subprocess/TTL prose with process-and-children/time-limit wording. | Copy audit; `@claim:process-tree` |
| CW10 | Replaced the vague demo heading with result-first sample copy. | Live sample preview |
| CW11 | Replaced the old scripted illustration with the isolated fake-data demo. | `@claim:demo-isolation` |
| CW12 | Removed “exactly” and CLI-parity overclaiming from the web sample. | Live `/demo/`; copy audit |
| CW13 | Replaced “Run fake command” with “Try it with sample data” and “Run sample again.” | Live demo; demo-isolation test |
| CW14 | Copy control says “Copy install command.” | Live `/`; browser suite |
| CW15 | Split the long authorized-command warning into two short sentences. | Copy audit |
| CW16 | Removed the vague “safety layer” claim. | Live `/`; copy audit |
| CW17 | Split the README opening into three short job/result sentences. | README; copy audit |
| CW18 | Replaced the long audience sentence with two short instructions. | README; copy audit |
| CW19 | Split process scope, output capture, redaction forms, and receipt behavior. | README; three CLI claim tests |
| CW20 | Split the authorized-process warning into two sentences. | README; copy audit |
| CW21 | Uses “percent-encoded” rather than ambiguous “URL.” | README; `@claim:redaction-forms` |
| CW22 | Says redaction “limits output leaks,” not “is containment.” | Live security-limits section |
| CW23 | Removed the license-token form with the unavailable paid flow. | Live link/control crawl |
| CW24 | Uses the direct “This is not a sandbox” limitation. | README; live `/` |
| CW25 | Uses “selected process tree” and “time limit,” not exactly-one/lease language. | README; `@claim:process-tree` |
| CW26 | Names every supported form instead of “common exact encodings.” | README; `@claim:redaction-forms` |

## Review 1 — claim findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| U01 | Removed the prompt-absence promise from home metadata. | Built/live metadata; F-2-2 regression check |
| U02 | Removed the compressed platform/backend/telemetry fact line. | Live hero facts |
| U03 | Removed the unconditional keychain-at-rest landing claim; demo-specific non-use is now tested. | `@claim:cli-demo` |
| U04 | Corrected named-process scope to process and children. | `@claim:process-tree` |
| U05 | Kept only named redaction forms. | `@claim:redaction-forms` |
| U06 | Kept only the precise no-value receipt behavior. | `@claim:captured-output-receipt` |
| U07 | Removed capability and unseen-credential promise. | Live `/`; copy audit |
| U08 | Removed the alias-resolves-in-keychain landing claim. | Live `/` |
| U09 | Removed the prompt-content guarantee. | Home metadata and live copy |
| U10 | Corrected scope and duration language. | `@claim:process-tree` |
| U11 | Captured stdout and stderr remain a tested statement. | `@claim:captured-output-receipt` |
| U12 | Raw, percent, Base64, Base64url, and hex forms are tested. | `@claim:redaction-forms` |
| U13 | Removed the landing receipt-field inventory. | Live `/` |
| U14 | Credential omission remains tested. | `@claim:captured-output-receipt` |
| U15 | Implemented a fake value, separate storage, reset, exit cleanup, and same-origin request assertion. | `@claim:demo-isolation`; live cold demo report |
| U16 | Removed exact CLI-output parity wording. | Live `/demo/` |
| U17 | Removed daemon/account/cloud-vault/telemetry endpoint bundle. | Live `/` |
| U18 | Removed the visitor-facing Rust 1.85 claim. | README now says “Build from source.” |
| U19 | Retained the explicit “not a sandbox” limitation without presenting it as protection. | Live security-limits section |
| U20 | Kept the controlled warning in two short sentences. | Copy audit; process-tree behavior test |
| U21 | Removed “safety layer stays free.” | Live `/` |
| U22 | Removed ungated-feature marketing. | Live `/` |
| U23 | Removed price, team-kit, signed-release, and update promises. | Live link crawl |
| U24 | Removed merchant-of-record copy with the purchase flow. | Live `/`, `/terms/` |
| U25 | Removed refund/revocation copy with the purchase flow. | Live `/`, `/terms/` |
| U26 | Removed “Free core active” status. | Live `/` |
| U27 | Removed license-storage status and code. | Demo-isolation/localStorage assertions |
| U28 | No-value receipt remains tested. | `@claim:captured-output-receipt` |
| U29 | Split the broad README statement into process, stream, and receipt claims. | Three corresponding claim tests |
| U30 | Removed the broad “secrets stay” sentence; runtime process exposure is explicit. | README security limits |
| U31 | Removed the unlisted telemetry/hosted-store README promise. | README |
| U32 | Retained the honest non-sandbox limitation. | README security limits |
| U33 | Removed the visitor-facing compiler-version claim. | README now says “Build from source.” |
| U34 | Removed future binary-release promises. | README |
| U35 | Kept the package command as a verification instruction; it passes. | `cargo package -p agent-secret-capsule --allow-dirty`: 9 files, 88.9 KiB |
| U36 | Uses standard-input instructions without a shell-history guarantee. | README |
| U37 | Corrected to a selected process tree with a time limit. | `@claim:process-tree` |
| U38 | Split output/form/exit behavior into precise tested statements. | Redaction and captured-output tests |
| U39 | Receipt and JSON commands remain interface instructions covered by Rust/parser tests. | `npm test` |
| U40 | Remove remains an interface command covered through the black-box parser path. | `valid_alias_commands_reach_their_operational_json_paths_without_panicking` |
| U41 | Help remains an interface instruction; parser suite completes without panic. | Rust black-box parser tests |
| U42 | Non-TTY `--stdin` behavior remains direct interface guidance. | Rust black-box parser tests |
| U43 | Split into process-tree, redaction-form, and receipt claims. | Three corresponding claims |
| U44 | Receipt omission remains tested on serialized output. | `@claim:captured-output-receipt` |
| U45 | Kept the explicit authorized-process warning. | README security limits |
| U46 | Test-script description matches `package.json`; the clean-clone `npm test` run passed. | Clean-clone full suite |
| U47 | Added observable browser privacy coverage for landing, demo, and privacy routes. | `@claim:site-privacy`; live same-origin report |
| U48 | Removed license-token storage and verification code. | LocalStorage remains empty in demo/privacy tests |
| U49 | Added a package claim and ensured the published crate contains the exact MIT license. | `@claim:license-package`; package file list |
| U50 | Removed lifetime-update wording. | Live `/` |
| U51 | Replaced history guarantee with “STORE / STDIN INPUT.” | Live install section |
| U52 | Uses the precise “RECEIPTS / JSON” label. | Live install section |
| U53 | Removed team rollout-kit promise. | Live `/` |
| U54 | Removed receipt-policy entitlement promise. | Live `/` |
| U55 | Home description now uses credential/time-limit language without lease jargon. | Live home metadata |
| U56 | Uses “redact,” not “scrub,” in visitor copy. | Copy audit |
| U57 | Receipt omission is explicit and tested. | `@claim:captured-output-receipt` |

## Final evidence

- Clean clone: all eight claim commands passed independently; `npm test`
  passed 10 Rust, 2 Vitest, and 29 Playwright tests with five intentional
  duplicate-project skips.
- Local screenshots and verifier reports:
  `.factory/evidence/polish-2-local/` and
  `.factory/evidence/polish-2-local-demo/`.
- Live cold screenshots and reports:
  `.factory/evidence/polish-2-live/`,
  `.factory/evidence/polish-2-live-demo/`, and
  `.factory/evidence/polish-2-live-404/`.
- Live Axe pass: zero serious/critical findings on all five routes at 1440×900
  and 390×844; every crawled link returned 200.
- Live Lighthouse mobile: 100 performance, 100 accessibility, 100 best
  practices, 100 SEO; LCP 1.5 s, CLS 0, TBT 10 ms.
