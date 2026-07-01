# Assessment — P5 `testing-end-user` (skill)

**Run:** `implement` cluster · **Producer:** `mochiko:transform-producer` · **Date:** 2026-07-01
**Sources read:** HIL `skills/testing-end-user/SKILL.md` (280 ln) + all 4 refs (`TASK-PARSING.md`, `EVIDENCE-CAPTURE.md`, `REPORT-TEMPLATES.md`, `TESTING-EVIDENCE.md`); boundary partner `plugins/mochiko/skills/patterns-vertical-tdd/SKILL.md` + `references/CYCLE-STRUCTURE.md`; run `context.md` + `contract.md`.

---

```
ASSESSMENT: testing-end-user
Class:        skill → branch PLAYS-a-role (model-invoked, agent-consumed)
Triage:       gate1=y gate2=y gate3=y  → full-lens
Disposition:  port-with-edits × standalone  (+ flag-for-reconcile: grammar dedupe w/ patterns-vertical-tdd)
```

Confirms the anticipated `port-with-edits × standalone`. Body is kernel-free and sound; the edits are localized (decouple orchestration phrasing, re-key the description to work-context, rebind evidence paths, reference the grammar instead of restating it). Not `keep-verbatim` (it owes decoupling + dedupe edits); not `redesign` (the runtime procedure needs no rebuild). Structural home is single: `qa-engineer`'s (P3) core validator-side skill. No structural split — it is the validator half by construction and does not self-grade.

---

## Role (two altitudes)

- **Skill-role:** consumed-procedure (a runtime execution discipline). Weakly emits-artifact — an *ephemeral* verification report (REPORT-TEMPLATES.md L316: "not persisted to disk by default").
- **Team-role it confers on its caller:** **validator.** This is the independent-validation half of the implement workflow's producer↔validator pair (`staff-engineer` produces via P4 `executing-tdd-cycle`; `qa-engineer` validates via this skill). Independence requirement: this skill mounts on the validator agent **only** — it must never share an agent with the producer-side P4. Enforced at the P3 `skills:` list, not named in the body.

## Triage rationale (why full-lens)

- **gate1 (orchestration-coupled?) = y** — body carries orchestration-*coupling* phrasing ("When dispatched as part of an implementation cycle verification", L154; "surfaced … to the cycle-checkpoint gate", L200). It is coupling to decouple, not a hard functional dependency: the core procedure runs on any `**TEST:**` task standalone. No content-coupling (no kernel/DAG/catalog/MCP).
- **gate2 (multi-responsibility / fans out?) = y** — parse, execute (setup/action/assert), evidence, runtime classification, result classification, quality-gate execution + auto-resolution, report generation, checkpoint presentation.
- **gate3 (non-machine-checkable artifact?) = y** — GUI/SUBJECTIVE verification results require human judgment (the report + recommendation). The CLI / quality-gate path is deterministic (exit codes) — that layer is machine-checkable, and is the validator's Tier-1 ground truth.

---

## LOAD-BEARING FINDING 1 — the reclaimed `**TEST:**` runtime-classification lands here **intact**

HIL SKILL.md **L111–122 "Task Classification"** is present and un-diluted:

| Classification | Criteria | Checkpoint behavior |
|---|---|---|
| CLI | backtick commands + measurable asserts | may auto-approve if 100% pass |
| GUI | UI actions / screenshot | always human checkpoint |
| SUBJECTIVE | qualitative terms | always human checkpoint |
| *(default)* | uncertain | **default to SUBJECTIVE (safe fallback)** |

This is the "who decides when to involve a human" mechanic parked → `implement/qa` in the tasks port (context.md L44). **P5 is its canonical runtime home; confirmed intact.** It is the **procedure half** of the confidence-based impl-gate's three-way split:

1. **placement + final-acceptance wiring** → the **lead** (P1 command) — where the gate sits; the named final acceptance gate.
2. **conservative judgment / disposition** → the **qa-engineer agent** (P3) — default-to-human-when-uncertain persona.
3. **the classification *rule itself*** → **this skill (P5)** — CLI+measurable+100%pass ⇒ auto-approve; GUI/SUBJECTIVE/any-fail ⇒ human checkpoint; default SUBJECTIVE.

Trace tag: **kept** (canonical runtime home; reclaimed, no drop). Maps to `loop-discipline`'s "low validator-confidence only" gate placement, per contract.md L13.

