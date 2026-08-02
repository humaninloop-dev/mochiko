# Strip notes — `commands/specify.md`

Entry formats: `strips/README.md`. Wave context: the specify cluster wave (the first of
the five one-shot-command waves; BACKLOG item 7 of the pattern-codification build). The
wave also ran the **D2 conversion assessment** and the **S8 home-revision checkpoint**
(shape v1 → v2 — see the REGISTRY `command-shape.md` row). **Stale as a standing claim:**
the shape is now **v5** (2026-07-30) — see the v0.35.0 section below. **Also stale:** the shape is
**v7**, and specify converted to v7 form at v0.43.0 — see the section immediately below.

---

## [v0.50.0] Goal/Harness lines reshaped for the UX-prototype stage
- **Disposition:** superseded → the UX-prototype-bearing forms of the same lines (Goal deliverable sentence + Not-done list · intent-stage agenda line · Independence line · Reserved-to-the-user list)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "UX mocking in specify (UX-D1–D9)"; record `.mochiko/brainstorms/ux-mocking-in-specify/record.md`)
- **Content:** the pre-edit forms: Goal deliverable listing Intent (scope / delivery / depth-rigor / constraints / out-of-scope) + stories/FR/SC/edge-cases/Delivery-Slices with acceptance covering "intent, requirements, and slicing together" and the Not-done list without the Screens & Flows items · the intent-stage agenda "scope boundary · delivery/slicing intent · depth-rigor expectation · constraints · out-of-scope" with the synthesis governing "the authoring brief, the Delivery Slices shape, and the stress-test's rigor" · the Independence line without the prototype walk · the Reserved list without the per-story clicking item and with acceptance covering "intent, requirements, and slicing whole".
- **Kept deliberately:** every prior clause survives inside the reshaped lines — nothing dropped, all lines extended for the UX-bearing ruling (D3), lockstep prototyping (D2), the prototype walk (D7), and the Screens & Flows deliverable (D5). Pure additions alongside: the Lockstep-prototyping harness block, the Prototype-craft binding.
- **Consumers assessed:** router entry row (co-edited) · ARCHITECTURE.md Specify section (co-edited) · spec-template (co-edited).

## [v0.49.0] Sparse-input enrichment line superseded by the intent stage
- **Disposition:** superseded → the Harness's "Intent stage first" block (adaptive-probe agenda via `analysis-iterative`: scope · delivery · depth-rigor · constraints · out-of-scope; one-screen user-confirmed synthesis governing brief/slicing/rigor, landing as the spec's Intent section)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D7+D8)
- **Content:** "Sparse input (Who / Problem / Value unclear) → enrich it yourself, inline, via `mochiko:analysis-iterative` before authoring starts." and the Goal/Bindings tails "or enrich from scratch with their consent" · "Next step: /mochiko:plan (or /mochiko:slice for a multi-story spec)".
- **Kept deliberately:** the rest of the v8 harness whole (plan approval · independence · governance-region surfacing · no-git-mutations · plain blocking acceptance). Command grew ~15 lines against the v8 ~30–50 guideline — accepted: the intent stage and Delivery Slices additions are ruled scope, not choreography creep.
- **Consumers assessed:** router entry row · ARCHITECTURE.md Specify section · spec-template (co-edited).

## [v0.48.0] Shape v8 goal+harness rewrite — choreography dies in place
- **Disposition:** superseded → the v8 goal+harness rewrite of this command (whole-file)
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/command-architecture-realignment/record.md` D1–D6; DECISIONS.md 2026-08-02 command-architecture row)
- **Content:** the entire v7-form file superseded — preamble dispatch-brief protocol · Seats & checks table + validation model · team-transport mandate + roster probe (D5: transport-neutral now) · seat lifecycle/recycling · every G-numbered gate, the run-start weight card, floor-gate set, counted bounds/caps/kill-switch, ordering invariants, ground-rules block · run-start declaration + departure trail + per-run contract file · KM-landing command steps · the Recovery section and resume table. Verbatim text below (pre-edit file at the v0.47.0 tree).
- **Kept deliberately:** the Goal's template-conformant, no-placeholder, stress-tested-from-the-file, user-accepted condition · lead-inline enrichment via analysis-iterative (KEPT survivor honored in substance; the `(KEPT: survivor)` marker itself dies with the v8 rewrite — supersedes the v0.44.0 entry's marker-stays clause, per the same D1 ruling) · author≠grader with grade-from-the-file (Independence line) · empty-$ARGUMENTS ask + governance-absence surfacing reserved to the user · governance obligated-read brief line · spec-template/paths bindings · never-offer-to-delete spec.md · no-git-mutation + plain-blocking-acceptance lines · output-style register pointer
- **Consumers assessed:** none — commands are entry points, nothing mounts them.

<details><summary>Verbatim superseded file (v0.47.0)</summary>

````markdown
---
description: Create a feature specification via an independent author→critic team loop — a standing requirements-analyst seat authors spec.md across bounded rounds, a cold devils-advocate seat stress-tests it from the file, the user accepts at a named gate; sparse input is enriched first; default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Specify — Feature Specification

**Goal:** turn a feature description into an accepted `spec.md` — prioritized user stories,
FR-XXX requirements, measurable SC-XXX criteria, edge cases — authored and independently
stress-tested before the user accepts it. `$ARGUMENTS` = the feature description; empty or sparse
is resolved at G1.

**You are the lead**: you compose the run and own its counters, every verdict, every escalation,
every human gate, and the user-facing conversation — agents produce and review, you adjudicate.
Every dispatch carries its own brief in the spawn or send prompt — the seat's role and skill
(named as a hint, the agent decides fit), the exact inputs to Read, where the output lands
(write vs return), the bar it must clear, its peer edges and holds, and the independence
reminder that matches the seat (author: never grade your own output; grader: read the artifact
itself, default FAIL, quote evidence) — the seat owns none of this context and gets all of it
from you; on a retry, a peer-routed gap list is pointed at and the round opened, a relayed one
pasted verbatim. This file is self-contained: specify's
whole contract lives here. **First-spawn probe:** the producer — always the first seat filled.

## Goal

`.mochiko/specs/<feature>/spec.md` exists, conforming to its template with no placeholder tokens,
with the round reports for the grading that actually ran, and no blocking gap left open against it;
G3 acceptance has cleared; and the KM landing ran.

**Not done:** default **FAIL** · a blocking gap open · a departure with no trail line · out of
rounds · G3 unaccepted.

## Seats & checks

| seat | agent × skill(s) | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `requirements-analyst` × `authoring-requirements`, `authoring-user-stories` | authors `spec.md` + `analyst-report.md` from their templates; never grades | one **named standing seat** across rounds; **probe seat** | hands each round's `spec.md` straight to the critic; round > 1 reaches it with the gap list already in hand |
| critic | `devils-advocate` × `review-specifications` | grades `spec.md` **from the file, never the producer's report** → `advocate-report.md`: severity-classified findings, product-framed clarifying questions, and a recommended verdict (`ready` / `needs-revision` / `critical-gaps`); never authors | cold at first critique, standing after — round > 1 re-Reads the revised spec | peer-edged with the producer |

**Validation model:** the loop's bounded in-loop critique, every round, **unsized by design**. The
critic's output is **lead-adjudicated input** (the `review-*` family boundary) and every verdict is
yours. No seat ever grades its own output.

**Team transport:** check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` before anything else — unset →
stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178); the first spawn is
the authoritative probe, and there is no teamless fallback. A seat is spawned with **`name:`**
("spawn a teammate named `producer`") — a nameless spawn is a one-shot subagent, the forbidden
transport. Every later round is a `SendMessage` to that same named seat. Verify from the roster:
`~/.claude/teams/<team>/config.json` (`<team>` = `session-` + first eight chars of the session ID)
holds a `members` array — the seat's `name` not in it ⇒ kill and respawn explicitly requesting an
agent team; failing again stops the run. Teammates don't load `skills:` frontmatter — every spawn
prompt names the skill and role itself. Tell the user up front they can watch or message any
teammate; announce each seat in one line when filled; never narrate or reply to teammate
housekeeping. A peer-routed gap list is a **hand-off, not a start signal** — the producer revises
only when you open the next round, and your brief carries that hold.

