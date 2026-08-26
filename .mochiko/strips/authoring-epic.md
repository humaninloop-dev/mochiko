# Strip notes — `skills/authoring-epic`

Entry formats: `strips/README.md`. Skill born at v0.72.0 (the multi-feature-plan-implement wave,
D1–D15); this file opens with the first edit that superseded any of its shipped text.

## [v0.91.0] Fix round 2 — the manifest's Why-together line cited the dead mint mechanism (V1 multiline sweep)

- **Disposition:** superseded → "the relatedness stated at the **desk mint** that opened it".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D4**: plan's inline epic mint door
  dies with the command; epic minting is desk-only). Caught by V1's **multiline-aware** sweep —
  this seat's greps were line-scoped, and the phrase "declare-and-contest" names the dead
  mechanism without containing any of the plan-vocabulary terms the sweeps matched on.
- **Content (superseded text, verbatim):**

  ```
  - **Why-together line** — the relatedness stated at the declare-and-contest that opened it.
  ```

- **Kept deliberately:** the Why-together line itself as a required manifest field, and its
  purpose — the manifest records *why these members belong together*, captured at the moment the
  epic opened. Only the naming of that moment changed, and it now matches the two sites the main
  pass already re-keyed: the desk-is-the-only-mint-door rule and the Red-Flag line.
- **Budget:** unbudgeted (hard-cap-only). Body 7,831 → **7,821** (the replacement is shorter);
  description unchanged at 496. (Canonical-snippet count taken after the edit, not estimated.)
- **Consumers assessed:** the desk (`/mochiko:feature`) is the sole mint door and owns the
  moment this line records — verified consistent with this file's own Minting section and Red
  Flags after the edit.

## [v0.91.0] Fix round — the When-NOT-to-Use grading pointer re-keyed (V2 B5)

- **Disposition:** superseded → "the design-phase and implement outputs are graded by their
  cluster reviewers".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1). Raised by the v0.91.0 wave audit
  as **V2 B5** — the main pass re-keyed six plan/implement pointers in this file and missed this
  seventh.
- **Content (superseded text, verbatim):**

  ```
  - **Grading the epic** — the plan/implement outputs are graded by their cluster reviewers; this
    skill authors the epic object and never grades its own output.
  ```

- **Kept deliberately:** the carve-out entire — this skill authors the epic **object** and never
  grades its own output; grading belongs to the cluster reviewers, not here. Only the name of the
  upstream cluster changed.
- **Budget:** unbudgeted (hard-cap-only). Body 7,819 → **7,831**; description unchanged at 496.
- **Consumers assessed:** the router's `authoring-epic` row (re-keyed in the main pass);
  `implement.md` (P1's rewrite) hosts both clusters that do the grading.

## [v0.91.0] "Joint plan-the-plan proposal" spine artifact re-keyed to the joint design-phase plan — plan-stage retirement D1 (b)

- **Disposition:** superseded → the **Joint design-phase plan** — one plan over all members,
  authored in the epic implement run's design phase.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 mechanic (b): "an epic run always
  fires the design phase for the joint spine … the epic's 'joint plan-the-plan proposal' spine
  artifact re-keys to the joint design-phase plan").
- **Content (superseded text, verbatim):**

  ```
  - **Joint plan-the-plan proposal** — one proposal over all members (spine artifacts + each
    member's artifact list).
  ```

  Replaced by the joint design-phase plan carrying the same contents, plus the mechanic-(b)
  obligation that an epic run always fires the design phase for this spine whatever the
  sufficiency verdict said.
- **Kept deliberately:** the one-over-all-members shape and its contents (spine artifacts +
  each member's artifact list) — D1 (b) moves where the artifact is authored and renames it;
  it does not change what it covers. The design-time cross-member seam-owner rule and the
  shared-baseline single-pen-holder rule are untouched, both explicitly carried by mechanic (b).
- **Consumers assessed:** the router's `authoring-epic` row (re-keyed same wave), `implement.md`
  (P1's rewrite fires the epic spine design phase always), `feature.md`'s epic dispatch line
  (re-keyed same wave).

## [v0.91.0] The `/mochiko:plan` inline epic mint door dies — desk-only minting — plan-stage retirement D4

- **Disposition:** superseded → two doors instead of three; the desk is the only mint door, and
  the overlap guard relocated onto the desk door it now qualifies.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D4: "Plan's inline epic mint door
  (declare-and-contest from a bare feature list) dies with it — epic minting is desk-only
  (`/mochiko:feature`), recorded supersession". The review's cross-examination recorded this as
  a *partial* kill of survivor B-C3: epic minting itself survives at the desk; only plan's
  inline door dies.)
- **Content (superseded text, verbatim):**

  ```
  feature's pending rows belong to **at most one open epic at a time**. Three doors:

  - **`/mochiko:feature` desk** owns the epic's life — mint, membership change, status view, close.
  - **`/mochiko:plan`** invoked with a bare feature list **may mint inline** through the
    declare-and-contest — but only after resolving against open epics: any **membership overlap
    with an existing epic surfaces to the user** (join it / rule on the overlap), never a silent
    duplicate.
  ```

  And, in Red Flags:

  ```
  - "Specify found a multi-capability derivation — I'll mint the epic now" — specify **proposes**,
    never mints; the desk or a plan invocation mints
  ```

- **Kept deliberately:** the **membership-overlap guard in full** — "any membership overlap with
  an existing epic surfaces to the user (join it / rule on the overlap), never a silent
  duplicate" now rides the desk door, which is the only door that mints. Mint-once, resolve-by-
  lookup, one-open-epic-per-feature's-pending-rows, and specify's proposes-never-mints door are
  all untouched. A new line makes the consequence explicit: an implement run resolves its
  `EPIC-XXX` by lookup and mints nothing.
- **Consumers assessed:** `specify.md`'s epic-proposal line (re-keyed same wave), `feature.md`'s
  epic stewardship (already desk-owned, unchanged), the router's `authoring-epic` row (re-keyed
  same wave).

## [v0.91.0] Plan/implement pointer re-keys across the skill — plan-stage retirement D1

- **Disposition:** superseded → the same five pointers naming `implement` alone.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1).
- **Content (superseded fragments, verbatim — five sites):**

  1. `description:` boundary clause: `NOT plan/implement mechanics`
  2. Overview: `` multi-feature batch** — members planned and built as one unit through `/mochiko:plan` and ``
     `` `/mochiko:implement`. It has identity ``
  3. Overview: `` commands that consume it (`plan`, `implement`, `feature`, `specify`) **reference this skill; ``
  4. Overview: `**spine is the tech view** — plan and implement consume the spine sections.`
  5. When NOT to Use:

     ```
     - **Plan / implement run mechanics** — gate shapes, cycle sequencing, and landing steps live in
       `plan.md` / `implement.md`; this skill single-sources only the epic object they consume.
     ```
  6. Spine files lead-in: `**Spine files** beside it — the tech view plan and implement consume.`
  7. Related: `` - `plan.md` / `implement.md` — consume the spine; reference this skill, never restate it ``

- **Kept deliberately:** every boundary the pointers carried — the epic-object-only remit, the
  map-marker/seam-grammar carve-out to `mochiko:authoring-feature-map`, the transport-floor
  carve-out, and the reference-never-restate contract with the consuming command.
- **Consumers assessed:** `implement.md` (P1's rewrite consumes the spine), the router's
  `authoring-epic` row (re-keyed same wave). `plan.md` is deleted by P2 in the same wave, so no
  dead pointer survives.

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
