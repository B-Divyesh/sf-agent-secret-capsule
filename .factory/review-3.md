# Adversarial first-read review 3 — FAIL

**Product:** Agent Secret Capsule

**Live URL:** https://agent-secret-capsule.sociobot.in/

**Repository revision:** `19b8058765876fc100311b781687eda950da8cf1`

**Review date:** 2026-08-28 UTC

**Viewports:** fresh Chromium contexts at 390×844 and 1440×900

## Verdict

**FAIL.** Two blocking findings and one minor finding remain. The one-click demo
route exists and is isolated, but neither initial viewport shows a usable
sample result. Several public CLI and privacy claims are still
absent from `.factory/claims.json`. The landing first screen answers the three
basic questions, but only one of its three required facts is fully visible on a
390×844 screen.

All eight listed claim commands pass from a clean clone. The ordinary test and
build gates also pass. Passing listed tests does not cover omitted claims or the
required immediate demo state.

## Findings, ordered by severity

### F-3-1 — BLOCKING — the demo does not show the product result on its first screen

**Location:** live `/demo/`, immediately after activating **“Try it with sample
data”**, in fresh 390×844 and 1440×900 contexts. This reopens the unresolved
presentation part of review-1 **B2**.

Exact text occupying the viewport:

> “Demo — sample data, nothing is saved”
>
> “See a credential redacted before it reaches the receipt.”
>
> “Use this sample to inspect the result. Run `asc demo` locally for the same
> fake-data workflow.”

The banner ends at y=214, the h1 occupies y=303–591, and its explanation ends at
y=728. The terminal starts at y=784, but its actual output starts at y=834, only
10 px before the 844 px viewport edge. The sample receipt starts at y=1154. A
visitor therefore sees explanation and a terminal border, not the redacted
stdout/stderr, expiry result, or receipt. At 1440×900, the terminal starts at
y=794 and its output at y=844; only the command line begins at the bottom edge.
The receipt starts at y=794, but only its heading and alias row appear. Neither
first screen shows the redaction, expiry, or completed receipt result.

The data is also generic: alias `demo-api`, executable `sh`, and an echo of the
credential do not represent the coding-agent API-call situation named in the
brief.

**Concrete fix:** At both review sizes, place the populated terminal output and
the key receipt result above the fold. Reduce the display heading/spacing or put
the workbench before the explanation. Seed a concrete developer scenario, such as a
read-only deployment-status alias executed against a bundled local fixture. Add
tests at 390×844 and 1440×900 asserting that redacted stdout, the expiry result,
and at least the receipt alias/outcome are fully within the initial viewport.

### F-3-2 — BLOCKING — public core-workflow claims remain outside the claims manifest

**Locations:** live landing/demo/privacy pages and `README.md`. This reopens
review-1 **B3**, **U16**, **U36**, and **U39–U42** as half-fixed.

The manifest contains eight passing claims, but it has no entry and no tagged,
successful end-to-end test for the following statements or instructions:

| Exact quote/location | Why it is unlisted or inadequately proved | Concrete fix |
| --- | --- | --- |
| Landing: “Store an alias locally.” README: “Store a credential from standard input:” plus `asc put`, `asc list --json`, and `asc remove cloudflare` | No manifest claim proves a credential can be stored, listed, then removed. `valid_alias_commands_reach_their_operational_json_paths_without_panicking` accepts exit code 3, so a keychain failure passes. | Add a `credential-lifecycle` claim. Run the packaged CLI against an isolated test keychain and assert successful put, alias-only list, run, and remove behavior. |
| Landing label: “RUN / 30-SECOND LIMIT.” README: “Run a selected process tree with a 30-second time limit:” | The `process-tree` test expires a library fixture after 30 ms. It neither invokes the documented CLI command nor measures the advertised 30-second value. | Change the copy to “SET A TIME LIMIT,” or add a quantitative packaged-CLI claim that verifies the documented duration with a stated margin. |
| README: “Inspect receipts or automate with JSON:” plus `asc receipts` and `asc receipts --json` | The receipt claim checks one serialized `Receipt`; it does not execute either documented receipt command or assert their output/schema. | Add a `receipt-commands` claim that creates receipts through the CLI, checks human and JSON output, row count, newest-first order, and absence of credential forms. |
| README: “Run `asc --help` or `asc <command> --help` for flags, exit codes, and examples.” and “When standard input is not a terminal, `put` requires `--stdin`.” | These interface promises are not in the manifest. Current parser tests do not enumerate every help path or test rejection of non-TTY input without `--stdin`. | Add a tagged `cli-interface` claim covering every documented help path, exit code, and the negative non-TTY case. |
| Privacy: “The CLI stores aliases and receipt metadata in your user data directory.” and “Receipts contain time, alias, environment variable name, executable name, duration, outcome, exit code, time limit, and redaction count.” | No manifest claim covers storage location or this exact receipt schema. | Add a `receipt-storage-schema` claim that runs the packaged CLI in an isolated data directory and asserts the path, permissions, exact fields, and absence of credential values. |
| Demo: “Run `asc demo` locally for the same fake-data workflow.” and “Shows the actual CLI demo command and no-value receipt shape.” | The web page is fixed HTML. The `cli-demo` and `demo-isolation` tests run separately and never compare the browser sample to CLI output. This is a parity claim. | Add a fixture/parity test generated from `asc demo`, or rewrite to say the browser is a static preview and link to the real CLI demo command. |