**Seat lifecycle:** at each gate pause, count each standing seat's completed rounds and recycle at
~≥3 (counted, never observed — no occupancy self-reports; the user may order a recycle at any
gate). A respawn is a reset: the successor is briefed from the on-disk artifact set alone and takes
a versioned name (`producer-2`) — never reuse a dead seat's bare name. A seat whose remaining work
is zero shuts down; no ritual sends — fold confirmations into the next real dispatch.

## Constraints

- **G1 entry** — evidence: `$ARGUMENTS`, and `CLAUDE.md`'s governance region
  (`<!-- mochiko:governance:begin -->`) · rules: the user · decides: the resolved feature
  description and whether the run is governed. Empty `$ARGUMENTS` → ask the user to re-enter it, or
  proceed and enrich from scratch. A missing governance
  region is **surfaced** — offer `/mochiko:setup` first, or proceed ungoverned for this spec —
  never auto-resolved. Then triage: *rich* (Who / Problem / Value clear from the description or the
  governance region's domain context) → the loop; *sparse* → enrichment.
- **Run-start weight card** — evidence: your stated read of the four rigor factors against this
  feature description — **reversibility** (how expensive is rework if this is wrong) · **blast
  radius** (how much downstream work reads the spec as authoritative) · **precedent**
  (first-of-kind, or mirroring an audit-cleared pattern) · **input confidence** (scored on the
  artifact under review; a user ruling discounts ambiguity risk only, and one that introduces new
  surface raises consistency risk) — plus the process you compose from it — the stated default
  below, or your departures from it · rules: the user · decides: the run's composed process. Rigor
  scales with cost-of-being-wrong, never task size; diff size is at most a hint.
- **G2 clarification** — evidence: a producer clarification it cannot resolve, or a finding you
  judge only the user can settle · rules: the user · decides: the answer fed forward into the next
  dispatch. You route each finding by judgment: answerable by investigation → a native `Explore`
  pass, never the user; a genuine judgment call → ruled here; bigger than the run was framed →
  escalate. An in-loop gate, never the done-condition.
- **G3 acceptance** — evidence: your clearing verdict on `spec.md` + `advocate-report.md` (story
  and FR counts, outstanding clarifications) · rules: the user · decides: **accept** (done) /
  **amend** (the requested changes become the gap list; still bounded, and clear a verdict again) /
  **reject** (abort; the draft stays under `.mochiko/specs/<feature>/`).
- **Escalation** — evidence: a cap trip, a gap set unchanged round-over-round, the kill-switch, or
  a `critical-gaps` verdict · rules: the user, on the last gap list plus the stop reason · decides:
  give-guidance-and-retry / accept-with-noted-gaps / abort — **the run stays FAIL unless the user
  explicitly accepts**.
- **Floor gates:** the run-start weight card · **G2**'s user ruling · **G3** · **Escalation**
  · and **G1** on its two user-ruled limbs — the empty-`$ARGUMENTS` ask and the governance-absence
  surfacing, never auto-resolved — the user's whatever you compose, never departable. G1's
  rich/sparse triage is yours, and departable. Batch rulings into the fewest checkpoints that
  respect these gates. No lead-penned surface takes a standing cold grade here: the uncertainty
  carrier is producer-authored, and enrichment lands in the producer's brief rather than in an
  artifact of yours — were you to pen a deliverable surface, it would take one cold-seat grade
  non-discretionarily, waivable only by recorded user waiver at the weight card.
- **Bounds:** cap **3** rounds, you count them; no-progress exit when the gap set is unchanged
  round-over-round; kill-switch — stop and escalate if `.mochiko/specs/<feature>/SPECIFY_STOP`
  exists, checked before each seat send; out of rounds = escalate, never done. Any bound this run
  declares — including a declared cost range — has you as its named counter, **rises only at a
  user checkpoint**, and is re-declared only on the record; busting a bound escalates, never
  silently continues.
- **Enrichment** *(sparse input only)* — **yours, inline, and once.** Invoke
  `Skill(mochiko:analysis-iterative)` to surface Who / Problem / Value and the feature shape
  interactively, and carry the enriched description forward in-session into the producer's brief —
  pre-loop only; the loop's own critique drives later rounds, so never re-enrich. **It is
  lead-inline because enrich-or-not is loop-entry triage: it conditions the input, and it neither
  authors nor grades** — handing it to the producer would have that seat author from input it shaped
  itself (`KEPT:` survivor).
- **Loop invariants:** hold every revision targeted — fix the flagged gaps, don't regress passing
  sections. **No devolved branch** — the critique is a judgment grade, never all-deterministic-CLI,
  so no gate is skipped and every verdict is yours.
- **Report hygiene:** round reports are cleaned by default (their outcome stamps live in the
  deliverable); the user may ask to retain them. **Never offer to delete `spec.md`** — it is the
  deliverable.
- **Ground rules:** kernel-free — no brain code, no capability catalogs, no DAG-mediated
  orchestration. Suggest commits; never run git mutations, never push. No internal machinery
  vocabulary in user-facing prose — the conversation is yours and the user's, in the mochiko
  register (`templates/output-style.md`). User acceptance is plain blocking text, never a timed
  prompt. The deliverable is written as the work progresses, never reconstructed at the end.

## Bindings

- **Artifacts** under `.mochiko/specs/<feature>/`, `<feature>` a kebab-case slug derived from the
  clear description (`mkdir -p` it): `spec.md`, producer-authored from `templates/spec-template.md`
  which you seed — P1/P2/P3 stories, FR-XXX requirements, SC-XXX criteria, no placeholder tokens ·
  `analyst-report.md` from `templates/analyst-report-template.md` · `advocate-report.md` from
  `templates/advocate-report-template.md`, carrying the critic's gap IDs.
- **Uncertainty carrier:** producer-authored — the spec template's **Assumptions** and **Open
  Questions** sections, not confidence marks.
- **Fact route:** `spec.md` and the reports themselves; a knowledge gap goes to a native `Explore`
  pass.
- **Governance brief:** where the region is present, governance reaches the producer natively at
  spawn — add to its brief the one-line **obligated read** naming the `.claude/rules/mochiko/` files
  relevant to what it authors (`paths`-scoped rules do not fire for from-scratch authoring).
- **Run-start declaration:** one line on `spec.md`'s `Status` header — where Recovery already notes
  the resume stage — for a default run; a run that departs from the stated default, or declares
  non-default bounds, writes a departure record at
  `.mochiko/specs/<feature>/specify-contract.md` beside the reports instead — the
  done-condition and bounds as (re-)declared, departures taken, and the counter state Recovery
  reads on resume. Counted unit: the
  **round**, the unit the Bounds already count.
- **Departure trail:** one line per departure from the stated default, appended under that same
  `Status`-header declaration as it is taken and carried into G3's evidence — never your context
  alone; the trail names the grading that actually ran. Departure is by record, never by silence.
- **KM landing:** `.mochiko/memory/knowledge-management.md` exists → run its landing ritual +
  invariants under fix-on-sight, and mint new domain terms into `GLOSSARY.md`. No copy → skip.

## Recovery

Note the resume stage on `spec.md`'s `Status` header line, with the run's counter state — rounds
consumed · bounds declared · departures taken. Sessions and teams do not survive `/resume`, and a
shared account limit can throttle the team and the main session together — escalation then has
nowhere to go but pause. Resume from workspace evidence, never a context `phase` field, respawning
only what the stage needs — a respawned producer re-reads `spec.md` + the gap list, and a respawn
is cold by design.

| Evidence | Resume at |
|----------|-----------|
| no `.mochiko/specs/<feature>/` | G1 triage |
| `spec.md` still the bare seeded template | loop (produce, round 1) |
| `spec.md` authored, no `advocate-report.md` this round | loop (critique) |
| `advocate-report.md` `needs-revision`/`critical-gaps`, within the cap | loop (produce) |
| `advocate-report.md` `ready`, not yet accepted | G3 |
| accepted | finalize — report the deliverable + the two reports, the round count, story / FR counts, a suggested commit (`docs: specify <feature>`), next step `/mochiko:plan` |
| `SPECIFY_STOP` present | escalate |
````

</details>

---
## [v0.46.0] Doctrine-purge rewrite — obligated reads out, shape mechanics inlined
- **Disposition:** superseded → the command's own text
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** the preamble's obligated reads "Read `${{CLAUDE_PLUGIN_ROOT}}/templates/command-shape.md` (both layers) and `mochiko:loop-discipline` before anything else" and the "team-form command in the mochiko command shape" framing left; the gap-routing taxonomy in G2 ("A preference gap is ruled here; a knowledge gap routes to a native `Explore` pass...; a scope gap escalates") reworded to plain lead-judgment routing; the floor-gate slot reference "P11 is producer-authored" reworded to plain words.
- **Kept deliberately:** every gate, bound, floor gate, binding and recovery row — and the absorbed shape mechanics now stated inline: weight-card factors, bound-integrity rule, checkpoint batching, cold-grade rule, team transport + roster verification, seat lifecycle, mesh hold, ground rules, counter-state recovery.
- **Consumers assessed:** none — a command has no downstream consumers.

---
**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] KEPT survivor's evidence pointer (lead-inline enrichment triage)
- **Disposition:** superseded → the pointer lives here; the `KEPT:` marker and its claim stay in
  the command per the amended P9.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above), executed under the lead's stage-A ruling (option (a)).
