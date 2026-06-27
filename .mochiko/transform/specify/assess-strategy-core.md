# Assessment — `strategy-core` (P10)

**Run:** transform-cluster / `specify` · **Produced:** 2026-06-27 · **Producer:** `mochiko:transform-producer`
**Source:** `human-in-loop/plugins/humaninloop/skills/strategy-core/SKILL.md` (single file; no reference files)
**Sole HIL consumer:** `agents/state-analyst.md` (`skills: [strategy-core, strategy-specification, strategy-implementation]`) — the dissolving brain/DAG agent
**Primary overlap target:** `plugins/mochiko/skills/loop-discipline/SKILL.md` · **Sibling:** `strategy-specification` (P9)

---

```
ASSESSMENT: strategy-core
Class:        skill → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=n  → full-lens
Disposition:  drop (universal layer — standalone skill not carried; already in loop-discipline)
              [port-with-edits reserved ONLY for the gap-classification residual, if reconcile keeps it]
              × flag-for-reconcile
Reconcile flags: dedupe-into-loop-discipline (assign per TRACE rule 3) ·
                 gap-classification residual home ·
                 CROSS-WORKFLOW shared-substrate (plan/implement) ·
                 joint resolution with strategy-specification
```

---

## Step 1 — Class / branch

A **skill** → **PLAYS-a-role**. Weight goes to: consumed-procedure vs emits-artifact, sibling overlap (the heavy check here), and the decoupling scan (this run's empirical doctrine test).

## Step 2 — Triage gate

- **gate1 Orchestration-coupled? YES.** Not in its *pattern bodies* (those are mechanism-free prose), but in its **framing + consumer**: the skill exists to be "consumed by the State Analyst to inform Supervisor briefings." Its only consumer is the DAG-coupled `state-analyst`, which dissolves kernel-free.
- **gate2 Multi-responsibility / fans out? YES.** Four pattern-clusters + five anti-patterns; and it is the **universal/shared** strategy skill, fanned out across specify + plan + implement via the state-analyst.
- **gate3 Non-machine-checkable artifact? NO.** Emits no artifact — it is a consumed advisory procedure.

One+ yes → **full 7-check lens.**

## Step 3 — The lens (PLAYS-a-role weighting)

**1 · Orchestration test (content vs orchestration split).**
- *Content-coupling: LOW.* The pattern bodies carry no catalog/MCP/DAG concepts — they are universal loop principles. The only content coupling is **framing**: the intro names the State Analyst + Supervisor briefing chain.
- *Orchestration-coupling: consumer-dissolves, not driven.* Nothing sequences strategy-core; it is simply *read*. But its sole consumer (`state-analyst`) dissolves. Who inherits the consumption? In mochiko the command supervisor (lead) owns the loop directly and already consumes **`loop-discipline`** — so the patterns strategy-core carried are inherited *not as new content* but as content the lead's doctrine skill already holds. This is the crux that makes the fate a dedupe, not a rehome.

**2 · Role at two altitudes.**
- *Skill-role:* consumed-procedure (advisory patterns), **not** an artifact-emitter → no producer↔validator partner needed.
- *Conferred team-role:* it informs the **lead/referee** reasoning (how to run a sound loop) — precisely `loop-discipline`'s territory.

**3 · Independence.** No self-grade leak — strategy-core grades nothing. (Notably, its *content* about independence — "every output passes a gate," "confidence is not evidence" — is itself dedupe-into-loop-discipline material, see check 5.)

**4 · Verdict-sink / loop-driver.** No verdict emitted. The loop-driving *guidance* it encodes (pass evolution = no-progress detection; halt escalation = escalate-don't-die) was advisory input the state-analyst fed the Supervisor. In mochiko, loop-driving is owned by the supervisor's bounded loop (`loop-discipline` Req 3) → that guidance **dedupes** into loop-discipline.

**5 · Sibling / overlap — THE heavy check. Precise map strategy-core → loop-discipline:**

| strategy-core responsibility | loop-discipline coverage | Verdict |
|---|---|---|
| **Validation** — every output passes a gate before downstream use; confidence is not evidence; skipping validation is the #1 source of wasted iterations | **Req 2** (external independent validation) + Rationalizations row "Producer self-check is faster" + Red Flag "the producer can just check its own work" | **dedupe** |
| **Pass Evolution** — early-vs-late passes; by pass 3+ converging vs recurring; diminishing returns → surface | **Req 3.2** no-progress exit + **3.4** escalate; "converging vs recurring" = no-progress detection | **dedupe** |
| **Halt Escalation** — never force-exit without consent; present options; user has final say | **Req 4** human gate + **Req 3.4** "escalate, don't silently die … never report done because you ran out of rounds" | **dedupe** |
| anti-pattern **Infinite looping** | Req 3 no-progress + Red Flags | **dedupe** |
| anti-pattern **Validation bypass** ("confidence is not evidence") | Req 2 + Red Flags / "absence of proof reads as not done" | **dedupe** |
| anti-pattern **Force-termination** | Req 4 | **dedupe** |
| anti-pattern **Re-enrichment** (post-pass-1 enrichment) | NOT universal — spec-flavored; appears **verbatim** in `strategy-specification` ("Post-pass-1 enrichment") | **dedupe → strategy-specification** (mis-filed in "core") |
| **Gap Classification** — knowledge/preference/scope **taxonomy + routing** (knowledge→research, preference→user, scope→halt/split) | **NOT in loop-discipline.** loop-discipline names human-gate *placements* ("preference-gap only", "low validator-confidence only") but carries no gap taxonomy or FAIL-routing. **Genuinely additive.** | **SURVIVOR → flag-for-reconcile** |
| anti-pattern **Preference-to-research** | corollary of Gap Classification | **travels with the survivor → flag-for-reconcile** |

- *Cross-sibling note:* the survivor overlaps **strategy-specification**, which carries the same gap-routing as "Research Before User" + the "Uniform gap treatment" anti-pattern. The two skills' residuals must be reconciled **jointly** to avoid double-homing one gap taxonomy.
- *Cross-workflow note:* strategy-core is shared substrate consumed across specify + plan + implement (alongside `strategy-implementation`). Its dedupe/drop has cluster-global consequences.

**6 · Coupling audit.**
- Hardcoded paths: **none** (no `.humaninloop/` paths to rebind).
- Prerequisites/handoffs: **none** — read advisory, assumes no prior artifact.
- Determinism boundary: **all model-judgment**, no deterministic script → confirms no validator partner is real (degenerate), consistent with gate3=n.

**7 · Conventions + loop placement + DECOUPLING SCAN.**
- Classification / router / triggers / pairing: largely **moot** — the bulk dedupes into `loop-discipline` (already a classified, routed, model-invoked doctrine skill). Relevant only to the residual, which rides its host's wiring.
- *Loop placement:* strategy-core only **describes** done-condition/validation/human-gate advisorily; loop-discipline encodes them as **requirements** — which is exactly why it supersedes, not co-exists.
- **Doctrine-hardening (record):** HIL frames these as *"advisory patterns… not prescriptive rules… the Supervisor… may deviate."* `loop-discipline` says the opposite: *"Violating the letter of these rules is violating the spirit… You cannot rationalize your way out of any of the four."* The **"may-deviate" stance is itself dropped** (superseded), not carried — a dedupe into a *stricter* successor.

### DECOUPLING-SCAN hits (RUN GOAL — empirical doctrine test)

strategy-core is a **textbook carrier** of deny-list tokens:

| Token | Where | Deny-list class |
|---|---|---|
| **"State Analyst"** | description (L3) + body intro (L8) | **sibling-agent name** |
| **"Supervisor" / "Supervisor briefings"** | description (L3) + body L8 ("The Supervisor uses judgment…") | consumer-role / caller-side context (state by *role*, push to `agent-dispatch`) |
| **"Universal workflow patterns" / "advisory… not prescriptive"** | name + L6, L8 | **"workflow-agnostic" / independence-by-declaration meta-label** |

**Doctrine-test result (positive):** because the bulk **dedupes** (the standalone skill is dropped), these tokens **vanish structurally with the skill** — decoupling by *absence*, not by edit. The only leak risk is the gap-classification residual: whatever host absorbs it MUST be scrubbed of "State Analyst" / "Supervisor" / "universal" by the wiring pass, and that scrub must survive `verify-output`'s decoupling grep + keystone test. This is a clean data point that HIL's brain-era strategy skills carry deny-list tokens by construction, and mochiko removes them by structural dedupe rather than cosmetic editing.

## Step 4 — Disposition

**body `drop`** (universal layer — the standalone skill is not carried into mochiko; its content already lives in `loop-discipline`)
— with **`port-with-edits` reserved** for the gap-classification residual slice *only if* reconcile elects to keep it as standalone/folded content (scrub decoupling tokens + harden advisory→requirement).
**× structural `flag-for-reconcile`.**

Rationale: `dedupe` is a **relational** verdict that `reconcile-cluster` assigns, never solo assessment (TRACE-TAGS rule 3). The residual's home additionally depends on strategy-specification's assessment and the cross-workflow (plan/implement) picture. The body proposal stands; the structural verdict is reconcile's.

## Step 5 — Responsibility trace (dedupe ÷ survive ÷ drop)

```
TRACE: strategy-core
  R1  Validation pattern (gate every output; confidence≠evidence)        → dedupe (loop-discipline Req 2)
  R2  Pass Evolution pattern (converging vs recurring; surface)          → dedupe (loop-discipline Req 3.2/3.4)
  R3  Halt Escalation pattern (no force-exit; present options; user says) → dedupe (loop-discipline Req 4 + 3.4)
  R4  anti-pattern: Infinite looping                                      → dedupe (loop-discipline Req 3)
  R5  anti-pattern: Validation bypass                                     → dedupe (loop-discipline Req 2)
  R6  anti-pattern: Force-termination                                     → dedupe (loop-discipline Req 4)
  R7  anti-pattern: Re-enrichment (post-pass-1)                           → dedupe (→ strategy-specification; spec-specific, mis-filed)
  R8  Gap Classification taxonomy + FAIL-routing                          → flag-for-reconcile  [SURVIVOR — additive vs loop-discipline]
  R9  anti-pattern: Preference-to-research (gap-routing corollary)        → flag-for-reconcile  [travels with R8]
  R10 "advisory, not prescriptive / may deviate" stance                  → dropped + reason: superseded by loop-discipline's
                                                                            non-negotiable framing; carrying it would contradict doctrine
  R11 "consumed by State Analyst → inform Supervisor briefings" role      → dropped + reason: state-analyst dissolves; brief-the-Supervisor
                                                                            consumption model is kernel-era; lead consumes loop-discipline directly
```

**dedupe-vs-survive split (the headline):**
- **DEDUPE into `loop-discipline` (universal sound-loop layer): R1, R2, R3, R4, R5, R6** — 6 of the substantive items map one-to-one onto loop-discipline's four requirements.
- **DEDUPE into `strategy-specification` (mis-filed spec pattern): R7.**
- **SURVIVE (additive, → reconcile): R8 + R9** — the gap-classification taxonomy/routing is the *only* content not already in loop-discipline.
- **DROPPED (superseded / kernel-era framing): R10, R11.**

**Verdict: NEAR-TOTAL DEDUPE.** Of 9 substantive patterns, 6 dedupe straight into `loop-discipline`, 1 into the spec sibling, 2 framing responsibilities are dropped; only the gap-classification slice genuinely survives — and even it overlaps strategy-specification and is cross-workflow. This **confirms the cluster thesis**: `loop-discipline` absorbed strategy-core's universal layer.

## Reconcile flags (for `reconcile-cluster`)

1. **Assign the dedupe (TRACE rule 3).** Confirm R1–R6 dedupe into `loop-discipline`; the human gate accepts the **drop of the standalone strategy-core skill** (contract §4 explicitly names "strategy-core / strategy-specification dedupe vs loop-discipline" as a Phase-2 gated disposition).
2. **Home the gap-classification residual (R8+R9).** Three candidates: (a) **dedupe/merge with strategy-specification** (which carries the same routing); (b) **fold into loop-discipline** as human-gate-routing guidance (knowledge→research, preference→human gate, scope→halt/split maps onto Req-4 placement); (c) **moved-to-lead** (specify supervisor's FAIL-handling). Decide with strategy-specification's assessment in hand.
3. **CROSS-WORKFLOW shared-substrate.** strategy-core feeds specify + plan + implement. The dedupe/drop must be done **once, cluster-globally** — plan/implement ports must not expect a strategy-core skill; their strategy patterns also dedupe into loop-discipline. If R8 survives, host it in a **shared** location (loop-discipline or a lead pattern), not specify-only. (`moved-to-other-cluster`-adjacent.)
4. **Resolve jointly with strategy-specification (P9).** Co-consumed siblings with overlapping gap-routing + validation patterns — reconcile their residuals together to avoid double-homing one taxonomy.

## Decoupling-scan hits (logged for the run goal)
- `State Analyst` (sibling-agent name) — desc L3 + body L8.
- `Supervisor` / `Supervisor briefings` (consumer-role / caller-side) — desc L3 + body L8.
- `Universal workflow patterns` / `advisory… not prescriptive` ("workflow-agnostic"/declaration meta-label) — name + L6/L8.
- **Outcome:** removed by structural dedupe (tokens vanish with the dropped skill); residual host must be scrubbed + keystone-tested by the wiring pass and audited by `verify-output`.
