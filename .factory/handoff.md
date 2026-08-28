# Agent Secret Capsule — polish 2 handoff

## Delivered

Every finding in `review-2.md` and the complete earlier `review-1.md` ledger is
closed. The per-finding change and evidence map is `.factory/polish-2.md`.

- The full developer audience line and **Try it with sample data** action fit
  inside both 1440×900 and 390×844 first screens.
- `/demo/` and `?demo=1` open the isolated sample. Reset and Start for real
  both discard `demo:asc:` session state. The CLI demo uses a fresh temporary
  directory and succeeds when `ASC_HOME` and the keychain session are unusable.
- The claims manifest now covers eight public claims. The prompt and compiler-
  version promises were removed; MIT/package and site-privacy claims gained
  observable tests.
- The published crate now includes the exact MIT `LICENSE` as well as declaring
  `MIT` in Cargo metadata.
- The 404 keeps its real HTTP 404 response and now has complete route-specific
  Open Graph and Twitter metadata.
- The long README warning was split, “secret leasing” was removed from visitor
  copy, and `.factory/copy-audit.md` has no sentence over 22 words.
- The existing brutalist concrete-and-moss identity, CLI artifact class, and
  static Vite deployment class remain unchanged.

Implementation commit: `3a36e49eded9f501730f90f2aa1c38a02883cd54`.

## Clean-clone verification

A fresh `git clone --no-local /work/repo` was created at
`/tmp/tmp.crLY5pPcnD/repo`, followed by `npm ci`. Every command in
`.factory/claims.json` was then executed independently:

| Claim | Result |
| --- | --- |
| `@claim:demo-isolation` | PASS, desktop and mobile |
| `@claim:offline-reload` | PASS, desktop and mobile |
| `@claim:cli-demo` | PASS, one execution plus intentional mobile duplicate skip |
| `redacts_raw_and_encoded_forms` | PASS |
| `@claim:process-tree` | PASS, one execution plus intentional mobile duplicate skip |
| `@claim:captured-output-receipt` | PASS, one execution plus intentional mobile duplicate skip |
| `@claim:license-package` | PASS, one execution plus intentional mobile duplicate skip |
| `@claim:site-privacy` | PASS, desktop and mobile |

The complete clean-clone `npm test` also passed: 10 Rust tests, 2 Vitest tests,
and 29 Playwright tests; five redundant mobile executions of CLI-only claims
were intentionally skipped.

Additional local gates:

```text
npm ci                                                        PASS (60 packages, 0 vulnerabilities)
npm test                                                      PASS
npm run build                                                 PASS (target/release/asc and dist/site)
cargo fmt --check                                             PASS
cargo clippy --workspace --all-targets --locked -- -D warnings PASS
cargo package -p agent-secret-capsule --allow-dirty            PASS (9 files, 88.9 KiB / 25.1 KiB compressed)
```

The browser suite covers the five pages at desktop and 390 px, serious/critical
Axe checks, exact 1440×900 and 390×844 hero bounds, keyboard skip navigation,
Back-button h1 focus, route metadata, a real 404, 44 px targets, no horizontal
overflow, reduced motion, demo isolation/exit cleanup, same-origin privacy, and
offline reload.

Local verifier evidence is in `.factory/evidence/polish-2-local/` and
`.factory/evidence/polish-2-local-demo/`. Both have correct titles, `lang=en`,
one h1, a main landmark, complete alt/button labels, and zero browser errors.

## Deployment and cold production verification

- Work-order build: `npm ci && npm run build:site`
- Deployment command: `/opt/fleet/lib/deploy-static.sh agent-secret-capsule dist/site`
- Deployment id: `13603aa0-fc26-4a4b-9f13-23e6cecd15b8`
- Live URL: <https://agent-secret-capsule.sociobot.in/>

Cold `verify-url.sh` checks passed on `/`, `/demo/`, `/privacy/`, and `/terms/`
with no console errors. A separate cold Chromium pass confirmed:

- desktop audience bottom 671 px and CTA bottom 750 px in a 900 px viewport;
- `?demo=1` enters the populated demo with the persistent banner;
- Reset and Start for real both clear demo state;
- the 390 px demo has no horizontal overflow and its two banner controls are
  169×44 px;
- all demo requests are same-origin and no cookies are created;
- Privacy → Terms → Back restores focus to the Privacy h1;
- `/not-a-real-route` returns HTTP 404 with every required Open Graph/Twitter
  field;
- all eight crawled internal/external links return 200;
- live Axe has zero serious/critical violations on all five pages at desktop
  and mobile.

Production artifacts exactly match the local build:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `790da589de22f46966e9f1e76ebcdbc86c08d30daf376e03e9d3d619f96199bf` |
| `assets/main-CO5TDLdx.js` | `c5f8a11c725252e47c1a84235497e9932d937a1eb8d10f396e46a170684b006c` |
| `assets/style-Dbw9iziQ.css` | `a2ad37de5da808b748050c180016980a987dbd61502e05f53e88559995d03d77` |

Live mobile Lighthouse scored **100 performance, 100 accessibility, 100 best
practices, and 100 SEO**. FCP was 0.9 s, LCP 1.5 s, speed index 1.0 s, TBT
10 ms, and CLS 0. The built site ships 3.1 KiB total JavaScript, 12.8 KiB CSS,
36.1 KiB fonts, and an 84.7 KiB mobile hero image.

Production evidence is under `.factory/evidence/polish-2-live*/`, including the
exact first-screen screenshot, direct-query mobile demo, designed 404, route
verifier reports, live Axe/link report, cold browser report, and Lighthouse
JSON.

## Run and package

```sh
npm ci
npm test
npm run build
cargo package -p agent-secret-capsule --allow-dirty
```

The factory owns registry publishing; do not publish from this checkout.

## Known gaps

None.