The ordinary Rust suite contains useful unit coverage, but the claims contract
requires every public behavior to be listed and observable through its declared
sandbox. Add the entries and tagged tests, or remove/narrow the copy.

### F-3-3 — MINOR — the mobile landing first screen does not contain all three required facts

**Location:** live `/`, fresh 390×844 context.

The first fact, **“Local CLI,”** occupies y=805–826. **“Sample data stays
separate”** occupies y=832–852 and is clipped. **“Free and open source”** is below
the viewport. The required first-screen fact group is therefore incomplete on
the review phone size. It also omits the already tested offline fact.

**Concrete fix:** Compact the mobile hero so all three lines fit above y=844.
Use the three already testable facts **“No analytics or third-party scripts,”
“Demo works offline after first visit,”** and **“Free and open source.”** Add a
390×844 bounding-box assertion for every fact, not only the audience and CTA.

## Cold first read

The earlier comprehension failure is fixed at both widths.

| Question | 390×844 | 1440×900 |
| --- | --- | --- |
| What does it do? | It gives one coding-agent command a temporary credential, redacts its output, and saves a receipt without the credential. | Same. |
| For whom? | Developers running coding agents. | Same. |
| What should I click first? | **Try it with sample data.** | **Try it with sample data.** |

The exact first-screen copy is:

> “Give one agent command a temporary credential.”
>
> “For developers running coding agents: run one command, redact its output,
> and save a receipt without the credential.”
>
> “Try it with sample data”
>
> “See a fake credential redacted and a no-value receipt.”

At desktop, the audience ends at y=671 and the CTA ends at y=750. At mobile,
they end at y=633 and y=724. There are no console errors or horizontal overflow
on the landing page.

## Copy audit

Counts use whitespace-delimited words; hyphenated compounds, commands, and paths
count as one token. No sentence exceeds 22 words. No banned marketing adjective
appears. Claim flags below map to F-3-2; the controls and headings are checked
after the sentence tables.

### Landing page sentences

| # | Sentence | Words | Result |
| --- | --- | ---: | --- |
| L01 | Run one coding-agent command with a temporary credential, redact its output, and save a no-value receipt. | 16 | Covered by `process-tree` and `captured-output-receipt` |
| L02 | A local CLI for running one selected command with a temporary credential. | 12 | Covered by `process-tree`; used twice in OG/Twitter metadata |
| L03 | Give one agent command a temporary credential. | 7 | Covered by `process-tree` |
| L04 | For developers running coding agents: run one command, redact its output, and save a receipt without the credential. | 18 | Covered by `process-tree` and `captured-output-receipt` |
| L05 | See a fake credential redacted and a no-value receipt. | 9 | Covered by `cli-demo` |
| L06 | One chosen path for one credential. | 6 | Descriptive figure caption |
| L07 | See the command result first. | 5 | Clear heading |
| L08 | The CLI ships with `asc demo`. | 6 | Covered by `cli-demo` |
| L09 | It creates fake sample receipts in a new temporary directory. | 10 | Covered by `cli-demo` |
| L10 | Use a local alias in the agent tool input. | 9 | Instruction |
| L11 | Store an alias locally. | 4 | **F-3-2: unlisted storage behavior** |
| L12 | Use the alias in the agent tool input. | 8 | Instruction |
| L13 | The selected process and its children receive the credential until exit or the time limit. | 15 | Covered by `process-tree` |
| L14 | ASC captures both output streams and replaces matching credential forms before printing them. | 13 | Covered by `redaction-forms` and `captured-output-receipt` |
| L15 | A no-value receipt omits the credential. | 6 | Covered by `captured-output-receipt` |
| L16 | Build the CLI. | 3 | Verified by the build gate |
| L17 | Run the sample. | 3 | Covered by `cli-demo` |
| L18 | Use the bundled demo before storing a real credential. | 9 | Covered by `cli-demo` |
| L19 | Redaction limits output leaks. | 4 | Honest limitation paired with `redaction-forms` |
| L20 | It is not a sandbox. | 5 | Honest limitation |
| L21 | An authorized command can send the credential over the network or write it to a file. | 16 | Honest limitation |
| L22 | It can also transform it or pass it to a child. | 11 | Honest limitation |
| L23 | Review the command and endpoint. | 5 | Clear instruction |
| L24 | Use a separate network and process sandbox for hostile code. | 10 | Clear instruction |
| L25 | One command. | 2 | Footer fragment |
| L26 | One no-value receipt. | 3 | Covered by `captured-output-receipt` |