- **Content (verbatim, the pointer only):**
```
 — evidence: `.mochiko/strips/specify.md`
```
- **Kept deliberately:** the whole survivor line — enrich-or-not is loop-entry triage, lead-inline
  because it conditions the input and neither authors nor grades, and handing it to the producer
  would have that seat author from input it shaped itself. `(KEPT: survivor)` still marks it.
- **Tier-2 evidence (preserved for the verify-path):** this note, the entries below.

## [v0.43.0] The `<!-- shape-form: v7 -->` marker retired from the preamble
- **Disposition:** superseded → deleted. The marker was added by this same version's conversion
  entry below and retires in the same version, at the wave close.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-01 wave-close
  ratifications row, *shape-form marker retirement when the last command converts*; the trigger
  was written into the marker clause itself). **Ground and full record:**
  `.mochiko/strips/command-shape.md` [v0.43.0 wave close], entry 1 — *The form marker and its
  Conformance bullet retired* — not restated here.
- **Content (verbatim):** `<!-- shape-form: v7 -->`
- **Kept deliberately:** the entire preamble otherwise — goal line, obligated reads, probe seat —
  and every P18–P20 binding the marker used to gate. The slots bind unconditionally now; nothing
  the marker declared was lost, because the marker declared only which grading branch to take, and
  there is one branch.
- **Consumers assessed:** `validation-command-shape` check 20 was the sole grep consumer and its
  form branch retired in the same ceremony. All six commands swept together — a marker left in any
  one of them would be the only file in the library still declaring a form.
- **Measured:** `commands/specify.md` **9,109 → 9,084 B** (−25). Derived figures in this note's
  conversion section re-measured accordingly, superseded values kept inline.

# v0.43.0 — the v6→v7 conversion (R21's **light site**)

**Wave context:** shape **v7** landed at v0.40.0 (`lead-owned-process-flexibility`,
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`; `DECISIONS.md` 2026-08-01 — the
lead-owned-process-flexibility row plus the shape-v7 wave-close ratification row), with **D4**
ruling **convert-on-touch** and all six commands staying v6-form. The first conversion
(`implement`, this wave) is **audit-cleared** and is this conversion's precedent
(`.mochiko/strips/implement.md` [v0.43.0]); the user **widened the wave to all six commands on
2026-08-01**, so specify converts here rather than at its next touch. BACKLOG:
"convert-on-touch residuals".

specify is also **R21's light site** — `lead-owned-process-flexibility` **R21** carries a
recorded-open obligation for *a measured cost estimate for declaration + trail + composition on one
light and one heavy run* (verify N3, narrowed by **A3** to the estimate alone). The heavy site was
measured at implement; the light-site figures are the final section of this entry, and
`.mochiko/strips/command-shape.md`'s R21 status paragraph is updated to match.

**Post-conversion measurement, all blocks, body-only in words** (`## Heading` lines excluded, per
check 6): preamble **90/130** (published as 94 while the 4-word form marker stood;
retired at the wave close) · Goal **60/150** · Seats & checks **147/190** (unchanged) ·
Constraints **560/750** · Bindings **224/266** · Recovery **120/158** (unchanged). Term derivation
as check 6 requires: **G = 5** — the four prior gate lines (G1 entry · G2 clarification · G3
acceptance · Escalation) plus the run-start weight card, all five carrying the complete three-part
`evidence:`/`rules:`/`decides:` form — so Constraints is 90·(5+2) = 630 **plus the +120 P18 term**
= 750. **S = 2** and **R = 7**, both unchanged, and **no P17 lifecycle line** (see the judgments
below), so Seats & checks keeps its un-augmented 100 + 45·2 = 190. **A = 3**, unchanged
(`spec.md` · `analyst-report.md` · `advocate-report.md`), so Bindings is 90 + 12·3 + 30
(KM/`GLOSSARY.md`) **plus the +110 P19/P20 term** = 266.

> **Preamble counting basis, stated because this figure is the one that drifts.** Check 6 excludes
> a block's `## Heading`; the preamble has no `##` heading, so the question is what else counts.
> The figure published here — **90** — is **title-included**, the basis this
> wave's precedent uses ("The 'preamble 114/130' figure above counts the `# ` title line; strict
> body-only is 103", `.mochiko/strips/implement.md` [v0.42.0]). Strict body-only is **85**; the
> `# Specify` title adds 5. *(Published as **94** while the 4-word form marker stood; the marker
> retired at the wave close, so this is re-measured, not re-based.)*
>
> **Reconciled against this file's own prior figure, because a mid-wave note reported it wrong in
> the other direction.** It was relayed that the [v0.35.0] headline was measured title-*excluded*,
> making the real baseline ~8 w higher than published. **It was not:** that entry's "preamble
> 90/130" is exactly 85 + 5, i.e. already title-included, and the pre-conversion file carried no
> marker to count — verified against the HEAD baseline this run. So the v6→v7 preamble delta is
> **90 → 94, +4 w, the marker and nothing else**, and no correction is owed to the [v0.35.0]
> figure. The finding the relay *did* surface is real but different: this note's own first draft
> published the strict **85** as its headline, off-basis from both the precedent and this file's
> prior entry. Fixed above. No preamble text was trimmed — the ceiling is not in play at **90/130
> (30.8% headroom)**, the marker-era 94/130 (27.7%) being superseded by the wave-close sweep, and a
> trim would have owed its own supersession entry.

> **The A-term judgment, carried from the precedent rather than re-derived.** P19 names
> `specify-contract.md` as a **departing** run's per-run carrier. It is **not counted in A**: it is
> neither a deliverable nor a round report, and it exists only on a departing run. Counting it
> (A = 4) would raise the Bindings ceiling to 278 and so only loosen the check — the conservative
> reading is the one measured here, matching implement's recorded call.

**Neither v7 ceiling term is re-keyed, and the light site is the evidence the calibration asked
for.** The grader's calibration clause states its basis as one v7-form body — the library's
heaviest command — and predicts that "a typical P18 should measure well under" implement's 100 w,
implement's carrying the ruled verification-depth floor no other command has. Measured here: P18 is
**74 w** against the **+120** term, and the P19 + P20 pair is **79 w** (52 + 27) against the
**+110** term. Both fit with room, which is explicitly *not* a re-key case — check **6**'s
calibration clause, which owns that sentence: "Re-key either term only if a later conversion lands
a materially larger binding; a conversion that merely fits is not a re-key case." (Check 20 states
the same rule in its own words; the quoted sentence is check 6's.) The prediction holds and the
terms stand unchanged.

