# Agent Secret Capsule

Agent Secret Capsule (`asc`) gives one selected process and its children a
temporary credential. It captures command output before printing it. It writes
a receipt without the credential value.

For developers whose coding agents need an authorized API call. Use a local
alias in the agent tool input.

## Try the sample

Run the bundled sample before storing a real credential:

```sh
cargo run -p agent-secret-capsule -- demo
```

The command checks a bundled fake deployment-status fixture. It uses a fake
credential. It creates a new temporary directory with sample no-value receipts
and prints its path. It does not read your keychain or `ASC_HOME`. Delete that
directory to reset the command-line sample.

The web sample is at `/demo/` or `/?demo=1`. It uses browser storage keys with
the `demo:asc` prefix. Reset demo clears those sample keys.

## Install

Build from source:

```sh
cargo install --path crates/asc
asc doctor
```

## Usage

Store a credential from standard input:

```sh
printf '%s' "$CLOUDFLARE_API_TOKEN" | asc put cloudflare --stdin
```

Run a selected process tree with a time limit:

```sh
asc run cloudflare --env CLOUDFLARE_API_TOKEN --ttl 30s -- \
  curl --fail --silent https://api.cloudflare.com/client/v4/user/tokens/verify
```

Inspect receipts or automate with JSON:

```sh
asc receipts
asc receipts --json
asc list --json
asc remove cloudflare
```

Run `asc --help` for commands and exit codes. Run `asc <command> --help` for
flags and examples. When standard input is not a terminal, `put` requires
`--stdin`.

## Security limits

ASC gives the credential to the selected process and its children until exit or
the time limit. It redacts raw, percent-encoded, Base64, Base64url, and hex
matches from captured stdout and stderr. A no-value receipt omits the
credential value.

This is not a sandbox. An authorized process can send the credential over the
network or write it to a file. It can also transform the credential or pass it
to a child. Review the exact command and endpoint. Use a separate sandbox for
hostile code.

## Develop and verify

```sh
npm ci
npm test
npm run build
cargo package -p agent-secret-capsule --allow-dirty
```

`npm test` runs Rust tests, site unit checks, and browser checks. Claim tests
are listed in `.factory/claims.json`. Build the static site with
`npm run build:site`; it writes `dist/site` for deployment.

## License

[MIT](LICENSE).
