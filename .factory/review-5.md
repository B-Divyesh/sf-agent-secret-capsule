# Adversarial first-read review 5 — PASS

**Product:** Agent Secret Capsule  
**Live URL:** <https://agent-secret-capsule.sociobot.in/>  
**Repository revision:** `29e7687d192ac4a8ab761ffb497cfb248bf07579`  
**Review date:** 2026-08-28 UTC  
**Viewports:** fresh Chromium contexts at 390×844 and 1440×900

## Verdict

**PASS.** No blocking, major, minor, copy, claim, demo, routing,
accessibility, privacy, or missed-leverage finding remains. There is no
untested public claim. No `F-5-k` identifier is assigned because this review
found nothing to fix.

The live Home, Demo, Privacy, Terms, and 404 documents have the same SHA-256
values as the clean build at the reviewed revision.

## Cold first read

Before scrolling, both viewports answer all three questions:

| Question | First-time answer |
| --- | --- |
| What does this do? | A local CLI gives one coding-agent command and its children a temporary credential, redacts output, and writes a receipt without the value. |
| For whom? | Developers whose coding agents need to make an authorized call. |
| What should I click first? | **Try it with sample data.** |

Exact first-screen copy:

> “Give one agent command a temporary credential.”
>
> “For developers running coding agents: run one command, redact its output,
> and save a receipt without the credential.”
>
> “Try it with sample data”
>
> “See a fake credential redacted and a no-value receipt.”

At 390×844, the audience, action, outcome note, and final fact end at y=471,
543, 594, and 673. At 1440×900, the audience, action, and final fact end at
y=657, 736, and 803. Both pages have no horizontal overflow or console errors.

## Complete copy audit

Counts are whitespace-delimited; hyphenated compounds count as one word.
Commands, paths, and field labels are listed separately when they are not
sentences. No sentence exceeds 22 words. No banned word, inconsistent term,
uninformative metaphor, vague heading, or non-result-naming action remains.

### Landing-page sentences

| ID | Location and exact sentence | Words | Result |
| --- | --- | ---: | --- |
| L01 | Description: “Run one coding-agent command with a temporary credential, redact its output, and save a no-value receipt.” | 16 | Clear; `process-tree`, `captured-output-receipt` |
| L02 | OG description: “A local CLI for running one selected command with a temporary credential.” | 12 | Clear; `process-tree` |
| L03 | Twitter description: “A local CLI for running one selected command with a temporary credential.” | 12 | Clear; `process-tree` |
| L04 | “Give one agent command a temporary credential.” | 7 | Clear; `process-tree` |
| L05 | “For developers running coding agents: run one command, redact its output, and save a receipt without the credential.” | 18 | Clear; names user and result |
| L06 | “See a fake credential redacted and a no-value receipt.” | 9 | Clear; `cli-demo` |
| L07 | Image alt: “A monolithic concrete capsule divided by one narrow seam of living moss” | 12 | Clear purpose description |
| L08 | “The selected process and its children receive one credential.” | 9 | Clear; `process-tree` |
| L09 | “See the command result first.” | 5 | Clear sample heading |
| L10 | “The CLI ships with `asc demo`.” | 6 | `cli-demo` |
| L11 | “It creates fake sample receipts in a new temporary directory.” | 10 | `cli-demo` |
| L12 | “Select.” | 1 | Clear step heading |
| L13 | “Run.” | 1 | Clear step heading |
| L14 | “Redact.” | 1 | Clear step heading |
| L15 | “Review the receipt.” | 3 | Clear step heading |
| L16 | “Use a local alias in the agent tool input.” | 9 | Clear instruction |
| L17 | “Store an alias locally.” | 4 | `credential-lifecycle` |
| L18 | “Use the alias in the agent tool input.” | 8 | Clear instruction |
| L19 | “The selected process and its children receive the credential until exit or the time limit.” | 15 | `process-tree` |
| L20 | “ASC captures both output streams and replaces matching credential forms before printing them.” | 13 | `redaction-forms`, `captured-output-receipt` |
| L21 | “A no-value receipt omits the credential.” | 6 | `captured-output-receipt` |
| L22 | “Build the CLI.” | 3 | Direct instruction; build passed |
| L23 | “Run the sample.” | 3 | `cli-demo` |
| L24 | “Use the bundled demo before storing a real credential.” | 9 | `cli-demo` |
| L25 | “Redaction limits output leaks.” | 4 | Honest bounded description |
| L26 | “It is not a sandbox.” | 5 | Explicit limitation |
| L27 | “An authorized command can send the credential over the network or write it to a file.” | 16 | Explicit limitation |
| L28 | “It can also transform it or pass it to a child.” | 11 | Explicit limitation |
| L29 | “Review the command and endpoint.” | 5 | Clear instruction |
| L30 | “Use a separate network and process sandbox for hostile code.” | 10 | Clear instruction |
| L31 | “One command.” | 2 | Informative footer fragment |
| L32 | “One no-value receipt.” | 3 | `captured-output-receipt` |

