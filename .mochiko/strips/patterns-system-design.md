# Strip notes — `skills/patterns-system-design/`

Entry formats: `strips/README.md`. First strip of this skill. Wave context: [v0.64.0] entry —
guardrails-vs-detail **Wave 2** (editorial extension of the D4 cut line to the untested
primitives; design: `.mochiko/brainstorms/validator-scope-and-verbosity/record.md`; build plan:
`.mochiko/benchmarks/guardrails-vs-detail/report/build-plan.md`, Wave 2 sketch). This skill is an
**M1 near-cap description** — its description was 1,514 chars, within ~22 of the 1,536 delivery
cap; the slim form is the highest-value part of this cut.

## [v0.81.0] Transformed to serve store deltas — the no-delta protected line RELOCATED; the per-feature `architecture.md` artifact superseded

Wave context: the product-architecture-store Stage-1 build
(`.mochiko/brainstorms/product-architecture-schema/record.md`, **D3 / D7 / D10 / D12**;
`DECISIONS.md` 2026-08-19). D3 kills the per-feature `architecture.md` artifact; D7 keeps this
skill **transformed** — altitude + diagram craft now serving the delta drafted against the product
architecture store. Three entries below: **A** relocates the protected no-delta line, **B**
supersedes everything keyed to the dead artifact, **C** re-points the diagram-conventions
reference.

### A. [v0.81.0] Supersession by RELOCATION — the no-delta protected line

- **Disposition:** relocated → the transformed `## The no-delta judgment` section of the same
  skill. **The protected sentence survives word-for-word** (bolded in its new home — see the
  emphasis note below); its home and its artifact framing move. One further sentence in the same
  span was **dropped rather than relocated** — corrected and accounted for below (A2).
