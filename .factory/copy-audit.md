# Copy audit — polish 1

All landing sentences were counted after the rewrite. No sentence exceeds 22
words and no banned marketing word appears.

| Location | Sentence | Words | Result |
| --- | --- | ---: | --- |
| Hero | Give one agent command a temporary credential. | 7 | Pass |
| Hero | For developers running coding agents: run one command, redact its output, and save a receipt without the credential. | 17 | Pass |
| Hero action note | See a fake credential redacted and a no-value receipt. | 10 | Pass |
| Sample | The CLI ships with `asc demo`. | 6 | Pass |
| Sample | It creates fake sample receipts in a new temporary directory. | 10 | Pass |
| How it works | Use a local alias in the agent tool input. | 9 | Pass |
| Run | The selected process and its children receive the credential until exit or the time limit. | 14 | Pass |
| Redact | ASC captures both output streams and replaces matching credential forms before printing them. | 13 | Pass |
| Receipt | The receipt records time, alias, executable, outcome, and redaction count. | 10 | Pass |
| Receipt | It omits the credential. | 4 | Pass |
| Install | Use the bundled demo before storing a real credential. | 10 | Pass |
| Limits | An authorized command can send the credential over the network or write it to a file. | 16 | Pass |
| Limits | It can also transform it or pass it to a child. | 11 | Pass |
| Limits | Review the command and endpoint. | 5 | Pass |
| Limits | Use a separate network and process sandbox for hostile code. | 10 | Pass |

## Terminology

| Concept | Single term |
| --- | --- |
| Protected item | credential |
| Identifier stored locally | alias |
| Execution boundary | selected process and its children |
| Output operation | redact |
| Record | no-value receipt |
| Duration | time limit |