### Landing headings, facts, labels, and actions

| Exact copy | Words | Result |
| --- | ---: | --- |
| CLI FOR CODING AGENTS | 4 | Names product type and audience |
| Try it with sample data | 5 | Result-naming primary action |
| No analytics or third-party scripts | 5 | `site-privacy` |
| Demo works offline after first visit | 6 | `offline-reload` |
| Free and open source | 4 | `license-package` |
| SAMPLE RUN | 2 | Names the preview |
| Open the sample run | 4 | Result-naming action |
| HOW THE CREDENTIAL TIME LIMIT WORKS | 6 | Names the mechanism |
| Select / Run / Redact / Review receipt | 1 / 1 / 1 / 2 | Clear step labels |
| INSTALL / v0.1.0 | 2 | Names section and version |
| Copy install command | 3 | Result-naming action |
| STORE / STDIN INPUT | 3 | Clear usage label |
| RUN / SET A TIME LIMIT | 5 | Clear usage label |
| RECEIPTS / JSON | 2 | Clear usage label |
| SECURITY LIMITS | 2 | Clear section label |
| Read the CLI reference on GitHub (external) | 7 | Clear destination and result |
| Source on GitHub (external) | 5 | Clear external destination |
| Built by Param Factory · v0.1.0 | 5 | Clear provenance and version |

### README sentences

| ID | Exact sentence | Words | Result |
| --- | --- | ---: | --- |
| R01 | “Agent Secret Capsule (`asc`) gives one selected process and its children a temporary credential.” | 14 | `process-tree` |
| R02 | “It captures command output before printing it.” | 7 | `captured-output-receipt` |
| R03 | “It writes a receipt without the credential value.” | 8 | `captured-output-receipt` |
| R04 | “For developers whose coding agents need an authorized API call.” | 10 | Clear audience statement |
| R05 | “Use a local alias in the agent tool input.” | 9 | Clear instruction |
| R06 | “Run the bundled sample before storing a real credential.” | 9 | `cli-demo` |
| R07 | “The command checks a bundled fake deployment-status fixture.” | 8 | `cli-demo` |
| R08 | “It uses a fake credential.” | 5 | `cli-demo` |
| R09 | “It creates a new temporary directory with sample no-value receipts and prints its path.” | 14 | `cli-demo` |
| R10 | “It does not read your keychain or `ASC_HOME`.” | 8 | `cli-demo` |
| R11 | “Delete that directory to reset the command-line sample.” | 8 | Clear instruction |
| R12 | “Try the web sample.” | 4 | Linked absolute demo URL; `demo-isolation` |
| R13 | “It uses browser storage keys with the `demo:asc` prefix.” | 9 | `demo-isolation` |
| R14 | “Reset demo clears those sample keys.” | 6 | `demo-isolation` |
| R15 | “Build from source.” | 3 | Direct instruction; build passed |
| R16 | “Store a credential from standard input.” | 6 | `credential-lifecycle` |
| R17 | “Run a selected process tree with a time limit.” | 9 | `process-tree` |
| R18 | “Inspect receipts or automate with JSON.” | 6 | `receipt-commands` |
| R19 | “Run `asc --help` for commands and exit codes.” | 8 | `cli-interface` |
| R20 | “Run `asc <command> --help` for flags and examples.” | 8 | `cli-interface` |
| R21 | “When standard input is not a terminal, `put` requires `--stdin`.” | 10 | `cli-interface` |
| R22 | “ASC gives the credential to the selected process and its children until exit or the time limit.” | 17 | `process-tree` |
| R23 | “It redacts raw, percent-encoded, Base64, Base64url, and hex matches from captured stdout and stderr.” | 14 | `redaction-forms` |
| R24 | “A no-value receipt omits the credential value.” | 7 | `captured-output-receipt` |
| R25 | “This is not a sandbox.” | 5 | Explicit limitation |
| R26 | “An authorized process can send the credential over the network or write it to a file.” | 16 | Explicit limitation |
| R27 | “It can also transform the credential or pass it to a child.” | 12 | Explicit limitation |
| R28 | “Review the exact command and endpoint.” | 6 | Clear instruction |
| R29 | “Use a separate sandbox for hostile code.” | 7 | Clear instruction |
| R30 | “The command writes `dist/site` for deployment.” | 6 | `build-output` |
| R31 | “MIT.” | 1 | `license-package` |

