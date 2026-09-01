# Strip notes — `skills/authoring-epic`

Entry formats: `strips/README.md`. Skill born at v0.72.0 (the multi-feature-plan-implement wave,
D1–D15); this file opens with the first edit that superseded any of its shipped text.

## [v0.101.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2A, authoring family)

Ruling for every entry below: skill-content-schema D3 (three-home boundary) / D8/C4
(protected transfers), `DECISIONS.md` 2026-09-01 rows (Skill-content schema ruled ·
Skill-schema wave-2 family doors ruled — the authoring-family door); census:
`.mochiko/brainstorms/skill-content-schema/census-authoring.md` §A (AE) + §B (AE).
Schema home: `plugins/mochiko/skills/authoring-epic/schema.yaml`. Minted IDs carry the
`authoring-epic.` prefix (omitted below). Map — census §B row → minted ID:
1 `single-sources-epic-shape` · 2 `identity-grammar` · 3 `one-type-two-faces` ·
4 `transient-role` · 5a `independent-grade` (C-A2 stub,
`extends: authoring-common.independent-grade`; `${artifact}` = `epic`, `${grader}` = "the
cluster reviewers of the design-phase and implement runs that consume it" — the protected
"authors the epic **object**" clause of the same carve-out is carried verbatim in
`run-mechanics-routing`'s text, per the wave-lead's residue rule) · 5b `map-grammar-routing`
· 5c `run-mechanics-routing` · 5d `transport-routing` · 5e `delta-batching-parked` ·
6 `home-directory-is-registry` · 7 `manifest-required-fields` ·
8 `design-phase-always-fires` · 9 `one-signed-store-delta` · 10 `seam-owner-named` ·
11 `ordering` · 12 `member-deltas-stay-per-feature` ·
13 `shared-baseline-single-pen-holder` · 14 `transport-floor-binding` · 15 `mint-once` ·
16 `one-open-epic-per-pending-rows` · 17a `desk-only-mint-door` ·
17b `overlap-surfaces-to-user` · 18 `specify-proposes-never-mints` ·
19 `implement-resolves-by-lookup` · 20 `selection-scope-only` · 21 `close-semantics` ·
22 `halt-disposition-user-owned`.
**Section distribution (build call, disclosed):** census gave AE "output 2" without naming
rows; built output = {7 `manifest-required-fields`, 21 `close-semantics`} (what the epic
emits and how it lands), artifact = the 15 rows above minus those two plus the mint
discipline (15 · 16 · 17a ride `sec.artifact`/`sec.reserved` as write mechanics /
reservations: 17a sits in `sec.reserved`). `sec.inputs` is deliberately empty
(`rules: []` + note — census: inputs 0). No `conditions:` block — no rule carries `when:`;
the load-first block legally omits the `when:` grammar sentence (wave-1 RCM-4 wave-wide
ruling).
**Deleted as dedup, no content loss:** `## Red Flags — STOP` (4 lines — each a mirror of
`mint-once` / `one-open-epic-per-pending-rows` + `overlap-surfaces-to-user` /
`specify-proposes-never-mints` + `desk-only-mint-door` / `halt-disposition-user-owned`),
`## Quality Checklist` (8 boxes, all mapped above), and `## Related` (4 pointers — carried
by `map-grammar-routing`, `transport-routing`, `run-mechanics-routing`,
`home-directory-is-registry`/`desk-only-mint-door`). The body keeps the overview, the
spine-vs-store disambiguation clause (the v0.81.0 decision-row addition), and a two-faces
teaching section.
Accounting: body 7,821 → 2,569 (−5,252, obligations out + the load-first Rules block in) +
schema 10,475 = **payload 13,044** (census §F estimate was ~18,800); the delta over the
pre-conversion body is structural overhead (IDs, keys, section scaffolding, reading
grammar) — no content growth claimed. AE was unbudgeted at birth (hard-cap-only); the
conversion re-seed is its first budget, via the ledger's third seeding path, no headroom
(census J-7 — P5 executes the ledger row).

## [v0.101.0] Mint-once + desk-only door + overlap guard — protection transfers (census §A row 1; multi-feature D4 + plan-stage D4, kept deliberately at v0.91.0)
- **Disposition:** superseded — protection transfers to `authoring-epic.mint-once` (floor), `authoring-epic.desk-only-mint-door` (floor), `authoring-epic.overlap-surfaces-to-user` (floor, reservation), and `authoring-epic.implement-resolves-by-lookup` (must), per D8/C4; the provenance sidecar carries the protected status.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema rows).
- **Content:** "**Mint-once.** Every workflow resolves `EPIC-XXX` by **lookup**; re-minting does not exist." · "**The desk is the only mint door.** An implement run resolves its `EPIC-XXX` by lookup and mints nothing." · "any **membership overlap with an existing epic surfaces to the user** (join it / rule on the overlap), never a silent duplicate"
- **Consumers assessed:** `feature.md` (the desk) and `specify.md` reference the skill, never restate the guard (untouched).

