# Assessment — `skills/analysis-codebase/` (P6)

Run: `setup` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/skills/analysis-codebase/`
Files: `SKILL.md` + `references/{BROWNFIELD-ANALYSIS.md, CONTEXT-GATHERING.md}` + `scripts/detect-stack.sh`

---

## Step 1 — Branch by class

**Class: skill → PLAYS-a-role branch.** What carries weight for a skill: consumed-procedure vs emits-artifact, trigger reliability, sibling overlap.

This is a **hybrid** skill:
- **Consumed-procedure** — detection heuristics (tech stack, framework, ORM, architecture, entity/endpoint, essential-floor) that its caller (principal-architect) runs.
- **Emits-artifact** — in Setup-Brownfield mode it produces `codebase-analysis.md`; in Brownfield mode it produces a JSON inventory.
- Bundles a **deterministic helper** `detect-stack.sh` (a determinism boundary, not a brain).

It is **multi-mode**: Context · Brownfield · Setup-Brownfield. Only Setup-Brownfield (+ the detect-stack script, + a slice of Context) is exercised by the setup cluster; the other modes feed clusters not ported this run.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = YES` — driven by two consumers: the **setup command** (Phase 0 runs `detect-stack.sh` directly) and the **principal-architect** agent (Phase 1 invokes the skill, mode setup-brownfield). NOTE: this is **markdown-supervisor / agent orchestration, NOT kernel** content-coupling — see Check 1.
2. **Multi-responsibility / fans out?** `gate2 = YES` — three modes, many responsibilities; fans out to setup (this run) AND spec/plan + constitution-context (other clusters).
3. **Emits a non-machine-checkable artifact?** `gate3 = YES` — `codebase-analysis.md` is a judgment report (essential-floor status, strengths to preserve, inconsistencies, recommended constitution focus). Not a schema/version-equality check. (The JSON inventory *is* schema-checkable against `codebase-inventory-schema.json`, but the markdown analysis is not.)

All three trip → **full 7-check lens**.

## Step 3 — The 7-check lens

**Check 1 — Orchestration test.** Orchestrated by the **setup command (markdown supervisor)** + **principal-architect agent**, NOT the Python brain. Separating the two couplings:
- **Content-coupling to kernel: NONE.** `grep` over the whole skill finds no `humaninloop_brain`, no `hil-dag`/MCP, no DAG, no catalog — only three `.humaninloop/` path strings. `detect-stack.sh` is pure `bash` + `jq` reading project files (`package.json`, `go.mod`, …) → JSON on stdout. No network, no kernel, no MCP. **It ports WITH the skill; it is NOT a brain script.**
- **Orchestration-coupling:** the two invocation points (Phase-0 script call; Phase-1 skill invocation) re-home onto mochiko-native homes — the **mochiko setup command** (Phase 0 script) and the **mochiko principal-architect** `skills:` list (Phase 1). The skill body owns no orchestration. **No kernel to shed.**

**Check 2 — Role (two altitudes).** Skill-role = **consumed-procedure + emits-artifact** (hybrid). Team-role it confers = **producer** (makes principal-architect the author of the codebase analysis). It confers **no validator role** — there is no grading inside this skill; it is pure analysis/production.

**Check 3 — Independence.** No produce+grade leak *inside this skill* — it is a clean producer-procedure. (The principal-architect carries `validation-constitution` too, but that is a constitution-sibling concern, not analysis-codebase.) The only gate on this skill's output is a **human checkpoint** (Phase 2 `AskUserQuestion`), which is a human gate, not an independent validator — see Check 7.

**Check 4 — Verdict-sink / loop-driver.** Consumers:
- setup command Phase 0 → `detect-stack.sh` JSON (deterministic baseline → brownfield-status heuristic + mode prompt).
- principal-architect Phase 1 → produces `codebase-analysis.md` + `architect-report.md`.
- setup command Phase 2 → human checkpoint reads `architect-report.md`; on Edit/Reject the supervisor **loops back to Phase 1** (re-invoke architect). The loop-driver is the **setup supervisor's Phase 1↔2 revision loop**, human-gated — no kernel owns it. That loop re-homes onto the setup command (a P1 concern, surfaced here as a relational fact).
- Downstream (other clusters): brownfield-constitution consumes the analysis; spec/plan consume the JSON inventory's collision risks.

