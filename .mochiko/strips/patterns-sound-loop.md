# Strip notes — `skills/patterns-sound-loop`

Entry formats: `strips/README.md`.

## [v0.91.0] Seat-wiring table: "plan-time deltas / the plan review pair" re-keyed to the design phase — plan-stage retirement D1/D5

- **Disposition:** superseded → "design-phase deltas graded by the design review pair beneath the
  user's sign-off".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 (the design phase absorbs plan's
  design work) and D5 (`review-plan-artifacts` / `review-feasibility` re-scope to grade the
  design-phase output); wording ruled by the wave lead 2026-08-26).
- **Content (superseded fragment, verbatim — the architecture-store row's middle column):**

  ```
  desk judgment writes take the tech-lead review leg; plan-time deltas graded by the plan review pair beneath the user's sign-off
  ```

- **Kept deliberately:** the row's full floor obligation — the tech-lead review leg on desk
  judgment writes, the beneath-the-user's-sign-off placement, the `authoring-architecture-store`
  pointer, and the third column's "full floor — satisfied by the desk's own loop where it runs;
  the bite is any store write outside it". The review **pair** survives as a pair; only its name
  moved with the stage.
- **Budget:** unbudgeted (hard-cap-only). Body 6,358 → **6,363**; description unchanged at 505,
  inside the 1,536 delivery cap.
- **Consumers assessed:** the pair's two skills were re-scoped earlier in this wave
  (`review-plan-artifacts` to the sufficiency gap list, `review-feasibility` to design-phase
  artifacts), so this row's naming now matches both. The three charter commands reference this
  floor without restating it — unaffected.

## [v0.81.0] Governing-surface row re-keyed: `ARCHITECTURE.md` folds → the architecture store — product-architecture-schema D3/D4/D11/D12

- **Disposition:** superseded → a row keyed on the **architecture store**
  (`.mochiko/product/architecture/`), the full floor. The old row's subject — `ARCHITECTURE.md`
  folds outside landings — no longer names a writable surface: the root doc is now a **derived
  index** the store skill regenerates on every store write (D4 fold, single writer), so there is
  nothing to hand-fold and the net-new bite moved to the store itself.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D3/D4 (one store, derived index) ·
  D11 (store-write review cadence, as narrowed at S6) · D12 (store homed under
  `.mochiko/product/architecture/`); `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim):**

  ```
  | `ARCHITECTURE.md` folds outside landings | landing-time diff only (`mochiko:authoring-architecture`) | full floor — net-new bite |
  ```

  Plus the `description:` clause naming `` `ARCHITECTURE.md` `` in the governing-surface list, and
  the product-baselines row's subject line, narrowed to "the rest of `.mochiko/product/`" so the
  two rows do not silently double-cover one directory.
- **Kept deliberately:** the floor's whole shape — the two-part trigger with no size threshold,
  the three legs, the three exemptions, the no-delta-card-exemption rule, out-of-remit hosting,
  and the disclosure line's pinned grammar. The **default seat-wiring table is untouched**: its
  "Architecture / baseline touches → `principal-architect` produces / `tech-lead` reviews" row
  already states exactly the pairing D11 rules for store writes.
- **Addition riding the decision row (no strip):** one sentence under Exemptions naming both
  halves of D11-as-narrowed — status flips and orphan cleanup are transcription; `As-built:` and
  `Drift:` writes are judgment and graded. Recorded because the omission's natural reading (the
  whole landing is bookkeeping) is precisely the under-classification the disclosure line exists
  to make auditable.
- **Consumers assessed:** the router's `patterns-sound-loop` row restates the governing-surface
  list and is re-keyed in the same edit set by this seat. The three charter commands
  (`feature.md` / `plan.md` / `implement.md`) carry the floor **pointer** in their Boundaries,
  never the surface list — unaffected (P2 owns them this wave regardless).
  `mochiko:authoring-architecture-store` (P1) is the newly named carrier.

## [v0.71.0] Neutrality line narrowed — transport-choice-neutral, transport-use carries a floor
- **Disposition:** superseded → `mochiko:patterns-transport-floor` (the new transport-use floor now carries the discipline the flat "Transport stays neutral" line elided)
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/teammate-message-races/record.md` D3–D5; `DECISIONS.md` 2026-08-14 row)
- **Content:** Transport stays neutral — a seat may be a teammate or a subagent, the lead's per-seat call; what dies above the floor is the lead absorbing the seat.
- **Kept deliberately:** the transport-*choice* neutrality (realignment D5) and the "lead absorbing the seat" clause both survive verbatim in the reworded Overview line; the ruling narrows only the scope of "neutral" — choice stays neutral, use gains a floor — it does not reverse D5.
- **Consumers assessed:** n/a — not a shared-mechanics file; the amended sentence is internal to patterns-sound-loop's Overview, referenced by no other primitive.
