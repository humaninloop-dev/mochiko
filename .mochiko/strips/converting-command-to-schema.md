# Strip notes — `.claude/skills/converting-command-to-schema/SKILL.md`

Entry formats: `strips/README.md`. First entries at v0.98.0 — no prior strip file existed
for this repo-side maintainer skill (it lives at `.claude/skills/` and is never shipped);
its mint and its v0.97.0 scaffold re-key rode their decision rows.

<!-- Wave context: the command-schema ontology wave (v0.98.0). Ruling:
`.mochiko/brainstorms/command-schema-ontology/record.md` D1–D11 as amended →
`DECISIONS.md` 2026-08-27 row. Clause inventory:
`.mochiko/brainstorms/command-schema-ontology/conversion-inventory.md` (sections G and H
carry the two forms this wave replaces). -->

## [v0.98.0] The `fail-condition` label re-keyed to `kind: fail` at four sites

- **Disposition:** superseded → the same four clauses, re-keyed to `kind: fail`, in the
  amended procedure (steps 11 and 13, step 14's criteria summary, and the Pitfalls list).
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/command-schema-ontology/record.md` D1 and build-surface item 4,
  "`kind: fail` replaces the `fail-condition` label as the operative selector";
  `DECISIONS.md` 2026-08-27. Reworded lines tabled in that session's
  `conversion-inventory.md` section H.)
- **Content:** the four clauses as they stood at v0.97.0, verbatim —

  1. Procedure step 8 (now step 11), the `.md` scaffold's Adaptive Goal Protocol item:
     "**Not done — default FAIL**, always last, pinning the fail-condition count in the
     literal phrase form the checker greps — \"the N rules labeled `fail-condition`\" — plus
     the out-of-sync halt clause (count mismatch = halt and surface before closing)."
  2. Procedure step 10 (now step 13): "No unchecked hard-coded counts — the fail-condition
     count is the sole checker-guarded one."
  3. Procedure step 11 (now step 14), the criteria summary: "scaffold conformance, set-wise
     section enumeration, label-keyed FAIL survival, D11/D14 ID continuity (rule + `.sec.`
     IDs), `class: floor` = must-survive, the substance legs, and the done-condition branch."
  4. Pitfalls, two bullets: "Hard-coded block counts drifting — strike every count except the
     checker-guarded fail-condition one." and "Forgetting the literal Not-done phrase — the
     checker greps \"the N rules labeled `fail-condition`\"; a paraphrase defeats the C2 count
     guard."

- **Kept deliberately:** every guard the old key carried survives on the new one — the
  literal-phrase requirement and the checker's grep of it, the out-of-sync halt clause, the
  sole-checked-count rule, and the paraphrase warning. The `<cmd>.sec.fail-conditions`
  section ID and the `<cmd>.fail.*` ID segment are untouched by the re-key: both are ID
  grammar, not the retired label. Step 9's registry-only rule for the eleven labels that
  remain is unchanged.
- **Consumers assessed:** `plugins/mochiko/schemas/command-labels.yaml` (the registry line
  removed the same wave; entry in `.mochiko/strips/command-labels.md`) ·
  `.claude/rules/mochiko/primitive-edits.md` criterion 3 (re-keyed the same wave; entry in
  `.mochiko/strips/primitive-edits.md`) · the six command `.md` Not-done lines and their
  schemas' fail nodes (re-keyed the same wave) · `scripts/check-command-schema.py`.

## [v0.98.0] The referential-closure namespace list superseded by the canonical header's

- **Disposition:** superseded → the addressable-namespace list of the canonical schema header
  comment (ontology `conversion-inventory.md` section G), restated in procedure step 4.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/command-schema-ontology/record.md` D1/D3/D4/D8 grow the namespace,
  and command-content-schema D16 evicted the inline `ruling:` field it still listed;
  `DECISIONS.md` 2026-08-26 and 2026-08-27.)
- **Content:** step 4 as it stood at v0.97.0, verbatim —

  > 4. **Referential closure** (D15). No deixis — every reference in a rule's text resolves
  >    in-block or via the addressable namespace: `${var}` names, rule IDs, section IDs, `class:`
  >    values, registry labels, `pointer:` skills, `ruling:` anchors, literal file paths. "this
  >    schema" and "the run" are legal self-reference.

- **Kept deliberately:** the whole D15 rule and every namespace member that survives —
  `${var}` names, rule IDs, section IDs, `class:` values, registry labels, `pointer:` skills,
  literal file paths, and the two legal self-references. Only `ruling:` anchors left the list,
  and they left because D16 had already moved decision anchors to `.mochiko/provenance.yaml`
  and made an inline `ruling:` field a checker finding — the same step 10 of this skill said
  so at v0.97.0, so the list was internally contradictory before this edit. Added, not
  removed: `common.*` block IDs, `kind:` values, and `conditions:`/`moments:` names.
- **Consumers assessed:** the six command schemas' header comments (the canonical header
  carries the identical list; normalized the same wave) · `scripts/check-command-schema.py`
  (its deixis lint and anchor resolution are unchanged by this edit).

<!-- Renumbering note, no content: three new procedure steps (6 run-shape grammar,
7 `enforces:`, 8 `extends:`) were inserted after `vars:`, shifting the former steps 6–13 to
9–16. `.mochiko/strips/primitive-edits.md`'s v0.97.0 entry cites "SKILL.md step 11" for the
primitive-edits coverage step; that step is now 14. The historical entry is left as written —
it was accurate at its stamp. -->
