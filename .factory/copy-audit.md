# Copy audit — polish 4

Reviewed after the round-four rewrite. Landing and README visitor sentences
are at most 22 words. Commands, field labels, route paths, and stamped UI
fragments are excluded from sentence counts. No banned marketing term appears.

## Landing page

| Location | Sentence or fact | Words | Claim/evidence |
| --- | --- | ---: | --- |
| Description | Run one coding-agent command with a temporary credential, redact its output, and save a no-value receipt. | 16 | `process-tree`, `captured-output-receipt` |
| Hero | Give one agent command a temporary credential. | 7 | `process-tree` |
| Hero | For developers running coding agents: run one command, redact its output, and save a receipt without the credential. | 18 | `process-tree`, `captured-output-receipt` |
| Action note | See a fake credential redacted and a no-value receipt. | 9 | `cli-demo` |
| Fact | No analytics or third-party scripts. | 5 | `site-privacy` |
| Fact | Demo works offline after first visit. | 6 | `offline-reload` |
| Fact | Free and open source. | 4 | `license-package` |
| Figure | The selected process and its children receive one credential. | 9 | `process-tree` |
| External link | Read the CLI reference on GitHub (external). | 7 | Destination is explicit |
| Sample | See the command result first. | 5 | Clear heading |
| Sample | The CLI ships with `asc demo`. | 6 | `cli-demo` |
| Sample | It creates fake sample receipts in a new temporary directory. | 10 | `cli-demo` |
| How it works | Use a local alias in the agent tool input. | 9 | `credential-lifecycle` |
| Select | Store an alias locally. | 4 | `credential-lifecycle` |
| Select | Use the alias in the agent tool input. | 8 | `credential-lifecycle` |
| Run | The selected process and its children receive the credential until exit or the time limit. | 15 | `process-tree` |
| Redact | ASC captures both output streams and replaces matching credential forms before printing them. | 13 | `redaction-forms`, `captured-output-receipt` |
| Receipt | A no-value receipt omits the credential. | 6 | `captured-output-receipt` |
| Install | Build the CLI. | 3 | Instruction |
| Install | Run the sample. | 3 | `cli-demo` |
| Install | Use the bundled demo before storing a real credential. | 9 | `cli-demo` |
| Limits | Redaction limits output leaks. | 4 | Limitation, paired with `redaction-forms` |
| Limits | It is not a sandbox. | 5 | Explicit limitation |
| Limits | An authorized command can send the credential over the network or write it to a file. | 16 | Explicit limitation |
| Limits | It can also transform it or pass it to a child. | 11 | Explicit limitation |
| Limits | Review the command and endpoint. | 5 | Clear instruction |
| Limits | Use a separate network and process sandbox for hostile code. | 10 | Clear instruction |
| Footer | One command. | 2 | Descriptive fragment |
| Footer | One no-value receipt. | 3 | `captured-output-receipt` |
| External footer link | Source on GitHub (external). | 5 | Destination is explicit |

## Demo page

| Location | Sentence | Words | Claim/evidence |
| --- | --- | ---: | --- |
| Heading | Inspect a sample deployment check. | 5 | Clear heading |
| Intro | A fake read-only token is redacted before the receipt is written. | 11 | `demo-parity`, `cli-demo` |
| Note | Uses a fake credential, never a real one. | 8 | `demo-isolation` |
| Note | Keeps browser sample state separate from normal site data. | 9 | `demo-isolation` |
| Note | Shows the same fake deployment-status result as `asc demo`. | 8 | `demo-parity` |
| Note | For the command-line sample, run `asc demo`. | 7 | `cli-demo` |
| Note | It creates a new temporary directory and prints its path. | 10 | `cli-demo` |

## README

| Location | Sentence | Words | Claim/evidence |
| --- | --- | ---: | --- |
| Opening | Agent Secret Capsule (`asc`) gives one selected process and its children a temporary credential. | 14 | `process-tree` |
| Opening | It captures command output before printing it. | 7 | `captured-output-receipt` |
| Opening | It writes a receipt without the credential value. | 8 | `captured-output-receipt` |
| Audience | For developers whose coding agents need an authorized API call. | 10 | Audience statement |
| Audience | Use a local alias in the agent tool input. | 9 | `credential-lifecycle` |
| Sample | Run the bundled sample before storing a real credential. | 9 | `cli-demo` |
| Sample | The command checks a bundled fake deployment-status fixture. | 8 | `cli-demo` |
| Sample | It uses a fake credential. | 6 | `cli-demo` |
| Sample | It creates a new temporary directory with sample no-value receipts and prints its path. | 14 | `cli-demo` |
| Sample | It does not read your keychain or `ASC_HOME`. | 8 | `cli-demo` |
| Sample | Delete that directory to reset the command-line sample. | 8 | Clear instruction |
| Web sample | Try the web sample. | 4 | `demo-isolation` |
| Web sample | It uses browser storage keys with the `demo:asc` prefix. | 9 | `demo-isolation` |
| Web sample | Reset demo clears those sample keys. | 6 | `demo-isolation` |
| Install | Build from source. | 3 | Instruction |
| Usage | Store a credential from standard input. | 6 | `credential-lifecycle` |
| Usage | Run a selected process tree with a time limit. | 9 | `process-tree` |
| Usage | Inspect receipts or automate with JSON. | 6 | `receipt-commands` |
| Usage | Run `asc --help` for commands and exit codes. | 9 | `cli-interface` |
| Usage | Run `asc <command> --help` for flags and examples. | 8 | `cli-interface` |
| Usage | When standard input is not a terminal, `put` requires `--stdin`. | 10 | `cli-interface` |
| Limits | ASC gives the credential to the selected process and its children until exit or the time limit. | 17 | `process-tree` |
| Limits | It redacts raw, percent-encoded, Base64, Base64url, and hex matches from captured stdout and stderr. | 14 | `redaction-forms` |
| Limits | A no-value receipt omits the credential value. | 7 | `captured-output-receipt` |
| Limits | This is not a sandbox. | 5 | Explicit limitation |
| Limits | An authorized process can send the credential over the network or write it to a file. | 16 | Explicit limitation |
| Limits | It can also transform the credential or pass it to a child. | 12 | Explicit limitation |
| Limits | Review the exact command and endpoint. | 6 | Clear instruction |
| Limits | Use a separate sandbox for hostile code. | 7 | Clear instruction |
| Develop and verify | The command writes `dist/site` for deployment. | 7 | `build-output` |

## Designed 404 page

| Location | Sentence | Words | Claim/evidence |
| --- | --- | ---: | --- |
| Heading | This page does not exist. | 5 | Clear page state |

## Catalog description

| Location | Sentence | Characters | Claim/evidence |
| --- | --- | ---: | --- |
| Catalog | Run one coding-agent command with a temporary credential, redact output, and save a no-value receipt. | 101 | `process-tree`, `redaction-forms`, `captured-output-receipt` |

## Terminology

| Concept | One term |
| --- | --- |
| Protected item | credential |
| Local identifier | alias |
| Execution boundary | selected process and its children |
| Output operation | redact |
| Record | no-value receipt |
| Duration | time limit |
