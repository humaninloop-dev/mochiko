# Reconcile — cluster `setup` (CORE scope)

Run: `transform-cluster setup` · Phase 2 (reconcile) · Producer: `transform-producer` · Skill: `mochiko:reconcile-cluster`
Reconciled: 2026-06-27 · Governed by `loop-discipline` · Inputs: P1–P10 assessments + contract.md + context.md

> **Reconcile decides; it does not edit.** This is the finalized disposition set + rehome map handed to Phase 3 (`transform-recipes`). **Zero open flags** (see end). Honors the Phase-0 human gate: CORE only; `syncing-claude-md` / `authoring-roadmap` / `evolution-roadmap-template.md` rebind-by-reference only; `brownfield-integration` deferred; kernel-free.

---

## A. Resolved relational verdicts — every flag → one concrete move

Each row resolves a `flag-for-reconcile` from the assessments. Rationale grounded in `loop-discipline` (LD) or the minimalism governor (MG).

### Agenda 1 — INDEPENDENCE SPLIT (P1 / P2 / P5)

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 1a | Self-grade leak: `principal-architect` co-mounts produce (`authoring-/brownfield-constitution`) + grade (`validation-constitution`) on one agent (P1 FLAG-1, P2 f1, P3 F2, P4 F4, P5 F1) | **split** the agent → spin out NEW `constitution-validator` agent; remove `validation-constitution` from `principal-architect` | LD-2: the producer must never grade its own output; a pairing is always a split across two agents, never a co-mount. |
| 1b | `validation-constitution` confers a validator team-role (P5 F2) | **promote** `validation-constitution` onto `constitution-validator` as its **load-bearing tool** | LD-2: the conferred validator role becomes an independent validator's load-bearing skill; promote, don't leave it on the producer. |
| 1c | `validation-constitution` is the shared grader for BOTH `authoring` + `brownfield` outputs (P3 F2, P5 F3) | **pair** — one independent `constitution-validator` grades both producer paths | MG: one validator serves both produced artifacts; no second validator needed. |
| 1d | `principal-architect` confers producer AND validator AND referee (P2 Check 2) | `principal-architect` becomes **producer-only**; the baked-in Feasibility Review is lifted out | LD-2 + MG: one agent confers one team-role. |

**Result:** `principal-architect` (producer) ≠ `constitution-validator` (validator); skills disjoint: `{authoring-constitution, analysis-codebase}` ∩ `{validation-constitution}` = ∅. The setup command dispatches `constitution-validator` as the loop's grading gate.

