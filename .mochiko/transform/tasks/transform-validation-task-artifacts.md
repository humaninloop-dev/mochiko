# TRANSFORM — P4 `validation-task-artifacts`

**Run:** `/mochiko:transform-cluster tasks` · **Phase 3 (transform, per-primitive · Wave 1 content)** · **Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes` · **Date:** 2026-07-01
**Consumes:** `assess-validation-task-artifacts.md` (P4, 31-item trace) + `reconcile.md` (Flag 4 scaffolding-dedupe + CONFIRMED-DISJOINT division of labor · Flag 5b mirror-P3 contract · Flag 7.4 parked-checks defer · P4 trace L230–236 · RQ-A = **Branch A** accepted at Phase-2 human gate).
**Model:** the ported sibling `plugins/mochiko/skills/validation-plan-artifacts/` (same author-agent `devils-advocate`; same "gap-finding INPUT + recommended 3-state verdict, the LEAD owns the clearing verdict" doctrine; same Critical/Important/Minor convention; same `advocate-report-template` deliverable reference).
**Applies:** the finalized disposition — reconcile + the human gate did the deciding; this run only edits + wires. Grading is `verify-output` (a **different** agent); this producer **did NOT grade** its own output.

---

TRANSFORM: validation-task-artifacts (P4)
Applied:   port-with-edits × standalone + wiring-pass (all 6)
Artifacts: plugins/mochiko/skills/validation-task-artifacts/{SKILL.md, references/ISSUE-TEMPLATES.md, references/PHASE-CHECKLISTS.md} (created)
New partners: none (standalone — no split/promote/pair; the task-architect ↔ devils-advocate(validation-task-artifacts) pair already exists and is independent)
Wiring:    classification=model-invoked (no disable-model-invocation) · router=Wave-2 obligation RECORDED (not applied) · triggers=work-context MUST/SHOULD, boundary-disclaimed vs validation-plan-artifacts · rebinds=[humaninloop:→mochiko: sibling refs · 1 illustrative specs/→.mochiko/specs/] · decouple=2 agent-name DROPS + Task-Architect→role + phase→artifact-type · single-source=advocate-report-template (report shape) + loop-discipline (anti-rationalization) referenced, not restated
Form:      MIRROR-CHECKLIST — named checks + per-check severity + verdict mechanically from counts (recommended 3-state) — kernel-free (no script; none owed, minimalism)

---

## Body edits applied (`port-with-edits`)

1. **Verdict ownership reframed (assess #4).** The skill emits a **gap-finding report + a RECOMMENDED 3-state verdict** (`ready` / `needs-revision` / `critical-gaps`), NOT a clearing PASS/FAIL. SKILL.md Overview + Verdict Criteria state the `/mochiko:tasks` **lead owns the clearing decision and routes the revision loop**; "Return to **Task Architect**" → "Return to the **responsible producer**" (Issue Classification, ISSUE-TEMPLATES severity table, verdict tree). Matches the sibling's framing (`moved-to-lead`).
2. **Decouple — 2 DROPS + role phrasing (assess #5, #12, #20, #25).** Dropped both sibling-agent namings: "provides phase-specific review criteria for the **Devil's Advocate**" (SKILL Overview) and "Used by the **Devil's Advocate**" (PHASE-CHECKLISTS header). "**Task Architect**" → role ("the producer" / "the responsible producer" / "the task-structuring producer"); "Wait for Task Architect…" → "wait for the producer to finish the artifact". No "dispatch" / "workflow-agnostic" tokens existed. Independence now stated by **role**, not by naming an agent.
3. **Phase → artifact-type reframe (assess #6; RQ-A Branch A).** The DAG-injected "Review Focus by **Phase**" framing is reframed to **artifact-type scope** — "the caller supplies which task artifacts are in scope" (SKILL.md, mirroring the sibling's "Review Focus by Artifact Type"). **Branch A honored:** the two-checklist structure is KEPT (separate Mapping + Tasks checklists + a Cross-Artifact review) — the coupling *framing* shifted, the proven two-phase *structure* did not collapse.
4. **Scaffolding dedupe (Flag 4).** The two embedded report templates in `ISSUE-TEMPLATES.md` ("Mapping Review Report" / "Tasks Review Report") are **REPLACED** with a single reference to `mochiko:advocate-report-template` (an "Assembled Report" pointer — "do not re-embed the report shape"). KEPT: the task-specific **Checks-Executed tables** (extracted from inside the old templates, now standalone Mapping / Tasks / Cross-Artifact ledgers), the `TM-`/`TT-`/`TX-` issue-ID conventions, and the verdict decision tree.
5. **Division of labor recorded (Flag 4 — CONFIRMED DISJOINT).** A new **Scope** section + the **When-NOT-to-Use** + the **description** record the mutual boundary vs `validation-plan-artifacts`: task artifacts {`task-mapping.md`, `tasks.md`} + vertical-slice/TDD/traceability checks **vs** plan artifacts {requirements, constraints-and-decisions, NFRs, data-model, contracts}. Only shared surface = scaffolding (severity taxonomy · 3-state verdict · issue-doc format · report shape), all single-sourced. **NO structural merge** — stated explicitly ("Confirmed complementary — no structural merge"). Completes the reciprocal of the sibling's L45 pointer.
6. **Mirror-P3 contract honored (Flag 5b).** Every check corresponds to a rule `mochiko:patterns-vertical-tdd` teaches — vertical-slice quality, TDD test-first **ordering**, cycle definition/sizing, `**TEST:**`-task presence, story→cycle→task traceability, marker/file-path/task-ID presence. SKILL.md → Key Principles + PHASE-CHECKLISTS intro state the mirror explicitly ("the reviewer checks exactly what the producer is taught to build"). The `**TEST:**` check is KEPT as a **design-time presence/structure** check (its author is the producer/P3; it is NOT the deferred runtime executor — PHASE-CHECKLISTS' Verification Task Validation says so explicitly).
7. **Parked checks kept dormant (Flag 7.4 / assess #26).** IP-XXX coverage · platform-app ordering · deployment cycle · `[EXTEND]`/`[MODIFY]` brownfield markers · constraint-task traceability stay **present but dormant** (tagged *dormant/parked* in both references + noted in SKILL.md), in lock-step with the producer's parked side. `kept (dormant/parked)`, **not dropped**.
8. **Description — work-context graded triggers.** Rewrote from the HIL user-utterance idiom ("when the user says 'review task mapping'…") to the sibling's **work-context** MUST/SHOULD form: grade a producer's task artifacts → severity-classified gap report + 3-state verdict; the task-artifact half of the producer↔validator pair; explicit disjoint boundary vs `validation-plan-artifacts`; defaults to FAIL; independent validator, never the author.

## Contracts honored (handed on from reconcile)

- **Flag 4 dedupe → `advocate-report-template`:** embedded report shapes removed; template referenced (5 refs). Verified: zero re-embedded "Advocate Report: … Review" skeletons.
- **Flag 4 disjoint boundary:** recorded in Scope + When-NOT + description; "no structural merge" explicit.
- **Flag 5b mirror:** every P4 check maps to a P3 authoring rule; the two must be edited together (noted for `verify-output` + the P3 transform).
- **RQ-A Branch A:** two-phase structure (Mapping + Tasks + Cross-Artifact) kept; not collapsed to a single pass.
- **Kernel-free:** all three files scanned — no `humaninloop_brain` / `hil-dag` / MCP / DAG / catalog / `.humaninloop/` / script tokens. No pre-assert script built (none owed; minimalism — the sibling's `check-artifacts.py` is plan-specific).

## Convention-wiring pass (all 6)

1. **Classification — model-invoked.** No `disable-model-invocation`; agent-consumed validator skill (default). ✓
2. **Router registration — Wave-2 obligation RECORDED, not applied** (per instruction: do not edit the router this run — the whole Tasks-cluster router section lands once in Wave 2, verified absent). **Row to add (verbatim, when the Tasks section lands):**
   > `validation-task-artifacts` (skill) | independently grading a producer's **task artifacts** (`task-mapping.md`, `tasks.md`) — vertical-slice quality / TDD test-first ordering / `**TEST:**`-task presence / story→cycle→task traceability → severity-classified gaps + recommended 3-state verdict (run by an independent validator, never the author). Boundary: **disjoint** from `validation-plan-artifacts` (plan artifacts) — complementary, no overlap.

   Plus (Wave 2, recorded): the `devils-advocate` agent row broadens to "**cross-workflow** — specify critic + plan completeness reviewer + **tasks task-artifact reviewer**."
3. **Trigger phrasing — work-context, boundary-disclaimed.** MUST/SHOULD graded triggers describing the review *work*; explicitly disclaims plan-artifact completeness (points at `validation-plan-artifacts`). Same treatment the sibling got.
4. **Path rebinding.** No `.humaninloop/` paths existed. Sibling skill cross-refs `humaninloop:` → `mochiko:` (When-NOT pointers to `validation-plan-artifacts` / `analysis-specifications` / `validation-constitution`). One illustrative `specs/042-…` example path → `.mochiko/specs/042-…`. Artifact names it reviews (`task-mapping.md` / `tasks.md`) are legitimate, kept.
5. **Decouple persona/skill — clean.** 2 agent-name DROPS + Task-Architect→role + phase→artifact-type reframe (see edit #2/#3). Grep confirms zero `devil'?s.advocate` / `task.architect` tokens. Independence stated by role.
6. **Single-source / de-duplicate.** The deliverable report **references** `mochiko:advocate-report-template` (not restated — Flag 4). The **generic** anti-rationalization doctrine is **referenced** to `loop-discipline` (Overview + Related); only the review-specific Red Flags + Common Rationalizations stay local. The 3-state verdict vocabulary is the same the lead already consumes — emitted, not re-specified.

