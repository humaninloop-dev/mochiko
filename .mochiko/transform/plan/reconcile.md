# RECONCILE — cluster `plan`

**Run:** `/mochiko:transform-cluster` (plan-core) · **Phase 2 (reconcile)** · **Date:** 2026-06-30
**Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:reconcile-cluster`
**Role:** decide relational verdicts + build the rehome map. **NOT** transform, **NOT** grade.
**Inputs consumed:** `context.md`, `contract.md`, all 15 `assess-*.md`.
**Done-condition:** zero open flags · zero homeless responsibilities · one coherent rehome map (P1 ⊕ P11).

> The two pivotal calls (RQ1 reviewer-architecture, RQ2 principal-architect role) are in the contract's
> **human-gated bundle** (§4a). This file resolves them as a **RECOMMENDATION + runner-up delta**, not a
> unilateral lock — every flag carries a concrete assigned move (so there are zero *open* flags), and the
> human ratifies/overrides the gated ones before any transform.

---

## 0. Executive summary

- **RQ1 → RECOMMEND option (i): two distinct validator agents.** `principal-architect` becomes plan's
  **feasibility reviewer** (adversarial-critique, 3-state `feasible/needs-revision/infeasible`);
  `devils-advocate` is the **completeness reviewer** (mirror-checklist via P9). Plan is the first cluster
  to run convention-5's two-form, as designed. Runner-up **(ii)** (fold feasibility into the advocate)
  is cheaper in new primitives but loses the deliberate feasibility≠completeness split, flips RQ4
  reuse→extend, and must actively guard against flattening the `infeasible`→business-escalation. Option
  **(iii)** (generic `validator`) is ruled out by the binary-vs-3-state contract mismatch.
- **RQ2 →** produce-here(setup)/review-there(plan) is decoupling-legitimate (confirmed, re-coupling risk
  LOW); the feasibility-review driver is a **new reviewing skill `validation-feasibility`** (not
  persona-only). **G1 honored:** `validation-constitution` is NOT re-mounted on `principal-architect`.
- **RQ3 →** P14 `cross-artifact-checklist` **folds into P9** (`merge-into-sibling`/`dedupe`); standalone
  template dropped (content survives in P9).
- **RQ4 →** P15 `advocate-report-template` **reused as-is** (`keep-verbatim × standalone`; no new file;
  REGISTRY re-file specify→shared). Holds cleanly under recommended (i).
- **RQ5 →** P11 `plan-context-template` **`drop × absorb-into-lead`** confirmed; under (i) its report-path
  slice resolves to **three** workspace report homes (techanalyst · feasibility · advocate).
- **RQ6 →** data-sensitivity canonical home = **P7**; x-integration canonical home = **P8**; P5↔P6
  constraints-and-decisions.md = **boundary + dedupe** (P5 owns artifact, P6 owns technique). Neither
  orphan dropped (P9 grades both Critical).
- **New primitives required:** exactly **one** — `validation-feasibility` (skill), under recommended (i).
- **Open flags: NONE.**

---

## JOB 1 — Relational verdicts

### RQ1 — reviewer architecture (THE pivotal, human-gated call) → RECOMMEND **(i)**

**The forcing fact:** feasibility review is **HOMELESS in mochiko today.** The ported `principal-architect`
(setup, producer-only) *actively disclaims* it (lines 107–109: "cross-artifact feasibility review … is
**not** your responsibility"). "ALREADY PORTED" is a trap — reusing P3 as-is silently drops plan's entire
cross-artifact-contradiction gate (P3 silent-drop #1, CRITICAL). Reconcile **must** give feasibility a home;
the only question is which.

**Recommendation: option (i) — two distinct validator agents.**

| | feasibility reviewer | completeness reviewer |
|---|---|---|
| **Agent** | `principal-architect` (re-broadened) | `devils-advocate` (re-mount) |
| **Skill (driver)** | **`validation-feasibility`** (NEW — adversarial-critique form) | `validation-plan-artifacts` (P9 — mirror-checklist form) |
| **Scope** | cross-artifact CONTRADICTION / IMPOSSIBILITY / BUILDABILITY | COVERAGE / MEASURABILITY / CONSISTENCY / PRESENCE |
| **Verdict** | 3-state `feasible / needs-revision / **infeasible**` (P13 report) | 3-state `ready / needs-revision / critical-gaps` (P15 report) |
| **When** | once, after Phase-1 analysis (skip re-review unless structural change) | both phases (Phase-2 = incremental mode) |

**Rationale (the five weighing axes the brief names):**

1. **Faithfulness to HIL's deliberate design.** HIL runs feasibility **once before** completeness
   ("don't waste time reviewing completeness of infeasible requirements", plan.md:908) and reserves a
   **distinct `infeasible` state** → a *business-level decision / escalation*, not a routine revision.
   Option (i) preserves both the sequencing and the distinct escalation cleanly; (ii) blurs the sequencing
   (one reviewer, two lenses) and risks flattening `infeasible`→`critical-gaps`; (iii) flattens to binary
   and loses `infeasible` entirely. Both plan reviewer reports are **3-state adversarial reports**, never
   binary grades — which is structurally why (iii) has friction and (i)/(ii) sit naturally.
2. **Persona fit (decisive).** "Can these pieces be built **together**?" is the architect's native
   judgment — a senior technical leader evaluating cross-artifact buildability. The advocate is a
   skeptical **gap-hunter** (a different professional disposition). Convention 4 says persona = *what the
   agent cares about*; the two reviews are two **personas**, not one persona with two procedures. HIL's
   original `principal-architect` was exactly "establishes **and evaluates** governance standards" before
   the setup port narrowed it — re-broadening restores one coherent professional. Folding buildability
   judgment onto the gap-hunter (ii) overloads one persona with two adversarial lenses and weakens both
   (P3 #4, P13 #2).
3. **Convention-5's two-form, as designed.** ROADMAP:80 — a *mirror-checklist* skill for objective
   criteria + an *adversarial-critique* skill for judgment artifacts. The two map cleanly: P9 = checklist,
   `validation-feasibility` = critique. Context §52 names plan as the **first cluster to exercise the
   two-form** — (i) realizes it as two named skills on two personas.
4. **Minimalism (the counter-pull — addressed, does not win).** P9 already overlaps feasibility
   (NFR-design feasibility, constraint-design alignment, TR-constraint contradictions, NFR-constraint
   conflicts — PHASE-CHECKLISTS.md), so (ii) is cheap. **But** minimalism governs the *body treatment a
   primitive earns*, not the *collapse of distinct responsibilities*. The reconcile merge rule is explicit:
   "merge only on a genuinely shared core; keep the variant-unique slice." Feasibility (buildability) and
   completeness (coverage) only **partially** overlap — they are not a shared core with a thin variant.
   Cheapness that loses the deliberate split and risks the `infeasible` flattening is a false economy. And
   (i) is not as expensive as it looks: it needs **one new skill and zero new agents** (PA already exists).
5. **Something must own the homeless responsibility.** (i) homes it on the persona that is its natural
   author, with full faithfulness, at the cost of one skill.

**Independence under (i) — holds by construction (attested in §Independence below):** producer
`technical-analyst` (authoring skills only) ≠ feasibility reviewer `principal-architect` ≠ completeness
reviewer `devils-advocate` (review skills only). Two independent reviewers both grade the analyst's output;
neither is the producer; the **lead** (plan command) referees and owns the clearing verdict. No agent
produces+grades the same artifact.

#### Runner-up **(ii)** — exact delta (what changes if the human overrides to fold-into-advocate)

| Aspect | Recommended (i) | Runner-up (ii) |
|--------|-----------------|----------------|
| `principal-architect` (P3) | `port-with-edits` — re-broaden to feasibility reviewer; mount `validation-feasibility` | **`keep-verbatim`** — stays producer-only; does **not** participate in plan; responsibility #7 → `moved-to-sibling-skill` (advocate) |
| New primitives | **`validation-feasibility` (skill)** | **none** if P9 absorbs feasibility (cheapest); **or** one feasibility-critique skill mounted on the *advocate* instead of the architect |
| `devils-advocate` (P4) | completeness-only (re-mount P9) | completeness **+ feasibility** (a merge-in); P9 expands to own the full cross-artifact contradiction intersection + the `infeasible` state |
| P13 architect-report | `port-with-edits × standalone` → ports as `feasibility-report` | **`merge-into-sibling`** → its unique slice (3 cross-artifact tables + `infeasible` + 4-field gate fuel) `moved-to-sibling-template` into P15 |
| P15 advocate-report (RQ4) | **reuse as-is** (`keep-verbatim`) | **EXTEND mandatory** — gains feasibility tables + must reconcile `infeasible` vs `critical-gaps`; RQ4 flips reuse→extend |
| P11 report homes | **three** (techanalyst · feasibility · advocate) | **two** (techanalyst · one merged reviewer report) |
| Standing risk | low — split is explicit | **must actively prevent `infeasible` flattening** (P3 #2 / P13 #1) and the feasibility≠completeness blur (P3 #4 / P13 #2) |

> **(iii) generic `validator`** — ruled out (recorded, not chosen): the generic validator is **binary
> PASS/FAIL by construction** (`validator.md`:73–80); feasibility is 3-state + suggested resolutions +
> AskUserQuestion-feeding. Hosting it forces either a 3→2 collapse (loses `infeasible`→escalate, capability
> loss) or breaking the validator's binary invariant. Friction confirmed by P3, P9, P13.

### RQ2 — principal-architect role & driver (rides on RQ1; resolved under (i))

- **Produce-here / review-there is decoupling-legitimate — CONFIRM.** Decoupling-by-absence permits it
  (the persona names no workflow); independence permits it (setup-produce the *constitution* vs plan-review
  the *analysis artifacts* = different artifacts, different workflows, **never** produce+grade the same
  artifact — P3 Independence table). Re-coupling risk **LOW**: the re-broaden adds **no** deny-list token
  (a senior architect who writes standards **and** reviews designs for buildability is one coherent
  professional naming no workflow), with a single trivial wiring fix (HIL line 138 "(that is the reviewer's
  job)" → role language).
- **Driver = a NEW reviewing skill `validation-feasibility` (not persona-only).** Decision rationale:
  (a) **convention 4** — the feasibility procedure (4 conflict types, 3-state verdict, in/out-of-scope) is
  definite procedure → factors into a skill; (b) **framework forward-pointers** already envision it
  (`assess-primitive/references/TRACE-TAGS.md:30`, `transform-recipes/references/RECIPES.md:53` both tag
  "feasibility review → folded-into-skill (new skill)"); (c) **independence auditability** — a named skill
  makes the disjoint producer/validator skill-sets `verify-output`-checkable, where a persona-only review
  is not; (d) **symmetry** — the completeness side mounts a named skill (P9); the two-form is realized as
  two named skills; (e) PA **already mounts skills** (`authoring-constitution`, `analysis-codebase`) — a
  review skill fits its existing composition.
- **Guardrail G1 — HONORED.** `validation-feasibility` operates over **plan analysis artifacts**, NOT the
  constitution. `validation-constitution` STAYS on the generic `validator` and is **never** mounted on
  `principal-architect`. PA's plan skill-set = `{authoring-constitution, analysis-codebase}` (setup
  producer role) ⊕ `{validation-feasibility}` (plan reviewer role) — disjoint artifact domains, no
  constitution self-grade recreated.

### RQ3 — cross-artifact-checklist (P14) → **FOLD INTO P9**

`merge-into-sibling` (→ `validation-plan-artifacts`) / standalone template `dropped`. Decisive evidence:
P14 is an **orphan** (`grep -rl cross-artifact-checklist` = empty — no file consumer); **near-total content
duplication** with P9 (its 4 check groups, How-to-Use, Time Budget, When-to-Escalate, Issue Classification
all already live in P9's `PHASE-CHECKLISTS.md` P3 + incremental table); the plan+tasks "shared source"
argument is **FALSE** (the tasks validator `validation-task-artifacts` self-contains a *different*,
phase-tuned cross-artifact review and does not import P14); it is **validator procedure**, not a fillable
template shape; single-source convention says reference-don't-restate. The cross-artifact **consistency**
content is the advocate's territory → folds into P9 under (i) (the architect owns *contradiction/feasibility*,
not *consistency*). The future tasks port carries its own checklist inside `validation-task-artifacts`
(mirrors HIL). The *responsibility* survives in P9; only the duplicate **file** drops.

### RQ4 — advocate-report-template (P15) → **REUSE AS-IS**

`keep-verbatim × standalone` — the specify port is reused unchanged; the plan run emits **NO new file**.
Evidence: identical 3-state verdict contract (`ready/needs-revision/critical-gaps`, = P9 SKILL.md:147–153);
identical Critical/Important/Minor severity; HIL binds **both** plan review sites to this exact template
(plan.md:435, :586); plan gap categories fold into the existing taxonomy (Missing/Ambiguous/Edge
Case/Assumption/Contradiction) with **no new gap type**. The one plan-only shape (incremental Cross-Artifact
Consistency matrix) homes in **P9**, not P15. Only touched artifact: the REGISTRY row broadens
`specify` → **shared (specify + plan)** (a wiring step). **Holds cleanly under recommended (i)** (advocate-report
stays completeness-only; the architect's feasibility findings live in P13). *(If the human overrides to (ii),
RQ4 flips to EXTEND — see the runner-up delta.)*

### RQ5 — plan-context-template (P11) → **DROP × ABSORB-INTO-LEAD** (confirmed)

Confirmed the strongly-evidenced absorb (memory-model Key Decision, 3rd cluster, richest carrier, all state
dissolves; consumption live & by-name at plan.md:232, State-Recovery resume at :884–898). The carrier folds
into the command lead (in-session loop state + workspace-as-state under `.mochiko/specs/<feature>/` +
`.mochiko/memory/` reads), no orphan template. **The RQ1-coupled report-path slice (F-P11-2) resolves under
(i) to THREE report homes:**
- `analyst_report_path` → `.mochiko/specs/<feature>/techanalyst-report.md` (P12)
- `architect_report_path` → `.mochiko/specs/<feature>/feasibility-report.md` (P13, renamed)
- `advocate_report_path` → `.mochiko/specs/<feature>/advocate-report.md` (P15)

Option (i) is the branch in which **all three** producer/reviewer report slices survive (under (ii) only two).
F-P11-1 (rehome coordination with P1) is executed in Job 2 below (one coherent map). Human-gated drops
(R1 `type`, R6 timestamps, R20 ephemeral lifecycle) listed for acceptance.

### RQ6 — producer-skill content homes (the highest silent-drop surface)

**RQ6a — data-sensitivity → canonical home P7.** The DS-XXX per-entity classification is split today
(P5 declares + delegates to P7; the full 4-level taxonomy lives in P5's `ARTIFACT-TEMPLATES §4`; P7 carries
only light PII markers; two divergent `data-model.md` templates). **Resolution:** **P7 absorbs** the DS-XXX
4-level taxonomy (Public/Internal/Confidential/Restricted) + sensitivity-details (encryption / retention /
masking / compliance) + the classification decision tree + becomes the **single canonical `data-model.md`
template** — P7 owns the artifact where DS is applied. P5 keeps a **thin DS-XXX analysis-concern
declaration** (flag *which* data is sensitive as a requirement) and **references P7** for the per-entity
procedure + template (the broken delegation is repaired by making P7 contain what P5 points to). The two
divergent templates **collapse to one** (P7's). Tags: P5 DS taxonomy + §4 template → `moved-to-sibling-skill`
(P7) + `dedupe`; P7 → absorbs (an additive `port-with-edits` slice). **Cannot drop** (P9 grades it Critical).

**RQ6b — x-integration → canonical home P8.** P5 forward-refs P8 ("see patterns-api-contracts") but **P8
has ZERO x-integration content** — a dangling pointer; the substance currently sits in P5 (INT-XXX:
system/protocol/criticality/failure-modes/fallback/auth); P2's persona promises it; **P9 grades it
Critical**; P1 lists it a Full-Review check. **Resolution:** **P8's `port-with-edits` ADDs** the
x-integration boundary authoring section + the per-endpoint `x-integration` OpenAPI-extension format
(system name, protocol, API version, criticality, failure modes [detection/impact/fallback], auth) — most
consistent with ADR-008, the run scope, and P5's own pointer. P5's INT-XXX authoring →
`moved-to-sibling-skill` (P8); P5 keeps a thin INT-XXX analysis declaration + reference. The dangling
pointer is repaired by making the target real. **Cannot drop** (P9 Critical).

**P5↔P6 constraints-and-decisions.md → BOUNDARY + DEDUPE (not merge).** Both write the same file + describe
D-XXX. **Resolution:** **P5 owns the artifact** (the `constraints-and-decisions.md` template, the Section-2
Technology-Decisions D-XXX field schema, C↔D + IP traceability); **P6 owns the technique** (evaluation
matrix, ADR depth, brownfield-alignment scoring, ≥2-alternatives discipline) and **references** P5's artifact
structure rather than restating it (P6 lines 92–119 → `dedupe`). Handoff edge **P5 → P6**. Both stay
`standalone`. NOT a merge (each holds a large unique slice).

**P5↔P9 authoring↔validation standard (F5) → keep distinct (the pair IS the mirror).** The traceability /
measurability / completeness criteria appear author-to (P5, producer self-check) and grade-against (P9,
independent gate) — this is the normal producer↔validator mirror across **two different agents**. Co-pointing
both at one source would couple a producer skill to a validator skill; minimalism + independence say keep
distinct. No action beyond recording the pair. (P5's Quality Checklist is a producer self-check, **never**
the independent gate — must not be mistaken for or merged with P9.)

**P7↔P8 conceptual-type substrate (F2) → keep distinct; handoff edge command-owned.** Shared conceptual-type
vocabulary at the data-model→api-schema seam (P7 defines types, P8 maps to OpenAPI) — a `dedupe` *candidate*,
but **lean keep-distinct** (factoring manufactures cross-skill coupling for marginal benefit, against
kernel-free minimalism; consistent with the specify-pair precedent). The handoff edge (P7 data-model → P8
schemas) is **command/lead-owned**; the schema-model **consistency check** is **P9-owned**. NOT a merge
(peer producers of distinct artifacts). Tertiary: the shared `validate-*.py` scaffold (P7/P8) → keep
distinct (minimalism).

**Sibling trigger de-collisions → handed to the convention-wiring pass (transform-recipes), NOT structural.**
After this reconcile sets the ownership boundaries, wiring phrases each `description` to its own side:
P5↔P6 (real — decision/D-XXX space; P5 = constraints/NFR/IP authoring + traceability, P6 = evaluate/ADR/
alternatives/trade-offs technique); P5↔P7 (minor — data classification); P5↔P8 (minor — integrations);
P7↔P8 (mild — data-model ↔ schema); P9 generic phrases ("plan quality"/"phase review"/"artifact review").

### Relational verdicts table (every flag → a concrete move)

| Primitive | Flag (from assess) | Resolved move | Partner / home |
|-----------|--------------------|---------------|----------------|
| P1 plan-command | RQ1 reviewer architecture | **(i)** two distinct validators | feasibility=PA+`validation-feasibility`, completeness=advocate+P9, referee=lead |
| P1 | P11 plan-context rehome | confirm `absorb-into-lead` | one coherent map with P11 (Job 2) |
| P1 | producer↔validator casting | resolved | producer=technical-analyst; validators=PA + advocate; lead=plan command |
| P2 technical-analyst | RQ1 pairing architecture | producer settled `standalone` | graded by PA (feasibility) + advocate (completeness); independence ∩=∅ |
| P2 | dedupe (Quality-Standards literal procedure) | `dedupe` → P5–P8 | defer literal procedure; keep taste |
| P2 | 3 caller-injection decouple edits (L76/88/136) | → wiring (`port-with-edits`) | push which-artifacts/where to `agent-dispatch.md` |
| P3 principal-architect | RQ1 + RQ2 (feasibility homeless) | **`folded-into-skill`** (resp. #7) | re-broaden PA + mount NEW `validation-feasibility`; G1 |
| P3 | #7b feasibility verdict taxonomy (incl. `infeasible`) | `kept` (all 3 states) | in `validation-feasibility` + P13 report |
| P3 | #7c feasibility≠completeness division | `kept` | honored under (i); decouple "reviewer's job"→role |
| P4 devils-advocate | RQ1 reviewer architecture | `kept-but-rebind` (re-mount P9, **completeness-only**) | does NOT gain feasibility under (i) |
| P4 | re-mount sequencing | record edge | P9 must land BEFORE P4 re-mount (else Tier-1 FAIL) |
| P5 authoring-technical-requirements | F3 DS-XXX (RQ6a) | `moved-to-sibling-skill` → P7 + `dedupe` | P7 canonical; P5 thin declaration + ref |
| P5 | F4 INT-XXX (RQ6b) | `moved-to-sibling-skill` → P8 | P8 canonical (ADD); P5 thin declaration + ref |
| P5 | F2 constraints-and-decisions (P6) | boundary + `dedupe` | P5 owns artifact; P6 owns technique |
| P5 | F1/F5 producer↔validator pair | pairing recorded; keep distinct | independent grade = P9 (advocate) + `validation-feasibility` (PA) |
| P6 patterns-technical-decisions | P5↔P6 ownership | boundary + `dedupe` (reference P5) | technique-owner; handoff P5→P6 |
| P6 | NEEDS CLARIFICATION escalation | `moved-to-lead` | marker stays; loop-driving → lead |
| P7 patterns-entity-modeling | F1 DS-XXX (RQ6a) | **ABSORB** taxonomy + canonical template | `moved-to-sibling-skill` IN from P5; one template |
| P7 | F2 P8 handoff/substrate | keep distinct | handoff command-owned; substrate dedupe declined |
| P7 | "Generated by Domain Architect" stamp | `kept-but-rebind` (scrub) | wiring decouple |
| P8 patterns-api-contracts | Flag 1 x-integration (RQ6b) | **ADD content** (`port-with-edits`) | P8 canonical; cannot drop (P9 Critical) |
| P8 | Flag 2 P7 handoff | keep distinct | as P7 F2 |
| P9 validation-plan-artifacts | RQ1 feasibility overlap | **boundary `dedupe`** | feasibility checks → `validation-feasibility`; P9 keeps coverage/consistency |
| P9 | RQ3 P14 fold-in | **absorb P14** (`merge-into-sibling` target) | P9 = single source of cross-artifact consistency |
| P9 | incremental-mode phase-switch | `folded-into-skill` (procedure) + `moved-to-lead` (selection) | lead picks {new}/{prior} sets |
| P9 | A0 codebase-discovery review | `moved-to-other-cluster` (out of plan-core) | confirmed deferred-by-reference |
| P10 plan-template | F1 roster ↔ producer consistency | confirm (matches techspec-merged form) | sensitivity col / integration col present |
| P10 | F2 Artifacts deliverable set | keep roster | 6 artifacts + plan.md; quickstart persona-produced |
| P10 | F3 assembly ownership | record `moved-to-lead` | template = lead's fill-target |
| P11 plan-context-template | F-P11-1 rehome coordination | execute in Job 2 | one map, paired with P1 |
| P11 | F-P11-2 report-path shape (RQ1) | **three report homes** under (i) | workspace-as-state |
| P11 | F-P11-3 producer content in Supervisor-Instructions | `moved-to-other-cluster` (skills) | IP-XXX→P5, DS→P7, x-int→P8, advocate checklists→P9, feasibility→`validation-feasibility` |
| P11 | F-P11-4 cross-cluster carrier genre | note | tasks/implement inherit workspace-as-state |
| P11 | F-P11-5 human gate (RQ5) | gated | accept absorb + drops |
| P12 techanalyst-report | edit-depth (P11-gated) | slim counts; **+ "What Changed This Round"** | lead reads artifacts directly |
| P12 | cross-report-family round/header (RQ1) | align round vocab across 3 reports | `iteration`→`round`; header shape may differ |
| P13 architect-report | RQ1 host/fate | **`port-with-edits × standalone`** → `feasibility-report` | the `validation-feasibility` report format |
| P13 | verdict-routing "Next Steps" | `moved-to-lead` | Feasibility Rejection Loop → lead |
| P14 cross-artifact-checklist | RQ3 fate | **`drop × merge-into-sibling`** → P9 | file drops; content folds into P9 |
| P15 advocate-report | RQ4 reuse-vs-extend | **`keep-verbatim`** (REUSE) | no new file; REGISTRY re-file |
| P15 | R11 consistency matrix | `moved-to-sibling-skill` → P9 | not owed by P15 |

---

## JOB 2 — Rehome map (the dissolving plan supervisor → the lead)

**One coherent map (P1 ⊕ P11), each responsibility homed exactly once — no double-home, no drop between
primitives.** The HIL markdown supervisor + the `.workflow/plan-context.md` state-carrier both dissolve onto
the **same** thinned `plan` command lead.

### A. Workflow-specific orchestration → the lead (the thinned `plan` command supervisor)

| # | Responsibility (source) | Re-homes to |
|---|--------------------------|-------------|
| 1 | **2-phase analysis→design sequence** (P1 §B · P11 R2/R7) | lead; phase position tracked **in-session + workspace evidence** (Phase-1 vs Phase-2 artifacts present) — NOT a `phase`/`*_status` field |
| 2 | **Architect-feasibility-ONCE-after-Phase-1, then advocate-completeness ordering** (P1 §B · P3 #8 · P13 T9) | lead; under (i): PA(`validation-feasibility`) once after Phase-1 → advocate(P9) both phases |
| 3 | **Skip-architect-re-review-unless-structural-change routing** (P1 §B) | lead routing param (re-review only on new/changed constraints · expanded req scope · modified NFR targets; clarification-only → straight to advocate) — **silent-drop risk, keep explicit** |
| 4 | **Phase-2 dual-mode incremental review** (P1 §B · P9 R9/R10 · P4 R16) | lead **selects** the mode + supplies {new}/{prior} sets; the **procedure** rides in P9 (`folded-into-skill`) |
| 5 | **Team casting** (P1 §B) | lead: technical-analyst produces both phases; PA grades Phase-1 feasibility; advocate grades both phases completeness |
| 6 | **Phase-4 plan.md assembly** (P1 §B · P10 F3) | lead extracts key-decisions (constraints-and-decisions) + entity summary incl. sensitivity (data-model) + endpoint summary incl. integrations (contracts/api.yaml); **P10 is the lead's fill-target** |
| 7 | **Done-condition parameters** (P1 §B) | lead, **filled into the `workflow-contract` artifact** (six artifacts present + validated; cap #; gate placements) — never inlined |
| 8 | **Constitution prerequisite** (P1 §B · P11 R12) | handoff edge: lead reads `.mochiko/memory/constitution.md` (rebind `.humaninloop/`→`.mochiko/`); not copied into a context file |
| 9 | **Entry gate** (P1 §B/§C · P11 R9) | handoff edge: spec.md present + specify accepted — **rebound to workspace evidence**, NOT a `status`-field read |
| 10 | **Brownfield check** (P1 §B · P11 R13) | handoff edge: lead reads `.mochiko/memory/codebase-analysis.md`, computes >14d staleness from workspace mtime, greenfield bypass |
| 11 | **@-input recovery** (P1 §B) | lead (empty `@`-ref → AskUserQuestion re-enter/proceed) |
| 12 | **State recovery / resume** (P1 §B/§C · P11 R18) | lead, **REBOUND to workspace evidence** (artifact presence) — NOT a context `phase`/`status` field |
| 13 | **Existing mid-loop human gates** (P1 §B) | lead: Feasibility Rejection Loop gate · Clarification Loop gate (incl. the **"Research this"** knowledge-gap→research branch) · "When to Exit Early" offering — **must survive alongside the NEW acceptance gate** |
| 14 | **Per-phase dispatch / Supervisor Instructions** (P11 R15) | lead's **in-session dispatch brief** (`agent-dispatch` pattern), not a file field |
| 15 | **Workspace path registry + 3 report paths** (P11 R10/R11 · P1 §C) | lead, **workspace-as-state** under `.mochiko/specs/<feature>/`; three report homes (techanalyst · feasibility · advocate) |
| 16 | **Clarification Log (human Q&A history)** (P11 R17) | lead, in-session (persisted to workspace if durability needed) |
| 17 | **iteration/round counting** (P11 R4) | lead, **now BOUNDED** (contract round cap 3 + no-progress + STOP) — an **UPGRADE** over HIL's "no hard caps" (plan.md:905) |
| 18 | **Operational handling** (P1 §B) | lead: verify-agent-output (confirm expected files created; on missing → log + ask retry/abort); agent-failure messaging |

### B. Generic loop-discipline → `dedupe` (REFERENCED, never `moved-to-lead`) — keeps the command thin

Iteration structure · default-FAIL mechanics · producer↔validator independence doctrine · validator tiers ·
tamper-proofing · the four guards (as *requirements*) · gap-type routing (incl. "Research this") ·
anti-rationalization · briefing mechanics · git-footer. The thin command **references** `loop-discipline`
+ `workflow-contract` + `agent-dispatch`; it never restates them (command-altitude held — contract §1c;
`verify-output` altitude gate must PASS).

### C. The FOUR gates HIL lacks — ADD (requirement `dedupe`→`loop-discipline`; placement/params `moved-to-lead`)

| Gate | What to add | Home |
|------|-------------|------|
| **Default-FAIL done-condition** | artifact starts FAIL; clears only on independent validation + human acceptance | requirement `dedupe`; params → lead (contract) |
| **Lead-OWNED verdict** | lead **Reads** the analysis/design artifacts + reviewer reports and owns the clearing verdict; the reviewer status is **input, not the gate** | doctrine `dedupe`; reversal note → lead |
| **Hard bound + kill-switch** | deterministic round cap (3) + no-progress exit (unchanged gap/fix set) + kill-switch file | requirements `dedupe`; params → lead |
| **NEW human ACCEPTANCE gate on plan.md** | accept→done / amend→bounded re-enter / reject→abort | named gate `dedupe`; placement → lead — **must NOT displace the existing mid-loop gates (A#13)** |

### D. Producer-side content that must NOT vanish with the dropped carrier → producer/validator SKILLS

(The thin command drops the inline `## Supervisor Instructions` wholesale — P1 §D / P11 R16 / F-P11-3. The
**highest-volume silent-drop surface**; survives ONLY if the skills carry it. Now each has an explicit home:)

