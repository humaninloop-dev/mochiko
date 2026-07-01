# Verify — P1 `implement` command (independent grade)

**Run:** `/mochiko:transform-cluster implement` · Phase-4 `verify-output` · **Artifact:** `plugins/mochiko/commands/implement.md` (80 ln)
**Validator:** `mochiko:validator` (independent — did NOT author the artifact) · **Skill:** `verify-output` · **Date:** 2026-07-01
**Graded against:** `contract.md` §1 done-condition + the two LOCKED intake decisions · the 43-responsibility trace in `assess-implement-command.md` · the P7 dissolution trace in `assess-strategy-implementation.md`

```
VERIFY: implement command (P1) — redesign × absorb-into-lead → thin sequential sound-loop
Evidence read (this run):
  plugins/mochiko/commands/implement.md                       (THE artifact — every line)
  .mochiko/transform/implement/transform-implement-command.md (producer realized-trace — audited, not trusted)
  .mochiko/transform/implement/contract.md §1                 (done-condition + LOCKED decisions)
  .mochiko/transform/implement/assess-implement-command.md    (43-responsibility source trace)
  .mochiko/transform/implement/assess-strategy-implementation.md (P7 dissolution / survivors)
  plugins/mochiko/commands/tasks.md + plan.md                 (thin-altitude precedent)
  plugins/mochiko/templates/workflow-contract.md              (the doctrinal contract that must be filled, not inlined)
  plugins/mochiko/skills/mochiko/SKILL.md                     (router — discoverability check)

VERDICT: PASS
```

---

## Deterministic pre-asserts (ground truth — run first, recorded as evidence)

| Scan (on the artifact) | Expected | Result |
|---|---|---|
| Kernel tokens: `hil-dag` `DAG` `catalog` `INV-` `State-Analyst` `.workflow/` `pass-lifecycle` `MCP` `check-prerequisites` `carry_forward` `dispatch_mode` `context.md` `Context-Protection` | 0 each | **0 each ✓** |
| Transliteration: `Task(` `AskUserQuestion(` `pipeline(` | 0 each | **0 each ✓** |
| `parallel(` | only in sanctioned deferral note | **1 hit — line 35 deferral note only ✓** |
| `grep -i strategy` | 0 | **0 ✓** (no strategy skill referenced) |
| Inlined-contract headers: `DEFAULTS TO FAIL` `Producer ↔ Validator` `Independence check` `Initial state` `Contract version` `Governed by` | 0 each | **0 each ✓** |
| Restated doctrine: `trustworthiness` `Tier-1/2` `rationaliz` `exhaustion` `Supervisor behaviors` `tamper` `gap-type` `self-report is unreliable` | 0 each | **0 each ✓** |
| Reference presence: `loop-discipline` / `agent-dispatch` / `workflow-contract` | ≥1 each | **5 / 3 / 1 ✓** |
| `^#{1,6}` section headers = inlined contract sections? | none | **none — only Contract-parameters + Phase 0–4 + State recovery ✓** |

The one `Bounded iteration` hit is line 10 (`honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates)`) — a **parenthetical reference list** immediately followed by "Those rules are not restated here." It is not a section header (0 header matches) and mirrors `tasks.md`/`plan.md` line 10 verbatim in shape.

---

## Part A — Conformance (each confirmed against the file)

