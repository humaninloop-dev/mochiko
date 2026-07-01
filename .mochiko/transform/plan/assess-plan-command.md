# ASSESSMENT: P1 — `plan` command

**Source:** `human-in-loop/plugins/humaninloop/commands/plan.md`
**Cluster:** plan (core-only port)
**Assessed:** 2026-06-30 · ROLE: assess/diagnose ONLY (no transform, no grade, no cross-primitive resolution)
**Branch context:** 3rd command port, FIRST net-new command since the 2026-06-30 altitude fix — the altitude stress test.

---

## Class / Branch

- **Class:** command → **IS-a-loop** branch.
- The HIL `plan` command IS a loop: a prose **Supervisor** that owns the loop, drives routing on agent verdicts, manages state via `.workflow/plan-context.md`, and runs two nested loops (Feasibility Rejection + Clarification) across two phases (Analysis → Design). What matters most for this branch: who drives the loop, the done-condition, where validation + human gates sit (or are missing), **and the altitude split** — generic loop-discipline (→ `dedupe`) vs. workflow-specific orchestration (→ `moved-to-lead`).

## Triage (3 gates)

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **YES** — it *is* a markdown supervisor; drives 3 agents, owns all routing/state via `.workflow/plan-context.md`, gated on upstream specify + constitution + brownfield analysis. |
| 2 | Multi-responsibility / fans out? | **YES** — 2 phases, 3 agents, 2 nested loops, plan.md summary assembly, state recovery, 3 pre-execution gates. |
| 3 | Emits artifact whose correctness is NOT machine-checkable? | **YES** — the deliverables (requirements / constraints-decisions / nfrs / data-model / contracts / quickstart / plan.md) are model-judgment artifacts; no version/schema assert can grade them → real producer↔validator pairing needed, not a degenerate one. |

**gate1=y gate2=y gate3=y → full-lens.** (All three trip; this is the maximal case.)

## Disposition

**`redesign` × `absorb-into-lead`** — for the command itself — **+ flag-for-reconcile** on the relational sub-decisions (RQ1 reviewer architecture, P11 plan-context-template rehome, producer↔validator team casting).

- **Body = `redesign`** (not `port-with-edits`). Minimalism governor considered and rejected: the body assumes a markdown-supervisor orchestration model with a `.workflow/plan-context.md` state-carrier, inlines every `Task()` / `AskUserQuestion` payload and `supervisor_instructions` block, declares **"no hard caps"** (Important Notes), trusts the agent's verdict field instead of Reading the artifact, and ships **no acceptance gate** on the deliverable. The target is the thin shape — goal + team + per-workflow contract params + references — identical to the situation specify (329→66) and setup (385→78) were redesigned out of. An edit cannot turn "inline everything" into "reference doctrine + fill a contract artifact"; the whole orchestration model changes and new gates are added. → `redesign`.
- **Structural = `absorb-into-lead`** — a command IS its own lead; the workflow-specific orchestration stays in the thinned supervisor, the generic discipline is *referenced* (`loop-discipline` + `workflow-contract` + `agent-dispatch`), and the `.workflow/plan-context.md` state-carrier dissolves into workspace-as-state + in-session (same absorb as `context-template` / `constitution-context-template`). No orphan skill is created.
- The command's **own** placement (absorb-into-lead) is not sibling-dependent and is proposed here. The **relational** sub-decisions inside it ARE sibling-dependent and are flagged below, not decided.

---

## Responsibility trace (COMPLETE — altitude rule applied)

Keystone test on every responsibility: *true of any sound loop → `dedupe` into `loop-discipline`; only true of THIS workflow → `moved-to-lead`.*

### A. Generic loop-discipline mechanics → `dedupe` (NEVER `moved-to-lead`)

These are true of any sound mochiko loop; they are already single-sourced in `loop-discipline` (+ `workflow-contract`, `agent-dispatch`). The command *references* them.