### README sentences

| # | Sentence | Words | Result |
| --- | --- | ---: | --- |
| R01 | Agent Secret Capsule (`asc`) gives one selected process and its children a temporary credential. | 14 | Covered by `process-tree` |
| R02 | It captures command output before printing it. | 7 | Covered by `captured-output-receipt` |
| R03 | It writes a receipt without the credential value. | 8 | Covered by `captured-output-receipt` |
| R04 | For developers whose coding agents need an authorized API call. | 10 | Audience statement |
| R05 | Use a local alias in the agent tool input. | 9 | Instruction |
| R06 | Run the bundled sample before storing a real credential. | 9 | Covered by `cli-demo` |
| R07 | The command uses a fake credential. | 6 | Covered by `cli-demo` |
| R08 | It creates a new temporary directory with sample no-value receipts and prints its path. | 14 | Covered by `cli-demo` |
| R09 | It does not read your keychain or `ASC_HOME`. | 8 | Covered by `cli-demo` |
| R10 | Delete that directory to reset the command-line sample. | 8 | Clear instruction |
| R11 | The web sample is at `/demo/` or `/?demo=1`. | 8 | Covered by `demo-isolation` |
| R12 | It uses browser storage keys with the `demo:asc` prefix. | 9 | Covered by `demo-isolation` |
| R13 | Reset demo clears those sample keys. | 6 | Covered by `demo-isolation` |
| R14 | Build from source. | 3 | Verified by the build gate |
| R15 | Store a credential from standard input. | 6 | **F-3-2: unlisted credential lifecycle** |
| R16 | Run a selected process tree with a 30-second time limit. | 10 | **F-3-2: quantitative duration not listed/tested** |
| R17 | Inspect receipts or automate with JSON. | 6 | **F-3-2: receipt commands not listed/tested** |
| R18 | Run `asc --help` or `asc <command> --help` for flags, exit codes, and examples. | 13 | **F-3-2: interface contract not listed/tested** |
| R19 | When standard input is not a terminal, `put` requires `--stdin`. | 10 | **F-3-2: negative behavior not listed/tested** |
| R20 | ASC gives the credential to the selected process and its children until exit or the time limit. | 17 | Covered by `process-tree` |
| R21 | It redacts raw, percent-encoded, Base64, Base64url, and hex matches from captured stdout and stderr. | 14 | Covered by `redaction-forms` and `captured-output-receipt` |
| R22 | A no-value receipt omits the credential value. | 7 | Covered by `captured-output-receipt` |
| R23 | This is not a sandbox. | 5 | Honest limitation |
| R24 | An authorized process can send the credential over the network or write it to a file. | 16 | Honest limitation |
| R25 | It can also transform the credential or pass it to a child. | 12 | Honest limitation |
| R26 | Review the exact command and endpoint. | 6 | Clear instruction |
| R27 | Use a separate sandbox for hostile code. | 7 | Clear instruction |
| R28 | `npm test` runs Rust tests, site unit checks, and browser checks. | 11 | Verified in this review |
| R29 | Claim tests are listed in `.factory/claims.json`. | 6 | Manifest exists |
| R30 | Build the static site with `npm run build:site`; it writes `dist/site` for deployment. | 13 | Verified in this review |
| R31 | MIT. | 1 | Covered by `license-package` |

### Headings, labels, and controls