### Agenda 2 — SOUND-LOOP REHOME (P1) — resolved as the rehome map in §B

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 2a | Unsound done-condition: "no clarifications OR max-3 → proceed" — LLM-controlled, default-DONE (P1 FLAG-2, #18) | **redesign** the loop spine onto the lead: **default-FAIL** done-condition gated on the validator's PASS | LD-1: absence of proof must read as *not done*; an LLM-controlled exit is not a done-condition. |
| 2b | No independent-validation gate anywhere (P1 Check 3, P2 f4, P3 F3, P5 F4) | **add** a produce→validate→repeat FAIL-loop with `constitution-validator` (rehome-orchestration: a MISSING gate to ADD, not relocate) | LD-2: independent validation is non-negotiable; the real transform is installing the gate the original never had. |
| 2c | `max-3` bound proceeds-as-done on cap; no no-progress exit; no kill-switch (P1 #19) | **rehome** onto the lead's bounded loop: on-cap **escalate (FAIL)**; add no-progress exit + STOP kill-switch | LD-3: all four iteration guards; running out of rounds is escalate, never done. |
| 2d | No human gate on constitution acceptance (P4 §7, P5 F4) | **add** a named human **acceptance gate** on the validated constitution (rehome) | LD-4: every contract names where the human validates; presence non-negotiable. |

### Agenda 3 — MERGE DECISION (P3 / P4)

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 3 | `brownfield-constitution` extends (defers to, doesn't duplicate) `authoring-constitution` — thin variant over a shared core (P3 F1, P4 F1) | **merge-into-sibling** — `authoring-constitution` **absorbs** brownfield as a `mode: greenfield \| brownfield` branch; keep only the variant-unique slice | Reconcile rule: a thin variant over a genuinely shared core merges into one skill with branches. brownfield *defers* the Three-Part Rule / RFC-2119 / SYNC-IMPACT to authoring (genuine shared core, not look-alike) → merge earned; MG collapses the duplicated trigger surface + substrate. |

**Variant-unique slice preserved on merge (`moved-to-sibling-skill`):** Essential Floor doctrine (`ESSENTIAL-FLOOR.md`), Emergent Ceiling doctrine (`EMERGENT-CEILING-PATTERNS.md`), brownfield constitution structure + Evolution Notes, brownfield quality checklist, brownfield-specific Common Mistakes. **Shared-core refs** (Three-Part Rule / RFC-2119 / SYNC-IMPACT) → `dedupe` (one source in authoring, branch references it).

**Resulting file shape:**
- `skills/authoring-constitution/SKILL.md` — gains a brownfield branch (greenfield path = core; brownfield path = the variant slice). One `description` carries graded triggers for both paths → resolves trigger de-collision (P3 F4, P4 F3) **within one skill**.
- `skills/authoring-constitution/references/` — `RECOMMENDED-PATTERNS.md`, `RFC-2119-KEYWORDS.md`, `SYNC-IMPACT-FORMAT.md` (core) **+ ESSENTIAL-FLOOR.md, EMERGENT-CEILING-PATTERNS.md** (moved in from brownfield).
- `skills/brownfield-constitution/` — **NOT created** (dissolved into authoring).

### Agenda 4 — ESSENTIAL-FLOOR DEDUPE (P4 / P6)

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 4 | Essential Floor appears as **require-floor** (brownfield-constitution) AND **assess-status** (analysis-codebase) (P4 F2, P6 F2) | **dedupe** the *definition*: canonical home = `authoring-constitution/references/ESSENTIAL-FLOOR.md` (the merged brownfield branch); `analysis-codebase` **references** it for its present/partial/absent assessment instead of re-defining the four categories | MG: one source of truth for the four NON-NEGOTIABLE categories. The floor is a *governance requirement* → owned by the constitution skill; analysis *assesses against* it. Two distinct **uses** (require vs assess) both kept; only the **definition substrate** collapses to one. |

### Agenda 5 — STATE CARRIER (P8 / P1)

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 5 | `constitution-context-template.md` is the old supervisor's separate ephemeral state-carrier; P1 already inlines a richer context block → cluster must end with ONE mechanism (P8 F-P8-1 **BLOCKING**, F-P8-2) | **absorb-into-lead** — fold the handoff/loop-state into the mochiko setup command lead (in-session dispatch context + the `.mochiko/memory/` workspace, mirroring how `transform-cluster.md` carries state via its `.mochiko/transform/<cluster>/` workspace). P8 body = **drop** as a standalone template. | MG + Workflow-first: two state mechanisms is the exact dedupe to avoid; mochiko is kernel-free / Workflow-first and carries loop state in-session + workspace, not in a separate ephemeral template. **ONE mechanism = the lead.** |

**P8 responsibilities re-home:** supervisor instructions / clarification log / iteration tracking / mode-routing signal → `moved-to-lead` (§B). Output-location binding → `kept-but-rebind` (lives in the lead's dispatch instructions). The separate-file lifecycle, the "add custom sections" affordance, and the standalone-registration → `dropped + reason` (no separate file exists after absorb).

### Agenda 6 — MODE-SCOPING (P6)

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 6 | `analysis-codebase` has modes beyond setup: Context (standalone report), Brownfield (JSON inventory → collision), Setup-Brownfield (P6 F1) | The setup port carries **only the Setup-Brownfield slice + `detect-stack.sh` + the Context-gathering sub-procedure as folded into setup-brownfield**. **Brownfield/collision mode** (JSON inventory) → `moved-to-other-cluster` (spec/plan); **standalone Context-mode report** → `moved-to-other-cluster` (constitution-context cluster). | Workflow-first: the setup workflow reveals it needs the markdown analysis (P9) + the deterministic script, not the JSON collision mode. Port the cluster's slice; the other modes land with their own clusters — not silently dropped. |
| 6b | `analysis-codebase` emits a reviewable `codebase-analysis.md` with no independent validator — independent analysis-validator vs human checkpoint? (P6 F3) | **No new analysis-validator.** The **Phase-2 human checkpoint (Confirm/Edit/Reject) is the deliberate gate**; the factual half is grounded by `detect-stack.sh` (deterministic baseline). | LD-2/LD-4 + MG: `codebase-analysis.md` is an *intermediate* input, not the cluster deliverable; the constitution (the deliverable) gets the independent validator. Adding a second validator agent for an intermediate artifact is over-engineering; the named human gate + deterministic baseline is the deliberate, sound gate. **One new validator agent in the cluster, not two.** |

### Agenda 7 — CROSS-CLUSTER & ORPHANS

| # | Flag (source) | Resolved move | One-line rationale |
|---|---|---|---|
| 7a | Roadmap (`authoring-roadmap`, `evolution-roadmap-template.md`) + CLAUDE.md sync (`syncing-claude-md`) (P1 FLAG-3, P2 #6/#7, P3, P4) | **moved-to-other-cluster**, **rebind-by-reference only** = a documented stub, **not a live skill mount**. Phase-4 roadmap orchestration + CLAUDE.md-sync reporting are removed from the setup command this run; the two skills are removed from `principal-architect`'s live `skills:` list (re-mount when those clusters port). The constitution-template's `## CLAUDE.md Synchronization` **section stays as constitution content** (kept) — only the operational sync **action** is moved-to-other-cluster. | Phase-0 gate: cross-cutting, not ported this run. A live `mochiko:` mount of an unported skill would dangle (a `verify-output` Tier-1 FAIL); a reference stub keeps it clean. |
| 7b | `approved-domain-deps.md` referenced by P7 (constitution-template L225) + P4 (EMERGENT-CEILING) — not in port set, not in cross-cutting moved list (P7 F-P7-1, P4 coupling note) | **drop-with-reason** — soften the Domain-Layer-Note to a plain-language reference carrying **no dangling plugin path**; `approved-domain-deps.md` is **not ported** | MG + no-dangling-path: deferring a live `${CLAUDE_PLUGIN_ROOT}/templates/approved-domain-deps.md` path to a file that won't exist this run is a coupling FAIL; dropping the path (keeping the intent as prose) is the minimal clean fix. **HUMAN-GATED (dropped).** |
| 7c | `codebase-inventory-schema.json` (P10) — orphan: referenced by nobody in HIL; its `collision_risks`/`spec_item` shape is the contract for the Brownfield/collision mode (spec/plan), not setup (P10 F-P10-1, F-P10-2) | **moved-to-other-cluster** — port with spec/plan alongside the Brownfield/collision mode (agenda 6); **not produced this run** | Workflow-first + MG: after mode-scoping moved the Brownfield/collision mode to spec/plan, P10 has **zero consumer in setup-core** (setup uses the markdown P9). Porting it here lands a dormant orphan. Move it with its consumer. **Deviates from the Phase-0 in-scope list → surfaced at the human gate as a scope refinement.** |

---

## B. REHOME MAP — dissolving HIL setup supervisor's orchestration → new home

The HIL `commands/setup.md` markdown supervisor dissolves onto the mochiko setup command **lead**. Every orchestration responsibility (from P1 + P8 traces + the MISSING gates surfaced by P2/P3/P4/P5) lands explicitly. **No kernel reintroduced** — all homes are the lead command, the new validator agent, the producer agent, a skill, or an accepted drop.

| Orchestration responsibility | Re-homes to |
|---|---|
| Ensure workspace dir (`.mochiko/memory`) | **lead** (deterministic step) |
| Brownfield detection (`detect-stack.sh` + source-file count) | **lead** (Phase-0 deterministic step; script lives in `analysis-codebase`, called by the lead) |
| Constitution-mode determination (create vs amend) + brownfield/greenfield heuristic | **lead** |
| Phase-0 mode-selection **human gate** | **lead** (named human gate — keep) |
| Phase sequencing / mode routing (brownfield/greenfield/amend) | **lead** (sequencing/dispatch) |
| Dispatch `principal-architect` (producer) for analysis + constitution | **lead** (dispatch) + `kept-but-rebind` (`humaninloop:` → `mochiko:` namespace) |
| Existence asserts (`test -f …`) | **lead** (deterministic existence assert — explicitly **NOT** the quality gate) |
| Phase-2 analysis-checkpoint **human gate** (Confirm/Edit/Reject) | **lead** (named human gate — keep; the deliberate gate for `codebase-analysis.md`, agenda 6b) |
| Phase-2 edit-loop (corrections → re-analyze) | **lead** bounded loop (round cap + no-progress; escalate-don't-die) |
| Mid-loop instruction injection / supervisor instructions (P8 R2) | **lead** (dispatch context, in-session) |
| Clarification loop (Phase-3 agent asks user) | **lead** — kept as a human-**clarification** gate, **NOT** the done-condition |
| **MISSING: independent constitution-validation gate** | **NEW `constitution-validator` agent** (dispatched by the lead; the loop's grading gate) |
| **MISSING: default-FAIL done-condition** | **lead** (constitution starts FAIL; flips only on validator PASS) |
| Old done-condition ("no-clarifications/max-3 → proceed") | **dropped + reason** (LLM-controlled, default-DONE — LD-1 violation) → replaced by the default-FAIL gate above |
| `max-3` iteration bound | **lead** bounded loop — on-cap **escalate (FAIL)**, not proceed; add no-progress exit + STOP kill-switch (LD-3) |
| **MISSING: human acceptance gate on validated constitution** | **lead** (NEW named human gate — ADD; LD-4) |
| State-carrier / context handoff (P8 + P1's inlined context) | **lead** — ONE mechanism: in-session dispatch context + `.mochiko/memory/` workspace (P8 absorbed; no separate template) |
| Iteration tracking / round count (P8 R4) | **lead** (bounded-loop round cap) |
| `mode: create\|amend` routing signal (P8 R7) | **lead** (folds into the lead's dispatch branch) |
| Authoring requirement prose embedded in dispatch prompt (P1 #15) | **folded-into-skill** (`authoring-constitution`) → **dedupe** (not duplicated in the supervisor prompt) |
| Per-phase report-format specs (P1 #16) | **folded-into-skill** (producer's output contract) |
| Cross-artifact Feasibility Review procedure (P2 #8) | **moved-to-other-cluster** (referee role over specify/plan artifacts; out of setup-core — lifted out of the `principal-architect` persona) |
| Essential Floor Knowledge table baked in persona (P2 #9) | **dedupe** (canonical home = `authoring-constitution` skill, not the persona) |
| Phase-4 evolution-roadmap orchestration (P1 #21) | **moved-to-other-cluster** (roadmap cluster — rebind-by-reference stub) |
| CLAUDE.md sync reporting (P1 #22) | **moved-to-other-cluster** (`syncing-claude-md` — rebind-by-reference stub) |
| Phase-5 finalize / user report (P1 #23) | **lead** + `kept-but-rebind` (`/humaninloop:specify` → `/mochiko:specify` reference stub) |
| Phase-5 cleanup gate ("delete context file?") (P1 #24, P8 R5) | **dropped + reason** (no separate ephemeral context file after absorb-into-lead — nothing to clean up) |
| State recovery (resume-point detection) (P1 #25) | **lead** — kept (simplified): resume from `.mochiko/memory/` workspace evidence, not a context-file `phase` field |
| Supervisor behaviors / mode-awareness (P1 #26) | **lead** |

---

## C. NEW primitives required

| Name | Type | Why | Spec |
|---|---|---|---|
| **`constitution-validator`** | agent | Independent validator partner from the §A split/promote — receives the grading responsibility so the producer (`principal-architect`) never grades its own constitution | See full spec below |
| (no new skill) | — | The promote re-uses the ported `validation-constitution` skill as the validator's load-bearing tool; the Feasibility Review procedure is `moved-to-other-cluster`, not created this run | — |

### `constitution-validator` agent — full spec (NET-NEW, product of the split + promote)

```
File:           plugins/mochiko/agents/constitution-validator.md
name:           constitution-validator
description:    Skeptical, independent reviewer who grades a DRAFTED constitution against the
                validation-constitution quality checklist — three-part-structure check, anti-pattern
                scan, placeholder scan, quantification enforcement, semantic version-bump call —
                and emits a binary PASS/FAIL + fix list + version bump. Reads the constitution
                artifact itself, never the author's say-so. Defaults to FAIL. NEVER authors a
                constitution. (graded examples per the transform-validator pattern)
model:          opus            (mirrors transform-validator; highest the judgment task allows)
color:          red             (validator convention)
skills:         validation-constitution        ← load-bearing tool (promoted from P5)
Classification: agent (dispatched by the setup command lead). The mounted skill
                `validation-constitution` stays model-invoked. Independence:
                  - agent ≠ principal-architect ✓
                  - skills {validation-constitution} ∩ producer skills {authoring-constitution,
                    analysis-codebase} = ∅ ✓
Role:           Reads .mochiko/memory/constitution.md (the producer's draft), runs
                validation-constitution, returns VALIDATION RESULT [PASS/FAIL] + issues-requiring-fix
                + version bump. The lead consumes the verdict and drives the bounded FAIL-loop.
                Tier-2 grounded validation (Check 6 confirms the grader is real, not a degenerate
                assert; the deterministic placeholder scan runs as a cheap pre-assert inside it).
```

---

## D. FINAL PRIMITIVE SET — Phase 3 artifacts (every disposition RESOLVED, no flags)

| # | Mochiko artifact | Action | Body × Structural (final) | Notes |
|---|---|---|---|---|
| P1 | `commands/setup.md` | **CREATE** | **redesign × standalone** (lead; absorb-target for the §B rehome map + P8 state-carrier) | Loop spine rebuilt to LD: default-FAIL done-condition, produce→validate→repeat with `constitution-validator`, round cap + no-progress + STOP, escalate-on-cap, named human acceptance gate. Phase **content** ported `port-with-edits`. **GATED (redesign + drops).** |
| P2 | `agents/principal-architect.md` | **CREATE** | **port-with-edits × split** | Producer-only. `skills: authoring-constitution, analysis-codebase`. Remove `validation-constitution` (→ P-NEW), remove `brownfield-constitution` (merged into authoring), remove `syncing-claude-md` + `authoring-roadmap` (moved-to-other-cluster, re-mount later). Lift out Feasibility Review (moved-to-other-cluster); dedupe Essential-Floor table. |
| P-NEW | `agents/constitution-validator.md` | **CREATE (net-new)** | n/a (new agent) × **split-product / promote-target** | `skills: validation-constitution`. Full spec in §C. **GATED (promote).** |
| P3 | `skills/authoring-constitution/` | **CREATE** | **port-with-edits × standalone (merge absorber)** | Gains brownfield branch + `ESSENTIAL-FLOOR.md` (canonical home, agenda 4) + `EMERGENT-CEILING-PATTERNS.md`. Reframe description to work-context + both-path graded triggers; rebind SYNC-IMPACT example paths to `.mochiko/`. |
| P4 | `skills/brownfield-constitution/` | **NOT created** | **port-with-edits × merge-into-sibling** (→ P3) | Variant-unique slice → `moved-to-sibling-skill` (into P3's brownfield branch); shared-core refs → `dedupe`. |
| P5 | `skills/validation-constitution/` | **CREATE** | **keep-verbatim × promote** (onto P-NEW) | Body mochiko-clean; reframe triggers to work-context + register in router (wiring only). Becomes `constitution-validator`'s load-bearing tool. **GATED (promote).** |
| P6 | `skills/analysis-codebase/` | **CREATE** | **port-with-edits × standalone** | Setup-Brownfield slice + `scripts/detect-stack.sh` (keep-verbatim) + Context-gathering sub-procedure. Brownfield/collision (JSON) mode + standalone Context report → `moved-to-other-cluster`. Essential-Floor definition references P3's canonical `ESSENTIAL-FLOOR.md`. Rebind paths + template refs; rephrase triggers. |
| P7 | `templates/constitution-template.md` | **CREATE** | **keep-verbatim × standalone** | Wiring pass only. `approved-domain-deps.md` path-reference **dropped** in the path-rebinding step (agenda 7b). **GATED (dropped reference).** |
| P8 | `templates/constitution-context-template.md` | **NOT created** | **drop × absorb-into-lead** (→ P1) | State-carrying folded into the lead (agenda 5). **GATED (absorb + drops).** |
| P9 | `templates/codebase-analysis-template.md` | **CREATE** | **keep-verbatim × standalone** | Wiring pass only. Clean. |
| P10 | `templates/codebase-inventory-schema.json` | **NOT created (this run)** | **keep-verbatim × moved-to-other-cluster** | Ports with spec/plan + the Brownfield/collision mode (agenda 7c). **Scope-refinement surfaced at human gate.** |
| — | `skills/mochiko/SKILL.md` (router) | **EDIT** | wiring-pass | Register: `/mochiko:setup` (user-invoked hint); agents `principal-architect` + `constitution-validator`; skills `authoring-constitution`, `validation-constitution`, `analysis-codebase`; templates P7, P9. (Discoverability floor; a `verify-output` Part-A item.) |

**Net Phase-3 artifacts:** 1 command (P1), 2 agents (P2 + P-NEW), 3 skills (P3 absorbing P4, P5, P6), 2 templates (P7, P9), 1 router edit. **Not produced:** P4 (merged), P8 (absorbed), P10 (moved-to-other-cluster).

---

## E. RE-EMITTED TRACES — every responsibility carries a concrete relational tag

Relational tags now **assigned** (no `flag-for-reconcile` remains). Only deltas from the assessments' already-concrete tags are re-stated; the full per-primitive traces in the assess files stand, with these flags now resolved:

**P1 setup command** — flagged items resolved: #18 old done-condition → **dropped + reason**; #19 max-3 bound → **moved-to-lead** (escalate-on-cap); #20 MISSING independent validation → **moved-to-validator** (`constitution-validator`); #24 cleanup gate → **dropped + reason**; #21 roadmap / #22 claude-md → **moved-to-other-cluster**. All 26 tagged.

**P2 principal-architect** — #5 validation-constitution → **moved-to-validator** (`constitution-validator`); #3 brownfield-constitution → **dedupe / moved-to-sibling-skill** (merged into authoring); #6 syncing-claude-md / #7 authoring-roadmap / #8 feasibility review → **moved-to-other-cluster**; #9 essential-floor table → **dedupe** (canonical home P3). All 10 tagged.

**P3 authoring-constitution** — shared core → **kept** (absorber); brownfield's variant slice arrives as **moved-to-sibling-skill**; Essential-Floor definition → **dedupe** (canonical home here); validation pointer → **kept-but-rebind**; sync-execution pointer → **moved-to-other-cluster**; enforced validation gate + FAIL-loop → **moved-to-lead**. All tagged.

**P4 brownfield-constitution** — Essential Floor / Emergent Ceiling / brownfield structure / Evolution Notes → **moved-to-sibling-skill** (into P3); shared-core refs → **dedupe**; evolution-roadmap + authoring-roadmap refs → **moved-to-other-cluster**; analysis handoff + validate loop + missing human gate → **moved-to-lead**; `approved-domain-deps.md` ref → **dropped + reason**. All tagged.

**P5 validation-constitution** — validator team-role + independence boundary → **moved-to-validator** (`constitution-validator`, via **promote**); verdict-sink + FAIL-loop + done-condition + human acceptance gate → **moved-to-lead** (MISSING gates ADDED); sibling refs + classification → **kept-but-rebind**. All 12 tagged; 0 dropped.

**P6 analysis-codebase** — R3 standalone Context report + R4 Brownfield/JSON collision mode → **moved-to-other-cluster**; R5 setup-brownfield analysis → **kept-but-rebind** (the setup responsibility); R6 essential-floor → **kept** (assess-use) + **dedupe** (definition → P3); R8 codebase-inventory-schema ref → **moved-to-other-cluster** (with R4's mode); R11 independence gap → **moved-to-lead** (Phase-2 human checkpoint = deliberate gate); R1 detect-stack.sh → **kept**; R12 related-skills → **kept-but-rebind** (brownfield ref → authoring's brownfield branch). All 12 tagged.

**P7 constitution-template** — R1–R7, R9 → **kept**; R6 operational sync → **moved-to-other-cluster**; R8 `approved-domain-deps.md` ref → **dropped + reason**; R10 discoverability → **kept-but-rebind**. All tagged.

**P8 constitution-context-template** — R1 transport / R2 supervisor-instructions / R3 clarification log / R4 iteration / R7 mode-routing → **moved-to-lead**; R6 output-location binding → **kept-but-rebind**; R5 ephemeral lifecycle / R8 custom-sections affordance / R9 standalone-registration → **dropped + reason**. All 9 tagged.

**P9 codebase-analysis-template** — R1–R7 → **kept**; R8 discoverability → **kept-but-rebind**. All tagged. Clean.

**P10 codebase-inventory-schema** — R1–R8 → **kept** (re-tag of disposition: **moved-to-other-cluster** — ports intact with spec/plan); R9 "enforces nothing / orphan" → **resolved by the move** (the contract lands with its real consumer; no longer a drop). All tagged.

### Dropped responsibilities (explicit — these go to the human gate)

| Dropped responsibility | Reason (for lead acceptance) |
|---|---|
| P1 #18 — old done-condition ("no clarifications OR max-3 → proceed") | LLM-controlled + default-DONE; violates LD-1. Replaced by a default-FAIL done-condition gated on the independent validator. |
| P1 #24 / P8 R5 — Phase-5 cleanup gate + ephemeral context-file lifecycle | After absorb-into-lead there is **no separate context file** to create/delete; state lives in-session + `.mochiko/memory/` workspace. Nothing to clean up. |
| P8 R8 — "supervisor may add custom sections" affordance | A property of an editable standalone state file; provided instead by the lead's own dispatch context. No artifact to carry it. |
| P8 R9 — standalone discoverability/registration | No standalone template survives the absorb; nothing to register. |
| P7 R8 / P4 — `approved-domain-deps.md` path reference | Out-of-run template, not in scope and not in the cross-cutting moved list. Path softened to prose to avoid a dangling `${CLAUDE_PLUGIN_ROOT}` reference (would be a coupling FAIL). |

No other responsibility is dropped. Every non-dropped responsibility has a concrete landing (`kept` / `kept-but-rebind` / `moved-to-lead` / `moved-to-validator` / `moved-to-sibling-skill` / `dedupe` / `moved-to-other-cluster`).

---

## F. HUMAN-GATED dispositions (per contract §4: redesign / absorb-into-lead / promote / dropped)

Present these at the Phase-2 human gate before Phase 3 applies them:

1. **P1 `redesign`** — rebuild the setup loop spine (default-FAIL done-condition, independent-validation gate, escalate-on-cap, human acceptance gate). *Accept / override / send-back.*
2. **P-NEW `constitution-validator` + P5 `promote`** — spin out the new independent validator agent; promote `validation-constitution` onto it; `principal-architect` becomes producer-only. *Accept the new agent + the split.*
3. **P8 `absorb-into-lead` + drops** — dissolve the constitution-context-template into the lead; ONE state mechanism. *Accept the absorb + the R5/R8/R9 drops.*
4. **Dropped reasons** (the §E table) — old done-condition, cleanup/ephemeral-file lifecycle, custom-sections affordance, standalone registration, `approved-domain-deps.md` reference. *Accept or reject each reason.*

**Scope-refinement (surface alongside, not a §4 contract gate):**
5. **P10 `moved-to-other-cluster`** — declines to port `codebase-inventory-schema.json` this run (no consumer in setup-core after mode-scoping). Deviates from the Phase-0 in-scope list → *accept the refinement, or override to port-it-dormant.*

**Not gated by §4 (presented in the reconciled plan, applied without explicit accept):**
- P3/P4 **merge-into-sibling** (structural consolidation; not in the §4 enumerated set).
- P2 **split** wiring (the producer-only `skills:` cleanup; realizes the gated promote).
- Mode-scoping moves of P6's other modes to other clusters; roadmap + claude-md rebind-by-reference stubs.

---

## G. Open flags

**NONE.** Every `flag-for-reconcile` (P1 FLAG-1/2/3; P2 f1–4; P3 F1–4; P4 F1–5; P5 F1–5; P6 F1–3; P7 F-P7-1; P8 F-P8-1/2; P10 F-P10-1/2) is resolved to a concrete move in §A, and every orphaned/missing responsibility has a home in §B/§E. Independence holds (producer ≠ validator, disjoint skills). Kernel-free maintained (every rehome lands on lead / validator / producer / skill / accepted-drop — no Python/MCP/DAG/catalog). Reconcile done-condition met: **zero open flags, zero homeless responsibilities.**
