# Phase 3 Transform — P1 `implement` command

**Run:** `/mochiko:transform-cluster implement` · **Primitive:** P1 `commands/implement.md`
**Producer:** `mochiko:transform-producer` · **Skill:** `transform-recipes` · **Date:** 2026-07-01
**Consumes:** `reconcile.md` (authoritative rehome map) · `assess-implement-command.md` (43-responsibility trace) · `contract.md` §1 + the two LOCKED decisions
**Applies:** the finalized disposition `redesign × absorb-into-lead` → a thin sequential sound-loop command.

> Transform applies the decision; it does not grade it. Independent grading is Phase-4 `verify-output`, run by `mochiko:validator` (a different agent).

---

## TRANSFORM output

```
TRANSFORM: implement command (P1)
Applied:   redesign × absorb-into-lead + convention-wiring pass
Artifacts: plugins/mochiko/commands/implement.md (80 ln, WRITTEN this run)
New partners: NONE — the producer↔validator pair (staff-engineer ↔ qa-engineer) already exists
             (the inverse-of-setup nuance, reconcile §2.0); no new agent, no new skill, no loop-discipline edit.
Wiring:    classification = user-invoked (disable-model-invocation: true)
           router          = DEFERRED to Wave-2 cluster wiring (see below); command auto-registers via commands/ glob
           triggers        = n/a (user-invoked command; no model-invocation description grading)
           rebinds         = .workflow/ + specs/{id}/ → .mochiko/specs/<feature>/ (R1/R2); IMPLEMENT_STOP kill-switch path
           single-source   = references loop-discipline (×5) + agent-dispatch (×3) + workflow-contract (fill, not inline)
                             + testing-end-user (classification home ×3); ZERO restated doctrine, ZERO inlined contract
```

**Redesign target (thin-command shape).** Matches the `tasks` (77 ln) / `plan` (82 ln) altitude: frontmatter → lead/team opening → sound-loop reference paragraph → `$ARGUMENTS` → "Contract parameters (fill the artifact)" → HIL-contrast note → phased body → State-recovery table → "What you own" footer. `implement` lands at **80 ln** — between its two siblings, despite carrying the most workflow-specific machinery of any command (entry gate + scaffolding + sequential cycle loop + per-cycle execute→verify + targeted retry + fix-pass + convergence-stall + two agents + a two-tier gate). Dense, not padded.

---

## Convention-wiring pass (all six ran)

1. **Classification** — `disable-model-invocation: true` (user-invoked command). ✓
2. **Router registration** — DEFERRED to the Wave-2 cluster wiring pass (see "Wave-2 wiring needed"). The command file itself auto-registers via the `plugin.json` `"commands": "./commands/"` glob; only the `mochiko` router SKILL.md needs a hand-added Implement-cluster hint row. ✓ (noted, not silently skipped)
3. **Trigger phrasing** — n/a. A user-invoked command has no graded model-invocation triggers; the `description` frontmatter is a user-facing summary, not an auto-fire trigger.
4. **Path rebinding** — all HIL brain/DAG/carrier paths dropped; live paths rebound to `.mochiko/specs/<feature>/` (design inputs, `tasks.md` entry evidence, `cycle-report.md`, `IMPLEMENT_STOP`). Scaffolding outputs kept project-relative. (R1/R2/R3 below.)
5. **Decouple** — n/a for a command body: the deny-list (sibling-agent names / "dispatch" / injected workflow modes) targets *personas & skills*, not the lead. The lead legitimately names its team and dispatches. The command's altitude risk is *restated doctrine*, and the grep-floor is clean (below).
6. **Single-source / de-duplicate** — every generic-loop responsibility is *referenced* (`mochiko:loop-discipline` / `agent-dispatch` / `templates/workflow-contract.md`), never restated; the contract is *filled as an artifact*, never inlined; the `**TEST:**` runtime classification is *referenced* to `testing-end-user`, not copied. No strategy skill is mounted or referenced (P7 dissolved).

**Mechanical grep-floor (self-check, not a grade):** kernel/DAG/MCP/catalog/`hil-dag`/INV-*/State-Analyst/`.workflow/`/`check-prerequisites` = **0 matching lines**. No `Task(` / `AskUserQuestion(` / `pipeline(` transliteration. `strategy` = 0. Only native-Workflow `parallel()` appears — inside the deferral note, naming the *deferred* (sanctioned) capability, not a kernel.