- The produce→review→revise **iteration structure** (the Feasibility Rejection Loop + Clarification Loop as *loops*) → `dedupe`.
- **Default-FAIL done-condition mechanics** (the requirement that the artifact starts FAILing) → `dedupe`. *(HIL lacks this — see §E; the mechanics live in loop-discipline, only the params move to lead.)*
- **Producer↔validator independence doctrine** ("validation is a different agent + different skill, grading from the artifact, lead owns the verdict") → `dedupe`. *(HIL states it as prose "agents are independent, no workflow knowledge"; the doctrine is single-sourced.)*
- **Validator trustworthiness tiers** (deterministic > grounded-LLM > LLM-judge) → `dedupe`.
- **Tamper-proofing** (no PASS without evidence Read from the real artifact) → `dedupe`.
- **The four iteration guards** (hard cap / no-progress / kill-switch / escalate-don't-die) as *requirements* → `dedupe`. *(HIL's "use judgment for iteration limits (no hard caps)" violates this — the deterministic versions are added in §E; the requirement is doctrine.)*
- **Gap-type routing** (knowledge→research · preference→human · scope→halt/split) → `dedupe`. HIL's Clarification-Loop **"Research this"** branch (Explore / WebSearch / context7 for a factual unknown) is a concrete instance of knowledge-gap→research → `dedupe`. HIL's "never cross the wires" is already loop-discipline doctrine.
- **Anti-rationalization** ("always give the user the choice — never force-terminate without consent", the exhaustion≠done stance) → `dedupe`.
- **Briefing-each-dispatch** mechanics (what a good brief carries) → `dedupe` into `agent-dispatch`. *(HIL transliterates full prompt bodies inline — see §C drop.)*
- Standing supervisor footer "Do NOT modify git config or push to remote" → `dedupe` (the standard lead-footer line specify/setup already carry; not plan-specific).

### B. Workflow-specific orchestration → `moved-to-lead` (only true of THIS workflow)

Only these stay as the thinned supervisor's body. (Several depend on RQ1 for their *final* shape — noted.)

