# Adversarial first-read review 1 — FAIL

**Product:** Agent Secret Capsule

**Live site:** https://agent-secret-capsule.sociobot.in

**Repository revision reviewed:** `c9ee1997b8343876ccb2ba86d109e87a275b2008`

**Review date:** 2026-08-28 UTC

**Viewports:** 390×844 and 1440×900, each in a fresh Chromium context

## Verdict

**FAIL.** There are six blocking findings. The first screen does not identify the
user or present a sample-data action, no required web or CLI demo sandbox exists,
the claims manifest is absent, a process-scope guarantee contradicts the product,
the paid checkout returns 404, and unknown routes silently return the home page.

Ordinary tests pass, and the site has a distinct product-specific visual identity.
Those results do not offset the blocking first-read, demo, claim, security-copy,
checkout, and routing failures.

## Findings, ordered by severity

### B1 — BLOCKING: the cold first screen does not answer all three questions

Exact first-screen copy:

> “A narrow passage for powerful secrets.”
>
> “Let an agent run the command. Keep the credential out of prompts, terminal output, and tool traces.”

| Check | 390×844 | 1440×900 |
| --- | --- | --- |
| What does it do? | I can infer that it is a local command-line security tool, but the metaphorical headline does not name secret injection, redaction, expiry, or receipts. | Same. |
| For whom? | Not stated. “An agent” is the actor, not the user. | Not stated. |
| What should I click first? | “Install the CLI” is the first product action, not a sample path. | No usable hero action is visible; the action boxes begin at y=897, while their labels fall below the 900 px viewport. |

The headline also uses the banned marketing adjective “powerful.” A visitor must
infer the job from supporting copy and cannot identify the intended developer from
the screen. On desktop, the first action is below the fold. This fails the required
five-second comprehension check.

Concrete replacement:

- Headline: **“Give one agent command a temporary secret.”**
- Audience line: **“For developers running coding agents: inject one credential, redact command output, and save a no-value receipt.”**
- Primary action: **“Try it with sample data”**
- Adjacent outcome: **“See a sample credential redacted from output and omitted from the receipt.”**
- Facts: **“Stored in your OS keychain.” “No telemetry.” “Free CLI; optional team kit costs $19.”**

### B2 — BLOCKING: there is no one-click sample demo or isolated demo mode

The only demo control appears after the mechanism section, not on the first screen:

> “Try the observable boundary.”
>
> “Run fake command”

Direct checks found:

- `/?demo=1` and `/demo` both render the normal landing page.
- Neither URL shows “Demo — sample data, nothing is saved,” “Reset demo,” or “Start for real.”
- There is no `.factory/demo.md`, `examples/` sample, demo storage namespace, or demo route.
- `asc demo` in a fresh temporary directory exits 2 with `unrecognized subcommand 'demo'` and creates no demo output.
- The browser control only replaces prewritten text with `[REDACTED:ASC]`; it does not run the CLI and the claimed fake credential never exists in the DOM.
- The post-click screen shows one synthetic command rather than realistic sample data that demonstrates storing, leasing, expiry, redaction variants, and receipt inspection.

The browser animation made no network requests and changed no local/session storage,
but that only confirms that this animation has no persistence. It does not establish
an isolated demo for the real product or prove that real data is untouched.

Concrete fix: put **“Try it with sample data”** in the first screen and link it to
`/demo`. Add a persistent demo banner, Reset, and Start for real. For this CLI, ship
an `examples/` fixture and `asc demo`; run it in a newly created temporary directory,
show the output path, exercise the actual binary, and leave the user keychain and
normal receipt directory untouched. Document the command, reset behavior, sample,
and namespace in `.factory/demo.md`.

### B3 — BLOCKING: `.factory/claims.json` is absent and every public claim is unlisted

A clean detached worktree has no `.factory/claims.json`, and `rg '@claim:'` finds no
claim-tagged tests. Therefore there were no listed claim commands to run. `npm test`
passes, but its 9 Rust, 5 Vitest, and 14 Playwright tests are not a substitute for the
required claim inventory and one tagged observable test per claim.

Every `Uxx` item in the copy audit below is an individual **unlisted claim**. The
following ledger gives the required concrete action for each. Until the manifest and
tests exist, a visitor cannot tell which security, privacy, compatibility, licensing,
and behavior statements are actually enforced.

