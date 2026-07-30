# Strip notes — `commands/plan.md`

Entry formats: `strips/README.md`. Wave context: the plan cluster wave (BACKLOG item 7, the third
one-shot-command wave after specify's v0.13.0 and slice's v0.14.0). The wave also ran the **D2
conversion assessment** (one-shot → team-form) and re-checked the **S8 home-revision checkpoint**
against plan's needs (a standing producer spanning two phases + two reviewer seats, one of them
fire-once — no new shape gap at that wave, when the shape was v2). **Stale as a standing claim:** the
shape is now **v4** (2026-07-30) — see the v0.31.0 entry below.

---

# v0.34.0 — the goal-shape pilot (CS-D10 step 2)

**Wave context:** command goal-shape rebuild, **step 2 of 4** — the pilot (design:
`.mochiko/brainstorms/command-succinctness-strip/record.md`, CS-D3/D4/D5 + D8 + D10;
`DECISIONS.md` 2026-07-30). plan was chosen as the pilot because it is the heaviest file and
carries every content class: 6 seats, 7 gates, slice-scoping, the architecture stage. Authored
against **shape v5** (`.mochiko/strips/command-shape.md` v0.33.0) with the obligated
`loop-discipline` read **retained** — the drop is step 4 and checkpoint-gated, so a v5 command
that omits it is non-conformant, not early. This file's rewrite is the **first live run** of the
revised `validation-command-shape`, including the negative direction of check 1 (plan declares the
in-loop branch, so it must **not** reference `sized-end-stage-review.md` — it does not).

**Measured: 4,439 → 1,950 words (−56.1%), 33,833 → 14,084 B (−58.4%)** — `wc`-measured after the
fix round, superseding this headline's pre-fix figures (1,940 w / 14,053 B / −56.3% / −58.5%).
Against the pre-pilot measured floor of 1,791 w: **+159 w (+8.9%)** — over, not under, which is the
safe side of CS-D8 (landing materially *under* a floor row would signal dropped content). The
overage is accounted line by line: the completeness reviewer's mode-selection binding (~45 w, a
v0.31.0 *Kept deliberately* item the floor draft compressed too far), the G3 render fallback
promoted from an HTML comment to visible body text (~50 w), G2's greenfield-degeneration case
(~20 w), `GLOSSARY.md` minting in the KM binding (~8 w), and the fix round's two audit-mandated
restores (~31 w — the `@`-reference recovery and the `preference → G5` class), less the 21 w of G7
provenance relocated to make room under the Constraints ceiling. Run-level: the file drops
19,749 B while the v5 shared read floor adds 2,895 B → **−16,854 B per plan run**, against the
−17,430 B the floor projected.

> **Standing habit adopted (auditor-suggested, 2026-07-30):** re-run `wc` and sweep every headline
> figure after **each** fix round, not only at first delivery. This was the **third** stale-headline
> instance in this build (the grader note's 180/1,861/12,441 at the v0.33.0 delta, this note's
> pre-fix figures here, and the interim Constraints 782/791 correction) — always the same cause: a
> summary written before the last edit landed. Carried into the step-4 wave briefing material so
> five commands do not reproduce it five times.

Block sizes against the grader's ceilings (terms as the grader counts them — **G=7** gate lines,
S=6 seat rows, A=15 artifacts, R=17 resume rows): preamble 99/130 · Goal 123/150 · Seats & checks
317/370 · **Constraints 791/810 (97.7%)** · Bindings 233/300 · Recovery 253/298.

**The ceiling was genuinely tested, and it held.** The fix round's two mandated restores took
Constraints to **812/810 — two words OVER**, a real floor FAIL on check 6. The restored content is
protected (check 14 demanded it), so it could not go; and loosening a ceiling I calibrated myself,
to fit a file I authored myself, is precisely the quota-override the sibling wave's D1 forbids —
in the opposite direction. Resolved instead by relocating the one piece of **pure provenance** in
the block: G7's sentence naming which two signatures dissolved into it (21 w), whose home is this
note's v0.32.0 entry. Constraints lands at **791/810** with 19 words spare.

