# Demo sandbox

## Web demo

Open `https://agent-secret-capsule.sociobot.in/demo/` or
`https://agent-secret-capsule.sociobot.in/?demo=1`. The first screen action,
**Try it with sample data**, opens the same route.

The page immediately displays a fake read-only deployment-status check for
`api-gateway` in production. It uses the same fake scenario as `asc demo`. It
saves only a sample run count in `sessionStorage` with the `demo:asc:` prefix.
It never reads or writes normal site data. **Reset demo** clears that prefix.
**Start for real** clears the same prefix before it leaves the route and
returns home.

## CLI demo

Run `asc demo`. It uses a fixed fake credential with the production lease,
redaction, and receipt code. It checks the bundled read-only
`examples/deployment-status.json` fixture. It creates a newly named directory
under the system temporary directory, writes two no-value receipts, prints the
directory path, and does not read the OS keychain or `ASC_HOME`. Delete the
printed directory to reset it. The shipped fixture explanation is
`examples/agent-output.txt`.
