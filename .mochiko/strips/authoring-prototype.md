# Strip notes — `skills/authoring-prototype`

Entry formats: `strips/README.md`. Shipped at v0.50.0 (ux-mocking-in-specify wave); first
strip entries at v0.58.0 (feature-map-layer wave).

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
