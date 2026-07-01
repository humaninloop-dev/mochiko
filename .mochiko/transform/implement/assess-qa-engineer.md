# Assessment — P3 `qa-engineer` (implement cluster)

**Assessed:** 2026-07-01 · **Producer:** `mochiko:transform-producer` · **Skill:** `assess-primitive` (branch: agent)
**Source:** `human-in-loop/plugins/humaninloop/agents/qa-engineer.md` (93 ln) · **Declared `skills:`** `testing-end-user` (P5)
**Diagnose-only.** No edits made. Disposition + trace for reconcile/transform to consume.

---

```
ASSESSMENT: qa-engineer
Class:        agent → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone
              (+ 3 flag-for-reconcile — relational wiring only, not a restructure of this agent)
```

**Role in cluster:** the **independent validator** of the implement team — counterpart to `staff-engineer` (P2, producer). An *unusually strong* validator: it verifies against **real infrastructure with captured evidence + quality-gate exit codes = Tier-1 deterministic ground truth** (contract §2). Its evidence-first / human-oversight-as-final-gate / distrust-inferred-outcomes identity is the **spine of that Tier-1 validator — INTRINSIC craft, kept whole.**

---

## Triage (why full-lens)

1. **Orchestration-coupled? YES** — in HIL it is a DAG node dispatched by the implement supervisor, which sequences it after the producer (execute→verify), loops on its FAIL verdict, and consumes its report at the cycle-checkpoint gate.
2. **Multi-responsibility? YES** — confers the validator team-role and holds several quality dimensions (evidence, reproducibility, honesty, conservatism, completeness) + multiple output types.
3. **Non-machine-checkable artifact? YES (partially)** — Tier-1 sub-checks (exit codes, CLI asserts) are deterministic, but the escalation/classification *judgment* is model judgment.

At least one YES → full 7-check lens.

---

## Lens findings (weighted PLAYS-a-role / agent)

**1 — Orchestration test.** Orchestrator = the HIL **implement command** (DAG-Supervisor + State-Analyst). It sequences qa after the producer, drives the FAIL-loop, and consumes its verdict.
- *Content-coupling (fix in body):* mild + localized — the word **"cycle"** in the `description` (line 5) and example 3 (lines 26-27). No kernel/DAG/catalog/MCP references in the persona at all.
- *Orchestration-coupling (structural → lead):* the execute→verify sequencing, the FAIL-loop, and the input handoff are **driven by the dissolving supervisor** → re-home to the thin lead (P1). The persona must carry none of it. → flags F1, F3.

**2 — Role at two altitudes.** Skill-role: consumes a procedure (`testing-end-user`) **and** emits reviewable artifacts (verification report, checkpoint presentation). Team-role: **validator** (independent). This is the producer↔validator pairing's validator half. INTRINSIC — keep.

**3 — Independence. CLEAN — no self-grade leak.** qa-engineer `skills: {testing-end-user}` (verification/grading); staff-engineer `skills: {executing-tdd-cycle, brownfield-integration}` (production). Different agents, **disjoint** skills. This is already the mochiko-correct arrangement (matches contract §2). Preserve it — any resolution that mounts produce+grade on one agent is wrong by construction. → confirm at reconcile (F1), do not alter.

**4 — Verdict-sink / loop-driver.** Consumer of its output = the cycle-checkpoint gate → the supervisor's loop. What loops on FAIL = targeted retry + fix-pass + convergence-stall (contract §1 c/d/e). **None of that is qa's responsibility** — it emits the verdict; the loop is lead-owned (P1). → F3.

**5 — Sibling / overlap.** Sibling = staff-engineer (P2), but as a **complement (pair), not an overlap** — no shared core, nothing to merge. Skill P5 (`testing-end-user`) vs P4 (`executing-tdd-cycle`) are disjoint procedures. No trigger collision (agent-selection examples, not model-invoke triggers). The relation to preserve is the **pairing** → F1.

