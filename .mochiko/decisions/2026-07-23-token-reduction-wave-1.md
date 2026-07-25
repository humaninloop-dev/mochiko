# Token-reduction wave 1 — report layer, build rulings

**Status:** ruled · **Date:** 2026-07-23 (v0.22.0)
**Context:** executing the `workflow-token-reduction` epic's report layer (epic record: `.mochiko/brainstorms/workflow-token-reduction/record.md`); four wave rulings were ratified in-session at build time. Full edit surface + audit trail: `.mochiko/archive/ROADMAP.md` (Decision Trail, "wave 1").

**Decisions (user, in-session):** (1) scope = all report formats, deliverables untouched; (2) form = YAML-first `.md`, machine-first — the sole consumer is the lead; (3) wave load = D2 manual run-cost baseline + D6a/b/d folded in; (4) strengths sections → a one-line `strengths:` field.

**Rationale:** the report layer was the epic's largest pure-waste pool (~102k tok est./feature, never read by the user).

**Alternatives:** per-format piecemeal strips (rejected — one envelope single-source, `templates/report-format.md`, prevents re-drift).