| # | Convention | Verdict | Evidence quoted from the artifact |
|---|-----------|---------|-----------------------------------|
| 1 | Classification | **PASS** | Frontmatter line 3: `disable-model-invocation: true` → user-invoked command. |
| 2 | Discoverability | **PASS** | Registered in `mochiko` router `SKILL.md` line 108: "`/mochiko:implement` \| you want to turn an accepted `tasks.md` into working, verified code — cycle-by-cycle (foundation → feature) via the staff-engineer→qa-engineer producer→verifier loop, with a confidence-based per-cycle gate and a named final-acceptance gate" + cluster section line 93 + agent rows 119–120. (Producer's trace claimed this "DEFERRED to Wave-2"; reality exceeds the claim — it is already wired.) |
| 3 | Reliable model-invocation | **PASS (n/a)** | User-invoked command — no graded model-invocation triggers; `description` is a user-facing summary. |
| 4 | Agent↔skill decoupling | **PASS (n/a to command body)** | The deny-list scan targets *personas & skills*, not callers. `commands/*.md` may name agents and own workflow knowledge; the lead legitimately names `mochiko:staff-engineer` / `mochiko:qa-engineer` and dispatches. (Persona decoupling of P2/P3 is graded in their own verifies.) |
| 5 | Producer↔validator pairing | **PASS** | Line 8: implemented by `staff-engineer` (`executing-tdd-cycle` + `brownfield-integration`), "then **independently verified** by a *different* agent, `mochiko:qa-engineer` (`testing-end-user`)… Never mount a verification skill on staff." Line 19: "Disjoint agents and skills — the verification skill is **never** mounted on staff." Independence is structural (disjoint agents + disjoint skill sets), never self-declared. |
| 6 | Sound-loop placement | **PASS** | Filled `workflow-contract` referenced as an artifact (line 16: "Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/implement-contract.md`… then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body."). Done-condition default-FAIL (line 18 "starts FAILing"), independent validation (line 8/19), bounded iteration (line 20), named human gate (line 21 G5 + confidence gate). |
| 7 | Kernel-free | **PASS** | All 13 kernel/DAG/MCP/catalog/carrier tokens grep to 0 (table above). Only native-Workflow `parallel()` appears, inside the deferral note (line 35), naming a *deferred* capability — not a kernel. |
| 8 | Altitude / single-source | **PASS** | See dedicated analysis below. |

---

## Part A item 8 — Command altitude (the critical one) — PASS

**Grep-floor (deterministic) — clean:**
- **No inlined filled contract.** The body says "fill the artifact — don't inline it" (line 14 header + line 16 body). The doctrinal section headers (`DEFAULTS TO FAIL`, `Producer ↔ Validator`, `Independence check`, `Initial state`, `Contract version`, `Governed by`) — which the template `workflow-contract.md` actually carries (§1–§4 + trustworthiness tier + tamper-proofing) — are **absent** from the command (0 each). The command supplies only **parameters** (Done-condition end-state · Team · Bounds · Gates), the per-workflow values the skill explicitly permits.
- **No transliterated payloads.** `Task(` / `AskUserQuestion(` / `pipeline(` = 0. Dispatch is prose ("Dispatch `mochiko:staff-engineer`…", lines 39/40/47).
- **No restated doctrine.** `trustworthiness` / `Tier` / `rationaliz` / `exhaustion` / `Supervisor behaviors` / `tamper` / `gap-type` / `self-report is unreliable` = 0 each. The four requirements are **named once** as a reference list (line 10) then explicitly deferred ("Those rules are not restated here"); `loop-discipline` is cited 5×.

**Keystone-ceiling (judgment) — every line is workflow-specific or a reference:**
- Done-condition (1)(2)(3)(4) at line 18 = the *implement* end-state (every cycle complete + `cycle-report.md`; qa passes each cycle + final; you Read reports + confirm; G5 cleared) — a workflow-specific done-condition, which the skill rules "craft, not duplication." Matches `tasks`/`plan` line 18 shape.
- Confidence-gate classification is **referenced**, not copied: line 41 "qa classifies each verification (the CLI / GUI / SUBJECTIVE classification procedure lives in `testing-end-user`)". The command states only the *placement* (CLI 100%-pass → auto-approve; GUI/subjective/any-fail → checkpoint).
- Tamper-proofing appears as the **act** without the doctrine: line 18(3)/line 41 "you Read the cycle-reports + verification reports" — never the "a PASS is invalid unless…" doctrine (grep `tamper`=0).
- HIL-contrast note (line 23) is a workflow-specific rationale (present in `tasks`/`plan` line 23), not `loop-discipline` doctrine.

**"What you own" footer (line 80) — scrutinized clause-by-clause — PASS.** It states *workflow-specific ownership*, not loop doctrine, and closes with a reference:
- cycle sequence (foundation before feature; current = first unchecked) — true only of this workflow ✓
- each cycle's loop (round counter, no-progress check, retry cap, kill-switch, escalation) — lead's operational ownership; identical pattern to `tasks.md` footer line 77 ✓
- execute→verify pairing (every staff cycle → qa, same round, never skipped) — workflow-specific ✓
- the verdict specifics (cycle-checkpoint = criteria-met + gates pass; final-validation = all `[x]` + gates + traceability + constitution alignment; qa's status is input) — workflow-specific gate-eval params ✓
- fix-pass routing + max-3; the two implementation gates (confidence gate + G5) + G1/G3/G4; tasks-complete entry gate; scaffolding; dispatch-wrote-its-files check; never mount verification on staff — all workflow-specific ✓
- closes "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool… **Full rules: `mochiko:loop-discipline`**." — lead invariants + single-source reference (matches `tasks`/`plan` footer verbatim) ✓

No "Supervisor behaviors" doctrine section; the four requirements are not re-listed; no validator tiers, no anti-rationalization list, no gap-routing enumeration. Doctrine is referenced, never leaked.

---

## Part B — Trace audit (against the 43-responsibility source trace)

**Completeness — PASS (43/43 accounted; all `moved-to-lead` verified present in the artifact):**

| Tag | Responsibility | Artifact anchor (confirmed) |
|---|---|---|
| L0 | Lead role + clearing-verdict ownership; qa status = input | line 8; footer line 80 |
| L1 | `$ARGUMENTS` + `@`-drop recovery | line 12 + Phase 0 step 1 (G1) |
| L2 | Done-condition end-state params | line 18 (1)(2)(3); Phase 2 step 2 |
| L3 | **(f)** entry gate tasks-workflow-complete | Phase 0 step 2 (line 28) |
| L4 | Load design inputs | Phase 0 step 4 (line 30) |
| L5 | **(g)** project scaffolding | Phase 0 step 5 (line 31) |
| L6 | **(a)** sequential sequencing, foundation→feature | Phase 1 "Sequencing" (line 35) |
| L7 | **(b)** execute→verify pairing, same round, never skipped | Phase 1 step 2 (line 40) + footer |
| L8 | **(c)** targeted retry, max 3/cycle | Bounds (line 20) + Phase 1 step 3 |
| L9 | **(d)** fix-pass, unconstrained by cycle boundaries, max 3 | Bounds (line 20) + Phase 2 step 3 (line 49) |
| L10 | **(e)** convergence-stall 2+ rounds → surface | Bounds (line 20); Phase 1 step 3; Phase 2 step 3 |
| L11 | Overall cap / no-progress + kill-switch `IMPLEMENT_STOP` | Bounds (line 20, "You count every round") |
| L12 | Mid-loop clarification gate (G3, "Research this"→Explore) | line 51 |
| L13 | Confidence-based per-cycle gate placement | Gates (line 21) + Phase 1 step 3 (line 41) |
| L14 | Named final-acceptance gate (NEW) | Gates (line 21) + Phase 3 (line 53, G5) |
| L15 | Completion / finalize summary | Phase 4 (lines 57–59) |
| L16 | Dispatch-via-Task; do-not-modify-git | footer line 80 |
| L17 | State-Analyst implement-slice reclaimed (direct Read + gate-eval params) | line 18(3); line 41; footer verdict clause |

- **dedupe (DD1–DD6)** → referenced, never restated: sound-loop para (line 10), "confirm it against `mochiko:loop-discipline`" (line 16), "apply the bounds" (line 20), "route … per `loop-discipline`" (line 41/51), "briefed per `agent-dispatch`" (lines 10/30/39/40/47), "Full rules: `mochiko:loop-discipline`" (line 80). No doctrine restated (grep-confirmed).
- **kept-but-rebind (R1–R3):** input paths → `.mochiko/specs/<feature>/` (line 30); entry evidence → `.mochiko/specs/<feature>/tasks.md` (line 28); scaffolding project-relative (line 31).
- **absorbed-into-lead (A1):** no carried `context.md`; realized by the State-recovery table (lines 61–76) resuming purely from workspace evidence.
- **moved-to-sibling-skill (S1):** `**TEST:**` CLI/GUI/SUBJECTIVE classification → `testing-end-user`, *consumed* at line 41. Landing verified: `testing-end-user` is registered in the router (cluster section line 93).
- **dropped (D1–D14):** all 14 brain-plumbing tokens grep to 0 in the artifact — correctly absent.

**Justified drops — PASS.** All 14 drops are kernel/DAG/MCP/carrier plumbing; reasons enumerated in the assess trace bucket (i) and, per `contract.md` §4(a) + the reconcile gated bundle, accepted at the human gate. Each token grep-confirmed absent.

**Landing integrity — PASS.** `moved-to-sibling-skill` → `testing-end-user` (router-confirmed present); `staff-engineer`/`qa-engineer` receiving agents present (router rows 119–120). No dangling destination.

**No capability loss — PASS.** Every a–g orchestration is present (sequencing L6, pairing L7, targeted-retry L8, fix-pass L9, convergence-stall L10, entry-gate L3, scaffolding L5); the confidence gate + G5 + G1/G3/G4 are all present. A user who could implement cycle-by-cycle with independent verification before can do so after.

**P7 (`strategy-implementation`) survivors — PASS, no double-home.** The 6 P7 `moved-to-lead` survivors (L-a…L-f) are the SAME responsibilities as P1's twins (L6/L7/L8/L9/L2 + the all-`[x]`-before-final ordering guard, realized at line 45 "Reachable when every cycle has cleared" + done-condition ordering). Each landed **once** on the lead. P7's DD1–DD4 doctrine deduped into `loop-discipline` and is **not** restated in the command (grep `strategy`=0; no doctrine tokens). The standalone strategy skill does not exist and is not referenced.

---

## Two LOCKED intake decisions — realized (verified with quotes)

1. **Sequential-only — PASS.** Line 35: "**Sequential-only** — one cycle at a time; parallel cycle execution (native-Workflow `parallel()`) is a deliberate `deliberate-shortcut-ledger` deferral pending dogfooding, **not** a capability drop." No `pipeline(`, no artifact-DAG (grep 0). The deferral note is present exactly as required.
2. **Confidence-based gate + named final-acceptance — PASS.** Line 21: "the **confidence gate** (per cycle: deterministic CLI verifications that 100% pass → auto-approve; GUI / subjective / any-failure → human checkpoint) · **G5** the named final-acceptance gate before 'done.'" Reinforced at Phase 1 step 3 (line 41) and Phase 3 (line 53, G5).

---

## Note (not a defect)

The producer's realized-trace (`transform-implement-command.md`) lists router registration as "DEFERRED to Wave-2." Independent check shows the `mochiko` router `SKILL.md` **already** carries the `/mochiko:implement` entry hint (line 108), the Implement-cluster section (line 93), and the staff/qa agent rows (119–120). The artifact is therefore *better*-wired than its own trace claimed; Part A item 2 passes on evidence, not on deferral. This does not affect the verdict.

---

## VERDICT: **PASS**

**Failing items:** none.
**Required fixes:** none. All 8 conformance items PASS; the full 43-responsibility trace is present-or-correctly-absent with zero silent drops; the a–g orchestration + P7 survivors are all carried on the thin lead; both LOCKED intake decisions are realized; the artifact is kernel-free and holds altitude (references doctrine, fills the contract as an artifact, never restates the four rules / tiers / gap-routing / tamper-proofing). Line count 80, between `tasks` (77) and `plan` (82) — dense, not padded.

**Verify version:** v1 · **Governed by:** `verify-output` · graded independently by `mochiko:validator`.