## [v0.43.0] The Goal's end state loses the critic's recommendation and the lead's read *(clause corrected at the audit's fix round 1)*
- **Disposition:** superseded → rewritten in place as artifact state, **conditionally phrased**.
  Both clauses collapse into the one condition they were evidence for: `spec.md` carries **no
  blocking gap left open against it**, alongside **the round reports for the grading that actually
  ran**.

  > **Corrected at the audit's fix round 1 — the first rewrite failed check 23, and the reasoning
  > that produced it is superseded here rather than quietly dropped.** The delivered clause read
  > *"with both round reports written and no blocking gap left open against it in
  > `advocate-report.md`"*, and this entry defended `with both round reports written` as artifact
  > state: *"It is artifact state (the two reports exist), not a round count, and it is the
  > v0.35.0 wave's own logged addition to P3."* **That defense is wrong, and the ground is worth
  > stating because it generalizes.** Artifact state is only a safe done-condition while the
  > artifact set is **fixed**; v7 makes the set **composed**. Under D1/U2 the critic pass is a
  > departable default, and specify's P18 does not floor it — so a run that composes the critique
  > out, takes its trail line, and is ruled at the weight card produces no `advocate-report.md` at
  > all. The old clause made that legitimate run **unable to reach done**, which is exactly the
  > obligation-reimposition check 23 exists to catch: naming a *fixed* artifact set is the same
  > defect as naming a round count, wearing different clothes. The repair is the wave's common
  > phrasing — the end state names **the grading that actually ran**, which is also floor
  > invariant 4's own language, and drops the `in \`advocate-report.md\`` anchor that presupposed
  > the report exists. Cost: **+3 w** (Goal 57 → 60, ceiling 150).
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, ratified at **A4**, 2026-08-01: *"Goal
  blocks lose process residue. Done = artifact state + floor compliance + user acceptance"*; graded
  by `validation-command-shape` check 23, v7-form only). "The critic recommends `ready`" is check
  23's named `the validator returned PASS` class verbatim; "you Read … and confirm" is the lead's
  own choreography.
- **Content (v6, verbatim — the two clauses that left):**
  ```
  the critic recommends `ready` grounded in the file; you Read
  `spec.md` + `advocate-report.md` and confirm no blocking gap remains;
  ```
- **Kept deliberately:**
  - **The lead's read of both artifacts, verbatim at its ledgered home** — G3's evidence clause,
    untouched this wave: "your clearing verdict on `spec.md` + `advocate-report.md` (story and FR
    counts, outstanding clarifications)". The read did not become optional; it stopped being a
    *done-condition element*.
  - **`grounded in the file, never the producer's report`** — untouched in the critic's seat row,
    which is where the v0.35.0 wave put it: "grades `spec.md` **from the file, never the producer's
    report**".
  - **The three-way status itself** (`ready` / `needs-revision` / `critical-gaps`) — untouched in
    the critic's seat row, in the Escalation gate's evidence (`a critical-gaps verdict`), and in
    the two Recovery rows that key on it — as *workspace evidence for resuming*, never as a
    done-condition.
  - **The round reports as an end-state element** — kept, but **re-phrased conditionally at the
    audit's fix round 1**: `with both round reports written` → `with the round reports for the
    grading that actually ran`. The v0.35.0 wave's logged addition to P3 (the artifact set naming
    the reports, not only `spec.md`) survives; what left is its unconditional *both*, which
    presupposed a critique that v7 makes departable. See the correction note above.
  - **"G3 acceptance has cleared" and "the KM landing ran"** — both are explicit end-state elements
    in the shape's own Goal spec (user acceptance; the KM landing under fix-on-sight), so neither
    reads as residue.
- **Consumers assessed:** not a shared primitive. Two cross-file consumers checked: the grader's
  check 23 (this is the text class it was written for — `.mochiko/strips/validation-command-shape.md`
  [v0.40.0]), and the wave's other commands, whose Goal blocks this entry does not touch — the
  residue clause is per-file, each command's own seat de-residues its own Goal under its own strip
  entry, and a command that has not yet converted stays v6-form and fully conformant meanwhile
  (shape note [v0.40.0], *The Goal block's process residue left the end state*, Consumers assessed).
  Written this way deliberately: a count of how many commands were still v6-form as I wrote would be
  false by wave close, this wave converting all six in parallel.

## [v0.43.0] The not-done state ``a critic status short of `ready` ``
- **Disposition:** superseded → deleted from the Goal. Nothing replaces it: the artifact-state
  condition it stood in for is the sibling state `a blocking gap open`, which is untouched.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above). It made the validator's
  returned status a done-condition element — the exact inversion check 23 fails, since under
  D1-as-amended whether the default critique runs at all is the lead's to compose.
- **Content (v6, verbatim):** ``a critic status short of `ready` `` — the trailing space inside the
  double-backtick fence is a markdown-fencing artifact, not part of the superseded text.
- **Kept deliberately:** the status triple and its routing, at all four homes named in the entry
  above; and the Escalation gate's `critical-gaps` trigger, which is the one place a critic status
  still *forces* something, and forces a **user** ruling rather than a done-condition.
- **Consumers assessed:** as above — not a shared primitive.

## [v0.43.0] The not-done state `the critic's status taken as the gate without your read`
- **Disposition:** superseded → deleted from the Goal. The rule it carried is unchanged at its two
  standing homes, neither touched this wave.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above). It is a lead-choreography
  clause ("without your read") in a done-condition; and it can never be rescued as a floor gate,
  because the lead's read is not a gate any line rules — check 21's floor-gate test keys on
  `rules: the user`, and no gate in this file rules the lead's own read.
- **Protected content, leaving by ruling and named as such:** this line is the recorded survivor of
  the [v0.35.0] entry *Three check-8 marker restatements removed from the Contract and Recovery*,
  which superseded "the critic's status is **input, never the gate**" and logged its replacement
  explicitly — *"The rule survives, the phrasing does not: the Goal's not-done states carry 'the
  critic's status taken as the gate without your read'."* Removing it is therefore a change to a
  recorded home, superseded by a cited ruling rather than dropped silently.
- **Kept deliberately — the rule, at both homes, re-read this run to confirm they hold it:**
  - the **validation-model line**, untouched: "The critic's output is **lead-adjudicated input**
    (the `review-*` family boundary) and every verdict is yours."
  - **`command-shape.md` Layer 2 *Clearing*** — the doctrine home the v0.35.0 entry relocated the
    phrasing to, and an obligated read of this command ("wherever judgment exists, the verifying
    seat's status is **input, never the gate**").
  - and **G3's evidence clause**, which names the lead's clearing verdict over both artifacts.
- **Consumers assessed:** as above — not a shared primitive.

## [v0.43.0] The Goal's amend sentence relocated into G3's decides-clause
- **Disposition:** superseded → **relocated** into the gate that already ruled it. G3's `amend`
  branch now reads "(the requested changes become the gap list; still bounded, **and clear a
  verdict again**)".
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above) — a done-condition sentence
  describing loop re-entry and a fresh verdict is process residue by construction. It is also the
  precedent's shape: implement carries exactly this content in **G5**'s decides-clause, never in
  its Goal ("amend (the changes become the failure list; re-enter the relevant cycle or fix pass,
  still bounded, and clear a verdict again)").
- **Content (v6, verbatim):** `A G3 **amend** re-enters the loop and must clear a verdict again.`
- **Kept deliberately:** **both halves of the responsibility, in one place.** Loop re-entry under
  the cap was already in G3's own text ("the requested changes become the gap list; still
  bounded"); the fresh-verdict half is what this relocation adds there (+5 w), so no clause of the
  superseded sentence is lost. Recorded as a relocation rather than a pure addition precisely
  because those five words are a survivor, not new content.
- **Consumers assessed:** as above — not a shared primitive.

*Pure additions this wave, riding the decision row rather than these entries:*

- **The form marker** `<!-- shape-form: v7 -->` in the preamble — check 20's branch key.
- **The run-start weight-card gate line** (P7) — U1-A's standing user stop, in the three-part
  countable form, taking **G from 4 to 5**.
- **`**Floor gates:**`** (P18) — the floor set (the run-start weight card · **G2**'s preference
  ruling · **G3** · **Escalation** · **G1** on its two user-ruled limbs) with the departable
  remainder named, so the absence is stated rather than inferred; and the lead-penned-surface
  element stated as an **absence**, specify's P11 being producer-authored.
