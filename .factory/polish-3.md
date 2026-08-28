# Polish 3 — cumulative finding closure

Implementation commit: `d9737dd9b8bfaf20ccae35ab8fcbe9cc6d90de00`  
Deployment: `09283e34-d89d-42b4-91f0-414ed0641b9d`  
Live site: <https://agent-secret-capsule.sociobot.in/>

This is the final ledger for review 3 plus every finding in reviews 1–2 and
polishes 1–2. “Removed” means absent from current visitor copy, metadata, and
controls; it is not merely hidden. All live URLs below were opened in fresh
browser contexts after deployment.

## Round-three findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-3-1 / B2 | Rebuilt `/demo/` around a realistic bundled read-only `api-gateway` deployment-status fixture. The compact initial workbench now shows redacted stdout/stderr, the 30ms expiry result, alias, and outcome. The web preview is checked against `asc demo`. | `the populated demo shows redaction, expiry, and receipt evidence in the initial viewport`; `@claim:demo-parity`; [live desktop](https://agent-secret-capsule.sociobot.in/demo/) and `.factory/evidence/polish-3-live/demo-desktop.png`; [live mobile](https://agent-secret-capsule.sociobot.in/demo/) and `.factory/evidence/polish-3-live/demo-mobile.png`. |
| F-3-2 / B3 / U16 / U36 / U39–U42 | Added seven missing manifest claims and compiled-CLI black-box tests: credential lifecycle, every named redaction form, process-tree expiry, captured streams/receipt, receipt commands, receipt storage schema, and help/non-TTY behavior. Added browser/CLI demo-parity coverage. | All 13 `.factory/claims.json` commands passed independently in clean clone `/tmp/asc-clean-WHfeRN`; `crates/asc/tests/cli_claims.rs`; [live demo](https://agent-secret-capsule.sociobot.in/demo/). |
| F-3-3 | Replaced the mobile facts with the three tested facts and tightened the phone hero. All are visible before 844px. | `mobile controls meet the touch target and never overflow`; `.factory/evidence/polish-3-live/home-mobile.png`; live `/` boxes end at 627, 650, and 673px. |

## Review-two findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1 / B1 | Retained the job-first headline, named audience, and one-click CTA; the revised mobile spacing still preserves them. | `desktop first screen shows its audience and sample action`; `.factory/evidence/polish-3-live/home-desktop.png`; live `/` audience/CTA end at 657/736px. |
| F-2-2 | Removed unsupported prompt/compiler promises. `cli-demo`, `license-package`, and `site-privacy` are listed and tested; all newly retained CLI behaviors now have claim entries too. | Clean-clone claim run; `.factory/claims.json`; [live privacy](https://agent-secret-capsule.sociobot.in/privacy/). |
| F-2-3 / CW07 / CW15 | Uses “time limit,” not lease jargon; README warnings remain split and under 22 words. | `.factory/copy-audit.md`; [live home](https://agent-secret-capsule.sociobot.in/). |
| F-2-4 / M1 | Kept route-specific 404 social metadata. | Live `/not-a-real-route`: HTTP 404, one h1, canonical, description, all OG/Twitter fields; `the 404 route has complete route-specific social metadata`. |

## Review-one severity and minor findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| B1 | Plain job headline, named developers, visible sample action, outcome note, and tested fact strip remain on the first screen. | `desktop first screen shows its audience and sample action`; live `/`. |
| B2 | Real `/demo/` and `?demo=1` sandbox, persistent banner, Reset, Start for real, separate `demo:asc:` storage, real `asc demo`, bundled fixtures, and immediate populated result. | `@claim:demo-isolation`, `@claim:cli-demo`, `@claim:demo-parity`; live `/demo/`. |
| B3 | Complete 13-entry observable claims manifest with one tagged test per claim. | `.factory/claims.json`; clean-clone independent commands. |
| B4 | Public process scope is “selected process and its children”; the CLI expiry test exercises a real child and time limit. | `@claim:process-tree`. |
| B5 | Unsupported checkout, license storage, price, and entitlement controls remain absent. | [live home](https://agent-secret-capsule.sociobot.in/); live link crawl. |
| B6 | Multi-page `/`, `/demo/`, `/privacy/`, `/terms/`, discovery assets, SWA 404 override, and designed HTTP 404 remain real routes. | `real routes load, unknown routes return 404, and discovery assets exist`; live route checks. |
| M1 | Every route has title, description, canonical, favicon/touch icon, OG/Twitter data; 404 was rechecked live. | Route metadata/a11y tests; live 404 metadata check. |
| M2 | Shared skeleton, legal links, route focus, announcements, and Back focus remain intact. | Live Privacy → Terms → Back h1-focus check; `real routes load…`. |
| M3 | Re-audited landing, demo, and README words with claim mappings. | `.factory/copy-audit.md`. |
| M4 | Phone controls remain 44px minimum with no horizontal overflow. | `mobile controls meet the touch target and never overflow`; live mobile screenshots. |

## Review-one copy findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| CW01 | Uses “Give one agent command a temporary credential.” | Copy audit; live `/`. |
| CW02 | Names developers running coding agents and their result. | Copy audit; first-screen tests. |
| CW03 | Abstract scope slogan remains removed. | Copy audit; live `/`. |
| CW04 | Scope names the selected process and children. | `@claim:process-tree`. |
| CW05 | README names the exact redaction forms. | `@claim:redaction-forms`. |
| CW06 | Receipt wording is precise: it omits the credential. | `@claim:captured-output-receipt`. |
| CW07 | Uses credential time limit consistently. | Copy audit; live `/`. |
| CW08 | Uses alias instead of capability jargon. | Copy audit. |
| CW09 | Uses process-and-children/time-limit wording. | `@claim:process-tree`. |
| CW10 | Demo heading states the observable deployment-check result. | Live `/demo/`; demo screenshots. |
| CW11 | Replaced the scripted illustration with the isolated fake-data demo. | `@claim:demo-isolation`; live `/demo/`. |
| CW12 | Replaced the static parity overclaim with a real CLI/browser parity test. | `@claim:demo-parity`. |
| CW13 | Keeps result-naming actions: sample, reset, start, rerun. | Live `/demo/`; browser suite. |
| CW14 | Keeps “Copy install command.” | Browser suite; live `/`. |
| CW15 | Keeps the authorized-command warning in two short sentences. | Copy audit. |
| CW16 | Vague “safety layer” claim remains removed. | Copy audit; live `/`. |
| CW17 | README opening remains split into clear job/result sentences. | Copy audit. |
| CW18 | README audience/instructions remain short and direct. | Copy audit. |
| CW19 | README separates scope, streams, forms, and receipt behavior. | CLI claim suite. |
| CW20 | README authorized-command warning remains split. | Copy audit. |
| CW21 | Uses “percent-encoded,” not ambiguous “URL.” | `@claim:redaction-forms`. |
| CW22 | Says redaction limits output leaks, not that it is a sandbox. | Live security-limits section. |
| CW23 | Unavailable license form remains absent. | Live link crawl. |
| CW24 | Keeps the explicit “not a sandbox” limitation. | Live `/`; README. |
| CW25 | Uses selected process tree/time limit. | `@claim:process-tree`. |
| CW26 | Names raw, percent, Base64, Base64url, and hex forms. | `@claim:redaction-forms`. |

## Review-one claim findings

| Finding | Change made | Evidence |
| --- | --- | --- |
| U01 | Removed prompt/traces guarantee. | Copy audit; live metadata. |
| U02 | Removed bundled platform/backend/telemetry fact. | Live first-screen facts. |
| U03 | Removed unconditional keychain-at-rest visitor claim. | Copy audit; `@claim:cli-demo` only states demo isolation. |
| U04 | Corrected scope to process and children. | `@claim:process-tree`. |
| U05 | Retained only exact named redaction forms. | `@claim:redaction-forms`. |
| U06 | Retained only precise no-value receipt behavior. | `@claim:captured-output-receipt`. |
| U07 | Removed capability/prompt promise. | Copy audit. |
| U08 | Removed keychain-resolution promise. | Copy audit. |
| U09 | Removed prompt-content promise. | Copy audit. |
| U10 | Corrected scope and duration language. | `@claim:process-tree`. |
| U11 | Captured stdout and stderr are now black-box tested. | `@claim:captured-output-receipt`. |
| U12 | Every named encoded form is emitted on both streams and redacted. | `@claim:redaction-forms`. |
| U13 | Receipt field inventory is now a private-policy claim with an exact schema test. | `@claim:receipt-storage-schema`. |
| U14 | Receipt omission is persisted-file tested. | `@claim:captured-output-receipt`. |
| U15 | Real fake-value demo storage/reset/request isolation remains tested. | `@claim:demo-isolation`; live `/demo/`. |
| U16 | Browser and CLI samples are compared rather than merely described as equivalent. | `@claim:demo-parity`. |
| U17 | Removed daemon/account/cloud-vault/telemetry bundle. | Copy audit. |
| U18 | Removed visitor-facing Rust-version claim. | README/copy audit. |
| U19 | Keeps explicit non-sandbox limitation. | Live `/`; README. |
| U20 | Keeps short authorized-process boundary warning. | Copy audit. |
| U21 | Removed vague free-tier claim. | Live `/`. |
| U22 | Removed ungated-feature marketing. | Live `/`. |
| U23 | Removed price, kit, signed-release, and update promises. | Live link crawl. |
| U24 | Removed merchant-of-record copy. | Live `/` and `/terms/`. |
| U25 | Removed refund/revocation promise. | Live `/` and `/terms/`. |
| U26 | Removed free-core status. | Live `/`. |
| U27 | Removed license-storage status/code. | `@claim:demo-isolation`; live browser storage check. |
| U28 | Kept no-value receipt claim with a persisted-file test. | `@claim:captured-output-receipt`. |
| U29 | Split broad README behavior into scope/output/receipt claims. | `process-tree`, `redaction-forms`, `captured-output-receipt`. |
| U30 | Removed broad OS-keychain-at-rest promise; runtime boundary is explicit. | README security limits. |
| U31 | Removed hosted-store/telemetry CLI claim. | README/copy audit. |
| U32 | Keeps honest containment limitation. | README and live limits. |
| U33 | Removed compiler-version promise. | README/copy audit. |
| U34 | Removed future binary-release promise. | README/copy audit. |
| U35 | Packaging instruction remains verified. | Clean clone `cargo package -p agent-secret-capsule --allow-dirty --list`. |
| U36 | Added real put/list/run/remove lifecycle claim using an isolated test keychain. | `@claim:credential-lifecycle`. |
| U37 | Corrected to process tree/time limit. | `@claim:process-tree`. |
| U38 | Output forms, streams, and exit behavior are tested through the compiled CLI. | `@claim:redaction-forms`, `@claim:captured-output-receipt`. |
| U39 | Added human/JSON receipts command, count, order, and no-value claim test. | `@claim:receipt-commands`. |
| U40 | Added successful remove behavior to lifecycle test. | `@claim:credential-lifecycle`. |
| U41 | Added every help path, root exit-code, and example check. | `@claim:cli-interface`. |
| U42 | Added non-TTY rejection check. | `@claim:cli-interface`. |
| U43 | Tests selected tree, expiry, forms, streams, and receipt through CLI claims. | CLI claim suite. |
| U44 | Inspects persisted receipt log for credential omission. | `@claim:captured-output-receipt`. |
| U45 | Keeps explicit authorized-process warning. | README security limits. |
| U46 | Current scripts are verified from fresh clone. | Clean clone `npm test` (32 Playwright pass, 6 skipped). |
| U47 | Browser privacy claim records requests/cookies/storage. | `@claim:site-privacy`; live privacy check. |
| U48 | License storage/verification code remains absent. | Browser storage checks; live `/`. |
| U49 | Source/package MIT claim includes package contents and fixture. | `@claim:license-package`. |
| U50 | Removed lifetime-update wording. | Live `/`. |
| U51 | Uses `STORE / STDIN INPUT`; no shell-history guarantee. | Live install section. |
| U52 | Uses `RECEIPTS / JSON`; receipt command is tested. | `@claim:receipt-commands`. |
| U53 | Removed team-rollout entitlement. | Live `/`. |
| U54 | Removed retention-policy entitlement. | Live `/`. |
| U55 | Home metadata uses credential/time-limit language. | Live `/` metadata test. |
| U56 | Uses “redact,” not “scrub.” | Copy audit. |
| U57 | Keeps exact no-value receipt claim. | `@claim:captured-output-receipt`. |

## Verification summary

- Fresh clone `/tmp/asc-clean-WHfeRN`: `npm ci`, every manifest command run
  independently, `npm test`, `npm run build`, and `cargo package … --list` all
  passed. The full suite reported 10 Rust tests, 2 Vitest tests, and 32
  Playwright passes with 6 intentional cross-project skips.
- Local `verify-url.sh` passed `/`, `/demo/`, `/privacy/`, and `/terms/` with
  no console errors. Local Axe integration found zero serious/critical issues.
- Local mobile Lighthouse: performance 100, accessibility 100, best practices
  100, SEO 100; LCP 1,825ms, CLS 0, TBT 0. Report:
  `.factory/evidence/polish-3-local/lighthouse.json`.
- Live `verify-url.sh` passed the same four routes. Cold live Axe scans had no
  serious/critical findings. The live link crawl returned 200 for every link;
  the unknown route returned 404 with complete metadata.
- The distinct concrete-and-moss visual direction is retained. The live
  screenshots above show the mechanical moss controls, type pairing, and
  containment-led layout rather than a generic dashboard.
