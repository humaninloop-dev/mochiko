# Strip notes — `templates/analyst-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: producer disclosures machine-first; "foreground prose / no parser"
doctrine reversed).

<!-- Wave context: wave 4 of the hook-enforced-artifact-schema build (v0.109.0) — the write-time
artifact gate ships. Every `.mochiko/` path a shipped primitive names was resolved against the homes
the migration log declares (`plugins/mochiko/migrations/0005-artifact-homes.yaml`, wave 3), and a
path the homes do not carry is re-pointed rather than left to be denied at write time. Ruling for
the [v0.109.0] entry below: `.mochiko/brainstorms/hook-enforced-artifact-schema/record.md` D2 (a
report lands in its home's `reports/` directory) and D3 (the homes as the migration declares them),
with that session's `wave4-plan.md` section (c). Pre-edit verbatim text:
`git show 794cea8:plugins/mochiko/templates/analyst-report-template.md`. -->

## [v0.109.0] Usage Note 5's output location sits at an undeclared name in the spec home

- **Disposition:** superseded → `.mochiko/specs/<slug>/reports/analyst-report.md`, the spec home's
  declared `reports/` directory, where a report is admitted under any name by its `report:` type
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/hook-enforced-artifact-schema/record.md` D2; migration
  `0005-artifact-homes.yaml`, the `spec` home's `reports:` block)
- **Content:** ``5. **Output location** — `.mochiko/specs/<feature>/analyst-report.md`, seeded and
  collected by the lead.``
- **Kept deliberately:** the seeded-and-collected-by-the-lead clause, and the template's existing
  `report: disclosure` frontmatter — `disclosure` is already a member of the envelope's closed
  type set, so the re-point needed no new report type and no sixth migration.
- **Consumers assessed:** the path appears in this template only; the analyst seat reads its
  output location here. The sibling `techanalyst-report-template.md` carries the same defect and
  is re-pointed in the same wave.

## [v0.22.0] Prose disclosure sections → frontmatter fields
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2): read in-round by the lead (progress/stall) and relayed; terminal once the round closes
- **Content:** `## Summary` free prose (dropped — the spec is what the critic/lead read); `## Assumptions Made` table → `assumptions:` list ({id, assumption, why} one-liners); `## What Changed This Round` prose → `changed_this_round:` ID-cited list (stall detection preserved); `## Notes` free prose → the conditional `## Notes of note` block (only when non-empty); the optional `## What I Created` count table (dropped — a convenience disclosure of counts the critic reads from spec.md directly). The "Foreground prose; write for a human-style reader… no parser" usage note reversed to machine-first. Preserved: no-self-verdict doctrine (now via the envelope), round semantics, output location.