- The **2-phase analysis→design sequence** (Phase 1 = requirements + constraints-and-decisions + nfrs; Phase 2 = data-model + contracts/api.yaml + quickstart) → `moved-to-lead`.
- The **architect-feasibility-ONCE-after-Phase-1, then advocate-completeness** ordering — and its rationale ("don't waste time reviewing completeness of infeasible requirements") → `moved-to-lead`. *(Final reviewer identity = RQ1.)*
- The **skip-architect-re-review-unless-structural-change** routing (re-review only on new/changed constraints, expanded requirement scope, or modified NFR targets; clarification-only revisions go straight back to the advocate) → `moved-to-lead`. **Silent-drop risk** (§F).
- The **Phase-2 incremental review** = *full* design review **+** a 2–3 min consistency check of the Phase-1 analysis artifacts (the dual-mode advocate pass) → `moved-to-lead`. **Silent-drop risk** (§F). *(Maps to validation-plan-artifacts P2/incremental — RQ1.)*
- **Team casting** — technical-analyst produces both phases; feasibility reviewer grades Phase 1; completeness reviewer grades both phases → `moved-to-lead`. *(The exact roster = RQ1 + technical-analyst port — flagged.)*
- **This loop's done-condition parameters** (measurable end state: all six analysis+design artifacts present and validated; the cap *number*; the named gate placements) → `moved-to-lead` (filled into the `workflow-contract` artifact, not inlined).
- **Phase 4 completion** — assemble `plan.md` summary by extracting key decisions (constraints-and-decisions), entity summary incl. sensitivity (data-model), endpoint summary incl. integrations (contracts/api.yaml) → `moved-to-lead` (workflow-specific deliverable assembly).
- **Pre-execution prerequisite wiring** (workflow-specific upstream handoff edges) → `moved-to-lead`:
  - **Constitution prerequisite** (plan requires `constitution.md`; HIL hard-blocks → setup). *(Keep-hard-block vs. soften-to-specify's-pattern is a lead design param, not relational.)*
  - **Entry gate** — specify must be complete: spec.md exists (rebound — see §C) and accepted.
  - **Brownfield check** — read constitution `project_type`; require `codebase-analysis.md`; staleness (>14d) warning; greenfield bypass.
- **Empty-input `@`-reference recovery** (the known Claude Code bug; AskUserQuestion re-enter / proceed) → `moved-to-lead` (same workflow-specific input handling as specify's G1).
- **State recovery / resume** — resume the in-progress run → `moved-to-lead`, **rebound** to workspace evidence (HIL resumes from `phase`+`status` fields in plan-context.md; mochiko resumes from workspace-as-state — see §C).
- **Existing mid-loop human gates** → `moved-to-lead` (must survive alongside the NEW acceptance gate — §E, §F):
  - **Feasibility Rejection Loop** gate (present architect concerns; user picks accept-resolution / relax / keep-as-is / provide-direction) — a preference-gap escalation.
  - **Clarification Loop** gate (present advocate gaps; user answers; "Research this" option) — preference-gap + the knowledge-gap research branch.
  - **"When to Exit Early"** offering (continue-refining / accept-current / stop-and-review). *(The generic "escalate, don't force-terminate" half is `dedupe` §A; only the plan-specific offering placement is `moved-to-lead`.)*
- **Operational handling** — verify-agent-output (confirm each expected file was created; on missing → log + ask retry/abort) and Agent-Failure messaging → `moved-to-lead`. *(Generic supervisor hygiene, but the specific file lists are plan-specific.)*

### C. Content / path / state-carrier couplings

- `.humaninloop/memory/constitution.md`, `.humaninloop/memory/codebase-analysis.md` → **`kept-but-rebind`** (`.humaninloop/` → `.mochiko/`).
- `specs/{feature-id}/…` workspace + `${CLAUDE_PLUGIN_ROOT}/templates/…` refs → **`kept-but-rebind`** (to `.mochiko/specs/<feature>/…` to match specify's workspace-as-state; the round reports `techanalyst-report.md` / `architect-report.md` / `advocate-report.md` survive as per-round artifacts under the workspace, like specify's analyst/advocate reports).
- **Entry-gate read of specify's `.workflow/context.md` `status` field** → **`kept-but-rebind`** — rebind to "spec.md present and accepted" workspace evidence; do **not** transliterate the status-field read (specify is now workspace-as-state, no context-file `phase`/`status`). **Transliteration risk** (§F).
- `.workflow/plan-context.md` **state-carrier** (the P11 template: all `{{…}}` placeholders, `supervisor_instructions`, `phase`/`status`/`iteration` fields, File-Paths table, Clarification Log) → **`dropped + reason`**: kernel-adjacent markdown state-carrier; mochiko uses workspace-as-state + in-session (same absorb as `context-template` / `constitution-context-template`). **The P11 plan-context-template absorb is FLAGGED for reconcile (§F).**
- Inlined `Task(...)` / `AskUserQuestion(...)` payload bodies and the full `supervisor_instructions` prose blocks (§2.1 / 2.5 / 2.7 / 3.1 / 3.5) → **`dropped + reason`**: transliterated mechanics; `agent-dispatch` briefing replaces inline prompt-body transliteration, and inlining them is the altitude defect `verify-output` now fails. The briefing *content* (which inputs to Read, where to Write, which skills to hint) → folded into concise `agent-dispatch`-style params (`moved-to-lead`).
- HIL `iteration` counter spelled out inline ("Use judgment… no hard caps") → **`dropped + reason`**: replaced by the contract's deterministic cap (§E); a hand-coded LLM-judged counter is the transliteration `verify-output`'s altitude floor rejects.

### D. Producer-side content embedded in the supervisor → other cluster / skill (NOT command-body)

These ride inside HIL's `supervisor_instructions` but are **producer** responsibilities (technical-analyst + its skills), not command-level orchestration. They must NOT be carried into the thin command, and must NOT be silently lost — they re-home to the plan **producer** cluster when it ports.

- **Infrastructure Planning** (IP-XXX in constraints-and-decisions Part 3) → `moved-to-other-cluster` (the `authoring-technical-requirements` skill / `technical-analyst` agent — both `[ ]` in REGISTRY).
- **Data-sensitivity annotations** (PII/Confidential/Restricted classification in data-model) → `moved-to-other-cluster` (`patterns-entity-modeling`).
- **Integration boundaries** (`x-integration` in contracts) → `moved-to-other-cluster` (`patterns-api-contracts`).
- The **advocate focus-area checklists** (FR-coverage, orphan-TRs, NFR-measurability, constraint-actionability, sensitivity-contract / integration-contract / infrastructure-design alignment) → `moved-to-other-cluster` (the validator-side skill `validation-plan-artifacts`, `[ ]` in REGISTRY, stubbed on mochiko devils-advocate for re-mount-when-plan-ports). **Silent-drop risk** (§F): the thin command drops the inline `supervisor_instructions` wholesale, so the producer/validator skills MUST carry this content.
- **techspec-merge note:** HIL merged the deprecated `techspec` INTO plan; its design responsibilities (data-model / contracts / NFRs / constraints) are *already* plan's Phase-1/2 work, covered by the §B 2-phase sequence. No separate techspec assessment (per instructions); recorded so the merge is not read as a silent drop.

### E. Missing loop-discipline gates to ADD (HIL lacks them)

Requirement = `dedupe` (mechanics live in `loop-discipline`); placement/params = `moved-to-lead` (filled into the contract).

- **Default-FAIL done-condition** — HIL has none (the run can self-declare done on a `ready` verdict at pass 1). ADD: artifact starts FAIL; clears only on independent validation + (new) human acceptance. Requirement `dedupe`; params `moved-to-lead`.
- **Lead-OWNED verdict** — HIL's supervisor reads the agent's verdict *field* and routes on it (orchestrator-evaluated, trusts the report). ADD: the lead **Reads** spec/analysis/design artifacts + the reviewer report and owns the clearing verdict; the reviewer's status is **input, not the gate** (the same reversal specify applied to the advocate's status). Doctrine `dedupe`; the reversal note is a workflow-specific design note `moved-to-lead`.
- **Hard bound + kill-switch** — HIL explicitly has "no hard caps" + only a model-judged "gaps aren't resolving" heuristic. ADD: deterministic round cap (number `moved-to-lead`), no-progress exit (unchanged gap/fix set round-over-round), kill-switch file. Requirements `dedupe`; params `moved-to-lead`.
- **NEW human ACCEPTANCE gate on the deliverable** — HIL has mid-loop human gates (feasibility, clarification, exit-early) but **no final acceptance gate** on `plan.md`. ADD a named acceptance gate (accept → done / amend → bounded re-enter / reject → abort), exactly like specify's G3 and setup's G3. Requirement (named human gate) `dedupe`; the placement `moved-to-lead`. **Must not displace** the existing mid-loop gates (§B, §F).

---

## Reconcile flags (relational — NOT decided here)

1. **RQ1 — reviewer architecture** *(the convention-5 two-form case).* HIL plan has **TWO** reviewers: **Principal Architect** = feasibility / cross-artifact-contradiction (an *adversarial-critique* / judgment form, Phase-1 only) and **Devil's Advocate** = completeness via `validation-plan-artifacts` (a *mirror-checklist* form, both phases — ROADMAP §80's two forms map cleanly onto these two reviewers). Resolve in reconcile, with full sibling context:
   - **(a) keep two distinct validators** — adversarial-critique feasibility reviewer + checklist completeness advocate (the genuine two-form case);
   - **(b) fold feasibility into the advocate** — the one-reviewer specify shape;
   - **(c) rehome feasibility onto the generic `validator`**.
   Sibling context reconcile must weigh: mochiko **`devils-advocate`** already exists (specify critic; `validation-plan-artifacts` *stubbed* for re-mount when plan ports); mochiko **`validator`** is the generic grader; and **feasibility-review is currently homeless** — mochiko `principal-architect` was ported setup-scoped *producer-only*, and the trace-tag reference example tags HIL's "cross-artifact feasibility review → folded-into-skill (new skill)". Whichever option wins, **independence is non-negotiable**: the producer (technical-analyst) must never also grade. Do NOT resolve a pairing onto one agent.
