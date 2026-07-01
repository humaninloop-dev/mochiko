# ASSESSMENT: principal-architect (P3) — DELTA against the existing setup port

**Run:** `transform-cluster plan` · **Role:** assess/diagnose ONLY (no transform, no grade) · **Date:** 2026-06-30
**HIL source:** `human-in-loop/plugins/humaninloop/agents/principal-architect.md`
**Current mochiko port:** `plugins/mochiko/agents/principal-architect.md` (ported for `setup`, producer-only)

```
Class:       agent → branch PLAYS-a-role
Triage:      gate1=y  gate2=y  gate3=y   → full-lens (all three trip)
Disposition: port-with-edits × flag-for-reconcile (RQ1 + RQ2)
             — body treatment is itself contingent on RQ1 (see §Disposition)
Reconcile flags: RQ1 (reviewer architecture) + RQ2 (PA role & skill-vs-persona) — NOT resolved here
```

---

## The DELTA in one paragraph

The HIL `principal-architect` is a **dual-role** agent: it **produces** governance (constitutions, codebase analyses, roadmaps, CLAUDE.md sync) **and** it **reviews** cross-artifact **feasibility** for the `plan` workflow (HIL agent §"Feasibility Review", lines 128–148; product #4 "Feasibility Reviews", line 66). The current mochiko port was cut for **setup only** and was deliberately narrowed to **producer-only**: grading was split to the generic `validator`, and the feasibility-review responsibility was **explicitly disclaimed** (mochiko port lines 107–109: *"Out of scope: cross-artifact feasibility review … is **not** your responsibility"*). So the plan delta is a **single, sharp capability gap**: the cross-artifact feasibility intersection review that `plan` Phase-1 depends on **does not exist anywhere in mochiko today** and is *actively refused* by the one agent that used to carry it. Where that responsibility lands is RQ1/RQ2 — a reconcile decision, flagged below.

---

## Triage gate (why full-lens)

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **YES** — in HIL the plan **supervisor** sequences the feasibility review (once, after Phase-1 analysis), routes on its verdict, and drives the feasibility-rejection loop (`commands/plan.md` §2.5–2.6, §"Feasibility Rejection Loop"). The agent only works because something else drives it. |
| 2 | Multi-responsibility / fans out? | **YES** — authoring + codebase analysis + (in HIL) feasibility review + roadmap + CLAUDE.md sync; and the persona is asked to serve **two workflows** (setup-produce, plan-review). |
| 3 | Emits a non-machine-checkable artifact? | **YES** — the constitution it authors needs a real validator (it has one); the feasibility **verdict** (feasible / needs-revision / infeasible) is pure cross-artifact model judgment, not a schema/version check. |

---

## Decoupling-by-absence scan (current mochiko persona)

**Deny-list grep — CLEAN.** Scanning the current port for the four deny-list token classes:

| Token class | Present? | Evidence |
|-------------|----------|----------|
| Sibling-agent names | **No** | only "Principal Architect" (self). No `technical-analyst` / `devils-advocate` / `validator` / `requirements-analyst`. |
| "dispatch" | **No** | absent. |
| Injected workflow modes/paths/phases in persona | **No** | no `.mochiko/specs/…` paths, no "Phase 1/2", no workflow names. Line 105's only path is a *skill* reference (`authoring-constitution`'s `references/ESSENTIAL-FLOOR.md`) — legitimate single-sourcing, not coupling. |
| "workflow-agnostic" / independence-by-declaration meta-label | **No** | absent. |

**But the persona is deny-list-clean *and* capability-NARROWED** — and the narrowing is the delta-blocker, not a deny-list hit:

- Description (line 4): *"brings governance judgment to **AUTHORING** constitutions"* — positively scoped to authoring.
- Body (line 32): *"You author and update the constitution and run the codebase analysis it is built on."*
- Three-Part Rule narrowed: HIL line 80 *"Every standard you write **or evaluate**"* → mochiko line 67 *"Every standard you **write**"* (the *evaluate* altitude was removed).
- "What You Produce" cut from HIL's **5** items to **2** (constitution, codebase analysis); "Feasibility Reviews" deleted.
- Lines 107–109: an explicit **negative carve-out** refusing feasibility review.