| Content | Canonical home (RQ6) |
|---------|----------------------|
| IP-XXX infrastructure planning | **P5** `authoring-technical-requirements` (`kept`) |
| Data-sensitivity taxonomy | **P7** `patterns-entity-modeling` (RQ6a — absorbed) |
| x-integration boundaries | **P8** `patterns-api-contracts` (RQ6b — added) |
| Advocate focus-area checklists (FR-coverage, coverage/consistency) | **P9** `validation-plan-artifacts` |
| Cross-artifact feasibility/contradiction checks | **`validation-feasibility`** (NEW, under (i)) |

### E. Drops (lead/human must accept — none silent)

P11: `type: plan-request` node-kind (R1) · `created`/`updated` timestamps (R6) · ephemeral create/delete
lifecycle (R20). · P1: the `plan-context.md` state-carrier file · inline `Task()`/`AskUserQuestion()`
payloads + `supervisor_instructions` prose (transliteration) · LLM-judged "no hard caps" iteration counter.
· P10: L6 producer-command stamp. · P12: `{{completion_status}}` self-asserted done (independence — verdict
is lead-owned) + brain-era count/metric framing. · P14: the standalone template FILE (content survives in
P9). · P13: `type` DAG frontmatter.

---

## JOB 2b — New primitives required