- Landing headings remain understandable out of context: **“See the command
  result first,” “Select. Run. Redact. Review the receipt,” “Build the CLI. Run
  the sample,”** and **“Redaction limits output leaks. It is not a sandbox.”**
- Landing actions name their results: **Try it with sample data**, **Open the
  sample run**, **Copy install command**, and **Read the CLI reference**.
- Demo actions name their results: **Reset demo**, **Start for real**, and **Run
  sample again**.
- README headings—**Try the sample, Install, Usage, Security limits, Develop and
  verify, License**—are understandable in context.
- Terminology is consistent: `credential`, `alias`, `selected process and its
  children`, `redact`, `no-value receipt`, and `time limit`.
- The fact strip has the viewport problem recorded as F-3-3. There are no other
  jargon, banned-word, overlength, inconsistent-term, heading, or action-label
  findings.

## Demo and sandbox verification

| Check | Result |
| --- | --- |
| Landing to demo | One click opens `/demo/` |
| Immediate desktop result | **FAIL — F-3-1; frames begin, but the result is below the fold** |
| Immediate mobile result | **FAIL — F-3-1** |
| Persistent banner | Present with **Reset demo** and **Start for real** |
| Browser namespace | Only `sessionStorage["demo:asc:run-count"]` is created |
| Reset | Returns state to `READY` and removes only the `demo:asc:` key |
| Start for real | Removes the demo key, returns home, and preserves seeded `real:sentinel` local/session values |
| Requests/privacy | All requests were same-origin; no cookies, third-party scripts, or console errors |
| Offline | After first load, `/demo/` reloaded offline with h1 and redacted output intact |
| CLI demo in temp directory | PASS; two receipts, both streams redacted, fake credential absent, sentinel `ASC_HOME` untouched |

The direct CLI check ran from a fresh temporary working directory with an
unavailable keychain session. It wrote only `README.txt` and `receipts.jsonl` to
a new `/tmp/asc-demo-*` directory and did not create the sentinel real-data
directory.

## Claims verification from a clean clone

A non-local clone of revision `19b8058765876fc100311b781687eda950da8cf1`
received `npm ci`, then each manifest command was run independently.

| Claim | Command | Result |
| --- | --- | --- |
| `demo-isolation` | `npm run test:claim -- --grep @claim:demo-isolation` | PASS, 2 tests |
| `offline-reload` | `npm run test:claim -- --grep @claim:offline-reload` | PASS, 2 tests |
| `cli-demo` | `npm run test:claim -- --grep @claim:cli-demo` | PASS, 1 test / 1 viewport skip |
| `redaction-forms` | `cargo test --locked redacts_raw_and_encoded_forms` | PASS, 1 test |
| `process-tree` | `npm run test:claim -- --grep @claim:process-tree` | PASS, 1 test / 1 viewport skip |
| `captured-output-receipt` | `npm run test:claim -- --grep @claim:captured-output-receipt` | PASS, 1 test / 1 viewport skip |
| `license-package` | `npm run test:claim -- --grep @claim:license-package` | PASS, 1 test / 1 viewport skip |
| `site-privacy` | `npm run test:claim -- --grep @claim:site-privacy` | PASS, 2 tests |

No listed claim test failed. F-3-2 concerns public claims missing from this list.

## Structure, routing, accessibility, and visual identity

- `/`, `/demo/`, `/privacy/`, `/terms/`, and the designed unknown-route page
  have the required route-specific title pattern, one h1, `lang=en`, one main,
  descriptions, canonicals, OG/Twitter data, SVG favicon, and touch icon.
- `/not-a-real-route` returns HTTP 404 and the designed concrete-and-moss page.
  `robots.txt`, `sitemap.xml`, favicon, touch icon, and social image return 200
  with the expected content types.
- Privacy → Terms → Back restores the Privacy title and h1 focus. Direct route
  loads focus the h1 and use the polite route announcement.
- All landing links returned 200, including both GitHub links. No dead link was
  found.
- Header/footer content is consistent across routes. Footer links include
  Privacy, Terms, Source, Param Factory attribution, and version.
- Live Axe scans found zero serious or critical violations on all five routes at
  desktop and mobile. `verify-url.sh` passed `/`, `/demo/`, `/privacy/`, and
  `/terms/` with no console errors.
- Heading levels do not skip, touch controls are at least 44 px, the 390 px
  layouts do not overflow, focus styling is visible, and reduced motion is
  implemented.
- The dark concrete slab, moss seam, serif/mono pairing, stamped labels, and
  mechanical controls match `.factory/design.md`. This is a distinct CLI
  containment identity, not a generic gradient/card SaaS template.