**Check 5 — Sibling / overlap.**
- vs **brownfield-constitution (P4):** both touch "Essential Floor" (Security/Testing/Error-Handling/Observability). analysis-codebase **assesses current status** (present/partial/absent + grep indicators); brownfield-constitution **requires** the floor (ESSENTIAL-FLOOR ref). Likely two uses of one concept, not duplicated substrate — but a **dedupe signal** to confirm with cluster context. → flag.
- vs **authoring/validation-constitution:** no trigger collision (constitution verbs vs analyze/scan/brownfield verbs); distinct.
- **Multi-cluster fan-out (the big signal):** Brownfield mode (JSON inventory → spec/plan collision) and the standalone Context-mode report (→ constitution-context) serve clusters **not ported this run**. Whether the setup port carries all three modes or splits off only the setup-brownfield slice (+ script) is a **split/scope decision** needing cluster context. → flag.
- Cross-cluster naming: the `analysis-*` family + the deferred `brownfield-integration` share trigger space ("brownfield", "existing code") — out of this run's scope, noted only.

**Check 6 — Coupling audit.**
- **Hardcoded paths (rebind):** `.humaninloop/memory/codebase-analysis.md` (SKILL.md L136, L321); `.humaninloop/memory/constitution.md` (CONTEXT-GATHERING.md L37) → `.mochiko/...`.
- **Template references:** `codebase-analysis-template.md` (P9) + `codebase-inventory-schema.json` (P10) — both in-scope this run; rebind path references to the mochiko templates dir.
- **Prerequisite/handoff:** script output → `context.md` → architect (Phase 0→1). Preserve the handoff edge.
- **Determinism boundary (core of Check 6):** `detect-stack.sh` is the **deterministic layer**; the LLM heuristics are the **model-judgment layer**. The skill states the boundary explicitly ("Run script first for deterministic baseline → Script findings are ground truth; LLM adds nuance"). This is a legitimate tool boundary mochiko KEEPS — verified kernel-free. Minor portability nits to note for the edit pass (not blockers, not coupling): `set -e` + `cd "$PROJECT_DIR"`, hard dependency on `jq`.

**Check 7 — Conventions + loop placement.**
- **Classification:** model-invoked (agent-consumed by principal-architect) — keep model-invoked (default). ✓
- **Trigger phrasing:** current description is user-phrased ("when the user says 'analyze codebase'…"). For an **agent-consumed** skill, triggers must describe the **work context** (analysis/transformation work), not "when the user says…" — a wiring-pass rewrite.
- **Router registration + path rebind:** owed by the convention-wiring pass (transform-recipes).
- **Loop placement:** the skill owns no loop. The loop + human gate live on the **setup command supervisor** (Phase 2 human checkpoint; Phase 1↔2 revision loop). **No independent validator** gates the analysis output today — only the human checkpoint; the in-skill quality checklists are self-checks, not independence. Whether the setup cluster wants an independent analysis-validator vs. accepting the human checkpoint as the deliberate gate is a **loop-placement decision** for setup command + reconcile. → flag.

## Step 4 — Disposition

**Body × Structural = `port-with-edits` × `flag-for-reconcile`** (proposed base: `standalone`).

- **Body = `port-with-edits`** (not keep-verbatim, not redesign). The body is mochiko-clean in substance — no kernel, pure procedure + a deterministic helper — but needs **localized edits**: path rebinds, template-path rebinds, trigger-phrasing rewrite, and mode-scoping markers once reconcile decides. Structure and voice preserved. `detect-stack.sh` itself is **keep-verbatim** within the port (the determinism boundary), modulo optional portability nits. Minimalism governor satisfied: an edit fixes it; no redesign earned.
- **Structural = `flag-for-reconcile`** — the placement depends on siblings (mode-scoping split, essential-floor dedupe, validator pairing). Proposed base **`standalone`** (lands as one mochiko skill with bundled script), modified by the reconcile verdicts. Do NOT guess the relational move here.

## Step 5 — Responsibility trace (every responsibility tagged)