**Implication for RQ2:** decoupling-by-absence (no positive workflow tokens) is *necessary but not sufficient*. The persona is decoupled, yet it cannot serve as plan's feasibility reviewer **as-is**, because it was specialized to authoring and *disclaims* the review. Serving plan requires **re-broadening** the persona back toward HIL's original *"establishes **and evaluates** governance standards"* framing (HIL line 38). That broadened persona is itself keystone-clean — *a senior architect who writes standards **and** reviews designs for buildability is one coherent professional, naming no workflow* — so the re-broaden adds **no** new deny-list token. **Re-coupling risk: LOW** (substantiated in RQ2 below).

---

## 7-check lens (weighted for agent / PLAYS-a-role)

**1 — Orchestration test.** Two couplings, cleanly separated:
- *Content-coupling:* none in the surviving body — the persona is mochiko-clean (no kernel/DAG/catalog tokens).
- *Orchestration-coupling:* the feasibility review is **driven by the plan supervisor** — sequenced once after Phase-1, verdict-routed, loop-driven on `needs-revision`/`infeasible`. When the HIL markdown supervisor dissolves, that sequencing + loop-driving + the verdict-sink must **re-home to the plan lead** (`moved-to-lead`), not to the agent. The agent contributes *judgment*; the lead owns *when it runs and what loops on its verdict*.

**2 — Role at two altitudes.**
- *Skill-role:* the feasibility review **emits a reviewable artifact** (the architect-report: a verdict + cited contradictions + suggested resolutions). It is a *verdict-producer*.
- *Team-role:* consuming it confers a **validator/reviewer** team-role on its caller. This is the load-bearing fact for RQ1 — a verdict-emitting reviewer is *promote/pair* material, the same shape that made `validation-constitution` promotable.
- Note: a reviewer's verdict is itself the validator-side output — it does **not** need its own downstream validator (no infinite regress). Its sink is the lead's loop.

**3 — Independence.** **No self-grade — confirmed on every axis** (full argument in §Independence below). In plan, the feasibility review grades the **technical-analyst's** output → different agent → structurally independent. Producing-in-setup and reviewing-in-plan are over **different artifacts in different workflows**, never the same artifact. The historical self-grade leak (HIL PA mounting both `authoring-constitution` **and** `validation-constitution`) was already fixed in the setup port; reconcile must **not** undo it (see silent-drop / guardrail #G1).