Datapoint for the checkpoint: plan is the heaviest command, so if any Constraints block cannot fit
90·(G+2), it is this one — and it fits, but only after provenance was moved out under audit
pressure. **Recommend confirming the ceiling, not loosening it**, with the caveat that it leaves
little room for a command that wants narrative provenance in Constraints. That is arguably the
ceiling working as designed: provenance belongs in a non-loaded note, not in a file paid on every
run.

## [v0.34.0] The phase body and the Contract section retired into the five-block anatomy
- **Disposition:** superseded → the goal-shaped anatomy. `Phase 0`→**G1** + the Slice-scope
  constraint · `Phase 1`→ the seat rows + G4/G5/G6 + the ordering invariants (its step-4 verdict
  narration is the record's D5 fold (a) graded exemplar, distilled to exactly the three ruled
  constraint lines) · `Phase 2`→**G2**/**G3** + ordering invariants · `Phase 3`→ the
  design-contradiction-returns-to-G3 invariant · `Phase 4`→ the mapping-before-tasks invariant +
  the mode-selection binding · `Phase 5`→ `plan.md` in Bindings + **G7** · `Phase 6`→ the KM
  binding + the Recovery table's accepted row. The `Contract` section's four clauses →
  **Goal** (done-condition + not-done states), the **Seats & checks** table (producer↔validator),
  **Constraints** (bounds + gates).
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4**
  "the connective procedure is deleted, and what survives is *restructured*" · **CS-D5** the
  five-block anatomy and the Contract-as-document inversion).
- **Content:** ten `## Phase`/`## Contract`/`## State recovery` sections, 2,873 words of ordered
  procedure and appendix. Not reproduced verbatim here — every *rule* inside them survives in the
  ledger below, and the deleted remainder is connective narration ("Then apply the bounds…", "loop
  to step 1", step numbering, and the lead's job description restated per phase). Recoverable in
  full at `git show c47684d:plugins/mochiko/commands/plan.md`.
- **Kept deliberately:** every gate, bound, routing decision, trigger, ordering rule and artifact
  binding — see the CS-D8 ledger below, which resolves each one individually.

## [v0.34.0] The `What you own (not the seats)` footer deleted
- **Disposition:** deleted.
- **Tier failed:** 1 — a declared duplicate. 157 words restating the gate list, the counter
  ownership, the verdict ownership, the peer-edge sequencing, the feasibility routing, the
  skip-architect rule, the deviation return, the governance two-exit, and the collapse
  prohibition — every one of which is now a Constraints line or a Seats-table cell. The checker
  map recorded this footer class as already-deduped-once at v0.13.0–v0.17.0 and still surviving
  (record §9.4); the anatomy leaves it nowhere to hide.
- **Kept deliberately:** nothing was unique to it. The one clause with no other home — "verifying
  each seat actually wrote its expected files (a missing output → log and ask retry/abort)" — is
  **not** dropped: it is the lead's dispatch hygiene, and it survives as the Recovery block's
  evidence-driven resume (a missing artifact *is* a resume row) plus G6's escalation menu.

## [v0.34.0] The `shape-exception` marker retired — its ground dissolved at v5
- **Disposition:** superseded → the AD-D8/R5 degrade-with-record fallback survives as **visible
  Constraints content** on the G3 line; the `<!-- shape-exception: ... -->` marker around it is
  retired. plan now carries **zero** exception markers.
- **Tier failed:** n/a — supersession by ruling (**CS-D8** re-grade + the checkpoint's
  re-justify-or-supersede instruction).
- **Content (the retired marker, verbatim):** `<!-- shape-exception: D8/R5 — when an attended
  session has none of those render surfaces, the gate degrades with record: present the diagram
  source + component table and record "presented un-rendered" on the artifact (a recorded absence,
  mirroring waiver discipline). Plan is never hard-blocked by rendering. -->`
- **Grounds for retirement, stated plainly because this is the pilot's one contestable call:** the
  marker existed because the fallback *mirrored the shape's waiver discipline* — recorded absence
  rather than silent degradation. At v5 that discipline no longer lives anywhere plan reads: it
  left Layer 1 with the sized-end-stage-review block, into a conditional home
  (`templates/sized-end-stage-review.md`) that plan is **forbidden** to load (it declares the
  in-loop branch; check 1 enforces the negative direction). A marker whose cited restatement target
  is unreachable from the graded file points a future auditor at content they cannot find — the
  exact false-positive class the v0.33.0 grader fix pass named for check 8's homeless markers. The
  fallback restates nothing in plan's v5 read set, so it is plain P7 content.
- **Kept deliberately:** the fallback's every element — the trigger (no render surface in an
  attended session), the degraded presentation (diagram source + component table), the recorded
  stamp ("presented un-rendered" on the artifact), and the never-hard-blocked guarantee. It is now
  *more* visible than at v4, where the whole rule sat inside an HTML comment.
- **Consequence for the audit:** plan's contribution to the surface's `shape-exception` inventory
  goes 1 → 0; `setup.md:100–101` remains the only live marker, unexamined here and due at step 4.

## [v0.34.0] Skill-owned content stripped from the command body
- **Disposition:** relocated → the skills that already own it (no new home written; verified by
  reading each skill's declaration this run).
- **Tier failed:** 1 (altitude).
- **Content:**
  - The architecture artifact's **scope bound** — "scoped to the delta neighborhood past the
    artifact's size threshold (the full-system view is linked, never inlined — the same scope bound
    governs the no-delta presentation)". Home: `mochiko:patterns-system-design`, which states
    "scopes the diagram to the delta neighborhood (changed components + direct collaborators; past
    a threshold the full view is linked)". **Kept deliberately:** plan's G3 line still says the
    no-delta case presents the *neighborhood-scoped* diagram — the binding survives as a reference,
    the rule's statement does not.
  - The **ADR discipline** for topology alternatives — "Genuine-alternative topology choices get
    D-XXX rows here (existing ADR discipline); the delta summary links each structural change to
    its D-XXX row, never restating it". Home: `mochiko:patterns-technical-decisions` (the
    decision/ADR technique) + `patterns-system-design` (the delta-summary→D-XXX link).
    **Kept deliberately:** the *designated structural-decisions section* and its architect
    ownership stay in plan's Bindings — that is plan's own artifact binding, not ADR technique.
  - The standing seats' **retention rationale** — e.g. "Its retained context is what makes each
    later stage's check incremental rather than a cold re-read". Home: this note's v0.15.0
    conversion entry, which records the retention bet in full. The Seats table carries the
    operative fact (standing, and across which stages).

## [v0.34.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects two sets: `KEPT:`/Tier-2-evidenced lines, **and** every
line traceable to a `DECISIONS.md` row. plan carries **no `KEPT:` survivor-provenance entries**;
its protection set is the *Kept deliberately* fields of the two prior supersessions plus the
DECISIONS row trace. Grepped before any cut, per D8's enumeration procedure. **All 23 rows survive
translated — zero superseded, zero dropped.**

**Two rows were restored at the pilot fix round, not found by the author.** The independent audit
FAILed the pilot on check 14 (preserved responsibilities) for the `@`-reference recovery and the
`preference → G5` routing class — both genuinely dropped in the first draft, both restored below
and marked. Recorded here rather than silently folded, because the pattern matters for step 4: the
losses were in *compressed evidence clauses*, not in deleted sections — G1's evidence list and
G5's routing enumeration each lost a clause while the surrounding gate line still read as
complete. The five-command wave should grep the routing classes and the named-cause recoveries
per command rather than trusting a gate line that looks whole.

| protected line | source | resolved |
|---|---|---|
| Every verdict stays the lead's; **no devolved branch** (plan has no deterministic-CLI verification, so shape D3's branch cannot apply — declared, not left implicit) | v0.31.0 *Kept deliberately* | Ordering invariants: "**No devolved branch** — every review here is a judgment grade, so no gate is skipped and no unit clears unread" + the validation-model line's "every verdict is yours" |
| Feasibility architect **lead-gated**, fires once, re-fires only on structural change | v0.31.0 *Kept deliberately* + second audit round | Seat row (spawn + peer-edge cells) **and** the ordering invariant naming the three structural triggers and the clarification-only exception |
| The completeness reviewer's **mode-selecting message** is the lead's policy call | v0.31.0 *Kept deliberately* | Validation-model line: you select skill + mode per stage and supply the artifact sets (incremental / cumulative, with both named) |
| **Delivery is not a start signal** — it grades only when you open the pass | v0.31.0 in-wave addition (plan-specific: two-reviewer ordering + mode-selected stage) | Seat row ("grades only when you open the pass") + the ordering invariant |
| Slice binding 1 — a producer designing beyond scope is a scope gap → G6 | v0.15.0 slice-scoped entry, *four genuine bindings kept* | Slice-scope constraint, first binding |
| Slice binding 2 — a `[MODIFY]` graded amendment surfaced for this round's reviews, migration flagged | same | Slice-scope constraint, second binding |
| Slice binding 3 — per-slice outputs → the done-condition's artifact set | same | Bindings' artifact preamble (per-slice layout → the Goal's artifact set) |
| Slice binding 4 — the reviewer briefing sets {this slice + extensions} / {prior accumulated} | same | Slice-scope constraint, third binding |
| Graduation contract is the single home; do not restate | v0.15.0 audit catch (the D1 churn liability) | Slice-scope constraint opens by naming it as the single home for the six rules, and restates none of them — the defect that entry was written about is not reintroduced |
| AD-D1 · AD-D2 — design-time architecture, first design artifact, own early sign-off | DECISIONS rows | Ordering invariant ("the **first** artifact of the design work") + **G3** |
| AD-D3 — delta model, baseline bootstrap, landing fold | DECISIONS row | **G2** (bootstrap + confirm-before-delta) · Bindings' `architecture.md` · the KM-landing binding (baseline → `ARCHITECTURE.md` via the scribe) |
| AD-D4 — artifact contents (C4 diagram, sequence, component table, deployment view) | DECISIONS row | Referenced, never restated: Bindings names `patterns-system-design` as the owner of structure **and** scope bound |
| AD-D5 — always-on, no-delta included | DECISIONS row | G3: "*(always-on)*" + the no-delta presentation with its one-line claim, "the judgment is shown, never silently made by the producer" |
| AD-D7 — `system-architect` × `patterns-system-design`; feasibility gains the architecture pass; structural D-XXX architect-authored | DECISIONS row (`Contested`) | Two seat rows + Bindings' designated structural-decisions section |
| AD-D8 / R5 — rendered-diagram sign-off, plan supervisor presents, degrade-with-record | DECISIONS row | G3, in full — presenter named, render surfaces enumerated, raw-mermaid prohibition, and the fallback now visible (marker retired above) |
| AD-D9 — governance binds the design; conflicts route to amendment/waiver, never overruled at a feature gate | DECISIONS row | **G4**, the two-exit with "the feature gate never overrules the constitution" |
| Team-method D4/D5 — plan absorbs tasks; **one** package acceptance | DECISIONS row | The mapping and tasks stages in the seat table + the mapping-before-tasks ordering invariant; **G7** declared as "the package's **one** standing acceptance". The *provenance* of the merge — which two signatures dissolved into G7 — is **not** in the command: it is history, already single-sourced in this note's v0.32.0 gate-renumber entry. Relocated at the fix round (see the ceiling note below); the ruling is encoded by the file's structure, not by narrating what the file used to be. |
| Vertical-graduation — slice-scoped entry variant | DECISIONS row | The Slice-scope constraint + Bindings' per-slice layout |
| The **`@`-reference recovery** — empty `$ARGUMENTS` has a *named cause* (the `@`-reference drop bug) and a two-option prompt (re-enter, or confirm the detected feature) | record §7 protected set (the `command-altitude` retrofit-regression warning names the `@`-reference recovery among the hard-won fixes verbosity encodes); still carried by `implement.md` | **G1** decides-clause. **Restored at the pilot fix round** — the first draft compressed G1's evidence list and lost both the cause and the prompt, leaving "empty is resolved at G1" with no recovery behavior. Exactly the retrofit-regression class §7 warned about, caught by the audit's check 14. |
| All three of the exemplar's **gap-routing classes** — knowledge → `Explore` / the research branch · **preference → G5** · scope → G6 | record D5 fold (a) graded exemplar, line 1; `loop-discipline` gap routing | **G5** names the preference class and the knowledge branch and points scope at G6; **G6**'s evidence carries the scope trigger. **Preference restored at the pilot fix round** — the first draft carried knowledge and scope but dropped preference, so the exemplar's own preservation standard was not met on the line it was drawn from. |
| The `quickstart.md` **null path recorded** in `plan.md` | current body (conditional artifact) | Bindings (conditional + null path) **and** the Goal's not-done states |
| `plan.md` is a summary over validated artifacts, **never new design** | current body | Bindings, on the `plan.md` entry |
| Round reports cleaned by default; never offer to delete a deliverable | current body | Bindings' Reports entry |

## [v0.32.0] Build note + shape-v4 re-conform — merged design-room command: absorbs `/mochiko:tasks` + gains the architecture stage (2026-07-30)

Design records: `.mochiko/brainstorms/team-method-vs-command-shape/record.md` (D4/D5 — plan absorbs
tasks) + `.mochiko/brainstorms/architecture-design-primitive/record.md` (AD-D1–D9 with folds R1–R10,
seam notes N1–N3). Not a strip wave — a feature build; the architecture-stage **additions** are recorded
in the `DECISIONS.md` rows AD-D1–D9 (lead-owned landing), not here (Job-4 rule: pure additions ride the
decision row, the v3 run-cost precedent). This note logs the version stamp, the **relocation** (tasks'
structuring loop moved *into* plan), the consequent cross-reference change, and the **shape-v4
re-conform** the merge required. Overall command surface 7 → 6 — see the tasks retirement note
(`strips/tasks.md` v0.32.0).

> **Version note:** this build was originally stamped **v0.30.0**; while it was in flight, origin/main
> released **v0.30.0** and **v0.31.0** (the shape-v3→v4 mesh rewrite + the six-command re-conform,
> below). The merge rebased this build onto v4, so it lands at **v0.32.0** and is re-stamped throughout.

- **Relocation IN (from `commands/tasks.md`, now retired):** the entire Mapping → Tasks structuring loop
  — the standing `task-architect` (`patterns-vertical-tdd`) producer seat, the `devils-advocate`
  (`review-task-artifacts`) reviewer in its early-mapping-then-cumulative modes, the two-sub-stage round
  loop, and the task-artifact deliverables (`task-mapping.md` · `tasks.md`) — relocated into plan's
  **Phase 4**. tasks' standalone `tasks.md`-acceptance gate (its G5) **dissolves** into plan's single
  final **package acceptance (G7)** per team-method D5 (the standalone signature was load-bearing only
  while a command boundary sat there). The `review-task-artifacts` validator is **unchanged** in
  structure — same agent, same skill, same checklists; only its caller moved. The completeness reviewer
  is now **one standing `devils-advocate` seat** that runs `review-plan-artifacts` across the design
  stages and `review-task-artifacts` across structuring (the skill is named per dispatch, never loaded as
  frontmatter — shape Layer 2), rather than two separately-spawned reviewers across two commands.
- **Addition — the architecture stage (AD-D1–D9; recorded in DECISIONS, summarized here for the trail):**
  a new **Phase 2** between Analysis and Detailed design, authored by a **new standing `system-architect`
  seat** (`mochiko:patterns-system-design`) — the delta `architecture.md` artifact + the structural D-XXX
  rows into `constraints-and-decisions.md`, its own **early sign-off gate (G3)** presenting the *rendered*
  diagram (degrade-with-record fallback, D8/R5, marked as a shape-exception in the command), always-on
  incl. the no-delta form (D5), and a bootstrap **baseline-confirmation gate (G2)** when no
  `ARCHITECTURE.md` exists (R6a). The `principal-architect` feasibility seat gains an **architecture pass**
  (topology feasibility + governance conformance) — the carve-out of its former "never grades past Phase 1"
  bar (R1, named build work); `review-plan-artifacts` gains architecture-coverage + conforms-to-architecture
  checks (referenced, the skill owns them). Detailed design (former Phase 2, now **Phase 3**) must conform
  to the approved architecture; a contradiction found in authoring **returns to G3** for a consented target
  amendment (R2).
- **Gate renumber (consequent):** the architecture gates insert early, so plan's gates renumber —
  G1 (entry) · **G2** baseline-confirm (bootstrap) · **G3** architecture sign-off · G4 feasibility/governance
  rejection (was G2, now also carrying the governance two-exit, D9.3) · G5 clarification (was G3) · G6
  exit-early (was G4) · **G7** final package acceptance (was plan's G5 *and* tasks' G5, merged). Note the
  renumber against main's v0.31.0 entry below: that entry conformed the *two-phase* plan, where
  feasibility-rejection was **G2**; in the merged command it is **G4** (its "G2" references are frozen
  two-phase history).
- **Shape-v4 re-conform (the merge work, this task):** the merged command was re-authored against
  `command-shape.md` **v4** (main's v0.31.0 bumped it from v3). The v4 idiom adopted: (a) the **in-loop
  mesh** — each producer is **peer-edged with the completeness reviewer**, handing finished artifacts
  directly (peer-routable delivery), while **delivery is not a start signal** (the lead opens every round
  and every review pass); (b) the **feasibility architect stays lead-gated** — fired selectively, its
  concerns routed through the lead at **G4** (not peer-edged, matching main's v0.31.0 narrowing on the
  two-phase plan); (c) the roster **names each seat's peer edges** per the v4 seat-roster PARAM; (d) the
  Contract states **"No devolved branch"** — every plan review is a judgment grade (feasibility,
  completeness, architecture coverage, task-artifact quality), never all-deterministic-CLI, so no gate is
  skipped and every verdict is the lead's; (e) "no producer↔reviewer contact" is dropped from the Contract
  (independence now rides disjoint agents/skills + cold *arrival*, not routing). The architecture stage's
  own peer edge: `system-architect` is peer-edged with the completeness reviewer for the coverage grade;
  the architecture *feasibility pass* is lead-gated like the analysis pass.
- **Cross-reference change:** Phase 5's next-step pointer `→ /mochiko:tasks` is superseded; the merged
  command produces the whole package and points `→ /mochiko:implement`. `templates/plan-template.md` gained
  an **Architecture** section (pointers to `architecture.md`, per the summary-not-restatement rule) and now
  lists `architecture.md` / `task-mapping.md` / `tasks.md` in its Artifacts manifest.
- **Producer report added:** `templates/sysarchitect-report-template.md` — the `system-architect`'s
  self-disclosure carrier (report: disclosure, per `report-format.md`), parallel to the techanalyst /
  taskarchitect report templates.
- **Conversion re-assessment:** the merge does not re-open the team-form ruling — all three producers
  (technical-analyst, system-architect, task-architect) and both reviewers stay standing/cold seats per the
  existing conversion assessments below and tasks' (retired) assessment. **S8 home-revision checkpoint
  re-checked:** the merged command is a larger team (3 producers + 2 reviewers, 7 gates) but rides the
  existing shape — Layer 1 as-you-go artifact + producer-authored uncertainty branch, Layer 2 mesh
  peer-edges + independence-by-cold-arrival — with **no new shape gap** (the rendered-diagram gate is a
  per-workflow gate, not shape doctrine; marked shape-exception where a line would otherwise restate
  shape). Shape stays **v4** (this build conforms to it, does not revise it). The first-dogfood
  confirm-or-revert checkpoint carries forward: the open "Dogfood `/mochiko:plan`" item now exercises the
  merged, architecture-first command.

## [v0.31.0] Lead-relayed gap lists superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place: both reviewers are still cold-spawned at their own stage (a spawn-timing parameter), and the producer↔reviewer peer edges are declared on the roster.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - producer seat: "Round > 1 within a phase is a message to the same seat carrying the reviewers' gap list verbatim"
  - feasibility reviewer: "spawned **cold after the Phase-1 analysis is authored**, never in contact with the producer"
  - completeness reviewer: "spawned **cold at the first completeness review**, never in contact with the producer"
  - Phase 1 step 1: "on round > 1 the message carries the reviewers' gap list for targeted revision"
  - Contract, Producer ↔ validator: "(both reviewers cold-spawned, gap lists lead-routed, no producer↔reviewer contact)"
- **Kept deliberately (not superseded):** every verdict stays the lead's — plan has no deterministic-CLI verification, so **D3's devolved branch cannot apply here**; the Contract now declares that absence rather than leaving it implicit. Also kept lead-gated: the **feasibility architect's engagement** (fired once, re-fired only on a structural change) and **Phase 2's mode-selecting message** — both policy calls under the traffic classes, not hand-offs.
- **In-wave correction (audit round, 2026-07-30):** the peer edge this wave first wrote was **blanket** — "**Peer-edged with both reviewers:** it hands each round's finished artifacts straight to them" — and its Phase 1 step 1 counterpart "handing them to the reviewers directly when the round's set is complete". The audit caught that this silently peer-routed the *architect*, whose fire-once/re-fire-on-structural-change engagement is a lead-gated policy call, and that it read as licensing a completeness pass before the feasibility gate. Narrowed to the **completeness reviewer only**, with the lead sequencing when it grades. Logged as an in-wave correction, not a separate version: the superseded text never shipped outside this wave.
  - **Second audit round (same wave):** the narrowing was applied to the roster bullet and Phase 1 step 1 but **not propagated**, leaving three sites still asserting the blanket edge. Substance was upheld; only propagation failed. Also superseded, same correction: the feasibility-reviewer bullet's "peer-edged with the producer thereafter" → "**lead-gated thereafter** — you fire it, and its concerns reach the producer through you (G2)"; and the Contract's "gap-list hand-offs peer-routed producer↔reviewer per the shape's mesh, with every verdict yours" → the completeness list peer-routed, "the architect's routes through you at G2". Added in the same pass (not a supersession): the completeness reviewer's **verifying-side hold** — "Delivery is not a start signal — it grades only when you open the pass (Phase 1: after the architect; Phase 2: on your mode-selecting message)". The producer-side hold is universal and lives in the shape home; this one is plan-specific — two-reviewer ordering plus a mode-selected Phase 2 — so it binds at the seat and makes Phase 1 step 1's "you sequence when it grades" a reference to a bound rule rather than a bare assertion.

## [v0.15.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** plan runs a producer↔two-reviewer cycle (≤3 rounds
  per phase, gap-list-driven revision, cold reviewers) across **two phases** (Analysis → Design) whose
  context-retention bet is plan's own — the longest horizon of any converted command: a **standing
  producer seat** holds (1) the Phase-1 analysis rationale carried into the Phase-2 design across six
  artifacts (why a decision beat its alternatives, which constraint shaped it, what NFR targets bind —
  authored from lived context, not reconstructed from files), and (2) the C↔D dependency web so a
  targeted revision after a feasibility rejection stays coherent. The two reviewers map to: a
  **standing completeness advocate** (`devils-advocate`, cold at first spawn, spans both phases — its
  retained Phase-1 context is what makes the Phase-2 incremental consistency check a spot-check, not a
  full re-read) and a **cold fire-once feasibility architect** (`principal-architect`, grades once
  post-Phase-1-produce, re-fires only on a structural change, never grades Phase 2). Neither reviewer
  contacts the producer — independence stays structural. Transport rides the v3 fix
  (`agent-dispatch.md` Seat transport + addressability probe on the producer's first spawn).
- **Steelman recorded (user-ratified with the conversion):** zero successful team-form runs at
  conversion time (two setup defect runs; specify's + slice's own checkpoints unfired; brainstorm v2
  measured standing seats *more* expensive than dispatches). Plan is the **most expensive command to
  run as a standing team** — three seats across two phases vs specify/slice's two — so it pays the
  largest team-form tax if the retention payoff doesn't land. The **fire-once architect is the weakest
  team-form fit**: it usually fires once and sits dormant, getting little from persistence (modeled as
  a standing seat messaged sparsely — uniform transport, and it keeps its Phase-1 read on a
  structural-change re-fire; the honest steelman is that the architect alone would be fine as a
  bounded one-shot subagent). And the design artifacts **reconstruct relatively cheaply from disk** —
  the six artifacts are richly ID'd and the FR→TR→entity→schema traceability is written *in the files*,
  so the retention payoff, while real, is smaller than "six artifacts / two phases" suggests. Ruled
  team-form anyway per D2's declared default + S4 (no prior dogfood evidence required; checkpoint
  below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open "Dogfood
  `/mochiko:plan`" BACKLOG item, Plan-port follow-ups) confirms the conversion or reverts it to
  one-shot Layer-1 form; a revert is logged as a `RETURNED:` entry here. Team-form named checks: the
  producer probe fires the addressability check; the standing producer seat is messaged (not
  respawned) across rounds and across the phase boundary; the completeness advocate spawns cold and is
  messaged in Phase 2 for incremental mode; the feasibility architect fires once and re-fires only on
  a structural change; neither reviewer contacts the producer.

## [v0.15.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the
  `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all
  four requirements (default-FAIL done-condition, independent validation, bounded iteration, named
  human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here…"
  — restated loop-discipline's own enumeration.

## [v0.15.0] Per-run contract fill (`workflow-contract.md` → `plan-contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract — the
  authoring-time-fill rule); the per-workflow values survive as the command's authoring-time Contract
  section (plan's are richer — a per-phase round cap and five gates)
- **Tier failed:** 1 (the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "## Contract parameters (fill the artifact — don't inline it) … Fill
  `templates/workflow-contract.md` → `.mochiko/specs/<feature>/plan-contract.md` with the values
  below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable
  proof — not this command body."

## [v0.15.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Done-condition / Producer↔validator clause; the
  `review-*` family boundary also lives in `review-feasibility` + `review-plan-artifacts` descriptions
  + REGISTRY). The per-phase Verdict *steps* (Phase 1 step 4, Phase 2 step 3) are workflow mechanics
  and survive.
- **Tier failed:** 1
- **Content:** stated at the lead framing ("Each reviewer *recommends* a status; **you own the
  clearing verdict** — their status is input, never the gate") and again in the footer ("the verdict
  (each reviewer grades from the files, you Read the artifacts and decide against the default-FAIL
  done-condition — their status is input)").

## [v0.15.0] Footer ground rules + Task-tool transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules);
  the Task-tool line superseded by the team-form conversion (transport now per shape Layer 2 +
  `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool
  (never inline agent behavior); do not modify git or push."

## [v0.15.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context
  `phase` field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`/`status`):" + the
  entry-gate parenthetical "(workspace evidence — there is no context-file `status` to read)". The
  recovery table (evidence → resume-at) is the workflow-specific Recovery PARAM and survives.

## [v0.15.0] "Why this done-condition differs from HIL's" blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical/motivational provenance; preserved in
  ROADMAP's Decision Trail + `.mochiko/transform/plan/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL declared "no hard caps" and routed on
  each agent's verdict *field* — it could self-declare done at pass 1, violating `loop-discipline`
  reqs 1 & 3. The two reviewers' three-state statuses survive only as input to your verdict; the
  deterministic cap and the new G5 acceptance gate close the gates HIL lacked." — the shape of
  specify's deleted HIL-comparison blockquote; its rationale is carried by the Contract done-condition
  + `review-feasibility`'s "Preserve `infeasible` as a distinct state" doctrine, so no unique behavior
  is lost.

## [v0.15.0] Slice-scoped entry — restated Graduation-contract rules (audit catch)
- **Disposition:** relocated → `templates/slices-template.md` (the **Graduation contract** section —
  the single home of the consumption rules); Phase 0 step 5 now *applies* the contract by reference
  for slice resolution, the staleness guard, scope, extend-mode, graded amendment, and artifact layout
- **Tier failed:** 1 (the step declared "the single source … do not restate it" and then restated most
  of it — the D1 churn liability)
- **Content:** the copied rules — slice resolution ("named in `$ARGUMENTS`, else the first slice in
  Slice-order lacking `slices/<slice>/plan.md`"), the staleness guard ("the live `spec.md` story-ID
  set must match the Spec stamp — mismatch → block and point to `/mochiko:slice`"), extend-mode ("the
  shared feature-root artifacts are brownfield input the producer extends in place — never re-derives,
  never forks per-slice copies"), and the graded-amendment definition ("a **breaking** change … is a
  graded amendment … never a silent rewrite"). The four genuine plan bindings were **kept**: G4 on
  over-scope, the `[MODIFY]`-surfaced-for-this-round's-reviews behavior, the per-slice-output →
  done-condition mapping, and the reviewer briefing sets.
- **Note:** caught by the `validation-command-shape` audit — the assessment had passed this entry as
  at-altitude on its "do not restate it" self-declaration; the audit found the restatement beneath it.
  Fixed in-wave, no version bump. The Graduation contract is on the ≥3-consumer queue (plan/tasks/
  implement slice-scoped variants) — this strip relocates plan's *local restatement* to the contract
  home; it does not rule the shared contract.