| # | Responsibility | Tag |
|---|----------------|-----|
| R1 | Deterministic tech-stack detection (project type, pkg mgr, frameworks, ORMs, architecture, CI/CD, file presence) via `detect-stack.sh` | **kept** (script bundled, keep-verbatim; deterministic boundary preserved; consumed by setup Phase-0 + skill) |
| R2 | Framework / ORM / architecture-pattern recognition heuristics (markdown tables — model-judgment layer over the script) | **kept** (port-with-edits, content preserved) |
| R3 | Context-gathering sub-procedure (conventions, CI gates, team signals, existing governance/docs) as folded into setup-brownfield | **kept**; standalone **Context-mode report** output → **flag-for-reconcile** (scope: may belong to constitution-context cluster) |
| R4 | Brownfield analysis (entity/endpoint detection, collision-risk assessment) → JSON inventory | **flag-for-reconcile** → likely `moved-to-other-cluster` (collision mode serves spec/plan), or split |
| R5 | Setup-Brownfield comprehensive analysis (Context+Brownfield+Floor) → `codebase-analysis.md` per template | **kept-but-rebind** (`.humaninloop/`→`.mochiko/`; template-path rebind) — THE setup-cluster responsibility |
| R6 | Essential-Floor assessment (Security/Testing/Error-Handling/Observability, present/partial/absent + grep indicators) | **kept** + **flag-for-reconcile** (dedupe check vs brownfield-constitution: assess-status vs require-floor) |
| R7 | Output path binding `.humaninloop/memory/codebase-analysis.md` + `ls .humaninloop/memory/constitution.md` detection | **kept-but-rebind** → `.mochiko/...` |
| R8 | Template consumption: `codebase-analysis-template.md` (md output) + `codebase-inventory-schema.json` (JSON output) | **kept-but-rebind** (rebind template references to mochiko templates dir; both in-scope this run) |
| R9 | Trigger / discoverability (model-invoked description, graded phrases) | **kept-but-rebind** (rephrase to agent-work-context triggers; stay model-invoked; router registration — convention-wiring floor) |
| R10 | Manual detection commands (fallback bash snippets when script insufficient) | **kept** (port-with-edits; preserved) |
| R11 | Self-check quality checklists (per-mode + Setup-Brownfield) | **kept** as authoring guidance; the **independent-validation gap** they do NOT close → loop-placement note re-homed to setup command/reconcile (see F3) — NOT independence by themselves |
| R12 | Cross-skill "Related Skills" references (brownfield/authoring/validation-constitution) | **kept-but-rebind** (rebind to mochiko skill names) |

No responsibility left untagged; no silent drop.

## Reconcile flags

- **F1 — Mode-scoping split.** Multi-mode skill fans out beyond setup. Decide whether the setup port carries all three modes or splits off only the **setup-brownfield slice + `detect-stack.sh`**, deferring/moving the **Brownfield/collision** mode (→ spec/plan) and the **standalone Context-mode report** (→ constitution-context). Partner context: spec/plan clusters, constitution-context (P8). Affects R3, R4.
- **F2 — Essential-floor dedupe vs `brownfield-constitution` (P4).** Confirm assess-status (here) vs require-floor (P4) are two uses of one concept (keep both) or duplicated substrate (dedupe to one source). Partner: P4. Affects R6.
- **F3 — Independent-validator pairing / loop placement.** `codebase-analysis.md` is a reviewable artifact whose only current gate is the Phase-2 human checkpoint — no independent validator. Decide with the setup command (P1) whether the cluster wants an independent analysis validator or the human checkpoint is the deliberate gate. Partner: P1. Affects R11 / Check-7 loop placement.

---

### Summary block

```
ASSESSMENT: analysis-codebase (P6)
Class:        skill → branch PLAYS-a-role (hybrid: consumed-procedure + emits-artifact, bundles deterministic helper)
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × flag-for-reconcile (proposed base: standalone)
Trace:
  - R1 deterministic stack detection (detect-stack.sh)            → kept (keep-verbatim; deterministic boundary, kernel-free)
  - R2 framework/ORM/architecture heuristics                     → kept
  - R3 context-gathering sub-procedure                           → kept; standalone Context-report → flag-for-reconcile (F1)
  - R4 brownfield analysis → JSON inventory (collision mode)     → flag-for-reconcile (F1; likely moved-to-other-cluster)
  - R5 setup-brownfield analysis → codebase-analysis.md          → kept-but-rebind
  - R6 essential-floor assessment                                → kept + flag-for-reconcile (F2 dedupe vs brownfield-constitution)
  - R7 output path .humaninloop/ + constitution detection         → kept-but-rebind (.mochiko/)
  - R8 template consumption (analysis-template + inventory-schema) → kept-but-rebind
  - R9 trigger/discoverability                                   → kept-but-rebind (agent-work-context triggers; router reg)
  - R10 manual detection commands                                → kept
  - R11 self-check quality checklists                            → kept; independence gap → setup command/reconcile (F3)
  - R12 related-skills cross-refs                                → kept-but-rebind
Reconcile flags:
  - F1 mode-scoping split (setup-brownfield slice + script vs other-cluster modes) — partners: spec/plan, constitution-context
  - F2 essential-floor dedupe vs brownfield-constitution (P4) — assess-status vs require-floor
  - F3 independent-validator pairing / loop placement — partner: setup command (P1); human checkpoint vs independent validator
```