| ID | Exact claim and location | Concrete fix/test |
| --- | --- | --- |
| U01 | Landing: “Keep the credential out of prompts, terminal output, and tool traces.” | Add an actual-binary demo test that inspects the prompt fixture, stdout, stderr, and trace; assert the configured value and listed encodings are absent. |
| U02 | Landing: “Single Rust binary · macOS Keychain · Linux Secret Service · no telemetry” | Split this into atomic claims. Test packaged artifacts and keychain backends on supported OSes; intercept CLI and site network activity for telemetry. |
| U03 | Landing: “Secret stays in your OS keychain.” | Rewrite to say when it leaves the keychain, then test storage plus process injection with a real test keychain. |
| U04 | Landing: “Only the named process receives it.” | This is false; use the B4 rewrite and add a process-tree inheritance test. |
| U05 | Landing: “Raw + encoded output is scrubbed.” | Name the supported forms and test raw, percent, Base64, Base64url, and hex on stdout and stderr through the packaged CLI. |
| U06 | Landing: “Receipt records everything but value.” | Name the receipt fields and assert its JSON schema and absence of all secret variants. |
| U07 | Landing: “The agent can name the capability it needs without ever seeing the credential that grants it.” | Replace “capability” with “alias”; test that the prompt/fixture contains the alias but not the credential. |
| U08 | Landing: “cloudflare resolves inside your OS keychain.” | Test a stored alias through the real CLI against each supported keychain. |
| U09 | Landing: “The prompt contains only the alias.” | Test the complete bundled agent-tool input, not only a redaction helper. |
| U10 | Landing: “The value exists in one subprocess environment until exit or the hard TTL.” | Rewrite “one subprocess” to include children; test environment lifetime and expiry. |
| U11 | Landing: “Stdout and stderr are captured.” | Test both streams through the packaged CLI. |
| U12 | Landing: “Exact, URL, base64, base64url, and hex forms are replaced.” | Align “URL” with the README’s “percent-encoded”; test every named form and case. |
| U13 | Landing: “Time, alias, command, outcome, and redaction count remain.” | Assert these fields in a real receipt after success, failure, and expiry. |
| U14 | Landing: “The value never does.” | Assert raw and supported encoded values are absent from receipt files. |
| U15 | Landing: “This browser demo uses a fake value and never sends it anywhere.” | Either implement a real sample value and intercept every demo request, or honestly call the current panel a static illustration. |
| U16 | Landing: “It shows exactly what the CLI releases to an agent tool trace.” | Remove “exactly” until the browser uses the packaged CLI/demo output and a parity test compares the two. |
| U17 | Landing: “There is no daemon, account, cloud vault, or telemetry endpoint.” | Add source/package checks plus network interception; split the four claims. |
| U18 | Landing: “Rust 1.85+ builds one binary.” | Build from a fresh checkout with Rust 1.85 and assert the expected executable artifact. |
| U19 | Landing: “It is not a sandbox.” | Keep the warning and test the boundary by showing an authorized child can write and make a controlled network request. |
| U20 | Landing: “An authorized command can still send the credential … or pass it to a child process.” | Add a controlled boundary test that demonstrates child inheritance and a local intercepted request without exposing a real secret. |
| U21 | Landing: “The safety layer stays free.” | Replace “safety layer” with named free features and test license gating for each. |
| U22 | Landing: “Core storage, leases, redaction, JSON receipts, and accessibility are never gated.” | Test every listed action without a license and test the locked kit separately. |
| U23 | Landing: “A $19 one-time license unlocks the team rollout kit and supports signed binary releases and lifetime updates.” | Test live catalog price, successful sandbox checkout/license unlock, downloadable kit, and entitlement terms; remove unsupported release promises. |
| U24 | Landing: “Sociobot/Dodo is the merchant of record.” | Verify against the configured checkout response or link to the applicable purchase terms. |
| U25 | Landing: “Refunds are handled there and revoke the license automatically.” | Add a billing sandbox test for refund-to-revocation behavior. |
| U26 | Landing status: “Free core active.” | Test the no-license state and free operations. |
| U27 | Landing status: “No license saved.” | Assert a fresh context has no license key and that demo mode never writes one. |
| U28 | Landing footer: “No secret in the receipt.” | Test receipt files for raw and supported encoded values. |
| U29 | README: “Agent Secret Capsule (`asc`) lets a coding or browser agent run one named command … without putting the raw value … in the audit receipt.” | Add an end-to-end packaged-CLI claim test covering the complete statement, or split it into smaller tested claims. |
| U30 | README: “Secrets stay in the operating-system keychain.” | Rewrite to disclose process injection, then test at-rest storage on supported OSes. |
| U31 | README: “There is no telemetry and no hosted secret store.” | Run the packaged CLI in a network-intercepted sandbox and inspect configured endpoints. |
| U32 | README: “It is a containment layer, not a sandbox or a secret-manager replacement.” | Keep this as an explicit limitation and link it to the controlled boundary test from U19/U20. |
| U33 | README: “Build the single binary with Rust 1.85 or newer.” | Add a clean Rust 1.85 build job and artifact-count assertion. |
| U34 | README: “Factory releases will provide checksummed binaries for macOS and Linux.” | Future tense is not current evidence; remove until releases exist, then test checksum publication for each artifact. |
| U35 | README: “The repository is ready for `cargo package -p agent-secret-capsule` …” | Add the exact package command as a claim test in a clean checkout. |
| U36 | README: “Store a credential without placing it in shell history.” | Test the documented stdin command in a temporary shell history and assert the value is absent. |
| U37 | README: “Run exactly one program with a 30-second lease.” | Test the packaged CLI’s selected process tree and measured timeout. |
| U38 | README: “`asc` captures both output streams, removes the raw secret and common exact encodings, then preserves the program's exit status.” | Add one end-to-end test that asserts both streams, each named encoding, and exit-code propagation. |
| U39 | README: “Inspect no-value receipts or automate with JSON.” | Test human and JSON receipt output and schema. |
| U40 | README: “Remove a credential.” | Test put/remove/read failure using a real test keychain. |
| U41 | README: “Run `asc --help` or `asc <command> --help` for flags, exit codes, and examples.” | Test every documented subcommand help and referenced exit code. |
| U42 | README: “There are no interactive prompts when stdin is not a terminal; `put` requires `--stdin` in CI.” | Add non-TTY black-box tests for prompt absence and exit behavior. |
| U43 | README: “`asc` injects the secret only into the selected process environment and its descendants …” | Test parent/child inheritance, cleanup after exit/timeout, all named redactions, and receipt creation through the binary. |
| U44 | README: “It never writes secret values to receipts.” | Inspect the on-disk receipt log for raw and encoded forms. |
| U45 | README: “An authorized process can still send the credential … or pass it to a child.” | Add the controlled boundary test from U20. |
| U46 | README: “`npm test` runs Rust tests, site unit checks, and Playwright accessibility and browser tests.” | Add a script-composition test or keep this synchronized with package scripts; current clean execution confirms it only for this revision. |
| U47 | README: “The static site has no runtime CDN, tracking, or account system.” | Intercept all fresh-page and demo requests; assert same-origin only and inspect scripts/storage. |
| U48 | README: “License tokens … stay in browser `localStorage` and are sent only to the Sociobot license verification endpoint.” | Test storage keys and intercept the entire token flow, including errors and redirects. |
| U49 | README: “MIT.” | Verify the distributed package and repository include the exact MIT license. |
| U50 | Landing list: “Lifetime product updates” | Define “lifetime” in terms and test the purchased entitlement; otherwise remove it. |
| U51 | Landing label: “STORE / NO HISTORY” | Add the shell-history test from U36 or rewrite to “STORE / STDIN INPUT.” |
| U52 | Landing label: “AUDIT / NO VALUES” | Add the receipt test from U28/U44. |
| U53 | Landing list: “Team threat-model and rollout checklist” | Test that a valid license downloads a kit containing this material. |
| U54 | Landing list: “Receipt-retention policy starter” | Test that the licensed download contains this material. |
| U55 | Home description: “Give one agent command an expiring secret lease.” | Add an end-to-end test for one alias, one process tree, and enforced expiry. |
| U56 | Home description: “Scrub its output.” | Replace “scrub” with the exact supported redaction forms and test both output streams. |
| U57 | Home description: “Keep a receipt without keeping the value.” | Assert receipt creation and absence of the configured value and its supported encodings. |

