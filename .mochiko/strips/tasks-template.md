# Strip notes — `templates/tasks-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4;
ratified 2026-07-24).

## [v0.49.0] Rewritten task list → cycle cards
- **Disposition:** superseded → the cycle-card form in the same file (per card: heading checkbox · Stories+rationale · type · Depends on · Case · acceptance criteria by ID · brownfield exposure line · `**TEST:**` gate block)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2+D3)
- **Content:** the `TN.X` task-line skeleton (per-task checkbox/ID/description/file path), the Cycle Format TDD ladder (TN.1 failing test … TN.X TEST), the 4-marker table (`[P]`/`[US#]`/`[EXTEND]`/`[MODIFY]` at task level), the Checkpoint line, and the Story→Cycle Mapping table with its derived-echo-of-`task-mapping.md` note. Full text: git history at v0.48.0.
- **Kept deliberately:** `[P]` at cycle level · the `**TEST:**` gate (grammar unchanged, TEST-GRAMMAR.md still owner) · the never-compress rule for TEST commands/paths · foundation-sequential/feature-parallel structure. The Stories+Case card lines absorb `task-mapping.md`'s content — that artifact had no template file; its mapping+rationale role ends here.
- **Consumers assessed:** patterns-vertical-tdd (fills it) · executing-tdd-cycle (reads cards, decomposes at build time) · testing-end-user (gate parsing re-keyed) · review-plan-artifacts (grades it) · plan/implement commands.

## [v0.23.0] Execution Strategy + Notes sections stripped from the tasks.md skeleton
- **Disposition:** deleted (doctrine restatement into the artifact — R2 rule 7)
- **Tier failed:** 1 (altitude): "Execution Strategy" (MVP delivery / incremental delivery / parallel team strategy) and "Notes" (TDD discipline, vertical slices, foundation-first, parallel features, commit strategy, checkpoint validation) restate what `patterns-vertical-tdd`, `executing-tdd-cycle`, and the implement command single-source — and were regenerated verbatim into every authored tasks.md (kinako s1/tasks.md 54k B)
- **Content:** the two sections verbatim as listed above; identical every feature, zero feature-specific substance.
- **Consumers assessed:** implement's producer/verifier read cycles + task lines, never these sections (epic F-c consumption evidence); review-task-artifacts' checks are cycle/task-level — no checklist referenced either section (grep-verified).

## [v0.23.0] Cycle-dependency ASCII diagram stripped; Story→Cycle table kept as the ID index
- **Disposition:** diagram deleted; the Story→Cycle derived-echo table **kept** and designated the artifact's **ID index** per `templates/artifact-format.md` (the derived-echo annotation stands — task-mapping.md remains the source of truth)
- **Tier failed:** 1 (restatement): every dependency in the diagram already lives on its cycle's `> Dependencies:` header line; the diagram was a hand-drawn second copy that could drift
- **Content:** the `### Cycle Dependencies` fenced ASCII graph; a pointer line ("dependencies live on each cycle's header — no separate diagram") lands in the kept section.
- **Consumers assessed:** implement's lead sequences cycles from the `> Dependencies:` headers; review-task-artifacts' dependency checks read per-cycle headers (PHASE-CHECKLISTS grep-verified — no diagram reference).

## [v0.23.0] Sample cycles trimmed 5 → 2 (one foundation, one feature)
- **Disposition:** deleted (template-load density); the fill-guidance comment now points at `patterns-vertical-tdd/references/CYCLE-STRUCTURE.md` for more worked examples (auth, filtering, brownfield markers)
- **Tier failed:** 1 (duplication): CYCLE-STRUCTURE.md already carries six worked cycle examples — three extra samples in the template taught nothing the producer's own reference doesn't
- **Content:** sample cycles 2 (auth foundation), 4, 5 (feature variants) — all present in equivalent form in CYCLE-STRUCTURE.md.
- **Consumers assessed:** tasks producer (loads both template and CYCLE-STRUCTURE per its skill); samples are replaced on fill by instruction, so downstream readers never saw them.
