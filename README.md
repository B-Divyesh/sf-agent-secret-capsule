# Agent Secret Capsule

Agent Secret Capsule (`asc`) lets a coding or browser agent run one named
command with one selected credential without putting the raw value in a prompt,
shell history, captured stdout/stderr, or the audit receipt. Secrets stay in the
operating-system keychain. There is no telemetry and no hosted secret store.

This is for developers who need an agent to make an authorized API call but do
not want to hand the credential to the agent's conversational context. It is a
containment layer, not a sandbox or a secret-manager replacement.

## Install

Build the single binary with Rust 1.85 or newer:

```sh
cargo install --path crates/asc
asc doctor
```

Factory releases will provide checksummed binaries for macOS and Linux. The
repository is ready for `cargo package -p agent-secret-capsule`; publishing is
performed by the factory, not from a development checkout.

## Usage

Store a credential without placing it in shell history:

```sh
printf '%s' "$CLOUDFLARE_API_TOKEN" | asc put cloudflare --stdin
```

Run exactly one program with a 30-second lease. `asc` captures both output
streams, removes the raw secret and common exact encodings, then preserves the
program's exit status:

```sh
asc run cloudflare --env CLOUDFLARE_API_TOKEN --ttl 30s -- \
  curl --fail --silent https://api.cloudflare.com/client/v4/user/tokens/verify
```

Inspect no-value receipts or automate with JSON:

```sh
asc receipts
asc receipts --json
asc list --json
```

Remove a credential:

```sh
asc remove cloudflare
```

Run `asc --help` or `asc <command> --help` for flags, exit codes, and examples.
There are no interactive prompts when stdin is not a terminal; `put` requires
`--stdin` in CI.

## Security boundary

`asc` injects the secret only into the selected process environment and its
descendants, removes it when that process tree exits, enforces a lease timeout,
and redacts the exact raw, percent-encoded, base64, base64url, and hex values
from captured output. It never writes secret values to receipts.

An authorized process can still send the credential—or data derived from it—to
the network, write it to a file, transform it into an encoding this version does
not recognize, or pass it to a child. Review the command and its network scope.
For hostile code, use a separate sandbox as well.

## Develop and verify

```sh
npm install
npm test
npm run build
npm run build:site   # landing site only -> dist/site
cargo package -p agent-secret-capsule --allow-dirty
```

`npm test` runs Rust tests, site unit checks, and Playwright accessibility and
browser tests. The static site has no runtime CDN, tracking, or account system.
License tokens entered on the pricing panel stay in browser `localStorage` and
are sent only to the Sociobot license verification endpoint.

## Repository layout

- `crates/asc` — Rust CLI and library
- `site` — dependency-light Vite landing/docs site
- `.factory/design.md` — product-specific visual system and asset provenance
- `.factory/handoff.md` — verification and release handoff

## License

MIT. See [LICENSE](LICENSE).
