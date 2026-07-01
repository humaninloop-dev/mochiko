# Phase 3 Transform — P3 `qa-engineer` (implement cluster)

**Transformed:** 2026-07-01 · **Producer:** `mochiko:transform-producer` · **Skill:** `transform-recipes` (recipe: `port-with-edits` + `standalone` + wiring pass)
**Consumes:** `assess-qa-engineer.md` (trace) + `reconcile.md` (Seam 2, Seam 4, §Job 2 F1/F3, §Job 3 P3 row)
**Source:** `human-in-loop/plugins/humaninloop/agents/qa-engineer.md` (93 ln)
**Artifact:** `plugins/mochiko/agents/qa-engineer.md`
**Applies a decision; does not make one. Does NOT grade its own output** — that is Phase-4 `verify-output`, run by a different agent.

---

```
TRANSFORM: qa-engineer
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/agents/qa-engineer.md (created)
New partners: NONE — the producer↔validator pair already exists (staff-engineer ↔ qa-engineer,
              disjoint skills, different agents); reconcile confirmed "cast, not created" (§2.0).
              qa stays one self-contained validator persona (standalone).
Wiring:    classification=agent (skills: list, not user/model-invoked)
           router=DEFERRED to Wave-2 (per task scope)
           triggers=n/a (agent-selection <example> blocks, not model-invoke triggers)
           rebinds=[ skills: testing-end-user → mochiko:testing-end-user (P5, ported) ]
           decouple=3 "cycle" tokens excised (desc L5, ex-3 L26/L27); persona already clean of
                    sibling-agent name / "dispatch" / modes-paths-phases / "workflow-agnostic"
           single-source=procedure referenced in P5, not restated; no loop-discipline inlined
```

---

## Body treatment applied — `port-with-edits` (the 3 earned "cycle" decouples ONLY)

The only body edits this agent earned (assess handoff note + reconcile §Job 3 P3 "cycle" row + Seam 4). Each decouples the implement workflow's unit-of-work token from otherwise-universal QA language; the underlying gate/verify responsibility survives as intrinsic craft (rebind, not drop).

| Loc (HIL) | Before | After | Keystone rationale |
|-----------|--------|-------|--------------------|
| `description` L5 | "gates **cycle** completion on human approval" | "gates **completion** on human approval" | "cycle" = this workflow's unit, not universal QA. Gate the *change/verification*, not the named cycle. |
| `description` ex-3 context L26 | "run as part of **cycle verification**" | "as part of **verification**" | workflow framing → generic verification-work context. |
| `description` ex-3 user L27 | "before we **close this cycle**" | "before we **approve this change**" | workflow framing → generic user request. |

**Post-edit deny-list scan (keystone, whole artifact):** `cycle` = 0 · `pass` (workflow-iteration sense) = 0 · `dispatch` = 0 · `staff-engineer` = 0 · `workflow-agnostic` = 0.
- The three surviving `pass` tokens ("Deterministic pass/fail", "a test that 'should' pass") are **universal test-pass/fail language**, not workflow-iteration "pass" — keystone-PASS, kept.
- The self-references in example commentary ("owned by the qa-engineer", "I'll use the qa-engineer to…") are the **standard Claude Code agent-selection format** (matches ported `mochiko:devils-advocate` / `task-architect`) — KEPT per task instruction.

**Everything else kept verbatim** (Core Identity, What You Produce, Quality Standards, Your Judgment, What You Reject, What You Embrace) — this is the intrinsic Tier-1-validator spine. Structure, headings, and voice preserved.

## Structural move applied — `standalone`

One self-contained validator persona placed at `plugins/mochiko/agents/qa-engineer.md`. No split, no merge, no promote — the independent producer↔validator pair already exists structurally (§2.0 inverse-of-setup). F1/F2/F3 are wiring the **lead** owns; none changes this agent's placement.

---

## Load-bearing invariants — CONFIRMED INTACT in the artifact