**4 — Verdict-sink / loop-driver.** The architect-report verdict is consumed by the **plan supervisor**, which drives the **Feasibility Rejection Loop** (present concerns via `AskUserQuestion`, route `feasible`→proceed / `needs-revision`|`infeasible`→revise, re-invoke analyst, re-submit). All of this is `moved-to-lead`. The **"review feasibility ONCE, before completeness"** ordering (HIL plan.md line 908: *prevents wasting time reviewing completeness of infeasible requirements*) is workflow-specific orchestration → also `moved-to-lead`. **Silent-drop risk if forgotten** (see #3, #4 below).

**5 — Sibling / overlap ("look sideways").** This is the heart of RQ1. Three sibling relations:
- vs **devils-advocate** (the plan completeness reviewer, re-mounting `validation-plan-artifacts` — P4/P9): the advocate **already hunts contradictions** (mochiko devils-advocate §5 "Contradictions and Conflicts", lines 86–89) — but *intra*-artifact (requirements conflicting with each other), whereas the architect hunts *cross*-artifact (requirements↔constraints↔NFRs↔decisions). **Related lens, different scope.** → merge candidate (RQ1-ii).
- vs the generic **`validator`** (mochiko-native binary grader): a verdict-emitting reviewer *could* rehome there — but the generic validator's contract is **binary PASS/FAIL** (`validator.md` lines 73–80), while feasibility is **3-state** (feasible/needs-revision/infeasible) with suggested resolutions. **Contract mismatch.** → rehome candidate (RQ1-iii), with friction.
- vs the **technical-analyst** (P2, the producer): the analyst authors the artifacts the architect reviews → this is the producer↔validator **pairing** itself.
→ **All three are `flag-for-reconcile`; cannot be resolved looking at P3 alone.**

**6 — Coupling audit.**
- *Paths:* HIL feasibility section names artifact **types** generically (requirements / constraints / NFRs / decisions), not workflow paths — so porting it rebinds cleanly. The concrete `specs/{feature}/…` read/write paths live in the **command** (plan.md §2.5), not the agent. `kept-but-rebind` is light.
- *Prerequisite:* feasibility review assumes Phase-1 analysis artifacts already exist (technical-analyst ran first) → an explicit **handoff edge** the lead owns.
- *Determinism boundary:* the review is **entirely model judgment** (cross-artifact contradiction hunting). No deterministic slice → the validator partner here is real, not degenerate.

**7 — Conventions + loop placement.**
- *Classification:* agent (model-invoked, `skills:` list). Present.
- *Discoverability:* router entry exists for the setup role; **must be updated** if a reviewer capability/skill is added.
- *Persona-vs-procedure split:* the HIL feasibility procedure (4 conflict types, 3 verdicts, in/out-of-scope) is **fairly procedural** and currently **persona-baked** — convention 4 pressure ("procedure factors into a skill") pushes toward `folded-into-skill`, but the host depends on RQ1 → **flag**.
- *Producer↔validator pairing (convention 5, two-form):* ROADMAP.md line 80 — *mirror-checklist* form for objective criteria, *adversarial-critique* form for judgment artifacts. Feasibility review is squarely the **adversarial-critique** form. Whether plan runs **both** forms on one producer (architect adversarial-critique + advocate checklist) is exactly RQ1-i → **flag**.
- *Sound-loop:* the loop (analyst produce → feasibility review → completeness review → human gate) gains its done-condition/validation/human-gate from the **plan lead**, not the agent.

---

## RQ2 — principal-architect's role (FLAG-FOR-RECONCILE — analysis only)

**Q(a): can one workflow-agnostic persona legitimately PRODUCE in setup and REVIEW in plan?**

**Analysis (not a decision): structurally YES, but the *current* port cannot do it without a re-broaden edit.**
- *Decoupling-by-absence permits it.* The persona names no workflow; a broadened *"establishes **and evaluates** governance standards, and hunts cross-artifact contradictions"* persona is one coherent professional and passes the keystone test (true of this professional on any job). HIL's original persona was exactly this broader shape (line 38) before the setup port narrowed it.
- *Independence permits it.* Produce-here / review-there are **different artifacts in different workflows** — never produce-and-grade the same artifact (see §Independence). So the dual role does **not** violate convention 5.
- *But the current port blocks it by construction.* It was specialized to authoring and **explicitly refuses** feasibility (lines 107–109), and dropped *evaluate* from the Three-Part Rule and "What You Produce." **To serve plan, the persona must be re-broadened** — this is the `port-with-edits` body treatment, contingent on RQ1 keeping feasibility on PA.

**Re-coupling check (the sub-question: "would adding a reviewer capability re-couple the clean persona?") — LOW risk.** Re-adding feasibility review:
- adds **no** sibling-agent name (HIL §Feasibility Review describes the *work* and names artifact *types*, not agents) — *one* trivial wiring fix: HIL line 138 *"(that is the reviewer's job)"* → decouple to *"a separate reviewer's concern"* (role, not implied agent);
- adds **no** "dispatch";
- adds **no** workflow phase — the *"once after Phase-1"* placement lives in the **command**, not the agent;
- adds **no** "workflow-agnostic" meta-label;
- does **NOT** re-create the constitution self-grade — feasibility operates over **plan analysis artifacts**, not the constitution; `validation-constitution` stays on the generic `validator`. (Guardrail #G1.)

**Q(b): what drives the feasibility review — a NEW reviewing skill, or persona-only?**

**Analysis (not a decision) — entangled with RQ1, so flagged, not resolved:**
- *Data point — the framework already left a forward-pointer.* Two dry-run examples tag this exact procedure `folded-into-skill`: `assess-primitive/references/TRACE-TAGS.md:30` (*"cross-artifact feasibility review → folded-into-skill (orphaned procedure → new skill)"*) and `transform-recipes/references/RECIPES.md:53` (*"extract the orphaned feasibility-review procedure into a skill"*). These are **illustrative**, not binding — but they show the authors envisioned a skill, and convention 4 (procedure → skill) agrees.
- *Counter-pull — persona-only is legitimate.* The decoupling doctrine explicitly allows an agent to be *"competent even when no skill fits."* Feasibility review is adversarial *judgment*, not a checklist; HIL itself keeps it **persona-baked with no skill**. A thin reviewing persona with no skill is defensible.
- *The entanglement:* whether it's a skill **and which agent mounts it** is decided by RQ1. If RQ1-i (distinct architect-reviewer) → a new adversarial-critique skill (e.g. `validation-feasibility` / `feasibility-intersection-review`) on PA, **or** persona-only on PA. If RQ1-ii (fold into advocate) → `moved-to-sibling-skill` (advocate gains a feasibility lens). If RQ1-iii (generic validator) → a new skill on the validator, against its binary-contract friction. **Reconcile picks; I do not.**

---

## RQ1 — reviewer architecture (FLAG-FOR-RECONCILE — analysis only)

HIL plan runs **two independent reviewers** over the analyst's output: principal-architect (**feasibility / cross-artifact contradiction**, adversarial judgment, 3-state verdict, once after Phase-1) and devils-advocate (**completeness / coverage**, via `validation-plan-artifacts`). The mochiko shape is a reconcile decision. Three options with trade-offs (NOT resolved):

| Option | Move | For | Against |
|--------|------|-----|---------|
| **(i)** keep two distinct validators | `rewire-cluster` + re-broaden PA (+ optional new skill) | Preserves HIL's deliberate feasibility-vs-completeness split. **Cleanly exercises convention-5 two-form** (ROADMAP:80): advocate = mirror-checklist (`validation-plan-artifacts`), architect = adversarial-critique (feasibility). Plan = **first cluster to run both forms on one producer**. | A third reviewer in the cluster; must decide skill-vs-persona (RQ2b); re-broaden edit on PA. |
| **(ii)** fold feasibility into the advocate | `merge-into-sibling` (→ devils-advocate) | One reviewer (the `specify` shape); advocate **already** has a contradiction lens (its §5). | Blurs the deliberate **feasibility≠completeness** separation; advocate verdict is `ready/needs-revision/critical-gaps` — the distinct **`infeasible`→business-decision** escalation risks being flattened (silent-drop #2); overloads one reviewer with two adversarial lenses. |
| **(iii)** rehome feasibility onto generic `validator` | `promote` / `moved-to-validator` (+ new skill) | Consistent with "one generic validator serves many workflows" (REGISTRY:49). | **Contract mismatch:** generic validator is **binary PASS/FAIL** (`validator.md`:73–80); feasibility is **3-state + suggested resolutions** — closer to an adversarial reviewer than a binary grader. |

**Grounded data point for reconcile:** both plan reviewers (architect 3-state, advocate 3-state) are **adversarial reviewers**, not binary graders — which is structurally why (iii) has friction and (i)/(ii) sit more naturally. The convention-5 two-form doctrine (i) and the advocate's existing contradiction lens (ii) are the two live contenders; **reconcile decides with P2/P4/P9/P13 in full view.**

---

## Independence (no self-grade) — CONFIRMED

| Axis | Check | Verdict |
|------|-------|---------|
| Plan feasibility review | reviews **technical-analyst's** artifacts (different agent) | independent ✓ |
| PA dual role | setup-produce (constitution) vs plan-review (analysis artifacts) = **different artifacts, different workflows** — never produce+grade the *same* artifact | independent ✓ |
| Setup grading | constitution graded by generic `validator`, not PA (setup port already split this) | independent ✓ |
| Historical leak | HIL PA mounted `authoring-constitution` **+** `validation-constitution` (self-grade) — **already removed** in the mochiko port | must stay removed (guardrail #G1) ✓ |

No resolution of RQ1/RQ2 may place produce + grade of one artifact on one agent. All three options above preserve this (the reviewer always grades the analyst's output, never its own).

---

## Disposition

```
port-with-edits × flag-for-reconcile   (RQ1 reviewer-architecture + RQ2 PA-role)
```

**Body treatment is contingent on RQ1 — stated explicitly so reconcile inherits a clean choice:**
- **If RQ1 keeps feasibility on PA (option i, or RQ2b persona-only):** body = **`port-with-edits`** — re-broaden the persona: remove the lines 107–109 anti-feasibility carve-out; restore *evaluate* to the framing (line 32) and the Three-Part Rule (line 67); re-add "Feasibility Reviews" to "What You Produce"; port HIL §Feasibility Review (lines 128–148) **decoupled** (the one *"(that is the reviewer's job)"* fix). Optionally extract the procedure to a new adversarial-critique skill (RQ2b).
- **If RQ1 rehomes feasibility away from PA (option ii/iii):** body for P3 collapses to **`keep-verbatim`** (the producer-only port already stands for setup) and the feasibility responsibility is tagged `moved-to-sibling-skill` (advocate) or `moved-to-validator` (generic validator) — landing on a *different* primitive, not P3.

Either way the structural move is **`flag-for-reconcile`**. The body recipe cannot be finalized until RQ1 picks the host.

---

## Responsibility trace (complete — every HIL responsibility tagged)

Each HIL `principal-architect` responsibility, traced through the setup port (done) and the plan delta (now):

| # | Responsibility (HIL) | Tag | Where / note |
|---|----------------------|-----|--------------|
| 1 | Greenfield constitution authoring | `kept` | mochiko PA via `authoring-constitution` (setup port). Not a plan delta. |
| 2 | Brownfield constitution authoring | `moved-to-sibling-skill` | folded into `authoring-constitution` greenfield\|brownfield branch (setup port). Not a plan delta. |
| 3 | Constitution quality grading (`validation-constitution`) | `moved-to-validator` | generic `validator` (setup port — the self-grade fix). **Must NOT be re-mounted on PA** when adding feasibility (guardrail #G1). |
| 4 | Codebase analysis (`analysis-codebase`) | `kept` | mochiko PA via `analysis-codebase` (setup port). Not a plan delta. |
| 5 | Evolution roadmap authoring (`authoring-roadmap`) | `moved-to-other-cluster` | DEFERRED out of plan-core (context.md §Scope; rebind-by-reference only). Tracked, not dropped. |
| 6 | CLAUDE.md governance sync (`syncing-claude-md`) | `moved-to-other-cluster` | DEFERRED cross-cutting stub (setup backlog). Tracked, not dropped. |
| 7 | **Cross-artifact feasibility review** (the procedure: 4 conflict types) | **`flag-for-reconcile`** | RQ1 host + RQ2b skill-vs-persona. Candidates: `kept`(re-broadened PA) / `folded-into-skill`(new adversarial-critique skill) / `moved-to-sibling-skill`(advocate) / `moved-to-validator`(generic validator). |
| 7b | Feasibility verdict taxonomy (feasible / needs-revision / **infeasible**) | `flag-for-reconcile` (travels with 7) | **Preserve the 3 states**, esp. `infeasible`→business-decision escalation (silent-drop #2). |
| 7c | Feasibility/completeness division-of-labor (in/out-of-scope list) | `flag-for-reconcile` + wiring | preserve the feasibility≠completeness split if two reviewers kept; decouple *"the reviewer's job"* → role. |
| 7d | Architect-report artifact format (P13 template) | `flag-for-reconcile` | separate primitive (P13); fate tied to RQ1. Cross-referenced, assessed under P13. |
| 8 | "Review ONCE before completeness" ordering + Feasibility-Rejection Loop (route on verdict, `AskUserQuestion`, re-invoke analyst) | `moved-to-lead` | the **plan command supervisor** — workflow-specific orchestration + verdict-sink. Clear regardless of RQ1. (Not strictly a PA responsibility — it *consumes* PA's verdict — but listed so it is not silently dropped with the HIL supervisor.) |
| 9 | Persona-narrowing artifacts of the current port (lines 107–109 carve-out; authoring-only framing; *write*-only Three-Part Rule) | `kept-but-rebind` *(if RQ1→PA hosts feasibility)* / `kept` *(else)* | re-broaden to HIL's "establishes **and evaluates**" framing iff PA keeps/gains the reviewer role; otherwise leave as-is. |

**No untagged responsibility remains → trace is complete (assessment done-condition met).**

---

## Silent-drop risks (the assess role's primary alarm)

1. **#1 — the entire feasibility intersection review (CRITICAL).** The "**ALREADY PORTED**" status is a **trap**: the existing port was cut for setup, is producer-only, and *actively disclaims* feasibility (lines 107–109). If the plan port reuses P3 as-is, the plan workflow **loses its cross-artifact contradiction gate entirely** — exactly the kind of mechanism-shedding-with-capability-loss the framework forbids. RQ1 **must** give this a home; it cannot be left orphaned.
2. **#2 — the 3-state feasibility verdict**, especially **`infeasible`→escalate-to-business-decision**. If RQ1 folds feasibility into the advocate (`ready/needs-revision/critical-gaps`), the distinct "fundamental conflict requiring a business-level decision" state can be flattened. Preserve all three.
3. **#3 — the feasibility-BEFORE-completeness ordering** (HIL plan.md line 908). Workflow-specific orchestration; must `moved-to-lead`, or the "don't review completeness of infeasible requirements" optimization is lost.
4. **#4 — the feasibility≠completeness division of labor.** HIL deliberately splits cross-artifact contradiction (architect) from coverage/measurability/alternatives (advocate). A careless merge (RQ1-ii) can blur two sharp adversarial lenses into one weaker review.
5. **#5 — adjacent PA-owned deferrals.** `authoring-roadmap` + `syncing-claude-md` stay deferred stubs (out of plan-core) — *not dropped*; tracked as `moved-to-other-cluster` so they are not forgotten when their clusters port.

**Guardrail #G1 (independence):** any re-broaden of PA must **NOT** re-mount `validation-constitution` (that recreates the original constitution self-grade, responsibility #3). Feasibility-review capability ≠ constitution-grading capability — different artifact domains. Reconcile must keep these straight.

---

## Reconcile flags handed to `reconcile-cluster` (the bundle)

1. **RQ1 — reviewer architecture.** Resolve {(i) two distinct validators / convention-5 two-form · (ii) fold feasibility into devils-advocate · (iii) rehome to generic `validator`}. Decides the **host** of responsibility #7 and the plan producer↔validator convention. Needs P2 (technical-analyst), P4/P9 (advocate + `validation-plan-artifacts`), P13 (architect-report-template) in view. Human-gated (contract §4a).
2. **RQ2 — PA role & driver.** Confirm produce-here/review-there is legitimate (analysis says yes, re-coupling risk LOW) → if so, authorize the **re-broaden `port-with-edits`** on P3. Decide **skill-vs-persona** for the feasibility driver (framework forward-pointers favor `folded-into-skill`; persona-only is defensible). Entangled with RQ1.
3. **Independence guardrail (#G1):** whichever option wins, never co-mount produce+grade of one artifact on one agent; never re-mount `validation-constitution` on PA.
4. **Cross-refs:** P13 architect-report-template fate rides on RQ1; verdict-sink + feasibility-rejection-loop + "once-before-completeness" ordering are `moved-to-lead` (P1 plan command) regardless of RQ1.

**Open flags after this assessment: RQ1, RQ2 (both intentionally unresolved — reconcile owns them).**
