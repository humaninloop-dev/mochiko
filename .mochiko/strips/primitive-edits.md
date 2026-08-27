# Strip notes — `.claude/rules/mochiko/primitive-edits.md`

Entry formats: `strips/README.md`. First entry at v0.97.0 — no prior strip file existed for
this `.claude/rules/` surface; earlier edits rode their decision rows.

<!-- Wave context: the command-schema ontology wave (v0.98.0). Ruling:
`.mochiko/brainstorms/command-schema-ontology/record.md` D1–D11 as amended →
`DECISIONS.md` 2026-08-27. Clause inventory:
`.mochiko/brainstorms/command-schema-ontology/conversion-inventory.md` (section H carries
the re-keyed form and names this file as a downstream consumer). -->

## [v0.98.0] Criterion 3's FAIL survival re-keyed from the `fail-condition` label set to `kind: fail`

- **Disposition:** superseded → criterion 3 of the same canonical-scaffold criteria block in
  `.claude/rules/mochiko/primitive-edits.md`, re-keyed to `kind: fail` and carrying the
  bidirectional `<cmd>.fail.*`-segment cross-check.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/command-schema-ontology/record.md` D1 and build-surface item 9,
  "criterion 3 re-keys from the label set to `kind: fail`"; `DECISIONS.md` 2026-08-27.
  Inventory: that session's `conversion-inventory.md` section H.)
- **Content:** criterion 3 as shipped at v0.97.0, verbatim —

  > 3. **FAIL survival** keys to the **`fail-condition` label set**: every so-labeled rule
  >    survives (a reword keeps its ID), and the `.md` Not-done line's hard-coded count
  >    matches the schema's.

- **Kept deliberately:** both guards the label-keyed clause carried survive on the new key —
  every fail rule must survive, a reword keeping its ID, and the `.md` Not-done line's
  hard-coded count must match the schema's. Nothing else in the criteria block was re-keyed:
  criterion 2's `<cmd>.sec.fail-conditions` section ID and the `<cmd>.fail.*` ID segment are
  ID grammar, not the retired label, and stand untouched. Added, not removed — the `.fail.*`
  segment ↔ `kind: fail` bidirectional correspondence, with `kind:` never defaulted on a
  `.fail.*` ID (I4): the checker already asserted it, the shipped criterion never stated it.
- **Consumers assessed:** `plugins/mochiko/schemas/command-labels.yaml` — the registry line
  retiring the label, removed the same wave (entry in `.mochiko/strips/command-labels.md`) ·
  `.claude/skills/converting-command-to-schema/SKILL.md` step 14's criteria summary, which
  cites this criterion by name and was re-keyed the same wave (entry in
  `.mochiko/strips/converting-command-to-schema.md`) · the six command `.md` Not-done lines
  and their schemas' fail nodes, re-keyed the same wave · `scripts/check-command-schema.py`,
  whose Not-done count check and segment cross-check move to the new key in the same wave ·
  `CLAUDE.md`'s two audit-ceremony sites, which name "FAIL survival" without keying it to a
  label — unchanged by this edit, and verified so.

<!-- Pure additions in the same edit, riding the decision row (no strip owed, per this file's
governing rule that additions carry no strip note): criterion 11, "Ontology-grammar
conformance (D1–D8)", and the `command-schema-ontology` D1–D11 entry appended to the block's
closing Rulings list (whose only other change is the sentence-final period becoming a list
separator). Criterion 11 was APPENDED rather than inserted beside the schema-structure
criteria, deliberately: criteria 3, 6, 7, and 8 are cited by number from the accepted
`.mochiko/brainstorms/command-md-scaffold-standardization/record.md` and from the v0.98.0
entries in `.mochiko/strips/command-labels.md` and
`.mochiko/strips/converting-command-to-schema.md`, so renumbering would falsify records
already landed. `converting-command-to-schema/SKILL.md` step 14 lists the criteria by name
rather than by number, so its summary order differing from the block's costs nothing. -->

<!-- Wave context: the command-`.md`-scaffold standardization wave (v0.97.0). Ruling:
`.mochiko/brainstorms/command-md-scaffold-standardization/record.md` D1, D6-R2 as amended →
`DECISIONS.md` 2026-08-27 row. Clause inventory: that record's Appendix A. -->

## [v0.97.0] Dual command-audit criteria blocks collapsed to one canonical-scaffold block

- **Disposition:** superseded → the single "Canonical-scaffold criteria — every pair-form
  command, all six" block in `.claude/rules/mochiko/primitive-edits.md`
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/command-md-scaffold-standardization/record.md` D1, D6-R2;
  `DECISIONS.md` 2026-08-27. Clause inventory: that record's Appendix A.)
- **Content:** the two blocks as shipped at v0.96.0 — the "Pair-form commands
  (command-content-schema D9; implement from v0.92.0, the D10 five-command rollout … from
  v0.95.0)" default block, and the "Exception — charter-form commands (`feature.md` D10
  v0.68.0; `plan.md` / `implement.md` ADR 2026-08-13-charter-plan-implement v0.69.0)"
  block with its six-clause per-run contract (i)–(vi) — plus the general audit paragraph's
  bar "`mochiko:validator` grading a command against **the command's own text** — internal
  coherence (default-FAIL goal · harness present … · bindings complete: paths, templates,
  entry condition)". Verbatim source: `git show HEAD:.claude/rules/mochiko/primitive-edits.md`,
  lines 42–48 and 50–99.
- **Kept deliberately:** every criterion both blocks graded, relocated into the new block —
  FAIL survival keyed to the `fail-condition` label set · D11/D14 ID continuity incl. section
  tombstones · `class: floor` = must-survive · the D13 checker as deterministic pre-pass ·
  the D16 provenance sidecar clause · floor present + the `mochiko:patterns-sound-loop`
  pointer at its unchanged three-command scope · the DM's bare-minimum responsibilities ·
  the per-visit desk contract · implement's Entry / run-open-confirmation / attempt-bounds /
  acceptance-gate legs · both do-not-demand negatives · plan approval, author ≠ grader
  independence, decisions reserved to the user, bindings complete.
  Two things did NOT survive, both deliberate: the retired `plan.md` historical-audit
  clause (retired v0.91.0 — nothing shipped for it to grade), and the per-version rollout
  provenance sentence (v0.92.0 / v0.95.0 dates), whose rulings are now cited in the block's
  closing Rulings list.
- **Consumers assessed:** `CLAUDE.md` (two sites, re-keyed the same wave) ·
  `.claude/skills/converting-command-to-schema/SKILL.md` step 11 (re-keyed this wave) ·
  `plugins/mochiko/skills/mochiko/SKILL.md` (re-keyed this wave; its own strip entry in
  `.mochiko/strips/mochiko.md`).
