# Adversarial first-read review 4 — FAIL

**Product:** Agent Secret Capsule

**Live URL:** https://agent-secret-capsule.sociobot.in/

**Repository revision:** `cd3d3e13661076dab0b7796a2fba07a2289b1403`

**Review date:** 2026-08-28 UTC

**Viewports:** fresh Chromium contexts at 390×844 and 1440×900

## Verdict

**FAIL.** Three blocking findings and two minor findings remain. The cold first
screen, populated demo, sandbox boundaries, listed claims, build, accessibility
scan, route metadata, HTTP routing, link availability, and visual identity pass.
The product is not at zero findings: Home does not receive focus or a route
announcement after leaving Demo; three factual README statements are outside the
claims manifest; metaphorical copy remains despite the earlier M3 closure; external
links are not identified as external; and the README does not provide a clickable
web-demo URL.

## Findings, ordered by severity

### F-4-1 — BLOCKING — Home loses route focus and announcement after Demo

**Prior finding reopened:** review-1 **M2**.

**Location:** live `/demo/` → **Start for real** → `/`; `site/src/main.ts`.

On live navigation, the resulting state is:

```text
URL: https://agent-secret-capsule.sociobot.in/
active element: BODY
h1 focused: false
route announcement: ""
```

Demo, Privacy, Terms, and 404 focus their `h1` on load. Home only handles a
persisted `pageshow`, so a normal full-page navigation to Home leaves focus on
`body` and does not update the polite live region. A keyboard or screen-reader
user receives no reliable indication that **Start for real** changed routes.

**Concrete fix:** run the same h1-focus and announcement behavior on Home's normal
`load` event as on legal pages. Add a browser test that activates **Start for real**
and asserts that Home's h1 is focused and the live region announces the Home title.

### F-4-2 — BLOCKING — factual README claims remain outside `claims.json`

**Prior findings reopened:** review-1 **B3/U46**, review-2 **F-2-2**, and
review-3 **F-3-2**.

**Location:** `README.md`, **Develop and verify**.

> “`npm test` runs Rust tests, site unit checks, and browser checks.”
>
> “Claim tests are listed in `.factory/claims.json`.”
>
> “Build the static site with `npm run build:site`; it writes `dist/site` for deployment.”

All three are factual statements a developer may rely on. They have no entries in
`.factory/claims.json`, even though they were manually verified in this round. The
claims contract requires the manifest—not a reviewer's ad hoc run—to inventory
every public claim.

**Concrete fix:** remove the first two descriptive sentences and leave the commands
as direct instructions. Add a `build-output` claim for the retained `dist/site`
statement with a test that runs `npm run build:site` and asserts the documented
output. If the test-suite composition remains public copy, cover it with a small
non-recursive manifest test that checks the package scripts and their successful
components.

### F-4-3 — BLOCKING — metaphorical copy remains after M3 was marked fixed

**Prior finding reopened:** review-1 **M3**.

**Locations:** live landing figure caption and live designed 404 h1.

> “One chosen path for one credential.”
>
> “This path has no receipt.”

The first line does not say what receives the credential; it relies on the concrete
and moss artwork to convey a “path.” The 404 h1 is a product pun rather than the page
state. Both violate the supplied plain-words rule that headings and sentences carry
usable information without metaphor. The earlier M3 closure said metaphorical copy
had been removed, so this is a half-fixed historical finding.

**Concrete fix:** replace the figure caption with **“The selected process and its
children receive one credential.”** Replace the 404 h1 with **“This page does not
exist.”** Keep the visual identity in the artwork and layout, not in required copy.

### F-4-4 — MINOR — external links do not identify their destination as external

**Locations:** live landing **“Read the CLI reference →”** and every footer's
**“Source”** link.

Both controls leave the product origin for GitHub, but neither visible text nor
accessible name says so. The arrow does not identify an external destination. This
fails the site-structure requirement that external links say they are external.

**Concrete fix:** use **“Read the CLI reference on GitHub (external)”** and
**“Source on GitHub (external)”**, or add an equivalent visible indicator plus
screen-reader text. Keep the current URLs.