README headings—**Try the sample**, **Install**, **Usage**, **Security limits**,
**Develop and verify**, and **License**—name their sections. README has no
button copy. The terminology is consistent: `credential`, `alias`, `selected
process and its children`, `redact`, `no-value receipt`, and `time limit`.

No live landing, demo, privacy, terms, metadata, or README claim-like sentence
is absent from `.factory/claims.json`. Instructions and security limitations
are not presented as guarantees.

## Demo and sandbox verification

| Check | Result |
| --- | --- |
| One-click entry | PASS — the Home action opens `/demo/` in one click |
| Direct entry | PASS — `/?demo=1` redirects to `/demo/` |
| Immediate product use | PASS — both initial viewports show the `api-gateway` production result, two redactions, expiry result, alias, outcome, and omitted credential |
| Demo banner | PASS — “Demo — sample data, nothing is saved,” Reset demo, and Start for real remain visible |
| Browser namespace | PASS — rerun creates only `sessionStorage["demo:asc:run-count"]` |
| Reset | PASS — clears the demo key and restores `READY` |
| Real-data isolation | PASS — seeded `real:sentinel` local/session keys survive Reset and Start for real |
| Requests and cookies | PASS — all observed requests are same-origin; no cookies, console errors, or page errors |
| Offline | PASS — after the first visit, `/demo/` reloads offline with the sample result |
| CLI demo | PASS — `asc demo` created a separate `/tmp/asc-demo-*` directory, redacted both streams, wrote two mode-0600 no-value receipts, and left sentinel `ASC_HOME` untouched |

The mobile redactions end at y=530 and y=548. Receipt alias, outcome, redaction
count, and omitted value end at y=715, 749, 782, and 841, within the 844px
viewport. The desktop result and receipt end well within 900px.

## Claims verification from a clean clone

Clean clone: `/tmp/asc-review5-clean.YNYrEU/repo` at the reviewed revision.
`npm ci` completed with zero vulnerabilities. Every manifest command was run
independently.