- **Tier failed:** n/a — supersession by ruling (record D3 + D10 review-fold **S13**: "a plan run
  judging the feature non-structural records that judgment as one line in the plan package, shown
  at gates, never made silently (carries `patterns-system-design`'s protected line forward)").
- **Content (verbatim, the whole superseded span — former `SKILL.md:83-88`):**

  ```
  ## The no-delta form

  Every feature produces the artifact, including one that changes nothing structurally. A **no-delta**
  feature presents the **unchanged** container diagram (reseeded from the baseline) plus a **one-line
  claim** — "this feature changes nothing structurally" — for approval. The no-delta judgment is always
  shown, never made silently. On a large system, the scale bound still governs: show the neighborhood
  the feature touches, link the rest.
  ```

- **Kept deliberately (the protected sentence, in its new home):** "The no-delta judgment is always
  shown, never made silently." The one-line claim survives with its exact wording ("this feature
  changes nothing structurally"), now recorded in the plan package and shown at the gate rather than
  presented on a reseeded artifact.
  - **Emphasis added (A3, declared at the V3 audit).** The sentence is carried **word-for-word**,
    but it is **bolded in its new home** — shipped as `**The no-delta judgment is always shown,
    never made silently.**` where the original was unemphasized prose. No word added, removed, or
    reordered; the change is markdown emphasis only. Declared because this entry's disposition line
    originally claimed the sentence "survives verbatim" without qualification, and a protected line
    deserves an exact account of every difference, formatting included. The bolding is deliberate:
    in the transformed section the sentence is the section's operative rule rather than a clause
    inside a longer artifact description, and the surrounding text no longer sets it off.
- **Superseded within the same span (D3):** "Every feature produces the artifact" — there is no
  per-feature artifact to produce; and "presents the **unchanged** container diagram (reseeded from
  the baseline)" — there is no baseline to reseed. The store's standing spine already carries the
  unchanged picture, so the replacement text rules the opposite: "No diagram is drawn and the store
  is untouched … redrawing it would assert a delta where none exists."
- **The scale-bound sentence was DROPPED, not relocated (A2 correction, declared at the V3 audit).**
  This entry originally stated that the sentence "survives, re-worded off the dead artifact and
  relocated into `## Scope the diagram to the delta neighborhood`". **That claim was wrong and is
  corrected here.** What actually happened:
  - The sentence "On a large system, the scale bound still governs: show the neighborhood the
    feature touches, link the rest." was **deleted with the rest of the `## The no-delta form`
    section**. Nothing was moved.
  - `## Scope the diagram to the delta neighborhood` is a **pre-existing section that already
    carried the same rule**; it was **reworded in place** (span declared below), not created or
    extended to receive relocated content.
  - **The obligation is nonetheless intact**, which is why the drop is a deletion of a *duplicate*
    rather than a loss: the surviving Scope section states the neighborhood-inlined /
    wider-map-linked rule as its whole subject, and the transformed no-delta section no longer needs
    a scale caveat because it now draws **no diagram at all** — the case the caveat governed
    ("a no-delta feature on a large system shows the touched neighborhood") cannot arise.
- **Undeclared reword, now declared (A2, second limb) — `## Scope the diagram to the delta
  neighborhood`, one pre-existing span reworded in place:**
  - **Before:** "the full system view is **linked, never inlined**; the artifact shows the
    neighborhood and points to the wider map."
  - **After:** "the full system view is **linked, never inlined**; the delta shows the neighborhood
    and points at the spine for the wider map."
  - **Ground:** D3 ("the artifact" no longer exists) and D4 (the spine is the wider map; repo-root
    `ARCHITECTURE.md` is a derived index, never a topology source). Two words of subject and object
    change; the rule, the threshold, and the emphasis are untouched. **The protected v0.67.0 A2
    content in the same section — the `~12 rendered nodes` default and the
    override-must-assert-altitude rule — is not touched by this reword** and remains byte-for-byte
    as shipped.
- **Consumers assessed:** the no-delta obligation is graded by `review-plan-artifacts`
  (ARTIFACT-CHECKLISTS Architecture section) and disclosed by the retired
  `templates/architect-report-template.md` (`delta: ["no structural change"]`, deleted this wave —
  `.mochiko/strips/architect-report-template.md`). The checklist re-key is P2/P4's wave ripple; the
  obligation itself is preserved here and must be findable in both this skill and the plan
  package's one-line claim.

### B. [v0.81.0] Supersession by ruling — the per-feature `architecture.md` artifact and everything keyed to it

- **Disposition:** superseded → the transformed skill (altitude + diagram craft serving store
  deltas); the artifact's role passes to the store (`mochiko:authoring-architecture-store`) and the
  delta drafted in the plan package.
- **Tier failed:** n/a — supersession by ruling (record **D3** one store / the per-feature artifact
  dies · **D7** crew · **D10** plan-time delta contract · **D12** absorb + structural-origin D-XXX
  die into store deltas; `DECISIONS.md` 2026-08-19).
- **Content (verbatim, the nine superseded spans):**

  1. Overview, former `:13-14`:
     ```
     The artifact is `architecture.md` in the feature's spec dir — a **delta view** (current state + proposed
     target, the structural change made visible) that the detailed design artifacts then conform to. It
     is the design-time architecture surface, authored **before** `data-model.md` and `contracts/`.
     ```
     *(Also closes the F5 path inconsistency's third limb — "the feature's spec dir" was the vague
     third answer against `plan.md`'s `.mochiko/features/FEAT-XXX/` and the architect template's
     `.mochiko/specs/<feature>/`. The dead artifact has no home to disagree about.)*

  2. The whole `## Seed the baseline before you design on it` section, former `:25-30`:
     ```
     ## Seed the baseline before you design on it

     The delta's current-state half must be real, not assumed:

     1. **`ARCHITECTURE.md` exists** → seed the current state from it.
     2. **Absent (the bootstrap)** → reconstruct the baseline topology from the code (and `codebase-analysis.md` when present), mark it **reconstructed** with a confidence note, and treat it as the seed only once it is **confirmed**. Never design a delta on an unconfirmed baseline — a wrong baseline makes the whole delta a fiction. The confirmed baseline is what lands as the initial `ARCHITECTURE.md` downstream.
     3. **Greenfield** → the current state is empty; the target *is* the whole picture (the delta degenerates cleanly, no bootstrap needed).
     ```
     Replaced by `## The baseline is the store`. **Reconstruction leaves this skill entirely** — per
     D16 it is the `/mochiko:architecture` desk's reconstruct-and-confirm work, and per D4 the
     confirmed baseline lands in the store, not as "the initial `ARCHITECTURE.md`".

  3. `## The four pieces` opener, former `:34`: "`architecture.md` carries four pieces." → "A delta
     carries up to four pieces." The **four-piece artifact structure** dies; the four **diagram
     types** survive as the delta's pieces.

  4. §3 register opener, former `:57`: "A **container-level register** — one line per
     deployable/runnable piece, mirroring `ARCHITECTURE.md`'s form" → a register **of the change**
     (one line per piece the delta adds, modifies, or removes). The mirrored prose doc is dead
     (`authoring-architecture` retired this wave).

  5. §3 delta-summary clause, former `:62-64`:
     ```
     a **delta
     summary** (prose) linking each structural change to the **D-XXX** row that ruled it — link, never
     restate the decision.
     ```
     Superseded by **D12**: structural-origin `D-XXX` rows die into store deltas ("the store ruling
     is the decision record"), so the link target no longer exists for structural changes. **The
     discipline is kept and the target re-pointed** (lead ruling at build, P3 question 1): each
     structural change links "**the ruling that made it**: the store element the change writes — the
     spine element (`SPN-XXX`) it adds, moves, or retires, or the concern row (`AX-XXX`) it answers
     — and, where an analysis-origin `D-XXX` row governs the fork, that row. **Link, never restate
     the the decision.**" Analysis-origin `D-XXX` survives per D12 (`constraints-and-decisions.md` survives
     reduced). Id grammar is `authoring-architecture-store`'s to own; the `SPN-XXX` / `AX-XXX` forms
     above are reconciled against P1's landed `plugins/mochiko/schemas/architecture-store.yaml`
     (`:55` spine required core `SPN-XXX`, `:70` concern required core `AX-XXX`) per the lead's
     build ruling 1.

  6. The whole `## architecture.md Structure` section, former `:90-137` — the canonical markdown
     envelope. Verbatim:
     ```
     ## architecture.md Structure

     Follows the deliverable envelope in [`artifact-format.md`](../../templates/artifact-format.md) —
     dense, one read, statement-carries-the-content. Density is not a gap; a gap is a missing component,
     an unlabelled arrow, or a qualifying flow with no sequence diagram.

     `````markdown
     # Architecture: {feature_id}

     > Container-level topology and the current→target delta. Sign-off surface for the shape; detail
     > (entities, endpoints) is drawn downstream against the approved target.

     ## Baseline  *(current state)*

     Seeded from `ARCHITECTURE.md` · or **reconstructed** from code (confidence: {high/medium/low}) · or
     greenfield (empty). {one line stating which}

     ## Container Diagram  *(target; delta marked)*

     ````mermaid
     flowchart TB
       %% subgraph boundaries, technology in node labels, protocol+purpose on arrows,
       %% classDef for new/modified, strike for removed — see DIAGRAM-CONVENTIONS.md
     ````

     ## Components  *(container-level register)*

     | Component | Responsibility | Boundary | Status |
     |-----------|----------------|----------|--------|
     | Profile API | serves + edits user profiles | owns Profile store | existing |
     | Avatar Worker | resizes uploaded avatars | reads queue, writes blob store | new |

     ### Delta summary

     - **Avatar Worker (new)** — decouples image processing from the request path. Ruled in **D-004**.
     - {each structural change → its D-XXX row; link, never restate}

     ## Key Flows

     ````mermaid
     sequenceDiagram
       %% one per qualifying flow (≥2 components, non-trivial ordering/failure) — see DIAGRAM-CONVENTIONS.md
     ````

     ## Deployment  *(conditional — only when IP-XXX rows exist)*

     {runtime/infra placement, or one line: "no deployment change — no IP-XXX rows"}
     `````
     ```
     *(Fence widths in the block above are widened by one backtick for nesting; the shipped original
     used ```` for the outer markdown fence and ``` for the two inner mermaid fences.)* The **section
     heading and the whole template** die with the artifact. **KEPT:** the section's opening
     paragraph — the `artifact-format.md` envelope pointer and the "Density is not a gap; a gap is a
     missing component, an unlabelled arrow, or a qualifying flow with no sequence diagram" rule —
     survives verbatim as the new `## Density` section, re-keyed to the delta.

  7. Quality-Checklist rows, former `:143` and `:153`:
     ```
     - [ ] The current-state baseline is seeded from `ARCHITECTURE.md`, or reconstructed **and confirmed** (confidence noted), or greenfield-empty
     - [ ] A no-delta feature still shows the reseeded diagram + the one-line no-structural-change claim
     ```
     Replaced by, respectively: "The current state is read from the store's spine — not reconstructed
     here, not assumed" and "A run that changes nothing structurally records the one-line no-delta
     claim rather than drawing a diagram". Every other checklist row survives; two are re-worded off
     the artifact ("delta register" for "component table"), the rest byte-for-byte.

  8. `## When NOT to Use` bullet 1, former `:18`:
     ```
     - **Repo-level `ARCHITECTURE.md`** — that living, current-state operating doc is `mochiko:authoring-architecture`, folded post-hoc at a landing. This skill is the design-time, feature-scope, delta artifact — distinct file, distinct moment.
     ```
     Replaced by two bullets, both required by D7's crew: **the store itself** (grammar, element
     lifecycle, statuses, graduation, health view, derived root index →
     `mochiko:authoring-architecture-store`) and **what stance a concern row takes** (shelf
     dimensions, defaults, upgrade triggers → `mochiko:patterns-architecture-shelves`, dealt at the
     `/mochiko:architecture` desk). The second is a **new** boundary the shelf skill's birth creates,
     not a re-point.

  9. The `description:` value, whole rewrite. Old value verbatim:
     > This skill MUST be invoked when designing a feature's architecture at design time — authoring the per-feature `architecture.md`: a C4-container-level delta diagram, sequence diagrams for qualifying flows, a D-XXX-linked component table, and a conditional deployment view. SHOULD also invoke on 'architecture design', 'container diagram', 'C4', 'system topology', or 'architecture delta'. Seeds the current-state baseline before drafting the delta. Distinct from `authoring-architecture` (repo ARCHITECTURE.md); does not grade its own output.

- **Kept deliberately:**
  - **Both v0.67.0 protected supersessions, re-asserted intact.** **A1** — the "Altitude check —
    every row is a container, not a C4-level-3 construct" paragraph survives **verbatim and in its
    existing home** inside §3 (deliberately not relocated to a section of its own: the protected
    paragraph's home is unchanged, which is the lower-risk edit). **A2** — the ~12-node default and
    the override-must-assert-altitude rule survive verbatim in `## Scope the diagram to the delta
    neighborhood`.
  - The **v0.64.0 RETURNED description clause** (user-ruled 2026-08-11, fire-rate-probe evidence).
    Its former form "Seeds the current-state baseline before drafting the delta." is **preserved
    re-pointed, not dropped**: "Reads the store's spine for the current state; never re-derives it."
    The clause exists to carry the current-state-then-delta disambiguation cue that the probe found
    missing — that cue is intact, and the sibling-distinction clause it pairs with now names
    `authoring-architecture-store`.
  - The **qualifying-flow definition** (≥2 components, non-trivial ordering or failure semantics;
    user journey *or* system flow; P1 the floor never the cap), the **C4-as-method /
    flowchart-as-carrier** rule, the **delta-visibility** contract (`classDef` new/modified, removed
    struck), the **conditional deployment view** with its `IP-XXX` trigger unchanged (lead ruling at
    build, P3 question 3 — only the four-piece *artifact framing* is superseded, never the view),
    the **register↔diagram bijection**, the **never-design-on-an-unconfirmed-baseline** floor
    (verbatim, re-homed in `## The baseline is the store`), the **before-`data-model.md`/`contracts/`
    ordering**, the **`artifact-format.md` density rule**, the **DIAGRAM-CONVENTIONS.md** pointer,
    and **all eight Common-Mistakes rows** (two re-worded off the artifact: "Topology in the wrong
    artifact" → "Topology in the wrong place"; "Restated decisions" now links the ruling).
- **Additive (rides the D7 decision row, no supersession):** one Common-Mistakes row, "Redrawing the
  unchanged" (the whole spine reseeded into the delta ❌ / the delta shows what changes, the spine
  already holds the rest ✅) — the failure mode D3's single-store model newly makes possible.
- **Char budget:** body **10,082 → 9,304** against the 11,047 budget (**net −778**, 1,743 chars of
  headroom — no overage). Description **541 → 649** against the 677 budget (28 chars of headroom;
  hard cap 1,536). Both measured with the canonical snippet in
  `.mochiko/memory/primitive-cost-budgets.md`.
- **Consumers assessed** (live tree; `.claude/worktrees/` copies excluded): `commands/plan.md:148`
  ("`architecture.md`'s structure and scope bound are `mochiko:patterns-system-design`'s") — the
  structure half of that sentence no longer resolves and is **P2's** re-key, in its approved plan ·
  `agents/principal-architect.md` (mounts it; re-pointed this wave, same producer) ·
  `skills/authoring-architecture/SKILL.md` (**retired this wave** — pointer dies with the file) ·
  `skills/authoring-technical-requirements/SKILL.md` + `references/ARTIFACT-TEMPLATES.md` (D-XXX
  `Origin: structural` cites this skill — **P4**, and materially affected by span 5's D12 re-point) ·
  `skills/review-feasibility/SKILL.md` + `references/FEASIBILITY-LENS.md` (architecture pass — P4) ·
  `skills/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` L100-133 (the graded Architecture
  checklist — P4; the four-piece grammar it grades is exactly what span 6 supersedes) ·
  `skills/patterns-code-minimalism/SKILL.md:27` (names this skill's judgment — P4) · router
  `skills/mochiko/SKILL.md:79` (P4). **No section anchor is linked by any consumer** — every
  reference is by skill name or by prose, so no removed heading breaks a link. Flagged and not
  touched by P3 per the lead's ruling 5.

### C. [v0.81.0] Supersession by ruling — `references/DIAGRAM-CONVENTIONS.md` re-pointed at the store

- **Disposition:** superseded → the same four spans, re-pointed at the delta and the store's
  topology spine. (`references/` files are budget-exempt; the strip entry is owed regardless.)
- **Tier failed:** n/a — supersession by ruling (record **D3 / D4** — the spine is the wider map the
  neighborhood links out to; `ARCHITECTURE.md` becomes a derived index, never a topology source).
- **Content (verbatim, four superseded spans — the fourth added at the V4 fix round):**
  1. `:3` — "Copy-ready mermaid patterns for the `architecture.md` pieces." → "Copy-ready mermaid
     patterns for the architecture delta's pieces."
  2. `:96` — "Author **only when the feature carries `IP-XXX` provisioning rows**" → "Author **only
     when the change carries `IP-XXX` provisioning rows**". Trigger semantics unchanged.
  3. `:122-124`:
     ```
     neighborhood; add one line linking the wider map
     (`ARCHITECTURE.md`, or the prior feature's architecture) for everything unchanged. A no-delta feature
     on a large system shows the touched neighborhood, not the whole estate.
     ```
     → "neighborhood; add one line linking the wider map — the store's topology spine — for
     everything unchanged. On a large system the spine is the estate; the delta is only ever the
     neighborhood the change touches." Both dead link targets go: repo `ARCHITECTURE.md` is now a
     derived index (D4) and "the prior feature's architecture" is the artifact D3 kills.
  4. `:89` (**added at the V4 fix round, advisory A5**) — "Participants are the same components
     named in the container diagram and **the component table** — keep the names identical across
     all three pieces." → "…named in the container diagram and **the delta register** — keep the
     names identical across all three pieces." The register was renamed from "component table" to
     "delta register" in the transformed `SKILL.md` (entry B span 4); this reference kept the old
     name, leaving the cross-piece naming rule pointing at a piece that no longer exists under that
     name. Two words; the identical-participant-names rule itself is untouched. **Declared as its
     own span deliberately** — the V3 audit's A2 finding was an undeclared in-place reword in this
     same wave, and a fourth reword to this file shipped silently would repeat exactly that defect.
- **Kept deliberately:** everything else in the reference, byte-for-byte — the copy-ready container
  delta diagram, the four-row delta-styling table (new / modified / removed / existing with their
  exact `classDef` values), the node-shape hints, the rendering rule, the sequence-diagram pattern
  with its `alt`/`opt`/`Note` guidance and the identical-participant-names rule, the deployment-view
  pattern, and the ~12-node scale bound itself.
- **Consumers assessed:** the reference is reachable only through this skill's body pointer, which
  is intact. No command, skill, or template links it directly (`grep -rn "DIAGRAM-CONVENTIONS"
  plugins/` returns exactly one hit: this skill's own body pointer at `SKILL.md:42`).

## [v0.67.0] Altitude bar hardened — container-level check made explicit; node-count override must assert altitude
- **Disposition:** superseded → the reworded lines in-place (the altitude check + the override rule); additive companions (checklist bullet, anti-pattern row, ladder pointer) ride the decision row.
- **Tier failed:** n/a — supersession by ruling (record `.mochiko/brainstorms/architect-role-pushback-and-abstraction/record.md` build-surface **item 4**; `DECISIONS.md` 2026-08-13 row L13 — the FEAT-002/F8 drift class: application-layer/domain/port-trait rows inside one process are C4-level-3, not containers).
- **Superseded lines (A1 + A2):**
  - **A1** — §3 Component table parenthetical `("Component" here is the container-level register sense, not C4-level-3.)` → promoted to an explicit **"Altitude check — every row is a container, not a C4-level-3 construct"** paragraph (each row separately deployable/runnable; an application or domain layer, a module, a port or a trait inside one process is not a container and earns no row — it belongs in the detailed design).
  - **A2** — §Scope the diagram, `**default: ~12 rendered nodes** (boxes), overridable per project` → the override now **must assert the altitude** (every extra node a genuine container per the altitude check), not merely cite a larger count; a high count that is really sub-container detail is drift, not a legitimate override. (The ~12-node default itself is unchanged.)
- **Additive (ride the decision row, no supersession):** a Quality-Checklist bullet ("Every register row is a deployable/runnable **container** — no application/domain layer, module, port, or trait … inside a single process"); a Common-Mistakes row ("Sub-container register rows" — `Preflight domain`, `Engine port trait`); one §When-NOT-to-Use pointer to `mochiko:patterns-plan-minimalism` (the design-time weight ladder governs the *amount* of structure; this skill governs *altitude*; no rung restated).
- **Body:** 8,837 → 10,082 chars (budget 11,047, OK). Description unchanged (541 chars, budget 677).
- **Kept deliberately:** the container-level register contract, the ~12-node default bound, the four pieces, the seed-the-baseline rules, the no-delta form, the canonical template, the DIAGRAM-CONVENTIONS pointer — untouched. The v0.64.0 RETURNED description clause and the v0.64.0 guardrails keep-set below stand.
- **Consumers assessed:** `commands/plan.md`, `agents/principal-architect.md` (now the consuming seat — `system-architect` retired this wave), `skills/authoring-architecture`, `skills/authoring-technical-requirements`, `skills/review-feasibility`, `skills/review-plan-artifacts/references`, `skills/patterns-code-minimalism`, router `skills/mochiko/SKILL.md` — all reference the skill by name; none links a removed section anchor (no section removed; edits are in-place rewords + additions). The four-pieces contract `review-feasibility` / `review-plan-artifacts` grade against is intact and strengthened.

## [v0.64.0] RETURNED: current-state-baseline clause in the description
- **Evidence:** the Wave 2 M1 fire-rate probe (14-scenario blind routing spot-check, lead
  dispatch 2026-08-11) hit S1 with a hesitation pulling toward `authoring-architecture` —
  the slim description had dropped the "seeds the current-state baseline" cue that carried
  the current-state-then-delta disambiguation. Minimal restore, one clause appended to the
  SHOULD sentence: "Seeds the current-state baseline before drafting the delta." Description
  481 → 541 chars (cap 1,536). User-ruled 2026-08-11.

## [v0.64.0] Guardrails cut — When-to-Use removed, four-pieces contract kept; M1 slim description (1,514 → 481)

- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line — When-to-Use bullets restate the description; M1 description slim off the near-cap value).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed — section-level inventory; body 9,280 → 8,837 chars, −443, −5%; description 1,514 → 481 chars):**
  - **Removed whole:** `## When to Use` — the four-bullet list ("Authoring `architecture.md` — the container-level topology, interaction flows, and current→target delta for a feature" · "Deciding component boundaries, interaction style (sync/async, request/response vs event), or where a responsibility lives" · "Producing the visual the reader signs off on before the data model and API contracts are drawn" · "Reconstructing a current-state baseline from code when no `ARCHITECTURE.md` exists"). Each bullet restates the description + Overview + the "Seed the baseline" section; no obligation lost.
  - **Description slimmed (M1):** the near-cap 1,514-char description dropped its inline diagram-convention detail (the mermaid-carrier/subgraph/protocol+purpose parenthetical, the qualifying-flow definition parenthetical, the baseline-seeding and delta-neighborhood-scoping clauses, the "how the components interact" trigger phrase, the "upstream of patterns-entity-modeling / patterns-api-contracts" clause) — all of which survive in the body (`## The four pieces`, `## Seed the baseline before you design on it`, `## Scope the diagram to the delta neighborhood`, `## When NOT to Use`). The slim form keeps the MUST trigger, the four-pieces gist, the top trigger phrases, the `authoring-architecture` sibling distinction, and the does-not-grade clause.
  - Old description verbatim: "This skill MUST be invoked when designing a feature's architecture at design time — authoring the per-feature `architecture.md` that the detailed design then conforms to: a C4-container-level delta diagram (mermaid flowchart carrier — subgraph boundaries, technology-labelled nodes, arrows labelled protocol + purpose, new/modified/removed styled distinctly), sequence diagrams for qualifying flows (any flow crossing ≥2 components with non-trivial ordering or failure semantics — user journey or system flow), a container-level component table (name — responsibility — boundary — status new/modified/existing) with a delta summary linking each structural change to its D-XXX row, and a conditional deployment view keyed to IP-XXX. SHOULD also invoke on \"architecture design\", \"container diagram\", \"C4\", \"system topology\", \"how the components interact\", \"architecture delta\", or an early architecture sign-off before data-model / contracts. Seeds the current-state baseline from `ARCHITECTURE.md` (absent → reconstruct from code, mark reconstructed, confirm before drafting a delta); scopes the diagram to the delta neighborhood (changed components + direct collaborators; past a threshold the full view is linked). Distinct from `authoring-architecture` (the repo-level `ARCHITECTURE.md` operating doc, folded post-hoc at landing) and upstream of `patterns-entity-modeling` / `patterns-api-contracts` (which detail the approved shape). Authors the design-time architecture artifact; does not grade its own output."
  - Verbatim removed text survives in: git history of the SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** `## Overview`, `## When NOT to Use` (the four sibling distinctions), `## Seed the baseline before you design on it` (the three cases + the "never design a delta on an unconfirmed baseline" floor), `## The four pieces` (the container delta diagram / sequence-diagram / component-table+delta-summary / conditional-deployment contract + the qualifying-flow definition + C4-as-method rule + the DIAGRAM-CONVENTIONS.md pointer), `## Scope the diagram to the delta neighborhood`, `## The no-delta form`, `## architecture.md Structure` (the canonical template), `## Quality Checklist`, `## Common Mistakes`.
- **Protected-content reconciliation (MANDATORY):** no prior strip file existed; grep of git history found no `KEPT:` / protected / `DECISIONS.md`-traceable line for this skill. The `## When to Use` list was never a protected survivor. Nothing silently dropped.
- **Consumers assessed:** `commands/plan.md`, `agents/system-architect.md`, `skills/authoring-architecture/SKILL.md`, `skills/authoring-technical-requirements/SKILL.md` (+ `references/ARTIFACT-TEMPLATES.md`), `skills/review-feasibility/SKILL.md` (+ `references/FEASIBILITY-LENS.md`), `skills/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md`, `skills/patterns-code-minimalism/SKILL.md`, router `skills/mochiko/SKILL.md` — all reference the skill by name; none links a removed section anchor. The `architecture.md` four-pieces contract that `review-feasibility` / `review-plan-artifacts` grade against is intact.