### F-4-5 — MINOR — the README web demo path is not a usable link

**Location:** `README.md`, **Try the sample**.

> “The web sample is at `/demo/` or `/?demo=1`.”

On GitHub these are unlinked root-relative code strings, so a first-time reader is
not given the product origin and cannot open the promised web sample in one click.
The landing-page demo path itself passes.

**Concrete fix:** replace the sentence with
**“[Try the web sample](https://agent-secret-capsule.sociobot.in/demo/).”** Keep
`/?demo=1` only in verifier documentation if both entry forms still need coverage.

## Cold first read

The cold first screen passes at both required sizes without scrolling.

| Question | 390×844 | 1440×900 |
| --- | --- | --- |
| What does it do? | It gives one coding-agent command a temporary credential, redacts output, and saves a receipt without the credential. | Same. |
| For whom? | Developers running coding agents. | Same. |
| What should I click first? | **Try it with sample data.** | Same. |

Exact supporting copy:

> “Give one agent command a temporary credential.”
>
> “For developers running coding agents: run one command, redact its output, and save a receipt without the credential.”
>
> “Try it with sample data”
>
> “See a fake credential redacted and a no-value receipt.”

At 390×844, the audience, action, action note, and final fact end at y=471,
543, 594, and 673. At 1440×900, the audience and action end at y=657 and 736.
There were no console errors or horizontal overflow.

## Demo and sandbox verification

The one-click demo passes.

| Check | Result |
| --- | --- |
| Landing to demo | One click opens `/demo/` |
| Direct entry | `/?demo=1` redirects to `/demo/` |
| Immediate result | Both viewports show the `api-gateway` production fixture, two redactions, expiry result, alias, and succeeded outcome |
| Banner | **“Demo — sample data, nothing is saved”**, **Reset demo**, and **Start for real** are present |
| Demo namespace | Only `sessionStorage["demo:asc:run-count"]` is created after rerun; `localStorage` remains empty |
| Reset | Clears the demo key and restores `READY` |
| Real data | Seeded `real:sentinel` local/session keys survive Reset and Start for real unchanged |
| Requests | 13 requests in each cold flow; every request is same-origin |
| Cookies/errors | No cookies and no console errors |
| Offline | `/demo/` reloads with its h1 and sample result after the context is set offline |
| CLI demo | `asc demo` ran in a temporary working directory, redacted both streams, created two no-value receipts, and did not create the sentinel `ASC_HOME` |

On mobile, the sample alias and outcome end at y=715 and y=749. The credential
omission ends at y=841, inside the 844 px first viewport. On desktop, the
redacted output ends at y=616 and the receipt outcome ends at y=509.

The CLI demo created `/tmp/asc-demo-4432-1787948128810756046` with a private
directory, a mode-0600 receipt log, the bundled deployment fixture, and a README.
Neither receipt contained the fake credential.

F-4-1 concerns what happens after leaving the valid demo, not demo isolation.

## Claims verification from a clean clone

Fresh clone: `/tmp/asc-review4-clean.hxaJNi/repo` at
`cd3d3e13661076dab0b7796a2fba07a2289b1403`. Each manifest command ran
independently after `npm ci`.

| Claim | Declared test | Result |
| --- | --- | --- |
| `demo-isolation` | `npm run test:claim -- --grep @claim:demo-isolation` | PASS, 2 projects |
| `offline-reload` | `npm run test:claim -- --grep @claim:offline-reload` | PASS, 2 projects |
| `cli-demo` | `npm run test:claim -- --grep @claim:cli-demo` | PASS, 1 project / 1 intentional skip |
| `redaction-forms` | `cargo test --locked --features test-keyring --test cli_claims claim_redaction_forms_removes_every_named_form_from_compiled_cli_output` | PASS, 1 test |
| `process-tree` | `cargo test --locked --features test-keyring --test cli_claims claim_process_tree_uses_the_documented_cli_and_stops_at_its_time_limit` | PASS, 1 test |
| `captured-output-receipt` | `cargo test --locked --features test-keyring --test cli_claims claim_captured_output_and_receipt_omit_the_credential` | PASS, 1 test |
| `credential-lifecycle` | `cargo test --locked --features test-keyring --test cli_claims claim_credential_lifecycle_stores_lists_runs_and_removes_an_alias` | PASS, 1 test |
| `receipt-commands` | `cargo test --locked --features test-keyring --test cli_claims claim_receipt_commands_return_newest_first_human_and_json_no_value_results` | PASS, 1 test |
| `receipt-storage-schema` | `cargo test --locked --features test-keyring --test cli_claims claim_receipt_storage_schema_is_private_and_contains_only_declared_metadata` | PASS, 1 test |
| `cli-interface` | `cargo test --locked --features test-keyring --test cli_claims claim_cli_interface_help_and_non_tty_input_behave_as_documented` | PASS, 1 test |
| `demo-parity` | `npm run test:claim -- --grep @claim:demo-parity` | PASS, 1 project / 1 intentional skip |
| `license-package` | `npm run test:claim -- --grep @claim:license-package` | PASS, 1 project / 1 intentional skip |
| `site-privacy` | `npm run test:claim -- --grep @claim:site-privacy` | PASS, 2 projects |

No listed test failed. F-4-2 records the public statements absent from this list.

## Copy audit

Counts use whitespace-delimited words. Hyphenated compounds count as one word.
Commands, paths, terminal output, field labels, and version labels are included in
the separate control check when they are not sentences.

### Landing sentences

| # | Location and exact sentence | Words | Result |
| --- | --- | ---: | --- |
| L01 | Description: Run one coding-agent command with a temporary credential, redact its output, and save a no-value receipt. | 16 | `process-tree`, `captured-output-receipt` |
| L02 | OG description: A local CLI for running one selected command with a temporary credential. | 12 | `process-tree` |
| L03 | Twitter description: A local CLI for running one selected command with a temporary credential. | 12 | `process-tree` |
| L04 | Give one agent command a temporary credential. | 7 | Clear; `process-tree` |
| L05 | For developers running coding agents: run one command, redact its output, and save a receipt without the credential. | 18 | Clear; `process-tree`, `captured-output-receipt` |
| L06 | See a fake credential redacted and a no-value receipt. | 9 | `cli-demo` |
| L07 | Alt: A monolithic concrete capsule divided by one narrow seam of living moss. | 12 | Clear image description |
| L08 | One chosen path for one credential. | 6 | **F-4-3: metaphor; does not name the process boundary** |
| L09 | See the command result first. | 5 | Clear heading |
| L10 | The CLI ships with `asc demo`. | 6 | `cli-demo` |
| L11 | It creates fake sample receipts in a new temporary directory. | 10 | `cli-demo` |
| L12 | Select. | 1 | Clear step heading |
| L13 | Run. | 1 | Clear step heading |
| L14 | Redact. | 1 | Clear step heading |
| L15 | Review the receipt. | 3 | Clear step heading |
| L16 | Use a local alias in the agent tool input. | 9 | Clear instruction |
| L17 | Store an alias locally. | 4 | `credential-lifecycle` |
| L18 | Use the alias in the agent tool input. | 8 | Clear instruction |
| L19 | The selected process and its children receive the credential until exit or the time limit. | 15 | `process-tree` |
| L20 | ASC captures both output streams and replaces matching credential forms before printing them. | 13 | `redaction-forms`, `captured-output-receipt` |
| L21 | A no-value receipt omits the credential. | 6 | `captured-output-receipt` |
| L22 | Build the CLI. | 3 | Clear instruction; build gate |
| L23 | Run the sample. | 3 | `cli-demo` |
| L24 | Use the bundled demo before storing a real credential. | 9 | `cli-demo` |
| L25 | Redaction limits output leaks. | 4 | Bounded by `redaction-forms` and adjacent warning |
| L26 | It is not a sandbox. | 5 | Clear limitation |
| L27 | An authorized command can send the credential over the network or write it to a file. | 16 | Clear limitation |
| L28 | It can also transform it or pass it to a child. | 11 | Clear limitation |
| L29 | Review the command and endpoint. | 5 | Clear instruction |
| L30 | Use a separate network and process sandbox for hostile code. | 10 | Clear instruction |
| L31 | One command. | 2 | Informative footer fragment |
| L32 | One no-value receipt. | 3 | `captured-output-receipt` |

No landing sentence exceeds 22 words or uses a banned marketing adjective.

### Landing headings, facts, labels, and controls

| Copy | Words | Result |
| --- | ---: | --- |
| CLI FOR CODING AGENTS | 4 | Names the product type and audience |
| No analytics or third-party scripts | 5 | `site-privacy` |
| Demo works offline after first visit | 6 | `offline-reload` |
| Free and open source | 4 | `license-package` |
| Try it with sample data | 5 | Result-naming primary action |
| Open the sample run | 4 | Result-naming action |
| HOW THE CREDENTIAL TIME LIMIT WORKS | 6 | Clear section label |
| INSTALL / v0.1.0 | 2 | Clear section/version label |
| Copy install command | 3 | Result-naming action |
| Read the CLI reference → | 4 | Result-naming, but **F-4-4** for external destination |
| SECURITY LIMITS | 2 | Clear section label |
| Source | 1 | **F-4-4: external destination is not identified** |
| This path has no receipt. | 5 | **F-4-3: metaphorical 404 h1** |

Other sample/terminal labels describe displayed state. No action uses a generic
“Submit,” “Go,” or “Continue” label.

### README sentences

| # | Exact sentence | Words | Result |
| --- | --- | ---: | --- |
| R01 | Agent Secret Capsule (`asc`) gives one selected process and its children a temporary credential. | 14 | `process-tree` |
| R02 | It captures command output before printing it. | 7 | `captured-output-receipt` |
| R03 | It writes a receipt without the credential value. | 8 | `captured-output-receipt` |
| R04 | For developers whose coding agents need an authorized API call. | 10 | Clear audience statement |
| R05 | Use a local alias in the agent tool input. | 9 | Clear instruction |
| R06 | Run the bundled sample before storing a real credential. | 9 | `cli-demo` |
| R07 | The command checks a bundled fake deployment-status fixture. | 8 | `cli-demo` |
| R08 | It uses a fake credential. | 5 | `cli-demo` |
| R09 | It creates a new temporary directory with sample no-value receipts and prints its path. | 14 | `cli-demo` |
| R10 | It does not read your keychain or `ASC_HOME`. | 8 | `cli-demo` |
| R11 | Delete that directory to reset the command-line sample. | 8 | Clear instruction |
| R12 | The web sample is at `/demo/` or `/?demo=1`. | 8 | `demo-isolation`; **F-4-5: not a clickable absolute URL** |
| R13 | It uses browser storage keys with the `demo:asc` prefix. | 9 | `demo-isolation` |
| R14 | Reset demo clears those sample keys. | 6 | `demo-isolation` |
| R15 | Build from source. | 3 | Clear instruction; build gate |
| R16 | Store a credential from standard input. | 6 | `credential-lifecycle` |
| R17 | Run a selected process tree with a time limit. | 9 | `process-tree` |
| R18 | Inspect receipts or automate with JSON. | 6 | `receipt-commands` |
| R19 | Run `asc --help` for commands and exit codes. | 8 | `cli-interface` |
| R20 | Run `asc <command> --help` for flags and examples. | 8 | `cli-interface` |
| R21 | When standard input is not a terminal, `put` requires `--stdin`. | 10 | `cli-interface` |
| R22 | ASC gives the credential to the selected process and its children until exit or the time limit. | 17 | `process-tree` |
| R23 | It redacts raw, percent-encoded, Base64, Base64url, and hex matches from captured stdout and stderr. | 14 | `redaction-forms`, `captured-output-receipt` |
| R24 | A no-value receipt omits the credential value. | 7 | `captured-output-receipt` |
| R25 | This is not a sandbox. | 5 | Clear limitation |
| R26 | An authorized process can send the credential over the network or write it to a file. | 16 | Clear limitation |
| R27 | It can also transform the credential or pass it to a child. | 12 | Clear limitation |
| R28 | Review the exact command and endpoint. | 6 | Clear instruction |
| R29 | Use a separate sandbox for hostile code. | 7 | Clear instruction |
| R30 | `npm test` runs Rust tests, site unit checks, and browser checks. | 11 | **F-4-2: unlisted claim** |
| R31 | Claim tests are listed in `.factory/claims.json`. | 6 | **F-4-2: unlisted factual statement** |
| R32 | Build the static site with `npm run build:site`; it writes `dist/site` for deployment. | 13 | **F-4-2: unlisted build-output claim** |
| R33 | MIT. | 1 | `license-package` |

No README sentence exceeds 22 words or contains a banned marketing adjective.
The terms `credential`, `alias`, `selected process and its children`, `redact`,
`no-value receipt`, and `time limit` are otherwise used consistently. README
headings—**Try the sample**, **Install**, **Usage**, **Security limits**,
**Develop and verify**, and **License**—are understandable in context. README has
no buttons.

## Structure, routing, accessibility, and visual identity

- `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and an unknown route have
  route-specific titles, one h1, `lang=en`, one main, descriptions, canonicals,
  Open Graph/Twitter data, favicon, and touch icon. Home's title is 52 characters.
- `/not-a-real-route` returns HTTP 404 with the designed page. `robots.txt`,
  `sitemap.xml`, assets, and service worker return their expected status and types.
- Every crawled live link returns 200 after redirects. F-4-4 concerns labeling,
  not availability.
- Privacy → Terms → Back restores the Privacy h1 focus. Demo, legal, and 404 direct
  loads focus their h1. Home's missing focus/announcement is F-4-1.
- `verify-url.sh` passes Home, Demo, Privacy, and Terms with no console errors.
- Live Axe scans report zero serious/critical violations on five routes at both
  viewports. Keyboard skip-link behavior and 44 px mobile targets also pass the
  repository browser suite.
- All valid-route requests were same-origin. The valid pages set no cookies and
  load no CDN fonts or third-party scripts.
- Built JavaScript chunks are each below 1.3 kB uncompressed; the first-load limit
  is comfortably met.
- The concrete slab/moss seam, serif/mono type pairing, stamped labels, mechanical
  controls, square geometry, and restrained state motion match `.factory/design.md`.
  This is a distinct containment-tool identity, not a generic SaaS template.
- The live Home, Demo, Privacy, and Terms HTML SHA-256 values exactly match the
  clean build, confirming that live/code comparisons refer to the reviewed revision.

## Earlier-finding verification

Every earlier `review-*.md`, `polish-*.md`, verification report, and the prior
handoff was read. Each prior review finding was checked again in live output and
current code.

### Severity findings from reviews 1–3

| Earlier ID | Status now | Evidence |
| --- | --- | --- |
| B1 | Fixed | Both cold first screens show job, audience, action, outcome, and three facts. |
| B2 | Fixed | Immediate populated demo, banner, controls, isolation, CLI temp demo, and parity all pass. |
| B3 | **Reopened by F-4-2** | All 13 listed claims pass, but the README still contains unlisted factual claims. |
| B4 | Fixed | Public scope says selected process and children; compiled CLI claim passes. |
| B5 | Fixed | No paid flow, checkout, license form, or billing link remains. |
| B6 | Fixed | Real routes load; the designed unknown route returns 404. |
| M1 | Fixed | Complete per-route metadata, discovery assets, social image, and favicons verified. |
| M2 | **Reopened by F-4-1** | Shared shell and legal Back focus pass; Demo → Home leaves focus on `body` and announces nothing. |
| M3 | **Reopened by F-4-3** | Length, jargon, and terminology pass; landing/404 metaphors remain. |
| M4 | Fixed | 390 px layout has no overflow and tested controls meet 44 px. |
| F-2-1 | Fixed | Desktop audience and action end at y=657 and y=736. |
| F-2-2 | **Reopened by F-4-2** | Its prior quoted product claims are fixed, but the claim inventory is still not exhaustive. |
| F-2-3 | Fixed | Warning is split; visitor copy uses “time limit,” not “lease.” |
| F-2-4 | Fixed | 404 has route-specific OG and Twitter metadata. |
| F-3-1 | Fixed | Both demo first screens show realistic redaction, expiry, alias, and outcome. |
| F-3-2 | **Reopened by F-4-2** | Core workflow claims are now covered; README verification/build claims are not. |
| F-3-3 | Fixed | All three tested facts end above y=844 on mobile. |

### Review-1 copy findings

| Earlier ID | Status now | Evidence |
| --- | --- | --- |
| CW01 | Fixed | Job-first seven-word headline; banned “powerful” is absent. |
| CW02 | Fixed | Developer/coding-agent audience is explicit. |
| CW03 | Fixed | “The scope is the safety feature” remains absent. |
| CW04 | Fixed | Scope includes the selected process and its children. |
| CW05 | Fixed | README names each supported redaction form. |
| CW06 | Fixed | Receipt copy says exactly that the credential is omitted. |
| CW07 | Fixed | Visitor copy uses “time limit,” not “lease.” |
| CW08 | Fixed | Instructions use “alias,” not “capability.” |
| CW09 | Fixed | Scope/duration wording is plain and accurate. |
| CW10 | Fixed | Sample heading names the command result. |
| CW11 | Fixed | Isolated real demo replaced the scripted illustration. |
| CW12 | Fixed | Browser/CLI sample parity has a passing test. |
| CW13 | Fixed | Required sample, reset, exit, and rerun actions are present. |
| CW14 | Fixed | Control says “Copy install command.” |
| CW15 | Fixed | Authorized-command warning is split below 22 words. |
| CW16 | Fixed | “Safety layer” marketing remains absent. |
| CW17 | Fixed | README opening is three short sentences. |
| CW18 | Fixed | README audience and instruction are short. |
| CW19 | Fixed | Scope, streams, forms, and receipt behavior are separate. |
| CW20 | Fixed | README authorized-process warning is split. |
| CW21 | Fixed | “percent-encoded” replaces ambiguous “URL.” |
| CW22 | Fixed | Copy says redaction limits leaks and is not a sandbox. |
| CW23 | Fixed | Unsupported license form remains absent. |
| CW24 | Fixed | Explicit non-sandbox limitation remains. |
| CW25 | Fixed | Selected process tree/time-limit wording remains. |
| CW26 | Fixed | Exact supported forms remain named. |

### Review-1 claim findings

| Earlier ID | Status now | Evidence |
| --- | --- | --- |
| U01 | Fixed | Prompt/tool-trace guarantee remains removed. |
| U02 | Fixed | Bundled platform/backend/telemetry fact remains removed. |
| U03 | Fixed | Unconditional keychain-at-rest landing claim remains removed. |
| U04 | Fixed | Correct process-and-children claim passes. |
| U05 | Fixed | Named redaction forms pass. |
| U06 | Fixed | Precise no-value receipt behavior passes. |
| U07 | Fixed | Capability/prompt promise remains removed. |
| U08 | Fixed | Keychain-resolution landing promise remains removed. |
| U09 | Fixed | Prompt-content promise remains removed. |
| U10 | Fixed | Correct scope and duration pass. |
| U11 | Fixed | Both captured streams are tested. |
| U12 | Fixed | Raw, percent, Base64, Base64url, and hex forms pass. |
| U13 | Fixed | Receipt schema is listed and tested in the privacy claim. |
| U14 | Fixed | Persisted receipt omission passes. |
| U15 | Fixed | Demo fake data, isolation, reset, and request scope pass. |
| U16 | Fixed | Browser/CLI fake-result parity passes. |
| U17 | Fixed | Daemon/account/cloud-vault/telemetry bundle remains absent. |
| U18 | Fixed | Visitor-facing compiler-version promise remains absent. |
| U19 | Fixed | Honest non-sandbox limitation remains. |
| U20 | Fixed | Authorized-command boundary warning remains. |
| U21 | Fixed | Vague free-tier promise remains absent. |
| U22 | Fixed | Ungated-feature marketing remains absent. |
| U23 | Fixed | Price, kit, release, and update promises remain absent. |
| U24 | Fixed | Merchant copy remains absent. |
| U25 | Fixed | Refund/revocation promise remains absent. |
| U26 | Fixed | “Free core active” remains absent. |
| U27 | Fixed | License-storage state/code remains absent. |
| U28 | Fixed | No-value receipt behavior passes. |
| U29 | Fixed | Broad README claim is split across passing behavior claims. |
| U30 | Fixed | Broad keychain-at-rest sentence remains absent. |
| U31 | Fixed | Hosted-store/CLI-telemetry sentence remains absent. |
| U32 | Fixed | Honest security limitation remains. |
| U33 | Fixed | Compiler-version promise remains absent. |
| U34 | Fixed | Future binary-release promise remains absent. |
| U35 | Fixed | Package command and package contents pass under `license-package`. |
| U36 | Fixed | Successful put/list/run/remove lifecycle claim passes. |
| U37 | Fixed | Process-tree expiry claim passes through the compiled CLI. |
| U38 | Fixed | Both streams/forms/receipt behavior pass. |
| U39 | Fixed | Human and JSON receipt command claim passes. |
| U40 | Fixed | Successful remove behavior passes. |
| U41 | Fixed | All documented help paths/examples pass. |
| U42 | Fixed | Non-TTY input rejection passes. |
| U43 | Fixed | Process, expiry, forms, streams, and receipt paths pass. |
| U44 | Fixed | Persisted receipt log omits the credential. |
| U45 | Fixed | Authorized-process warning remains explicit. |
| U46 | **Reopened by F-4-2** | `npm test` passed, but its public composition statement still lacks a manifest entry. |
| U47 | Fixed | Live and claim request logs are same-origin; cookies are empty. |
| U48 | Fixed | License storage and verification code remain absent. |
| U49 | Fixed | Source/package MIT claim passes. |
| U50 | Fixed | Lifetime-update wording remains absent. |
| U51 | Fixed | Label is `STORE / STDIN INPUT`; no history guarantee remains. |
| U52 | Fixed | Label is `RECEIPTS / JSON`; receipt commands pass. |
| U53 | Fixed | Team-rollout entitlement remains absent. |
| U54 | Fixed | Receipt-policy entitlement remains absent. |
| U55 | Fixed | Home metadata uses credential wording without lease jargon. |
| U56 | Fixed | Visitor copy uses “redact,” not “scrub.” |
| U57 | Fixed | Receipt omission remains explicit and tested. |

The verification-1 parser panic is fixed by current compiled-binary parser and
lifecycle tests. Its immutable-cache issue is fixed in live headers. The later
verification-2 environment limitation remains an environment note rather than a
product defect; this round used the isolated claim keychain for successful lifecycle
coverage and confirmed that the normal CLI demo does not access it.

## Quality gates and evidence

```text
npm ci                 PASS (60 packages, 0 vulnerabilities)
13 manifest commands   PASS independently
npm test               PASS (10 Rust, 2 Vitest, 32 Playwright; 6 intentional skips)
npm run build          PASS (release CLI and dist/site)
cargo fmt --check      PASS
cargo clippy ...       PASS with -D warnings
verify-url.sh          PASS on /, /demo/, /privacy/, /terms/
live Axe               0 serious/critical findings, 5 routes × 2 viewports
live link crawl        every link 200; unknown route HTTP 404
```

## Missed leverage

No additional AI, sync, or import feature is justified. The brief calls for a
narrow local credential boundary; sending credential context to an AI gateway
would widen that boundary without improving the core job. Human and JSON receipt
output already provide the useful export path. No decorative AI feature or embedded
provider key is present.

## What would make this perfect

Restore h1 focus and a polite announcement when Home is reached from another route.
Remove or test the three unlisted README statements. Replace the landing caption and
404 pun with literal copy. Mark GitHub links as external, and make the README web
demo an absolute clickable link. Then rerun all 13 claim commands, the complete
suite/build, both cold first screens, Demo → Home focus, the sandbox/offline checks,
the live link/Axe crawl, and this full historical ledger. A pass requires all five
findings to be gone.