### B4 — BLOCKING: the most prominent process-scope guarantee is false

> “Only the named process receives it”

The README says the secret is injected into the selected process “and its
descendants,” the boundary says it can be passed to a child, and the implementation
uses a normal inherited environment. A developer could choose this tool specifically
because the hero guarantees a narrower boundary than the CLI provides.

Concrete fix: replace it with **“The selected process and its children receive the
credential.”** Add a claim test that starts a child, confirms inheritance, expires
the lease, and confirms the process tree is stopped. Do not use “only” for a process
tree boundary.

### B5 — BLOCKING: the paid primary flow is a dead link

> “Buy a supporter license”

The rendered link requests
`https://api.sociobot.in/api/v1/products/agent-secret-capsule/checkout` and returns
HTTP 404 with `{"error":"enabled factory product","status":404}`. A visitor can be
shown a price and asked to buy, but cannot begin checkout.

Concrete fix: register/enable the product through the approved Sociobot billing API,
then add a live-link test that expects the documented checkout response or redirect.
Until that succeeds, remove the buy action and state that purchase is unavailable.

### B6 — BLOCKING: broken routing hides every missing route behind the home page

`/not-a-real-route`, `/demo`, `/robots.txt`, `/sitemap.xml`, and `/favicon.ico` all
return HTTP 200 `text/html` with the home title and home `h1`. This is a soft-404
catch-all, not a designed 404. It also makes absent discovery assets appear healthy.