- **`**Run-start declaration:**`** (P19) and **`**Departure trail:**`** (P20) in Bindings — the
  declaration on `spec.md`'s `Status` header for a default run, an instantiated
  `specify-contract.md` for a departing one, and the **round** named as the counted unit (check 22),
  the same unit the Bounds already count.
- **One new not-done state** — `a departure with no trail line`, the honest-trail invariant made
  visible in the Goal as floor compliance.

**Four judgments made here rather than deferred, flagged for the grader.**

1. **The floor-gate set is all five gates — four whole, G1 on two scoped limbs — and the ground is
   *who rules*.** specify has **no `rules: you` gate at all**: unlike implement, it runs no
   deterministic verification and declares no devolved branch, so every one of its five gate lines
   reads `rules: the user` and the who-rules test returns floor on each. Marking fewer would
   reproduce the contradiction the precedent's audit caught at fix round 1 — a `rules: the user`
   gate sitting in a not-floor list whose own sentence defines floor as never-departable.
   - **G2 is marked floor on its narrow limb only** — the *preference ruling* is the user's under
     floor invariant 1; *when* it is presented stays the lead's under **D3**'s consolidation
     authority, which is home doctrine and is deliberately not restated in the command. This is
     the precedent's treatment of implement's G3, which is the same gate.
   - **Why G1 is floor here although implement's G1 was not.** implement's G1 cleared on a
     structural ground: the package gate is floor and its evidence is the resolved feature's
     package, so a lead composing out G1's confirm still puts the resolution in front of the user
     on the very next gate. **specify has no such downstream carrier** — it is the pipeline's entry
     command, gating no upstream package. Its two user-ruled limbs are the empty-`$ARGUMENTS` ask
     (protected at [v0.37.0]: "specify is an entry command with no feature to fall back to, so the
     ask is legitimate on its own merit") and the governance-absence surfacing, whose own text
     reads **never auto-resolved** — the same clause, on the same gate content, that the implement
     audit *moved into* the floor set. Excluding it would repeat a corrected error.
   - **What is scoped out, and why it is not a hedge:** G1's *rich/sparse triage*. The Enrichment
     invariant already declares enrich-or-not to be the lead's loop-entry triage, and enrichment
     itself is an interactive user-facing session — so composing the triage differently removes no
     user ruling, which is exactly the departability test.
2. **The declaration and the trail share `spec.md`'s `Status` header.** That header is already the
   lead's own line on a producer-authored file — bound there by the v0.35.0 Recovery block and
   audit-cleared since — so a resumed lead finds declaration, departures and resume state in one
   place, and P19/P15 name one surface rather than two. **`advocate-report.md` was rejected as a
   home:** it is critic-owned and rewritten every round, the same overwrite hazard that made the
   precedent reject `cycle-report.md`.
3. **The counted unit is the `round`, and no P17 lifecycle line is owed.** specify's Bounds count
   exactly one thing ("cap **3** rounds, you count them"), so the lifecycle denominator is
   unambiguous from the file — the condition that forced implement's P17 override (three different
   counters) does not hold here. Under check 12's carve-out an unbound P17 states nothing at all,
   so nothing was written, and Seats & checks keeps its un-augmented 190 ceiling.
4. **`specify-contract.md` is not counted in A** — the precedent's recorded judgment applied, not
   re-derived; see the note above the entries.

**Recovery left untouched, deliberately.** The shape's v7 counter-state clause is home doctrine;
specify's pause line — "Note the resume stage on `spec.md`'s `Status` header line" — does not
contradict it and names the same surface P19 binds, so no edit was owed. Same call as the
precedent.

### R21 light-site measurement — the light half of the estimate closes

**R21's obligation** (`lead-owned-process-flexibility` OQ-1, verify N3, narrowed by **A3** to the
estimate alone) named *one light and one heavy run*. implement took the heavy site at this same
wave; specify is the light site — a twice-stripped entry command (v0.13.0 and v0.31.0) with the
library's smallest protected surface. Figures are `wc`-measured after the last edit. **The one
offsetting saving is real and is the largest of the conversion:** the D6(b) Goal strip returned
191 B against 1,434 B of additions.

**File growth.** `commands/specify.md` **7,866 → 9,084 B** (+1,218; words 1,095 → 1,269, +15.9%).
Attribution, each construct measured on its own text — the columns reconcile to the file delta
exactly:

| construct | bytes | words |
|---|---|---|
| ~~`<!-- shape-form: v7 -->` marker~~ — added here, **retired at the wave close** | ±0 | ±0 |
| run-start weight-card gate line (P7) | +276 | +45 |
| `**Floor gates:**` — floor set + P11 absence (P18) | +514 | +74 |
| `**Run-start declaration:**` (P19) | +414 | +52 |
| `**Departure trail:**` (P20) | +178 | +27 |
| G3 decides-clause, the relocated verdict half | +27 | +5 |
| Goal block, D6(b) residue strip | −191 | −29 |
| **net** | **+1,218** | **+174** |

**Light vs heavy — the comparison R21 wanted both sites for.** Set against implement's
14,502 → 16,021 B (+1,519):

| | heavy (`implement`) | light (`specify`) |
|---|---|---|
| conversion delta | +1,544 B | **+1,243 B** |
| …as % of the command | 10.6% | **15.8%** |
| P18 binding | 100 w | **74 w** |
| P19 + P20 binding | 88 w | **79 w** |
| Goal strip (offset) | −67 B | **−191 B** |
| departing run's contract, vs the command's own pre-conversion size | 38% | **71%** |

**A third conversion corroborates the trend, and the series is stated on one declared basis.**
`slice` also converted this wave. All three rows are **byte deltas against each command's HEAD
baseline** — the basis this whole section uses, since the per-construct attribution above is
byte-attributed. The series is monotone in the command's own size, which is the finding below
stated as data:

| command | pre-conversion size | conversion delta | as % of the command |
|---|---|---|---|
| `implement` (heavy site) | 14,502 B | +1,544 B | 10.6% |
| `slice` (corroborating) | 8,715 B | +1,214 B | 13.9% |
| `specify` (light site) | **7,866 B** | +1,243 B | **15.8%** |

> **Basis note, so slice's two published percentages reconcile at a glance.** slice's own note
> reports that conversion as **+14.6%**, which is its **word** delta (1,230 → 1,409 w;
> 179/1,230 = 14.55%); the **13.9%** above is the same conversion's **byte** delta
> (1,214/8,715). Both are correct on their own basis — this note carries the identical duality for
> specify (**+16.3%** words, **+15.8%** bytes) — and the trend is the same either way. Recorded
> because the two numbers otherwise read as a disagreement: they are one measurement in two units,
> not two measurements.

**The finding, and it is the one a single site could not have produced: the v7 constructs are
near-constant in size, so the lighter the command, the heavier the conversion in relative terms.**
Four of the six constructs land within ~20% of their heavy-site counterparts — the marker is
byte-identical, and P20 is 5 B *larger* here — because each states a fixed thing (a home, a counted
unit, a floor set) whose length is set by the shape, not by the command. The absolute bill is
smaller at the light site and the percentage bill is half again larger.

**Per-run read cost.** specify.md is an obligated read once per run, so this is **+1,243 B on every
specify run** — 15.8% on top of the command itself, against 10.6% at the heavy site. It is not the
whole delta a run pays this wave: the shape-home clause adds **+450 B to `command-shape.md`**
(31,816 → 32,266), and *that* one is paid by **every team-form run of any command**, converted or
not, the shape home being the shared always-read floor — it is recorded once, at
`.mochiko/strips/command-shape.md` [v0.43.0], and is not double-counted as specify's. Against v7's
own doctrine cost (+11,399 B/run, measured at v0.40.0), conversion remains the small half of the
bill at both sites.

**Run-time cost of declaration + trail — an estimate, and marked as one.** Three components, none
yet observed on a live run; the per-component figures mirror the heavy site's, because the
components are the shape's, not the command's:

- **The declaration, every run.** One line on `spec.md`'s `Status` header stating the four-factor
  read and the composed process. At the density this repo's own cards use, ~30–60 words
  (~200–400 B), produced once and re-read on every resume. It is the only one of the three a
  **default** run pays.
