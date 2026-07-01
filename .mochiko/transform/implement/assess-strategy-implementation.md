# Assessment — P7 `strategy-implementation` skill

**Run:** `transform-cluster implement` · **Primitive:** P7 `skills/strategy-implementation/SKILL.md` (90 ln, single file, no reference bundle) · **Assessed:** 2026-07-01
**Producer:** `mochiko:transform-producer` · **Skill:** `assess-primitive` (branch → PLAYS-a-role)
**Sole HIL consumer:** `agents/state-analyst.md` (`skills: [strategy-core, strategy-specification, strategy-implementation]`) — the dissolving brain/DAG agent
**Primary overlap targets:** `plugins/mochiko/skills/loop-discipline/SKILL.md` (dedupe) · P1 `implement` command lead (rehome) · the dissolved State-Analyst implement-slice
**Precedent:** the **5th consecutive strategy dissolution** — `strategy-core` (P10, specify: near-total dedupe + Gap-Classification fold) and `strategy-specification` (P9, specify: dedupe + survivors→lead) both dissolved into `loop-discipline` in the specify run; REGISTRY line 68 predicts "the same dissolution into `loop-discipline`."

---

## Output (skill format)

```
ASSESSMENT: strategy-implementation
Class:        skill → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=n  → full-lens
Disposition:  drop (universal/advisory layer NOT carried as a standalone skill; patterns either
                    already live in loop-discipline (dedupe) or rehome to the P1 lead)
              × flag-for-reconcile
Loop-discipline edit needed?  NO. Nothing is genuinely-universal-but-missing. The one universal
              gap the strategy family carried — Gap Classification — was already folded into
              loop-discipline in the specify run. This is the CLEANEST strategy dissolution:
              zero loop-discipline additions, zero residual skill.
Reconcile flags: FR-A  dedupe-assignment (TRACE rule 3) — the human gate accepts DROP of the
                       standalone skill; contract §4(a) names this disposition explicitly
                 FR-B  RESOLVE JOINTLY WITH P1 FR-1 — every moved-to-lead survivor here is the
                       SAME responsibility P1 already enumerated (L2, L6–L9); rehome ONCE, do not
                       double-home. This flag MERGES into P1 FR-1, it does not compete with it.
                 FR-C  cross-workflow shared-substrate — strategy-implementation is state-analyst
                       substrate; its dissolution must be cluster-global (no strategy-* skill
                       survives in mochiko for any workflow)
```

---

## Step 1 — Class & branch