Concrete fix: add real `/demo` and `/404` pages, return an actual 404 for unknown
paths, and narrow the navigation fallback to supported client routes. Serve real
`robots.txt`, `sitemap.xml`, and favicon files with correct content types. Test direct
loads, reloads, back/forward, and an unknown URL.

### M1 — MAJOR: route metadata and discovery assets are incomplete

The home, Privacy, and Terms pages have titles, descriptions, `lang`, one `h1`, and
`main`. The home title suffix, “one command, one secret lease,” uses unexplained
`lease` jargon instead of saying what the product does in plain words. All three
lack canonical links, Open Graph metadata, Twitter card
metadata, favicon declarations, and an apple-touch icon. No real 1200×630 social
image, `robots.txt`, or `sitemap.xml` is served. `/demo` also reuses the home title
instead of `Demo — Agent Secret Capsule`.

Concrete fix: add per-route canonicals and social metadata, a product-art 1200×630
image, SVG/favicon and 180 px apple-touch assets, real robots/sitemap files, and the
required demo title. Rename the home title to **“Agent Secret Capsule — give one
command a credential.”** Add metadata assertions for every route.

### M2 — MAJOR: navigation, footer, section order, and focus do not follow the required skeleton

- The landing header has no Demo or Privacy link; legal headers use a different nav.
- The landing footer has Privacy and Terms, but Privacy omits its own Privacy link and
  Terms omits its own Terms link.
- No footer says “Built by Param Factory” or includes a build id.
- The live product preview comes after “How it works,” rather than immediately after
  the first screen.
- Navigating Privacy → Terms and using Back leaves focus on `<body>`, not the new
  route’s `<h1>`; there is no route announcement region.

Concrete fix: use one header/footer skeleton on all routes, add Demo/Privacy/Terms as
required, include factory attribution plus version/build id, move the working demo
before “How it works,” and focus/announce the route `h1` after navigation and history
changes.

### M3 — MAJOR: multiple copy units exceed the limit, use jargon, or overclaim

The full count audit follows. Specific flagged rewrites:

