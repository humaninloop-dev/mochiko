ASSESSMENT: strategy-specification  (P9)
Source: human-in-loop/plugins/humaninloop/skills/strategy-specification/SKILL.md (single file, no reference bundle)
Class:        skill → branch PLAYS-a-role
Triage:       gate1=YES (orchestration-coupled: consumed by the dissolving state-analyst; body carries DAG/catalog vocab)
              gate2=YES (multi-responsibility: input-assessment + produce-then-validate + gap-informed revision + research-before-user + guardrails)
              gate3=NO  (emits NO artifact — it is a consumed advisory-procedure, not an artifact-emitter)
              → FULL LENS (≥1 yes)

Disposition:  redesign × flag-for-reconcile
  - BODY (proposed): redesign. The body is built around a mechanism that mochiko sheds (state-analyst
    consumes these patterns → briefs the Supervisor → drives the DAG loop). Most patterns DEDUPE into
    loop-discipline; a thin spec-specific slice SURVIVES and must be re-expressed kernel-free (no
    "State Analyst / Supervisor briefing", no "informed-by edges / passes / INV-xxx"). This is not
    keep-verbatim (deny-list tokens + DAG vocab) and not port-with-edits (the organizing frame — the
    briefing mechanism — is gone, and the bulk dedupes). NOT a wholesale `drop`: the spec-specific
    responsibilities SHOULD exist in mochiko, just rehomed.
  - STRUCTURAL: flag-for-reconcile. Where the survivors land (absorb-into-lead vs merge with
    strategy-core into one residual specify-strategy skill vs fold input-assessment into
    analysis-iterative) cannot be decided from this primitive alone. See Reconcile flags.

================================================================================
DEDUPE-vs-SURVIVE SPLIT  (the assigned (a)/(b) analysis)  + rehome targets
================================================================================

(a) DEDUPE — generic loop patterns the lead ALREADY carries via loop-discipline:
  - Produce-then-Validate ("a different agent validates"; "self-review is unreliable";
    "adversarial framing catches what the producer cannot see")     → loop-discipline Req 2 (external independent validation)
  - Guardrail "Never skip the advocate gate — every output validated" → loop-discipline Req 2
  - Anti-pattern "Recurring-gap iteration" (stop when same gaps recur) → loop-discipline Req 3 (no-progress exit)
  - Guardrail "After 5 passes, surface to user with options" (INV-004) → loop-discipline Req 3 (round cap) + Req 4 (human gate); "5" is just a parameter the lead sets
  - Generic "iterate on FAIL" half of Gap-Informed Revision            → loop-discipline loop mechanics
  Rehome target: NONE — these are already on the lead through loop-discipline. Do not re-port.

(b) SURVIVES — genuinely spec-specific strategy with NO loop-discipline coverage:
  - INPUT ASSESSMENT — sparse (missing Who/Problem/Value) → enrich; rich/clear-framing → straight
    to analyst; domain-context (constitution, codebase-analysis) can substitute for enrichment.
    THE genuine nugget. loop-discipline says nothing about triaging the *input* to a spec pass.
    Rehome → lead's spec-loop entry decision OR fold into analysis-iterative (enrichment skill).  [reconcile picks]
  - TARGETED-REVISION TACTIC — within gap-informed revision: carry the SPECIFIC flagged gaps
    forward and focus revision there, do NOT rewrite clean sections. (Beyond generic iterate-on-FAIL.)
    Rehome → lead's spec-loop revision step (the FAIL→re-produce handoff content).
  - SPEC DONE-CONDITION CONTENT — Goal: validated spec, advocate verdict `ready`; Success Criteria:
    spec.md with user stories + functional requirements. This is the specify workflow's
    pre-declared done-condition *content* (loop-discipline mandates a done-condition; this fills it in).
    Rehome → lead's contract (done-condition for the specify run).
  - CONSTITUTION-PREREQUISITE (INV-002: constitution must exist before spec work) — a real
    cross-cluster handoff edge to setup, not a loop rule.
    Rehome → explicit handoff edge in the contract (kept-but-rebind: drop the INV-002 label).
  - Anti-pattern "Post-pass-1 enrichment" — rides with Input Assessment (when enrichment is wasteful).
    Rehome → with Input Assessment.

