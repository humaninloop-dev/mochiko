# Strip notes — `templates/techanalyst-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: producer disclosures machine-first).

## [v0.46.0] "the shape's producer-authored branch" re-pointed (audit finding 1)
- **Disposition:** superseded → "the dispatching command's producer-authored branch"
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row) — audit-caught consumer, fixed at re-grade
- **Content:** rule 6's parenthetical "(the shape's producer-authored branch)" — the shape home was deleted this wave; the ADR's re-point list missed this consumer, caught by the wave audit.
- **Consumers assessed:** the producing seat's briefs unchanged.

## [v0.22.0] Prose disclosure sections → frontmatter fields
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2)
- **Content:** `## What Was Produced` prose → the `produced:` artifact list (the artifacts themselves are what reviewers grade); `## What Changed This Round` prose → `changed_this_round:` ID-cited list; `## Governance Alignment` prose → `governance_alignment:` one-liner; `## Open Questions` prose → `open_questions:` list; `## Handoff to Review` prose → `handoff:` one-line pointer; the optional `## Artifacts Produced` table (folded into `produced:`). The "Foreground prose… no parser" usage note reversed to machine-first. Preserved: no-self-verdict (no Completion/ready field), phase disclosure semantics, handoff-is-a-pointer-not-a-claim, output location.
