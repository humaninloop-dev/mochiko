# Assessment — `commands/setup.md` (P1, cluster `setup`)

Source: `human-in-loop/plugins/humaninloop/commands/setup.md`
Class: **command** → branch **IS-a-loop** (the cluster's loop owner).
Assessed: 2026-06-27 · Skill: `mochiko:assess-primitive`

---

## Step 1 — Branch

A command IS a loop. Weight: **who drives the loop · the done-condition · where validation + human gates sit (or are missing) · the iteration bound · multi-mode phasing**. Classification/trigger checks carry little weight (user-invoked command); loop-discipline checks (4 and 7) carry the most.

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | Orchestration-coupled? | **YES** — it *is* a markdown supervisor; it sequences phases, dispatches `principal-architect`, manages a context-file, and loops. It is the orchestrator. |
| 2 | Multi-responsibility / fans out? | **YES** — detection, mode routing, context-state management, dispatch, two distinct loops, three human gates, finalize, state recovery. |
| 3 | Emits non-machine-checkable artifact? | **YES (indirectly)** — it drives production of `constitution.md` / `codebase-analysis.md` / `evolution-roadmap.md`, whose correctness is model judgment, not a schema/version assert. Its own deterministic `test -f` checks prove *existence*, not *quality*. |

`gate1=y gate2=y gate3=y` → **full 7-check lens** (a command trips gates by construction).

---

## Step 3 — The lens (IS-a-loop weighting)

### Check 1 — Orchestration test (separate supervisor- from kernel-coupling)
- **Layer that orchestrates this:** it is itself a **markdown supervisor** (`commands/setup.md`) driven by the main session. **No kernel.** Body contains only `bash` + `Task(...)` + `AskUserQuestion(...)` + file ops. There is **no Python brain, no MCP `hil-dag`, no DAG, no catalog JSON** anywhere in it. (Kernel infra lives only in HIL `specify`/`plan`/`implement`.)
- **Content-coupling (body work):** light. Only `.humaninloop/memory/...` paths, the `${CLAUDE_PLUGIN_ROOT}/skills/analysis-codebase/scripts/detect-stack.sh` script path, the `humaninloop:principal-architect` subagent namespace, and the `/humaninloop:specify` next-step reference. **All are `kept-but-rebind`, none are kernel content.** There is nothing to "shed."
- **Orchestration-coupling (structural work):** this primitive *is* the orchestration. When it dissolves into a mochiko command supervisor it stays a command, but its loop control must be rebuilt to loop-discipline. Responsibilities to re-home: phase sequencing, mode routing, context-state handoff, dispatch, the clarification/edit loops, the iteration bound, the three human gates, finalize, state recovery.
- **Strategic finding (matches the dry-run prediction):** the real transformation here is **NOT "remove a kernel"** (there is none). It is **"install the independent-validation gate and the default-FAIL done-condition the original never had."** The human gates already exist; the *validation* gate does not.

### Check 2 — Role (two altitudes)
- **Skill-role:** N/A as a skill; as a command its altitude is **lead/supervisor**. It does **not** emit a reviewable artifact directly.
- **Team-role it confers:** it **is the lead**, and it confers the **producer** role on `principal-architect` via `Task` dispatch. It confers **no validator** role on anyone — that role is absent from the loop.

### Check 3 — Independence (the load-bearing finding)
- **Self-grade leak, two layers deep:** (a) `principal-architect` mounts `authoring-constitution` + `brownfield-constitution` **and** `validation-constitution` on one agent — produce + grade on the same agent; (b) worse, the **supervisor never dispatches a grading pass at all** — the loop exits on *absence of agent questions*, so in practice the constitution is **either self-graded or ungraded**. There is no independent verdict anywhere.
- **Where it hides:** in `principal-architect`'s `skills:` line (invisible from `setup.md` alone) and in the *missing* Phase between "constitution written" and "report to user."
- **Independence is violated structurally** → relational fix required → **`flag-for-reconcile`**.

### Check 4 — Verdict-sink / loop-driver
- **Consumers of output:** the user (via the human checkpoints) and downstream `/specify`.
- **What loops on FAIL:** two LLM/human-driven loops, *neither gated on an independent verdict*:
  1. Phase 2 **edit-loop** — user clicks "Edit" → corrections appended to context → loop back to Phase 1 (re-analyze).
  2. Phase 3 **clarification-loop** — *if the agent emitted `## Clarifications Needed`* → loop back to re-invoke; **else proceed**.
- **Done-condition is UNSOUND:** "no clarifications **or** max-3 iterations reached → proceed." This is (a) **LLM-controlled** ("stop when the model stops asking"), and (b) **defaults to DONE, not FAIL** — on hitting the cap it *proceeds as done* rather than escalating. Both are exactly what `loop-discipline` forbids. The `max-3` bound is real (good) but its **on-cap semantics are inverted** (declares done instead of escalating), and there is **no no-progress exit and no kill-switch**.
- This loop-driving must re-home onto the supervisor's **bounded loop**, re-spined as produce → **independent-validate** → repeat-on-FAIL, default FAIL, escalate-on-cap.

### Check 5 — Sibling / overlap
- No merge sibling (it is the unique loop owner). No trigger collision (user-invoked command).
- **One relational dependency:** it requires an **independent validator partner that does not exist** in the cluster. Creating it is `reconcile-cluster`'s job (pair/split), entangled with `principal-architect`'s skills-list (Check 3). → reconcile.

### Check 6 — Coupling audit
- **Hardcoded paths → `kept-but-rebind`:** `.humaninloop/memory/` (constitution.md, codebase-analysis.md, architect-report.md, evolution-roadmap.md, `setup-context-*.md`) → `.mochiko/memory/`; `detect-stack.sh` script path → ported `analysis-codebase` skill path; `subagent_type: humaninloop:principal-architect` → `mochiko:principal-architect`; `/humaninloop:specify` next-step → `/mochiko:specify` (note: `specify` not ported yet — rebind-by-reference stub).
- **Upstream prerequisites:** none — `setup` is the cluster entry point. Brownfield mode reads the live codebase (deterministic), not a prior artifact.
- **Determinism boundary:** `detect-stack.sh`, the source-file count, and the `test -f` existence checks are **deterministic asserts** — keep them, but they are **existence/structure checks, NOT the quality-validation gate**. Constitution authoring, codebase analysis, and roadmap gap-analysis are **model judgment** → require a real Tier-2 grounded validator (confirms the validator partner from Check 3 is *real*, not degenerate).

### Check 7 — Conventions + loop placement
- **Classification:** command → **user-invoked** (assign on port).
- **Discoverability:** register in the `mochiko` router (hinted, user-invoked).
- **Trigger phrasing / agent↔skill composition:** N/A (command, not a model-invoked skill or agent).
- **Producer↔validator pairing:** **MISSING** — primary loop gap (Check 3).
- **Sound-loop placement scorecard:**
  - Done-condition present-but-unsound: LLM-controlled + defaults-to-done → **GAP** (must become default-FAIL).
  - Independent validation: **MISSING → GAP**.
  - Human gate: **PRESENT** (Phase 0 mode-select; Phase 2 analysis checkpoint; Phase 5 cleanup) → keep + rehome.
  - Bounded iteration: **PARTIAL** — `max-3` exists but on-cap proceeds-as-done; no no-progress exit; no kill-switch → **GAP**.

---

## Step 4 — Disposition

**`redesign` × `flag-for-reconcile`**

- **Body = `redesign`.** Not to shed a kernel (there is none) but because the **load-bearing element of a command — its loop control spine — is unsound for mochiko**: the done-condition is LLM-controlled and default-DONE, there is no independent-validation gate, and on-cap declares done instead of escalating. Rebuilding the control flow is not a localized edit. **Carry the responsibilities, not the broken mechanism:** the phase *content* (Phase-0 detection, mode routing, the architect dispatch prompts, the per-mode report formats, the Phase-5 finalize reporting) is reusable `port-with-edits`-grade material and should be preserved; the *loop discipline* (done-condition, validation gate, escalation) is what gets redesigned.
- **Structural = `flag-for-reconcile`.** The command stays a command (it is the lead — no merge/promote of the body itself), but its sound form **depends on a sibling that does not yet exist**: an independent constitution-validator partner, plus the resolution of `principal-architect`'s self-grade skills-list. Spinning out a validator (pair/split) and rehoming the loop/gate orchestration is `reconcile-cluster`'s job with full cluster context — **do not guess it here.** Body treatment is proposed; structural move is flagged.

**Human-gate note (per contract §4):** `redesign` is a gated disposition — the human gate must accept it before transform applies.

---

## Step 5 — Responsibility trace (complete — every responsibility tagged)

| # | Responsibility (currently in `setup.md`) | Tag |
|---|---|---|
| 1 | Ensure workspace dir exists (`mkdir -p .humaninloop/memory`) | `kept-but-rebind` → `.mochiko/memory` |
| 2 | Brownfield detection — run `detect-stack.sh` + count source files | `kept-but-rebind` (script path); deterministic step on the supervisor |
| 3 | Constitution-mode determination (create vs amend via reading existing `constitution.md`) | `kept-but-rebind` |
| 4 | Brownfield-vs-greenfield heuristic (files > 5 AND framework detected) | `kept` (supervisor logic) |
| 5 | Phase-0 **mode-selection human gate** (AskUserQuestion: brownfield/greenfield/amend) | `moved-to-lead` (supervisor's named human gate) |
| 6 | Mode routing + phase sequencing (route to Phase 1 vs Phase 3; brownfield/greenfield/amend phasing) | `moved-to-lead` (sequencing/dispatch order) |
| 7 | Context-file state artifact (`$CONTEXT_FILE` frontmatter + supervisor instructions + clarification log) — the handoff medium | `kept-but-rebind` → `.mochiko/`; remains the supervisor's Task-handoff mechanism |
| 8 | Mid-loop instruction injection ("modifies context", appends to clarification log) | `moved-to-lead` |
| 9 | Dispatch `principal-architect` (producer) for analysis + constitution + roadmap phases | `moved-to-lead` (dispatch) + `kept-but-rebind` (`humaninloop:` → `mochiko:` subagent namespace) |
| 10 | Verify output exists (`test -f codebase-analysis.md` / `architect-report.md` / `constitution.md` / `evolution-roadmap.md`) | `kept-but-rebind` — deterministic **existence** assert; explicitly **not** the quality gate |
| 11 | Phase-2 **analysis-checkpoint human gate** (AskUserQuestion: confirm/edit/reject) | `moved-to-lead` (supervisor's named human gate) |
| 12 | Phase-2 edit-loop (corrections → append → loop back to Phase 1) | `moved-to-lead` (loop-driving), **rebuilt** under bounded-loop discipline (escalate-don't-die) |
| 13 | Phase-2 reject handling (offer greenfield fallback / abort) | `moved-to-lead` |
| 14 | Constitution-generation dispatch + per-mode supervisor instructions (brownfield/greenfield/amend) | `moved-to-lead` (dispatch) |
| 15 | Authoring **requirement prose** embedded in the dispatch prompt ("CRITICAL — no placeholders", essential-floor list, populate-from-analysis, V–VII architectural principles) | `folded-into-skill` (belongs in `authoring-constitution` / `brownfield-constitution`, not duplicated in the supervisor prompt) → `dedupe` |
| 16 | Per-phase **report-format specs** (architect-report.md structure) | `folded-into-skill` (producer's output contract) |
| 17 | Phase-3 **clarification-loop** (agent asks user → answers appended → re-invoke) | `moved-to-lead` — kept as a legitimate human-**clarification** gate, but it **must not be the done-condition** |
| 18 | **Done-condition** = "no clarifications OR max-3 reached → proceed" | `dropped + reason`: LLM-controlled, default-DONE exit violates `loop-discipline`; **replaced** by a default-FAIL done-condition gated on independent validation (lead must accept the drop) |
| 19 | `max-3-iterations` bound | `kept-but-rebind` → `moved-to-lead`, **on-cap semantics inverted**: must **escalate (FAIL)**, not proceed-as-done; add no-progress exit + kill-switch |
| 20 | **Independent quality validation of the constitution** | **MISSING responsibility** the sound loop must hold → `flag-for-reconcile` → `moved-to-validator` (new constitution-validator agent + `validation-constitution` skill; created in reconcile) |
| 21 | Phase-4 **evolution-roadmap** orchestration (gap-analysis dispatch) | `moved-to-other-cluster` — depends on `authoring-roadmap` + `evolution-roadmap-template.md`, **cross-cutting, not ported this run**; rebind-by-reference stub |
| 22 | CLAUDE.md sync (report "CLAUDE.md Sync Status"; architect's `syncing-claude-md` skill) | `moved-to-other-cluster` — `syncing-claude-md` cross-cutting, not ported this run |
| 23 | Phase-5 finalize / user report (per-mode completion summary, suggested commit, next steps) | `moved-to-lead` + `kept-but-rebind` (`/humaninloop:specify` → `/mochiko:specify` reference stub) |
| 24 | Phase-5 **cleanup human gate** (AskUserQuestion: delete context file?) | `moved-to-lead` (minor human gate) |
| 25 | State recovery (resume-point detection from context-file `phase`) | `moved-to-lead` + `kept-but-rebind`; supervisor may retain or simplify — not silently dropped |
| 26 | Supervisor behaviors / mode-awareness ("owns the loop", tracks brownfield vs greenfield) | `moved-to-lead` |

No untagged responsibility remains → trace done-condition satisfied.

---

## Reconcile flags (for `reconcile-cluster`)

- **FLAG-1 — independence / missing validation gate (relational, primary).**
  Signal: *emits a reviewable artifact (constitution) with no independent validator, AND a self-grade leak* — `principal-architect` mounts `validation-constitution` alongside the authoring skills.
  Expected resolution: **pair → split** — spin out an independent **constitution-validator** agent whose load-bearing tool is `validation-constitution`; **remove `validation-constitution` from `principal-architect`**; the supervisor dispatches the validator as the loop's grading gate (Tier-2 grounded — Check 6 confirms it is real, not degenerate). Partner primitives: `principal-architect` (P2), `validation-constitution` (P5).

- **FLAG-2 — rehome-orchestration: unsound done-condition + loop semantics (orchestration).**
  Replace the LLM-controlled "no-questions/max-3-then-proceed" exit with a **default-FAIL** done-condition gated on FLAG-1's independent verdict; make **on-cap escalate**, not proceed; add a **no-progress exit**. Re-homes onto the supervisor's bounded loop (per `loop-discipline`).

- **FLAG-3 — cross-cluster boundaries (orchestration / rebind).**
  Phase-4 roadmap orchestration (`authoring-roadmap`, `evolution-roadmap-template.md`) and CLAUDE.md sync (`syncing-claude-md`) are cross-cutting — **rebind by reference only this run**; the responsibilities land when those clusters port (`moved-to-other-cluster`). Confirm reconcile leaves a clean reference stub, not a dangling dispatch.

---

## Summary block

```
ASSESSMENT: setup (commands/setup.md)
Class:        command → branch IS-a-loop (cluster loop owner)
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  redesign × flag-for-reconcile
              (body redesign = rebuild the loop control spine — default-FAIL done-condition,
               independent-validation gate, escalate-on-cap; CARRY phase content as port-with-edits.
               structural flagged: spin out a validator partner + rehome orchestration — reconcile decides.)
Key finding:  NO kernel to shed (markdown supervisor only). Real work = install the independent-
              validation gate + default-FAIL done-condition the original never had. Human gates already exist.
Trace:        26 responsibilities, all tagged (kept / kept-but-rebind / moved-to-lead /
              folded-into-skill / dedupe / moved-to-validator(flagged) / moved-to-other-cluster /
              1 dropped[old LLM-controlled done-condition, reason given, lead must accept]).
Reconcile flags:
  - FLAG-1 independence/missing-validator → pair→split (constitution-validator; partners P2, P5)
  - FLAG-2 rehome unsound done-condition + loop semantics → supervisor bounded loop
  - FLAG-3 cross-cluster roadmap + claude-md-sync → moved-to-other-cluster (rebind-by-ref only)
```