2. **P11 — `plan-context-template` rehome.** Disposition signal: `absorb-into-lead` (workspace-as-state + in-session, mirroring `context-template` / `constitution-context-template`), template `dropped`. Confirm in reconcile (it is a separate primitive; its absorb is a cross-primitive structural call). Per instructions, flagged not decided.
3. **Producer↔validator team casting / pairing.** Producer **`technical-analyst`** is `[ ]` (not yet ported); the validator roster depends on RQ1. The pairing (who produces, who grades, disjoint skills) is a reconcile `pair`/casting decision once RQ1 lands and technical-analyst is assessed. Independence is structurally sound in HIL (analyst ≠ architect ≠ advocate) — preserve it.

*(Cluster dependencies, not command-body flags — noted for the cluster port, not decided here: `plan-template.md`, `architect-report-template.md`, `techanalyst-report-template.md`, `advocate-report-template.md`, `cross-artifact-checklist.md`, and the producer skills `authoring-technical-requirements` / `patterns-entity-modeling` / `patterns-api-contracts` / `patterns-technical-decisions`. The §D producer-side content lands in these.)*

## Silent-drop risks (for the lead/human to accept)

- **Skip-architect-re-review-unless-structural-change** (§B) — a subtle round-saving optimization, trivially lost in a thin rewrite. Must survive as an explicit lead routing param.
- **Phase-2 dual-mode incremental review** (§B) — full design review + 2–3 min Phase-1 consistency check; easy to flatten to "review the design." Tied to RQ1 (validation-plan-artifacts P2/incremental).
- **§D producer-side content** (IP-XXX infra planning, data-sensitivity, integration boundaries, the advocate focus-area checklists) — the thin command drops the inline `supervisor_instructions`; these capabilities survive ONLY if the producer/validator **skills** carry them. Highest-volume silent-drop surface.
- **Existing mid-loop human gates** (feasibility / clarification / exit-early) — must not be displaced by the new acceptance gate; ADD the acceptance gate, KEEP these.
- **Entry-gate `.workflow/context.md` status read** (§C) — must rebind to workspace evidence, not be transliterated; specify no longer writes that status field.
- **"Research this" knowledge-gap→research branch** — a real capability; `dedupe`'d to loop-discipline's gap-routing, but the lead must actually route knowledge gaps to `Explore` (don't lose it in the dedupe).

---

## Output block

```
ASSESSMENT: P1 — plan command
Class:        command → branch IS-a-loop
Triage:       gate1=y gate2=y gate3=y  → full-lens
Disposition:  redesign × absorb-into-lead  (+ flag-for-reconcile: RQ1 reviewer architecture · P11 plan-context-template rehome · producer↔validator team casting)
Trace:        complete — every responsibility tagged (see §A–§E)
  altitude split → dedupe (generic): iteration structure · default-FAIL mechanics · independence doctrine ·
                   validator tiers · tamper-proofing · the four guards (requirements) · gap-type routing
                   (incl. "Research this") · anti-rationalization · briefing mechanics · git-footer
                → moved-to-lead (workflow-specific): 2-phase analysis→design sequence · architect-feasibility-
                   ONCE-then-advocate ordering · skip-architect-unless-structural routing · Phase-2 dual-mode
                   incremental review · team casting · done-condition params (cap #, gate placements) ·
                   plan.md assembly · constitution/entry/brownfield prerequisites · @-input recovery ·
                   state-recovery (rebound) · feasibility+clarification+exit-early human gates · op handling
                → kept-but-rebind: .humaninloop→.mochiko · specs/→.mochiko/specs · round reports ·
                   entry-gate status→workspace evidence
                → dropped+reason: plan-context.md state-carrier (FLAG P11) · inline Task/AskUserQuestion
                   payloads + supervisor_instructions (transliteration) · LLM-judged iteration counter
                → moved-to-other-cluster: IP-XXX infra · data-sensitivity · integration boundaries ·
                   advocate focus-area checklists (→ technical-analyst + producer/validator skills)
  ADD (HIL gaps): default-FAIL done-condition · lead-owned verdict (status=input) · hard cap + kill-switch ·
                   NEW human acceptance gate on plan.md
Reconcile flags: RQ1 reviewer architecture (two-form) · P11 plan-context-template rehome · producer↔validator casting
Silent-drop risks: skip-architect routing · Phase-2 dual-mode review · §D producer content · existing mid-loop
                   gates · entry-gate status rebind · "Research this" branch
```