| Flag | Exact text | Why it fails | Proposed rewrite |
| --- | --- | --- | --- |
| CW01 | “A narrow passage for powerful secrets.” | Metaphor; “powerful” is banned; does not state the job. | “Give one agent command a temporary secret.” |
| CW02 | “Let an agent run the command.” | Does not name developers or coding/browser agents; “the command” lacks context. | “For developers running coding agents: inject one credential, redact command output, and save a no-value receipt.” |
| CW03 | “The scope is the safety feature.” | Abstract security jargon. | “A smaller scope reduces where the credential can appear.” |
| CW04 | “Only the named process receives it.” | Contradicts child inheritance. | “The selected process and its children receive the credential.” |
| CW05 | “Raw + encoded output is scrubbed.” | “Encoded” is unbounded and “scrubbed” conflicts with “redacted.” | “Exact raw, percent-encoded, Base64, Base64url, and hex values are redacted.” |
| CW06 | “Receipt records everything but value.” | “Everything” is false; only named fields are stored. | “The receipt stores time, alias, executable, outcome, and redaction count. It omits the configured value.” |
| CW07 | “One lease in. One receipt out.” | “Lease” is unexplained product jargon. | “Run one command. Save one no-secret receipt.” |
| CW08 | “The agent can name the capability it needs …” | “Capability” is jargon and inconsistent with “alias.” | “The agent names an alias. ASC retrieves the credential without adding it to the prompt.” |
| CW09 | “The value exists in one subprocess environment until exit or the hard TTL.” | “Subprocess” and “hard TTL” slow a first read; “one” obscures descendants. | “ASC removes the credential when the command exits or reaches its time limit.” |
| CW10 | “Try the observable boundary.” | The heading does not name a result. | “See what ASC redacts from command output.” |
| CW11 | “This browser demo uses a fake value and never sends it anywhere.” | “It” is ambiguous and the panel does not actually process a value. | “This static illustration makes no network request.” |
| CW12 | “It shows exactly what the CLI releases …” | “Exactly” overstates a scripted text swap. | “This panel illustrates redacted output; it does not run the CLI.” |
| CW13 | “Run fake command” | It is not the required sample-data action and implies execution that does not occur. | “View redacted sample output” until a real “Try it with sample data” demo exists. |
| CW14 | “Copy” | The button does not name what will be copied. | “Copy install command” |
| CW15 | “An authorized command can still send … child process.” | 29 words, above the 22-word cap. | “An authorized command can send the credential over the network or write it to a file. It can also transform it or pass it to a child.” |
| CW16 | “The safety layer stays free.” | “Safety layer” is vague and not used elsewhere. | “Core CLI features remain free.” |
| CW17 | README opening sentence (34 words) | Four ideas in one sentence. | “Agent Secret Capsule lets a coding or browser agent run one command with one credential. It redacts the value before printing output or writing a receipt.” |
| CW18 | README audience sentence (27 words) | Above the cap and uses “conversational context.” | “For developers whose coding agents must make authorized API calls. The credential stays outside the conversation.” |
| CW19 | README `asc injects … captured output` sentence (38 words) | Above the cap and combines injection, cleanup, timeout, and redaction. | “ASC injects the credential into the selected process tree until it exits or times out. It captures both output streams. It redacts exact raw, percent-encoded, Base64, Base64url, and hex forms.” |
| CW20 | README authorized-process warning (37 words) | Above the cap and contains four failure modes. | “An authorized process can still send the credential or derived data over the network. It can write the value, transform it, or pass it to a child.” |
| CW21 | “Exact, URL, base64, base64url, and hex forms are replaced.” | `URL` is imprecise and differs from README terminology. | “Exact raw, percent-encoded, Base64, Base64url, and hex forms are redacted.” |
| CW22 | “Redaction is containment.” | Abstract and may imply that redaction contains every leak path. | “Redaction limits accidental output leaks.” |
| CW23 | “Have a license? Paste it” | A question and instruction are used as the form label instead of naming the field. | Label it “License token” and add helper text “Paste the token from your receipt.” |
| CW24 | “It is a containment layer …” | `containment layer` is unexplained jargon. | “It reduces accidental output leaks. It does not replace a sandbox or secret manager.” |
| CW25 | “Run exactly one program with a 30-second lease.” | “Exactly one” conflicts with inherited child processes. | “Run one selected process tree with a 30-second time limit.” |
| CW26 | “common exact encodings” | The phrase does not tell the reader which encodings are covered. | “raw, percent-encoded, Base64, Base64url, and hex forms” |

Terminology is inconsistent:

| Concept | Current terms | Use consistently |
| --- | --- | --- |
| Protected item | secret, credential, value, token | `credential`; use `value` only for its bytes and `license token` only for billing |
| Execution scope | command, program, process, subprocess, process tree, descendants | `selected process and its children` |
| Output operation | scrub, redact, remove, replace | `redact` |
| Receipt | receipt, audit receipt, no-value receipt | `no-value receipt` |
| Stored identifier | name, secret name, alias, capability | `alias` |
| Time limit | lease, TTL, hard TTL, lease timeout | `time limit`; define `TTL` once in CLI reference |

### M4 — MAJOR: several mobile touch targets are shorter than 44 px