A **skill** → **PLAYS-a-role**. Weight (per branch): consumed-procedure vs emits-artifact, the **sibling/overlap check** (the heavy one — this is where dedupe vs rehome is decided), and the **decoupling scan** (this run's empirical doctrine test, 5th surface). Classification/router/trigger wiring is largely moot because the standalone skill is not carried; its content lands on primitives that are already wired.

## Step 2 — Fast-path triage gate

- **gate1 Orchestration-coupled? YES.** Two ways. (a) *Framing + consumer:* the skill exists to be "consumed alongside `strategy-core` to produce targeted briefings" (desc L3, Overview L10) for the DAG-coupled `state-analyst`, which dissolves kernel-free. (b) *Body vocab:* it carries DAG/brain concepts directly — "execution node / verification node" (L47), "within the same pass" / "across implementation passes" (L18, L47), "final-validation **gate** verdict `ready`" + "implementation-complete **milestone** `achieved`" (L29, L34–35).
- **gate2 Multi-responsibility / fans out? YES.** Five core patterns + three guardrails + five anti-patterns; and it is implement-workflow substrate fanned out across the implement passes via the state-analyst.
- **gate3 Non-machine-checkable artifact? NO.** Emits **no** artifact — it is a consumed advisory-procedure (all model judgment; no deterministic script → no producer↔validator partner is real).

One+ yes → **full 7-check lens.**

---

## Step 3 — The lens (weighted for PLAYS-a-role)

**1 · Orchestration test (content vs orchestration split).**
- *Content-coupling: MODERATE.* The pattern *bodies* are mostly universal loop prose, but the wrapper carries DAG vocab (node/pass/gate/milestone) and the sibling-briefing frame.
- *Orchestration-coupling: consumer-dissolves, not driven.* Nothing sequences this skill; it is simply *read* by the state-analyst to brief the Supervisor, which drives the `implement-catalog.json` DAG. Its sole consumer dissolves. **Who inherits the consumption?** In mochiko the P1 command **is** the lead that owns the implement loop directly and already consumes `loop-discipline`. So the patterns split cleanly: the *sound-loop* patterns are content the lead's doctrine skill (`loop-discipline`) already holds → **dedupe**; the *implement-specific* patterns become the lead's own orchestration content → **moved-to-lead** (and P1's trace already holds them). This is the crux that makes the fate dedupe + rehome, never a carried skill.

**2 · Role at two altitudes.**
- *Skill-role:* consumed-procedure (advisory heuristics), **not** an artifact-emitter → no producer↔validator partner needed.
- *Conferred team-role:* it informs the **lead/referee** reasoning (how to run a sound implement loop) — precisely `loop-discipline`'s + the P1 lead's territory. Confers neither producer nor validator.

**3 · Independence.** **No self-grade leak** — strategy-implementation grades nothing. Notably its *content* about independence — Execute-then-Verify's "Never skip verification, even when the cycle report claims all tasks passed … Self-reported success is unreliable" — is itself the `loop-discipline` Req-2 doctrine restated (→ dedupe; see check 5). It encodes the independence principle rather than violating it.

**4 · Verdict-sink / loop-driver.** No verdict emitted. The loop-driving *guidance* it encodes (execute→verify pairing; retry-then-escalate; fix-pass scoping) was advisory input the state-analyst fed the Supervisor to drive the DAG. In mochiko, loop-driving is owned by the P1 lead's bounded loop; the *doctrine half* dedupes into `loop-discipline` Req 3, the *implement-parameter half* rehomes to the lead. This is exactly the biggest thing the kernel/supervisor owned — and P1 already reclaimed it (L6–L9, L2).

**5 · Sibling / overlap — THE heavy check.** Precise per-pattern map (this is the headline verdict table the run asks for):

| # | Pattern (HIL lines) | Verdict | `loop-discipline` coverage / rehome target |
|---|---------------------|---------|--------------------------------------------|
| 1 | **Cycle Sequencing** (L39–43): foundation cycles before feature; current cycle = first with unchecked `tasks.md` tasks | **rehome-to-lead** | NOT in loop-discipline (workflow-specific: "foundation-before-feature" and "read `tasks.md` checkboxes" only make sense for a TDD implement loop with cycles). → P1 **L6** = constraint **(a)**. → FR-B |
| 2 | **Execute-then-Verify** (L45–49): every execution node followed by a verification node in the same pass; never skip even when the report claims all passed; gates run independently of the implementer | **SPLIT** — *principle* → **dedupe**; *pairing shape* → **rehome-to-lead** | *Principle* (independent verification because self-report is unreliable; "systematic blind spots") = **Req 2** (external independent validation) + Req-2 tamper-proofing ("A PASS is invalid unless the evidence was Read from the real artifact"). *Pairing shape* (every staff cycle → a qa verification, same round, never skipped) → P1 **L7** = constraint **(b)** + casting FR-2. → FR-B |
| 3 | **Targeted Retry** (L51–55): trace checkpoint failure to responsible tasks, re-open only those, don't full-reimplement | **SPLIT** — *iterate-on-FAIL backbone* → **dedupe**; *scoping tactic (+ max-3/cycle param)* → **rehome-to-lead** | *Backbone* (a loop iterates on FAIL, bounded) = **Req 3** loop mechanics. *Scoping* (re-open only failed tasks; don't rewrite working code) + the max-3/cycle number → P1 **L8** = constraint **(c)**. **Precedent-locked:** the sibling tactic (`strategy-specification` targeted-revision — "carry the specific flagged gaps forward … do NOT rewrite clean sections") was tagged **moved-to-lead** in specify, not a loop-discipline addition. → FR-B |
| 4 | **Escalate Before Stall** (L57–61): after 3 attempts (or 3 fix passes), escalate to the user with the report | **dedupe** (doctrine) + cap-number → **lead** (param) | **Req 3.4** ("Escalate, don't silently die — hand off to the human gate **with failure context**; never report done because you ran out of rounds") + **Req 4** (human gate) FULLY cover the doctrine. The literal "3" is a lead-set contract parameter (P1 **L8/L9**), the way `strategy-specification`'s "after 5 passes" was a parameter. **No gap.** |
| 5 | **Fix Pass Scoping** (L63–67): a fix pass is scoped strictly to the validation-report failures; not a refactoring/quality-improvement license; address exactly what failed | **rehome-to-lead** | NOT in loop-discipline ("fix pass" is a named implement mode — the post-final-validation staff fix-mode; "scope strictly to reported failures" is implement orchestration). → P1 **L9** = constraint **(d)** + the fix-*craft*/fix-*routing* split FR-3. → FR-B |

- *Done-condition survivors (Goal L27–29 + Success Criteria L31–35):* "Complete all implementation cycles with all gates passing; final-validation verdict `ready`; all `tasks.md` `[x]`; implementation-complete achieved." This is the implement workflow's **done-condition CONTENT** (loop-discipline mandates a done-condition; this fills it in) → **moved-to-lead** (P1 **L2** + the filled `workflow-contract`), directly analogous to `strategy-specification`'s "spec done-condition content → moved-to-lead." The DAG "gate verdict / milestone" *labels* drop; the substance survives.
- *Cross-sibling note:* the `state-analyst` dissolves and its implement-slice re-homes to the same lead; P1 already flagged that overlap (FR-1). Reconcile resolves them together.
- *Cross-workflow note:* strategy-implementation is state-analyst substrate; its dissolution has cluster-global reach — **no `strategy-*` skill survives in mochiko for any workflow** (strategy-core + strategy-specification already gone; this closes the family). → FR-C.

**6 · Coupling audit.**
- *Hardcoded paths:* **none** — no `.humaninloop/` paths to rebind (`tasks.md` is referenced by name only; the path binding lives on the P1 lead, `.mochiko/specs/<feature>/tasks.md`, already handled in P1 R1/R2).
- *Prerequisites/handoffs:* none of its own — advisory read; assumes the implement DAG already assembled its context.
- *Determinism boundary:* **all model-judgment**, no deterministic script → confirms no validator partner is real (degenerate), consistent with gate3=n.

**7 · Conventions + loop placement + DECOUPLING SCAN.**
- Classification / router / triggers / pairing: **moot** — the bulk dedupes into `loop-discipline` (already a classified, routed, model-invoked doctrine skill) and the survivors land on the P1 command (already user-invoked + router-registered by its own port). No residual skill to wire.
- *Loop placement:* strategy-implementation only **describes** the execute→verify / retry / escalate discipline *advisorily*; `loop-discipline` encodes it as **requirements** and the P1 lead instantiates the parameters — which is exactly why it is superseded, not co-existing. HIL frames these as "heuristics … the Supervisor uses judgment and may deviate"; the mochiko successor is non-negotiable ("You cannot rationalize your way out of any of the four"). The **advisory / may-deviate stance is dropped** (superseded), matching P10 R10.

### DECOUPLING-SCAN hits (run goal — 5th empirical doctrine test)

strategy-implementation is a **textbook carrier** of deny-list tokens (grep-confirmed):

| Token | Where | Deny-list class |
|-------|-------|-----------------|
| **`strategy-core`** | desc L3 + Overview L10 ("Consumed alongside `humaninloop:strategy-core`") | **sibling-skill name** |
| **`strategy-specification`, `executing-tdd-cycle`, `testing-end-user`** | When-NOT-to-Use L22, L24, L25 ("Use X instead") | **sibling-skill names** |
| **`hil-dag` MCP tools** | When-NOT-to-Use L23 | kernel/DAG/MCP vocab |
| **"targeted briefings" / "briefings for … passes"** | desc L3, Overview L10, When-to-Use L14 | brain-briefing (state-analyst→Supervisor) framing |
| **"execution node / verification node" / "within the same pass" / "across implementation passes"** | L18, L47 | DAG node + workflow-phase ("pass") vocab |
| **"final-validation gate verdict" / "implementation-complete milestone"** | L29, L34–35 | DAG gate/milestone vocab |
| **"heuristics … advisory … Supervisor may deviate"** (concept, via strategy-core framing) | Overview | "workflow-agnostic"/advisory meta-label |

**Doctrine-test result (positive — the cleanest of the family):** because the standalone skill is **dropped**, these tokens **vanish structurally with the skill** — decoupling by *absence*, not by edit. And **unlike strategy-core, there is NO residual skill to scrub** (strategy-core's gap-classification residual had to be re-homed and cleaned; strategy-implementation's survivors go to the **P1 command lead**, where naming agents and "dispatch" is *legitimate* — the deny-list targets personas/skills, not the command supervisor; see P1 Check 7). So the decoupling outcome is zero-scrub-risk. A clean 5th data point that HIL's brain-era strategy skills carry deny-list tokens by construction and mochiko removes them by structural dedupe, not cosmetic editing.

---

## The "genuinely-universal-but-missing" check — is a `loop-discipline` edit needed?

**No — nothing is missing. No gated `loop-discipline` edit this run.** Walking each pattern against the current `loop-discipline` (which now includes the specify-run Gap-Classification fold):

- Pattern 2 principle (independent verification, distrust self-report) → already **Req 2** + its tamper-proofing. Covered.
- Pattern 4 (bounded attempts → escalate to the human with context) → already **Req 3.4 + Req 4**. Covered verbatim in spirit.
- Patterns 1, 5 and the scoping half of Pattern 3 are **workflow-specific** (cycles, foundation/feature, fix-pass mode, tasks.md checkboxes) — correctly absent from a workflow-agnostic doctrine skill; they belong on the lead, not in `loop-discipline`.

**The one candidate that could masquerade as universal — adjudicated:** "on retry, carry the specific failures forward and re-open only the failed slice; don't redo clean work" (Targeted Retry #3 / Fix Pass Scoping #5). This *looks* like it might be a general iteration principle worth adding. It is **not** added, for two reasons: (1) **precedent** — the identical sibling tactic (`strategy-specification` targeted-revision) was assessed as **moved-to-lead** in the specify run, not folded into `loop-discipline`; adding it here would be an inconsistency, and a foundational-primitive change needs the human gate for a reason. (2) **altitude** — the tactic is only ever expressed in workflow vocabulary (tasks / cycles / checkpoint report / fix pass); its thin universal residue ("converge by not undoing progress") is already implied by Req 3's bounded-iteration + no-progress machinery. Default holds: **already covered.**

**Why this dissolution is cleaner than strategy-core's.** strategy-core had exactly one genuine survivor (Gap Classification) that *was* missing and required a gated additive edit to `loop-discipline`. strategy-implementation has **zero** — because the family's one universal gap was already closed in the specify run, and everything remaining here is either already-covered sound-loop doctrine or workflow-specific implement orchestration. Pure dedupe + rehome; no doctrine edit; no residual skill. This is the strongest possible confirmation of the cluster thesis: `loop-discipline` (+ the P1 lead) has fully absorbed the strategy layer.

---

## Step 4 — Disposition

**body `drop`** — the standalone `strategy-implementation` skill is **not carried** into mochiko. No `strategy-implementation.md` exists in mochiko form; its patterns either already live in `loop-discipline` (dedupe) or become the P1 lead's own orchestration content (moved-to-lead). This is `drop`, not `redesign`: unlike `strategy-specification` (P9), there is **no residual skill to re-express kernel-free** — the survivors land on the command lead, which P1's port already builds. And unlike `strategy-core` (P10), there is **not even a reserved `port-with-edits` residual**, because no additive `loop-discipline` slice survives. Not `keep-verbatim` (deny-list tokens + DAG vocab throughout); not a full responsibility-drop (the responsibilities SHOULD exist in mochiko — they are relocated, not deleted).

**× structural `flag-for-reconcile`.**

Rationale (why the structural verdict is reconcile's, not solo): `dedupe` is a **relational** verdict `reconcile-cluster` assigns, never solo assessment (TRACE-TAGS rule 3). More decisively, **every `moved-to-lead` survivor here is the SAME responsibility P1 already enumerated** (Cycle Sequencing=L6, Execute-then-Verify pairing=L7, Targeted Retry=L8, Fix Pass Scoping=L9, done-condition=L2) and overlaps the dissolved State-Analyst implement-slice. The rehome map must be built **ONCE, cluster-wide** — P1 already opened **FR-1** for exactly this "(a)–(g) rehome split [overlaps P7 + the dissolved State-Analyst]." This assessment therefore **does not unilaterally assign** the shared survivors; it hands them to reconcile to land once. The realized post-reconcile structural move will be `absorb-into-lead` (survivors → P1) + `dedupe` (doctrine → `loop-discipline`), with **no `loop-discipline` edit**.

**Human-gate note:** contract §4(a) names "`strategy-implementation` dissolution into `loop-discipline` + which survivors rehome to the lead vs. are already covered" as an explicit Phase-2 gated bundle. The human accepts the DROP of the standalone skill and the survivor split at reconcile.

---

## Step 5 — Responsibility trace (COMPLETE — every line lands; no silent drop)

> Enumerated by walking the skill top-to-bottom. `(a)`–`(d)` map to contract §1 constraints. `[=P1 Lx]`
> marks the SAME responsibility P1's trace already holds (reconcile lands it once — FR-B). No P7 survivor
> introduces a new orphan the lead doesn't already carry; P7 is the *doctrine source* for P1's enumeration.

### Bucket (i) — `dedupe` → `loop-discipline` · generic sound-loop doctrine (do NOT re-inline; already single-sourced) — **4**
| # | Responsibility (source lines) | Single-source |
|---|-------------------------------|---------------|
| DD1 | Execute-then-Verify **principle** — independent verification required; distrust self-report; "systematic blind spots" (L45–49; Common-Mistake "Skipping Verification" L77–78) | `loop-discipline` **Req 2** + tamper-proofing |
| DD2 | Iterate-on-FAIL **backbone** under Targeted Retry — a loop re-runs on failure, bounded (L51–55 substrate) | `loop-discipline` **Req 3** loop mechanics |
| DD3 | Escalate-Before-Stall **doctrine** — at the attempt cap, escalate to the human with the report rather than continuing; N failures = structural problem (L57–61; Common-Mistake "Continuing Past 3 Retries" L83–84) | `loop-discipline` **Req 3.4** (escalate-don't-die) + **Req 4** (human gate) |
| DD4 | "Assessing convergence signals across implementation passes" (When-to-Use L18) — converging vs recurring | `loop-discipline` **Req 3.2** (no-progress exit) *(this is strategy-core's Pass-Evolution flavor, already deduped in specify; not a distinct 6th pattern)* |

### Bucket (ii) — `moved-to-lead` · workflow-specific implement orchestration (= P1 survivors; reconcile lands once) — **6**
| # | Responsibility (source lines) | Note |
|---|-------------------------------|------|
| L-a | **(a)** Cycle Sequencing — foundation cycles before feature; current cycle = first with unchecked `tasks.md` tasks (L39–43) | `[=P1 L6]` → FR-B |
| L-b | **(b)** Execute-then-Verify **pairing shape** — every staff cycle followed by a qa verification in the same round, never skipped (L45–47; Guardrail L72; Common-Mistake L77–78) | `[=P1 L7]` + casting FR-2 → FR-B |
| L-c | **(c)** Targeted Retry **scoping tactic** — trace checkpoint failure to its tasks, re-open only those, don't full-reimplement; **max 3/cycle** parameter (L51–55; Guardrail L73; Common-Mistake "Full Re-Implementation" L80–81) | `[=P1 L8]` → FR-B |
| L-d | **(d)** Fix Pass Scoping — post-final-validation fix pass scoped strictly to reported failures, no refactoring/sweeping changes; **max 3 fix passes** parameter (L63–67; Guardrail L73; Common-Mistake "Using Fix Passes for Refactoring" L89–90) | `[=P1 L9]` + fix-craft/routing split FR-3 → FR-B |
| L-e | **Done-condition CONTENT** — Goal + Success Criteria: all cycles complete/all gates pass; all `tasks.md` `[x]`; final-validation `ready`; implementation-complete (L27–35) | `[=P1 L2]` + the filled `workflow-contract`; DAG "gate/milestone" labels drop, substance survives → FR-B |
| L-f | **Ordering guard** — all tasks `[x]` before final-validation runs (Guardrail L71; Common-Mistake "Premature Final-Validation" L86–87) | lead sequencing rule (rides with L-e done-condition ordering) → FR-B |

### Bucket (iii) — `dropped + reason` · brain-briefing / DAG / sibling-routing framing that dissolves — **5**
| # | Responsibility (source lines) | Reason (lead/human gate must accept) |
|---|-------------------------------|--------------------------------------|
| D1 | Frontmatter trigger `description` — "MUST be invoked when the user says 'implementation strategy'…"; "Provides … patterns consumed alongside strategy-core for targeted briefings" (L3) | The skill dissolves; its user-trigger + briefing framing vanish with it. Brain-era (state-analyst→Supervisor briefing) vocab; `strategy-core` itself already dissolved. |
| D2 | Overview + When-to-Use consumption framing — "Consumed alongside `humaninloop:strategy-core` to produce targeted briefings"; "Producing briefings for implementation workflow passes" (L10, L14) | State-analyst dissolves; the brief-the-Supervisor consumption model is kernel-era. The lead consumes `loop-discipline` directly + owns the loop. |
| D3 | When-NOT-to-Use sibling/DAG routing — "Use `strategy-specification` / `hil-dag` MCP / `executing-tdd-cycle` / `testing-end-user` instead" (L22–25) | Routing vocabulary for a dissolving skill; the named strategy siblings also dissolved; `hil-dag`/MCP shed (kernel-free). |
| D4 | DAG vocabulary — "execution node / verification node," "within the same pass," "gate verdict," "milestone `achieved`" (L18, L29, L34–35, L47) | Kernel-free; DAG node/pass/gate/milestone machinery shed. The *substance* under these labels survives at DD1 / L-b / L-e; only the labels drop. |
| D5 | "Heuristics … advisory … the Supervisor uses judgment and may deviate" stance (Overview framing via strategy-core) | Superseded by `loop-discipline`'s non-negotiable framing ("you cannot rationalize your way out of any of the four"); carrying it would contradict doctrine. Matches P10 R10. |

**Bucket counts:** dedupe **4** · moved-to-lead **6** · dropped **5**. Every section of the 90-line skill (frontmatter, Overview, When-to-Use, When-NOT-to-Use, Goal, Success Criteria, all 5 Core Patterns + rationales, all 3 Guardrails, all 5 Common Mistakes) is accounted for. **Pattern rationales** travel with their pattern's tag (the rationale for a moved-to-lead parameter rides to the lead as its justification; the rationale for a deduped doctrine is already in `loop-discipline`). **No silent drop.**

**Headline:** of the 10 substantive responsibilities, **4 dedupe** straight into `loop-discipline` (already there), **6 rehome to the P1 lead** (already enumerated in P1's trace), **5 framing responsibilities drop** as brain/DAG/briefing vocab, and **0 survive as new content requiring a residual skill or a `loop-discipline` edit.** Confirms REGISTRY's prediction and completes the strategy family's dissolution.

---

## Reconcile flags (for `reconcile-cluster` — cannot be decided solo)

**FR-A — assign the dedupe + accept the drop (TRACE rule 3).** Confirm DD1–DD4 dedupe into `loop-discipline` (no edit — already covered) and the human gate accepts the **DROP of the standalone `strategy-implementation` skill**. Contract §4(a) names this as an explicit Phase-2 gated disposition.

**FR-B — RESOLVE JOINTLY WITH P1 FR-1 (the (a)–(g) rehome split).** This is the load-bearing flag. The six `moved-to-lead` survivors (L-a…L-f) are the **same responsibilities** P1 already enumerated (L6/L7/L8/L9/L2) and overlap the dissolved State-Analyst implement-slice. P7 is the *doctrine source*; P1 is the *consumer*. Reconcile must land each ONCE on the thin lead and confirm the `dedupe`→`loop-discipline` vs `moved-to-lead` boundary per constraint (the caps/no-progress/escalation *doctrine* dedupes; the implement *parameters* — foundation-before-feature, max-3/cycle, max-3-fix-passes, execute→verify-every-round, the done-condition end-state — rehome). **This flag MERGES into P1 FR-1; it does not create a competing rehome claim.** Do not double-home any survivor.

**FR-C — cross-workflow shared-substrate / close the family.** strategy-implementation was state-analyst substrate; its dissolution is cluster-global. Confirm that **no `strategy-*` skill survives in mochiko for any workflow** (strategy-core + strategy-specification already dissolved; this is the third and final). Any future workflow's "strategy" is `loop-discipline` (doctrine) + that workflow's lead (parameters), never a carried strategy skill.

---

## Decoupling-scan hits (logged for the run goal — 5th empirical test)
- `strategy-core` (sibling-skill name) — desc L3 + Overview L10.
- `strategy-specification` / `executing-tdd-cycle` / `testing-end-user` (sibling-skill names) — When-NOT-to-Use L22, L24, L25.
- `hil-dag` MCP (kernel/DAG) — L23.
- "targeted briefings" / "briefings for … passes" (brain-briefing framing) — desc L3, L10, L14.
- "execution node / verification node," "within the same pass," "gate verdict," "milestone" (DAG node/pass/gate/milestone vocab) — L18, L29, L34–35, L47.
- "heuristics / advisory / may deviate" (advisory meta-label) — Overview.
- **Outcome:** removed by **structural dedupe** — tokens vanish with the dropped skill; **no residual skill to scrub** (survivors land on the P1 command lead, where agent-naming/dispatch is legitimate). Zero-scrub-risk. Audited by `verify-output`'s decoupling grep on the P1 artifact (the lead is where the survivors materialize).

---

**Assessment version:** v1 · **Governed by:** `assess-primitive` (branch PLAYS-a-role) · **Disposition:** `drop × flag-for-reconcile`, no `loop-discipline` edit · **Next:** Phase-2 `reconcile-cluster` — resolve FR-A/FR-B/FR-C, MERGE FR-B into P1 FR-1, return zero open flags and a rehome map that lands the six survivors once on the thin lead. No artifact produced here (that is `transform-recipes`, post-reconcile).
