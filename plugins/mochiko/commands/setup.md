---
description: Create or update the project constitution via an independent producer→validator loop with a human acceptance gate — brownfield-aware (greenfield | brownfield | amend).
---

# Setup — Project Constitution

You are the **lead / supervisor** for establishing or updating the project constitution. There is no kernel and no separate lead agent — **you, executing this command, are the supervisor**. You own the loop, the done-condition, the verdict, and the human gates. The constitution itself is **authored** by a dispatched producer (`mochiko:principal-architect`) and **graded** by a different, independent validator (`mochiko:validator`). You never let those two collapse into one agent — independence is the point.

This workflow is a mochiko **sound loop**. Honor `mochiko:loop-discipline` throughout: a pre-declared done-condition that **defaults to FAIL**, independent validation (the author never grades its own constitution), bounded iteration, and named human gates. The filled `workflow-contract` for this loop is inlined below; it is the inspectable proof that all four requirements are met.

**Argument:** `$ARGUMENTS` = optional setup request (e.g. "set up project governance", "amend the testing principle"). Empty is fine — Phase 0 detects mode.

---

## Done-condition (pre-declared, DEFAULT FAIL)

The constitution **starts in the FAILING state** and flips to done **only** when **all** of these hold:

- `mochiko:validator` returns **PASS** on `.mochiko/memory/constitution.md`, graded from the artifact file itself (not from the author's report), **AND**
- the **human acceptance gate** (Phase 4) has cleared the validated constitution, **AND**
- in brownfield mode, the Phase-2 **analysis checkpoint** was confirmed by the human.

Until all three hold, the run is **FAIL**. Running out of rounds is **FAIL-and-escalate**, never "done."

> **DROPPED (replaced):** HIL's exit — *"no clarifications **OR** max-3 iterations reached → proceed"* — is removed. It was LLM-controlled (stop when the model stops asking) and **defaulted to DONE** (on-cap it declared done). Both violate `loop-discipline` requirement 1. The clarification exchange survives only as a **human-clarification gate inside the loop** (below); it is **not** the done-condition. Acceptance of this drop was recorded at the Phase-2 reconcile human gate.

---

## Workflow contract (this loop)

<!-- Filled instance of templates/workflow-contract.md. No open brackets = sound per loop-discipline. -->

**1. Done-condition (DEFAULTS TO FAIL)** — stated above. Measurable end state: validator PASS on `.mochiko/memory/constitution.md` + human acceptance cleared (+ brownfield analysis confirmed). Stated check: the validator **Reads** the constitution file and confirms the `validation-constitution` checklist. Constraint: no Essential-Floor principle silently dropped; no `[PLACEHOLDER]` tokens ship; kernel-free (no brain/DAG/MCP/catalog). Initial state: **FAIL**.

**2. Producer ↔ Validator (independence on two axes)**

| Role | Agent | Skill(s) | Notes |
|------|-------|----------|-------|
| **Producer** | `mochiko:principal-architect` | `authoring-constitution`, `analysis-codebase` | authors `constitution.md` / `codebase-analysis.md`; **never grades** |
| **Validator** | `mochiko:validator` | `validation-constitution` | grades from the constitution file itself, never the author's say-so |

- **Independence check:** producer agent ≠ validator agent ✓ **AND** producer skills `{authoring-constitution, analysis-codebase}` ∩ validator skill `{validation-constitution}` = ∅ ✓.
- **Validator trustworthiness tier:** **Tier 2 — separate-context grounded LLM** (highest the artifact allows; constitution quality is model judgment, not a schema/version equality). Tier-1 deterministic sub-checks are layered in where they exist: the placeholder scan (`grep` for `[PLACEHOLDER]`/`[...]` tokens), three-part-structure presence, and — in brownfield mode — cross-check of named tools/versions against `.mochiko/memory/codebase-analysis.md`.
- **Tamper-proofing:** no PASS unless the validator **Read `.mochiko/memory/constitution.md` this run** and cites evidence quoted from it. "Looks like it passes" is an automatic FAIL. Validator defaults to FAIL.

**3. Bounded iteration**

- **Hard round cap:** **3** produce↔validate rounds, counted by **you** (the lead), not by the model feeling done.
- **No-progress exit:** stop if a round's validator fix-list is **unchanged** from the previous round (the loop is stuck).
- **Budget / kill-switch:** out-of-band halt via sentinel file `.mochiko/memory/SETUP_STOP` — checked **before each producer and validator dispatch**. If present, halt and escalate.
- **On hitting a guard:** **escalate** to the human gate (`AskUserQuestion`) with the validator's last fix list and the failure context. **Never** mark the run done on cap / no-progress / kill-switch exhaustion.

**4. Human gate** — *presence non-negotiable; this loop names four:*

| # | Gate | Where it fires | What the human decides |
|---|------|----------------|------------------------|
| G1 | **Mode-select** | Phase 0 | brownfield / greenfield / amend |
| G2 | **Analysis checkpoint** | Phase 2 (brownfield only) | confirm / edit / reject the codebase analysis |
| G3 | **Constitution acceptance** (NEW) | Phase 4 | accept / amend / reject the **validator-PASSed** constitution |
| G4 | **Cleanup** | Phase 5 | retain or clean the `.mochiko/memory/` workspace |
| — | **Escalation** | any Phase-3 guard trip | resolve cap / no-progress / kill-switch exhaustion |

**Contract version:** v1 · **Governed by:** `loop-discipline`

---

## Phase 0 — Detect & mode-select  *(human gate G1)*

1. **Ensure the workspace exists**
   ```bash
   mkdir -p .mochiko/memory
   ```

2. **Run stack detection** (deterministic — an *input*, not the quality gate)
   ```bash
   bash ${CLAUDE_PLUGIN_ROOT}/skills/analysis-codebase/scripts/detect-stack.sh .
   ```

3. **Count source files** (heuristic for brownfield detection)
   ```bash
   find . -type f \( -name "*.ts" -o -name "*.js" -o -name "*.py" -o -name "*.go" -o -name "*.java" -o -name "*.rb" -o -name "*.rs" \) \
     -not -path "*/node_modules/*" -not -path "*/.git/*" -not -path "*/vendor/*" -not -path "*/__pycache__/*" | wc -l
   ```

4. **Determine constitution mode** (create vs amend)
   ```bash
   cat .mochiko/memory/constitution.md 2>/dev/null
   ```
   - If it exists → `constitution_mode: amend`
   - If not → `constitution_mode: create`

5. **Brownfield heuristic:** source-file count > 5 **AND** detect-stack found a framework/ORM → suggest **brownfield**; otherwise default **greenfield**.

6. **Present the detection result and ask the user** (G1):
   ```
   AskUserQuestion(
     questions: [{
       question: "Found [N] source files with [framework/language] detected.\n\nBrownfield analysis will inventory existing patterns and entities, assess the Essential Floor (Security, Testing, Error Handling, Observability), and write codebase-analysis.md alongside constitution.md.\n\nHow would you like to proceed?",
       header: "Setup Mode",
       options: [
         {label: "Brownfield - Full analysis", description: "Analyze the codebase, then author the constitution from real patterns (recommended for existing code)"},
         {label: "Greenfield - Constitution only", description: "Author the constitution with opinionated defaults; skip analysis"},
         {label: "Amend existing", description: "Update the existing constitution without re-analysis"}
       ],
       multiSelect: false
     }]
   )
   ```

7. **Route:** Brownfield → Phase 1 · Greenfield → Phase 3 (greenfield) · Amend → Phase 3 (amend).

> State carried so far lives **in-session** (mode, detection output, file count) plus the `.mochiko/memory/` workspace. There is **no separate context-handoff file** — that mechanism (HIL's `setup-context-*.md`) was absorbed into this lead.

---

## Phase 1 — Brownfield analysis  *(brownfield mode only)*

Dispatch the **producer** to author the codebase analysis (the setup-brownfield slice of `analysis-codebase`):

```
Task(
  subagent_type: "mochiko:principal-architect",
  description: "Analyze existing codebase",
  prompt: "Run analysis-codebase (mode: setup-brownfield) on the project in the current working directory.
           Inputs from Phase 0 (passed in-session below): the detect-stack output + the source-file count.
           Inventory existing patterns and entities; assess the Essential Floor — Security, Testing,
           Error Handling, Observability — as present / partial / absent. Use
           templates/codebase-analysis-template.md for structure.
           WRITE the analysis to .mochiko/memory/codebase-analysis.md.
           RETURN (in your reply, not a file) a short report: summary, Essential-Floor status table,
           entities found, architecture pattern, and any clarifications needed."
)
```

**Verify existence** (deterministic — confirms the file exists, NOT that it is correct; correctness is the human checkpoint in Phase 2):
```bash
test -f .mochiko/memory/codebase-analysis.md && echo "analysis present"
```

---

## Phase 2 — Analysis checkpoint  *(brownfield mode only · human gate G2)*

Present the producer's returned analysis summary and ask the user (G2):

```
AskUserQuestion(
  questions: [{
    question: "[Summary + Essential-Floor table + architecture from the producer's report]\n\nIs this analysis accurate?",
    header: "Analysis Review",
    options: [
      {label: "Confirm - Proceed to constitution", description: "Analysis is accurate, continue"},
      {label: "Edit - Provide corrections", description: "I'll add corrections, re-run analysis"},
      {label: "Reject - Start over", description: "Analysis is wrong; fall back to greenfield or abort"}
    ],
    multiSelect: false
  }]
)
```

**Route:**
- **Confirm** → Phase 3 (brownfield).
- **Edit** → collect corrections, re-dispatch Phase 1 with the corrections appended in-session. This re-analysis is **bounded by the same round discipline** as Phase 3 (max 3 analysis re-runs; escalate, don't spin). It is a human-correction gate, **not** a done-condition.
- **Reject** → ask whether to continue in greenfield mode (→ Phase 3 greenfield) or abort the run.

> `codebase-analysis.md` is an **intermediate input**, not the cluster deliverable. Its deliberate gate is this **human checkpoint** plus the deterministic `detect-stack.sh` baseline — there is no separate analysis-validator agent (that would over-engineer an intermediate artifact). The **constitution** is the deliverable, and it gets the independent validator in Phase 3.

---

## Phase 3 — Constitution loop  *(all modes · the sound loop)*

This is the produce → validate → repeat-on-FAIL core. **You own the round counter.** Initialize `round = 0`; the constitution is **FAIL** until proven otherwise.

### 3a. Produce — dispatch the author

Before dispatch, check the kill-switch: if `.mochiko/memory/SETUP_STOP` exists, **escalate** (do not dispatch).

```
Task(
  subagent_type: "mochiko:principal-architect",
  description: "Author constitution (round N)",
  prompt: "Run authoring-constitution to write .mochiko/memory/constitution.md.
           MODE = <greenfield | brownfield | amend>:
             - brownfield: READ .mochiko/memory/codebase-analysis.md; use ACTUAL tools/versions/commands
               (e.g. real `pytest --cov`, named scanners), NEVER [PLACEHOLDER] tokens; include the four
               Essential-Floor principles + an Emergent Ceiling codifying existing good patterns; add
               project_type: brownfield.
             - greenfield: opinionated defaults — Essential Floor (I-IV) + architectural principles
               (V-VII: Hexagonal Architecture, Single Responsibility, Dependency Discipline); concrete
               commands for the detected stack; no placeholders; add project_type: greenfield.
             - amend: READ the existing constitution; preserve principles unless explicitly changing;
               bump the version per semantic versioning.
           Use templates/constitution-template.md. Every principle: Statement, Enforcement, Testability,
           Rationale; RFC-2119 keywords (MUST/SHOULD/MAY).
           ROUND > 1: address EVERY item in the validator fix list below and do not regress passing
           items. FIX LIST: <paste the validator's issues-requiring-fix from the prior round, or 'none — first round'>.
           WRITE to .mochiko/memory/constitution.md. RETURN (in your reply) a short report: what you
           created/changed, assumptions made, and any clarifications needed. Do NOT grade your own output."
)
```

**Clarification sub-gate (human, inside the loop — NOT the done-condition):** if the producer's report contains questions it cannot resolve, present them to the user, collect answers, and feed them into the next produce dispatch as added context. This exchange **never** ends the loop on its own; the validator gate (3b) still must PASS.

**Verify existence** (deterministic, not quality):
```bash
test -f .mochiko/memory/constitution.md && echo "constitution present"
```

### 3b. Validate — dispatch the independent grader

Check the kill-switch again (`.mochiko/memory/SETUP_STOP`). Then dispatch the **validator** — **never** the producer:

```
Task(
  subagent_type: "mochiko:validator",
  description: "Validate constitution (round N)",
  prompt: "Run validation-constitution on .mochiko/memory/constitution.md. READ the artifact file
           itself — never the author's report. Check: three-part structure (Statement / Enforcement /
           Testability / Rationale on every principle), anti-pattern scan, placeholder scan (no
           [PLACEHOLDER] tokens), quantification of thresholds, and the semantic version-bump call.
           In brownfield mode, cross-check named tools/versions against .mochiko/memory/codebase-analysis.md.
           RETURN a binary PASS/FAIL + issues-requiring-fix (the fix list) + the version-bump call.
           Default FAIL; no PASS without evidence quoted from the artifact."
)
```

### 3c. Loop control (bounded iteration — you own this)

- **PASS** → record the verdict and the version bump; proceed to Phase 4.
- **FAIL** → increment `round`. Then:
  - If `round >= 3` (**round cap**) → **escalate** (3d). Do not loop again.
  - If this round's fix list is **unchanged** from the previous round (**no-progress**) → **escalate** (3d).
  - If `.mochiko/memory/SETUP_STOP` exists (**kill-switch**) → **escalate** (3d).
  - Otherwise → loop back to **3a**, passing the fix list to the author.

### 3d. Escalate (never declare done on exhaustion)

```
AskUserQuestion(
  questions: [{
    question: "Constitution did not reach an independent PASS within the bound.\n\nLast validator verdict: FAIL\nOutstanding fix list:\n[validator's issues]\n\nReason the loop stopped: [round cap | no-progress | kill-switch]\n\nHow should we proceed?",
    header: "Setup Escalation",
    options: [
      {label: "Give guidance and retry", description: "I'll add direction; run one more bounded pass"},
      {label: "Accept with noted gaps", description: "Ship the constitution as-is; I accept the outstanding items"},
      {label: "Abort", description: "Stop the run; leave the draft in .mochiko/memory/ for later"}
    ],
    multiSelect: false
  }]
)
```
The run stays **FAIL** unless the human explicitly accepts. "Ran out of rounds" is never "done."

---

## Phase 4 — Constitution acceptance  *(NEW human gate G3)*

Only reachable when the validator returned **PASS**. Present the validated constitution to the user for the final acceptance the original workflow never had (G3):

```
AskUserQuestion(
  questions: [{
    question: "The constitution at .mochiko/memory/constitution.md PASSED independent validation (graded by the independent validator).\n\nVersion: [from the version-bump call]\nPrinciples: [count]\nEssential Floor: [status]\n\nAccept this constitution?",
    header: "Constitution Acceptance",
    options: [
      {label: "Accept", description: "Adopt the constitution; proceed to finalize"},
      {label: "Amend - more changes", description: "Send specific changes back through the author→validator loop"},
      {label: "Reject", description: "Do not adopt; abort the run"}
    ],
    multiSelect: false
  }]
)
```

- **Accept** → Phase 5. The done-condition is now satisfied (validator PASS **and** human acceptance, **and** — brownfield — confirmed analysis).
- **Amend** → re-enter Phase 3 with the requested changes as the fix list (still bounded; the validator must PASS the amended version before returning here).
- **Reject** → abort; the draft remains under `.mochiko/memory/` for a later run.

---

## Phase 5 — Finalize  *(human gate G4)*

1. **Report to the user** (per mode). Read the accepted constitution for the summary:

   **Brownfield**
   ```markdown
   ## Setup Complete (Brownfield Mode)

   ### Artifacts
   - `.mochiko/memory/codebase-analysis.md` — codebase inventory and Essential-Floor assessment
   - `.mochiko/memory/constitution.md` — project governance ([version])

   ### Summary
   - Principles defined: [count]
   - Essential Floor: [status]
   - Independent validation: PASS (independent validator); accepted at the human gate

   ### Suggested commit
   `docs: create constitution [version] with brownfield analysis`

   ### Next steps
   1. Review the constitution at `.mochiko/memory/constitution.md`
   2. Run `/mochiko:specify` to start feature specification *(reference stub — `specify` not ported yet)*
   ```

   **Greenfield**
   ```markdown
   ## Setup Complete (Greenfield Mode)

   ### Artifacts
   - `.mochiko/memory/constitution.md` — project governance ([version])

   ### Summary
   - Principles defined: [count] (Essential Floor I-IV + architectural V-VII)
   - Independent validation: PASS (independent validator); accepted at the human gate

   ### Suggested commit
   `docs: create constitution [version]`

   ### Next steps
   1. Review the constitution at `.mochiko/memory/constitution.md`
   2. Run `/mochiko:specify` to start feature specification *(reference stub — `specify` not ported yet)*
   ```

   **Amend**
   ```markdown
   ## Setup Complete (Amendment)

   ### Artifacts
   - `.mochiko/memory/constitution.md` — updated to [new version]

   ### Summary
   - [changes summary]; independent validation: PASS; accepted at the human gate

   ### Suggested commit
   `docs: update constitution to [new version]`
   ```

2. **Cross-cutting follow-ups — documented stubs (NOT wired this run).** These responsibilities are real but live in other clusters; they are referenced here so nothing is silently lost. Do **not** dispatch them — there are no live skill mounts for them in this command:
   - **CLAUDE.md sync** — propagating the constitution into agent instructions is *handled by the claude-md-sync cluster (`syncing-claude-md`), not wired this run*. The constitution template still carries its own `## CLAUDE.md Synchronization` section as content; only the operational sync action is deferred.
   - **Evolution roadmap** — gap analysis between codebase and constitution (brownfield) is *handled by the roadmap cluster (`authoring-roadmap` + `evolution-roadmap-template.md`), not wired this run*.

3. **Cleanup** (G4): the workspace under `.mochiko/memory/` holds **durable** state (`constitution.md`, and in brownfield `codebase-analysis.md`) — there is no ephemeral context file to delete (the HIL `setup-context-*.md` was absorbed into this lead). Offer a lightweight retain/clean choice:
   ```
   AskUserQuestion(
     questions: [{
       question: "Setup is complete. The constitution (and any analysis) live under .mochiko/memory/.\n\nKeep the workspace as-is, or remove the intermediate analysis artifact?",
       header: "Cleanup",
       options: [
         {label: "Keep everything", description: "Retain constitution + analysis for reference"},
         {label: "Remove analysis only", description: "Delete codebase-analysis.md; keep the constitution"}
       ],
       multiSelect: false
     }]
   )
   ```
   Never offer to delete `constitution.md` — it is the deliverable.

---

## State recovery

If interrupted, resume from evidence in the `.mochiko/memory/` workspace (not from a context-file `phase` field — there is none):

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| `.mochiko/memory/` missing or empty | Phase 0 |
| brownfield chosen, `codebase-analysis.md` missing | Phase 1 |
| `codebase-analysis.md` present, not yet confirmed | Phase 2 |
| analysis confirmed (or greenfield/amend), no `constitution.md` | Phase 3 (produce) |
| `constitution.md` present, no recorded validator PASS | Phase 3 (validate) |
| validator PASS recorded, not yet accepted | Phase 4 |
| accepted | Phase 5 |
| `.mochiko/memory/SETUP_STOP` present | halt → escalate (3d) |

---

## Supervisor behaviors (what you own, not the agents)

- **Own the loop:** the round counter, the no-progress check, the round cap, the kill-switch, the escalation. The model's "looks done" is never the exit — the validator PASS + human acceptance is.
- **Own the verdict:** the author proposes, the validator grades from the artifact, **you** declare done — and only against the pre-declared, default-FAIL done-condition.
- **Own the human gates:** mode-select (G1), analysis checkpoint (G2), constitution acceptance (G3), cleanup (G4), and every escalation. Never auto-accept a constitution, a dropped item, or an exhaustion.
- **Track the mode** (brownfield / greenfield / amend) throughout; it selects the producer's branch and which phases run.
- **Never let producer and validator collapse into one agent.** `principal-architect` authors; the independent `validator` grades; they share no skills. Independence is the point.
- **Stay kernel-free:** no brain, DAG, MCP, or catalog. All orchestration is this command + the two dispatched agents + the `.mochiko/memory/` workspace.
