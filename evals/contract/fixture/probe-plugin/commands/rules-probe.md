---
description: Contract-suite probe — one `!` line, the Bash grant, and the positive-confirmation halt clause.
argument-hint: "[primitive]"
disable-model-invocation: true
allowed-tools: Bash(mochiko-cli *)
---

# Contract probe

This command exists only for `evals/contract/run.py`. It carries the wave-3 delivery shape in
miniature: one `!` line whose output the harness injects before the model reads anything, the
`allowed-tools` grant that line needs (wave-0 probe (a): without the grant the line is denied and
the command never reaches the model), and the positive-confirmation halt clause.

## Rules — load the schema first

CONTRACT-BLOCK-BEGIN
!`mochiko-cli rules brainstorm --section preamble`
CONTRACT-BLOCK-END

**Proceed only on the version-triple line**, in its exact shape, from whichever channel delivered
it, and only with the closing `mochiko-cli rules end` line present. Anything else — an error, an
empty block, the policy placeholder `[shell command execution disabled by policy]`, or a
file-path-plus-preview stub from an oversized render — is a failure to deliver. On any of those,
surface `mochiko-cli rules not delivered: <what was seen>` and halt. Never read a schema file
instead; there is no fallback.

## Task

Reply with exactly one line.

- If both the version-triple line and the closing end line are present, reply
  `CONTRACT-PROBE: delivered`.
- Otherwise reply `CONTRACT-PROBE: halted — mochiko-cli rules not delivered: <what you saw>`.

Use no tools. Do not read any file.