## LOAD-BEARING FINDING 2 — the design↔runtime boundary is INVERTED for the grammar → **flag-for-reconcile**

Two facets of the `**TEST:**` construct, two owners:

- **Authoring facet (design-time grammar):** marker variants, field names, action-modifier catalog, assert-pattern catalog. **Canonical owner = `patterns-vertical-tdd`** — `CYCLE-STRUCTURE.md` L3 declares itself "the canonical home of the `**TEST:**` verification-task grammar."
- **Runtime facet (execution):** detect/parse → execute → capture → classify → report → checkpoint. **Canonical owner = P5.**

**Classification boundary is already clean** (correct direction): `CYCLE-STRUCTURE.md` L248–256 explicitly *cedes* the CLI/GUI/SUBJECTIVE runtime decision to "a downstream verification step" — that step is P5. patterns-vertical-tdd points **to** P5; P5 owns the classification. ✓

**Grammar boundary is violated** (duplication): P5 **re-defines** the grammar that patterns-vertical-tdd canonically owns —
- SKILL.md **L50–55**: supported-marker list (`**TEST:**` / `TEST:VERIFY` / `TEST:CONTRACT` / `HUMAN VERIFICATION`) — dup of CYCLE-STRUCTURE.md L238–246 + L350–355.
- SKILL.md **L81–86**: action-modifier catalog (`(background)`/`(timeout Ns)`/`(in path)`) — dup of CYCLE-STRUCTURE.md L268–273.
- SKILL.md **L93–99**: assert-pattern catalog (`Console contains` / `File exists` / `Response status`) — dup of CYCLE-STRUCTURE.md L277–282.
- `TASK-PARSING.md` **field-marker + modifier + assert tables** (L54–65, L135–197) re-encode the same legal set (as regexes).

This is the exact hazard named in contract.md L16 ("the `**TEST:**` grammar stays owned by patterns-vertical-tdd, *consumed* by testing-end-user"). **Emit flag-for-reconcile.** Anticipated resolution = **dedupe / single-source** (a wiring action, not a structural split): P5 **references** CYCLE-STRUCTURE.md for the *legal set* (which markers/fields/modifiers/assert-patterns exist) and keeps only the **execution/parse semantics** (how to detect, extract, run a `(background)` action + track PID, evaluate `Console contains` against captured output). The catalog and the how-to must be split cleanly so they cannot drift.

**Reciprocal minor note (for reconcile):** CYCLE-STRUCTURE.md L250–254 reproduces the CLI/GUI/SUBJECTIVE table "for context" even though it cedes ownership. Canonical source is P5. Lower priority (direction already correct; patterns-vertical-tdd is already ported) — reconcile decides whether to slim that table to a reference back to P5.

**Sibling P4 (`executing-tdd-cycle`) boundary — confirm at reconcile, not asserted here:** P4 is the producer-side runtime (write code red/green/refactor); P5 is the validator-side runtime (verify against real infra). Complementary, opposite sides of the same `**TEST:**` gate. Confirm at reconcile that P4 does not re-implement P5's substance (evidence capture / checkpoint presentation / result classification) and that shared discipline is a referenced banner, not duplicated body.

---

## Lens (7 checks, weighted for skill/PLAYS-a-role)