| Claim | Declared command | Result |
| --- | --- | --- |
| `demo-isolation` | `npm run test:claim -- --grep @claim:demo-isolation` | PASS, 2 projects |
| `offline-reload` | `npm run test:claim -- --grep @claim:offline-reload` | PASS, 2 projects |
| `cli-demo` | `npm run test:claim -- --grep @claim:cli-demo` | PASS, 1 project / 1 intentional skip |
| `redaction-forms` | `cargo test --locked --features test-keyring --test cli_claims claim_redaction_forms_removes_every_named_form_from_compiled_cli_output` | PASS |
| `process-tree` | `cargo test --locked --features test-keyring --test cli_claims claim_process_tree_uses_the_documented_cli_and_stops_at_its_time_limit` | PASS |
| `captured-output-receipt` | `cargo test --locked --features test-keyring --test cli_claims claim_captured_output_and_receipt_omit_the_credential` | PASS |
| `credential-lifecycle` | `cargo test --locked --features test-keyring --test cli_claims claim_credential_lifecycle_stores_lists_runs_and_removes_an_alias` | PASS |
| `receipt-commands` | `cargo test --locked --features test-keyring --test cli_claims claim_receipt_commands_return_newest_first_human_and_json_no_value_results` | PASS |
| `receipt-storage-schema` | `cargo test --locked --features test-keyring --test cli_claims claim_receipt_storage_schema_is_private_and_contains_only_declared_metadata` | PASS |
| `cli-interface` | `cargo test --locked --features test-keyring --test cli_claims claim_cli_interface_help_and_non_tty_input_behave_as_documented` | PASS |
| `demo-parity` | `npm run test:claim -- --grep @claim:demo-parity` | PASS, 1 project / 1 intentional skip |
| `license-package` | `npm run test:claim -- --grep @claim:license-package` | PASS, 1 project / 1 intentional skip |
| `site-privacy` | `npm run test:claim -- --grep @claim:site-privacy` | PASS, 2 projects |
| `build-output` | `npm run test:claim -- --grep @claim:build-output` | PASS, 2 projects |

No listed claim failed and no public claim is untested.

## Structure, accessibility, privacy, and visual identity

- `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` have route-specific
  titles, descriptions, canonicals, OG/Twitter metadata, favicon/touch icon,
  `lang=en`, one h1, and one main landmark.
- `/not-a-real-route` returns HTTP 404 with the designed 404. `robots.txt`,
  `sitemap.xml`, SVG favicon, touch icon, and 1200×630 social image return 200
  with the correct content types.
- Every site link returns 200. Home anchors resolve. Both GitHub destinations
  return 200 and identify themselves as external.
- Direct route loads focus and announce the h1. Privacy → Terms → Back restores
  Privacy h1 focus. Demo → Home focuses and announces the Home h1.
- The shared header, footer, skip link, legal links, Param Factory attribution,
  and version are present on every route.
- Live Axe found zero serious/critical violations on all five routes at 390px.
  The full Playwright suite checks both configured viewports. `verify-url.sh`
  passed Home, Demo, Privacy, and Terms with no console errors, missing alt text,
  or unlabeled buttons.
- Touch targets, focus styles, heading order, reduced motion, and 390px overflow
  checks pass. Built JavaScript chunks are each at most 1.22 kB uncompressed.
- The concrete slab, moss seam, serif/mono pairing, stamped labels, square
  controls, and restrained state motion match `.factory/design.md`. This is a
  distinct containment-tool identity, not a generic SaaS template.
- No analytics, advertising cookie, CDN script/font, runtime AI call, embedded
  provider key, payment flow, or decorative AI feature is present.

## Earlier-finding verification

Every earlier `review-*.md`, `polish-*.md`, and the prior handoff was read. Each
finding was checked against the live site and current code, not merely its prior
closure label.

### Severity and round findings

| Earlier ID | Status | Current evidence |
| --- | --- | --- |
| B1 | Fixed | Both first screens show job, audience, action, outcome, and three facts. |
| B2 | Fixed | Web/CLI demos, immediate result, banner, Reset, exit, namespace, and real-data isolation pass. |
| B3 | Fixed | All 14 public claims are listed and independently pass. |
| B4 | Fixed | Copy and compiled-CLI test use the selected process and its children. |
| B5 | Fixed | No paid flow, checkout, license form, price, or billing link remains. |
| B6 | Fixed | Real routes and discovery files load; unknown paths return the designed HTTP 404. |
| M1 | Fixed | Every route has complete, route-specific metadata and discovery assets. |
| M2 | Fixed | Shared shell, correct order, route focus/announcement, and Back behavior pass. |
| M3 | Fixed | Current copy audit has no length, jargon, metaphor, slogan, or terminology failure. |
| M4 | Fixed | Mobile controls meet 44px and no 390px layout overflows. |
| F-2-1 | Fixed | Desktop audience and action end at y=657 and y=736. |
| F-2-2 | Fixed | Prompt/compiler claims remain removed; demo, license, and site-privacy claims pass. |
| F-2-3 | Fixed | Warning is split; visitor copy uses “time limit,” not “lease.” |
| F-2-4 | Fixed | The 404 has route-specific OG and Twitter metadata. |
| F-3-1 | Fixed | Both demo first screens show realistic redaction, expiry, alias, outcome, and omission. |
| F-3-2 | Fixed | Lifecycle, receipts, schema, interface, and parity claims all pass through the compiled CLI/browser. |
| F-3-3 | Fixed | All three tested facts end above y=844. |
| F-4-1 | Fixed | Demo → Home focuses and announces the Home h1. |
| F-4-2 | Fixed | Obsolete README test-composition claims are absent; `build-output` is listed and passes. |
| F-4-3 | Fixed | Literal process caption and “This page does not exist” replace both metaphors. |
| F-4-4 | Fixed | GitHub links visibly say “GitHub (external).” |
| F-4-5 | Fixed | README supplies a clickable absolute web-demo URL. |