- **The trail, per departure.** ~15–25 words (~100–170 B) a line. A run that takes the stated
  default pays **zero**, and the cost scales with departures — the intended shape: the lead buys
  flexibility by the line.
- **The contract, departing runs only.** `templates/workflow-contract.md` measures **5,572 B**
  today, so a departing specify run reads 5.6 KB and writes a filled copy of comparable size to
  `.mochiko/specs/<feature>/specify-contract.md`. The largest run-time item by far, and
  **conditional by construction** — no default run touches it. **This is where the light site
  diverges most sharply:** that template is 71% of specify's own pre-conversion size (38% at
  implement), so on a light command the departure carrier, not the conversion, is the dominant
  cost — the first place to look if the estimate is ever revisited against live-run evidence.

**The honest read at this site.** A default specify run pays the +1,243 B read plus one declaration
line — **~1.5 KB**, against the heavy site's ~1.8 KB. A departing run adds ~5.6 KB of template plus
its fill, plus a line per departure. **Both R21 sites are now measured**, so the estimate obligation
is discharged pending only a live-run confirmation; `.mochiko/strips/command-shape.md`'s R21 status
paragraph is updated to say so.

---

## [v0.37.0] `@`-reference drop-bug attribution removed — the bug is resolved
- **Disposition:** superseded → user ruling (2026-08-01). Only the bug-cause parenthetical retires; the empty-args ask is fully kept.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/decisions/2026-08-01-at-reference-recovery-superseded.md`; `DECISIONS.md` 2026-08-01).
- **Content (superseded, verbatim):** the parenthetical "(the known `@`-reference drop bug)" inside G1's "Empty `$ARGUMENTS` (the known `@`-reference drop bug) → ask the user to re-enter it, or proceed and enrich from scratch."
- **Kept deliberately:** the whole empty-args recovery — "Empty `$ARGUMENTS` → ask the user to re-enter it, or proceed and enrich from scratch." specify is an entry command with no feature to fall back to, so the ask is legitimate on its own merit; only the resolved-bug attribution is gone.
- **Consumers assessed:** five-command recovery — see the shared consumer list in the `strips/plan.md` v0.37.0 entry; specify/brainstorm remove attribution only, plan/implement/slice keep a detected-feature confirm.
- **Protected-set note:** as recorded in the plan entry — record §7's protection premise for this recovery is spent now the bug is resolved; deliberate supersession, not a check-14 re-drop.

# v0.35.0 — the goal-shape rebuild wave (CS-D10 step 4)

**Wave context:** command goal-shape rebuild, **step 4 of 4** — the five-command wave after the
audit-PASSed plan pilot (design: `.mochiko/brainstorms/command-succinctness-strip/record.md`,
CS-D3/D4/D5 + D8 + D10; `DECISIONS.md` 2026-07-30 rows). Authored against **shape v5** with the
obligated `loop-discipline` read **retained** — the drop is deferred to a named live-run trigger
(pilot-checkpoint ADR `.mochiko/decisions/2026-07-30-goal-shape-pilot-checkpoint.md`), so a v5
command that omits it is non-conformant, not early. specify declares the **in-loop critique
branch**, so it must not reference `sized-end-stage-review.md` — check 1's negative direction; it
does not (see the phrasing entry below, which removed the one near-miss).

**Baseline provenance — read this before auditing the ledger.** The working tree held a partial
rewrite by a since-stopped seat executing a superseded instruction. **This ledger is derived from
`HEAD` (the authoritative 146-line baseline), not from that draft**, and every one of HEAD's 146
lines was walked clause by clause against the delivered file. The audit found **one genuine
fidelity gap** the inherited draft had introduced and this wave restored: HEAD listed the enriched
description as an explicit producer-**brief** field, which the draft had compressed to "carry the
enriched description forward in-session" with no destination named. The delivered Enrichment
invariant now reads "forward in-session **into the producer's brief**". Mechanical backstop for the
same class: every backticked token in HEAD was diffed against the delivered file, and each of the 11
absences is an accounted relocation, a namespace-prefix convention the pilot already set
(`mochiko:requirements-analyst` → the table's bare `requirements-analyst`), or deleted loop
arithmetic (`round = 1`, `round += 1`).

**Measured: 1,273 → 1,100 words (−13.6%), 9,390 → 7,901 B (−15.9%)** — `wc`-measured after the
re-derivation fix round landed, per the pilot's standing habit. Against the pre-wave measured floor
of 991 w: **+109 w (+11.0%)** — over, not under, which is the safe side of CS-D8 (landing materially
*under* a floor row would signal dropped content). The overage is accounted, measured not estimated:
the P4 not-done states are new v5 content the floor row's arithmetic did not carry (**44 w**), the
Recovery table's `accepted` row absorbed the whole Finalize paragraph rather than dropping it
(**26 w**), the `KEPT:` survivor's evidence pointer is newly bound (**8 w**), the restored
brief-destination clause above (**6 w**), and the residual **25 w** is the four gate lines'
three-part `evidence · rules · decides` scaffolding, which the flow prose carried implicitly in
sentence form.

**Run-level, stated honestly because specify is the first command where it does not flatter the
rebuild:** the file drops 1,489 B, while the v5 shared read floor **adds 3,225 B** per run — read
set 29,611 → 32,836 B (`command-shape.md` 12,502 → 16,735 = +4,233; `agent-dispatch.md` 5,183 →
4,175 = −1,008; `loop-discipline/SKILL.md` 11,926, unchanged; `sized-end-stage-review.md` is *not*
in specify's read set). So a specify-only session is net **+1,736 B** against v4. Two readings, both
true: (a) charged per run, the goal-shape rebuild is
net-negative on an already-twice-stripped light command — specify was cut at v0.13.0 and again at
v0.31.0, so there was little narration left to delete; (b) charged once across the surface, the
shared floor is a surface-wide investment that plan's −19,749 B alone repays roughly six times
over. Not a reason to cut protected content (CS-D8 forbids it), but the wave ceremony should have
the datapoint: **the anatomy pays for itself on heavy commands and is byte-neutral-to-negative on
light ones.** Conformance, not a percentage, is CS-D2′'s success criterion — that is what this file
is delivered against.

**Correction to a pilot figure:** plan's v0.34.0 note records the shared read-floor delta as
**+2,895 B**. That was correct when measured at v0.33.0 (shape 16,405 B); the pilot's own commit
then grew the shape home by 330 B with the deferred-read transition note. The live figure is
**+3,225 B**. Same stale-headline cause the pilot's standing habit names — a figure measured before
a later edit landed — this time across two notes rather than within one.

> **Correction to this entry's own replacement figure, ratification round (auditor-caught).** This
> note first stated **+3,223 B**, from a baseline of 29,613. Wrong by 2 B, and the cause is worth
> recording because it is a *different* failure from the stale headline above: I anchored the v4
> baseline to `e30533e` (the commit where the *shape* went v4) instead of to `70f4efd`, the tree
> immediately before the v5 revision. `agent-dispatch.md` kept evolving between them — `21cb75e`
> trimmed it 5,185 → 5,183 — so the shape's own version stamp is not a safe proxy for the read
> set's state. The auditor's **29,611 → 32,836 = +3,225** is authoritative, and it reconciles the
> whole chain: plan's +2,895 is exactly 32,506 − 29,611, i.e. the pilot used this baseline
> correctly. **Lesson for the ceremony:** anchor a shared-read-floor baseline to the revision
> commit's parent, never to the version stamp of one file in the set.

Block sizes against the grader's ceilings (terms as the grader counts them — **G=4** gate lines,
S=2 seat rows, A=3 artifacts, R=7 resume rows): preamble 90/130 · Goal 89/150 · Seats & checks
147/190 · Constraints 441/540 (82%) · **Bindings 145/156 (93%)** · Recovery 120/158. Tightest is
Bindings, and the `+30` KM/index term is what makes it fit — without that term specify's Bindings
floor would sit at 145/126, a FAIL. Second live datapoint for confirming the term at the ceremony.

## [v0.35.0] The flow body, the seat bullets, and the Contract section retired into the five-block anatomy
- **Disposition:** superseded → the goal-shaped anatomy. `Team-form parameters`→ the preamble's
  probe line + shape Layer 2 (see the next entry) · `Session constraints`→ **Bindings** (workspace
  + deliverable + IDs) and the **Bounds** line (kill-switch) · `The seats`→ the **Seats & checks**
  table + G2 (producer clarifications) + the Enrichment/Loop invariants · `The flow`'s Triage→**G1**
  · Enrichment→ the **Enrichment** invariant · Spec loop→ the **Goal**, G2's routing classes, the
  **Bounds** line and the **Escalation** gate · Acceptance→**G3** · Finalize→ the Recovery table's
  `accepted` row + **Report hygiene** + the KM binding. The `Contract` section's four clauses →
  **Goal** (done-condition + not-done states), the **Seats & checks** table and its validation-model
  line (producer↔validator), **Constraints** (bounds + the four gate lines).
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4** the
  connective procedure is deleted and what survives is *restructured* · **CS-D5** the five-block
  anatomy and the Contract-as-document inversion).
- **Content:** five `## `-level sections of ordered procedure and appendix (`Recovery` is the sixth
  and survives, restructured). Not reproduced verbatim — every *rule* inside them is resolved
  individually in the CS-D8 ledger below, and the deleted remainder is connective narration
  (`initialize round = 1`, "loop to produce", "then apply the bounds", the round-arithmetic sentence,
  and the lead's job description restated per section). Recoverable in full at
  `git show 7898d86:plugins/mochiko/commands/specify.md`.
- **Kept deliberately:** every gate, bound, routing class, trigger, ordering rule and artifact
  binding — the ledger below resolves each one.

## [v0.35.0] The `Team-form parameters` section retired — three lines, three different fates
- **Disposition:** split.
  - "Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape" → **relocated** to
    `command-shape.md` Layer 2 (Hard requirement), which the command Reads. The user-facing
    declaration survives in `description:` — which is also what makes the file grep-detectable as
    team-form for check 1.
  - "Transport mechanics + the addressability check: `templates/agent-dispatch.md` (Seat transport)"
    → **superseded**: at v5 Seat transport was absorbed *into* `command-shape.md` Layer 2 (CS-D6),
    so this pointer named a section that no longer exists in the file it points at. A stale
    cross-reference, retired rather than re-aimed — the shape home is already an obligated read, so
    re-aiming it would restate a read the preamble mandates.
  - "The no-fallback bet is the same `Contested` dogfood-pilot ruling as the other team-form
    commands" → **relocated** to `command-shape.md` Layer 2, which carries it as "**no fallback
    transport**, a dogfood-pilot bet marked `Contested`, to be revisited when mochiko distributes
    beyond the author's machines" (read this run to confirm the home holds the content).
    DECISIONS-traceable (`2026-07-04` brainstorm-v2 row, hard-require teams / no fallback /
    `Contested`) → superseded by a home that states it, not dropped.
- **Tier failed:** 1 (altitude) for the two relocations; n/a for the stale pointer.

## [v0.35.0] The `What you own (not the seats)` footer deleted
- **Disposition:** deleted.
- **Tier failed:** 1 — a declared duplicate. Every clause now sits in a block: input triage → G1 ·
  the enrichment call → the Enrichment invariant · the loop and its counter → the Bounds line ·
  gap routing → G2's three classes · the verdict against the default-FAIL done-condition → the Goal
  and the validation-model line · the human gates → the four gate lines · "never letting producer
  and critic collapse into one seat" → the Seats table, where check 7 grades it mechanically.
- **Kept deliberately:** nothing was unique to it. The closing "Full rules: `mochiko:loop-discipline`"
  survives as the preamble's obligated read.

## [v0.35.0] Three check-8 marker restatements removed from the Contract and Recovery
- **Disposition:** superseded → their homes in `command-shape.md`. At v5 each is a *floor-FAIL
  marker* the grader greps for, so carrying them would fail the file mechanically regardless of
  prose quality:
  - "the critic's status is **input, never the gate**" (Contract done-condition, clause 2) → home:
    Layer 2 Clearing. **The rule survives, the phrasing does not:** the Goal's not-done states carry
    "the critic's status taken as the gate without your read".
  - "disjoint agents, disjoint skills, **structurally separated**" (Contract, Producer↔validator) →
    home: Layer 2 Independence by structure, which states both phrasings. The table now *shows* what
    that sentence asserted; check 7 grades it.
  - "a critic **respawn is cold by design**" (Recovery preamble) → home: Layer 2 Independence by
    structure. The Recovery block keeps the workflow-specific half — a respawned producer re-reads
    `spec.md` + the gap list.
- **Tier failed:** 1 (altitude) — each names one homed rule.

## [v0.35.0] "no sized end-stage review here" → "unsized by design"
- **Disposition:** rewritten in place on the validation-model line.
- **Tier failed:** n/a — a hazard fix, not a strip. The clause's *content* is unchanged and its
  word count barely moves; what changed is that the v4 phrasing wrapped as "no sized / end-stage
  review", which normalizes to a near-miss of check 1's negative direction for specify's branch
  (`sized-end-stage-review` must be absent). A literal grep of the path token never matched it, but
  a fuzzy one does — and pointing an auditor at a hit that is really a declaration of *absence* is
  the same false-positive class the pilot retired its `shape-exception` marker over. P6 requires the
  branch to be **named**, not the other branch to be denied, so "unsized by design" carries the
  declaration with no token to trip on.

## [v0.35.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects two sets: `KEPT:`/Tier-2-evidenced lines, **and** every
line traceable to a `DECISIONS.md` row. specify carries **one live `KEPT:` entry** (the v0.13.0
enrichment-boundary survivor, re-graded in full below) plus the *Kept deliberately* field of the
v0.31.0 supersession and the DECISIONS row trace. Grepped before any cut. **All 15 rows survive,
zero dropped: 14 translated into the file's own blocks, 1 resolved by relocation** — the
no-fallback-transport bet, whose home (`command-shape.md` Layer 2) is an obligated read of this
command and was Read this run to confirm it holds the content.

Per the pilot's step-4 instruction, the **compressed-evidence clauses were grepped rather than
trusted** — the pilot lost content inside gate lines that still read as complete. Both of specify's
are intact and verified line by line: G1 keeps the `@`-reference recovery's **named cause** *and*
its **two-option prompt** (re-enter, or proceed and enrich from scratch), and G2 keeps **all three**
routing classes (preference ruled here · knowledge → a native `Explore` pass, never to the user ·
scope escalates).

**The `KEPT:` re-grade** (the v0.13.0 entry below — the lead-inline enrichment boundary).
**Verdict: translated, not superseded. The failure still has a path in the goal-shaped file.**

The supersession this entry could have taken would rest on a structural-prevention claim, and
check 14 verifies those against the anatomy rather than the author's say-so. Tested, it does not
hold:

- The **Seats & checks** table's one hard structural rule is that no row grades its own output
  (check 7). A producer seat that *enriched* the input and then authored from it would still grade
  nothing — check 7 passes on that arrangement. The coupling this survivor guards against is
  author-from-self-shaped-input, which is not the producer↔validator collapse the table detects.
- The anatomy constrains which **class** of content each block carries, not which **actor** owns a
  given responsibility. Adding "conditions sparse input" to the producer row's produces cell, and
  deleting the Constraints invariant, would be a shape-conformant file.

**Partial prevention does exist and is worth recording:** at v4 the seats and the enrichment step
were adjacent procedural prose (`## The seats` / `## The flow`) at the same altitude, so ownership
was ambiguous by layout; at v5 they sit in different blocks with different owners, so the
*placement* is legible at a glance. But legible placement is not prevention — and the specific
pressure this survivor exists to resist is a future altitude pass reading the rationale clause as
strippable prose, which the anatomy does nothing to stop.

**Resolved:** translated into Constraints as the **Enrichment** invariant with the rationale clause
intact ("it conditions the input, and it neither authors nor grades — handing it to the producer
would have that seat author from input it shaped itself"), now additionally carrying the
**evidence pointer P9 mandates** and the v4 file never had. Boundary provenance is unchanged: the
specify port's rehome ruling (`.mochiko/transform/specify/reconcile.md`).

| protected line | source | resolved |
|---|---|---|
| Lead-inline enrichment boundary — it conditions the input, neither authors nor grades | v0.13.0 `KEPT:` (Tier-2) | **Enrichment** invariant, rationale intact + evidence pointer newly bound — full re-grade above |
| Every verdict stays the lead's; **no devolved branch** (specify has no deterministic-CLI verification, so shape D3's branch cannot apply — declared, not left implicit) | v0.31.0 *Kept deliberately* | **Loop invariants**: "No devolved branch — the critique is a judgment grade, never all-deterministic-CLI, so no gate is skipped and every verdict is yours" + the validation-model line |
| The **`@`-reference recovery** — empty `$ARGUMENTS` has a *named cause* (the `@`-reference drop bug) and a two-option prompt | `command-altitude` DECISIONS row (its retrofit-regression warning names this recovery among the hard-won fixes verbosity encodes); the class the pilot dropped and had restored under audit | **G1** decides-clause, both halves present. Grepped, not assumed — the pilot's named failure mode |
| All three gap-**routing classes** — preference ruled at the gate · knowledge → a native `Explore` pass, never to the user · scope escalates | record D5 fold (a) graded exemplar; `loop-discipline` gap routing | **G2**, all three named; the scope class lands on the **Escalation** gate's evidence. The other class the pilot dropped |
| In-loop mesh — producer hands work to the verifying seat directly; the lead is the exception handler | Team-method D1 (`Contested`) + Layer-2 mesh rewrite row | Seat table's **peer edges** column, both rows |
| Cold arrival is a property of the **stage**, not of the traffic | Team-method D2 | Critic row's spawn cell: "cold at first critique, standing after" |
| Devolved clean-cycle verdicts — and specify's **declared absence** of that branch | Team-method D3 | Declared, see the no-devolved-branch row above |
| Hard-require agent teams, **no fallback transport** (`Contested` dogfood-pilot bet) | brainstorm-v2 row (2026-07-04) | `description:` declaration + `command-shape.md` Layer 2 — see the Team-form-parameters entry |
| The `review-*` family boundary — a reviewer produces **lead-adjudicated input**, never the authoritative grade | setup-adversarial-review row (the `validation-*`/`review-*` split) | Validation-model line, once (deduped to a single site at v0.13.0 and still single) |
| Governance region is a **prerequisite, surfaced never auto-resolved**; `paths`-scoped rules do not fire for from-scratch authoring, so the producer gets a one-line obligated read | constitution-native-surfaces + governance-injection-probe rows | **G1** (surface + the two exits) and Bindings' **Governance brief** (the obligated-read line) |
| A knowledge gap routes to a native `Explore` pass — the cheap-explorer avenue, never the user | model-tiered-seats row | **G2** and Bindings' **Fact route** |
| KM landing ritual + invariants under fix-on-sight, naming the **project copy** `.mochiko/memory/knowledge-management.md` | OD-D6 (subtractive landing) + the CS step-1 adjudication making the KM reference mandatory in KM-carrying commands | Bindings' **KM landing**; check 1's KM member greps the project path, and it is the project path |
| New domain terms minted into `GLOSSARY.md` | OD-D10 (glossary joins core, `Contested`) | Bindings' KM landing |
| Round reports cleaned by default; **never offer to delete the deliverable** | current body | **Report hygiene** invariant |
| Uncertainty rides the spec template's **Assumptions / Open Questions**, not confidence marks (the shape's producer-authored branch) | current body (P11) | Bindings' **Uncertainty carrier** |

