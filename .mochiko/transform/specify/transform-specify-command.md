# TRANSFORM (realized trace) — `specify` command (P1)

Run: `transform-cluster specify` · Phase 3 (transform) · Producer: `transform-producer` · Skill: `mochiko:transform-recipes`
Realized: 2026-06-27 · Governed by `loop-discipline`
Artifact under trace: `plugins/mochiko/commands/specify.md` (FINISHED; this is the RECORD of where every responsibility landed — **not** a grade; Phase-4 `verify-output` grades it).
Inputs: `assess-specify-command.md` (A1–A35 + B1–B14 + C1–C4) · `reconcile.md` §B (rehome map B.1–B.7) + §E (P1 row + drop table).

> **This is a record of what landed, not a grade.** Every responsibility in the assessment and the reconcile rehome map is confirmed against the actual command text, with a section/line anchor. No PASS/FAIL is rendered here.

---

## FLAGS — silent-drop check (read first)

**Zero genuine silent-drops.** Every domain capability in §B/§E landed exactly where the rehome map says, at the anchor cited in the trace below. The four ADDED gates all landed. The A30 reversal, the deterministic constitution-prereq, Input Assessment, targeted-revision, and Gap-Classification routing are all present.

Two **benign, precedented deviations** from the assessment's *literal* tags are recorded for transparency (neither is a capability loss; both match the all-PASS `setup` precedent — surfaced so `verify-output` can confirm, not because they are at risk):

1. **A7 (resolve project root via `git rev-parse`) — assessment tag `kept`; realized as `kept-but-rebind`.** The written command has **no** `git rev-parse`/`PROJECT_ROOT` resolution (`grep` confirms absent). All paths are cwd-relative `.mochiko/specs/<feature>/` and `.mochiko/memory/`. The *capability* (root the workspace at the project) is realized via the relative-path convention (folds into A14), exactly as `setup.md` does (`mkdir -p .mochiko/memory`, no `rev-parse`). The explicit `git rev-parse` *mechanism* is shed; the capability survives. Not a domain-capability drop.
2. **Classification tag — §G calls for "user-invoked (`disable-model-invocation: true`)"; realized as a bare slash command.** Neither `specify.md` nor the all-PASS `setup.md` carries `disable-model-invocation: true`. A command is user-invoked **by nature** (it is a slash command); that frontmatter flag is a *skill* convention, not a command one. Classification is correctly realized; the flag's absence matches the precedent, not a gap.

No other deviation. Nothing in §B is homeless; nothing in §E's drop table re-appears as a live capability.

---

## TRANSFORM

```
TRANSFORM:    specify command (P1)
Applied:      redesign × rewire-cluster + convention-wiring pass
Artifacts:    plugins/mochiko/commands/specify.md  (CREATE — the lead; absorb-target for §B)
New partners: none  (producer↔validator pair already exists: requirements-analyst ↔ devils-advocate,
                     disjoint skills, different agents — CONFIRMED, not constructed. No reviewable
                     artifact lacks a validator; nothing promoted onto the validator.)
Wiring:       classification = user-invoked slash command (matches setup precedent; no
                               disable-model-invocation flag — that is a skill convention)
              router         = SEPARATE artifact (skills/mochiko/SKILL.md EDIT, §D) — not this file;
                               not traced here
              triggers       = N/A (command, not a model-invoked skill)
              rebinds        = .humaninloop/memory/constitution.md → .mochiko/memory/constitution.md (0b/2b/2c);
                               specs/{feature-id}/.workflow/ → .mochiko/specs/<feature>/ (throughout);
                               kill-switch sentinel → .mochiko/specs/<feature>/SPECIFY_STOP (§3, 2b/2c/2d);
                               catalog/DAG/MCP paths → dropped (absent);
                               agent/skill refs → mochiko: (requirements-analyst, devils-advocate,
                               analysis-iterative, loop-discipline);
                               next-steps /humaninloop:plan → /mochiko:plan reference stub (Phase 4)
              decouple       = command legitimately names its agents / "dispatch" / phases (lead = PASS by
                               construction); workflow vocab is NOT pushed into agent personas — agents
                               briefed per agent-dispatch.md (Supervisor behaviors, line 328)
```

