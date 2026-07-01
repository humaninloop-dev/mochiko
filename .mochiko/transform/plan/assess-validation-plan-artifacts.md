# ASSESSMENT: validation-plan-artifacts (P9)

**Source:** `human-in-loop/plugins/humaninloop/skills/validation-plan-artifacts/`
(SKILL.md + references/{PHASE-CHECKLISTS.md, ISSUE-TEMPLATES.md} + scripts/check-artifacts.py)
**Run:** transform-cluster / `plan` (plan-core scope) · **Assessed:** 2026-06-30
**Assessor:** `mochiko:transform-producer` (assess-only; no edit applied, no grade)
**Run-context mandate:** this is the **validator-side** skill of the plan producer↔validator pair —
the **completeness reviewer's** checklist that `devils-advocate` (P4) mounts to grade
`technical-analyst`'s (P2) plan artifacts. Independence and the convention-5 *form* carry the most weight.

---

## Header

```
Class:        skill → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=y  → full-lens
Disposition:  port-with-edits × standalone   (matches contract hypothesis)
              + flag-for-reconcile: RQ1 (reviewer architecture — feasibility overlap)
              + flag-for-reconcile: RQ3 (P14 cross-artifact-checklist fold-in / dedupe)
Validator form: MIRROR-CHECKLIST (convention-5 objective-acceptance-criteria form),
                wrapped in anti-rationalization discipline. NOT adversarial-critique.
Reconcile flags: RQ1 (primary, relational) · RQ3 (relational, P9↔P14) ·
                 + incremental-mode phase-switch split (procedure→skill / selection→lead) ·
                 + A0 scope note (codebase-discovery review = out of plan-core)
```