**Additions, logged rather than folded silently** (pure additions ride the decision row per the
Job-4 rule; these are within-command precision, not doctrine): the Goal's artifact set now names
"both round reports written" (P3 previously named only `spec.md`), and the flow's escalation clause
is promoted to a named **Escalation** gate line in the three-part form, which is what makes
specify's gate set countable at **G=4**.

## [v0.31.0] Lead-relayed gap lists superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place: the critic is still cold-spawned at first critique, and the producer↔critic peer edge is declared on the roster.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - producer seat: "Round > 1 is a message to the same seat carrying the critic's gap list verbatim"
  - critic seat: "spawned **cold at first critique**, never in contact with the producer"
  - Contract, Producer ↔ validator: "(critic cold-spawned, gap lists lead-routed, no producer↔critic contact)"
- **Kept deliberately (not superseded):** every verdict stays the lead's — specify has no deterministic-CLI verification, so **D3's devolved branch cannot apply here**; the Contract now declares that absence rather than leaving it implicit.

## [v0.13.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** specify's producer spans up to 3
  revision rounds plus clarification feedback — the same longer-horizon context-retention
  bet `/mochiko:setup`'s authoring loop was ruled team-form on (standing analyst seat;
  gap lists lead-routed verbatim). The critic maps to setup's validator seat: cold at
  first critique, same-seat messages after, no producer contact — independence stays
  structural. Transport rides the v3 fix (`agent-dispatch.md` Seat transport +
  addressability probe).
