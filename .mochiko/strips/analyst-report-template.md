# Strip notes — `templates/analyst-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: producer disclosures machine-first; "foreground prose / no parser"
doctrine reversed).

## [v0.22.0] Prose disclosure sections → frontmatter fields
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2): read in-round by the lead (progress/stall) and relayed; terminal once the round closes
- **Content:** `## Summary` free prose (dropped — the spec is what the critic/lead read); `## Assumptions Made` table → `assumptions:` list ({id, assumption, why} one-liners); `## What Changed This Round` prose → `changed_this_round:` ID-cited list (stall detection preserved); `## Notes` free prose → the conditional `## Notes of note` block (only when non-empty); the optional `## What I Created` count table (dropped — a convenience disclosure of counts the critic reads from spec.md directly). The "Foreground prose; write for a human-style reader… no parser" usage note reversed to machine-first. Preserved: no-self-verdict doctrine (now via the envelope), round semantics, output location.
