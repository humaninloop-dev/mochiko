# Assessment — `analysis-specifications` (P6)

**Run:** transform-cluster / `specify` · **Assessed:** 2026-06-27 · **Producer:** `mochiko:transform-producer`
**Source:** `human-in-loop/plugins/humaninloop/skills/analysis-specifications/SKILL.md` (single file; no reference bundle)
**Skill used:** `mochiko:assess-primitive` (branch: SKILL → PLAYS-a-role) · ASSESS ONLY — no edits made.

---

```
ASSESSMENT: analysis-specifications
Class:        skill → branch PLAYS-a-role
Triage:       gate1=n gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone   (+ flag-for-reconcile: confirm-pairing/independence)
Trace:
  - find spec gaps (missing reqs, ambiguities, edge cases, assumption gaps)   → kept
  - core principle: product-framed (WHAT) not implementation (HOW) questions  → kept
  - question format (decision + concrete options + "why this matters")        → kept
  - gap-category taxonomy (user-exp / business-rules / scope / states / perms)→ kept
  - "what to avoid" boundary (defer technical concerns to design)             → kept (soften "planning phase" wording)
  - severity classification (Critical/Important/Minor + action)               → kept  [INPUT to referee's verdict, not the verdict]
  - structured output (Gaps Found by severity)                                → kept  [do NOT add a clearing PASS/FAIL — referee owns the verdict]
  - review process (7 steps)                                                  → kept
  - reviewer self-check (Quality Checklist)                                   → kept
  - reviewer anti-rationalization (Common Mistakes)                           → kept
  - applicability bullet naming "Devil's Advocate" (line 19)                  → kept-but-rebind  [HARD decouple → state by role]
  - cross-skill pointer `humaninloop:authoring-requirements` (line 27)        → kept-but-rebind  [→ mochiko: namespace]
  - workflow-phase framing ("specification/planning phase", lines 15/18/74)   → kept-but-rebind  [restate by activity; keystone-survivor wording]
  - trigger phrasing "when the user says…" (line 3 description)               → kept-but-rebind  [agent-consumed → work-context phrasing]
  - classification tag (model-invoked)                                        → kept-but-rebind  [confirm model-invoked + agent-consumed]
Reconcile flags: confirm-pairing/independence ; verdict-ownership boundary ; trigger de-collision (P5) ; cross-cluster consumer (audit)
```

---

## Step 1 — Class / branch

**skill → PLAYS-a-role.** This is the adversarial critic's core procedure, not a loop. Weight (per the brief): consumed-procedure-that-emits-a-reviewable-artifact, the team-role it confers (independent **validator/critic**), trigger reliability, sibling overlap with the analyst's authoring skills (the producer side), and decoupling. Classification/loop-ownership checks are a category error here — they sit on the command (P1).

## Step 2 — Fast-path triage

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled (needs kernel / supervisor / command / DAG to **function**)? | **no** — the body is a pure review procedure; no DAG/catalog/MCP/brain content anywhere. Coupling is naming + triggers, fixable by edit, not a function dependency. |
| 2 | Multi-responsibility / fans out? | **yes** — primary consumer is the critic agent (`devils-advocate`); secondary consumers are the `audit` command (`commands/audit.md:118`) and sibling "use this instead" cross-pointers. |
| 3 | Emits an artifact whose correctness is NOT machine-checkable? | **yes** — a gap report / clarifying questions; "are these the right gaps, framed as product not technical?" is model judgment, not a schema/version assert. This is exactly why the critic role is real and non-degenerate. |

Two gates trip → **full 7-check lens.**

## Step 3 — Lens (weighted PLAYS-a-role)