(c) OVERLAPS strategy-core (3-way, NOT loop-discipline) — reconcile decides:
  - "Research Before User" + anti-patterns "Uniform gap treatment" / "User-for-researchable"
    = strategy-core's Gap Classification (knowledge→research / preference→user / scope→split).
    A spec-flavored restatement of the sibling. → dedupe-or-merge between the two strategy skills.

================================================================================
7-CHECK LENS (weighted PLAYS-a-role)
================================================================================
1. Orchestration test:
   - Orchestration-coupling: works ONLY because the state-analyst consumes it to brief the Supervisor,
     who drives the DAG loop (specify-catalog.json). The skill orchestrates nothing itself — it is
     advisory input to an orchestration layer that dissolves. Per cluster thesis the state-analyst
     DISSOLVES and the lead owns the spec loop directly → the surviving strategy must re-home to the lead.
   - Content-coupling: body carries DAG/catalog concepts directly — "informed-by edges", "passes",
     "pass 1", "After 5 passes", INV-002, INV-004 → body work (redesign / drop the mechanism).
2. Role (two altitudes):
   - skill-role: consumed-procedure (advisory patterns). Emits NO reviewable artifact → needs NO validator partner.
   - team-role conferred: a strategist/loop-driver advisory for the spec loop — i.e. lead/referee
     loop-driving KNOWLEDGE. Confers neither producer nor validator.
3. Independence: introduces NO self-grade leak. It actually *encodes* the independence doctrine
   (produce-then-validate) — which is precisely the overlap with loop-discipline Req 2 (→ dedupe).
