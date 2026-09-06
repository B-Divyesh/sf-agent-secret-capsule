# Give one agent command a temporary credential — verification 4

**Verdict: PASS.** This independent verification found zero findings of every
severity and zero untested public claims.

- Implementation reviewed: `6ce85df2f4a76d674fa95a08bda005918257cfcb`
- Documentation baseline: `12aba05065845d074c5a4ba92bd2744f01292a4a`
- Live URL: <https://agent-secret-capsule.sociobot.in/>
- Date: 2026-09-06 UTC
- Finding count: **0**
- Untested claim count: **0**

Only `.factory/handoff.md` changed between the implementation and documentation
SHAs. The live Home, Demo, Privacy, Terms, 404, service worker, emitted scripts,
stylesheet, and phone hero matched the implementation build byte for byte.

## Job, audience, and first action

Fresh Chromium contexts at 1440×900 and 390×844 showed the required information
before scrolling:

| Check | Live result |
| --- | --- |
| Job | “Give one agent command a temporary credential.” |
| Audience | “For developers running coding agents…” |
| First action | **Try it with sample data** |
| Next result | “See a fake credential redacted and a no-value receipt.” |
| Desktop bounds | Audience y=657; action y=736; all three facts end by y=803. |
| Phone bounds | Audience y=471; action y=543; all three facts end by y=673. |

The wording is literal and names the job. It contains no metaphor or mood
heading. The title is “Agent Secret Capsule — give one command a credential.”

## Demo and real-data isolation

The Home action opened `/demo/` in one click. `/?demo=1` also opened that route.
The initial view already showed a realistic `api-gateway` production status,
redacted stdout and stderr, a 30 ms time-limit result, and a no-value receipt
with alias, outcome, redaction count, and omitted credential value.

The persistent label read **“Demo — sample data, nothing is saved.”** Running
the sample created only `sessionStorage["demo:asc:run-count"]`. Reset returned
the state to `READY` and removed that key. **Start for real** also removed the
demo key. Separate localStorage and sessionStorage `real:sentinel` values
survived both actions unchanged. The sample output contained redaction markers
and no configured fake credential.

The CLI demo created a new temporary directory, printed its path, returned
redacted stdout and stderr, and supplied two no-value receipts. A sentinel
`ASC_HOME` path was not created.

## Live routes, accessibility, privacy, and offline use

- Home, Demo, Privacy, Terms, and the designed 404 passed `verify-url.sh`.
  Each has its route title, `lang=en`, one `h1`, a `main`, image alternatives,
  labeled buttons, and zero unexpected console errors.
- Fresh desktop and phone Axe scans found zero serious or critical violations
  on all five documents. The deliberate unknown route returned HTTP 404 with
  “This page does not exist”; its expected browser 404 log is not a defect.
- The skip link was first in keyboard order and moved focus to `main`. Space
  activated the focused sample button. Route loads, Demo → Home, and browser
  Back focused and announced the new page heading.
- All visible phone header, footer, and button targets were at least 44×44 CSS
  pixels. No checked route overflowed at 390 px. A 200% text-size smoke test
  kept the heading and primary action visible.
- Reduced motion shortened the redaction transition to `0.00001s`. Nothing
  flashes or loops.
- A fresh service-worker context was controlled, had no waiting update, and
  reloaded the populated Demo while offline. Connectivity was restored before
  that isolated context closed.
- All observed browser requests were same-origin. Fresh contexts had no
  cookies or localStorage entries before the explicit isolation sentinel.
  There were no analytics, advertising, CDN scripts, runtime AI calls, or
  payment calls.
- The Privacy page explains local receipt data, browser demo storage, network
  limits, deletion, and a privacy-request route through the linked public
  repository. Privacy and Terms both loaded with their own titles.
- Every internal link, both GitHub destinations, `robots.txt`, `sitemap.xml`,
  favicon, touch icon, and social image returned HTTP 200.