---

## Realized responsibility trace (all 31 assess items — flipped to realized tags · no silent loss)

**SKILL.md body**

| # | Responsibility | Realized tag |
|---|----------------|--------------|
| 1 | Task-artifact gap-finding procedure (vertical slice · TDD · traceability) | **kept** — the skill's reason to exist |
| 2 | Severity classification (Critical/Important/Minor) | **kept** |
| 3 | 3-state verdict derivation from issue counts | **kept** (mechanical; recommended, loop fuel) |
| 4 | Verdict-as-clearing + "Return to Task Architect" routing | **moved-to-lead** — recommended verdict; lead owns clearing + FAIL-loop; "Return to the responsible producer" |
| 5 | "review criteria for the **Devil's Advocate**" (naming caller agent) | **dropped + reason** — decouple; independence by role, not agent name (DROP 1 of 2) |
| 6 | Phase split (Mapping vs Tasks criteria) | **kept-but-rebind** — reframed phase→artifact-type; **Branch A → two sets kept** (RQ-A resolved) |
| 7 | Review process (gather → execute → cross-ref → report) | **kept** |
| 8 | Cross-artifact traceability checks (story→cycle→task) | **kept** |
| 9 | Key principles to validate | **kept** — reframed as the explicit mirror of P3 |
| 10 | Quality checklist (reviewer self-check) | **kept** |
| 11 | Common mistakes | **kept** (+ Red Flags + Common Rationalizations, sibling-modeled) |
| 12 | "Wait for Task Architect to complete" (When-NOT / mistakes) | **kept-but-rebind** — "wait for the producer to finish the artifact"; agent-name dropped |
| 13 | When-NOT pointers to sibling skills | **kept-but-rebind** — `humaninloop:`→`mochiko:`; the disjoint-boundary pointers |

