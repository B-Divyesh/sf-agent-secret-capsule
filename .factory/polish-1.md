# Polish 1 — review finding closure

Candidate repaired from `c9ee1997b8343876ccb2ba86d109e87a275b2008`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| B1 | Rewrote the first screen with the plain job headline, named developers running coding agents, a visible sample action, outcome note, and three short facts. | `tests/e2e/site.spec.ts` landing metadata/a11y test; `.factory/evidence/polish-1-local/screenshot-desktop.png` |
| B2 | Added `/demo/`, `?demo=1` redirect, banner, reset/start controls, `demo:asc:` storage, `asc demo`, `examples/agent-output.txt`, and `.factory/demo.md`. | `@claim:demo-isolation`; `@claim:cli-demo`; `.factory/evidence/polish-1-demo/screenshot-mobile.png` |
| B3 | Added `.factory/claims.json` with one executable test command per public behavioral claim; deleted unsupported commercial and broad marketing promises. | All six manifest commands pass; commands recorded in handoff. |
| B4 | Replaced false named-process wording with the selected process-and-children boundary. | `@claim:process-tree` runs child inheritance and expiry fixtures. |
| B5 | Removed the unavailable paid checkout, license storage, and purchase claims. | No checkout or billing URL remains in the built landing pages; route/link browser test passes. |
| B6 | Added built `/demo/` and `404.html`, disabled SPA fallback for this multi-page site, added SWA 404 override, robots, sitemap, and favicon. | `real routes load, unknown routes return 404, and discovery assets exist` |
| M1 | Added per-route titles, descriptions, canonical, OG/Twitter metadata, SVG favicon, 180px touch icon, and 1200×630 social image. | Route metadata/a11y browser tests; `capsule-social.webp` is 1200×630. |
| M2 | Made header/footer consistent, added Demo/Privacy/legal links, factory build attribution, h1 focus, and polite route announcements. | Route/focus browser test and local verify-url reports. |
| M3 | Replaced metaphor, jargon, ambiguous labels, unsupported guarantees, and overlong copy; recorded the audit and terminology. | `.factory/copy-audit.md` |
| M4 | Enlarged footer and demo/legal controls to 44px minimum and tested at 390px. | `mobile controls meet the touch target and never overflow`; mobile screenshot. |
| CW01–CW26 | Replaced or removed each listed copy unit with the audited plain-language text; unsupported scripted-demo and license copy were removed. | `.factory/copy-audit.md`; landing screenshot. |

## Earlier unlisted-claim ledger

Each earlier `U` finding is either covered by a current claim test or removed
with the related unsupported wording. No paid, checkout, merchant, release,
telemetry, keychain-backend, shell-history, or future-entitlement promise
remains in the landing copy or README.

| Finding | Resolution | Evidence |
| --- | --- | --- |
| U01 | Removed prompt/traces promise. | Current copy audit |
| U02 | Removed packed platform/telemetry assertion. | Current landing |
| U03 | Removed keychain-at-rest assertion. | Current landing |
| U04 | Corrected to process-tree boundary. | `@claim:process-tree` |
| U05 | Replaced with named supported forms. | `@claim:redaction-forms` |
| U06 | Replaced with no-value receipt statement. | `@claim:captured-output-receipt` |
| U07 | Removed capability/prompt promise. | Current copy audit |
| U08 | Removed alias/keychain resolution claim. | Current landing |
| U09 | Removed prompt-content claim. | Current landing |
| U10 | Corrected process-tree/time-limit wording. | `@claim:process-tree` |
| U11 | Kept as precise captured-stream claim. | `@claim:captured-output-receipt` |
| U12 | Kept named raw/percent/Base64/Base64url/hex scope. | `@claim:redaction-forms` |
| U13 | Removed receipt-field inventory. | Current landing |
| U14 | Kept precise omission claim. | `@claim:captured-output-receipt` |
| U15 | Replaced scripted panel with isolated fake-data demo. | `@claim:demo-isolation` |
| U16 | Removed CLI-parity wording from browser sample. | Current demo copy |
| U17 | Removed daemon/account/vault/telemetry claim. | Current landing |
| U18 | Removed Rust-version/binary claim from landing. | Current landing |
| U19 | Kept the explicit non-sandbox warning. | Boundary section/browser screenshot |
| U20 | Kept shorter boundary warning. | Boundary section/browser screenshot |
| U21 | Removed vague free-tier assertion. | Current landing |
| U22 | Removed ungated-feature assertion. | Current landing |
| U23 | Removed $19 purchase/release promise. | Current landing |
| U24 | Removed merchant claim. | Current landing |
| U25 | Removed refund/revocation claim. | Current landing |
| U26 | Removed free-core status. | Current landing |
| U27 | Removed license storage status. | Current landing |
| U28 | Kept no-value receipt wording. | `@claim:captured-output-receipt` |
| U29 | Split to current process/output/receipt claims. | `@claim:process-tree`, `@claim:captured-output-receipt` |
| U30 | Removed OS-keychain-at-rest statement. | README |
| U31 | Removed telemetry/hosted-store statement. | README |
| U32 | Kept the non-sandbox limitation in plain words. | README boundary |
| U33 | Kept build instruction only; removed release claim. | README |
| U34 | Removed future binary promise. | README |
| U35 | Kept package command as a verification instruction. | `cargo package -p agent-secret-capsule --allow-dirty` |
| U36 | Reworded to stdin instruction without history promise. | README |
| U37 | Corrected to selected process tree. | `@claim:process-tree` |
| U38 | Split streams/forms/receipt behavior. | `@claim:redaction-forms`, `@claim:captured-output-receipt` |
| U39 | Kept documented CLI commands as interface instructions. | `cargo test --workspace --locked` |
| U40 | Kept documented remove command as interface instruction. | CLI parser test |
| U41 | Kept help instruction. | CLI parser tests |
| U42 | Kept CI stdin instruction. | CLI parser test |
| U43 | Split into current process-tree, forms, and receipt claims. | Three tagged claim tests |
| U44 | Kept no-value receipt statement. | `@claim:captured-output-receipt` |
| U45 | Kept plain non-sandbox warning. | README boundary |
| U46 | Kept test-script description synchronized with `package.json`. | `npm test` |
| U47 | Removed broad site privacy promise from README. | README |
| U48 | Removed license-token storage claim. | Removed license code |
| U49 | Retained MIT link. | `LICENSE`; `cargo package` |
| U50 | Removed lifetime-update label. | Current landing |
| U51 | Replaced “no history” label with stdin input. | Landing install section |
| U52 | Replaced audit label with receipts/JSON instruction. | Landing install section |
| U53 | Removed team-kit entitlement. | Current landing |
| U54 | Removed retention-policy entitlement. | Current landing |
| U55 | Reworded title/description without lease jargon. | Route metadata test |
| U56 | Uses “redact,” not “scrub.” | Copy audit |
| U57 | Kept the no-value receipt claim. | `@claim:captured-output-receipt` |

## Local evidence

- Landing: `.factory/evidence/polish-1-local/screenshot-desktop.png`
- Demo/mobile: `.factory/evidence/polish-1-demo/screenshot-mobile.png`
- Semantic/console reports: `.factory/evidence/polish-1-local/verify.json` and
  `.factory/evidence/polish-1-demo/verify.json`
- Public checks are recorded in `.factory/handoff.md` after deployment.