At 390 px, the license-panel Privacy and Terms links are 19 px high, and the footer
Source, Privacy, and Terms links are 21 px high. The main buttons meet the target.

Concrete fix: give every interactive link at least a 44×44 px hit area without
changing its visual text size, then add a mobile bounding-box assertion.

## Complete copy audit

Counts treat hyphenated terms, versions, paths, and slash terms as one word. Commands
are excluded from sentence counts and are checked as interface copy. `Uxx` means the
sentence is also an unlisted claim from B3; `CWxx` links to a rewrite above.

### Landing-page sentences

| # | Sentence | Words | Flags |
| --- | --- | ---: | --- |
| L01 | A narrow passage for powerful secrets. | 6 | CW01 |
| L02 | Let an agent run the command. | 6 | CW02 |
| L03 | Keep the credential out of prompts, terminal output, and tool traces. | 11 | U01 |
| L04 | The scope is the safety feature. | 6 | CW03 |
| L05 | Secret stays in your OS keychain. | 6 | U03 |
| L06 | Only the named process receives it. | 6 | U04, CW04, false scope |
| L07 | Raw + encoded output is scrubbed. | 5 | U05, CW05 |
| L08 | Receipt records everything but value. | 5 | U06, CW06 |
| L09 | One lease in. | 3 | CW07 |
| L10 | One receipt out. | 3 | CW07 |
| L11 | The agent can name the capability it needs without ever seeing the credential that grants it. | 16 | U07, CW08 |
| L12 | `cloudflare` resolves inside your OS keychain. | 6 | U08 |
| L13 | The prompt contains only the alias. | 6 | U09 |
| L14 | The value exists in one subprocess environment until exit or the hard TTL. | 13 | U10, CW09 |
| L15 | Stdout and stderr are captured. | 5 | U11 |
| L16 | Exact, URL, base64, base64url, and hex forms are replaced. | 9 | U12, CW21 |
| L17 | Time, alias, command, outcome, and redaction count remain. | 8 | U13 |
| L18 | The value never does. | 4 | U14 |
| L19 | Try the observable boundary. | 4 | CW10 |
| L20 | This browser demo uses a fake value and never sends it anywhere. | 12 | U15, CW11 |
| L21 | It shows exactly what the CLI releases to an agent tool trace. | 12 | U16, CW12 |
| L22 | Waiting for a local command… | 5 | — |
| L23 | Inspect it. | 2 | — |
| L24 | Build it. | 2 | — |
| L25 | Keep it local. | 3 | — |
| L26 | There is no daemon, account, cloud vault, or telemetry endpoint. | 10 | U17 |
| L27 | Rust 1.85+ builds one binary. | 5 | U18 |
| L28 | Redaction is containment. | 3 | CW22 |
| L29 | It is not a sandbox. | 5 | U19 |
| L30 | An authorized command can still send the credential over the network, write it to a file, transform it into an unknown encoding, or pass it to a child process. | 29 | **over 22**, U20, CW15 |
| L31 | Review the exact command and endpoint. | 6 | — |
| L32 | Use a separate network/process sandbox for hostile code. | 8 | — |
| L33 | The safety layer stays free. | 5 | U21, CW16 |
| L34 | Support the release. | 3 | — |
| L35 | Core storage, leases, redaction, JSON receipts, and accessibility are never gated. | 11 | U22 |
| L36 | A $19 one-time license unlocks the team rollout kit and supports signed binary releases and lifetime updates. | 17 | U23 |
| L37 | Sociobot/Dodo is the merchant of record. | 6 | U24 |
| L38 | Refunds are handled there and revoke the license automatically. | 9 | U25 |
| L39 | Have a license? | 3 | CW23 |
| L40 | Paste it. | 2 | CW23 |
| L41 | Free core active. | 3 | U26 |
| L42 | No license saved. | 3 | U27 |
| L43 | One command. | 2 | — |
| L44 | One lease. | 2 | unexplained term |
| L45 | No secret in the receipt. | 5 | U28 |

### Landing headings, facts, labels, and actions that are not sentences