| Name | Kind | Why | Notes |
|------|------|-----|-------|
| **`validation-feasibility`** | **skill** | Under recommended RQ1 **(i)**: the feasibility reviewer's driver — the adversarial-critique form of convention-5's two-form. Homes the **currently-homeless** cross-artifact feasibility intersection review. | Mounted on the re-broadened `principal-architect`. **Scope:** cross-artifact contradictions, constraint-decision conflicts, NFR-constraint impossibilities, NFR-design feasibility, constraint-design alignment → 3-state `feasible/needs-revision/infeasible` verdict into the **`feasibility-report`** (P13, renamed). **Independence:** grades the technical-analyst's artifacts (different agent). **G1:** operates over plan analysis artifacts, NOT the constitution; is NOT `validation-constitution`. Canonical home for the feasibility checks **deduped out of P9**. Description = work-context idiom (agent-consumed). |

> **Zero new agents.** `principal-architect` already exists (re-broadened, not new); `devils-advocate`
> already exists (re-mount); `validator` unchanged. Under runner-up **(ii)**: **zero** new primitives if P9
> absorbs feasibility (or one feasibility-critique skill on the *advocate*).

---

## Finalized dispositions (recommended option (i))

| # | Primitive | Body × Structural | Key reconcile note |
|---|-----------|-------------------|--------------------|
| P1 | plan-command | **`redesign × absorb-into-lead`** | thin sound loop; rehome map (Job 2); +4 gates |
| P2 | technical-analyst | **`port-with-edits × standalone`** | producer; mount P5–P8 (rebind); 3 decouple edits; light dedupe |
| P3 | principal-architect | **`port-with-edits × standalone`** | re-broaden to feasibility reviewer; mount NEW `validation-feasibility`; G1 |
| P4 | devils-advocate | **`port-with-edits × standalone`** | re-mount P9 (completeness-only); P9-before-P4 sequencing |
| P5 | authoring-technical-requirements | **`port-with-edits × standalone`** | DS→P7, INT→P8 (moved-to-sibling-skill); P6 boundary; thin declarations + refs |
| P6 | patterns-technical-decisions | **`port-with-edits × standalone`** | technique-owner; reference P5's artifact (dedupe) |
| P7 | patterns-entity-modeling | **`port-with-edits × standalone`** | **ABSORB DS taxonomy + canonical data-model.md template**; scrub stamp |
| P8 | patterns-api-contracts | **`port-with-edits × standalone`** | **ADD x-integration authoring content** (cannot drop) |
| P9 | validation-plan-artifacts | **`port-with-edits × standalone`** | completeness/coverage/consistency; **absorb P14**; feasibility checks dedupe→`validation-feasibility`; decouple phase axis |
| P10 | plan-template | **`port-with-edits × standalone`** | lead's fill-target; keep roster + `[...]` style |
| P11 | plan-context-template | **`drop × absorb-into-lead`** | three report homes under (i); accept drops R1/R6/R20 |
| P12 | techanalyst-report-template | **`port-with-edits × standalone`** | slim counts; drop `completion_status`; `iteration`→`round`; +round-delta |
| P13 | architect-report-template | **`port-with-edits × standalone`** | **rename → `feasibility-report`**; the `validation-feasibility` report format; lift Next-Steps to lead |
| P14 | cross-artifact-checklist | **`drop × merge-into-sibling`** | fold into P9; standalone file dropped (content survives) |
| P15 | advocate-report-template | **`keep-verbatim × standalone`** | REUSE; no new file; REGISTRY re-file specify→shared |
| — | **`validation-feasibility`** | **NEW skill (created `× standalone`)** | feasibility reviewer's driver on PA (under (i)) |

