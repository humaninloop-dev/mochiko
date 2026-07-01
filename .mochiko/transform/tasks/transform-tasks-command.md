# TRANSFORM — P1 `tasks` command (+ absorbed P6 `tasks-context-template`)

**Run:** `/mochiko:transform-cluster tasks` · **Phase 3 (transform)** · **Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes` · **Date:** 2026-07-01
**Consumes:** `assess-tasks-command.md` (P1 trace) · `reconcile.md` (JOB 2 rehome map, JOB 3 P1/P6 re-emitted traces) · `contract.md`.
**Human gate (Phase-2 reconcile) — decisions honored:** RQ-A = **Branch A** (two artifacts / two phases) · producer report = **`taskarchitect-report-template`** (separate template, self-verdict fields dropped) · P6 = **`absorb-into-lead`** (no context file).

```
TRANSFORM: tasks command (P1) + tasks-context-template (P6, absorbed)
Applied:   redesign × absorb-into-lead + wiring-pass
Artifacts: plugins/mochiko/commands/tasks.md (77 lines; ≤ ~90 altitude target; cf. plan.md 82, specify.md 66)
New partners: none created by THIS transform (taskarchitect-report-template is the P2 dispatch's product; referenced, not authored here)
Wiring:    classification=user-invoked (disable-model-invocation: true) · router=DEFERRED to Wave-2 (scope boundary) ·
           triggers=n/a (user-invoked) · rebinds=.humaninloop→.mochiko, .workflow carrier→workspace-as-state ·
           single-source=loop-discipline + agent-dispatch referenced; workflow-contract filled at runtime, not inlined
Trace (realized): every P1 §A–§E + P6 R1–R21 responsibility tagged below; zero silent drops (audit at close)
```

**Grading is `verify-output`, a different agent — not done here.** This trace is the auditable input to that grade.

---

## Realized trace — P1 responsibilities (assessment §A–§E → final home)

### §A Generic loop-discipline mechanics → `dedupe` (REFERENCED, never copied — altitude floor)

| Responsibility | Realized tag | Where it landed in `tasks.md` |
|---|---|---|
| Produce→review iteration structure (2 phase-loops + clarification loop *as loops*) | `dedupe` | Sound-loop para invokes `loop-discipline`; phases reference it, do not restate the loop mechanics |
| Default-FAIL done-condition **mechanics** | `dedupe` | Contract done-condition ("starts FAILing"); mechanics live in `loop-discipline` (referenced), only params inlined |
| Producer↔validator independence **doctrine** | `dedupe` | Lead para "Never let the producer grade its own output" + team "Disjoint agents and skills" — the *doctrine* is `loop-discipline`, not restated |
| Validator trustworthiness tiers · tamper-proofing | `dedupe` | Not restated; carried by the filled `tasks-contract.md` (workflow-contract §2) + `loop-discipline` |
| The four iteration guards **as requirements** | `dedupe` | Bounds bullet states the *params* (cap 3/phase · no-progress · kill-switch); the requirement is `loop-discipline` |
| Gap-type routing (incl. "Research this" knowledge→research) | `dedupe` | Verdict steps "route per `loop-discipline`'s gap-routing (→ G3)"; G3 "Research this" → native `Explore` |
| Anti-rationalization (exhaustion ≠ done) | `dedupe` | "the run stays FAIL unless the human explicitly accepts"; "Out of rounds = escalate, never done" — via `loop-discipline` |
| Briefing-each-dispatch mechanics | `dedupe`→`agent-dispatch` | "briefed per `agent-dispatch`" (both produce steps + sound-loop para) |
| Standing lead footer (no git/push · Task tool · agents have no workflow knowledge) | `dedupe` | Footer one-liner: "Stay kernel-free; brief per `agent-dispatch`; dispatch via the Task tool; do not modify git or push." |

**Altitude check:** none of the four rules, tiers, gap-routing, or a filled contract is restated in the body → the verbosity defect is avoided.

### §B Workflow-specific orchestration → `moved-to-lead` (only true of THIS workflow)

| Responsibility | Realized tag | Where it landed |
|---|---|---|
| 2-phase **Mapping → Tasks** sequence (Branch A) | `moved-to-lead` | Phase 1 (Mapping) → Phase 2 (Tasks) |
| **CUMULATIVE review** — mode-select + supply BOTH artifacts (the #1 tasks-specific nuance) | `moved-to-lead` | Phase 2 step 2 "cumulative mode … you select the mode and supply **both** artifact sets {`tasks.md`}/{`task-mapping.md`}"; footer restates ownership. NOT flattened to "review tasks.md" |
| **Early mapping-review gate** (slicing reviewed before the full TDD breakdown) | `moved-to-lead` | Phase 1 intro ("grades slicing quality **before** the expensive full TDD breakdown — the tasks analogue of plan's feasibility-once gate") + Phase 1 step 2 |
| Team casting (single-reviewer `specify` shape) | `moved-to-lead` | Lead para + Team bullet (`task-architect` produces; `devils-advocate` grades; disjoint) |
| Entry-gate **plan-workflow-complete** | `moved-to-lead` | Phase 0 item 2 |
| Constitution as **governing context** (softer than plan — not a blocking gate) | `moved-to-lead` | Phase 0 item 3 ("governing context, not a blocking gate") |
| Brownfield-from-plan (inherit; no re-analysis) | `moved-to-lead` | Phase 0 item 4 |
| Done-condition **params** (end-state, cap #, gate placements) | `moved-to-lead` | Contract parameters (filled artifact, not inlined) |
| Phase-4 completion report + next-step | `moved-to-lead` (rebound) | Phase 4; next-step rebound to `/mochiko:implement` (forward reference) |
| Empty-input `@`-recovery | `moved-to-lead` | Phase 0 item 1 (G1) |
| Resume detection + state recovery | `moved-to-lead` (rebound) | State recovery table (workspace evidence) |
| Existing mid-loop gates (clarification + exit-early) | `moved-to-lead` | Mid-loop gates para (G3 clarification; G4 exit-early) — survive **alongside** the new G5 |
| Operational handling (verify-agent-output; agent-failure) | `moved-to-lead` | Footer ("verifying each dispatch actually wrote its expected files … missing output → log and ask retry/abort") |

### §C Content / path / state-carrier couplings

| Responsibility | Realized tag | Where it landed |
|---|---|---|
| `.humaninloop/memory/constitution.md` | `kept-but-rebind` | `.mochiko/memory/constitution.md` (Phase 0 item 3) |
| `specs/{feature-id}/…` + `${CLAUDE_PLUGIN_ROOT}/templates/…` | `kept-but-rebind` | `.mochiko/specs/<feature>/…`; round reports under the workspace |
| Entry-gate read of `plan-context.md` `status == completed` | `kept-but-rebind` | Phase 0 item 2 — **rebound** to "`plan.md` present" workspace evidence; "mochiko `plan` writes none" (transliteration risk resolved) |
| `.workflow/tasks-context.md` state-carrier (= P6 template) | `dropped + reason` | Absorbed into workspace-as-state + in-session; no carrier file. Reason: kernel-adjacent markdown state-carrier (3×-confirmed absorb precedent) — human-gate accepted |
| Inlined `Task()` / `AskUserQuestion()` payloads + `supervisor_instructions` prose | `dropped + reason` | Not present; dispatch via Task tool briefed per `agent-dispatch`. Reason: transliterated mechanics / altitude defect |
| HIL `iteration` counter + "no hard caps" | `dropped + reason` | Replaced by deterministic cap **3/phase** (contract bounds). Reason: LLM-judged counter violates the four-guards requirement |
| Architecture / Communication / Agents-Used diagrams | `dropped + reason` | Not present. Reason: restated doctrine/illustration (altitude) |

### §D Producer / validator content → `moved-to-sibling-skill` (NOT command-body)

Survives because P2/P3/P4 port **this run** — the command only *hints* the skills; it does not restate the procedure.

| Responsibility | Realized tag | Where it landed |
|---|---|---|
| Vertical-slice identification + TDD cycle structure | `moved-to-sibling-skill` → P3/P2 | Team bullet hints `patterns-vertical-tdd`; no procedure in the body |
| Advocate phase checks (Mapping / Tasks / **Cross-Artifact** Review) | `moved-to-sibling-skill` → P4 | Named by reference only (Phase 1 "`validation-task-artifacts` Mapping checklist"; Phase 2 "`validation-task-artifacts` Cross-Artifact Review"); the checks live in P4 |
| IP-XXX infrastructure coverage (mapping→tasks) | `moved-to-sibling-skill` → P3/P4 | Not restated in the command; lives in the siblings |
| `**TEST:**` verification-task discipline | `moved-to-sibling-skill` → P3/P4 | **Deliberately NOT named in the command body** (matches reconcile Phase-2 spec line 193, which omits it): authored by P3, checked by P4. Keeping it out is the correct altitude — it is producer/validator content, not lead orchestration |
| Brownfield `evolution-roadmap.md` / `[GAP:XXX]` read | `deferred-by-reference` | Phase 0 item 4 ("a documented stub, deferred") — roadmap track not ported this run |

### §E Missing loop-discipline gates ADDED (the four gates HIL lacked)

Requirement = `dedupe` (`loop-discipline`); placement/params = `moved-to-lead` (contract + phases).

| Added gate | Where it landed |
|---|---|
| **Default-FAIL done-condition** | Contract done-condition — both artifacts start FAIL; clear only on independent validation **+** human acceptance |
| **Lead-OWNED verdict** (advocate status = input, not the gate) | Lead para + both Verdict steps ("*you* Read the artifacts and decide") + the "Why this differs from HIL's" note |
| **Hard cap + no-progress + kill-switch** | Contract bounds — cap **3/phase**, no-progress exit, `.mochiko/specs/<feature>/TASKS_STOP` |
| **NEW human ACCEPTANCE gate on `tasks.md`** | Phase 3 (**G5**) — accept / amend (bounded re-enter) / reject; does NOT displace the mid-loop gates |

---

## Realized trace — absorbed P6 `tasks-context-template` (R1–R21 → final home)

Disposition `drop × absorb-into-lead` (no artifact produced). The dissolving carrier's slices land in the **same** lead, each homed exactly once (no double-home, no inter-primitive drop — F-P6-1 satisfied).

| P6 slice | Realized tag | Where it landed |
|---|---|---|
| R2–R4 within-phase dispatch status / iteration | `moved-to-lead` | In-session loop state + bounded round counter (contract bounds) |
| R5 `@`-input recovery / feature identity | `moved-to-lead` | Phase 0 item 1 (G1) |
| R7 `plan_status` | `moved-to-lead` | Phase 0 item 2 — workspace evidence (`plan.md` present), NOT a status field |
| R8 constitution | `moved-to-lead` | Phase 0 item 3 |
| R9 upstream INPUT reads (spec/requirements/constraints/nfrs/data-model/contracts) | `moved-to-lead` | Phase 0 item 5 |
| R10 mapping row + `phase: mapping` | `moved-to-lead` (Branch A) | Workspace file presence (`task-mapping.md`) — State recovery table |
| R11 per-artifact statuses | `moved-to-lead` | Workspace file presence (`task-mapping.md`/`tasks.md`) — State recovery table |
| R12 `architect_report_path` | `moved-to-lead` → `taskarchitect-report-template` | `taskarchitect-report.md` seeded/collected by the lead (Phase 1/2 produce) |
| R13 `advocate_report_path` | `moved-to-lead` → `advocate-report-template` | `advocate-report.md` (Phase 1/2 review) — settled, reused |
| R14 supervisor→agent dispatch channel (`## Supervisor Instructions`) | `moved-to-lead` | In-session dispatch brief per `agent-dispatch`; not a file field |
| **R15 producer/reviewer CONTENT inside Supervisor Instructions** | `moved-to-sibling-skill` | Producer content → P2/P3; reviewer content → P4. **Highest-volume silent-drop surface — landed** because the siblings port this run |
| R16 clarification history / "Research this" (`## Clarification Log`) | `moved-to-lead` | In-session human-gate history (G3); knowledge-gap → native `Explore` |
| R17 state recovery / resume | `moved-to-lead` (rebound) | State recovery table (workspace evidence, not `phase`/`status`) |
| R18 cross-context state handoff (the carrier's raison d'être) | `moved-to-lead` | Single-session lead holds state and briefs each agent with its slice |
| R20 misc dispatch bookkeeping | `moved-to-lead` | In-session loop state |
| R1 `type` node-kind · R6 `created`/`updated` timestamps · R19 ephemeral create/delete lifecycle | `dropped + reason` | No carrier file exists; in-session + workspace-as-state needs no bookkeeping — human-gate accepted |
| R21 shared carrier schema | `moved-to-other-cluster` | `implement` inherits workspace-as-state, not a re-ported carrier (not in this command) |

---

## Convention-wiring pass (applied to this artifact; scope-limited to the command file)

1. **Classification** — user-invoked: `disable-model-invocation: true` in frontmatter (matches `plan.md`). ✓
2. **Router registration** — **DEFERRED to the Wave-2 wiring pass** (the task's scope boundary: do NOT edit the router / `plugin.json` / `devils-advocate.md`). Recorded here so it is not a silent drop: the router needs a Tasks-cluster section + a `/mochiko:tasks` Entry-point row (reconcile Flag 7.2). Not done in this transform. ⏸
3. **Triggers** — n/a: a user-invoked command carries no model-invoked trigger phrasing; the frontmatter `description` is the user-facing summary. ✓
4. **Path rebinding** — all paths `.mochiko/…` (`.mochiko/specs/<feature>/`, `.mochiko/memory/constitution.md`, `TASKS_STOP`); zero `.humaninloop/`, zero kernel/DAG/MCP/catalog paths; the `.workflow/` carrier is gone (workspace-as-state). ✓
5. **Decouple** — n/a for the **lead** command: naming the dispatched agents (`task-architect`, `devils-advocate`) is the required team casting, and "Dispatch" is the Task-tool verb every lead uses (`plan.md`/`specify.md` precedent). The deny-list applies to personas/skills, not the supervisor. ✓
6. **Single-source / de-duplicate** — references `loop-discipline` + `agent-dispatch`; fills a `workflow-contract` artifact at runtime (`.mochiko/specs/<feature>/tasks-contract.md`); no restated rules/tiers/gap-routing, no inlined filled contract. ✓

---

## Silent-drop audit — the 6 assessment §F risks, each confirmed surviving

| Silent-drop risk (assessment §F) | Survived? | Evidence in `tasks.md` |
|---|---|---|
| **CUMULATIVE review** (trivially flattened to "review tasks.md") | ✅ | Phase 2 step 2 + footer: cumulative mode, lead selects mode + supplies BOTH artifact sets |
| **Entry-gate plan status rebind** (don't transliterate a status field) | ✅ | Phase 0 item 2: rebound to `plan.md`-present workspace evidence; "mochiko `plan` writes none" |
| **Early mapping gate / two-phase sequence** (Branch A) | ✅ | Phase 1 (early mapping review before the full breakdown) |
| **§D producer/validator content** (slice/TDD/IP-XXX/`**TEST:**`) | ✅ | Command hints `patterns-vertical-tdd` / `validation-task-artifacts`; the capability lives in P2/P3/P4 (ported this run) |
| **Existing mid-loop gates** (must not be displaced by G5) | ✅ | Mid-loop gates para keeps G3 + G4; Phase 3 G5 is additive |
| **"Research this" knowledge-gap→research branch** | ✅ | G3 routes a "Research this" answer to native `Explore`, never the user |

**Result: zero silent drops.** Every P1 §A–§E responsibility and every P6 R1–R21 slice carries a realized tag with a concrete home; every `dropped` carries a reason the human gate accepted (reconcile §GATED-DECISIONS item 6). The four gates HIL lacked are added, not merely relocated.

**Out-of-scope for this transform (Wave-2 wiring pass — flagged, not dropped):** router Tasks-cluster section + `/mochiko:tasks` entry row; `plugin.json` `"./agents/task-architect.md"`; `devils-advocate` `validation-task-artifacts` re-mount (stub→live). These are P2/P3/P4/wiring responsibilities, not command-body.

**Next:** `verify-output` (independent `mochiko:validator`, a different agent) grades `plugins/mochiko/commands/tasks.md` + this realized trace — five-convention conformance, sound-loop placement, command-altitude scan (item 8), and trace completeness.
