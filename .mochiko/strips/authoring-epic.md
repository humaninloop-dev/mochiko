# Strip notes — `skills/authoring-epic`

Entry formats: `strips/README.md`. Skill born at v0.72.0 (the multi-feature-plan-implement wave,
D1–D15); this file opens with the first edit that superseded any of its shipped text.

## [v0.81.0] Joint architecture delta = one signed store delta; In-flight pointer clear → store key clear — product-architecture-schema D3/D10

- **Disposition:** superseded → the epic's joint architecture output is **one signed store
  delta** for the whole epic (rendered once, signed once), and its close semantics re-key from
  the retired In-flight-pointer machinery to the store's own lifecycle: in-flight-class elements
  flip `built` and their `FEAT-XXX` keys clear.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D3 (one store) · D10 (six-step
  delta lifecycle; the orphan rule supersedes the pinned AT-D6-C In-flight-pointer invariant);
  `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — two fragments):**

  ```
  - **Joint architecture + seam design** — one rendered delta; **each cross-member seam names its
    owner explicitly** (members land simultaneously, so no later-lander default applies). The
    assignment lives here; the map writes it at close.
  ```

  ```
  One acceptance landing executes **each member's graduation batch** (extent fold, status
  `delivered`, In-flight pointer clear) **plus the epic close**:
  ```
- **Kept deliberately:** the epic's **one joint sign-off** survives exactly as D10 preserves it —
  one delta, one rendered diagram, one signature for the whole epic, never one per member. Every
  seam-owner rule is untouched: explicit naming in the spine, no later-lander default inside an
  epic, the map writing the assignment at close. The shared-baseline single-pen-holder rule and
  the fold-each-baseline-exactly-once guarantee are untouched — and now cover the store, which is
  a baseline under `.mochiko/product/` like any other. The member-scoped halt disposition stays
  the user's.
- **Addition riding the decision row (no strip):** a one-clause disambiguation on the Spine-files
  line — *spine* in this skill is the epic's, never the architecture store's topology spine. The
  two senses collide only after this wave introduces the second one; naming it once is cheaper
  than a reader conflating an epic spine file with the store's `spine.md`.
- **Consumers assessed:** `plan.md` / `implement.md` consume the spine and are P2's in this wave
  (the joint-sign-off gate is stated there and re-keyed there). `mochiko:authoring-feature-map`
  owns the `[EPIC-XXX]` marker grammar — untouched by this edit. The router's `authoring-epic`
  row was checked and **left unedited**: it names "joint architecture/seam design" and
  "shared-baseline joint deltas authored once under a single pen-holder" generically, both of
  which stay literally true once the joint delta is a store delta.
