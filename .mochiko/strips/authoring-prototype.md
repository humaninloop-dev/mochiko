# Strip notes — `skills/authoring-prototype`

Entry formats: `strips/README.md`. Shipped at v0.50.0 (ux-mocking-in-specify wave); first
strip entries at v0.58.0 (feature-map-layer wave).

## [v0.76.0] `spec-template.md` read-pointers → `spec` schema (two-arm CLI / raw Read) — schema-based-template-guidance D1/D8
- **Disposition:** superseded → `mochiko-cli template spec`, or Read `plugins/mochiko/schemas/spec.yaml` raw (D8-first-class). Two sites re-pointed: the Overview "in the shape … defines" pointer and the Related-section pointer.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D1/D3/D8; `DECISIONS.md` "Template-schema CLI ruled").
- **Content (superseded, verbatim):**
  - `in the shape` / `[\`spec-template.md\`](../../templates/spec-template.md) defines:` — Overview item 2
  - `- [\`spec-template.md\`](../../templates/spec-template.md) — owns the Screens & Flows section shape this skill fills` — Related
- **Kept deliberately:** the `artifact-format.md` pointer (lines 26, 140) — not an in-scope template, stays `.md`; all surrounding descriptive text.
- **Consumers assessed:** n/a (single-writer skill; no shared-primitive fan-out).

## [v0.63.0] Guardrails body + slim description (guardrails-vs-detail benchmark verdict)
- **Disposition:** superseded → benchmark-ruled guardrails body + slim description
  (`.mochiko/benchmarks/guardrails-vs-detail/variants/body/authoring-prototype/` and
  `variants/descriptions/authoring-prototype/`; the shipped file is the deterministic merge of
  the two).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict,
  `DECISIONS.md` 2026-08-10 benchmark-verdict row; record
  `.mochiko/brainstorms/validator-scope-and-verbosity/record.md`, Benchmark execution;
  `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md` — body arm formal D6 PASS,
  descriptions arm 0 fire misses).
- **Content (faithfully compressed):** body 10,203 → 8,898 chars (−13%); description 1,006 → 493
  chars (−51%). Body cut: **When to Use** deleted whole (four bullets restating invocation
  conditions incl. lockstep authoring, gap-list revision, the FEAT re-tag pass) and the
  six-step **Process** walkthrough deleted whole (read intent ruling → discover design system →
  build skeleton → per-story lockstep → FEAT re-tag → self-walk; each step's obligation
  survives as an invariant or checklist row). Description cut: the SCR/FLOW shape detail,
  trigger-phrase enumeration and boundary sentences compressed to the MUST clause + core
  triggers. Verbatim homes: git history of this file (pre-v0.63.0), the before/after pair under
  `variants/`, and archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the guardrails keep-set — the two-deliverables Overview, When NOT to
  Use, all eight invariants (skeleton-first, lockstep, scenario keying, manifest↔HTML agreement,
  low-fi discipline, design system, FEAT tags carried with the re-tag pass, rejected-story
  greying — the [v0.58.0] supersession's replacement grammar intact), the checklist, and red
  flags. No prior KEPT or protected line is touched; the [v0.58.0] entry's kept-set survives in
  full.
- **Consumers assessed:** product-engineer (mounts it; invariants unchanged, so persona contract
  intact) · specify (binds it; the deleted Process steps' obligations remain enforced by the
  invariants + checklist the command reads) · review-specifications (its Screens & Flows table
  grades the same manifest shape, untouched by this cut). Contract intact.

## [v0.58.0] Slice-tag grammar re-keyed to FEAT tags (R10)
- **Disposition:** superseded → the same greying grammar re-keyed: invariant 7 becomes "FEAT tags carried — a re-tag pass at derivation" (tags cannot exist during lockstep authoring — derivation runs after stories, so FEAT tags land as a re-tag pass over the SCR/FLOW manifest); new invariant 8 keeps a filter-rejected story's screens greyed, marked rejected, pointed at the recorded rejection. Map machinery single-sourced in `mochiko:authoring-feature-map`.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)", record `.mochiko/brainstorms/feature-map-layer/record.md`, D4 slices retire · R10 filter-vs-lockstep build lines).
- **Content (verbatim, the superseded lines):**
  - description clauses: `SCR-XXX screen entries (purpose, data shown, slice tag)` and `FLOW-XXX click-path entries (step sequences keyed to story acceptance scenarios, slice tag)` — "slice tag" → "FEAT tag" in both;
  - Overview manifest parenthetical: `SCR-XXX rows (screen, purpose, data shown, slice)` / `FLOW-XXX rows (click-path steps, the story acceptance scenario each keys to, slice)`;
  - When-to-Use bullet: `Marking out-of-slice screens as coming-soon when the spec decomposes into slices`;
  - invariant 7:
    ```
    7. **Slice tags carried.** When the spec decomposes, every SCR/FLOW row carries its slice tag;
       screens outside the current slice stay present but visibly greyed **coming-soon** — the app
       stays a coherent whole, not a stub maze.
    ```
  - Structure bullet clause: `or a stub page carrying the slice tag`;
  - process step 5: `**Tag slices** — when the Delivery Slices section lands, tag every row and grey the out-of-slice screens.`;
  - checklist row: `- [ ] Slice tags on every row where the spec is decomposed; out-of-slice screens greyed, reachable`;
  - red flag: `"Skip the greyed screens, they're not in this slice" — dead-end navigation breaks the clickable whole; coming-soon is cheap`;
  - Related bullet: `mochiko:authoring-slices — the Delivery Slices section whose slice tags the manifest carries`.
- **Kept deliberately:** the greying grammar itself (coming-soon at reduced opacity or stub page, always reachable, never a dead end) — same mechanism, new key; every other invariant (skeleton-first, lockstep, scenario keying, manifest↔HTML agreement, low-fi discipline, design system) untouched.
- **Consumers assessed:** product-engineer (mounts it; re-tag pass is a timing addition, not a persona change) · specify (binds it; wave-2 rebuild lands the derivation stage the re-tag pass follows) · review-specifications (its Screens & Flows check 7 re-keyed to FEAT tags in the same v0.58.0 edit).
