# Copy audit — polish 2

The landing page, home metadata, and README were checked after the round-two
rewrite. No sentence exceeds 22 words, and no banned marketing word appears.
Interface labels, commands, paths, and receipt fields are excluded from sentence
counts.

## Landing page sentences

| Location | Sentence | Words | Result |
| --- | --- | ---: | --- |
| Home description | Run one coding-agent command with a temporary credential, redact its output, and save a no-value receipt. | 16 | Pass |
| Hero | Give one agent command a temporary credential. | 7 | Pass |
| Hero | For developers running coding agents: run one command, redact its output, and save a receipt without the credential. | 18 | Pass |
| Hero action note | See a fake credential redacted and a no-value receipt. | 9 | Pass |
| Hero figure | One chosen path for one credential. | 6 | Pass |
| Sample | See the command result first. | 5 | Pass |
| Sample | The CLI ships with `asc demo`. | 6 | Pass |
| Sample | It creates fake sample receipts in a new temporary directory. | 10 | Pass |
| How it works | Use a local alias in the agent tool input. | 9 | Pass |
| Select | Store an alias locally. | 4 | Pass |
| Select | Use the alias in the agent tool input. | 9 | Pass |
| Run | The selected process and its children receive the credential until exit or the time limit. | 14 | Pass |
| Redact | ASC captures both output streams and replaces matching credential forms before printing them. | 13 | Pass |
| Receipt | A no-value receipt omits the credential. | 6 | Pass |
| Install | Build the CLI. | 3 | Pass |
| Install | Run the sample. | 3 | Pass |
| Install | Use the bundled demo before storing a real credential. | 9 | Pass |
| Limits | Redaction limits output leaks. | 4 | Pass |
| Limits | It is not a sandbox. | 5 | Pass |
| Limits | An authorized command can send the credential over the network or write it to a file. | 16 | Pass |
| Limits | It can also transform it or pass it to a child. | 11 | Pass |
| Limits | Review the command and endpoint. | 5 | Pass |
| Limits | Use a separate network and process sandbox for hostile code. | 10 | Pass |
| Footer | One command. | 2 | Pass |
| Footer | One no-value receipt. | 3 | Pass |

## README check

The prior 23-word security warning is now two sentences: “An authorized process
can send the credential over the network or write it to a file.” (14 words) and
“It can also transform the credential or pass it to a child.” (11 words).
“Build with Rust 1.85 or newer” was replaced with the non-claim instruction
“Build from source.”

## Terminology

| Concept | Single term |
| --- | --- |
| Protected item | credential |
| Identifier stored locally | alias |
| Execution boundary | selected process and its children |
| Output operation | redact |
| Record | no-value receipt |
| Duration | time limit |

The former landing label “secret leasing” is now “credential time limit.”