- Response headers included CSP, HSTS, `nosniff`, strict-origin referrer policy,
  and camera, microphone, and geolocation denial. Hashed assets use one-year
  immutable caching; the service worker uses `no-cache`.

## Declared claims from a clean checkout

A fresh remote clone was pinned to the implementation SHA. `npm ci` installed
60 packages with zero reported vulnerabilities. Every command in
`.factory/claims.json` was run independently and passed. Each claim tag occurs
exactly once in the test sources.

| Claim | Declared command | Result |
| --- | --- | --- |
| `demo-isolation` | `npm run test:claim -- --grep @claim:demo-isolation` | PASS, 2 projects |
| `offline-reload` | `npm run test:claim -- --grep @claim:offline-reload` | PASS, 2 projects |
| `cli-demo` | `npm run test:claim -- --grep @claim:cli-demo` | PASS, 1 pass / 1 intentional project skip |
| `redaction-forms` | `cargo test --locked --features test-keyring --test cli_claims claim_redaction_forms_removes_every_named_form_from_compiled_cli_output` | PASS |
| `process-tree` | `cargo test --locked --features test-keyring --test cli_claims claim_process_tree_uses_the_documented_cli_and_stops_at_its_time_limit` | PASS |
| `captured-output-receipt` | `cargo test --locked --features test-keyring --test cli_claims claim_captured_output_and_receipt_omit_the_credential` | PASS |
| `credential-lifecycle` | `cargo test --locked --features test-keyring --test cli_claims claim_credential_lifecycle_stores_lists_runs_and_removes_an_alias` | PASS |
| `receipt-commands` | `cargo test --locked --features test-keyring --test cli_claims claim_receipt_commands_return_newest_first_human_and_json_no_value_results` | PASS |
| `receipt-storage-schema` | `cargo test --locked --features test-keyring --test cli_claims claim_receipt_storage_schema_is_private_and_contains_only_declared_metadata` | PASS |
| `cli-interface` | `cargo test --locked --features test-keyring --test cli_claims claim_cli_interface_help_and_non_tty_input_behave_as_documented` | PASS |
| `demo-parity` | `npm run test:claim -- --grep @claim:demo-parity` | PASS, 1 pass / 1 intentional project skip |
| `license-package` | `npm run test:claim -- --grep @claim:license-package` | PASS, 1 pass / 1 intentional project skip |
| `site-privacy` | `npm run test:claim -- --grep @claim:site-privacy` | PASS, 2 projects |
| `build-output` | `npm run test:claim -- --grep @claim:build-output` | PASS, 2 projects |

The landing pages, route metadata, legal pages, README, changelog, installed
help, and successful human output were cross-checked against the manifest. The
former native-keychain success wording is absent. Every retained observable
promise maps to a passing claim; no public claim is unlisted or untested.

## CLI, package, and recovery checks

| Check | Result |
| --- | --- |
| `npm test` | PASS: 11 Rust tests, 2 Vitest tests, 40 Playwright passes, 6 intentional duplicate-project skips |
| `npm run build` | PASS; produced `target/release/asc` and `dist/site` |
| `cargo fmt --check` | PASS |
| Strict Clippy | PASS with workspace, all targets, all features, locked dependencies, and warnings denied |
| `cargo package -p agent-secret-capsule --allow-dirty` | PASS; verified 11 packaged files |
| Clean package consumer | PASS; installed `asc 0.1.0` from the packaged crate into a new prefix |

The installed artifact's root help and `put` help are direct and contain no
native-store success promise. `asc --json demo` returned the expected redacted
fixture and two receipts. Empty `list --json` and `receipts --json` returned
valid empty states. Invalid aliases and environment names return usage errors;
an over-limit 61-minute duration returns exit 2. The unavailable native-store
path returns exit 3 without echoing input.