**One-line:** A clean, kernel-free completeness checklist whose *body* is sound but is **saturated with
HIL workflow-phase identifiers (A0/P1/P2/P3)** the supervisor injects as dispatch parameters
(`validation-plan-artifacts (phase: P1)`, `(phase: P2, mode: incremental)` — plan.md:422,561). The port
keeps the checks, **decouples the phase axis** (reorganize by artifact-type; the phase-switch is the
lead's), and **flags two relational questions** it cannot answer alone: its feasibility-overlap with the
architect (RQ1) and the orphan P14 template that duplicates its already-bundled cross-artifact content (RQ3).

---

## Step 1 — Branch by class

**Class: skill → PLAYS-a-role branch.** Weighting: consumed-procedure-vs-emits-artifact, trigger
reliability, sibling overlap, and **decoupling** (no agent names; independence stated by *role*). The
**independence linchpin (Check 3)** carries extra weight by run-context mandate — this is a validator-side
skill, and the failure mode is co-mounting it on the producer. Loop-ownership (done-condition /
who-drives / human gate) is NOT weighted on the skill — it sits on the `plan` lead (P1).

## Step 2 — Fast-path triage gate

| Gate | Q | Verdict |
|------|---|---------|
| 1 | **Orchestration-coupled?** | **YES (orchestration) / NO (kernel-content).** No kernel/MCP/DAG/catalog in body or script. But the skill only runs because the **plan supervisor dispatches it with injected `phase:`/`mode:` parameters** (plan.md:422 `validation-plan-artifacts (phase: P1)`; :561 `(phase: P2, mode: incremental)`) and loops on its verdict. The coupling lives *around* the skill (rehome to P1) **plus** a content-level phase-label coupling baked into the body (decouple-edit). |
| 2 | **Multi-responsibility / fans out?** | **YES.** Holds analysis-phase checks (FR→TR coverage, orphan TRs, testable criteria, ≥2 alternatives, constraint sourcing, NFR measurability…), design-phase checks (entity/endpoint coverage, schema-model consistency, sensitivity annotations, integration boundaries…), cross-artifact consistency, severity calibration, verdict emission, an incremental-review mode, anti-rationalization discipline, and a deterministic script — feeding one reviewer report consumed by the lead. |
| 3 | **Emits a non-machine-checkable artifact?** | **YES.** Emits a gap/issue report + `ready`/`needs-revision`/`critical-gaps` verdict. Core judgments are model-grade ("is the `User` entity from FR-003 present in data-model.md?", "is the rationale convincing or just restating the choice?", "can the design meet the NFR target?"). The script covers only a thin objective slice. **Real validator, not a degenerate deterministic assert.** |

All three trip → **full 7-check lens.**

---

## Step 3 — The lens (weighted PLAYS-a-role / skill; validator-side)

### 1. Orchestration test (content-coupling vs orchestration-coupling)

- **Orchestrating layer:** the HIL `plan` **command supervisor** (`commands/plan.md`) — a DAG-mediated
  markdown supervisor. It dispatches P9 *to* the `devils-advocate` agent, **parameterized by phase/mode**
  (`phase: P1` in the Analysis completeness review §2.x; `phase: P2, mode: incremental` in the Design
  review §3.5), sequences architect-feasibility (Phase 1 only) before/around advocate-completeness (both
  phases), and drives the FAIL-loop ("Feasibility Iteration {N}", "Iteration {N}").
- **Content-coupling (kernel):** **NONE.** Grep of SKILL.md + refs + script for
  `DAG|catalog|MCP|hil-dag|brain|networkx|pydantic|\.json` → zero. The script imports stdlib only
  (`sys, os, re, json, pathlib, typing`). Body is clean markdown + a self-contained Python checker.
- **Content-coupling (workflow phases) — the real body defect:** the skill is **organized around HIL's
  phase identifiers**. 20 phase-token occurrences (A0/P1/P2/P3). The "Review Focus by Phase" table
  (SKILL.md:33-38), the "When to Use" phase lines (:16-17), the "Phase Application" table (:140-143), and
  "For Phase P2 (Design), use incremental review" (:87) all make **phase-number the organizing axis**.
  That is workflow-orchestration injected into the skill body → a **`port-with-edits` decouple action**
  (reorganize by **artifact-type**: analysis artifacts / design artifacts / cross-artifact; drop the
  A0/P1/P2/P3 labels; the *phase sequencing* is the lead's).
- **Orchestration-coupling to re-home (→ P1):** dispatch/sequencing, the architect-once-then-advocate
  ordering, the skip-architect-re-review-unless-structural-change routing, the incremental-mode
  *selection*, verdict consumption, and the FAIL-loop. None of these live in P9's body — they are the
  **lead's** (contract §1 explicitly requires they be rehomed, not dropped). P9 carries the *checks* and
  the incremental *procedure*; P1 carries the *when*.
- **One stray orchestration token in body:** SKILL.md:106 "If contradictions detected → **flag for
  supervisor**." Generalize (report the contradiction; the caller/lead routes) — light decouple edit.

### 2. Role at two altitudes

- **Skill-role:** a **consumed procedure that emits a reviewable artifact** — a phase/artifact checklist a
  caller runs to produce a classified-issue report + 3-state verdict over plan artifacts. It does not
  author requirements/data-model/contracts (those are P5–P8 on the analyst); it **grades** them.
- **Team-role conferred:** **VALIDATOR** (completeness reviewer). Whoever runs this skill becomes the
  grading half of the plan producer↔validator pair (producer = `technical-analyst`). Terminal grader —
  its verdict is arbitrated by the lead + human gate; it needs no downstream validator of its own.
- **Convention-5 FORM — MIRROR-CHECKLIST (decisive determination):**
  - P9 is the **mirror-checklist** form (ROADMAP convention 5: "a *mirror checklist* skill for objective
    acceptance criteria"). Evidence: enumerable named checks with fixed questions and a per-check severity
    (PHASE-CHECKLISTS.md tables), and a verdict **mechanically derived from issue counts**
    (`ready` = 0 Critical + 0 Important; `needs-revision` = 1–3 Important; `critical-gaps` = 1+ Critical or
    4+ Important — SKILL.md:147-153). Backed by a deterministic script for the objective slice.
  - The anti-rationalization prose (Red Flags, Common Rationalizations, "letter = spirit") is **not**
    adversarial-critique — it is the **discipline wrapper** that keeps the checklist honest (don't
    rubber-stamp, don't downgrade severity). A mirror-checklist *with rigor*, not open-ended hunting.
  - **Contrast with its sibling on the same persona:** `analysis-specifications` (devils-advocate's
    *specify* skill) is the **adversarial-critique** form (open-ended gap hunting, product-framed "what if"
    questions, no fixed checklist). **So the one `devils-advocate` persona mounts BOTH convention-5 forms
    across two workflows** — adversarial-critique (specify) + mirror-checklist (plan). This corroborates
    (from the P9 side) the P4 finding that the "two forms" are two **skill** forms under one persona, not
    two personas. **Direct input to RQ1.**

### 3. Independence — THE linchpin (validator-side skill); re-confirmed CLEAN

- **No self-grade leak.** Verified directly: producer `technical-analyst` mounts
  `authoring-technical-requirements, patterns-technical-decisions, patterns-entity-modeling,
  patterns-api-contracts` (HIL `agents/technical-analyst.md:46`) — **all author/design, none grade.**
  Reviewer `devils-advocate` mounts review skills only (`analysis-specifications` +, this run,
  `validation-plan-artifacts`). **Disjoint sets, different agents → no agent both produces and grades the
  plan artifacts.**
- **This is the key contrast with the `setup` precedent.** `validation-constitution` LEAKED — it was
  co-mounted with the authoring skills on `principal-architect` (one agent wrote *and* graded). **P9 does
  NOT leak** — the already-ported `devils-advocate` keeps it review-only, and the plan producer is a
  *different* agent. Independence here is already structural, not a defect to fix.
- **Load-bearing port hazard (preserve at transform):** the convention-5 mandate — **never co-mount
  `validation-plan-artifacts` (or any review skill) on `technical-analyst`, and never mount an
  authoring/patterns skill on `devils-advocate`.** Independence is the separate-agent + disjoint-skill
  structure, **not** a prose declaration inside this skill. The P4 re-mount keeps `devils-advocate`
  review-only → preserved.
- **Sequencing:** the re-mount (P4's edit) may only be applied **after** P9 lands at
  `plugins/mochiko/skills/validation-plan-artifacts/` (a live mount of an unported skill dangles →
  `verify-output` Tier-1 FAIL). P9's *own* port is the prerequisite; the mount is P4's body, not P9's.

### 4. Verdict-sink / loop-driver

- P9 **emits** the loop fuel: a classified-issue report + `ready`/`needs-revision`/`critical-gaps` verdict
  (the **same 3-state vocabulary** as `analysis-specifications` — SKILL.md:147-153 — so P1 learns no new
  verdict contract across the two review skills).
- **Consumer:** the **plan lead (P1)**. On `needs-revision`/`critical-gaps` the lead's bounded loop
  re-dispatches the producer (`technical-analyst`). HIL already has this loop ("Iteration {N}") — so this
  is **rehome-EXISTING orchestration onto the mochiko lead**, not add-missing (contrast
  `validation-constitution`, where the verdict-sink + FAIL-loop were *absent* in `setup.md` and had to be
  ADDED). The mochiko upgrade is the **default-FAIL contract + round cap/no-progress bound + human
  acceptance gate**.
- P9's own job is to *emit* the verdict → **kept**; the *consuming* + *looping* → **moved-to-lead (P1)**.

### 5. Sibling / overlap ("look sideways") — where RQ1 + RQ3 live

- **RQ1 — feasibility overlap with the architect (primary, relational).** HIL `plan` runs **two**
  independent reviewers: `principal-architect` (feasibility / cross-artifact contradiction, Phase 1 only —
  plan.md:55,87) and `devils-advocate` (completeness, both phases, via P9 — plan.md:56,88). **P9 already
  reaches into feasibility territory** — its P3 cross-artifact checklist includes **"NFR-design
  feasibility"** ("Can the design as specified meet the NFR targets?"), **"Constraint-design alignment"**,
  "Requirements-decisions alignment", "Integration-contract alignment" (PHASE-CHECKLISTS.md:144-156), and
  P1 includes **"TR-constraint contradictions"** + **"NFR-constraint conflicts"** (:42,62). So the
  completeness-checklist already absorbs cross-artifact consistency **and a slice of feasibility** that
  overlaps the architect's job. Whether mochiko (i) keeps two distinct validators — *checklist* advocate
  (P9) + *adversarial-critique/feasibility* architect, (ii) folds feasibility into the advocate (one
  reviewer, the specify shape — P9 would *absorb* the architect's feasibility checks), or (iii) rehomes
  feasibility onto the generic `validator`, is a **relational decision across P9 ↔ principal-architect ↔
  validator** → `flag-for-reconcile` (detail below). **P9 is the completeness checklist in all three
  options**, so its standalone placement is not blocked on RQ1.
- **RQ3 — P14 `cross-artifact-checklist.md` (relational, P9↔P14).** **Decisive source finding:** nothing
  in HIL references the standalone template — `grep -rln "cross-artifact-checklist"
  human-in-loop/plugins/humaninloop/` returns **zero**. **P9 does NOT consume it.** P9's incremental-mode
  "Consistency Check" points at its **OWN bundled** `references/PHASE-CHECKLISTS.md#cross-artifact-consistency`
  (SKILL.md:97), and the cross-artifact consistency content is **already inside P9** (PHASE-CHECKLISTS.md
  P3 "Final Cross-Artifact Review" :140-161 + the SKILL.md incremental-mode consistency table :122-131).
  The standalone P14 is a **near-verbatim DUPLICATE orphan** (entity-name / requirement-traceability /
  decision-consistency / naming sections + a 2-3 min time budget + escalation rule — all mirrored in P9).
  → Strong **fold-into-P9 / dedupe** signal: P9 should be the single source of the cross-artifact
  consistency checklist; P14 either folds in or is dropped as a duplicate orphan. **Relational (P9↔P14)
  → `flag-for-reconcile`; do not decide solo** (matches context.md P14 row: "standalone vs fold into
  validation-plan-artifacts").
- **No merge-with-producer signal.** The analyst↔advocate relation is a *designed pair*, not an overlap —
  no shared-core merge.
- **`validation-task-artifacts`** (the tasks-cluster sibling) shares the report/severity/verdict shape but
  is a different artifact target → stays deferred (tasks cluster), re-mount when tasks ports. Not this run.
- **Trigger collision (minor, wiring):** generic phrases `"plan quality"`/`"phase review"`/`"artifact
  review"` could collide with other validators → de-collide in the wiring pass, not structural.

### 6. Coupling audit

- **Hardcoded paths:** the **script-invocation examples** use `specs/{feature-id}/...`
  (PHASE-CHECKLISTS.md:171-177) and `validate-openapi.py` (:190; script :326) → `kept-but-rebind`
  (`specs/{feature}/` → mochiko workspace `.mochiko/specs/<feature>/`; the bundled script path
  `scripts/check-artifacts.py` stays relative to the skill).
- **Cross-skill script reference:** `validate-openapi.py` is **not** in P9's bundle — it lives in
  **P8** `patterns-api-contracts/scripts/validate-openapi.py` (which ports this same run). → rebind to the
  ported P8 location or soften the reference (note for transform/reconcile; both port this run so no
  dangling-by-default).
- **Sibling skill cross-refs (When NOT to Use, SKILL.md:24-26):** `humaninloop:validation-task-artifacts`,
  `humaninloop:analysis-specifications`, `humaninloop:validation-constitution` → `kept-but-rebind`
  (`humaninloop:` → `mochiko:`). These are **skill** namespaces, **not** agent names — no deny-list hit.
- **Upstream prerequisite / handoff:** assumes the **plan artifacts already exist** to review
  (requirements.md / constraints-and-decisions.md / nfrs.md after analysis; data-model.md /
  contracts/api.yaml / quickstart.md after design). Explicit handoff edge the **plan lead** wires
  (analyst → artifacts → reviewer) → `moved-to-lead (P1)`.
- **Determinism boundary (where the validator is real vs degenerate):** **MIXED.**
  - *Deterministic slice (keep as a Tier-1 pre-assert):* `check-artifacts.py` — unresolved-marker scan,
    required-section presence, FR-/US- traceability presence, PII-annotation heuristic, entity-name
    cross-file consistency. Greppable, cheap, runs before the model review.
  - *Model-judgment slice (the bulk, → grounded LLM validator):* "is every FR mapped to a TR?", "≥2
    alternatives considered?", "is the rationale convincing?", "does the schema match the data model?",
    "is the integration failure-mode realistic or aspirational?", "can the design meet the NFR target?".
  - The judgment bulk confirms a **real** validator partner; the script is the layered deterministic
    sub-check, not the whole gate.

### 7. Conventions + loop placement + DECOUPLING SCAN

- **(1) Classification:** model-invoked (graded triggers in `description`, no `disable-model-invocation`).
  Stays model-invoked (agent-consumed validator skill).
- **(2) Discoverability (router):** not yet in the `mochiko` router → register as a hint (work-context
  wording), alongside the broadened `devils-advocate` entry. Separate wiring step.
- **(3) Reliable model-invocation (triggers):** description is written in the **user-utterance idiom**
  ("This skill MUST be invoked when the user says 'review research'…") — for an **agent-consumed validator**
  this should be rephrased to **work-context** ("grade the technical-analyst's plan artifacts —
  requirements / data-model / contracts — for completeness, coverage, and cross-artifact consistency").
  Wiring-pass fix (same treatment `validation-constitution` got).
- **(4) Agent↔skill composition:** procedure correctly lives in the skill; persona stays on
  `devils-advocate`. The phase-axis decouple (Check 1) is the composition cleanup.
- **(5) Producer↔validator pairing:** **structurally satisfied** for the plan pair (P9 on the reviewer;
  producer = `technical-analyst`; disjoint skills, different agents). Whether plan runs ONE or TWO
  validators is **RQ1**, not an independence defect of this skill. ✓ (independence) / → flag (architecture).
- **(6) Sound-loop:** the loop P9 sits in has independent validation (P9 is the completeness half);
  done-condition / FAIL-loop / human gate are the **plan lead's (P1)** — rehome-existing + add the
  default-FAIL/bound/human-gate upgrade.

**Decoupling-doctrine verdict — CLEAN (one light token + the phase axis):**

- **Sibling-agent names → NONE.** No "Devil's Advocate", "technical-analyst", "principal-architect",
  "task-architect", "state-analyst", etc. in the body. (The only sibling references are **skill**
  namespaces in "When NOT to Use" — allowed, just rebind `humaninloop:`→`mochiko:`.)
- **"dispatch" → NONE** · **"workflow-agnostic"/independence-by-declaration → NONE.**
- **Injected workflow modes/paths/phases → YES (the one real decouple target):** A0/P1/P2/P3 phase
  identifiers as the organizing axis (20 occurrences) + the incremental "mode" framing. These are the
  `port-with-edits` decouple action — reorganize by artifact-type; externalize phase **selection** to the
  lead.
- **One stray orchestration token:** "flag for supervisor" (:106) → generalize.
- **Net:** the *checks* are keystone-clean (true of any competent completeness reviewer on any plan-style
  artifact set); the *phase labels* are the coupling to cut. No agent-name leak.

---

## The incremental-mode phase-switch — the split (flagged, per run-context)

HIL bundles a "incremental review" capability (SKILL.md:86-143): in Design, **fully review the new design
artifacts** (data-model/contracts/quickstart) **+ a 2-3 min consistency check of the prior analysis
artifacts** (requirements/constraints/nfrs), escalating to a full re-read if 2+ inconsistencies surface.
This must **split**, not move wholesale:

- **Procedure → `folded-into-skill` (stays in P9):** *how* to do a full review of a new artifact set vs a
  bounded consistency check of a prior set, and the "escalate to full re-read on 2+ inconsistencies" rule.
  This is role-intrinsic reviewer discipline — parameterize by `{new-set}` + `{prior-set}` the **caller
  supplies**, not by HIL phase number.
- **Mode-SELECTION → `moved-to-lead (P1)`:** *which* artifacts are "new" (full) vs "prior"
  (consistency-check) — i.e. the "Phase Application" P1/P2 table (:140-143) and "For Phase P2 use
  incremental" (:87). The lead knows the workflow shape (analysis → design) and tells the reviewer which
  set is which. **This is the phase-switch the supervisor injects (`mode: incremental` — plan.md:561);
  it is orchestration, not the skill's body.**

---

## Step 4 — Disposition

**`port-with-edits × standalone`** (matches the contract hypothesis)
**+ `flag-for-reconcile: RQ1`** (reviewer architecture / feasibility overlap — relational)
**+ `flag-for-reconcile: RQ3`** (P14 cross-artifact-checklist fold-in / dedupe — relational).

- **Body = `port-with-edits`** (not `keep-verbatim`): the content is kernel-free and the checks are sound,
  but P9 **earns** localized body edits beyond the always-run wiring floor — chiefly **decoupling the
  phase axis** (reorganize "Review Focus by Phase" → by artifact-type; drop A0/P1/P2/P3 labels;
  externalize the "Phase Application" selection table to the lead), generalizing "flag for supervisor",
  and splitting the incremental-mode selection from its procedure. (Contrast `validation-constitution` =
  `keep-verbatim`, whose only changes were pure wiring-pass items; P9 crosses into earned body work
  because the phase axis is structural, not a one-line scrub.) Not `redesign`: no kernel, approach sound.
- **Structural = `standalone`:** P9 stays **one skill, one home, mounted on the reviewer**
  (`devils-advocate`). It is the completeness checklist in **all three RQ1 options** and independent of
  the P14 decision, so the base placement does not depend on a sibling. The RQ1/RQ3 flags may *layer onto*
  the standalone base (RQ1 could divide the cross-artifact/feasibility checks between P9 and the
  architect's skill; RQ3 could fold P14's duplicate content *into* P9) — but neither moves P9's home. Mirrors
  how the P4 (devils-advocate) assessment landed `standalone + flag-for-reconcile: RQ1`.

## Step 5 — Responsibility trace (complete — every responsibility tagged; no silent drops)

| # | Responsibility (currently held) | Tag |
|---|----------------------------------|-----|
| R1 | **Analysis completeness checks** (FR→TR coverage, orphan TRs, testable criteria, dependency refs, priority, RFC2119; sourced constraints, TR-constraint contradictions, ≥2 alternatives, rationale quality, constraint-decision cross-refs, constitution alignment, IP-XXX + IP-NFR coverage, brownfield consideration, trade-offs; NFR measurability, method, source tracing, NFR-constraint conflicts, category coverage) | **kept** (the checks ARE the body; reorganize under "analysis artifacts", drop the "P1" label) |
| R2 | **Design completeness checks** (entity/attribute coverage, relationships+cardinality+delete behavior, PII/sensitivity classification + details, compliance, retention, encryption, validation rules, state machines, standard fields, traceability; endpoint coverage, schema completeness, error handling, schema-model consistency, integration boundaries + failure modes, auth, examples, naming; integration-guide flow/auth/error/external) | **kept** (reorganize under "design artifacts", drop the "P2" label) |
| R3 | **Cross-artifact / consistency checks** (requirements-decisions alignment, decisions-model consistency, model-contract consistency, sensitivity-contract alignment, integration-contract alignment, FR→TR→entity→endpoint traceability, constitution compliance, infrastructure completeness, **constraint-design alignment**, **NFR-design feasibility**) | **kept** — **+ RQ1 flag** (the feasibility-adjacent checks overlap the architect; how they divide is reconcile's) |
| R4 | **Codebase-discovery review (Phase A0)** — coverage, entity/endpoint detection, collision assessment | **moved-to-other-cluster** (brownfield/discovery) — *reason:* plan-core's `technical-analyst` produces **no** codebase-discovery artifact (plan.md core loop dispatches only P1/P2; brownfield/roadmap track is deferred-by-reference in context.md). Out of plan-core scope → rebind by reference, confirm at reconcile. **Not dropped** |
| R5 | **Severity calibration** (Critical/Important/Minor + classification rules) | **kept** (role-intrinsic; same model `analysis-specifications` uses) |
| R6 | **Verdict emission** (`ready`/`needs-revision`/`critical-gaps` + verdict criteria) | **kept** (loop fuel; same 3-state vocab the lead already consumes from the advocate) |
| R7 | **Review process** (gather context → execute checks → cross-reference → generate report) | **kept** (consumed procedure) |
| R8 | **Issue documentation + report formats** (ISSUE-TEMPLATES.md: individual/table/report) | **kept** (reviewer's issue-report format; distinct from P12 analyst-report / P15 advocate-report) |
| R9 | **Incremental-review PROCEDURE** (full review of new set + bounded consistency check of prior set + escalate-on-2+-inconsistencies) | **folded-into-skill** (parameterize by caller-supplied `{new}`/`{prior}` sets) |
| R10 | **Incremental-review MODE-SELECTION** ("Phase Application" P1/P2 table; "use incremental for P2"; which artifacts are new vs prior) | **moved-to-lead (P1)** — the phase-switch the supervisor injects (`mode: incremental`) |
| R11 | **Anti-rationalization discipline** (Red Flags, Common Rationalizations, "letter = spirit") | **kept** — keystone-true of any honest reviewer; *light* `dedupe` note: dovetails with `loop-discipline`'s anti-rationalization doctrine, but the review-specific red-flags are skill-local; reference-not-restate where it overlaps |
| R12 | **Automated validation script** `check-artifacts.py` (unresolved markers, required sections, FR/US traceability, PII annotations, entity consistency) | **kept** (deterministic Tier-1 pre-assert; stdlib-only, kernel-free) |
| R13 | Script + checklist **path conventions** (`specs/{feature-id}/...`) | **kept-but-rebind** (`specs/{feature}/` → `.mochiko/specs/<feature>/`; bundled `scripts/check-artifacts.py` path stays relative) |
| R14 | **`validate-openapi.py` cross-skill reference** (PHASE-CHECKLISTS.md:190; script :326) | **kept-but-rebind** — lives in **P8** `patterns-api-contracts` (ports this run); rebind to ported location or soften |
| R15 | **Sibling skill cross-refs** ("When NOT to Use": validation-task-artifacts / analysis-specifications / validation-constitution) | **kept-but-rebind** (`humaninloop:` → `mochiko:`; skill namespaces, not agent names) |
| R16 | **Phase-identifier organizing axis** (A0/P1/P2/P3 as the skill's primary structure) | **moved-to-lead (P1)** (phase SEQUENCING) **+ body decouple** (reorganize skill by artifact-type; drop labels) |
| R17 | **"flag for supervisor" routing** (SKILL.md:106) | **moved-to-lead (P1)** (generalize: report the contradiction; the lead routes) |
| R18 | **Classification + trigger description** (model-invoked; user-utterance idiom) | **kept-but-rebind** (stays model-invoked; rephrase triggers to work-context; register in router) |
| R19 | **Upstream handoff** (assumes the reviewed plan artifacts already exist) | **moved-to-lead (P1)** (analyst → {requirements/data-model/contracts} → reviewer edge) |
| R20 | **Verdict CONSUMER + FAIL-loop + done-condition + human gate** (P9 only *emits* the verdict) | **moved-to-lead (P1)** — rehome-EXISTING HIL loop ("Iteration {N}") onto the mochiko lead; **add** the default-FAIL contract + round-cap/no-progress bound + human acceptance gate (the mochiko upgrade) |
| R21 | **Validator-side independence role** (the completeness checklist; must NOT co-mount on the producer) | **kept** (structural) — confirmed disjoint from `technical-analyst`; **port hazard: never cross-mount.** Record-only pairing edge for the rehome map |
| R22 | **Cross-artifact CONSISTENCY checklist content** (bundled PHASE-CHECKLISTS.md P3 + SKILL.md incremental table) | **kept** — **+ RQ3 flag**: the standalone P14 `cross-artifact-checklist.md` duplicates THIS; reconcile decides fold-into-P9 / dedupe / drop-orphan |

**Trace complete — every responsibility carries a tag. No `dropped` tags** (R4 is `moved-to-other-cluster`
with an accepted reason, not a drop). Relational tags (RQ1 on R3, RQ3 on R22) and the
`moved-to-lead` orchestration items are **flagged here, assigned by `reconcile-cluster`.**

---

## Reconcile flags (signals for `reconcile-cluster` — human-gated, contract §4a)

**PRIMARY — `flag-for-reconcile: RQ1` (reviewer architecture / feasibility overlap).**
P9 is the **mirror-checklist** completeness reviewer, but **its P3/P1 cross-artifact checks already
reach into the architect's feasibility/contradiction territory** (NFR-design feasibility,
constraint-design alignment, TR-constraint contradictions, NFR-constraint conflicts,
requirements-decisions alignment, integration-contract alignment). The mochiko shape is a relational
decision across `validation-plan-artifacts` (P9) ↔ `principal-architect` ↔ generic `validator`:
- **(i) two distinct validators** — *mirror-checklist* advocate (P9) + *adversarial-critique/feasibility*
  architect. The **convention-5 two-form case; `plan` is the FIRST cluster to exercise it.** Decide how the
  overlapping cross-artifact/feasibility checks divide (does the architect own NFR-design feasibility +
  constraint contradictions while P9 keeps coverage/consistency?).
- **(ii) fold feasibility into the advocate** (one reviewer, the specify shape) — **P9 would *absorb* the
  architect's feasibility checks** (it already holds most of them).
- **(iii) rehome feasibility onto the generic `validator`.**

**Reconcile input (not a decision):** P9 is the completeness reviewer in all three → its standalone
placement is **not blocked**. And **the one `devils-advocate` persona already mounts BOTH convention-5
forms** (adversarial-critique `analysis-specifications` in specify + mirror-checklist
`validation-plan-artifacts` in plan) — strong evidence the two forms are two *skill* forms under one
persona, not two personas. Human-gated.

**SECONDARY — `flag-for-reconcile: RQ3` (P14 `cross-artifact-checklist.md` fate).**
**Decisive source facts for reconcile:** (a) the standalone P14 template is an **orphan** — nothing in HIL
references it; (b) **P9 does NOT consume it** — P9's consistency check points at its **own bundled**
`PHASE-CHECKLISTS.md` (SKILL.md:97); (c) P9 **already contains** equivalent cross-artifact consistency
content (PHASE-CHECKLISTS.md P3 + SKILL.md incremental table); (d) P14 is a **near-verbatim duplicate**
(entity/requirement/decision/naming + 2-3 min budget + escalation). → **fold-into-P9 / dedupe** (P9 is the
single source) with P14 either absorbed or dropped-as-duplicate-orphan. Relational (P9↔P14) → reconcile
decides; do not decide solo (matches context.md P14 row + RQ3).

**TERTIARY — incremental-mode phase-switch split (record for the rehome map).**
Procedure → `folded-into-skill`; mode-**selection** ("Phase Application" P1/P2 table) → `moved-to-lead (P1)`.
Contract §1 requires the incremental orchestration be rehomed to the lead, not dropped.

**RECORD-ONLY edges (non-blocking — for the rehome map):**
1. **Plan producer↔validator pairing — record it.** Producer `technical-analyst` (P2, authoring/patterns
   skills) ↔ completeness validator `devils-advocate` (P4, review skills incl. P9). Disjoint skills,
   different agents → **independence already structural; no new validator needs constructing for
   completeness** (contrast `setup`, which had to *build* the independent constitution validator). The
   lead (P1) referees + owns verdict/FAIL-loop/handoff. The *feasibility-side* validator of this pair is
   the subject of RQ1.
2. **P9-lands-before-P4-remount sequencing.** P9 must be ported to
   `plugins/mochiko/skills/validation-plan-artifacts/` **before** the P4 re-mount is applied (a live mount
   of an unported skill dangles). Reverse of the specify stub; log in the rehome map.
3. **`validate-openapi.py` cross-skill reference (R14)** — rebind to P8 `patterns-api-contracts` (ports
   this run) or soften.
4. **A0 codebase-discovery scope note (R4)** — confirm it is out of plan-core (deferred-by-reference,
   brownfield/discovery), not silently dropped.

*Soft notes (deferred to transform):* trigger rephrase to work-context (R18); router registration; light
`dedupe` of the anti-rationalization overlap with `loop-discipline` (R11) — reference, not restate.

---

## Output block

```
ASSESSMENT: validation-plan-artifacts (P9)
Class:        skill → branch PLAYS-a-role
Triage:       gate1=y gate2=y gate3=y  [full-lens]
Disposition:  port-with-edits × standalone
              + flag-for-reconcile: RQ1 (reviewer architecture — P9's feasibility overlap)
              + flag-for-reconcile: RQ3 (P14 cross-artifact-checklist fold-into-P9 / dedupe)
Validator form: MIRROR-CHECKLIST (convention-5 objective-acceptance-criteria), discipline-wrapped;
                NOT adversarial-critique (its persona-sibling analysis-specifications is the critique form)
Independence: CLEAN — P9 on the reviewer (devils-advocate); producer (technical-analyst) mounts only
              authoring/patterns skills; disjoint, different agents; NO self-grade leak. Hazard: never co-mount on the producer
Decoupling:   no agent names; injected workflow PHASES (A0/P1/P2/P3, 20 hits, supervisor-injected
              `phase:`/`mode:` params) → decouple-edit (reorganize by artifact-type); "flag for supervisor"
              → generalize. Kernel-free (script = stdlib only)
Trace:        22 responsibilities tagged; 0 dropped (R4 = moved-to-other-cluster w/ reason);
              relational tags (RQ1@R3, RQ3@R22) + moved-to-lead orchestration flagged, assigned by reconcile
Reconcile flags: RQ1 (primary) · RQ3 (P9↔P14) · incremental phase-switch split ·
                 + 4 record-only edges (pairing record, P9-before-P4 sequencing, validate-openapi rebind, A0 scope)
```

---

**Assessment version:** v1 · **Governed by:** `loop-discipline` · **Role:** assess/diagnose only —
no edit, no grade applied. RQ1 + RQ3 are emitted as `flag-for-reconcile`, not decided here.