4. Verdict-sink / loop-driver: consumer = the state-analyst (→ briefs Supervisor). The FAIL-loop it
   informs (gap-informed revision, halt-after-5, recurring-gap stop) is the spec loop the Supervisor
   drove. This loop-driving KNOWLEDGE is the biggest thing the kernel/supervisor owned; on dissolution
   it re-homes to the lead (generic half → loop-discipline; spec half → lead's spec-loop content).
5. Sibling / overlap ("look sideways") — TWO overlaps, both → reconcile:
   - vs strategy-core: both are state-analyst "strategy" skills; strategy-specification is largely a
     spec-flavored restatement of strategy-core (Gap Classification, Pass Evolution, Validation, Halt)
     plus a thin spec slice. Strong merge/dedupe signal.
   - vs loop-discipline (mochiko): heavy dedupe — see split (a). loop-discipline already carries
     produce-then-validate, no-progress, round-cap, human-gate.
6. Coupling audit:
   - Artifact names: spec.md, advocate-report.md (handoff context).
   - Prerequisite/handoff: constitution-must-exist (INV-002) → real edge to setup cluster.
   - Catalog labels: INV-002 / INV-004 → drop labels, keep substance (prereq edge; 5-pass = cap param).
   - DAG vocab: "informed-by edges", "passes" → drop mechanism, keep targeted-revision responsibility.
   - Determinism boundary: ALL model judgment, no deterministic script → no degenerate-validator concern.
7. Conventions + loop placement:
   - Classification: agent-consumed (by state-analyst), not user/auto-triggered. Moot if absorbed;
     any surviving residual skill → model-invoked / agent-consumed.
   - Sound-loop placement: this is loop-driving knowledge. In mochiko the lead owns the loop with
     loop-discipline; the survivors supply the spec-specific CONTENT of that loop (done-condition,
     entry triage, revision targeting), while the loop MACHINERY (gate, cap, human gate) is
     loop-discipline's, already on the lead.
   - DECOUPLING SCAN → see below.

================================================================================
DECOUPLING-SCAN HITS  (RUN GOAL: decoupling-doctrine empirical test — RECORDED)
================================================================================
strategy-specification DOES explicitly name the state-analyst / DAG passes. Confirmed deny-list tokens:
  - description (frontmatter): "...consumed by the State Analyst alongside strategy-core for targeted
    Supervisor briefings."  → sibling-agent name (State Analyst) + supervisor + briefing mechanism.
  - body L8: "Consumed by the State Analyst alongside `strategy-core` to produce targeted Supervisor
    briefings."  → same (sibling-agent name + dispatch-adjacent framing).
  - "Use informed-by edges to carry gap context forward" (Gap-Informed Revision) → DAG-mechanism vocab.
  - "the next analyst pass" / "Pass Evolution" / "pass 1" / "After 5 passes" → injected workflow phases/modes.
  - INV-002 / INV-004 → catalog (workflow-machinery) labels.
These are largely MOOT under dissolution, but per the run goal they are recorded as the empirical
confirmation that the cluster thesis is correct (the skill is state-analyst/DAG-coupled). ANY survivor
that becomes a skill must be scrubbed: state independence by ROLE, drop edge/pass/INV vocabulary.

================================================================================
RESPONSIBILITY TRACE  (done-condition: every responsibility tagged — no silent loss)
================================================================================
  - Supply spec-pass INPUT-ASSESSMENT strategy (sparse vs rich; enrich-or-not; domain substitution)
        → moved-to-lead  [SURVIVES; reconcile may re-tag folded-into-skill → analysis-iterative]
  - Supply TARGETED-REVISION tactic (carry specific gaps forward; focus, don't rewrite clean sections)
        → moved-to-lead  [SURVIVES — spec-loop revision-step content]
  - Supply SPEC DONE-CONDITION content (advocate verdict `ready`; spec.md with stories + FRs)
        → moved-to-lead  [SURVIVES — becomes the specify contract's pre-declared done-condition]
  - Constitution-prerequisite guardrail (INV-002)
        → kept-but-rebind  [SURVIVES as wiring — handoff edge in contract; drop INV-002 label]
  - "Post-pass-1 enrichment" anti-pattern
        → moved-to-lead  [SURVIVES — rides with Input Assessment]
  - Produce-then-Validate / independent-validation doctrine
        → dedupe  [loop-discipline Req 2 — lead already carries it]
  - "Never skip the advocate gate" guardrail + validation-bypass framing
        → dedupe  [loop-discipline Req 2]
  - "Recurring-gap iteration" anti-pattern (stop when gaps recur)
        → dedupe  [loop-discipline Req 3 no-progress exit]
  - "After 5 passes surface to user" guardrail (INV-004)
        → dedupe  [loop-discipline Req 3 round cap + Req 4 human gate; "5" is a lead-set parameter]
  - Generic iterate-on-FAIL half of Gap-Informed Revision
        → dedupe  [loop-discipline loop mechanics]
  - "Research Before User" + gap-typing (Uniform-gap-treatment / User-for-researchable anti-patterns)
        → dedupe  [overlaps strategy-core Gap Classification — RECONCILE assigns dedupe vs moved-to-sibling-skill]
  - State-analyst-briefing framing + DAG vocab (informed-by edges, passes, Supervisor briefings)
        → dropped + reason  [kernel/DAG/state-analyst machinery mochiko sheds; the state-analyst CONSUMER
          dissolves per cluster thesis. Lead must accept at the human gate.]

================================================================================
RECONCILE FLAGS  (relational signals for reconcile-cluster — cannot be decided solo)
================================================================================
  1. THREE-WAY DEDUPE: strategy-specification × strategy-core × loop-discipline. Both HIL strategy
     skills dedupe heavily into mochiko's loop-discipline. Decide: does ANY residual specify-strategy
     skill survive, or do both strategy-core + strategy-specification absorb-into-lead entirely?
  2. REHOME THE SURVIVORS: input-assessment + targeted-revision + spec-done-condition +
     constitution-prereq → moved-to-lead (lead owns the spec loop per cluster thesis) vs
     folded-into-skill (input-assessment → analysis-iterative P5). Needs the P1 lead spec-loop +
     P5 analysis-iterative assessments to choose.
  3. STRATEGY-CORE RELATION: the gap-typing slice (Research-Before-User) overlaps strategy-core's Gap
     Classification — merge-or-dedupe between the two siblings (pairs with P10 strategy-core assessment).
  4. PAIRS WITH P4 state-analyst DISSOLUTION: this skill's entire reason to exist (feed the
     state-analyst's Supervisor briefings) dissolves with the state-analyst. Its surviving strategy
     content must rehome to the lead ALONGSIDE state-analyst's non-DAG briefing/convergence/parse
     responsibilities — same rehome target, resolve together.

Hypothesis (context.md / task): flag-for-reconcile — CONFIRMED. Fate depends on the loop-discipline
overlap (mostly dedupe) + strategy-core relation + where the lead's spec-loop content lives.
