# Assessment — `templates/cross-artifact-checklist.md` (P14)

Run: `plan` cluster transform · Phase 1 (assess) · Producer: `transform-producer`
Source (read-only): `human-in-loop/plugins/humaninloop/templates/cross-artifact-checklist.md`
Consumed by (HIL): the **devils-advocate** during the plan command's **Phase 3.5 — Devil's Advocate Review (Incremental)** "consistency check" on the Phase-1 analysis artifacts (`commands/plan.md` L544, L573-580); the operative checks are driven by **P9 `validation-plan-artifacts`** (the advocate's skill — `plan.md` L561, `mode: incremental`).
ROLE THIS PASS: **assess/diagnose ONLY** — no transform, no grade. RQ3 (fold vs standalone) is **flagged for reconcile**, not resolved.

---

## Step 1 — Branch by class

**Class: template → artifact branch.** Weight goes to **placeholders, what consumes it, and path coupling** — not loop-ownership, not persona/procedure.

This is a **lightweight cross-artifact consistency checklist**: an H1 + a stated purpose ("incremental validation — fully review only the new artifact, quick-check previous artifacts without full re-read") + four check groups (**Entity Name Consistency**, **Requirement Traceability**, **Decision Consistency**, **Naming Conventions**), a 5-step **How to Use** method (extract entity mentions → grep previous artifacts → spot-check 2-3 requirement IDs → verify tech choices → flag inconsistencies), a **Time Budget** table (1-2 min/artifact, 2-3 min/phase), a **When to Escalate** rule, and an **Issue Classification** table (Critical/Important/Minor with examples). It is **passive reference content** — read as a checklist by the validating advocate; never filled, never executed, emits no artifact.

It is **misfiled as a "template."** A template (spec-template, plan-template, the report templates) defines an *artifact shape a producer fills*. This file defines no shape and is filled by no one — it is **validator procedure** (a checklist a reviewer executes), the same category as a `PHASE-CHECKLISTS.md` reference inside a `validation-*` skill. That category mismatch is the root of RQ3.

## Step 2 — Fast-path triage gate

1. **Orchestration-coupled?** `gate1 = NO` — passive content; as a static checklist it does **not** depend on a kernel, markdown supervisor, command, or DAG to *be a valid checklist*. Body grep for `CLAUDE_PLUGIN_ROOT | humaninloop | .humaninloop | hil-dag | catalog | mcp | dag | scripts/ | .sh | .py | agent-name | dispatch | brain` returns **ZERO hits** — fully clean. Its invocation *inside* the advocate's incremental review is the consumer's (P9 / the command's) orchestration, not this file's body coupling. (Same posture as the `spec-template` gate1=NO precedent.)
2. **Multi-responsibility / fans out?** `gate2 = YES` — REGISTRY files it under **BOTH `plan` AND `tasks`** (REGISTRY L138), so it nominally fans out to two clusters' validators. This is the gate that trips → full lens, and it *is* the RQ3 tension.
3. **Emits a non-machine-checkable artifact?** `gate3 = NO` (with a nuance) — the checklist **emits nothing**; it is consumed-reference, not a producer-emitter. (Contrast `spec-template` gate3=YES, because the `spec.md` it shapes is judgment-graded.) So **no producer↔validator partner is owed *for this file*.** Nuance: the *consistency verdict the checklist informs* is genuine model judgment (semantic entity-name matching, decision-honoring) — partly degenerate-deterministic (entity-name *presence* is a grep) — but that judgment belongs to its consumer (the advocate/P9), which **already has** its validator pairing in the plan loop.

`gate2` trips → **full lens** (no fast-path). The lens converges on the sibling-overlap check (5), which is where RQ3 lives.

## Step 3 — The lens (weighted for artifact branch)

**Check 1 — Orchestration test.** Orchestrated as **content**, not driven. *Content-coupling to kernel/DAG/catalog/MCP* = **NONE in the body** (grep clean, Step 2). *Orchestration-coupling owned by others* (re-home / single-source, do not edit the body):
- The plan command's Phase 3.5 **names** "cross-artifact checklist" (L544) but then **inlines the actual checks** (L573-580: entity-names-match, decisions-honored, requirement-IDs-trace, sensitivity-contract, integration-contract, infra-design, NFR-design) **and** points the advocate at **P9** (`validation-plan-artifacts`, `mode: incremental`, L561). The standalone file is referenced by **no file path anywhere in HIL** (`grep -rl cross-artifact-checklist plugins/` = empty). So the "consumer" relationship is to the **concept**, not the file; the operative checklist lives **inside P9** (`SKILL.md` "Incremental Review Mode" L85-143 + `PHASE-CHECKLISTS.md` "Phase P3" L140-156) and **inlined in the command**.
- Under mochiko **altitude doctrine**, the command's inlined checks (L573-580) must **not** survive in the ported command body — they single-source into the consuming validator skill (P9). So Check 1 produces a single-source/dedupe signal, not a body edit.

**Check 2 — Role (two altitudes).** Skill-role (artifact-altitude) = **a checklist a reviewer executes** (consumed procedure in template clothing). Team-role it confers = it makes its reader a **validator** (the advocate running the consistency check). It emits no artifact → **owes no validator partner**; it *participates in* the plan producer↔validator pairing that RQ1 is deciding (technical-analyst produces ↔ advocate[+architect] review). The conferred **validator** team-role is exactly what makes "this is validator procedure, not a standalone artifact" the live fold question.

**Check 3 — Independence.** No produce+grade leak (inert content; names no agent). The boundary it sits inside is sound: the consistency check is run by the **reviewer side** (advocate/P9), never by the producing analyst. The file neither holds nor threatens independence. Neutral. (Caveat for reconcile: whichever validator owns this checklist after RQ1 must **not** also be the plan producer — trivially satisfied since the analyst never runs it.)

**Check 4 — Verdict-sink / loop-driver.** The checklist drives a **consistency sub-verdict** that feeds the advocate's overall `ready / needs-revision / critical-gaps` verdict (P9 Verdict Criteria), on which the **lead** loops. The file owns **no loop and no verdict** — it is one input to the advocate's judgment. Loop-driving already re-homes to the plan lead (contract §1, §3); nothing new owed here.

**Check 5 — Sibling / overlap ("look sideways") — THE LOAD-BEARING CHECK (→ RQ3).**
- **Near-total content duplication with its only real consumer (P9).** Every responsibility in this file already exists inside `validation-plan-artifacts`:
  - the 4 check groups ↔ P9 `PHASE-CHECKLISTS.md` "Phase P3: Final Cross-Artifact Review" (L140-156) **and** P9 `SKILL.md` incremental-mode report rows (L123-131: entity-names-match, schemas-match-model, decisions-honored, sensitivity-present, integration-documented);
  - the **How to Use** method ↔ P9 `SKILL.md` "Consistency Check (Previous Artifacts)" steps (L95-101);
  - the **Time Budget** ↔ P9's "1-2 minutes per previous artifact" (L101, L143);
  - **When to Escalate** ↔ P9 "When to Escalate to Full Re-Review" (L103-107);
  - **Issue Classification** ↔ P9 Issue Classification table (L44-50) + `ISSUE-TEMPLATES.md`.
  → This is a **thin variant over a shared core that the core (P9) already owns** — the canonical `merge-into-sibling` / `dedupe` signal. P14 is effectively an **orphaned third copy** (the other two live in P9 and the command body).
- **The tasks-cluster "reuse" factor (REGISTRY files it under `tasks` too) — examined, it ARGUES AGAINST standalone-for-sharing.** The tasks-cluster validator `validation-task-artifacts` **already self-contains its own** "Cross-Artifact Review" (`PHASE-CHECKLISTS.md` L117-148) with **tasks-specific, different** checks (Mapping↔Tasks alignment, Story→Cycle→Tasks traceability, cycle/dependency consistency) — and it does **not** import this template. So plan's checklist (entity/decision/sensitivity/integration/NFR) and tasks' checklist (mapping/cycle/dependency) are **not one shared checklist**; each validator carries a **phase-tuned** copy. The REGISTRY dual-filing records "the *concept* of a cross-artifact check appears in both clusters," **not** "one file is imported by both." The single-source-standalone premise is **not borne out by the HIL artifacts**.
- **→ `flag-for-reconcile` (RQ3).** A relational move (fold-into-P9 vs keep-standalone-shared) cannot be decided from one primitive; it also intersects **RQ1** (which validator owns the consistency check) and is **gated** (contract §4a).

**Check 6 — Coupling audit.**
- **Path coupling — IN BODY: NONE.** No `.humaninloop/`, no `${CLAUDE_PLUGIN_ROOT}`, no catalog/MCP/DAG/script path. Grep clean.
- **Reference coupling (rebind-by-reference targets):** the body names HIL plan **artifact filenames** — `data-model.md` (L12-13, 44), `constraints-and-decisions.md` (L22, 27, 36, 43) — and ID conventions `FR-XXX` / `US-XXX` (L17), plus an example `userId`/`user_id` (L66). These are **downstream-plan artifact names** (the ported `plan-template`/`data-model` land later this same cluster, P10/P12; the mochiko names are not yet fixed). → `kept-but-rebind` **by reference** (align to the mochiko plan artifact names when those primitives land; not a `.humaninloop/`→`.mochiko/` path swap — there is no path here to swap).
- **Excluded-script residue:** none in P14's body. (P9's `PHASE-CHECKLISTS.md` L165-196 references the **excluded** `scripts/check-artifacts.py` for automated entity-consistency — that is **P9's** coupling to clean on P9's port, **not P14's**.)
- **Determinism boundary:** mostly **model judgment** (semantic entity-name consistency, decision-honoring, requirement-trace correctness). Entity-name *presence* is a Tier-1 grep the validator may layer; the *substance* is graded by the reviewing model. So the checklist is a **judgment aid for the validator**, not a deterministic schema — reinforcing "validator procedure, not a fillable template."

**Check 7 — Conventions + loop placement.**
- **Classification:** inert artifact — no `disable-model-invocation`. The convention-wiring floor still applies, but its *shape* is **contingent on RQ3**: if **standalone**, a hinted router/template entry + path-bind to the mochiko templates dir; if **folded into P9**, **no separate router entry** (it becomes a section/reference of P9, which carries its own classification). No trigger phrasing (not a skill); no `skills:` (not an agent).
- **Decoupling scan: CLEAN.** Body names **no agent** (no "devils-advocate"/"principal-architect"), no "dispatch," no injected workflow modes/paths/phases, no "workflow-agnostic" meta-label (carries no persona). Nothing for the deny-list grep — **decoupling PASS**.
- **Single-source convention (wiring step 6):** this is the **headline convention finding** — the file **restates** doctrine its real consumer (P9) already owns. mochiko's single-source rule says *reference, don't restate*. That is the engine behind the fold recommendation, but the move itself is **reconcile's** to make (RQ3 + RQ1).
- **Loop placement:** the checklist supplies a **consistency sub-surface** the advocate validates against; it does **not** itself supply a done-condition, independent validation, or a human gate — those live on the advocate / the plan lead. **No new loop gap introduced.**

## Step 4 — Disposition

**Body × Structural = `port-with-edits` × `flag-for-reconcile` (RQ3: fold-into-P9 vs standalone).**

- **Body = `port-with-edits` (proposed; contingent on the structural verdict).** The prose is mochiko-clean — zero kernel/DAG/catalog/path coupling — so no `redesign` is owed (no mechanism to rewrite around) and `drop` is wrong (the consistency-check *responsibility* must survive). But `keep-verbatim` is **not** honest either, because **either** reconcile outcome forces edits:
  - **If folded into P9:** the duplicated scaffolding (Time Budget, When-to-Escalate, Issue Classification, the How-to-Use method) is **`dedupe`** against P9's existing equivalents; only the variant-unique checklist rows are `folded-into-skill`. That is a trim, i.e. `port-with-edits` (tending toward partial removal).
  - **If kept standalone:** the body still needs the **artifact-name rebind** (R6) + classification/router wiring. That is a light `port-with-edits` (tending toward `keep-verbatim` + `kept-but-rebind`).
  So `port-with-edits` is the honest floor under both branches; the *magnitude* (heavy trim vs light rebind) is set by RQ3. Per the assess rule, the body treatment is **proposed**; the structural move is **not guessed**.
- **Structural = `flag-for-reconcile`.** Signal: **thin variant whose entire content is already single-sourced inside its only real consumer (P9)**; **`merge-into-sibling` / `dedupe` / fold-into-validator-skill** candidate. The nominal second consumer (tasks) **self-contains a different, phase-tuned** cross-artifact checklist and does not import this file, so the single-source-standalone case is weak. The decision intersects **RQ1** (reviewer architecture — *which* validator owns the consistency check) and is in the **gated bundle** (contract §4a). **Reconcile decides, human-gated; assess does not resolve.**

## Step 5 — Responsibility trace (every responsibility tagged; relational tags pending reconcile per TRACE-TAGS rule 3)

| # | Responsibility | Tag |
|---|----------------|-----|
| R1 | Provide the **cross-artifact consistency checklist content** — the 4 check groups (Entity-Name Consistency, Requirement Traceability, Decision Consistency, Naming Conventions) the advocate executes | **flag-for-reconcile (RQ3)** → `folded-into-skill` (into P9's incremental-review reference) **if fold**; `kept-but-rebind` **if standalone**. Pending reconcile. |
| R2 | Define the **incremental-validation method** (How to Use: extract entities → grep previous artifacts → spot-check 2-3 IDs → verify tech choices → flag) — **already present in P9** `SKILL.md` "Consistency Check (Previous Artifacts)" L95-101 | **dedupe** → P9 (single source). Pending reconcile confirm. |
| R3 | **Time Budget** (1-2 min/artifact, 2-3 min/phase) — **duplicates** P9 L101/L143 | **dedupe** → P9 |
| R4 | **When-to-Escalate** rule (2+ issues → full re-review; contradictions → flag) — **duplicates** P9 "When to Escalate to Full Re-Review" L103-107 | **dedupe** → P9 |
| R5 | **Issue Classification** (Critical/Important/Minor + examples) — **duplicates** P9 Issue Classification L44-50 + `ISSUE-TEMPLATES.md` | **dedupe** → P9 |
| R6 | **Artifact-name / ID references** (`data-model.md`, `constraints-and-decisions.md`, `FR-XXX`/`US-XXX`, `userId`/`user_id` example) — point at downstream HIL plan artifacts | **kept-but-rebind** (by reference — align to mochiko plan artifact names when P10/P12 land; **no path swap**, no `.humaninloop/` present) |
| R7 | **Nominal tasks-cluster reuse** (REGISTRY files under `tasks`) — tasks validator already self-contains a **different** cross-artifact review and does not import this file | **flag-for-reconcile (RQ3 factor)** → `moved-to-other-cluster` is **NOT owed** (no import dependency); confirm at reconcile that the `tasks` port carries its own. Tasks not ported this run (deferred). |
| R8 | Be the **consistency-check input surface** the advocate (P9) executes in Phase-3.5 incremental review | **flag-for-reconcile (RQ3)** → lands wherever R1 lands (P9 section if fold; standalone file if standalone). The execution **action** already lives on the advocate/P9 (pairing exists). |
| R9 | **Convention-wiring floor** (classification/router/path-bind) | **kept-but-rebind** (wiring floor) — **shape contingent on RQ3**: hinted router+template-path if standalone; **no separate entry** if folded into P9 |

No responsibility left untagged. **No silent drop** — R1/R8 are flagged (not dropped); R2-R5 are `dedupe` into the single source (P9), not loss; R6 is a by-reference rebind; R7 is a reconcile factor with **no** orphan (tasks self-contains).

## Reconcile / coupling flags

- **F-RQ3 (BLOCKING structural — the headline; gated, contract §4a) — cross-artifact-checklist fate: STANDALONE vs FOLD INTO P9.**
  - **Case FOR fold into `validation-plan-artifacts` (P9):** (1) **near-total content duplication** — every section (checks, method, time budget, escalation, classification) already exists in P9 (Check 5); P14 is an **orphaned third copy**. (2) **No file-path consumer** — HIL references the *concept* ("using cross-artifact checklist", `plan.md` L544), never the file; the live checklist is P9's bundled reference + the command's inlined checks. (3) **Category fit** — it is **validator procedure** (a checklist a reviewer executes), not a fillable artifact shape; procedure belongs **in the consuming skill**, the same way the report templates stay standalone but `strategy-*` dissolved into `loop-discipline`. (4) **mochiko single-source convention** (wiring step 6) says reference, don't restate. → fold = lean, single-source, kills the duplicate.
  - **Case FOR standalone (REGISTRY plan+tasks dual-filing → single source for two validators):** **the strongest-sounding argument, but undercut by the artifacts** — the tasks validator (`validation-task-artifacts`) **already self-contains a different, phase-tuned** cross-artifact review (mapping/cycle/dependency, not entity/decision/NFR) and **does not import** this file. So "one shared source for two clusters" is **not the HIL reality**; each cluster carries its own. Standalone-for-sharing would be **building shared infra no workflow actually shares** — against mochiko's "don't pre-define conventions no workflow has needed."
  - **Tasks-cluster-reuse factor (the named decider):** examined, it **weakens** standalone rather than supporting it (above). `tasks` is **deferred** this run (plan-core only), so reconcile decides between: **(a)** fold into P9 now, and let the future `tasks` port carry its own cross-artifact checklist inside `validation-task-artifacts` (mirrors HIL reality); or **(b)** extract a shared cross-cluster checklist primitive (speculative — no workflow shares it today).
  - **Intersection with RQ1 (reviewer architecture):** the checklist's home depends on **which** validator owns the plan consistency check. If RQ1 → **two distinct validators** (checklist advocate + adversarial architect), the consistency checklist sits with the **checklist advocate** (P9 lineage) → fold target is unambiguous. If RQ1 → **one reviewer** (specify shape), still folds into that one reviewer's skill. Either RQ1 outcome points the fold at the **advocate/P9 side**, never the architect's feasibility side.
  - **Producer's lean (for reconcile, not a decision):** **fold into P9** (`merge-into-sibling`/`dedupe`) — duplication is near-total, there is no file consumer, the tasks "reuse" is not a real import, and single-source is the convention. Resolve under RQ1; human-gated.
- **F-RQ3a (soft, reconcile/rebind).** R6 artifact-name references rebind **by reference** to the mochiko plan artifact names once P10 (`plan-template`) / P12 (`techanalyst-report-template`) / the data-model artifact land. No `.humaninloop/` path exists to swap; this is name-alignment, not path-rebind.
- **Decoupling scan: PASS** — body names no agent, no "dispatch," no workflow modes/paths/phases, no "workflow-agnostic" label. Nothing for `verify-output`'s deny-list grep.
- **Kernel-free scan: PASS** — body grep for kernel/DAG/catalog/MCP/script tokens = zero hits. (Any `check-artifacts.py` residue is **P9's** to clean, not P14's.)

---

### Summary block

```
ASSESSMENT: cross-artifact-checklist.md (P14)
Class:        template → branch artifact (lightweight cross-artifact CONSISTENCY CHECKLIST; consumed by the advocate / P9 in plan Phase-3.5 incremental review; emits nothing)
Triage:       gate1=n gate2=y gate3=n  [full-lens]   (gate2 trips: REGISTRY files it under plan AND tasks)
Disposition:  port-with-edits × flag-for-reconcile (RQ3: fold-into-P9 vs standalone)
              body proposed (clean prose; either outcome forces edits — dedupe-trim if fold, name-rebind if standalone); structural NOT guessed
Trace:
  - R1 cross-artifact checklist content (4 check groups)                         → flag-for-reconcile RQ3 (folded-into-skill→P9 if fold | kept-but-rebind if standalone)
  - R2 incremental-validation method (How-to-Use steps) — already in P9 L95-101  → dedupe → P9
  - R3 Time Budget — duplicates P9 L101/L143                                      → dedupe → P9
  - R4 When-to-Escalate — duplicates P9 L103-107                                  → dedupe → P9
  - R5 Issue Classification — duplicates P9 L44-50 + ISSUE-TEMPLATES.md           → dedupe → P9
  - R6 artifact-name/ID refs (data-model.md, constraints-and-decisions.md, FR/US-XXX) → kept-but-rebind (by reference; no .humaninloop/ path to swap)
  - R7 nominal tasks-cluster reuse (REGISTRY) — tasks validator self-contains its own → flag-for-reconcile RQ3 factor (moved-to-other-cluster NOT owed; no orphan)
  - R8 consistency-check input surface the advocate executes                     → flag-for-reconcile RQ3 (lands where R1 lands; execution action already on advocate/P9)
  - R9 convention-wiring floor (classification/router/path-bind)                 → kept-but-rebind (shape contingent on RQ3)
Path coupling:    IN-BODY = none (grep clean). Refs = HIL plan artifact filenames → rebind-by-reference (R6).
Kernel/DAG/agent: body grep ZERO hits — kernel/DAG/catalog/MCP/script/agent clean. Decoupling scan PASS. (check-artifacts.py residue is P9's, not P14's.)
Reconcile flags:
  - F-RQ3 (BLOCKING, gated) cross-artifact-checklist fate: STANDALONE vs FOLD-INTO-P9.
      FOR fold: near-total dup of P9; no file-path consumer (concept-ref only); it's validator PROCEDURE not a fillable shape; single-source convention.
      FOR standalone: REGISTRY plan+tasks dual-filing — BUT undercut: tasks validator self-contains a DIFFERENT phase-tuned checklist & does not import this file → "shared single-source" not the HIL reality.
      Tasks-reuse factor: WEAKENS standalone (no real import; tasks deferred this run). Intersects RQ1 (which validator owns the check → points fold at the advocate/P9 side either way).
      Producer lean (reconcile decides): FOLD into P9 (merge-into-sibling/dedupe). Human-gated.
  - F-RQ3a (soft) R6 name-rebind by reference once plan-template/data-model land.
  - No independence leak; no new loop gap; no orphaned responsibility (tasks self-contains).
```