| Copy | Words | Check |
| --- | ---: | --- |
| Skip to main content | 4 | Clear |
| ASC / AGENT SECRET CAPSULE | 4 | Clear wordmark |
| Mechanism | 1 | Vague out of context; use “How secret leasing works” |
| Install | 1 | Clear navigation destination |
| Boundary | 1 | Vague out of context; use “Security limits” |
| Get a license | 3 | Result-naming action, but its downstream buy link is dead |
| LOCAL SECURITY TOOL | 3 | Broad; use “CLI FOR CODING AGENTS” |
| Install the CLI | 3 | Result-naming action; not the required first demo action |
| Inspect the source | 3 | Result-naming action |
| Single Rust binary · macOS Keychain · Linux Secret Service · no telemetry | 10 | U02; four claims compressed into one fact line |
| FIG. 01 | 1 | Clear figure label |
| LEASE CLOSED / 0 values retained / receipt asc-1787… | 6 | Example state; `lease` is unexplained and “0 values retained” needs U28 coverage |
| THE MECHANISM | 2 | Vague out of context |
| Select / Lease / Scrub / Receipt | 4 | Use “Select / Run / Redact / Review receipt” |
| CONTROLLED DEMO | 2 | Misnames a scripted illustration; use “STATIC OUTPUT EXAMPLE” until the demo is real |
| Run fake command | 3 | CW13 |
| INSTALL / v0.1.0 | 2 | Clear |
| Copy | 1 | CW14; not result-naming |
| STORE / NO HISTORY | 3 | U51 |
| LEASE / 30 SECONDS | 3 | Use “RUN / 30-SECOND LIMIT” |
| AUDIT / NO VALUES | 3 | U52 |
| Read the full CLI reference | 5 | Result-naming action |
| HONEST BOUNDARY | 2 | Vague out of context; use “Security limits” |
| ONE-TIME SUPPORTER LICENSE | 3 | Clear |
| Team threat-model and rollout checklist | 5 | U53 |
| Receipt-retention policy starter | 3 | U54 |
| Lifetime product updates | 3 | U50; undefined entitlement |
| Buy a supporter license | 4 | Result-naming action; dead destination |
| Verify license | 2 | Result-naming action |
| Download the team rollout kit | 5 | Result-naming action |

The home meta description adds three search-facing sentences: “Give one agent
command an expiring secret lease.” (8 words, U55), “Scrub its output.” (3 words,
U56; inconsistent with `redact`), and “Keep a receipt without keeping the value.”
(7 words, U57). The hero alt text is “A monolithic concrete capsule divided by one
narrow seam of living moss” (11 words) and clearly describes the image.

### README sentences

| # | Sentence | Words | Flags |
| --- | --- | ---: | --- |
| R01 | Agent Secret Capsule (`asc`) lets a coding or browser agent run one named command with one selected credential without putting the raw value in a prompt, shell history, captured stdout/stderr, or the audit receipt. | 34 | **over 22**, U29, CW17 |
| R02 | Secrets stay in the operating-system keychain. | 6 | U30; omits runtime process injection |
| R03 | There is no telemetry and no hosted secret store. | 9 | U31 |
| R04 | This is for developers who need an agent to make an authorized API call but do not want to hand the credential to the agent's conversational context. | 27 | **over 22**, CW18 |
| R05 | It is a containment layer, not a sandbox or a secret-manager replacement. | 12 | U32, CW24 |
| R06 | Build the single binary with Rust 1.85 or newer: | 9 | U33 |
| R07 | Factory releases will provide checksummed binaries for macOS and Linux. | 10 | U34; future promise |
| R08 | The repository is ready for `cargo package -p agent-secret-capsule`; publishing is performed by the factory, not from a development checkout. | 20 | U35 |
| R09 | Store a credential without placing it in shell history: | 9 | U36 |
| R10 | Run exactly one program with a 30-second lease. | 8 | U37, CW25 |
| R11 | `asc` captures both output streams, removes the raw secret and common exact encodings, then preserves the program's exit status: | 19 | U38, CW26 |
| R12 | Inspect no-value receipts or automate with JSON: | 7 | U39 |
| R13 | Remove a credential: | 3 | U40 |
| R14 | Run `asc --help` or `asc <command> --help` for flags, exit codes, and examples. | 13 | U41 |
| R15 | There are no interactive prompts when stdin is not a terminal; `put` requires `--stdin` in CI. | 16 | U42 |
| R16 | `asc` injects the secret only into the selected process environment and its descendants, removes it when that process tree exits, enforces a lease timeout, and redacts the exact raw, percent-encoded, base64, base64url, and hex values from captured output. | 38 | **over 22**, U43, CW19 |
| R17 | It never writes secret values to receipts. | 7 | U44 |
| R18 | An authorized process can still send the credential—or data derived from it—to the network, write it to a file, transform it into an encoding this version does not recognize, or pass it to a child. | 37 | **over 22**, U45, CW20 |
| R19 | Review the command and its network scope. | 7 | — |
| R20 | For hostile code, use a separate sandbox as well. | 9 | — |
| R21 | `npm test` runs Rust tests, site unit checks, and Playwright accessibility and browser tests. | 14 | U46 |
| R22 | The static site has no runtime CDN, tracking, or account system. | 11 | U47 |
| R23 | License tokens entered on the pricing panel stay in browser `localStorage` and are sent only to the Sociobot license verification endpoint. | 21 | U48 |
| R24 | MIT. | 1 | U49 |
| R25 | See LICENSE. | 2 | Clear |