**Tier-1 determinism boundary (do NOT dilute) — KEPT.**
- "What You Produce" #2: "**Quality Gate Results** — Deterministic pass/fail for lint, build, and test suites" — the exit-code ground truth, verbatim.
- The skill reference names "running the quality gates and classifying results **by exit code**" — the CLI-assert / exit-code discipline lives in P5 (`testing-end-user`), referenced not restated. Not softened into pure LLM judgment.
- "Evidence-first" / "Reproducible" quality standards (captured proof, exact commands) — the deterministic-evidence spine — verbatim.

**Tier-1 validator spine (all intrinsic craft) — KEPT verbatim.**
- evidence-first verification (QS: Evidence-first) · reproducibility (QS: Reproducible) · honesty (QS: Honest, "should-pass-but-didn't = failure, full stop") · completeness (QS: Complete) · distrust-inferred-outcomes (Your Judgment; Reject: "Inferred outcomes…") · escalate-ambiguous-evidence (Embrace) · human-oversight-as-final-gate (Embrace: "not a formality, a feature") · real-infra-over-mocks (Reject/Embrace) · "verification without evidence is just opinion" (Core Identity).

## Confidence-gate seam (Seam 4) — the split as realized in this persona

The persona keeps **only the conservative judgment/disposition**; the classification *procedure* and the gate *placement* are NOT here.

| Piece | Home | Realized in this artifact |
|-------|------|---------------------------|
| Conservative **judgment/disposition** — GUI/subjective always → human; ambiguous evidence → escalate; uncertain → default to human oversight | **P3 persona (KEPT)** | "Conservative" quality standard; "Your Judgment" (ambiguity → escalate, unexpected output → checkpoint); Reject ("Auto-approving anything that requires human judgment—GUI interactions and subjective assessments always get a checkpoint"); Embrace ("Escalating ambiguous evidence…"). |
| Classification **procedure** — the CLI/GUI/SUBJECTIVE table, 100%-pass ⇒ auto-approve, default-SUBJECTIVE | **P5 `testing-end-user`** (folded-into-skill) | NOT in the persona. The Skills-Available line references the skill as the single source of the classify procedure; the persona references, does not restate. |
| Gate **placement + final-acceptance wiring** | **the lead (P1)** | NOT in the persona. The persona feeds the lead its conservative judgment; the auto-approve-vs-checkpoint routing and the named final-acceptance gate are lead-owned. |

**Confirmed:** no classification table and no gate-placement prose in the persona. The disposition lives in exactly one place (P3), the procedure in exactly one (P5), the placement in exactly one (lead) — no piece leaks across.

## Mount / independence

- Frontmatter `skills: testing-end-user` (bare, matching ported-agent convention); body Skills-Available reference rebound to **`mochiko:testing-end-user`** (P5, ported). Tag: `kept-but-rebind`.
- **HARD INDEPENDENCE RULE honored:** producer skills `executing-tdd-cycle` / `brownfield-integration` are **NEVER** mounted here — mounting produce+grade on one agent is wrong by construction (Seam 2). qa mounts the grading skill **only**; staff mounts the producer skills **only**; disjoint.
- `model: opus`, `color: cyan` frontmatter — kept.

---

## Responsibility trace (realized — every responsibility from the assessment carries a final tag; no silent loss)