1. **Orchestration test.** Body is mochiko-clean — **no content-coupling** (no DAG/catalog/MCP/brain/`.humaninloop`). The brain machinery (`specify-catalog.json` + `state-analyst` parsing + DAG loop) was always *external* to this skill. So there is nothing to `redesign` and **nothing to `drop`** in the body — the whole transformation is naming/trigger/path *decouple* (`port-with-edits`). Orchestration-coupling (who invokes it, who consumes its output, what loops on it) rehomes to the lead — but those are the lead's responsibilities (P1/P4), not this skill's.
2. **Role (two altitudes).** Skill-role: **consumed-procedure that emits a reviewable artifact** (the gap report). Team-role conferred on its caller: **independent validator / adversarial critic.** Synthesis `[[adversarial-pair-convergence-loop]]` names this the *essence of specify* — "an adversarial critic finds gaps (**never blesses**)." The "never bless" stance lives in the agent persona (P3); the gap-finding *procedure* is this skill. Clean persona-vs-procedure split.
3. **Independence (cluster-scoped).** No self-grade leak at the skill: in HIL the producer (`requirements-analyst` + `authoring-requirements`/`authoring-user-stories`) and the critic (`devils-advocate` + this skill) are **already separate agents**. This is the sharp contrast with the setup precedent — `validation-constitution` needed `promote`/`split` because `principal-architect` self-graded (mounted author + grader). **No equivalent leak here** → no promote, no split; the validator partner already exists. Reconcile must only *confirm* the agent-level independence holds on port (no cross-mount). → flag-for-reconcile.
4. **Verdict-sink / loop-driver.** The gap report (severity-bucketed) feeds the **lead/referee**, who owns the clearing verdict and drives produce→critique→revise under a round cap + human escalation. Per the synthesis invariant **"the referee owns the verdict"** (so the pair can't collude into agreeableness), the *clearing* verdict must NOT be folded into this skill. The skill's severity buckets are the **input** to that verdict, not the verdict. Loop machinery rehomes to the lead (traced on P1; the contract already specifies the cap + gate). → reconcile note (verdict-ownership boundary).
5. **Sibling / overlap ("look sideways").** No merge signal. `analysis-iterative` (P5) is *input enrichment* (Who/Problem/Value, producer-side, pre-authoring); this is *output gap-review* (critic-side, post-draft) — different role, side, and stage. The HIL "When NOT to Use → spec review → use `analysis-specifications` instead" cross-pointers (in `analysis-iterative`, and in the deferred `validation-plan-artifacts`/`validation-task-artifacts`) are **trigger de-collision markers**, not merge signals → a convention-wiring fix (transform-recipes), not structural. The authoring skills (P7/P8) are the *producer* side of the same pair — pairing relationship, not overlap. → reconcile flags (de-collision + pairing).
6. **Coupling audit.** Path rebind: `humaninloop:authoring-requirements` → `mochiko:` (line 27). Prerequisite edge: assumes a drafted spec exists (produced upstream by the analyst) — an explicit handoff edge, not a kernel dependency. Determinism boundary: **fully model-judgment** (no deterministic sub-check) → the critic partner is genuinely needed, never a degenerate assert.
7. **Conventions + loop placement.**
   - *Classification:* model-invoked today ("MUST be invoked when the user says…"); keep model-invoked but it is **agent-consumed** → describe work-context, not "when the user says." (wiring)
   - *Discoverability:* register in the `mochiko` router as the critic's gap-review procedure (hinted). (wiring add)
   - *Reliable invocation:* graded MUST/SHOULD phrases already present ("review spec", "find gaps", "what's missing", "clarify requirements") — good base; reframe away from user-says to work-context. (port-with-edits)
   - *Agent↔skill composition + DECOUPLING:* persona-vs-procedure split is clean. **Decoupling scan → one HARD hit** (agent name, line 19) + soft phase-vocabulary; see below.
   - *Producer↔validator pairing:* structurally guaranteed (critic agent ≠ producer agent) — already satisfied; confirm in reconcile.
   - *Sound-loop:* done-condition / independent-validation / human-gate for the produce→critique→revise loop are the **lead's** (rehome P1/P4); this skill supplies the independent-critique input, not the loop.

## Step 4 — Decoupling scan (RUN GOAL — empirical doctrine result)

Deny-list grepped (sibling-agent **names** · `dispatch` · `workflow-agnostic`/independence-by-declaration meta-labels · injected workflow modes/paths/**phases**), then keystone-tested.

| Line | Hit | Class | Fix |
|------|-----|-------|-----|
| 19 | `When Devil's Advocate reviews specification artifacts` | **HARD — sibling-agent name** | state by **role** ("during independent/adversarial review of a drafted spec") or drop the bullet (other When-to-Use bullets already carry the work-context). → `port-with-edits` decouple. |
| 15 | `…completeness after specification phase` | SOFT — phase vocab | keystone **survives** (validating completeness is craft); soften → "after a spec is drafted." |
| 18 | `Quality gate before planning phase begins` | SOFT — phase vocab | keystone **survives** (review before downstream work is craft); soften → "before downstream planning/design." |
| 74 | `…belong in the planning phase, not specification` | SOFT — phase vocab | keystone **survives** (deferring technical concerns to design is intrinsic to spec review); soften "planning phase" → "design/implementation work." |
| 27 | `humaninloop:authoring-requirements` | REBIND (allowed cross-pointer) | `→ mochiko:` namespace. Not a deny-list violation. |
| 3 | `…when the user says "review spec"…` | TRIGGER convention | work-context phrasing for an agent-consumed skill. Not a deny-list violation. |

**Clean of:** `dispatch`, `workflow-agnostic`/independence-by-declaration meta-labels, injected modes/paths, DAG/catalog/brain/kernel/`.humaninloop`, `state-analyst`.

**Empirical finding for the doctrine:** the validator-side skill states most of its independence by **role** already (the procedure is role-neutral); it carries exactly **one HARD deny-list token** — the canonical case the doctrine targets (an agent name embedded in a skill), and it is grep-catchable. The three phase-vocabulary lines are precisely the keystone-test discriminator cases: the underlying point **survives** (true of a spec reviewer on any job) while the "phase" *wording* is workflow-coupled and should soften. This confirms the grep-then-keystone design both catches the canonical violation and correctly discriminates the borderline ones.

## Step 5 — Disposition

**`port-with-edits × standalone`** — matches the hypothesis.

- **Body = `port-with-edits`** (NOT `redesign`): the body is mochiko-clean with **no kernel content to rewrite**; the edits are localized — (1) decouple the agent name (line 19), (2) soften phase vocabulary (15/18/74), (3) rebind the namespace (27), (4) reframe triggers to work-context (3). Minimalism governor: an edit fixes it; redesign is unearned.
- **Structural = `standalone`**: self-contained, one home — the critic agent's load-bearing procedure. No `split` (already one cohesive procedure), no `merge` (no shared core with siblings), no `promote` (the independence a promote would create **already exists** — critic agent ≠ producer agent).
- **Do NOT add a clearing verdict.** The "referee owns the verdict" invariant means the skill stays a gap-finder; its severity buckets feed the lead's verdict. Read "structured verdict output" in the brief as the **structured gap report** (severity + clarifying questions), not a binary PASS/FAIL (the validation-constitution shape does **not** transfer to a critic-side skill).

## Reconcile flags (cluster-scoped — for `reconcile-cluster`)

1. **confirm-pairing / independence** — this is the **critic side** of the spec producer↔critic pair. Confirm on port: this skill lands on the critic agent (`devils-advocate` port), the authoring skills (P7/P8) land on the producer agent (`requirements-analyst` port), with **zero cross-mounting** (no agent mounts both an `authoring-*` and `analysis-specifications`). Pair already exists in HIL → **confirm, not create** (contrast setup's promote).
2. **verdict-ownership boundary** — confirm the **clearing** verdict is owned by the **lead/referee** (synthesis invariant), not folded into this skill and not double-owned by the critic agent's persona (P3). Guards against accidentally upgrading the skill into a binary validator.
3. **trigger de-collision** — `analysis-iterative` (P5, enrichment) vs this (gap-review) must keep non-overlapping triggers (transform-recipes wiring). Deferred `validation-plan-artifacts`/`validation-task-artifacts` carry the same cross-pointer → re-check when plan/tasks port (out of scope now).
4. **cross-cluster consumer** — `commands/audit.md` also applies "`analysis-specifications` skill criteria." Skill **stays in specify** (no move); audit's reference rebinds to the mochiko-ported skill if/when audit ports.

## Notes for the lead

- **No `dropped` responsibilities** — nothing in this skill's body is kernel/DAG/catalog plumbing; the body was always clean, which is why `port-with-edits` (not `redesign`) holds. No drop needs the human gate's acceptance for this primitive.
- Responsibilities that are **not** this skill's (so verify-output can confirm the boundary, not flag silent loss): the **clearing verdict** → lead/referee (P1); the **"never bless" adversarial calibration** → critic persona (P3); the **produce→critique→revise loop + round cap + human escalation** → lead (P1/P4) + contract. This skill never held these.
- REGISTRY: currently `[ ]` ("Feeds specify — gap analysis"). Flip on transform; the trace above doubles as the migration record.