- **Steelman recorded:** zero successful team-form runs observed at conversion time (two
  setup defect runs; the kinako acceptance test pending); brainstorm v2 measured standing
  seats as more expensive than dispatches. Ruled team-form anyway per D2's declared
  default + S4 (no prior dogfood evidence required; checkpoint below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open
  "Dogfood `/mochiko:specify`" BACKLOG item) confirms the conversion or reverts it to
  one-shot Layer-1 form.

## [v0.13.0] Per-run contract fill (`workflow-contract.md` → `contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract section — the authoring-time-fill rule); the per-workflow values survive as the command's Contract section
- **Tier failed:** 1 (altitude — the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body."

## [v0.13.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko sound loop: invoke `mochiko:loop-discipline` and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per `agent-dispatch`. Those rules are not restated here…" — the requirement list restated `loop-discipline`'s own enumeration.

## [v0.13.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Producer ↔ validator clause; the `review-*` family boundary also lives in `review-specifications`' description + REGISTRY)
- **Tier failed:** 1
- **Content:** stated three times pre-wave — L8 ("The critic *recommends* a status; **you own the clearing verdict** — its status is input, never the gate"), L19 (Team clause "it produces lead-adjudicated input, never the authoritative grade"), L67 (footer "the advocate recommends a status *from* the spec, you Read the artifacts and decide").

## [v0.13.0] HIL done-condition comparison blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical provenance; preserved in ROADMAP's Decision Trail + `.mochiko/transform/specify/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL exited on the State-Analyst's *autonomous* verdict with no human acceptance — it could self-declare done on pass 1, violating `loop-discipline` req. 1. The advocate's three-way status survives as input to your verdict, plus the new G3 acceptance gate."

## [v0.13.0] Feature-numbering-script aside
- **Disposition:** deleted
- **Tier failed:** 2 (HIL-history note; provenance in `.mochiko/transform/specify/`)
- **Content:** "(No feature-numbering script — workspace-as-state replaces it.)"

## [v0.13.0] Spec-grammar enumeration in the produce brief
- **Disposition:** relocated → the grammar's single sources: `mochiko:authoring-requirements` + `mochiko:authoring-user-stories` + `templates/spec-template.md` (user-ratified; the brief keeps "the template to fill per those skills — no placeholder tokens", and the goal line names the deliverable's parts once)
- **Tier failed:** 1
- **Content:** "(prioritized P1/P2/P3 user stories with Given-When-Then, FR-XXX requirements, measurable SC-XXX, edge cases; technology-agnostic; no placeholder tokens)"

## [v0.13.0] Footer ground rules + one-shot transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules — homed at shape v2, this wave); the Task-tool line superseded by the team-form conversion (transport now per shape Layer 2 + `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push."

## [v0.13.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context `phase` field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`)"

## [v0.13.0] KEPT: "Lead-inline because enrich-or-not is loop-entry triage: it conditions the input, it neither authors nor grades."
- **Tier-2 evidence:** guards the lead-inline vs seat boundary — without it the natural reading is to hand enrichment to the producer seat, coupling input conditioning into authoring (the producer would then author from input it shaped itself). Boundary provenance: the specify port's rehome ruling (`.mochiko/transform/specify/reconcile.md` — enrichment landed on the lead, not the analyst).