**Intrinsic validator craft → `kept` (the Tier-1 spine, verbatim in the artifact):**
- Evidence-first verification (no "passed" without captured proof) → **kept** (QS: Evidence-first)
- Reproducibility (re-runnable; exact commands/env/timing) → **kept** (QS: Reproducible)
- Honesty (report observed not expected) → **kept** (QS: Honest)
- Completeness (all setup/actions/asserts; no partial-as-complete) → **kept** (QS: Complete; Reject)
- Distrust of inferred outcomes → **kept** (Your Judgment; Reject)
- Conservative disposition — uncertain/ambiguous → default to human oversight/checkpoint → **kept** (QS: Conservative; Your Judgment) *(disposition only; classification procedure is P5)*
- Escalating ambiguous evidence to human judgment → **kept** (Embrace)
- Human-oversight-as-final-gate (the value) → **kept** (Embrace: "not a formality, a feature")
- Real-infrastructure-first (reject mocks when real infra available) → **kept** (Reject; Embrace)
- Deterministic quality-gate verdict discipline (exit-code ground truth) → **kept** (What You Produce #2) *(the Tier-1 determinism boundary, un-diluted)*
- "What You Produce" outputs (verification reports, quality-gate results, checkpoint presentations, evidence artifacts) → **kept** *(persona names its outputs; report templates live in P5)*
- Reporting judgment (minimal for clean pass, rich for attention) → **kept** (Embrace) *(the templates are P5)*
- "verification without evidence is just opinion" → **kept** (Core Identity)

**Procedure (persona-vs-procedure boundary) → `folded-into-skill` (P5 `testing-end-user` — persona references, does not restate):**
- Parse/execute `**TEST:**` Setup/Action/Assert + evidence-capture mechanics → **folded-into-skill (P5)** (referenced in Skills-Available)
- Task-classification procedure (CLI→auto-approve if 100% pass · GUI/SUBJECTIVE→checkpoint · default-SUBJECTIVE) → **folded-into-skill (P5)** (Seam 4; referenced, not restated)
- Quality-gate execution mechanics (identify commands, run, exit-code classify, report block) → **folded-into-skill (P5)** (referenced as "running the quality gates and classifying results by exit code")

**Wiring → `kept-but-rebind` / `kept`:**
- `skills: testing-end-user` → **kept-but-rebind** (→ ported `mochiko:testing-end-user` in body reference)
- `model: opus`, `color: cyan` frontmatter → **kept**
- "gates cycle completion" / "cycle verification" / "close this cycle" wording → **kept-but-rebind** (the 3 "cycle" decouples above; gating *responsibility* survives as intrinsic craft — not a drop)

**Orchestration (not qa's to own) → `moved-to-lead` (P1) — realized OUT of the persona (confirmed absent from the artifact):**
- Execute→verify pairing/sequencing with the producer → **moved-to-lead** (§Job 2 (b); F1). Persona carries none; the pairing is declared by the lead's casting, not this persona.
- FAIL-loop driving (targeted retry, fix-pass, convergence-stall) → **moved-to-lead** (§Job 2 c/d/e; F3). Persona emits the verdict only; carries no loop-driving.
- Auto-approve-vs-checkpoint gate placement + final-acceptance wiring → **moved-to-lead** (§Job 2 gate placements; F2). Persona feeds the conservative judgment (kept); does not place the gate.

**Dropped:** NONE. No kernel/DAG/catalog plumbing in this persona to shed; "cycle" is decoupled (rebind), not dropped — the underlying gate/verify capability survives. (Consistent with reconcile gated-bundle item 9: "P3 — 0 drops.")

---

## Wave-2 wiring needed (deferred per task scope — NOT done here)

1. **Router registration** — register `qa-engineer` in the `mochiko` router with when-to-reach-it guidance. Deferred to Wave-2 (out of this task's local-wiring scope).
2. **`plugin.json` manifest** — add `agents/qa-engineer.md` to the `mochiko` plugin manifest. Deferred to Wave-2.
3. **Lead-side casting (P1 `implement` command)** — the execute→verify pairing (F1), the FAIL-loop driving (F3), and the confidence-gate placement + named final-acceptance gate (F2) land on the thin lead when P1 is transformed. Not this artifact's concern; noted so the cross-primitive wiring is not lost.
4. **P5 dependency** — the body reference `mochiko:testing-end-user` resolves once P5 lands in this same cluster run (its disposition is `port-with-edits × standalone`).

---

## Handoff to `verify-output` (Phase 4, independent agent)

An independent reader can check every claim above against the cited HIL line numbers and the artifact:
- 3 "cycle" decouples applied exactly (L5/L26/L27); whole-artifact deny-list scan clean.
- Tier-1 determinism boundary + validator spine intact and verbatim.
- Seam-4 split realized: persona = disposition only; procedure → P5; placement → lead. No classification table, no gate placement in the persona.
- Independence: only the grading skill mounted; producer skills never mounted.
- Trace complete: every assessment responsibility carries a realized tag; zero silent drops.

**Transform version:** v1 · **Governed by:** `transform-recipes` · **Next:** Phase-4 `verify-output` grades this artifact + trace independently (different agent).