---

## JOB 3 — Closed trace (relational tags re-emitted; zero open flags)

Every previously-flagged responsibility now carries an assigned relational tag:

- **Feasibility intersection review** (P3 #7, was `flag-for-reconcile`) → **`folded-into-skill`** → new
  `validation-feasibility` on re-broadened `principal-architect`. Verdict taxonomy incl. `infeasible` → `kept`.
- **technical-analyst producer↔validator pairing** (P2, was `flag-for-reconcile`) → producer `standalone`;
  validators = PA (`validation-feasibility`) + advocate (P9); **independence ∩ = ∅** preserved.
- **devils-advocate R7** (was stubbed `moved-to-other-cluster`) → **`kept-but-rebind`** (live mount of P9,
  completeness-only). R8 `validation-task-artifacts` stays stubbed (tasks deferred — not dropped).
- **P9 cross-artifact/feasibility overlap** (R3, was `flag`) → feasibility checks `dedupe` →
  `validation-feasibility`; consistency/coverage `kept` in P9. **P14 content** (R22 + P14 R1) →
  `folded-into-skill` / `dedupe` → P9 (single source).
- **architect-report** (P13, was `flag`) → `kept`/`kept-but-rebind` as the `feasibility-report` format
  (3-state verdict + 3 cross-artifact tables + 4-field gate fuel all preserved); Next-Steps `moved-to-lead`.
- **advocate-report** (P15, was `flag` RQ4) → `keep-verbatim` (REUSE); R11 consistency matrix
  `moved-to-sibling-skill` → P9; R5 verdict-consumption `moved-to-lead` (P1).
- **DS-XXX** (P5 R6 / P7 F1) → `moved-to-sibling-skill` → P7 (canonical) + `dedupe` (one template).
- **INT-XXX / x-integration** (P5 R7 / P8 Flag 1) → `moved-to-sibling-skill` → P8 (added; canonical).
- **constraints-and-decisions.md** (P5 R3 / P6 R10) → boundary + `dedupe` (P5 artifact-owner, P6
  technique-owner references P5).
- **P11 state/loop/dispatch slices** (R2/R3/R4/R7/R8/R10/R11/R15/R17/R18) → `moved-to-lead` (Job 2A); the
  producer content inside Supervisor-Instructions (R16) → `moved-to-other-cluster` (Job 2D skills); drops
  (R1/R6/R20) accepted.
- **All `moved-to-lead` orchestration** (P1 §B, P3 #8, P4 R13/R14/R16/R17, P9 R10/R19/R20, P10 F3, P12, P13
  T9, P15 R5) → landed in the one coherent rehome map (Job 2A), with the four missing gates ADDED (Job 2C).

**Homeless-responsibility check:** none. Every responsibility from all 15 traces is `kept`, `kept-but-rebind`,
`folded-into-skill`, `moved-to-lead`, `moved-to-sibling-skill`, `moved-to-other-cluster`, `dedupe`, or
`dropped + reason`. The single highest-risk surface (feasibility review) is explicitly homed; the highest-volume
silent-drop surface (producer content in Supervisor-Instructions) is explicitly homed to P5/P7/P8/P9/
`validation-feasibility`.

### Independence attestation (non-negotiable — re-verified)

- **Producer** `technical-analyst` mounts `{authoring-technical-requirements, patterns-technical-decisions,
  patterns-entity-modeling, patterns-api-contracts}` — all authoring/patterns; **no grading skill**.
- **Feasibility reviewer** `principal-architect` mounts `{authoring-constitution, analysis-codebase}` (setup
  producer role) ⊕ `{validation-feasibility}` (plan reviewer role). Grades the **analyst's** plan artifacts
  (different agent); produces the **constitution** in setup (graded by the generic `validator`, not PA).
  **Never produces+grades the same artifact.** **G1:** `validation-constitution` NOT mounted.
- **Completeness reviewer** `devils-advocate` mounts `{analysis-specifications, validation-plan-artifacts}` —
  both review skills; **no authoring skill**. Grades the analyst's artifacts (different agent).
- Producer skills ∩ (PA reviewer skill ∪ advocate review skills) = **∅**. Two independent reviewers, neither
  the producer; the **lead** referees and owns the clearing verdict. **No produce+grade on one agent — by
  construction.** Port hazard recorded: never co-mount a review skill on `technical-analyst`, never mount an
  authoring/patterns skill on a reviewer.

---

## Output block (reconcile-cluster format)

```
RECONCILE: plan
Relational verdicts:
  - P1 plan-command       : RQ1 → (i) two distinct validators (feasibility=PA+validation-feasibility,
                                      completeness=advocate+P9, referee=lead) [RECOMMENDED, human-gated]
  - P1                    : P11 rehome → absorb-into-lead, one coherent map (Job 2)
  - P2 technical-analyst  : RQ1 pairing → producer standalone; graded by PA + advocate (independence ∩=∅)
  - P3 principal-architect: RQ1+RQ2 → feasibility resp. folded-into-skill (NEW validation-feasibility);
                                      re-broaden persona; G1 (no validation-constitution)
  - P4 devils-advocate    : RQ1 → kept-but-rebind (re-mount P9, completeness-only; P9-before-P4 seq)
  - P5 authoring-tech-req : DS→P7 (moved-to-sibling-skill+dedupe); INT→P8 (moved-to-sibling-skill);
                                      P6 boundary+dedupe
  - P6 patterns-tech-dec  : P5↔P6 → boundary+dedupe (P5 artifact-owner / P6 technique-owner ref P5)
  - P7 patterns-entity    : RQ6a → ABSORB DS taxonomy + canonical data-model.md template (one template)
  - P8 patterns-api       : RQ6b → ADD x-integration authoring content (cannot drop; P9 Critical)
  - P9 validation-plan    : RQ1 → boundary dedupe (feasibility→validation-feasibility; keep coverage/consist.)
                            RQ3 → absorb P14 (merge-into-sibling target); incremental: procedure folded / selection→lead
  - P10 plan-template     : keep roster; lead's fill-target (assembly moved-to-lead)
  - P11 plan-context      : RQ5 → drop × absorb-into-lead; THREE report homes under (i)
  - P12 techanalyst-report: slim counts; drop completion_status; iteration→round; +round-delta
  - P13 architect-report  : RQ1 → port-with-edits × standalone, RENAME → feasibility-report
  - P14 cross-artifact-chk: RQ3 → drop × merge-into-sibling (fold into P9; file dropped, content survives)
  - P15 advocate-report   : RQ4 → keep-verbatim × standalone (REUSE; no new file; REGISTRY re-file)
Rehome map (orchestration):
  - 2-phase analysis→design sequence            → lead (in-session + workspace evidence)
  - architect-feasibility-once→advocate ordering → lead (PA once after Phase-1; advocate both phases)
  - skip-architect-unless-structural routing     → lead routing param (keep explicit)
  - Phase-2 dual-mode incremental review         → lead selects mode; procedure folded into P9
  - team casting / plan.md assembly              → lead (P10 = fill-target)
  - done-condition params                        → lead → filled into workflow-contract artifact (not inlined)
  - constitution / entry / brownfield prereqs    → handoff edges (lead reads .mochiko/memory + workspace)
  - @-input recovery / state-recovery (rebound)  → lead (workspace evidence, not phase/status field)
  - mid-loop human gates (feasibility/clarif/exit + "Research this") → lead (survive alongside new gate)
  - dispatch / Supervisor-Instructions / Clarif Log → lead in-session (agent-dispatch)
  - iteration → bounded round counter            → lead (UPGRADE: cap 3 + no-progress + STOP)
  - ADD default-FAIL done-condition              → lead (mechanics dedupe→loop-discipline)
  - ADD lead-owned verdict (reviewer status=input)→ lead
  - ADD hard bound + kill-switch                 → lead
  - ADD human acceptance gate on plan.md         → lead human-gate (must not displace mid-loop gates)
  - producer content (IP-XXX/DS/x-int/checklists/feasibility) → P5 / P7 / P8 / P9 / validation-feasibility
New primitives required:
  - validation-feasibility (skill) — feasibility reviewer's driver on re-broadened principal-architect
                                     (under recommended RQ1 option (i)); homes the homeless feasibility review;
                                     3-state feasible/needs-revision/infeasible; G1-safe (not the constitution)
Finalized dispositions:
  P1 redesign×absorb-into-lead · P2 port-with-edits×standalone · P3 port-with-edits×standalone ·
  P4 port-with-edits×standalone · P5 port-with-edits×standalone · P6 port-with-edits×standalone ·
  P7 port-with-edits×standalone · P8 port-with-edits×standalone · P9 port-with-edits×standalone ·
  P10 port-with-edits×standalone · P11 drop×absorb-into-lead · P12 port-with-edits×standalone ·
  P13 port-with-edits×standalone(rename feasibility-report) · P14 drop×merge-into-sibling(→P9) ·
  P15 keep-verbatim×standalone(REUSE) · NEW validation-feasibility (skill)
Open flags: NONE
Human-gate bundle (lead → human before transform): RQ1 (recommend i; runner-up ii delta spelled out) ·
  RQ2 (PA produce-here/review-there + new validation-feasibility skill; G1) · P1 redesign · P11 absorb +
  drops R1/R6/R20 · P14 fold-into-P9 (drop standalone) · P15 reuse · RQ6 homes (P7 DS / P8 x-int) ·
  P12 completion_status drop · P10 L6 stamp drop.
```

---

**Reconcile version:** v1 · **Governed by:** `loop-discipline` · **Role:** decide + rehome only — no edit,
no grade. RQ1/RQ2 carry a RECOMMENDATION for the human gate; all flags carry an assigned move → **zero open
flags**.