The release-configured regression forces the Linux store unavailable while
setting the test-store variable. It proves the default binary does not activate
the test-only file store, does not write alias metadata, does not reveal the
input, and exits safely. Successful lifecycle behavior remains covered only by
the explicitly feature-gated isolated test store, which matches the narrower
public claim. No native-keychain success is advertised.

## Performance

Fresh mobile Lighthouse scored **100 performance, 100 accessibility, 100 best
practices, and 100 SEO**. FCP was 0.85 seconds, LCP 1.52 seconds, CLS 0, total
blocking time 0 ms, and transfer size 129,903 bytes.

The build emits 3.3 KiB of JavaScript across all chunks, 13.55 KiB CSS, 36.06
KiB self-hosted fonts, and an 84.74 KiB phone hero. These remain inside the
declared static budgets.

## Earlier finding disposition

All prior review, polish, verification, and handoff files were read. Findings
were checked against current live output, the clean build, or the installed
artifact rather than accepted from earlier closure notes.

| Earlier finding | Current disposition and proof |
| --- | --- |
| Verification 1 parser panic | Fixed. Compiled parser tests and clean-consumer alias commands reach usage or operational results without panic. |
| Verification 1 cache gap | Fixed. Live hashed assets are immutable for one year; `sw.js` is `no-cache`. |
| Review 1 B1 / Review 2 F-2-1 | Fixed. Job, audience, action, outcome, and three facts are before scrolling at both required sizes. |
| Review 1 B2 / Review 3 F-3-1 | Fixed. Browser and CLI demos open directly with populated results; banner, Reset, Start for real, and storage isolation pass. |
| Review 1 B3 and U01–U57 claim issues / Reviews 2–4 F-2-2, F-3-2, F-4-2 | Fixed or removed. Fourteen retained claims each have one tag and every declared command passes. Removed prompt, compiler, billing, lifetime-update, telemetry, broad keychain, and other unsupported promises remain absent. |
| Review 1 B4 | Fixed. Copy and tests consistently state the selected process and its children. |
| Review 1 B5 | Fixed by removal. No paid tier, checkout, restore, license, price, or billing claim remains. |
| Review 1 B6 / M1 / Review 2 F-2-4 | Fixed. Route metadata and discovery assets load; an unknown URL returns the designed HTTP 404. |
| Review 1 M2 / Review 4 F-4-1 | Fixed. Shared shell, section order, skip link, route focus, announcements, and Back behavior pass live. |
| Review 1 M3 and CW01–CW26 / Reviews 2–4 F-2-3, F-4-3, F-4-4, F-4-5 | Fixed or removed. Current copy is literal, within the word limit, consistent, and names GitHub links and the absolute README demo URL. |
| Review 1 M4 / Review 3 F-3-3 | Fixed. Phone targets meet 44 px, facts fit before scrolling, and routes do not overflow. |
| Review 6 F-6-1 | Fixed. The false paid-license restore changelog line is absent. |
| Review 6 F-6-2 | Fixed. Terms warranty copy is split into sentences of at most 22 words. |
| Review 6 F-6-3 | Fixed. Shared Demo and Terms links measure at least 44×44. |
| Review 7 F-7-1 | Fixed. Changelog, root help, `put` help, and success output no longer promise native-keychain success; release-configured failure is tested fail-closed. |
| Verification 2 environment note | Correctly bounded. This worker has no unlocked Secret Service session, and the product makes no native-success claim. |
| Reviews 5 and verification 3 | Those rounds reported no additional findings. Their covered behavior remains green here. |

No backend is part of this CLI/static-site product, so tenant isolation, server
restart persistence, health, and HTTP 429 behavior do not apply. The brief does
not benefit from runtime AI; adding it would expand the credential boundary, so
there is no missed-leverage finding.

## Evidence

Screenshots, route reports, live browser JSON, hash comparisons, and Lighthouse
output are under `/work/.evidence/agent-secret-capsule-verify-4/`.

**Final verdict: PASS — 0 findings, 0 untested claims.**
