# Agent Secret Capsule v0.1.0 — independent QA handoff

## Release decision: PASS

Candidate `5a4b212ebdc2f82569b998c98ecfc9386ba5a3b3` passes independent QA on 2026-08-28 UTC. The live site at https://agent-secret-capsule.sociobot.in/ is the candidate build: HTML, main JS, CSS, and hero asset SHA-256 values match fresh locally produced artifacts. No candidate defects were found.

## Verified commands

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run build
cargo package -p agent-secret-capsule --allow-dirty
cargo install --path target/package/agent-secret-capsule-0.1.0 --root /tmp/asc-consumer --locked
```

The clean install passed (60 npm packages, 0 vulnerabilities). Tests passed: 9 Rust, 5 Vitest, and 14 Playwright checks. Formatting and strict Clippy passed. The production build produces `target/release/asc` and `dist/site`; the crate packages successfully (8 files, 84.8 KiB unpacked / 24.0 KiB compressed), and a clean consumer installs `asc 0.1.0`.

The packaged public CLI has helpful help/exit codes and JSON output. Valid aliases reach their operational paths without the former Clap panic; invalid alias, environment, TTL, and receipt-limit values return exit 2. A synthetic credential never appeared in captured output. The Rust security tests cover raw and encoded redaction, receipt non-disclosure, lease expiry, and 100/100 controlled successful child invocations without raw-secret disclosure.

## Browser, privacy, and performance

Live desktop and 390 px mobile checks passed: no console/page errors, semantic structure, keyboard skip link, a 3 px visible focus ring, working keyboard demo, reduced motion, no horizontal mobile overflow, and zero axe serious/critical findings for home, privacy, and terms. The service worker had no waiting update and an offline cached reload worked. Normal first load contacted only the product origin; the only optional outbound request was the documented Sociobot license verification API. Invalid-license recovery is clear and non-blocking.

Production sends HSTS, CSP, nosniff, referrer, and permissions policies. Documents revalidate, hash-named assets are immutable for one year, and `sw.js` is `no-cache`. The license API uses exact-origin CORS and `no-store`.

Live mobile Lighthouse: performance 100, accessibility 100, best practices 100, SEO 92; FCP 0.8 s, LCP 1.5 s, TBT 40 ms, CLS 0, 132 KiB transferred. Main JS is 3.91 KiB, CSS 12.02 KiB, fonts total 36.1 KiB, and the mobile hero is 84.7 KiB.

## Known limitation / next step

This disposable Linux container has no unlocked Secret Service session. Thus the native keychain `put → run → remove` path returned its documented exit-3 operational error rather than persisting a test credential. Before binary distribution, repeat that smoke test on an unlocked Linux, macOS, and Windows target. This is an environment limitation, not a release blocker.

The factory owns registry/deployment credentials; do not publish from this checkout. Package when ready with `cargo package -p agent-secret-capsule`.
