# Architecture Design as a First-Class Primitive — Decision Record

**Status:** accepted (2026-07-30) — pair-reviewed, clearing verdict READY (11 raised → 10 merged survivors → 10/10 resolved via user-adopted batch R1–R10; verify pass CLEAN after one B1 repair + bounded re-verify). Landed: DECISIONS.md rows AD-D1–D9 · BACKLOG "Architecture-primitive build items" · ROADMAP Next row.
**Opened:** 2026-07-30
**Topic:** How architecture design becomes a first-class primitive in the mochiko workflow — architecture alignment integral to planning and implementing, with a visual representation (standard design diagrams) so the user is never blind to the components, their interactions, and the shape of the service.

**Seed (user's words, lightly cleaned):** "I want to brainstorm how architecture design can be a first-class primitive in the mochiko workflow. I would almost like architecture alignment to be integral to planning and implementing. I, as a user, am often blind to what the different components will be, how they will interact, how the service will look. I would like a visual representation using an appropriate standard design diagram to be there."

---

## Fact-checker map

*(checker-authored, pasted verbatim 2026-07-30)*

*Scope: worktree at `.../worktrees/brainstorm-architecture-design`, plugin source `plugins/mochiko/`. In-repo plugin version is **0.29.0** (`plugins/mochiko/.claude-plugin/plugin.json`), one bump ahead of the installed 0.28.0 cache. Every claim below is path-cited.*

### 1. The pipeline

Commands in `plugins/mochiko/commands/`: `brainstorm.md`, `setup.md`, `specify.md`, `slice.md`, `plan.md`, `tasks.md`, `implement.md` (7 total). Design-relevant flow is `specify → (slice) → plan → tasks → implement`, all writing under `.mochiko/specs/<feature>/`.

- **plan** (`commands/plan.md`) — two-phase producer→two-reviewer team loop. **Phase 1 (Analysis)** produces `requirements.md` (FR→TR), `constraints-and-decisions.md` (C-XXX/D-XXX/IP-XXX), `nfrs.md` (NFR-XXX). **Phase 2 (Design)** produces `data-model.md` (entities + sensitivity), `contracts/api.yaml` (OpenAPI + `x-integration`), and conditionally `quickstart.md` (only when a real external-integration surface exists). Assembles `plan.md` at Phase 3 from `templates/plan-template.md`. Producer = `technical-analyst`; feasibility graded once by `principal-architect` (`review-feasibility`), completeness by `devils-advocate` (`review-plan-artifacts`). User is gated at **G5** (accept/amend/reject `plan.md`). No component/system view is produced — see §2.
- **specify** → `spec.md`; **slice** → `slices.md` overlay (null-exit-aware); **tasks** → `task-mapping.md` + `tasks.md`; **implement** → working code + `cycle-report.md` per cycle + verification reports.
- **plan-absorbs-tasks merge**: NOT visible in the command set — `plan.md` and `tasks.md` are still **separate commands**. The commit `7920ccb docs: ratify team-method-vs-command-shape rulings (mesh Layer 2, plan absorbs tasks)` is in the log, but on disk `plan.md`'s next-step still points to `/mochiko:tasks` (`commands/plan.md` Phase 5) and `tasks.md` exists as its own supervisor. So the ruling is recorded but **not yet implemented in the command files** — a decision-vs-artifact skew worth flagging.

### 2. Design-layer skills and agents — is there a component/system view?

**No feature-design-time primitive produces a component-level or system-level view.** The design set is strictly entity-level + endpoint-level + decision-level:

- `patterns-entity-modeling` — the `data-model.md`: entities, attributes, relationships (cardinality/delete), state machines, per-attribute DS-XXX sensitivity. Data model only.
- `patterns-api-contracts` — `contracts/api.yaml`: user-action→REST endpoint, request/response schemas, errors, pagination, `x-integration` boundaries. Endpoint level only.
- `patterns-technical-decisions` — the decision *technique*: ≥2 alternatives, weighted criteria, ADR depth, brownfield-alignment. Fills D-XXX rows; no topology.
- `authoring-technical-requirements` — owns `constraints-and-decisions.md`/`nfrs.md` structure (TR/C/NFR/IP + traceability). Text requirements.
- `review-plan-artifacts` — completeness grade (FR→TR coverage, entity/endpoint coverage, schema-model consistency).
- `review-feasibility` — adversarial cross-artifact buildability (constraint↔decision, NFR↔constraint, NFR↔design). Grades contradiction; produces no view.
- `technical-analyst` agent (`agents/technical-analyst.md`) — persona authoring all of the above; its self-description covers "entity models, API contracts, technology decisions" — **no components/interactions/deployment**.
- `principal-architect` agent (`agents/principal-architect.md`) — governance authoring + feasibility review; despite the name, its scope is constitution + cross-artifact feasibility, NOT system design. **But** `agents/principal-architect.md:57` lists `authoring-architecture` as one of its skills ("the living system view (`ARCHITECTURE.md`)") — see §4.

The nearest thing to a component/system view is `ARCHITECTURE.md` via `authoring-architecture` (§4), but that is a repo-level operating doc written **post-hoc at landing**, not a feature-design artifact — and it's prose, not visual.

`plan-template.md` (58 lines) section headers: Summary · Key Decisions · Infrastructure Requirements · Entities · Endpoints · Artifacts · Next Steps. **No Components / System / Deployment section.**

### 3. Diagrams / visuals

Case-insensitive sweep of plugin source + templates + `.mochiko/` for mermaid / C4 / UML / sequence-diagram / component-diagram / deployment / visual:

- **The only real diagrams in the repo are in `README.md`** — 3 mermaid `flowchart` blocks (lines 22, 51, 65): the path-decision flow, the pipeline flow, and the sound-loop shape. These are **docs about mochiko-the-framework**, not anything a workflow produces.
- **The plugin's produced artifacts contain zero mermaid / C4 / UML / rendered diagrams.** The only "diagrams" any skill emits are **ASCII text-diagrams**, both inside `patterns-entity-modeling`:
  - `references/RELATIONSHIP-PATTERNS.md:151` — a "Text-Based Diagram Format" using `User ──1:N──▶ Task` arrow notation with a symbol table.
  - `references/STATE-MACHINES.md:37` — an ASCII state diagram (`[draft] ──start──▶ [active]…`).
  These are entity/state scope, hand-drawn ASCII, no rendering pipeline.
- `templates/tasks-template.md:117` explicitly **rejects** a diagram: "no separate dependency diagram."
- `templates/taskarchitect-report-template.md:18` contains `"C4: C4 split into C4/C5"` — **false positive**: "C4" here = **Cycle 4**, not the C4 architecture model. No C4/UML anywhere in the design sense.
- **Zero-hit finding**: no `mermaid`, `C4` (arch), `UML`, `sequence diagram`, `component diagram`, or `deployment diagram` in any command, agent, design skill, or produced artifact/template. There is **no visual/diagram representation of architecture anywhere in the produced-artifact surface.**

### 4. `ARCHITECTURE.md` and `authoring-architecture`

Both are **brand new — added in the single most recent commit `edf0a7e "Add DECISIONS & redesign operating-docs (KM module)"`, the same commit that bumped 0.28.0→0.29.0.** So they postdate the installed 0.28.0 cache and are the freshest surface in the repo.

- **What `ARCHITECTURE.md` is** (`templates/constitution-modules/knowledge-management.md:46`): a **core KM operating doc at repo root** — "the living system view — components, boundaries, data flow; decisions record *changes*, this records the *resulting system*." Writer moment = "plan/implement landings on structural change." Carrier = `mochiko:authoring-architecture`. It's one of the KM core docs (alongside `BACKLOG`/`ROADMAP`/`DECISIONS`/`GLOSSARY`), adopted or declined **whole** with the KM module.
- **Who writes it / when** (`skills/authoring-architecture/SKILL.md`): authored/updated **only at a plan or implement landing whose work changed components, boundaries, data flow, or cross-component contracts.** "Internal refactors, cosmetic moves, and feature work inside an existing component do not fire it." "No structural change → no update." Dispatched as a **disposable Finalize dispatch** by a fresh `principal-architect` seat (see `commands/plan.md` Phase 5 KM-landing and `commands/implement.md` Phase 4 KM-landing — "structural change → `ARCHITECTURE.md` via a fresh `principal-architect` dispatch, never the feasibility seat").
- **Scope/output**: present-tense, current-state-only prose map — Components (name — responsibility — boundary), Data flow, External integrations. Explicitly **not** rationale (links to `DECISIONS.md`), **not** history, **not** feature-scope design (data-model/contracts stay in specs). "A component earns a line, not a chapter." Quality checks demand every named component exist in code. **No diagram — it's a prose/line map.** The skill dir has only `SKILL.md` (no references, no template).
- **Does the mochiko repo itself have a top-level `ARCHITECTURE.md`?** **No** — top level has `BACKLOG.md`, `CLAUDE.md`, `DECISIONS.md`, `README.md`, `ROADMAP.md`, `plugins/`. So the framework defines the doc but hasn't scaffolded one for itself.

### 5. Implement-time conformance / drift

**No architecture-conformance or drift check exists at implement time.** What implement consumes from plan (`commands/implement.md` Phase 0 step 4): it **reads** `plan.md`, `task-mapping.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `requirements.md` as the producer's **design inputs** — the `staff-engineer` builds from them. Verification (`qa-engineer` via `testing-end-user`) is against **real infrastructure** — executing `**TEST:**` tasks + quality gates (lint/build/test exit codes) + capturing evidence. The done-condition (Phase 5 Contract) checks "traceability to requirements holds" and "aligns with the project's governance (CLAUDE.md region + rules files)" — i.e. **requirement traceability and governance alignment, NOT design-artifact conformance.** Nothing diffs the built code against `data-model.md` entities or `contracts/api.yaml` endpoints; nothing detects design drift. The one governance-side drift mechanism is `.claude/rules/mochiko/` path-scoped rules firing when code-touching seats read files (e.g. layer-import rules), which is constitution-level, not feature-design-level. `commands/setup.md:216` even lists "drift detection between invocations" as **explicitly out of scope**.

### 6. Constitution / architecture-as-governance overlap

"Architecture" already lives as a **governance** concept, distinct from feature design:

- `templates/constitution-modules/layer-rules.md` — attached when the setup synthesis keeps a **layered-architecture card (e.g. BE-HEX)** or mints such an intent. Carries a Project-Structure tree (LAYER_1/2/3 dirs), a **Layer Import Rules** table (MAY / MUST NOT import), and the domain-dependency registry. This is the closest thing to a codified *structural* architecture — but it's **layer-import governance**, enforced by import-linter/CI, not a component view or a per-feature artifact.
- `analysis-codebase` (`skills/analysis-codebase/SKILL.md`) — brownfield **architecture extraction**: detects the architecture pattern with evidence ("Architecture pattern identified with evidence") into `codebase-analysis.md`. `templates/codebase-analysis-template.md:43` has an "Architecture Pattern" table (pattern | evidence) — a **label + evidence**, not a diagram or component map.
- `principal-architect` — owns constitution authoring/amendment + brownfield codification + cross-artifact feasibility, and (new at 0.29.0) is the `authoring-architecture` dispatch persona. Governance judgment, not system-design authoring.

### 7. Version skew

In-repo plugin = **0.29.0** (`plugins/mochiko/.claude-plugin/plugin.json`); installed cache = 0.28.0. Design-relevant primitives that **postdate 0.28.0** (all in commit `edf0a7e`, the 0.29.0 bump):
- **`skills/authoring-architecture/`** — the entire skill (added; only appears at 0.29.0).
- **The `ARCHITECTURE.md` role** in `templates/constitution-modules/knowledge-management.md` (the whole KM module was redesigned in this commit).
- **`DECISIONS.md`** as an operating doc + the KM landing-ritual/invariants that plan & implement now invoke at Finalize (the `ARCHITECTURE.md`-via-`principal-architect` dispatch lines in `commands/plan.md` Phase 5 and `commands/implement.md` Phase 4).

So: anyone testing against the installed 0.28.0 plugin will **not** see `authoring-architecture` or `ARCHITECTURE.md` at all — they exist only in this worktree/repo.

### Facts that cut against the premise

- **Something architecture-shaped already exists, and it's ~5 days old.** `ARCHITECTURE.md` (a repo-root "living system view — components, boundaries, data flow, external integrations") + its authoring skill `authoring-architecture` were added at 0.29.0 in the most recent commit. The premise "architecture design is not yet a first-class primitive" is **partly already answered** — a system-view doc, a writer moment (plan/implement structural landing), and a carrier skill all exist.
- **However, three gaps keep the premise alive**, all file-grounded: (a) `ARCHITECTURE.md` is **post-hoc and repo-level** — written at *landing* to record the *resulting* system, not a *design-time* artifact the pipeline produces or reviews before building; (b) it's **prose/line-map only — zero visual/diagram** (the only diagrams in the whole repo are 3 mermaid blocks in `README.md`, none produced by any workflow); (c) it's **whole-system, not feature-scope** — no per-feature component/interaction view exists between `plan`'s entity+endpoint artifacts and the code.
- **The `principal-architect` name overpromises.** Despite "architect," the agent owns governance + feasibility, and now the `ARCHITECTURE.md` scribe dispatch — it does **not** do feature-level system design. So a reader assuming an architecture-design owner already exists would be mistaken about scope.
- **No drift/conformance loop closes the design→code gap** (§5): the design artifacts are consumed as inputs but never checked against the built code.
- **A recorded-but-unbuilt "plan absorbs tasks" ruling** (§1) means the pipeline shape is mid-flux; any architecture-primitive design should account for `plan`/`tasks` possibly merging.

---

## Decisions

### D1 — The primitive is design-time with a pre-build sign-off, not post-hoc documentation · `Confident`

**Statement:** The problem locus is confirmed as (a) plan-time invisibility — the user cannot picture the system from plan's existing artifacts (`data-model.md`, `api.yaml`, constraints) — and (b) the absence of an architecture *sign-off* before build. The primitive must therefore produce a comprehensible architecture view at design time and put the user's explicit approval on it **before** implementation consumes it.

**Rationale (user's words):** "I couldn't picture the system; also, the architecture signoff is a really important point as it decides how the application will be built and cost of getting it wrong cascades." This rules out the post-hoc-only answer (rendering diagrams from `ARCHITECTURE.md` at landing) as sufficient — the map shows `ARCHITECTURE.md` records the *resulting* system after the fact, which cannot carry a pre-build sign-off.

### D2 — Architecture is the first design artifact inside `/mochiko:plan`, with its own early sign-off · `Confident`

**Statement:** Architecture design + sign-off live inside `/mochiko:plan` as the **first** artifact of the design work: components, interactions, deployment shape (+ diagram) are authored *before* `data-model.md` and `contracts/api.yaml`; the user approves the architecture at its own early human gate; the detailed design artifacts are then derived from — and must conform to — the approved architecture. No new command.

**Rationale:** Puts the sign-off at the earliest cheap moment (before detailed design, not just before code), directly answering D1's cascade concern — nothing downstream is authored against an unapproved shape. Keeps the command surface at its ruled size (the 7→6 plan-absorbs-tasks merge is ruled but unbuilt; a new `/mochiko:architect` command would cut against it). Gives `review-feasibility` a topology to grade NFRs/constraints against, which it currently lacks. Alternatives rejected: bundling into the existing G5 package (cascade survives inside plan — detailed artifacts authored concurrently with an unapproved architecture); a separate stage command (surface growth, every feature pays it). User chose the recommendation directly.

### D3 — Delta model: the artifact shows current + proposed target state; landing folds built reality into `ARCHITECTURE.md` · `Confident`

**Statement:** Plan's architecture artifact is a **delta view**: current system state (seeded from `ARCHITECTURE.md` when it exists, from the codebase / `codebase-analysis.md` when it doesn't) *plus* the proposed target state, with the structural change highlighted. Sign-off approves the **target**. At landing, the existing `authoring-architecture` writer moment folds the *built* reality into `ARCHITECTURE.md` — plan proposes, implement builds, landing records. Greenfield degenerates cleanly (empty current state; target = the whole picture). The artifact stays under `.mochiko/specs/<feature>/`, never in the operating doc.

**Rationale:** Only the delta shape answers "how will the service look" for brownfield features — a standalone feature fragment reproduces the original blindness; editing `ARCHITECTURE.md` directly would inject unbuilt proposals into a doc whose just-ruled contract is present-tense/current-state-only (and an abandoned plan would leave a lie in the living view). Composes with v0.29.0's landing step instead of fighting it: the post-hoc scribe moment becomes the close of an approved loop (proposed target vs. built reality). User chose the recommendation directly.

**Review folds (user-adopted batch, 2026-07-30):** **R6a (S6):** when no `ARCHITECTURE.md` exists, the run opens with a **current-state bootstrap** — the architect reconstructs the baseline topology from code, marks it *reconstructed* (confidence noted), and the user confirms the **baseline** before any delta is designed on it; the confirmed baseline is the seed and lands as the initial `ARCHITECTURE.md` at landing. **R6b (S6):** scale bound — the container diagram scopes to the **delta neighborhood** (changed components + their direct collaborators); past a size threshold (exact number at build) the full system view is linked, never inlined.

### D4 — Artifact content: C4-container delta diagram + key-flow sequence diagrams + component table + conditional deployment view · `Confident`

**Statement:** The architecture artifact carries four pieces, all kept by user arbitration of the recommended set:
1. **Container diagram (C4 level 2) as the sign-off surface** — one mermaid diagram of the target state (services, workers, stores, queues, external systems; arrows labeled with protocol + purpose), with the delta visually marked (new/changed styled distinctly, removed struck). C4-as-method, mermaid-flowchart-as-carrier (mermaid's dedicated C4 syntax is experimental; standard flowchart syntax with subgraph boundaries + technology labels renders reliably).
2. **Sequence diagrams for key flows** — one mermaid `sequenceDiagram` per P1 user journey crossing ≥2 components; capped at P1 journeys.
3. **Component table + delta summary (prose)** — `name — responsibility — boundary — status (new/modified/existing)` per component, mirroring `ARCHITECTURE.md`'s one-line register; delta summary links each structural change to its D-XXX row, never restating it.
4. **Deployment view, conditional** — only when the feature changes deployment reality; trigger tied to IP-XXX rows existing. (Trigger crispness flagged as a design-detail to nail at build.)

**Excluded deliberately:** C4 level 3 (per-feature overkill; sign-off lives at container level), ER diagrams (`data-model.md` owns entities, downstream), class diagrams (implementation detail).

**Rationale:** The three blind spots (what pieces, how they talk, how the service looks) map directly onto C4's container level; sequence diagrams carry the interaction/failure-ordering errors a topology can't show; the table gives reviewers a greppable, traceable surface. Delta marking confirmed as matching how the user wants to read a proposed change.

**Streak note:** D2–D4 are three consecutive direct adoptions of the lead's recommendation. Flagged to the user per the questioning discipline; the next fork (D5) was posed with steelmanned options and **no recommendation**.

**Review folds (user-adopted batch, 2026-07-30):** **R4 (S4):** the sequence-diagram trigger in D4.2 is re-keyed from "P1 user journeys" (a story-priority cap misaligned with ordering complexity — the counterpart-conceded payment-flow scenario) to **flows crossing ≥2 components with non-trivial ordering or failure semantics — user journeys or system flows** — with P1 journeys as the floor, never the cap. **R9 (S9):** disambiguation — "component" in D4.3's table means the **container-level register** (one line per deployable/runnable piece, mirroring `ARCHITECTURE.md`), distinct from the excluded C4-level-3 "component" sense.

### D5 — Always-on: every plan run produces the artifact and stops at the architecture sign-off · `Confident`

**Statement:** The architecture stage is **always-on**. Every `/mochiko:plan` run produces the architecture artifact and stops at its early sign-off — including no-delta features, which present the unchanged container diagram plus a one-line "this feature changes nothing structurally" claim for approval. No structural-change trigger decides whether the stage runs; the no-delta judgment is always shown, never silently made by the producer.

**Rationale:** Ruled by the user at a deliberately recommendation-free fork (post-streak). The steelman the user sided with: blindness was the norm, not the exception; "nothing changes structurally" is itself a claim to see and approve — a producer deciding it unilaterally is exactly the unreviewed structural judgment D1 exists to prevent; re-seeing the service every feature is part of the cure. Accepted cost, eyes-open: one more human gate on every plan run, against the token-reduction epic's grain (keyed and produce-but-conditionally-gate alternatives were steelmanned and declined). Mitigation available at build time: a no-delta sign-off is a cheap gate — the artifact is a reseeded diagram + one line.

### D6 — Implement honors the architecture via briefed input + deviation escalation + landing diff · `Confident`

**Statement:** Three mechanisms close the design→code loop at implement time:
1. **Briefed input** — the approved architecture artifact joins the design inputs the implementing engineer reads at implement start (alongside `plan.md`, `data-model.md`, `contracts/api.yaml`, …).
2. **Deviation escalation** — standing rule: a cycle that finds it *needs* to deviate structurally (new component, changed interaction, moved boundary) **stops and surfaces the deviation at that cycle's verdict**; the user re-rules, and the approved target is amendable mid-implement with user consent. Drift is caught at the moment it is created, one cycle deep.
3. **Landing diff (backstop)** — the `authoring-architecture` dispatch at implement's landing (already firing per D3) additionally receives the approved target and reports *built vs. approved* divergence before folding built reality into `ARCHITECTURE.md`; the user sees "built as approved" or the delta at implement's acceptance.

**Rationale:** Matches ratified doctrine — judgment-shaped outcomes escalate, clean outcomes don't (team-method D3) — and places the human moment where the cascade starts: the deliberate deviation decision. Rejected: landing-diff-only (drift discovered after all code exists — cascade already run; the sign-off reduced to a hope); per-cycle conformance verification (recurring check with usually nothing to find — exactly the always-on verification weight the token-reduction rulings cut). User chose the recommendation.

**Review folds (user-adopted batch, 2026-07-30):** **R2 (S2):** the deviation escalation extends **upstream to design time** — Phase-2 authoring (`data-model.md`/`contracts`) that reveals a contradiction with the approved architecture stops and returns to the sign-off for a consented target amendment, the same mechanism as mid-implement; D6 is design-and-build-time. **R7 (S7, fused F5+D):** the structural trigger becomes **diagram-anchored and mechanical** — "does this change add/remove a box, add/remove/redirect an arrow, or move a responsibility across a boundary on the approved diagram?" — self-checked at **cycle open and cycle close**; and the landing diff fires **whenever an approved structural delta existed**, independent of what was built — closing the subtractive direction (an approved-but-not-built change can no longer escape both mechanisms). **R8 (S8):** the landing diff is acknowledged as **new build capability** — a built-vs-approved topology diff taking the approved artifact as input, assigned to the `authoring-architecture` dispatch as a named build item, not an assumed existing capability.

### D7 — A new dedicated persona authors the architecture artifact · `Contested`

**Statement:** A **new dedicated agent persona** (working name `system-architect`; naming at build) authors the architecture artifact, carried by a **new model-invoked skill** (working name `patterns-system-design`; distinct from `authoring-architecture`, which keeps the operating-doc fold at landings). The grading side is unchanged in structure: `principal-architect` keeps the feasibility review, now with a topology to grade NFRs/constraints against; `review-plan-artifacts` gains coverage checks (every component in the table appears in the diagram; every qualifying flow — ≥2 components with non-trivial ordering or failure semantics, per D4.2's R4 re-key — has a sequence diagram). <!-- B1 verify-pass propagation 2026-07-30: was "P1 journeys have sequence diagrams"; re-keyed to match R4 so the verification enforces the amended requirement, not the floor. --> `technical-analyst` consumes the approved architecture downstream — `data-model.md` and `contracts/api.yaml` must conform to it.

**Rationale (user's, maintained after direct challenge):** "Distinct judgment deserves its own persona — it is an important step." Topology judgment (event-driven vs request/response, monolith vs services, where boundaries cut) is not requirements-decomposition judgment, and D5 made architecture the weightiest always-signed-off artifact in plan. Side benefit accepted into the rationale: author ≠ consumer forces the artifact to be hand-off legible — the same property the sign-off needs. **Lead's counterargument, recorded:** +1 agent against minimalism doctrine, and the design context splits mid-pyramid — `technical-analyst` can misread an architecture it didn't design, surfacing only at review. Recommendation was `technical-analyst`-stretches (A); user ruled C knowing the cost. Mark: `Contested` per discipline.

**Review folds (user-adopted batch, 2026-07-30):** **R1 (S1):** the claim "grading side unchanged in structure" is corrected to **extended, not restructured** — the feasibility seat (`principal-architect`) gains an explicit **architecture pass** (topology feasibility + governance conformance per D9.2), and `review-plan-artifacts` gains the **conforms-to-approved-architecture** check over `data-model.md`/`contracts`, beyond internal consistency. The plan-command boundary edit this requires (the feasibility seat's current Phase-2 bar) is named build work. **R3 (S3):** `system-architect` authors the **structural D-XXX rows** at architecture time, written into a designated structural-decisions section of `constraints-and-decisions.md`; `technical-analyst` keeps the Phase-1 rows — the D9.4 chain's previously unowned link now has a named author and phase.

### D8 — The sign-off presents the *rendered* diagram, never raw source · `Confident`

**Statement:** The architecture sign-off obligates the lead to present the **rendered** diagram to the user — via whatever render surface the session has (side-panel file render, published artifact, IDE preview) — never a raw mermaid code block in the terminal. The visual reaching the user's eyes is part of the gate's definition, not a courtesy.

**Rationale:** The original ask is visual comprehension; mermaid source in a terminal does not cure blindness. A sign-off ruled over un-rendered source would be exactly the blind approval D1 exists to prevent. User: "yes, log."

**Review folds (user-adopted batch, 2026-07-30):** **R5 (S5):** no-render-surface fallback — when an attended session has none of the named render surfaces, the gate **degrades with record**: the diagram source + component table are presented and the artifact records "presented un-rendered" (a recorded absence, mirroring waiver discipline); plan is never hard-blocked by rendering. **R10 (S10):** the presenter is the **plan supervisor (the gate owner)** — "the lead" in this decision names that role.

### D9 — Governance integration: bound at design, verified at review, amended never overruled, logged through existing machinery · `Confident`

**Statement:** Four pieces, all kept by user arbitration ("coverage good"):
1. **Constitution as design input** — the `system-architect` brief includes the governance region + relevant rules files (layer-rules, domain-dependency registry when attached); the artifact cites the principles that bound the target ("respects BE-HEX layering per GI-XXX").
2. **Governance conformance graded at review** — `principal-architect` (already feasibility grader *and* governance owner) explicitly grades the proposed architecture against the constitution's architectural surface: layers honored, dependencies within allowlist, NFR-linked principles satisfiable by the topology. Conformance verified, not asserted.
3. **Conflict routes to the amendment/waiver path** — a proposed architecture that needs to break governance surfaces the conflict at sign-off with exactly two exits: redesign to conform, or a user-ruled amendment/waiver through the existing `governance-ledger.md` machinery. The feature gate never overrules the constitution.
4. **Decision logging rides existing machinery, link mandatory** — plan-time structural choices with genuine alternatives get D-XXX rows (existing ADR discipline); the delta summary links to them (per D4.3); landing's KM ritual (v0.29.0) provides the `DECISIONS.md` row + `ARCHITECTURE.md` fold. The new obligation is the unbroken chain **diagram delta ↔ D-XXX ↔ `DECISIONS.md` row ↔ `ARCHITECTURE.md` state**, with the review coverage check verifying the links exist.

**Rationale:** Binds the new design-time artifact to both existing "architecture" homes — the constitution layer (governance) and the KM layer (decision/state records) — inventing no new machinery, only the connections. The user was offered a wider reading ("architecture principles as a setup-time constitution module") and did not take it up; not ruled on, available to a future session.

**Review fold (user-adopted batch, 2026-07-30):** **R3 (S3):** the chain's unowned link is closed — structural D-XXX rows are authored by `system-architect` at architecture time in a designated section of `constraints-and-decisions.md` (full fold at D7).

---

## Review

**Sizing:** user ruled **pair** at the sizing gate (2026-07-30). Lens split per doctrine: `reviewer-decision` (decision quality) + `reviewer-integrity` (record integrity). Both read the frozen record cold, formed findings independently, then ran the one-shot four-message cross-examination. Fact substrate: the record's fact-checker map; integrity's sample audit of it came back **clean** (9/9 load-bearing path-cited claims verified). No fact disputes routed by either side.

**Tally (lead-merged):** 11 raised (6 decision-lens + 5 integrity-lens) → 11 survived cross-exam (1 facet fell: F6(a), withdrawn — retrievable) → **10 merged survivors** (F5+D fused on their shared root per both reviewers' recommendation). Severity: 7 Important, 3 Minor. Both reviewers' recommended status: **needs-revision** — every survivor resolvable by the session; no broken load-bearing fact.

### Merged survivor queue

- **S1 [Important] (integrity A) — no seated grader for the architecture artifact.** D7 claims "grading side unchanged," but plan's feasibility seat is barred from Phase 2 and the added `review-plan-artifacts` checks are internal-consistency only — nothing is chartered to grade architecture-vs-constitution (D9.2) or data-model/contracts-vs-approved-architecture (D2). *Disposition: **resolved** — user-adopted batch (2026-07-30); fold recorded in the amended decision.*
- **S2 [Important] (decision F1) — no design-time correction path.** Architecture is signed off before `data-model.md`/`api.yaml` exist; if Phase-2 authoring reveals the approved topology wrong, D6's escalation is implement-only — nothing re-triggers the sign-off. *Disposition: **resolved** — user-adopted batch (2026-07-30); fold recorded in the amended decision.*
- **S3 [Important] (integrity B) — the D-XXX chain has an unowned link.** Topology choices are made at Phase-2 architecture time by `system-architect`, but D-XXX rows live in the Phase-1 artifact owned by `technical-analyst` — the structural-decision row has no named author/phase. *Disposition: **resolved** — user-adopted batch (2026-07-30); fold recorded in the amended decision.*
- **S4 [Important] (decision F2, counterpart conceded) — the sequence-diagram cap is misaligned with its purpose.** "P1 user journeys" tracks story priority, not ordering complexity; the ordering-critical system flow (async settlement/retry/webhook) is exactly what gets omitted. *Disposition: **resolved** — user-adopted batch (R4); the D4.2 amendment carries the user's word.*
- **S5 [Important] (decision F3, narrowed) — D5×D8 unsatisfiable with no render surface.** An attended plain-terminal session has none of the three named render surfaces; always-on stop + render-mandatory gate = a gate that cannot be satisfied; no fallback defined. *Disposition: **resolved** — user-adopted batch (R5, degrade-with-record over hard-block); fold at D8.*
- **S6 [Important] (decision F4) — current-state half of the delta: unreliable seed, unscalable presentation.** (a) No-`ARCHITECTURE.md` brownfield forces reverse-engineering the baseline every run (`codebase-analysis.md` yields a pattern label, not topology) — sign-off against a possibly hallucinated baseline; (b) a 50-service system renders a 50-box wall for a 2-node delta. *Disposition: **resolved** — user-adopted batch (R6a bootstrap + R6b scale bound); folds at D3.*
- **S7 [Important] (decision F5 + integrity D, fused root) — D6's closure leaks in both directions.** Additive: mid-cycle deviation rests on the engineer self-classifying "structural" with no operational test (unapproved cache slips through until landing). Subtractive: a silently descoped approved delta triggers neither D6.2 (no self-recognized deviation) nor D6.3 (nothing structural built → `authoring-architecture` never fires) — no backstop at all in that direction. *Disposition: **resolved** — user-adopted batch (2026-07-30); fold recorded in the amended decision.*
- **S8 [Minor] (decision F6(b)) — the landing diff assigns nonexistent capability.** `authoring-architecture` writes prose from built code; no built-vs-approved conformance mechanism exists anywhere — D6.3 is new build work, unflagged. *Disposition: **resolved** — user-adopted batch (R8, named build item); fold at D6, rides R7's repair.*
- **S9 [Minor] (integrity C, mitigated) — "component" overloaded in D4** (excluded C4-L3 sense vs kept container-register sense), unflagged. *Disposition: **resolved** — user-adopted batch (R9 disambiguation); fold at D4.*
- **S10 [Minor] (integrity E, borderline — counterpart objection attached: referent inferable) — D8's "the lead" names no plan-command role.** *Disposition: **resolved** — user-adopted batch (R10, one-line fold naming the plan supervisor as presenter; the drop was declined). Counterpart objection noted and mooted by the fold.*

### Dispositions and verify

**User ruling (2026-07-30):** "adopt the batch" — R1–R10 adopted whole, after the lead's per-item recommendation (R10 as fold-not-drop; R5 as degrade-with-record over hard-block). 10/10 survivors dispositioned **resolved**; zero recorded-open; zero overruled.

**Verify pass:** reviewer-integrity (the integrity lens owns it per doctrine) — first pass: **9/10 folds clean, 1 blocking (B1)**. B1: R4's re-key had not propagated into D7's coverage-check clause — the verification would have enforced the P1 floor and missed exactly the ordering-critical flow S4 exists to guarantee. **B1 folded** (D7's clause re-keyed to "every qualifying flow," marked inline); bounded re-verify of the B1 fold: **VERIFIED CLEAN** (landed · closes the under-enforcement · no new contradiction). **Reviewer's final verdict (record-integrity lens): READY** — all ten folds verify clean; bookkeeping consistent; confidence marks honest; the record stands alone. **Lead's clearing verdict: READY** — 11 raised → 10 merged survivors → 10/10 resolved (user-adopted batch R1–R10) → verify pass clean after one blocking repair (B1). Awaiting user acceptance.

**Build-seam notes from the verify pass (carry-forward, not record defects):**
- **N1:** the `authoring-architecture` dispatch now has two distinct firing conditions — the **diff** fires on approved-delta-existed (R7, broad); the **`ARCHITECTURE.md` fold** stays on built-structural-change (D3, narrow). Make the separation explicit at build so an approved-but-not-built delta triggers the diff without forcing a doc update.
- **N2:** R6b × D5 no-delta on a large system — the "past threshold → link, never inline" rule governs the no-delta presentation too; state it at build.
- **N3:** R5 is a bounded, recorded exception to D8's never-raw-source rule — intentional, user-adopted eyes-open.