**Description / triggers**

| # | Responsibility | Realized tag |
|---|----------------|--------------|
| 14 | Trigger phrases (HIL "when the user says…") | **kept-but-rebind** — rewritten to work-context MUST/SHOULD; boundary-disclaimed vs `validation-plan-artifacts` |

**ISSUE-TEMPLATES.md**

| # | Responsibility | Realized tag |
|---|----------------|--------------|
| 15 | Severity classification rules (categories + examples) | **kept** |
| 16 | Issue-documentation format (standard + TM/TT/TX examples) | **kept** |
| 17 | Embedded report templates (Mapping/Tasks Review Report) | **dedupe** → reference `advocate-report-template`; the extracted **Checks-Executed tables kept** (Flag 4) |
| 18 | Verdict decision tree (mechanical derivation) | **kept** (+ "recommendation, lead owns clearing" note) |
| 19 | Issue-ID conventions (`TM-`/`TT-`/`TX-`) | **kept** |
| 20 | "Return to **Task Architect**" action label | **kept-but-rebind** — "Return to the responsible producer"; agent-name dropped |

**PHASE-CHECKLISTS.md**

| # | Responsibility | Realized tag |
|---|----------------|--------------|
| 21 | Mapping checklist (coverage · slice · foundation · sizing · traceability) | **kept** — mirror-aligned with P3 |
| 22 | Tasks checklist (cycle · TDD · file paths · `**TEST:**` · IDs · labels · markers · checkpoints · deps) | **kept** — `**TEST:**` KEPT as **design-time** presence/structure check |
| 23 | Cross-artifact review checklist (mapping↔tasks · traceability · consistency) | **kept** — Branch A cumulative pass |
| 24 | Vertical-slice / TDD / verification validation guidance (good vs bad) | **kept** |
| 25 | "Used by the **Devil's Advocate**" (header L3) | **dropped + reason** — decouple; agent-name (DROP 2 of 2) |
| 26 | IP-XXX · platform-app ordering · deployment cycle · brownfield markers · constraint-task traceability | **kept (dormant/parked)** — Flag 7.4 defer; lock-step with P3's parked side; **not** a silent drop |
| 27 | Common-issues tables (mapping / tasks) | **kept** |
| 28 | Example review report (embedded, illustrative) | **kept** — reframed as "Illustrative Worked Example"; agent-name decoupled; points at `advocate-report-template` (no competing blank skeleton) |