### Review-1 copy findings

| Earlier ID | Status and current evidence |
| --- | --- |
| CW01 | Fixed — seven-word job headline; “powerful” is absent. |
| CW02 | Fixed — the developer/coding-agent audience is explicit. |
| CW03 | Fixed — the abstract scope slogan remains absent. |
| CW04 | Fixed — scope includes the selected process and children. |
| CW05 | Fixed — copy says “redact” and names supported forms. |
| CW06 | Fixed — receipt wording precisely says the credential is omitted. |
| CW07 | Fixed — visitor copy consistently uses “time limit.” |
| CW08 | Fixed — instructions use “alias,” not “capability.” |
| CW09 | Fixed — process-and-children/time-limit wording is literal. |
| CW10 | Fixed — sample heading names the command result. |
| CW11 | Fixed — isolated fake-data demo replaced the illustration. |
| CW12 | Fixed — browser/CLI parity has an observable passing test. |
| CW13 | Fixed — sample, reset, exit, and rerun actions name results. |
| CW14 | Fixed — control says “Copy install command.” |
| CW15 | Fixed — authorized-command warning is split below 22 words. |
| CW16 | Fixed — vague “safety layer” copy remains absent. |
| CW17 | Fixed — README opening is three short job/result sentences. |
| CW18 | Fixed — README audience and instruction are short and direct. |
| CW19 | Fixed — scope, streams, forms, and receipt behavior are separate. |
| CW20 | Fixed — README authorized-process warning is split. |
| CW21 | Fixed — “percent-encoded” replaces ambiguous “URL.” |
| CW22 | Fixed — copy says redaction limits leaks and is not a sandbox. |
| CW23 | Fixed — unsupported license form remains absent. |
| CW24 | Fixed — the non-sandbox limitation is direct. |
| CW25 | Fixed — copy uses selected process tree/time limit. |
| CW26 | Fixed — all supported credential forms are named. |

### Review-1 claim findings