1. **Orchestration test** — Orchestrated in HIL by the implement command's DAG-Supervisor dispatching `qa-engineer` → this skill. Content-coupling: **none** (no kernel/DAG/catalog/MCP/`hil-dag`/`.humaninloop` in body; only native `AskUserQuestion` + shell). Orchestration-coupling: the "dispatched as part of an implementation cycle" (L154) and "cycle-checkpoint gate" (L200) phrasing. On DAG-Supervisor dissolution (P1 → thin lead), the *dispatch/sequencing* and *gate-placement* re-home to the lead (already covered by contract §1 constraint (b) execute-then-verify + (c) targeted retry); this skill's *phrasing* is decoupled to role-neutral.
2. **Role (two altitudes)** — see Role section above. consumed-procedure + confers **validator**.
3. **Independence** — no self-grade leak at the skill. The load-bearing constraint sits at the agent tier: this validator-side skill on `qa-engineer` **only**; producer-side P4 on `staff-engineer` **only**. Contract §2 keeps the pair split. Body must not name `qa-engineer` (see decouple).
4. **Verdict-sink / loop-driver** — Output = verification report + checkpoint decision (approve/reject/retry) + quality-gate results. Consumers = the human (at the checkpoint) and the lead's cycle-checkpoint gate (P1). **Loop-on-FAIL is the lead's**, not this skill's: targeted retry (re-open failed tasks, max 3/cycle — contract constraint (c)) = `moved-to-lead`. This skill *presents* the checkpoint and *reports* the verdict; it does not own the retry loop. **Deterministic sub-verdict:** quality-gate exit codes are Tier-1 ground truth that auto-resolve (no checkpoint) and feed the gate deterministically — must survive un-diluted (see trace #8).
5. **Sibling / overlap** — the big one. **P5 ↔ patterns-vertical-tdd: grammar duplicated → flag-for-reconcile (dedupe).** Reciprocal classification-table note. P4 boundary to confirm at reconcile. (Details in Finding 2.)
6. **Coupling audit** — (a) **Hardcoded evidence paths** `/tmp/claude/qa-engineer-{task}-*.log` (EVIDENCE-CAPTURE.md throughout; REPORT-TEMPLATES.md L297) bake the sibling-agent name `qa-engineer` into filenames → `kept-but-rebind` (role-neutral prefix / mochiko scratch). (b) **Upstream handoff:** consumes `tasks.md` `## Quality Gates` + `plan.md` build config (L160) — design→runtime handoff edge (patterns-vertical-tdd authors `tasks.md`; P5 consumes). Relative artifact names, no `.humaninloop/` absolute paths — confirm the handoff edge, no rebind needed beyond workspace root. (c) **Determinism boundary (a STRENGTH — preserve):** deterministic = quality-gate exit codes + CLI measurable asserts (auto-approve at 100%); model/human judgment = GUI/SUBJECTIVE classification, PARTIAL results, rich-report analysis, checkpoint decision. This split is what makes qa a Tier-1 validator (contract §2) — must not be diluted. (d) Checkpoint uses native `AskUserQuestion` — keep.
7. **Conventions + loop placement + decoupling scan** —
   - **Classification tag:** model-invoked (no `disable-model-invocation`); keep. But it is **agent-consumed** — per convention 3, description must describe the *work*, not user utterances.
   - **Trigger phrasing:** graded RFC-2119 **present** (MUST primary + SHOULD secondary) ✓ — but keyed to "when the user says 'TEST:'…", a false-auto-trigger framing for an agent-consumed skill → `port-with-edits`: keep the MUST/SHOULD grading, re-key to work-context (executing `**TEST:**` verification against real infrastructure — parse Setup/Action/Assert, capture evidence, classify CLI/GUI/SUBJECTIVE, run quality gates, present a checkpoint).
   - **Decouple scan (deny-list hits):** "dispatched" + injected phase "implementation cycle verification" (L154); "cycle-checkpoint gate" (L200); `qa-engineer` baked in evidence filenames. All → decouple / rebind. Keystone-test survivors (parse/execute/classify/evidence/report/checkpoint; anti-rationalization content) are true of this verification professional on any job → **keep**.
   - **Single-source:** the `**TEST:**` grammar → reference patterns-vertical-tdd, don't restate → flag-for-reconcile.
   - **Loop placement — supplies all three:** done-condition = the Quality Gates checklist (L144–152) gating checkpoint presentation ✓; independent validation = this skill *is* the validator-side procedure ✓; human gate = the checkpoint presentation (L107–109; REPORT-TEMPLATES checkpoint) + the confidence-based auto-approve (reclaimed classification) ✓.

---

## Responsibility trace (done-condition — every responsibility tagged, no silent drop)

