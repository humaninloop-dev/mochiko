# Strip notes — `hooks/scripts/artifact-gate.sh`

Entry formats: `strips/README.md`.

## [v0.118.0] Wave 3 back-fill: the kernel-tooling-admission cite joined

- **Disposition:** superseded → the header comment (`artifact-gate.sh`:2) now cites
  "ledger GI-019-kernel-tooling-admission clause iv".
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/human-readable-ids/record.md` D11-cross-session-qualifier,
  D14-live-layer-backfill and D15-protected-line-rewrites — every live-layer mention joined to its
  definition's slug, cross-session mentions qualified, verbatim spans masked; wave plan
  `.mochiko/brainstorms/human-readable-ids/wave3-backfill.md` items 7–9)
- **Content (superseded):** every changed line at `0b8982a`, verbatim, as `<file>:<line>: <text>`
  (files under `plugins/mochiko/hooks/scripts/`):

  ```text
  artifact-gate.sh:2: # PreToolUse — the write-time artifact gate (record D1, D3, D7c; ledger GI-019 clause iv).
  ```

- **Kept deliberately:** the comments' "record D1, D3, D7c" (:2), "(record D9)" (:7, :44),
  "(record D7c)" (:36) and "D7c floor" (:47): each names its ruling only as "record", with no
  qualifier or link, so no owner resolves and the mention stays as written
  (`human-readable-ids` D11-cross-session-qualifier as narrowed at build;
  `human-readable-ids` D18-build-done-check as changed at build); "D7c" is also a clause pointer
  (wave plan S8).