---

## Realized responsibility trace (all 43 accounted for; artifact anchors given)

The assess trace enumerated 43 responsibilities (14 dropped · 6 dedupe · 18 moved-to-lead · 3 rebind · 1 absorbed · 1 moved-to-sibling-skill). Each realized tag below, with **where it landed** in `implement.md` so an independent reader can confirm no silent loss. Per the reconcile map, each `moved-to-lead` row is split into its *dedupe-referenced* half (generic → `loop-discipline`, never restated) and its *rehomed* half (workflow-specific param, stated).

### `moved-to-lead` (18) — workflow-specific orchestration, landed on the thin lead

| # | Responsibility | Realized landing (artifact anchor) | Reconcile split |
|---|----------------|-----------------------------------|-----------------|
| L0 | Lead role + **clearing-verdict ownership** | Opening para ("you own the clearing verdict … qa's status is input"); footer verdict clause | verdict-ownership *principle* → **dedupe**→loop-discipline Req 2; eval → L17 |
| L1 | `$ARGUMENTS` capture + empty-input (`@`-drop) recovery | `**Argument:**` line + Phase 0 step 1 (**G1**) | rehome-to-lead |
| L2 | Done-condition **end-state params** (all cycles `[x]`, final-validation clear, `tasks.md` `[x]`) | Contract "Done-condition" bullet (1)+(2)+(3); Phase 2 step 2 verdict | mechanics → **dedupe**→loop-discipline Req 1; **merges P7 L-e** |
| L3 | **(f)** entry gate — tasks-workflow-complete | Phase 0 step 2 (rebind → `.mochiko/specs/<feature>/tasks.md` present/complete) | rehome-to-lead as handoff edge; **=P7 L-f ordering-guard** |
| L4 | Load design inputs as producer inputs | Phase 0 step 4 (paths rebound; "briefed per `agent-dispatch`") | briefing → **dedupe**→agent-dispatch; paths → rehome (R1) |
| L5 | **(g)** project scaffolding — ignore-files from stack | Phase 0 step 5 | rehome-to-lead |
| L6 | **(a)** sequential cycle sequencing — foundation before feature; current = first unchecked | Phase 1 "Sequencing" | rehome-to-lead; **merges P7 L-a** |
| L7 | **(b)** execute→verify pairing — every cycle → qa, same round, never skipped | Phase 1 step 2 + footer pairing clause | shape → rehome; *independent-verification principle* → **dedupe**→loop-discipline Req 2; **merges P7 L-b(shape)** |
| L8 | **(c)** targeted retry — re-open only failed tasks, **max 3/cycle** | Contract "Bounds" + Phase 1 step 3 | tactic+param → rehome; backbone → **dedupe**→loop-discipline Req 3; **merges P7 L-c**; HOW-craft kept in P4 |
| L9 | **(d)** fix-pass — failure-scoped, unconstrained by cycle boundaries, **max 3** | Contract "Bounds" + Phase 2 step 3 | routing/param → rehome; backbone → **dedupe**→loop-discipline Req 3; **merges P7 L-d**; craft kept in P2 |
| L10 | **(e)** convergence-stall — same failure pattern **2+ rounds** → surface | Contract "Bounds"; Phase 1 step 3 + Phase 2 step 3 | param → rehome; no-progress *doctrine* → **dedupe**→loop-discipline Req 3.2 |
| L11 | Overall-loop cap / no-progress **params** + kill-switch | Contract "Bounds" (`IMPLEMENT_STOP`; no-progress = unchanged failing set; "you count") | params → rehome (fill contract); four-guards *doctrine* → **dedupe**→loop-discipline Req 3 |
| L12 | Mid-loop clarification gate | "Mid-loop gates" para (**G3**) | placement → rehome; gap-routing → **dedupe**→loop-discipline |
| L13 | **Confidence-based** per-cycle gate placement (CLI 100%-pass → auto-approve; GUI/subj/any-fail → checkpoint) | Contract "Gates" bullet + Phase 1 step 3 ("**confidence gate**") | placement/wiring → rehome; *classification procedure* → **P5** (S1); placement *doctrine* → **dedupe**→loop-discipline Req 4 |
| L14 | **Named final-acceptance** gate before "done" (NEW; HIL had none) | Phase 3 (**G5**) + Contract "Gates" bullet | placement → rehome; human-gate *presence* → **dedupe**→loop-discipline Req 4 |
| L15 | Completion / finalize — summary (cycle/task/fix-pass counts, gates, artifacts, next step) | Phase 4 | rehome-to-lead |
| L16 | Dispatch-via-Task; do-not-modify-git | Footer ("always dispatch via the Task tool"; "do not modify git or push") | dispatch → **dedupe**→agent-dispatch; git invariant → rehome |
| L17 | **State-Analyst implement-slice reclaimed** — direct Read of `cycle-report`/`verification-report`; gate-eval params (cycle-checkpoint = criteria-met + gates pass; final-validation = all `[x]` + gates + traceability + constitution) | Done-condition (3) + footer verdict clause | eval params → rehome; direct-Read tamper-proofing → **dedupe**→loop-discipline Req 2 |