| # | Responsibility (source) | Tag |
|---|---|---|
| 1 | TEST-task **detection/parsing** — extract Task ID/type/setup/actions/asserts/capture/human-review (SKILL L38–72; TASK-PARSING.md algorithm) | **kept** (execution-side) — *but the grammar catalog it parses (marker variants, field/modifier/assert names) →* **dedupe** *(reference patterns-vertical-tdd; flag-for-reconcile)* |
| 2 | **Setup execution** — sequential, fail-fast, record output (SKILL L73–75) | kept |
| 3 | **Action execution** — modifiers background/timeout/in, console capture, PID tracking, timeout enforcement (SKILL L78–87; EVIDENCE-CAPTURE.md) | kept (execution semantics) — *modifier catalog →* dedupe/reference |
| 4 | **Assert evaluation** — Console contains / File exists / Response status vs evidence (SKILL L89–99) | kept (evaluation) — *assert-pattern catalog →* dedupe/reference |
| 5 | **Evidence capture** — console/screenshot/logs/timing, structured storage, background cleanup, timeout partial-output (EVIDENCE-CAPTURE.md) | kept; **evidence path filenames embedding `qa-engineer` →** kept-but-rebind (role-neutral / mochiko scratch) |
| 6 | **CLI/GUI/SUBJECTIVE runtime classification** — auto-approve vs human-checkpoint; default SUBJECTIVE (SKILL L111–122) — **THE RECLAIM** | **kept** (canonical runtime home; procedure half of the confidence-based gate 3-way split) |
| 7 | **Result classification** — PASS/FAIL/PARTIAL/TIMEOUT/ERROR (SKILL L124–132) | kept |
| 8 | **Quality-gate execution + deterministic auto-resolution** — identify gate cmds, run lint/build/test, exit 0=pass / non-zero=fail, **no human checkpoint**, feed the gate (SKILL L154–208) — **Tier-1 exit-code ground truth** | **kept — un-diluted**; the "cycle-checkpoint gate" phrasing → kept-but-rebind (role-neutral); the *sequencing* "as part of the cycle" → the lead references it |
| 9 | **Report generation** — minimal(pass)/rich(fail)/PARTIAL/TIMEOUT/ERROR, truncation rules, `quality_gates` YAML frontmatter (REPORT-TEMPLATES.md; SKILL L166–190) | kept |
| 10 | **Checkpoint presentation** — AskUserQuestion approve/reject/retry, human gates completion, retry-adjustments prompt (SKILL L107–109; REPORT-TEMPLATES L223–278) | **kept** (runtime human-gate presentation) — *the decision to loop on reject (targeted retry) is the lead's →* moved-to-lead (P1 constraint (c)) |
| 11 | **Anti-rationalization** — red-flags, common-rationalizations, common-mistakes, "no exceptions" (SKILL L202–273) | kept (intrinsic craft; keystone-passes) |
| 12 | **Legacy format normalization** — HUMAN VERIFICATION → unified mapping (TASK-PARSING.md L257–299) | kept (runtime normalization) — *legacy-marker list overlaps grammar →* dedupe/reference |
| 13 | **Skill test provenance** — RED/GREEN/REFACTOR hardening record (TESTING-EVIDENCE.md) | kept (own provenance/maintenance doc) |
| 14 | **Orchestration framing** — "dispatched as part of an implementation cycle" (L154) | decouple (kept-but-rebind phrasing); the actual dispatch/execute-then-verify pairing → moved-to-lead (P1 constraint (b)) |

**No `dropped` tags.** Everything is kept / kept-but-rebind / dedupe-reference / (orchestration) moved-to-lead. No silent capability loss.

---

## Reconcile flags (relational signals for `reconcile-cluster`)

1. **Grammar dedupe (primary, contract §Boundary-integrity):** P5 restates the `**TEST:**` grammar (marker variants + action-modifier catalog + assert-pattern catalog + field definitions, in SKILL.md L50–99 and TASK-PARSING.md) that `patterns-vertical-tdd`/`CYCLE-STRUCTURE.md` canonically owns. → **dedupe / single-source:** P5 references the legal set; keeps only execution/parse semantics. Confirm no drift. (Structural disposition stays `standalone`; this is a wiring/reference resolution, not a split.)
2. **Reciprocal classification table (minor):** CYCLE-STRUCTURE.md L250–254 reproduces the CLI/GUI/SUBJECTIVE table that is canonically P5's. Reconcile decides whether to slim it to a reference back to P5. Direction already correct (patterns-vertical-tdd cedes ownership); low priority.
3. **P4 ↔ P5 runtime boundary (confirm):** confirm `executing-tdd-cycle` (producer-side runtime) does not duplicate P5's substance (evidence capture / checkpoint / result classification); shared discipline banner referenced, not duplicated.
4. **Independence mount (confirm at reconcile/transform):** P5 mounts on `qa-engineer` only; P4 on `staff-engineer` only — never co-mounted (would recreate the self-grade sin).

## Rehome pointers already owned by P1 (the lead) — noted, not this skill's to place

- execute-then-verify pairing / dispatch-after-each-cycle (constraint (b)) · targeted-retry loop on reject, max 3/cycle (constraint (c)) · gate placement + named final acceptance (intake decision 2). This skill *supplies* the verdict + checkpoint; the lead *consumes and loops*.
