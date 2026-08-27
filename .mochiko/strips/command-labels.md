# Strip notes — `plugins/mochiko/schemas/command-labels.yaml`

Entry formats: `strips/README.md`. First entry at v0.98.0 — no prior strip file existed for
this registry; earlier edits (its mint at v0.92.0, the D10 rollout additions at v0.95.0)
rode their decision rows as pure additions.

<!-- Wave context: the command-schema ontology wave (v0.98.0). Ruling:
`.mochiko/brainstorms/command-schema-ontology/record.md` D1–D11 as amended →
`DECISIONS.md` 2026-08-27 row. Clause inventory:
`.mochiko/brainstorms/command-schema-ontology/conversion-inventory.md`. -->

## [v0.98.0] The `fail-condition` label retired — `kind: fail` is the selector

- **Disposition:** superseded → the `kind: fail` field on the fail nodes of the six command
  schemas (`plugins/mochiko/schemas/<cmd>.yaml`), which is now the operative selector for
  the Not-done set; the registry header carries a one-line retirement note in its place.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/command-schema-ontology/record.md` D1 and build-surface item 4,
  "`kind: fail` replaces the `fail-condition` label as the operative selector";
  `DECISIONS.md` 2026-08-27. Inventory: that session's `conversion-inventory.md` section H.)
- **Content:** the registry line as shipped at v0.97.0, verbatim —

  ```yaml
    fail-condition: A Not-done clause — any one standing fails the run; the command .md's hard-coded count keys to exactly this set.
  ```

- **Kept deliberately:** everything the label carried survives on the new key, not in the
  registry. The Not-done count pin survives re-keyed — each command `.md` counts "the N
  rules of `kind: fail`" and keeps its out-of-sync halt clause (section H). The
  bidirectional cross-check survives too: the D13 checker keeps asserting `<cmd>.fail.*`
  segment membership against `kind: fail` in both directions (I4), with `kind: fail` never
  defaulted onto a `.fail.*` ID. The label's 36 `labels:` occurrences leave the six schemas
  in the same wave; the eleven registry labels that remain are untouched.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` criterion 3 (re-keyed
  this wave; its own strip entry in `.mochiko/strips/primitive-edits.md`) ·
  `.claude/skills/converting-command-to-schema/SKILL.md` steps 8 and 10 plus its Pitfalls
  list (re-keyed this wave; strip entries in
  `.mochiko/strips/converting-command-to-schema.md`) · the six command `.md` Not-done lines
  and the six schemas' fail nodes (re-keyed this wave, per schema strips) ·
  `scripts/check-command-schema.py` (checker extension, same wave).

## [v0.98.0] The registry header's "no shared rule library" clause narrowed

- **Disposition:** superseded → the amended clause in the same header, which names
  `plugins/mochiko/schemas/common.yaml` as the one exception and states its bar (text that
  is an exact duplicate across three or more command schemas).
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/command-schema-ontology/record.md` D8 as amended by C2 —
  "a recorded supersession-by-ruling of command-content-schema D3 … amended, not
  reversed"; `DECISIONS.md` 2026-08-27.)
- **Content:** the clause as shipped at v0.97.0, verbatim —

  ```
  # (command-content-schema D3/D8, DECISIONS.md 2026-08-26). Rules live per-command
  # (no shared rule library); the SAME LABEL on rules in different command schemas is
  # the cross-command link.
  ```

- **Kept deliberately:** both halves of what D3 bought. Per-command rules remain the
  **default**, and the same label across command schemas remains the cross-command link —
  D8 narrowed the prohibition, it did not retire the mechanism. The header still cites
  command-content-schema D3/D8 as the registry's own ruling.
- **Consumers assessed:** `plugins/mochiko/schemas/common.yaml` (the exception this clause
  now names, shipped the same wave) · `.claude/skills/converting-command-to-schema/SKILL.md`
  step 6 (labels-from-the-registry-only, unchanged — the registry bar is untouched by D8)
  and its new `extends:` step · the six command schemas' header comments (the canonical
  header, section G of the inventory, carries the `extends:` grammar).