**6 — Coupling audit.**
- *Hardcoded paths:* **NONE in the persona.** Examples are clean user-request framings ("Implementation is done. Run the verification tasks."). Notably qa-engineer does **not** carry the `Read your instructions from: specs/…/context.md` examples that staff-engineer (P2) does — that deny-list item does not apply here.
- *Prerequisite/handoff:* assumes implementation complete + `**TEST:**` tasks and quality gates present in `tasks.md`. That handoff edge is workflow-specific → wired by the lead (P1), not the persona.
- *Determinism boundary (load-bearing):* deterministic = quality-gate exit codes (0/non-0) + CLI assert evaluation (substring / file-exists / HTTP status) — **this is the Tier-1 ground truth that makes qa an unusually strong validator.** Model-judgment = ambiguous classification + escalation + GUI/subjective (deferred to human). **Keep both sides; do not dilute the deterministic half into pure LLM judgment.**

**7 — Conventions + loop placement.**
- *Classification:* agent → `skills:` list, not user/model-invoked. `skills: testing-end-user` → rebind to ported P5.
- *Persona-vs-procedure split:* CLEAN. Persona holds *what it cares about* (evidence, conservatism, honesty); the *how* (parse TEST:, classify, capture, quality-gate mechanics, report templates) lives in P5.
- *Producer↔validator pairing:* qa is the validator half; the pairing must be **structurally guaranteed by the lead's casting** (different agents + disjoint skills), **not declared in the persona** (the persona must never name staff-engineer — and it does not). ✓
- *Loop placement:* qa **provides** the independent validation + the human checkpoint; the done-condition + FAIL-loop + gate *placement* are lead-owned (P1).

---

## Decoupling scan (agent deny-list, keystone-tested line-by-line)

**Deny-list HITS → `port-with-edits` decouple (the only body edits this agent earns):**

| Line | Exact token | Keystone verdict | Fix |
|------|-------------|------------------|-----|
| 5 (`description`) | "gates **cycle** completion on human approval" | "cycle" = the implement workflow's unit, not universal QA | → "gates completion on human approval" (gate the *change/verification*, not the named cycle) |
| 26 (`description`, ex-3 context) | "run as part of **cycle verification**" | workflow framing | → "as part of verification" |
| 27 (`description`, ex-3 user) | "before we **close this cycle**" | workflow framing | → generic, e.g. "before we approve this change" |