## Earlier-finding verification

Every prior review and polish ledger plus the current handoff was read. The
following checks were repeated against both live output and current code.

### Severity findings

| Earlier ID | Status now | Evidence |
| --- | --- | --- |
| B1 | Fixed | Both cold first screens state job, user, and sample action. |
| B2 | **Reopened by F-3-1** | Isolation exists, but both post-click first screens hide the usable result. |
| B3 | **Reopened by F-3-2** | Eight claims exist and pass; the inventory is still incomplete. |
| B4 | Fixed | Public scope says process and children; `process-tree` passes. |
| B5 | Fixed | Paid flow and billing links remain absent. |
| B6 | Fixed | Real routes and designed HTTP 404 verified live. |
| M1 | Fixed | Complete route metadata and discovery assets verified live. |
| M2 | Fixed | Shared shell, section order, route announcement, and back-focus verified. |
| M3 | Fixed | Full copy audit has no sentence over 22 words or banned marketing word. |
| M4 | Fixed | Mobile targets and overflow checks pass. |
| F-2-1 | Fixed | Desktop audience and CTA end at y=671 and y=750. |
| F-2-2 | Fixed for its six quoted claims | Prompt/Rust claims remain removed; demo, MIT, and site privacy tests pass. F-3-2 identifies other omissions. |
| F-2-3 | Fixed | Warning split; no visitor-facing “secret leasing.” |
| F-2-4 | Fixed | 404 has complete route-specific social metadata. |

### Copy findings

| Earlier ID | Status now | Evidence |
| --- | --- | --- |
| CW01 | Fixed | Job-first seven-word headline; no “powerful.” |
| CW02 | Fixed | Developer/coding-agent audience is visible. |
| CW03 | Fixed | Abstract scope slogan remains removed. |
| CW04 | Fixed | Process-and-children scope is explicit. |
| CW05 | Fixed | README names every redacted form. |
| CW06 | Fixed | Copy says precisely that the receipt omits the credential. |
| CW07 | Fixed | Visitor copy uses “time limit,” not “lease.” |
| CW08 | Fixed | Visitor instructions consistently use “alias.” |
| CW09 | Fixed | Process/children and time-limit wording is plain. |
| CW10 | Fixed | Sample heading names the result. |
| CW11 | Fixed in implementation | Isolated fake-data route exists; mobile presentation is F-3-1. |
| CW12 | **Regressed in substance via F-3-2/U16** | “Exactly” is gone, but “same fake-data workflow” is an untested parity claim. |
| CW13 | Fixed | Required sample-data action is present. |
| CW14 | Fixed | Control says “Copy install command.” |
| CW15 | Fixed | Network/file warning is split and under 22 words. |
| CW16 | Fixed | Vague “safety layer” copy remains absent. |
| CW17 | Fixed | README opening is three short sentences. |
| CW18 | Fixed | README audience is short and direct. |
| CW19 | Fixed | Process, streams, forms, and receipt statements are split. |
| CW20 | Fixed | Authorized-process warning is split. |
| CW21 | Fixed | “percent-encoded” is used consistently. |
| CW22 | Fixed | Copy says redaction limits leaks. |
| CW23 | Fixed | License form remains removed. |
| CW24 | Fixed | Direct non-sandbox warning remains. |
| CW25 | Fixed | Process-tree/time-limit wording remains. |
| CW26 | Fixed | README names all supported encoding forms. |

### Earlier claim findings