---

## Trace (realized) — Section A: the command's own 35 responsibilities

| # | Responsibility | Assess tag | Realized tag | Landed at (in `commands/specify.md`) |
|---|----------------|-----------|--------------|---------------------------------------|
| A1 | Argument parse + lost-input (`@`-bug) recovery | `kept` | **`kept`** | Phase 0a (`feature_description = $ARGUMENTS.trim()`; empty → G1 `AskUserQuestion`), L69–89 |
| A2 | Goal: produce validated `spec.md` (deliverable) | `kept` | **`kept`** | Intro L7; Done-condition; Phase 4 artifacts L267–269 |
| A2′ | Done-condition definition → default-FAIL, lead-owned | `kept-but-rebind` | **`kept-but-rebind`** | Done-condition L15–25 + contract §1 L33 (advocate `ready` AND lead-confirmed AND human acceptance; initial state FAIL) |
| A3 | DAG Vocabulary (node/edge/pass types) | `dropped + reason` | **`dropped + reason`** | Absent — kernel-free; no DAG vocab anywhere |
| A3′ | Gate-verdict taxonomy `ready`/`needs-revision`/`critical-gaps` | `kept-but-rebind` | **`kept-but-rebind`** | §2 verdict ownership L44; 2c dispatch prompt L189; 2d routing L201–212 (advocate's *recommended* status; lead routes) |
| A4 | Catalog resource path | `dropped + reason` | **`dropped + reason`** | Absent — catalog excluded |
| A4′ | Context-template resource path | `kept-but-rebind` → absorbed | **`kept-but-rebind`** (absorbed) | No context file; in-session + workspace, stated L285, L305 (B.2) |
| A5 | "Ask the Analyst" outbound verb | `dropped + reason` | **`dropped + reason`** | Absent — state-analyst dissolved; no delegation layer |
| A5′ | "Dispatch the agent" outbound verb | `kept` | **`kept`** | 2b `Task(requirements-analyst)`; 2c `Task(devils-advocate)` |
| A6 | "Zero direct CLI / all hil-dag delegated" | `dropped + reason` | **`dropped + reason`** | Absent — lead runs bash directly (0b/2a/2b) |
| A7 | Resolve project root (`git rev-parse`) | `kept` | **`kept-but-rebind`** ⚑ | No `rev-parse`; cwd-relative `.mochiko/` workspace (folds into A14; setup precedent). See flag #1. |
| A8 | `create-new-feature.sh` invocation (script) | `dropped + reason` | **`dropped + reason`** | Absent — 2a uses `mkdir -p` + `cp`, no script |
| A8′ | Feature-workspace create + id derivation + seed `spec.md` | `moved-to-lead` | **`moved-to-lead`** | 2a L131–140 (kebab-case slug; `mkdir -p`; `cp` spec-template → spec.md) |
| A9 | Verify/install `hil-dag` MCP | `dropped + reason` | **`dropped + reason`** | Absent — kernel-free |
| A10 | Parse script JSON for BRANCH_NAME/SPEC_FILE/FEATURE_NUM | `dropped + reason` | **`dropped + reason`** | Absent — feature-id folds into A8′ slug derivation |
| A11 | `mkdir .workflow/dags`, set `dag_path` | `dropped + reason` | **`dropped + reason`** | Absent — no DAG directory |
| A12 | Create `context.md` from context-template | `dropped + reason` | **`dropped + reason`** | Absent — no context-handoff file (L285, L305) |
| A12′ | Carry run state across the loop (capability) | `moved-to-lead` | **`moved-to-lead`** | In-session + workspace: 2a `round = 1`; 2d round counter; State recovery L303–315 |
| A13 | Seed `spec.md` from template, fill placeholders | `kept-but-rebind` | **`kept-but-rebind`** | 2a lead seeds (`cp`); 2b producer Reads seeded template then authors L154–155; path `.mochiko/specs/<feature>/` |
| A14 | Absolute-paths-rooted-at-PROJECT_ROOT convention | `kept-but-rebind` | **`kept-but-rebind`** | Rebound to cwd-relative `.mochiko/specs/<feature>/` + `.mochiko/memory/` (throughout) |
| A15 | Supervisor-loop brief-and-assemble delegation | `dropped + reason` | **`dropped + reason`** | Absent — no analyst/DAG assembly; briefing capability → B1/B8 (2b/2c) |
| A16 | Override / `re-brief` | `dropped + reason` | **`dropped + reason`** | Absent — fixed produce→validate loop, no node-menu |
| A17 | Per-node `dispatch_mode` routing | `dropped + reason` | **`dropped + reason`** | Absent — DAG dispatch-mode machinery |
| A17′ | Actual dispatch capability | `moved-to-lead` | **`moved-to-lead`** | 2b/2c `Task`; Phase 1 `Skill(analysis-iterative)`; 0a/2b `AskUserQuestion` collect-input |
| A18 | Supervisor-owned `collect-input` (`AskUserQuestion`) | `kept` | **`kept`** | Clarification sub-gate 2b L169; G1/G2/G3 |
| A18′ | Supervisor-owned `evaluate-gate` → lead owns verdict | `moved-to-lead` | **`moved-to-lead`** | 2d "you own the verdict" L197–201; §2 verdict ownership L44; Supervisor behaviors L322 |
| A18″ | Supervisor-owned `verify-milestone` | `moved-to-lead` | **`moved-to-lead`** | 2d lead confirms done-condition L199–201; Done-condition L15–25 |
| A19 | `parse-and-advance` delegation | `dropped + reason` | **`dropped + reason`** | Absent — lead reads reports directly (2d); capabilities → B4/B6 |
| A20 | `update-and-advance` delegation | `dropped + reason` | **`dropped + reason`** | Absent — capture-answers → lead (2b clarification fed into next dispatch) |
| A21 | Evaluate-and-Route on `advance.action_taken` | `dropped + reason` | **`dropped + reason`** | Absent — routing logic → lead 2d L201–212 |
| A22 | Rule 1 — `needs-revision` → freeze + new pass | `moved-to-lead` | **`moved-to-lead`** | 2d "Gaps remain … loop back to 2b" L202–211; freeze → round counter |
| A23 | Rule 2 — `ready` → completion | `moved-to-lead` + redesigned | **`moved-to-lead`** (redesigned) | 2d confirm `ready` → Phase 3 L201; done **only** after acceptance (Phase 3 → 4) |
| A24 | Rule 3 — `critical-gaps` → escalate | `moved-to-lead` | **`moved-to-lead`** | 2d `critical-gaps` → 2e L212; 2e escalate |
| A25 | Rule 5 — convergence stall (no-progress) | `moved-to-lead` | **`moved-to-lead`** | §3 no-progress exit L50; 2d unchanged-gap-set → escalate L209 |
| A26 | Rule 6 — 5-pass cap → hard round cap | `moved-to-lead` + `kept-but-rebind` | **`moved-to-lead` + `kept-but-rebind`** | §3 hard cap **3** L49; 2d `round >= 3` → escalate L208 (surface → escalate) |
| A27 | Rule 7 — unexpected/error → present to user | `moved-to-lead` | **`moved-to-lead`** | 2e escalate L214–230; Supervisor behaviors |
| A28 | Pass lifecycle / freezing (StrategyGraph) | `dropped + reason` | **`dropped + reason`** | DAG freezing absent; round demarcation → lead (round counter 2a/2d) |
| A29 | Completion: output summary | `kept` | **`kept`** | Phase 4 report L261–283 |
| A29′ | Completion: "update context status to `completed`" | `dropped + reason` | **`dropped + reason`** | Absent — no context-file status; state-recovery-from-workspace instead |
| A29″ | Next-steps → `/humaninloop:plan` | `kept-but-rebind` | **`kept-but-rebind`** | Phase 4 L282 `/mochiko:plan` *(reference stub — plan not ported yet)* |
| A30 | "NEVER read domain agent reports directly" | `dropped + reason` (reversal) | **`dropped + reason`** (deliberate reversal) | **Reversed & documented:** 2d "Read the evidence yourself" L199; Supervisor behaviors L322 ("deliberate reversal of HIL's 'never read agent reports directly' …"); Done-condition DROPPED note L25 |
| A31 | "NEVER call hil-dag / zero CLI; ALWAYS parse-and-advance" | `dropped + reason` | **`dropped + reason`** | Absent — analyst/kernel protocol |
| A32 | Responsibility-Boundaries table (Supervisor vs Analyst) | `dropped + reason` | **`dropped + reason`** | Division dissolved; replaced by "Supervisor behaviors (what you own, not the agents)" L319–329 (lead owns all) |
| A33 | Note: do NOT modify git config / push | `kept` | **`kept`** | L329 |
| A34 | Note: always use Task tool for agents | `kept` | **`kept`** | L329 |
| A35 | "Domain agents have NO workflow knowledge — context via files" | `kept-but-rebind` | **`kept-but-rebind`** | L328 (brief per `agent-dispatch.md`; context in dispatch prompt + workspace, never baked into personas) |

---

## Trace (realized) — Section B: the dissolving `state-analyst` orchestration (RF1)

Every (b) judgment capability re-homes onto this lead; only the (a) kernel *mechanism* drops.

| # | State-analyst responsibility | Assess tag | Realized tag | Landed at |
|---|------------------------------|-----------|--------------|-----------|
| B1 | Produce briefing (state summary, pass/round context, patterns) | `moved-to-lead` | **`moved-to-lead`** | In-session briefing assembled into 2b/2c dispatch prompts (round, gap list, governing context) |
| B2 | Gap classification (knowledge / preference / scope) | `moved-to-lead` (tentative; RF3) | **`moved-to-lead`** (applies `loop-discipline`'s folded-in taxonomy) | 2d "classify each gap per `loop-discipline`'s Gap Classification" L202–206; Supervisor behaviors "Classify gaps (via loop-discipline)" L325. *(Taxonomy home = `loop-discipline` edit, a sibling artifact; the command applies it.)* |
| B3 | Ranked recommendations / next-node selection from catalog | `dropped + reason` | **`dropped + reason`** | Absent — fixed produce→validate loop, no node catalog |
| B4 | Report parsing (extract verdict/gaps/summary) | `moved-to-lead` | **`moved-to-lead`** | 2d "Read them **directly** — there is no parse layer" L199; structured by P12/P13 |
| B5 | Node assembly via `hil-dag` | `dropped + reason` | **`dropped + reason`** | Absent — DAG/MCP |
| B6 | Pass freezing via `hil-dag` | `dropped + reason` | **`dropped + reason`** | Mechanism absent; round demarcation → lead (= A28) |
| B7 | Status updates via `hil-dag` | `dropped + reason` | **`dropped + reason`** | Absent — DAG/MCP |
| B8 | NL prompt construction (point agents at artifacts; minimal brief) | `moved-to-lead` | **`moved-to-lead`** | 2b/2c dispatch prompts; Supervisor behaviors agent-dispatch convention L328 |
| B9 | Graph invariant validation | `dropped + reason` | **`dropped + reason`** | Absent — DAG kernel |
| B10 | Constitution prereq (INV-002 `carry_forward` auto-resolution) | mech `dropped` + cap `moved-to-lead` | **mech `dropped + reason` + cap `moved-to-lead`/`kept-but-rebind`** | **0b "deterministic handoff edge"** L91–100: `test -f .mochiko/memory/constitution.md`; missing → "do **not** silently proceed … Never auto-resolve" (INV-002 silent recovery removed) |
| B11 | Convergence-watch (gaps trend across passes) | `moved-to-lead` (= A25) | **`moved-to-lead`** | §3 no-progress L50; 2d L209 |
| B12 | 5-pass cap awareness | `moved-to-lead` (= A26) | **`moved-to-lead`** | §3 hard cap 3 L49; 2d L208 |
| B13 | Escalation surfacing on non-resolvable error | `moved-to-lead` (= A27) | **`moved-to-lead`** | 2e L214–230 |
| B14 | Strategy-skill consumption (`strategy-core`/`-specification`) | RF3 (dedupe + residual `moved-to-lead`) | **`dedupe` + `moved-to-lead`** | Patterns `dedupe` → `loop-discipline` (command "Honor `mochiko:loop-discipline` throughout" L9); survivors → lead: Input Assessment 0c, targeted-revision 2b L159–161, done-condition §1 |

---

## Trace (realized) — Section C: the four ADDED gates (RF6 — these never existed in HIL)

| # | New responsibility (`loop-discipline`) | Assess tag | Realized tag | Landed at |
|---|----------------------------------------|-----------|--------------|-----------|
| C1 | **Pre-declared default-FAIL done-condition + inlined `workflow-contract`** | `moved-to-lead` (NEW) | **`moved-to-lead`** | **Done-condition L15–25** ("starts in the FAILING state"; FAIL until all three hold) + **inlined contract §1 L33** ("Initial state: FAIL") |
| C2 | **Lead owns the verdict** (replaces analyst-autonomous gate) | `moved-to-lead` (NEW) | **`moved-to-lead`** | **2d "Loop control — you own the verdict" L197–201** + §2 verdict ownership L44 + Supervisor behaviors L322 |
| C3 | **Lead-owned kill-switch / budget + hard bound** | `moved-to-lead` (NEW) | **`moved-to-lead`** | **§2 contract bound + §3 kill-switch** (SPECIFY_STOP) L47–52; checked before each dispatch **2b** L144 / **2c** L178 / **2d** L210 |
| C4 | **Human ACCEPTANCE gate** on the validated spec (headline missing gate) | `moved-to-lead` (NEW) | **`moved-to-lead`** | **Phase 3 — NEW human gate G3 L234–255**; contract §4 G3 L60 |

---

## §B rehome-map landing confirmation (every B.1–B.7 row homed exactly once)

**B.1 Loop-control → lead** (doctrine `dedupe`s into `loop-discipline`):
sequencing/dispatch order → Phase 2 fixed loop (2b→2c→2d) ✓ · verdict ownership → 2d/§2/L322 ✓ · FAIL-loop driving → 2d L202–211 ✓ · no-progress exit → §3/2d ✓ · hard round cap → §3 (3)/2d ✓ · kill-switch → §3 + 2b/2c/2d ✓ · escalation surfacing → 2e ✓ · produce-then-validate ordering → 2c always after 2b (never skip the advocate gate) ✓ · read/write referee boundary (lead authors nothing, grades nothing at skill level) → intro L7 + Supervisor behaviors + §2 independence; lead mounts no producer/validator skill ✓

**B.2 State + workspace → lead** (workspace-as-state; one mechanism):
run/loop state (status, round, active feature) → 2a `round=1` / 2d / in-session ✓ · workspace identity + layout → 2a `.mochiko/specs/<feature>/` ✓ · file-path "registry" (spec.md / analyst-report.md / advocate-report.md) → fixed workspace layout (2a/2b/2c/Phase 4) ✓ · user/enriched/project-context transport → 0a capture + Phase 1 in-session + 0b/2b constitution context ✓ · clarification history → in-session (2b → next dispatch); G2 ✓ · state recovery from workspace evidence → State recovery L303–315 ✓

**B.3 Briefing / dispatch → lead** (`agent-dispatch` convention):
state briefing → 2b/2c prompts ✓ · dispatch-prompt construction / domain-agent briefing → 2b/2c + L328 ✓ · report-parsing → lead reads directly (2d) ✓ · decision-node answer persistence → 2b clarification → next dispatch ✓

**B.4 Additive survivors → single homes:**
Gap Classification taxonomy → `loop-discipline` (sibling edit), **applied** by lead at 2d L202–206 ✓ · **Input Assessment** (sparse/rich) → **0c L102–109** → invokes `analysis-iterative` (Phase 1) when sparse ✓ · **targeted-revision tactic** → **2b round>1 L159–161** ("address EVERY item … do NOT rewrite or regress sections that already passed") ✓ · spec done-condition content → contract §1 + Done-condition ✓ · briefing/recommendation judgment → 2d gap classification + routing ✓

**B.5 Prerequisite / handoff edges → contract:**
constitution-must-exist → 0b deterministic check (INV-002 label dropped) ✓ · enriched-input → producer → Phase 1 → 2b dispatch ("enriched in-session where applicable") ✓ · spec.md (producer) → advocate → 2b writes / 2c reads ✓ · template placement (seed spec.md) → 2a `cp` ✓

**B.6 ADDED gates** (= Section C above) + the in-loop **clarification sub-gate** → 2b L169 + G2 (kept as sub-gate, NOT the done-condition) ✓

**B.7 Dropped (kernel/DAG/catalog/MCP/brain-script mechanism)** → confirmed absent from the command (every A*/B* `dropped + reason` row above); **no domain capability among them** — each re-homed above ✓

---

## §E re-emitted-trace confirmation (P1 RF1–RF7 + drop table)

- **RF1** state-analyst absorbs onto this lead → entire Section B realized on the command ✓
- **RF2** verdict ownership `moved-to-lead` (2d/§2) **+ pair confirmed** (§2 table: producer agent ≠ validator agent; skills `{authoring-requirements, authoring-user-stories}` ∩ `{analysis-specifications}` = ∅, L42) ✓
- **RF3** strategy survivors `moved-to-lead` (Input Assessment 0c / targeted-revision 2b / done-condition §1) + patterns `dedupe` → `loop-discipline` (honored L9) ✓
- **RF4** context-template `absorb` → in-session + workspace; no context-handoff file (L285/L305) ✓
- **RF5** template handoff edges → spec-template (2a seed L137) · analyst-report-template (2b L163) · advocate-report-template (2c L191) ✓
- **RF6** C1–C4 the four gates → all landed (Section C) ✓
- **RF7** paths `kept-but-rebind` → `.mochiko/specs/<feature>/` + `.mochiko/memory/constitution.md`; kill-switch → `.mochiko/specs/<feature>/SPECIFY_STOP` ✓

**§E drop table (key judgment-level drops) confirmed in the artifact:**
- **A30 reversal** ("never read reports" → lead reads directly) — documented at Supervisor behaviors **L322** and the Done-condition DROPPED note **L25** ✓
- **a9 silent `carry_forward` removal** — 0b "mochiko does **not** silently recover a missing prerequisite" / "Never auto-resolve" **L93/L100** ✓
- HIL exit (analyst-evaluated `ready` + self-certified milestone, no human acceptance) — DROPPED note **L25**; replaced by default-FAIL + lead verdict + human acceptance ✓
- All kernel/DAG/catalog/MCP/brain-script mechanism (A3/A4/A5/A6/A9–A12/A15–A17/A19–A21/A28/A29′/A31/A32; B3/B5/B6/B7/B9; B10 mech) — confirmed absent ✓

---

## Kernel-free confirmation

No brain, DAG, MCP, catalog, or state-analyst anywhere in the artifact. Stated explicitly: intro L7 ("There is no kernel, no DAG, no catalog, no `state-analyst`, and no separate lead agent"); contract §1 constraint "kernel-free (no brain/DAG/MCP/catalog/state-analyst)" L33; Supervisor behaviors "Stay kernel-free" L327. All orchestration is this command + two dispatched agents (+ `analysis-iterative` / native `Explore` as needed) + the `.mochiko/specs/<feature>/` workspace + `.mochiko/memory/constitution.md`.

---

## Realized disposition

**`redesign × rewire-cluster` + convention-wiring pass — APPLIED.** Body rebuilt around the mochiko sound loop (default-FAIL done-condition, inlined `workflow-contract`, lead-owned produce→critique→revise, named human gates, state-recovery-from-workspace). Structural: the command **is** the lead the dissolving `state-analyst` (P4), the strategy skills (P9/P10), and the `context-template` (P14) re-home into; the analyst↔advocate pair is **confirmed, not constructed**; the four missing `loop-discipline` gates are **added**. New partners: **none**.

**Trace done-condition met:** every A1–A35, B1–B14, C1–C4 responsibility and every §B/§E rehome row carries a realized tag with an anchor; no responsibility is homeless; no domain capability is silently dropped (the two recorded deviations are benign, precedented, and capability-preserving).

---

**Transform version:** v1 · **Governed by:** `loop-discipline` · **Next:** Phase 4 `verify-output` (independent grade by a different agent — NOT done here).