## [v0.101.0] One open epic per feature's pending rows — protection transfers (census §A row 2; multi-feature D4)
- **Disposition:** superseded — protection transfers to `authoring-epic.one-open-epic-per-pending-rows` (floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "A feature's pending rows belong to **at most one open epic at a time**."
- **Consumers assessed:** the Red-Flag mirror deleted as dedup (map entry above).

## [v0.101.0] Epic run always fires the design phase — protection transfers (census §A row 3; plan-stage mechanic (b), v0.91.0)
- **Disposition:** superseded — protection transfers to `authoring-epic.design-phase-always-fires` (floor), per D8/C4; the joint-plan contents clause (spine artifacts + each member's artifact list) rides the same rule text.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "An epic run **always fires the design phase** for this joint spine, whatever the sufficiency verdict said."
- **Consumers assessed:** `implement.md` fires the epic spine design phase always (untouched).

## [v0.101.0] One signed store delta — protection transfers (census §A row 4; product-architecture-schema D3/D10, kept deliberately at v0.81.0)
- **Disposition:** superseded — protection transfers to `authoring-epic.one-signed-store-delta` (floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "**one signed store delta** for the whole epic, rendered once and signed off once"
- **Consumers assessed:** `mochiko:authoring-architecture-store` owns the write gate the sign-off feeds (its own pair converted this wave, same seat).

## [v0.101.0] Cross-member seam owner named — protection transfers (census §A row 5; multi-feature D13)
- **Disposition:** superseded — protection transfers to `authoring-epic.seam-owner-named` (must), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "**each cross-member seam names its owner explicitly** (members land simultaneously, so no later-lander default applies). The assignment lives here; the map writes it at close."
- **Consumers assessed:** `mochiko:authoring-feature-map` owns the marker/seam grammar the assignment lands in (P3's member this wave, untouched by this seat).

## [v0.101.0] Shared-baseline single pen-holder + fold exactly once — protection transfers (census §A row 6; multi-feature D10 review fold C1)
- **Disposition:** superseded — protection transfers to `authoring-epic.shared-baseline-single-pen-holder` (floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "A product baseline touched by **two or more members** gets **one joint delta authored in the spine** under a **single pen-holder** … The landing folds each baseline **exactly once** — spine delta for shared baselines, feature delta otherwise — preserving the singular-delta-per-baseline graded fold."
- **Consumers assessed:** `implement.md`'s landing consumes the fold contract by reference (untouched).

## [v0.101.0] Selection-scope only — protection transfers (census §A row 7; multi-feature D11)
- **Disposition:** superseded — protection transfers to `authoring-epic.selection-scope-only` (floor), per D8/C4; the correct-by-constraint rationale line stays with the rule's context, not restated in the body.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "Every member enters as **selection scope** — a spec-accepted selection or growth rows. **Delta-scope cards** (bug/improvement deltas on delivered capabilities) **cannot join an epic**; the graduation-shaped close is thereby correct by constraint, not presumption."
- **Consumers assessed:** the `description:` boundary clause carries a sanctioned dual statement (census J-5 — no rule minted for the description copy).

## [v0.101.0] Carve-out / hold reserved to the user — protection transfers (census §A row 8; multi-feature D7)
- **Disposition:** superseded — protection transfers to `authoring-epic.halt-disposition-user-owned` (floor, reservation), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "A member that **exhausts its attempt bound or hits the no-progress stop** halts **member-scoped**. The disposition — **carve the member out** (its rows return to `pending`, the epic continues, manifest status `closed-partial`) or **hold the whole run** — is **reserved to the user**, never the lead's: carve-out breaks the one-unit promise."
- **Consumers assessed:** `implement.md`'s member-halt gate references the reservation (untouched).

## [v0.101.0] Transient role + directory persists + two-typed map — protection transfers (census §A row 9; multi-feature D3)
- **Disposition:** superseded — protection transfers to `authoring-epic.transient-role` (floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "The epic's **role is transient** … **The directory persists as readable record — never as a living map layer.** The map stays two-typed (durable capabilities + transient work rows); an epic is not a third type."
- **Consumers assessed:** `mochiko:authoring-feature-map`'s two-typed invariant is the same ruling's other face (P3's member, untouched by this seat).

## [v0.101.0] Remaining body obligations relocated (census §B rows 1 · 2 · 3 · 5b–5e · 6 · 7 · 11 · 12 · 14 · 18 · 21)
- **Disposition:** relocated → `plugins/mochiko/skills/authoring-epic/schema.yaml`, per the map entry above (D3).
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 rows).
- **Content (decisive line per row):** 1 "The commands that consume it … **reference this skill; they never restate it**" · 2 "identity (`EPIC-XXX`, sequential, same family as `FEAT-XXX`)" · 3 "One epic type, two faces (no product/tech split): the **manifest is the product view**, the **spine is the tech view**" · 5b "this skill *names* the seam owner in the spine, the map writes it" · 5c "**Implement run mechanics** — gate shapes, cycle sequencing, and landing steps live in `implement.md`; this skill single-sources only the epic object it consumes." (the "authors the epic object" clause rides the 5a carve-out, per the map entry above) · 5d "worktree isolation vs single pen-holder is `mochiko:patterns-transport-floor`; referenced, never restated" · 5e "parked open threads; not epic scope today" · 6 "**No separate epics index — the directory is the registry; the `/mochiko:feature` desk lists it.**" · 7 the manifest field set (members linked · status enum · why-together line at the desk mint) · 11 "**Ordering** — shared-foundation first, then in-epic dependency order" · 12 "design deltas stay in each member's `.mochiko/features/FEAT-XXX/` dir, linked from the manifest" · 14 "The **transport floor's composition steer** … governs every epic shared-write surface … disclosed at run open" · 18 "specify **proposes**, never mints" · 21 "the `[EPIC-XXX]` markers vanish with the folded rows, the manifest is **stamped `delivered` + dated**, and the **directory stays in place as record** — no move, no trail file"
- **Consumers assessed:** none restate these rules; the router's `authoring-epic` row describes the skill generically and stays true.

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