| Earlier ID | Status now | Evidence |
| --- | --- | --- |
| U01 | Fixed | Prompt-absence promise remains removed. |
| U02 | Fixed | Platform/backend/telemetry bundle remains removed. |
| U03 | Fixed | General keychain-at-rest promise remains removed; demo non-use is tested. |
| U04 | Fixed | Correct process-tree claim passes. |
| U05 | Fixed | Named redaction forms pass. |
| U06 | Fixed | Precise receipt-omission claim passes. |
| U07 | Fixed | Capability/prompt promise remains removed. |
| U08 | Fixed | Keychain-resolution landing promise remains removed. |
| U09 | Fixed | Prompt-content promise remains removed. |
| U10 | Fixed | Process-tree/time-limit wording and test agree. |
| U11 | Fixed | Both output streams are tested. |
| U12 | Fixed | Raw, percent, Base64, Base64url, and hex forms are tested. |
| U13 | Fixed | Landing receipt-field inventory remains removed. |
| U14 | Fixed | Receipt omission is tested. |
| U15 | Fixed | Browser fake-data isolation/reset is tested. |
| U16 | **Reopened by F-3-2** | Demo now says “same fake-data workflow” without a browser/CLI parity test. |
| U17 | Fixed | Daemon/account/vault/telemetry bundle remains removed. |
| U18 | Fixed | Visitor-facing Rust-version claim remains removed. |
| U19 | Fixed | Retained as an honest non-sandbox limitation. |
| U20 | Fixed | Retained as an honest authorized-command warning. |
| U21 | Fixed | “Safety layer stays free” remains removed. |
| U22 | Fixed | Ungated-feature marketing remains removed. |
| U23 | Fixed | Price/team-kit/release promises remain removed. |
| U24 | Fixed | Merchant-of-record claim remains removed. |
| U25 | Fixed | Refund/revocation claim remains removed. |
| U26 | Fixed | “Free core active” remains removed. |
| U27 | Fixed | License-storage state remains removed. |
| U28 | Fixed | No-value receipt behavior is tested. |
| U29 | Fixed | Broad README claim is split across passing process/output/receipt claims. |
| U30 | Fixed | Broad keychain-at-rest sentence remains removed. |
| U31 | Fixed | Broad CLI telemetry/store sentence remains removed. |
| U32 | Fixed | Retained as an honest limitation. |
| U33 | Fixed | Compiler-version claim remains removed. |
| U34 | Fixed | Future binary-release promise remains removed. |
| U35 | Fixed | Package contents/license test passes. |
| U36 | **Reopened by F-3-2** | `asc put` success is still not a listed, successful keychain test. |
| U37 | Fixed | Process-tree time-limit behavior passes. |
| U38 | Fixed | Streams/forms/receipt behavior is covered by three passing claims. |
| U39 | **Reopened by F-3-2** | Documented receipt commands lack a claim test. |
| U40 | **Reopened by F-3-2** | Remove parser test accepts an operational failure; no successful lifecycle claim exists. |
| U41 | **Reopened by F-3-2** | Documented help/exit-code contract lacks a manifest test. |
| U42 | **Reopened by F-3-2** | Required non-TTY rejection is not tested. |
| U43 | Fixed | Process tree, output, and receipt behaviors have passing claims. |
| U44 | Fixed | Receipt value omission passes. |
| U45 | Fixed | Retained as an honest authorized-process warning. |
| U46 | Fixed | Current `npm test` behavior matches README and passes. |
| U47 | Fixed | Same-origin/no-cookie/no-third-party test passes. |
| U48 | Fixed | License token/storage feature remains removed. |
| U49 | Fixed | MIT source/package claim passes. |
| U50 | Fixed | Lifetime-update wording remains removed. |
| U51 | Fixed | Label remains “STORE / STDIN INPUT.” |
| U52 | Fixed | Label remains “RECEIPTS / JSON.” |
| U53 | Fixed | Team rollout-kit promise remains removed. |
| U54 | Fixed | Receipt-policy promise remains removed. |
| U55 | Fixed | Home metadata uses credential/time-limit language. |
| U56 | Fixed | Visitor copy consistently says “redact.” |
| U57 | Fixed | Receipt omission is explicit and tested. |

## Quality gates

```text
npm ci                 PASS (60 packages, 0 vulnerabilities)
npm test               PASS (10 Rust, 2 Vitest, 29 Playwright; 5 intentional skips)
npm run build          PASS (target/release/asc and dist/site)
verify-url.sh           PASS on /, /demo/, /privacy/, /terms/
live Axe               0 serious/critical findings on 5 routes × 2 viewports
live link crawl         all expected links 200; unknown route 404
```

## Missed leverage

No additional AI, sync, or import feature is justified. The brief calls for a
narrow local credential boundary; sending credential context to an AI gateway
would widen it. JSON receipt output already provides the obvious export path.
The missing leverage is proof of the existing credential lifecycle, recorded in
F-3-2, not a new feature.

## What would make this perfect

Make the populated demo result visible without scrolling at both review sizes
and use a realistic agent-command fixture. Complete the claims manifest with
successful packaged-CLI tests for credential lifecycle, receipt commands/schema,
the documented duration, and interface behavior; either test browser/CLI sample
parity or describe the browser output as a static preview. Fit all three tested
facts into the mobile landing viewport. Then repeat the cold mobile/desktop,
clean-clone claim, sandbox, route, accessibility, and full history checks.
