# Strip notes — `templates/architect-report-template.md` (formerly `sysarchitect-report-template.md`, renamed v0.67.0)

Entry formats: `strips/README.md`. This log file retains its original name
(`strips/sysarchitect-report-template.md`) as the primitive's history anchor; the primitive it
tracks is now `templates/architect-report-template.md`.

---

## [v0.67.0] Seat rotation — renamed to `architect-report-template.md`; seat re-keyed system-architect → architecture producer (principal-architect)
- **Disposition:** superseded → `templates/architect-report-template.md` (git mv rename) with the seat naming re-keyed in place; the report machinery survives, only the retired seat name rotates.
- **Tier failed:** n/a — supersession by ruling (record `.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md` **D1** — the two-architect split dies; the architecture producer is now `principal-architect`; `DECISIONS.md` 2026-08-13 row L13).
- **Content (verbatim re-keys — retired-seat text → rotated text):**
  - Title: `# System Architect Report Template` → `# Architect Report Template`
  - Header: `The system-architect's self-disclosure report — authored alongside \`architecture.md\` and the …` → `The architecture producer's (principal-architect) self-disclosure report — authored alongside \`architecture.md\` and the …`
  - Usage note 8 (output location): `.mochiko/specs/<feature>/sysarchitect-report.md` → `.mochiko/specs/<feature>/architect-report.md`
  - Usage note 9: `the system-architect fills in actual content following this structure` → `the architecture producer (principal-architect) fills in actual content following this structure`
- **Kept deliberately:** the entire payload YAML block (report/feature/round/produced/baseline/delta/scope/structural_decisions/changed_this_round/governance_alignment/assumptions/open_questions/handoff) and every self-disclosure / no-self-verdict / machine-first usage rule — untouched; the payload names no seat, so the template's function is unchanged, only its producing seat rotated. The v0.46.0 entry below stands.
- **Consumers assessed (grep of `plugins/` for old filename + output name):** no live command/skill/template references `sysarchitect-report-template.md` or the output `sysarchitect-report.md` by name — `grep -rn` over `plugins/` returned only this template's own (now re-keyed) self-references; the plan-cluster harness collects the architect report generically. Out-of-cluster references left for the lead: `.mochiko/strips/plan.md` L844 (a historical [vX] entry naming the old filename + seat — a stale historical reference in the plan cluster's file, not amended here); router `skills/mochiko/SKILL.md` + `plugin.json` (the lead's ripple). `.mochiko/brainstorms/verbosity-caveman-ops-separation/record.md` L145 is a frozen point-in-time template census — not a live pointer, left as-is.
- **Strip-file rename (flagged):** recommend `git mv` of this log to `strips/architect-report-template.md` at ripple for filename-parity with the renamed primitive; left at the original path per the fix-round instruction.

## [v0.46.0] "the shape's producer-authored branch" re-pointed (audit finding 2)
- **Disposition:** superseded → "the dispatching command's producer-authored branch"
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row) — audit-caught consumer, fixed at re-grade
- **Content:** rule 6's parenthetical "(the shape's producer-authored branch)" — the shape home was deleted this wave; the ADR's re-point list missed this consumer, caught by the wave audit.
- **Consumers assessed:** the producing seat's briefs unchanged.