**Verified CLEAN (no deny-list token — contrast staff-engineer):**
- No sibling-agent name — `staff-engineer` never appears. ✓
- No "dispatch"/"dispatched" in the persona (the only "dispatched" is in P5 `testing-end-user` line 156 — out of scope for P3). ✓
- No "workflow-agnostic" / independence-by-declaration meta-label. ✓
- No injected workflow **modes/paths/phases** — there is **no** "Execution Modes" section (staff-engineer's highest-risk surface; qa has none). ✓
- No `context.md` / `specs/…` path reads in examples. ✓
- Self-reference in example commentary ("owned by the qa-engineer", "I'll use the qa-engineer to…", line 30 etc.) is the standard Claude Code agent-selection format (matches ported `mochiko:devils-advocate`) — **KEEP.**

**Intrinsic craft, keystone-PASSED ("true of this QA professional on any job") → KEEP verbatim:**
- Core Identity (lines 50-54): real-infra scars, evidence audit trail, **"verification without evidence is just opinion,"** trust through rigor.
- Quality Standards (65-69): evidence-first · reproducible · honest ("should-pass-but-didn't = failure, full stop") · complete · **conservative (uncertain → default to human oversight)**.
- Your Judgment (73): distrust inferred outcomes · **"ambiguity is never a reason to auto-approve — it's a reason to escalate"** · unexpected output → checkpoint, not silent approval. *(This is the persona's disposition to escalate; the classification **procedure** is P5.)*
- What You Reject (77-83) / What You Embrace (87-93): real-infra > mocks · evidence-based · **human oversight as the final quality gate — a feature not a formality** · escalating ambiguous evidence · rigor regardless of simplicity · minimal-for-clean/rich-for-attention reporting.
- "Deterministic pass/fail for lint, build, and test suites" (59): the Tier-1 determinism — **KEEP** (load-bearing for the validator tier).

---

## Responsibility trace (complete — no silent drops)

**Intrinsic validator craft → `kept` (the Tier-1 spine):**
- Evidence-first verification (no "passed" without captured proof) → **kept**
- Reproducibility (re-runnable; exact commands/env/timing) → **kept**
- Honesty (report observed not expected) → **kept**
- Completeness (all setup/actions/asserts; no partial-as-complete) → **kept**
- Distrust of inferred outcomes → **kept**
- **Conservative disposition** — uncertain/ambiguous → default to human oversight/checkpoint → **kept** *(the disposition; the classification procedure is P5, below)*
- Escalating ambiguous evidence to human judgment → **kept**
- Human-oversight-as-final-gate (the value) → **kept**
- Real-infrastructure-first (reject mocks when real infra available) → **kept**
- Deterministic quality-gate verdict discipline (exit-code ground truth) → **kept** *(the Tier-1 determinism boundary)*
- "What You Produce" outputs (verification reports, quality-gate results, checkpoint presentations, evidence artifacts) → **kept** *(persona names its outputs; the report **templates** live in P5)*
- Reporting judgment (minimal for clean pass, rich for attention) → **kept** *(the templates are P5)*

**Procedure (persona-vs-procedure boundary) → `folded-into-skill` (P5 `testing-end-user`, already resident — persona references, does not restate):**
- Parse/execute `**TEST:**` Setup/Action/Assert + evidence-capture mechanics → **folded-into-skill (P5)**
- **Task-classification procedure** (CLI→may-auto-approve if 100% pass · GUI→checkpoint · SUBJECTIVE→checkpoint · default-SUBJECTIVE) → **folded-into-skill (P5, lines 111-122)** *(the runtime classification reclaimed by P5 per context.md; persona keeps only the disposition, above)*
- Quality-gate execution mechanics (identify commands, run, exit-code classify, report block) → **folded-into-skill (P5, lines 154-200)**

**Wiring → `kept-but-rebind`:**
- `skills: testing-end-user` → **kept-but-rebind** (resolve to the ported mochiko P5)
- `model: opus`, `color: cyan` frontmatter → **kept**
- "gates cycle completion" / "cycle verification" / "close this cycle" wording → **kept-but-rebind** (decouple the "cycle" binding; the gating *responsibility* survives as intrinsic craft — see decouple table)

**Orchestration (not qa's to own) → `moved-to-lead` (P1) — flagged for reconcile, not decided here:**
- Execute→verify **pairing/sequencing** with the producer (runs after implementation, same round, never skipped) → **moved-to-lead** (contract §1b) → F1
- FAIL-**loop driving** (targeted retry, fix-pass, convergence-stall) → **moved-to-lead** (contract §1 c/d/e) → F3
- Auto-approve-vs-checkpoint **gate placement** + final-acceptance gate wiring (intake decision 2) → **moved-to-lead** (P1); persona feeds it the conservative judgment (kept) → F2

**Dropped:** none. No kernel/DAG/catalog plumbing in this persona to shed; "cycle" is decoupled (rebind), not dropped — the underlying gate/verify responsibility survives.

---

## Reconcile flags (relational — cross-primitive; `reconcile-cluster` owns these)

- **F1 — Preserve the producer↔validator pairing.** qa-engineer (validator) ↔ staff-engineer (P2, producer): independence already holds (different agents, disjoint skills — do **not** merge onto one agent). Reconcile confirms it and re-homes the **execute→verify wiring** to the thin lead (P1), declared by the lead's casting, not in either persona. (Ties contract §1b, §2.)
- **F2 — Confidence-based gate is a three-way split; confirm the seam.** classification **procedure** = P5 (already resident) · conservative **judgment/disposition** = qa persona (kept) · gate **placement + final-acceptance wiring** = lead (P1, intake decision 2). Reconcile confirms no piece is double-owned or dropped — this is where the auto-approve-vs-checkpoint *wiring* lands (persona vs skill vs lead).
- **F3 — FAIL-loop driving belongs to the lead.** targeted retry / fix-pass / convergence-stall (contract §1 c/d/e) → P1 lead; confirm the qa persona carries none of it (it emits the verdict only).

*None of F1–F3 changes this agent's structural move — it stays `standalone` (one self-contained validator persona). They are wiring that P1's redesign + reconcile own.*

---

## Handoff note

Disposition `port-with-edits × standalone` is decidable solo and stated. The **only** body edits earned: decouple the three "cycle" tokens (table above); everything else is intrinsic Tier-1-validator craft kept verbatim. The persona-vs-procedure boundary is already clean (procedure in P5). Independence is already correct — reconcile's job is to **confirm and wire**, not to restructure. An independent reader can verify every claim against the cited line numbers in the source.