### `dedupe` → `loop-discipline` / `agent-dispatch` (6) — referenced, NEVER restated

| # | Responsibility | Referenced at (artifact) | Single-source |
|---|----------------|--------------------------|---------------|
| DD1 | default-FAIL done-condition *mechanics* + "how proven" | sound-loop para; "confirm it against `mochiko:loop-discipline`" | loop-discipline Req 1 |
| DD2 | produce→check→repeat *pattern* + generic routing | sound-loop para; footer "Full rules" | loop-discipline |
| DD3 | the four iteration-guard *doctrines* (a cap / no-progress / kill-switch / escalate-don't-die exist) | Bounds "apply the bounds"; footer | loop-discipline Req 3 |
| DD4 | independent-validation *principle* | opening independence clause (realized structurally staff≠qa) | loop-discipline Req 2 |
| DD5 | gap-routing (knowledge→research, preference→human, scope→halt) | Phase 1 step 3 + Mid-loop gates ("route … per `loop-discipline`") | loop-discipline |
| DD6 | agent briefing / prompt-construction as caller-side context | every "briefed per `agent-dispatch`" | agent-dispatch |

### `kept-but-rebind` (3) — paths

- **R1** artifact input paths → `.mochiko/specs/<feature>/` (Phase 0 step 4). *(was HIL `specs/{id}/…`)*
- **R2** entry-gate evidence → `.mochiko/specs/<feature>/tasks.md` present/complete (Phase 0 step 2). *(was `{FEATURE_DIR}/.workflow/tasks-context.md` frontmatter `status`)*
- **R3** scaffolding output paths (`.gitignore`/`.dockerignore`/lint-ignore) — project-relative, kept (Phase 0 step 5).

### `absorbed-into-lead` (1) — state carrier

- **A1** `.workflow/context.md` → **no carried artifact**; in-session lead state + workspace-as-state under `.mochiko/specs/<feature>/`. Realized by the **State-recovery table** (resume purely from workspace evidence; there is no context-file `phase`/`status`).

### `moved-to-sibling-skill` (1) — reclaim

- **S1** `**TEST:**` runtime task-classification (CLI/GUI/SUBJECTIVE → auto-approve vs. checkpoint) → **P5 `testing-end-user`**; the command **consumes** it to drive L13. Realized at Phase 1 step 3: "qa classifies each verification (the CLI / GUI / SUBJECTIVE classification procedure lives in `testing-end-user`)". Paired-with P5.

### `dropped + reason` (14) — brain-plumbing (correctly ABSENT from the artifact)

D1 DAG vocabulary · D2 catalog path/assembly · D3 context-template path · D4 State-Analyst 4 verbs · D5 `hil-dag` CLI/MCP · D6 `check-prerequisites.sh` · D7 DAG paths/`mkdir` · D8 dispatch_mode routing · D9 INV-001…005 · D10 carry_forward · D11 pass lifecycle · D12 Context-Protection *device* (reversed — the lead Reads artifacts directly, loop-discipline Req 2 tamper-proofing) · D13 State-Analyst boundaries · D14 `re-brief` override. **All kernel-free; each reason enumerated in reconcile §Gated bundle item 9 for the human gate.** Grep-confirmed: 0 of these tokens survive in `implement.md`.

### P7 `strategy-implementation` survivors — merged (no double-home)

P7's six workflow-specific survivors (L-a…L-f) were **merged into their P1 twins** (=L6/L7/L8/L9/L2 above), landed **once** on the lead; P7's DD1–DD4 doctrine deduped into `loop-discipline` (no edit). **The standalone strategy skill does not exist in mochiko and is not referenced here** — grep `strategy` = 0. Strategy family closed (reconcile §1.5).

**Realized bucket counts:** moved-to-lead 18 · dedupe 6 · kept-but-rebind 3 · absorbed 1 · moved-to-sibling-skill 1 · dropped 14 → **43/43 tagged, zero silent loss.**

---

## Two LOCKED intake decisions — realized

1. **Sequential-only** — Phase 1 "Sequencing" (foundation before feature; one cycle at a time). **No** `pipeline()`/`parallel()`, **no** artifact-DAG. Parallelism named as a `deliberate-shortcut-ledger` deferral ("not a capability drop"). ✓
2. **Confidence-based + named final-acceptance gate** — the per-cycle **confidence gate** (Phase 1 step 3 + Gates bullet) and the named **G5 final-acceptance gate** (Phase 3). ✓

## Altitude self-check (verify-output item 8 anticipated — reference vs. restate)

| Doctrine | Restated? | Referenced at |
|----------|-----------|---------------|
| The four sound-loop rules | **No** — named as a reference list only (tasks/plan precedent) | sound-loop para → `mochiko:loop-discipline` |
| Validator trustworthiness tiers | **No** — qa's role described by *capability* ("real infrastructure + evidence + quality-gate exit codes"), the tier number never cited | opening + Team bullet |
| Gap-routing table | **No** — routed, not enumerated as doctrine | "route … per `loop-discipline`" |
| The filled workflow-contract | **No** — filled as an *artifact* (`→ implement-contract.md`) | Contract-parameters section |
| Tamper-proofing / Context-Protection | **No** — the *act* ("you Read the cycle-reports") without the doctrine | done-condition (3) + footer |
| Confidence-gate classification | **No** — referenced to `testing-end-user` | Phase 1 step 3 |
| `Task()` / `AskUserQuestion()` payloads | **No** — prose "Dispatch `mochiko:staff-engineer`" | throughout |
| "Supervisor behaviors" doctrine footer | **No** — the footer is workflow-specific *ownership* ("What you own"), matching tasks/plan | footer |

**Conclusion (self-check, not a grade): zero restated doctrine; every extra line over `tasks`/`plan` is a workflow-specific param (the a–g contract values, the two gates, the entry gate, scaffolding, the two-agent execute→verify loop). Independent adjudication is `verify-output`.**

---

## Wave-2 wiring needed (DEFERRED — flagged, not silently skipped)

The command's *local* wiring (frontmatter + body + team-by-name + gate names + state-recovery table) is complete. The following are **cluster-scoped** and belong to the Wave-2 cluster wiring pass:

1. **`plugin.json` `agents` array** — add `./agents/staff-engineer.md` and `./agents/qa-engineer.md` (P2/P3 land them this run; the array is explicit — skills + commands are directory globs and need no edit, so `implement.md` and the ported skills auto-register).
2. **`mochiko` router SKILL.md** — add an **Implement cluster** section (user-invoked entry hint for `/mochiko:implement`; model-invoked rows for `executing-tdd-cycle`, `testing-end-user`, `brownfield-integration`) mirroring the Setup/Specify/Tasks cluster tables.
3. **`REGISTRY.md`** (Phase-5 delta per reconcile) — L38 `implement` command `[ ]`→`[x]`; and the companion P2–P7 rows.
4. **Team-artifact dependency** — `implement.md` references `mochiko:staff-engineer`, `mochiko:qa-engineer`, `executing-tdd-cycle`, `testing-end-user`, `brownfield-integration` by name; those artifacts are produced by P2–P6 in this same cluster run and must land for the references to resolve (the tasks/plan precedent: a command ships alongside its cluster's agents/skills).

**Trace version:** v1 · **Governed by:** `transform-recipes` · **Next:** Phase-4 `verify-output` grades `implement.md` + this realized trace independently (via `mochiko:validator`).