README headings are: “Agent Secret Capsule” (3), “Install” (1), “Usage” (1),
“Security boundary” (2), “Develop and verify” (3), “Repository layout” (2), and
“License” (1). They are understandable in context; “Usage” is generic but conventional.
No README button copy exists. No additional banned words were found beyond
“powerful” in the landing headline.

## Structure, accessibility, privacy, and behavior evidence

### Passed checks

- Fresh live contexts at both widths returned HTTP 200 with no console or page errors.
- Home, Privacy, and Terms each have `lang="en"`, one `h1`, one `main`, a title,
  a meta description, and alt text on the hero image.
- The clean Playwright axe integration found zero serious/critical violations on all
  three routes at desktop and 390 px.
- `/opt/fleet/lib/verify-url.sh` passed: title, language, one `h1`, main landmark,
  zero missing image alts, zero unlabeled buttons, and zero console errors.
- The home page has no horizontal overflow at 390 px. Reduced-motion behavior is
  covered by the existing browser implementation/test path.
- A fresh page plus the browser illustration made seven requests, all to the product
  origin. Clicking the illustration added no request and changed no web storage.
- After one online load, a forced offline reload retained the title and main content.
- The distinct dark concrete-and-moss visual identity matches `.factory/design.md`;
  it does not resemble a generic gradient/card SaaS template.
- Internal home/hash/legal links and both GitHub links returned 200.
- Live responses include HSTS, `nosniff`, strict-origin referrer policy, restrictive
  permissions policy, and a self-based CSP. No CSP console violation was observed.

### Failed or incomplete checks

| Check | Result |
| --- | --- |
| One-click sample path | Missing; BLOCKING B2 |
| Real demo isolation/reset | Cannot be confirmed because no demo mode exists |
| Claim manifest/tagged tests | Missing; BLOCKING B3 |
| Checkout | HTTP 404; BLOCKING B5 |
| Designed 404 and correct unknown-route status | Missing; BLOCKING B6 |
| Canonical/OG/Twitter/favicon/apple-touch | Missing on every tested route |
| robots/sitemap | Soft-404 home HTML, not the required files |
| Focus on route change/back | Active element is `<body>`, not the route `h1` |
| Consistent header/footer | Missing Demo/current legal links, factory attribution, and build id |
| 44 px touch targets | Five legal/footer links fail |

## Verification commands and outcomes

From a clean detached worktree at the reviewed revision:

```text
.factory/claims.json                         MISSING
rg '@claim:'                                 no matches
npm ci                                       PASS (60 packages, 0 vulnerabilities)
npm test                                     PASS
  Rust                                       9 passed
  Vitest                                     5 passed
  Playwright desktop + mobile                14 passed
npm run build                                PASS; produced target/release/asc and dist/site
```

CLI demo check in a fresh temporary directory:

```text
$ ASC_HOME=<temp>/asc-home cargo run --quiet -p agent-secret-capsule -- demo
error: unrecognized subcommand 'demo'
exit 2
```

Live route/link checks:

```text
GET /not-a-real-route                        200 text/html (home page)
GET /robots.txt                              200 text/html (home page)
GET /sitemap.xml                             200 text/html (home page)
GET /favicon.ico                             200 text/html (home page)
GET Sociobot checkout URL                    404 application/json
```

No product code was modified during this review.