**Cluster wiring**

| # | Responsibility | Realized tag |
|---|----------------|--------------|
| 29 | `devils-advocate` re-mount (stub → live) | **kept-but-rebind — RECORDED, Wave-2 (not applied here)** — add `validation-task-artifacts` to `skills:` (L28) + Skills-Available bullet + delete deferred comment; mounts on the **reviewer** (independence preserved) |
| 30 | Router registration (new Tasks-cluster section) | **kept-but-rebind — RECORDED, Wave-2 (not applied)** — row above; section verified absent |
| 31 | `plugin.json` manifest registration | **no edit owed** — skills auto-discover via the `"skills": "./skills/"` glob (reconcile Flag 7.3); P4 auto-registers |

**Trace complete — no silent loss.** 20 `kept` · 5 `kept-but-rebind` (2 of them Wave-2 recorded obligations) · 1 `moved-to-lead` · 1 `dedupe` · 1 `kept (dormant/parked)` · 1 `no-edit-owed` · **2 `dropped + reason`** (both agent-name decouples — #5, #25 — each auditable, human-gate-accepted in the reconcile routine bundle). Every SKILL.md section, both references, the triggers, the re-mount, and the router are tagged.

## Cluster-wiring obligations RECORDED (Wave 2 — not silently dropped, not applied this run)

Per the run plan (Wave 1 = 5 content primitives; Wave 2 = wiring pass) the following are **recorded, not applied** here — this call produces the skill files only:
- **`devils-advocate` re-mount** (stub L25–28 → live) on the **reviewer** agent — item 29.
- **Router Tasks-cluster section** — item 30 (row above).
- **`plugin.json`** — no edit owed (glob auto-discovery) — item 31.

Editing any of these now would half-build the shared Wave-2 wiring pass and risk colliding with the P3 / command / agent transform calls.

---

**Next:** `verify-output` (independent `validator`, a **different** agent than this producer) grades `plugins/mochiko/skills/validation-task-artifacts/{SKILL.md, references/ISSUE-TEMPLATES.md, references/PHASE-CHECKLISTS.md}` against the transform done-condition (five conventions + sound-loop placement + kernel-free) and audits this realized trace for silent loss. Mirror-P3 (Flag 5b) and disjoint-boundary (Flag 4) are cross-checked against the P3 port + the sibling.
**Transform version:** v1 · **Governed by:** `loop-discipline` · **Role:** apply + wire only — no grade applied.