| Earlier ID | Status and current evidence |
| --- | --- |
| U01 | Fixed — prompt/tool-trace guarantee remains removed. |
| U02 | Fixed — platform/backend/telemetry bundle remains removed. |
| U03 | Fixed — unconditional keychain-at-rest claim remains removed; demo non-use passes. |
| U04 | Fixed — corrected process-and-children claim passes. |
| U05 | Fixed — named redaction forms pass. |
| U06 | Fixed — precise no-value receipt behavior passes. |
| U07 | Fixed — capability/prompt promise remains absent. |
| U08 | Fixed — alias/keychain-resolution promise remains absent. |
| U09 | Fixed — prompt-content promise remains absent. |
| U10 | Fixed — corrected scope and time-limit language pass. |
| U11 | Fixed — both captured streams are tested. |
| U12 | Fixed — raw, percent, Base64, Base64url, and hex forms pass. |
| U13 | Fixed — exact receipt metadata schema is listed and tested. |
| U14 | Fixed — persisted receipts omit the credential. |
| U15 | Fixed — fake data, namespace, reset, exit, and request isolation pass. |
| U16 | Fixed — browser/CLI sample parity passes. |
| U17 | Fixed — daemon/account/cloud-vault/telemetry bundle remains absent. |
| U18 | Fixed — visitor-facing compiler-version promise remains absent. |
| U19 | Fixed — retained only as an honest non-sandbox limitation. |
| U20 | Fixed — retained only as an honest authorized-command warning. |
| U21 | Fixed — vague free-tier claim remains absent. |
| U22 | Fixed — ungated-feature marketing remains absent. |
| U23 | Fixed — price, kit, release, and update promises remain absent. |
| U24 | Fixed — merchant-of-record copy remains absent. |
| U25 | Fixed — refund/revocation promise remains absent. |
| U26 | Fixed — “Free core active” remains absent. |
| U27 | Fixed — license-storage status/code remains absent. |
| U28 | Fixed — no-value receipt behavior passes. |
| U29 | Fixed — broad README promise is split across passing scope/output/receipt claims. |
| U30 | Fixed — broad keychain-at-rest statement remains absent. |
| U31 | Fixed — hosted-store/CLI-telemetry statement remains absent. |
| U32 | Fixed — retained only as an explicit limitation. |
| U33 | Fixed — compiler-version claim remains absent. |
| U34 | Fixed — future binary-release promise remains absent. |
| U35 | Fixed — package command and contents pass under `license-package`. |
| U36 | Fixed — successful put/list/run/remove lifecycle passes. |
| U37 | Fixed — compiled CLI process-tree expiry passes. |
| U38 | Fixed — streams, forms, receipt omission, and exit behavior pass. |
| U39 | Fixed — human/JSON receipt commands, order, and omission pass. |
| U40 | Fixed — successful remove behavior passes. |
| U41 | Fixed — documented help paths, examples, and exit information pass. |
| U42 | Fixed — non-TTY input rejection passes. |
| U43 | Fixed — process, expiry, forms, streams, and receipt paths pass. |
| U44 | Fixed — persisted receipt log omits the credential. |
| U45 | Fixed — authorized-process warning remains explicit. |
| U46 | Fixed — obsolete suite-composition sentence is absent; full suite passes. |
| U47 | Fixed — browser request/cookie/script privacy claim passes. |
| U48 | Fixed — license storage and verification feature remain absent. |
| U49 | Fixed — source and packaged CLI use the MIT License. |
| U50 | Fixed — lifetime-update wording remains absent. |
| U51 | Fixed — label is “STORE / STDIN INPUT”; no history guarantee remains. |
| U52 | Fixed — label is “RECEIPTS / JSON”; receipt commands pass. |
| U53 | Fixed — team-rollout entitlement remains absent. |
| U54 | Fixed — receipt-policy entitlement remains absent. |
| U55 | Fixed — Home metadata uses credential wording without lease jargon. |
| U56 | Fixed — visitor copy uses “redact,” not “scrub.” |
| U57 | Fixed — receipt omission is explicit and tested. |

The earlier parser panic remains fixed by compiled-binary parser and lifecycle
tests. The cache-header repair remains present. The normal CLI demo continues
to avoid the keychain and `ASC_HOME`.

## Quality gates

```text
npm ci                 PASS (60 packages, 0 vulnerabilities)
14 manifest commands   PASS independently
npm test               PASS (10 Rust, 2 Vitest, 38 Playwright; 6 intentional skips)
npm run build          PASS (release CLI and dist/site)
cargo fmt --check      PASS
cargo clippy ...       PASS with -D warnings
cargo package ...      PASS; expected package file list
verify-url.sh          PASS on /, /demo/, /privacy/, /terms/
live Axe               0 serious/critical findings on five mobile routes
live link crawl        every link 200; unknown route HTTP 404
```

## Missed leverage

No finding. The brief calls for a narrow local credential boundary. Sending
credential context to an AI gateway or adding sync would widen that boundary
without improving the core job. Human and JSON receipts already provide the
expected export path. No decorative AI feature or embedded provider key exists.

## What would make this perfect

Nothing remains to change or test for this review. Preserve the current
first-screen clarity, isolated web and CLI samples, exhaustive claim manifest,
literal copy, route behavior, and concrete-and-moss identity in future changes.
