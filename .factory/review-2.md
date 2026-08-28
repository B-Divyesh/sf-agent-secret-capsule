# Adversarial first-read review 2 — FAIL

**Product:** Agent Secret Capsule  
**Live URL:** https://agent-secret-capsule.sociobot.in/  
**Revision:** \`0f4225bb1806d4fa8333248fe18b39fa719aeb8c\`  
**Date:** 2026-08-28 UTC

## Verdict

**FAIL.** Two blocking and two minor findings remain. The desktop initial viewport does not show the audience explanation or the primary sample action. Several visitor-facing claims remain outside \`.factory/claims.json\`.

## Cold first read

Fresh 390×844: I can answer all three questions: it gives an agent command a temporary credential; it is for developers running coding agents; click **“Try it with sample data.”**

Fresh 1440×900: I can infer the purpose from **“Give one agent command a temporary credential.”** I cannot read whom it is for: the audience text begins at y=847 and is cut after **“For developers running coding agents: run one”**. I cannot identify what to click first: **“Try it with sample data”** is below the viewport. **“CLI FOR CODING AGENTS”** names a category, not the human visitor.

## Findings

### F-2-1 — BLOCKING — desktop first screen hides the user and first action

**Location:** live \`/\` at 1440×900; \`site/src/styles.css:108–134\`.

> “Give one agent command a temporary credential.”
>
> “For developers running coding agents: run one command, redact its output, and save a receipt without the credential.”
>
> “Try it with sample data”

The 108px heading wraps to six lines in the left grid column. The CTA is below the initial desktop viewport, so the mandatory first-screen action is absent. Reduce the desktop heading or widen the copy column until the full audience line and CTA end within 900px. Add a 1440×900 Playwright assertion for both bounding boxes.

**History:** reopens review-1 **B1**. Mobile is repaired; desktop is not.

### F-2-2 — BLOCKING — visitor-facing claims are unlisted

The manifest has six passing entries: \`demo-isolation\`, \`offline-reload\`, \`cli-demo\`, \`redaction-forms\`, \`process-tree\`, and \`captured-output-receipt\`. These current claims have no matching manifest entry and observable test:

| Location | Exact unlisted claim | Concrete fix |
| --- | --- | --- |
| Home description | “Give a coding agent one temporary credential **without placing its value in the prompt** or receipt.” | Remove “prompt,” or test documented agent-tool input for absence of the configured value. |
| Home fact | “Free and open source” | Add a license/package claim test or remove it. |
| README sample | “It does not read your keychain or \`ASC_HOME\`.” | Extend \`cli-demo\` to prove both conditions or remove the keychain promise. |
| README install | “Build with Rust 1.85 or newer.” | Add a clean Rust-1.85 build claim/CI test or remove it. |
| Privacy | “This site uses no analytics, advertising cookies, or third-party scripts.” | Add landing/demo request, cookie, and storage claim coverage or remove/narrow it. |
| Terms | “Agent Secret Capsule is open-source software provided under the MIT License.” | Cover with the explicit license/package claim. |

The same-origin demo request assertion is useful, but it neither lists nor fully proves the prompt, keychain, compiler, cookie, or license claims. This reopens review-1 **B3** and the substance of **U01**, **U03**, **U18/U33**, and **U47**.

### F-2-3 — MINOR — a README sentence exceeds 22 words; lease jargon returns

**Location:** README Security limits; landing How-it-works label.

> “An authorized process can send a credential over the network, write it to a file, transform it, or pass it to a child.”

This is 23 words and joins four consequences. Rewrite: **“An authorized process can send the credential over the network or write it to a file. It can also transform the credential or pass it to a child.”**

**“HOW SECRET LEASING WORKS”** uses unexplained “leasing” while the product otherwise says “time limit.” Rewrite it to **“HOW THE CREDENTIAL TIME LIMIT WORKS.”** This returns review-1 **CW07**; the prior long warning **CW15** was otherwise repaired.

### F-2-4 — MINOR — 404 lacks social metadata

**Location:** live \`/not-a-real-route\` (real HTTP 404); \`site/404.html\`.

The 404 has title, description, canonical, favicon, h1, header/footer, and recovery links, but no \`og:type\`, \`og:title\`, \`og:description\`, \`og:image\`, or Twitter-card metadata. Add 404-specific tags and a metadata test. Review-1 **M1** is only partly fixed.

## Demo, sandbox, and privacy verification

**Pass.** From a fresh mobile context, one click opened \`/demo/\` directly to populated redacted stdout/stderr, a succeeded result, an expiry sample, and a sample receipt. The persistent banner read **“Demo — sample data, nothing is saved”** and provided **Reset demo** and **Start for real**.

After **Run sample again**, storage was exactly \`sessionStorage["demo:asc:run-count"] = "1"\`; localStorage was empty. Reset cleared both. The landing-to-demo request log was same-origin only and there were no console/page errors.

I ran release \`asc --json demo\` from a new temporary directory with a separate sentinel \`ASC_HOME\`. It created a distinct \`/tmp/asc-demo-*\` directory, returned two receipts, printed \`[REDACTED:ASC]\` for both streams, and left the sentinel directory empty.

## Claim tests from a clean clone

| Claim | Command | Result |
| --- | --- | --- |
| \`demo-isolation\` | \`npm run test:claim -- --grep @claim:demo-isolation\` | PASS |
| \`offline-reload\` | \`npm run test:claim -- --grep @claim:offline-reload\` | PASS |
| \`cli-demo\` | \`npm run test:claim -- --grep @claim:cli-demo\` | PASS |
| \`redaction-forms\` | \`cargo test --locked redacts_raw_and_encoded_forms\` | PASS |
| \`process-tree\` | \`npm run test:claim -- --grep @claim:process-tree\` | PASS |
| \`captured-output-receipt\` | \`npm run test:claim -- --grep @claim:captured-output-receipt\` | PASS |

\`npm test\` also passed: 10 Rust, 2 Vitest, and 21 Playwright tests (3 mobile-only skips).

## Copy audit

Hyphenated compounds count as one word. Commands, paths, terminal output, field labels, version labels, and controls are not sentences.

### Landing sentences

| Sentence | Words | Flag |
| --- | ---: | --- |
| Give one agent command a temporary credential. | 7 | — |
| For developers running coding agents: run one command, redact its output, and save a receipt without the credential. | 18 | — |
| See a fake credential redacted and a no-value receipt. | 9 | — |
| One chosen path for one credential. | 6 | — |
| See the command result first. | 5 | — |
| The CLI ships with \`asc demo\`. | 6 | — |
| It creates fake sample receipts in a new temporary directory. | 10 | — |
| Use a local alias in the agent tool input. | 9 | — |
| Store an alias locally. | 4 | — |
| Use the alias in the agent tool input. | 9 | — |
| The selected process and its children receive the credential until exit or the time limit. | 14 | — |
| ASC captures both output streams and replaces matching credential forms before printing them. | 13 | — |
| A no-value receipt omits the credential. | 6 | — |
| Build the CLI. / Run the sample. | 3 / 3 | — |
| Use the bundled demo before storing a real credential. | 10 | — |
| Redaction limits output leaks. / It is not a sandbox. | 4 / 5 | — |
| An authorized command can send the credential over the network or write it to a file. | 16 | — |
| It can also transform it or pass it to a child. | 11 | — |
| Review the command and endpoint. / Use a separate network and process sandbox for hostile code. | 5 / 10 | — |
| One command. / One no-value receipt. | 2 / 3 | — |

### README sentences

| Sentence | Words | Flag |
| --- | ---: | --- |
| Agent Secret Capsule (\`asc\`) gives one selected process and its children a temporary credential. | 14 | — |
| It captures command output before printing it. / It writes a receipt without the credential value. | 7 / 8 | — |
| For developers whose coding agents need an authorized API call. / Use a local alias in the agent tool input. | 10 / 9 | — |
| Run the bundled sample before storing a real credential. | 9 | — |
| The command uses a fake credential. / It creates a new temporary directory with sample no-value receipts and prints its path. | 6 / 14 | — |
| It does not read your keychain or \`ASC_HOME\`. | 9 | F-2-2 |
| Delete that directory to reset the command-line sample. | 8 | — |
| The web sample is at \`/demo/\` or \`/?demo=1\`. / It uses browser storage keys with the \`demo:asc\` prefix. / Reset demo clears those sample keys. | 8 / 9 / 6 | — |
| Build with Rust 1.85 or newer. | 6 | F-2-2 |
| Store a credential from standard input. / Run a selected process tree with a 30-second time limit. / Inspect receipts or automate with JSON. | 6 / 10 / 6 | — |
| Run \`asc --help\` or \`asc <command> --help\` for flags, exit codes, and examples. / When standard input is not a terminal, \`put\` requires \`--stdin\`. | 13 / 10 | — |
| ASC gives the credential to the selected process and its children until exit or the time limit. | 15 | — |
| It redacts raw, percent-encoded, Base64, Base64url, and hex matches from captured stdout and stderr. | 14 | — |
| A no-value receipt omits the credential value. / This is not a sandbox. | 7 / 5 | — |
| An authorized process can send a credential over the network, write it to a file, transform it, or pass it to a child. | 23 | F-2-3 |
| Review the exact command and endpoint. / Use a separate sandbox for hostile code. | 6 / 7 | — |
| \`npm test\` runs Rust tests, site unit checks, and browser checks. / Claim tests are listed in \`.factory/claims.json\`. | 11 / 7 | — |
| Build the static site with \`npm run build:site\`; it writes \`dist/site\` for deployment. | 13 | — |

Controls are result-naming verbs. No banned marketing adjective was found.

## Structure and history

Routes \`/\`, \`/demo/\`, \`/privacy/\`, and \`/terms/\` have expected titles, one h1, description, canonical, favicon, OG/Twitter metadata, and h1 focus. Privacy → Terms → Back restored the correct h1 focus. Internal links returned 200 and the unknown route correctly returned 404; GitHub source links returned 200. Header/footer, skip link, mobile targets, and concrete-and-moss identity are present.

Read history: \`review-1.md\`, \`polish-1.md\`, \`verification-1.md\`, \`verification-2.md\`, and handoff. B2, B4–B6, M2, M4, CW01–CW06, CW08–CW26, U02, U04–U17, U19–U32, U34–U46, U48, and U50–U57 are confirmed fixed/removed. B1, B3, CW07, U01, U03, U18/U33, and U47 are re-opened above; M1 is partial in F-2-4. The prior parser defect is fixed by current clean-clone parser tests.

## Missed leverage

No finding. This narrow local CLI already has the implied sample and receipt/JSON workflow. AI, sync, or import would widen the secret boundary without a brief-supported need.

## What would make this perfect

Keep the desktop audience and sample CTA in the first viewport; make every remaining public claim testable and listed (or remove it); split the long warning and rename the lease heading; add 404 social metadata; then repeat the full cold-context, claims, CLI-demo, route, and history review.

