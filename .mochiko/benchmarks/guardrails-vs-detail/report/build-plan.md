# Guardrails-vs-detail — build plan

**Status:** draft for user approval (2026-08-10)
**Ruling basis:** `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` (D1–D8 + Benchmark execution addendum) · `report/final-verdict.md` · `DECISIONS.md` rows 2026-08-10 (D1–D8 ruling + benchmark-verdict row) · user two-wave ruling 2026-08-10 (commands excluded).
**Work order:** `BACKLOG.md` → "## Guardrails-vs-detail build".

This plan ships the 28 evidence-backed cut variants and lands the D7 cost gate in **one plugin.json bump (Wave 1)**. Wave 2 (editorial extension to the untested primitives) is sketched at the end and executed later under a separate bump. Commands are excluded from both waves.

---

## 0. What ships in Wave 1

Three variant classes, all under `.mochiko/benchmarks/guardrails-vs-detail/variants/`:

- **11 guardrails skill bodies** (`variants/body/<name>/SKILL.md`) — pure body deletions; frontmatter is byte-identical to the current original.
- **11 slim skill descriptions** (`variants/descriptions/<name>/SKILL.md`) — slim frontmatter `description:`; body is byte-identical to the current original.
- **6 prose-only agent descriptions** (`variants/agents/<name>.md`) — `<example>` blocks stripped from the frontmatter `description:`; body byte-identical to the current original.

The 11 body skills and 11 description skills are **the same 11 skills**. Each shipped `SKILL.md` therefore takes a single **merged edit**: its new file = **the description-variant's frontmatter + the body-variant's body**. This is deterministic — no hand-merging judgment is required (see §2 pre-verify).

The 11 skills: `analysis-iterative`, `analysis-codebase`, `authoring-constitution`, `authoring-feature-map`, `review-governance-intent`, `validation-constitution`, `testing-governance-injection`, `authoring-requirements`, `authoring-user-stories`, `authoring-prototype`, `review-specifications`.

The 6 agents (`plugins/mochiko/agents/`): `principal-architect`, `validator`, `devils-advocate`, `requirements-analyst`, `product-manager`, `product-engineer`.

Plus: the **D7 cost gate** (§4) and the **two floor lines** from the cross-cutting finding (§4.3).

**Pre-verify result (run 2026-08-10 at plan time):** all 28 variants apply cleanly against current `main`/worktree originals — every body variant's frontmatter matches its original byte-for-byte, every description and agent variant's body matches its original byte-for-byte. The build still re-runs this as a gate at execution start (§2), because other worktrees may land intervening edits.

---

## 1. Work breakdown — parallelizable tracks and seats

Hard constraint throughout: **author ≠ grader.** A seat that edits a primitive or writes its strip never audits that same primitive. Benchmark variant authors were benchmark subagents and are gone, so every audit seat spun up for this build is clean by construction.

### Tracks

| Track | Work | Seats | Depends on |
|---|---|---|---|
| T0 Pre-verify | Re-confirm all 28 variants apply cleanly against current originals (§2) | 1 verify seat (or lead) | plan approval |
| T1 Skill ship | 11 merged skill edits + 11 skill strip files | 3 skill-shipper seats (batches 4/4/3) | T0 PASS |
| T2 Agent ship | 6 agent description edits + 6 agent strip files | 1 agent-shipper seat | T0 PASS |
| T3 Cost gate | primitive-edits.md gate wording + budget ledger + 2 floor lines drafted (floor lines applied inside T1 edits) | 1 cost-gate seat | T0 PASS |
| T4 Audit | Independent author≠grader audit of all 17 strip'd primitives | 3–4 auditor seats | T1, T2 complete for the audited primitive |
| T5 Landing | plugin.json bump + CHANGELOG + marketplace sync + DECISIONS row + BACKLOG→trail + ROADMAP touch | lead | T4 all PASS + user final approval |

T1, T2, T3 run in parallel after T0. T4 begins per-primitive as soon as that primitive's edit+strip is done (no need to wait for the whole batch).

### Seat assignment matrix (author ≠ grader)

Suggested batches — any assignment works as long as no auditor grades a primitive it shipped or stripped:

- **skill-shipper-A:** analysis-iterative, analysis-codebase, authoring-constitution, authoring-feature-map
- **skill-shipper-B:** review-governance-intent, validation-constitution, testing-governance-injection, authoring-requirements
- **skill-shipper-C:** authoring-user-stories, authoring-prototype, review-specifications
- **agent-shipper:** all 6 agents
- **auditors:** 3–4 fresh validator-agent seats. Assign each auditor a set of primitives it did **not** ship/strip. Cheapest clean option if seat budget is tight: cross-audit among shippers (shipper-A audits B's and C's primitives, etc.) — this satisfies author≠grader because A didn't author B/C's work. Cleaner and recommended: dedicated fresh auditor seats, since the audit brief is identical across primitives and fresh seats carry zero edit context to rationalize from.

**Editing + strip-writing by the same seat is allowed** — both are the "author" side of author≠grader. Only the audit must be disjoint.

**Seat count:** 3 skill-shippers + 1 agent-shipper + 1 cost-gate + 3–4 auditors + lead for landing = **8–9 working seats + lead**.

---

## 2. Per-primitive ceremony checklist

Every primitive runs the same four-step ceremony (`.claude/rules/mochiko/primitive-edits.md`).

### 2a. Pre-verify (T0, gate — one seat, all 28 at once)

Run the deterministic check below. It must be **all-clean** before any edit; a single drift is a stop-and-re-derive.

- **Body variants (×11):** the body variant's frontmatter block is byte-identical to the current original's frontmatter block. (Confirms the slim body was cut against today's frontmatter, no drift.)
- **Description variants (×11):** the description variant's body is byte-identical to the current original's body. (Confirms the slim description was cut against today's body.)
- **Agent variants (×6):** the agent variant's body is byte-identical to the current original's body; only the `description:` value differs. (Confirms only the example blocks were touched.)

Merge target for each skill is then fixed: **new frontmatter = description-variant frontmatter; new body = body-variant body.** No other source contributes.

If any check fails for a primitive, that primitive drops out of the automated path: re-derive its variant against the current original (re-apply the D4 cut line to the drifted section) before it can ship, and note the re-derivation in its strip. Clean primitives proceed regardless.

### 2b. Merged edit (T1/T2)

- **Skills:** write the merged file (description-variant frontmatter + body-variant body). Add the applicable floor line(s) from §4.3 as a **pure addition** to the body where that skill is a floor-line home.
- **Agents:** replace the frontmatter `description:` value with the prose-only variant's; body untouched.

### 2c. Strip entry (T1/T2) — see §3 for the strategy

One strip file per primitive, `.mochiko/strips/<primitive>.md`, newest-first, stamped with the shipping version (0.63.0). Skill strips cover **both** cuts (body + description) under the one version stamp. Agent strips cover the example-block removal.

### 2d. Audit dispatch (T4)

Independent validator-agent seat, handed the wave audit bar (§5). FAIL → fix → re-audit until PASS. The floor-line additions are graded within the same audit (do they read as guardrails, not returned playbook prose; do they not duplicate an existing floor).

---

## 3. Strip strategy — recommendation

**17 strip files** (11 skills, each carrying a merged body+description entry; 6 agents). Some skill bodies lose 2–15 KB.

**Recommendation: supersession-by-ruling form, faithfully-compressed content, with a mandatory protected-content reconciliation subsection.**

`.mochiko/strips/README.md` permits both a verbatim and a "faithfully compressed" Content field, and permits both strip entries (altitude/duplication cuts) and supersession-by-ruling entries (a decision retired the line). These cuts are **decision-driven**, not altitude judgments — the benchmark ruled that the detail earned no measured quality and dies. That is a supersession by ruling, so:

- **Disposition:** `superseded → benchmark verdict (guardrails variant is the new body / slim description is the new frontmatter)`
- **Tier failed:** `n/a — supersession by ruling` citing **the `DECISIONS.md` benchmark-verdict row (2026-08-10)** + `record.md` Benchmark-execution section + `report/final-verdict.md`.
- **Content:** *faithfully compressed* — a **section-level inventory** of what was removed (which sections/subsections, char delta per the §4 tables) plus an explicit pointer to the verbatim removed text, which survives in three places: (a) git history of the original `SKILL.md`, (b) the corresponding `variants/body/` (after-state) and the original (before-state) in this tree, (c) the archive branch `worktree-brainstorm-validator-scope`. Verbatim reproduction of 15 KB bodies in the log is rejected — the log is explicitly non-loaded and read at maintenance time only, and three verbatim homes already exist.
- **Kept deliberately:** the guardrails keep-set (goal/contract, floor, anti-patterns, `references/` pointers) — named so the next auditor doesn't read the survivors as an oversight.
- **Consumers assessed:** for shared skills (`analysis-iterative`, `authoring-feature-map` per D2), enumerate consuming surfaces.

**Mandatory protected-content reconciliation (per skill).** Before finalizing each skill strip, grep its existing strip file for `KEPT:`, protected, or `superseded` lines, and reconcile each against the body cut:

- Any prior `KEPT:` / protected / `DECISIONS.md`-traceable line that the guardrails cut **removes** must appear in this strip as an explicit recorded supersession-by-ruling — a silent drop is exactly what the audit's preserved-responsibilities check reads as a regression (primitive-edits.md, "Protected content leaves ONLY by ruling").
- **`authoring-constitution` is the known case:** its `[v0.28.0] KEPT:` entry ruled the entire post-doctrine body a behavioral-core survivor. Its guardrails cut is the smallest proportionally (17,240 vs 19,408 chars, −11%) precisely because most of that body is the KEPT core — but the strip and audit must confirm which KEPT elements survive the cut and record any that don't as a supersession. This is the top protected-content risk in the wave.

Agent strips are simpler: one supersession-by-ruling entry each for the `<example>`-block removal, citing the agents-arm ruling (b) in the same DECISIONS row; the prose framing survives (Kept deliberately).

A **strip template** for the shippers is worth pinning once at build start so the 17 entries are uniform.

---

## 4. D7 cost gate landing

### 4.1 Carrier files (recommended)

The author≠grader audit is a **mochiko-repo-internal ceremony** — it fires when mochiko's own shipped primitives are edited; adopters never edit mochiko primitives. Its home is `.claude/rules/mochiko/primitive-edits.md` (repo-side, path-scoped on `plugins/mochiko/**`, **not** shipped in the plugin). Therefore:

- **Gate wording** → add to `.claude/rules/mochiko/primitive-edits.md`, in the **Check** step, as a deterministic pre-assert that runs before the model-judgment audit.
- **Budget table** → a new companion file `.mochiko/memory/primitive-cost-budgets.md` (repo-side, non-shipped, non-loaded like `.mochiko/strips/`), cited by the Check step. Rationale: keeps the terse rule file terse and the numbers maintainable in one place; matches the existing pattern of a thin rule pointing at a fuller memory doc.

Both carriers are **outside `plugins/mochiko/`**, so editing them is **not** a shipped-primitive edit — no strip, no author≠grader audit is owed for the gate itself. Recommend one lightweight independent read of the gate wording + budget table for sanity (not a formal audit).

**Alternative considered:** inline the budget table in `primitive-edits.md`. Rejected — it bloats a deliberately terse rule and mixes procedure with data.

### 4.2 Gate text (draft)

To add under the **Check** step of `primitive-edits.md`:

> **Char-budget pre-assert (D7).** Before the model-judgment audit, the grader runs a deterministic character count of the edited primitive's budgeted classes and compares each to its budget in `.mochiko/memory/primitive-cost-budgets.md`:
> - **Skill body**, **skill frontmatter `description:` value**, **agent frontmatter `description:` value** — each measured as **characters of the parsed value**, never `wc -c` bytes (byte counts over-reject unicode-bearing text that is under the char cap).
> - Budget = the benchmark winning-variant char count **+25% headroom**, per the class/primitive table. Where the benchmark set no measured winner for a primitive, the standing hard caps apply instead (skill `description:` ≤ 1,536 delivery cap; no body/agent budget is enforced until a measured winner exists).
> - **Over budget = FAIL.** Exemption path: the editor names the overage in the audit brief with justification; the grader rules whether the justification holds (a genuine new obligation, not restored playbook prose). `references/` files are **exempt** as on-demand data.

The gate rides the existing audit — **no new seats, no new ceremony.**

### 4.3 Budget table (draft — chars of parsed value, from plan-time measurement)

Winning-variant char counts × 1.25, rounded. These seed `.mochiko/memory/primitive-cost-budgets.md`.

**Skill bodies (per-skill):**

| skill | winner chars | budget (+25%) |
|---|---|---|
| analysis-iterative | 3,942 | 4,927 |
| analysis-codebase | 6,509 | 8,136 |
| authoring-constitution | 17,240 | 21,550 |
| authoring-feature-map | 12,330 | 15,412 |
| review-governance-intent | 7,089 | 8,861 |
| validation-constitution | 6,734 | 8,417 |
| testing-governance-injection | 3,540 | 4,425 |
| authoring-requirements | 4,101 | 5,126 |
| authoring-user-stories | 5,361 | 6,701 |
| authoring-prototype | 8,898 | 11,122 |
| review-specifications | 11,271 | 14,088 |

**Skill descriptions (per-skill; all slim ≤500, hard cap 1,536 stays):**

| skill | winner chars | budget (+25%) |
|---|---|---|
| analysis-iterative | 476 | 595 |
| analysis-codebase | 349 | 436 |
| authoring-constitution | 481 | 601 |
| authoring-feature-map | 495 | 618 |
| review-governance-intent | 483 | 603 |
| validation-constitution | 481 | 601 |
| testing-governance-injection | 483 | 603 |
| authoring-requirements | 379 | 473 |
| authoring-user-stories | 425 | 531 |
| authoring-prototype | 493 | 616 |
| review-specifications | 490 | 612 |

**Agent descriptions (per-agent):**

| agent | winner chars | budget (+25%) |
|---|---|---|
| principal-architect | 750 | 937 |
| validator | 271 | 338 |
| devils-advocate | 314 | 392 |
| requirements-analyst | 307 | 383 |
| product-manager | 446 | 557 |
| product-engineer | 398 | 497 |

**Untested primitives** (the other 17 skills, 4 agents, and all commands): **no per-primitive body/agent budget** until a measured winner exists (Wave 2 or a future benchmark). They are covered only by the standing hard cap (skill `description:` ≤ 1,536) plus the justified-exemption path. This is deliberate — it prevents the gate from force-failing the four near-cap untested description skills (M1) before they are measured, while still catching over-cap growth on all skills via the 1,536 hard cap. **Commands** inherit the v8 goal+harness status quo as their budget only if/when measured; not budgeted in Wave 1 (commands are out of scope by the user's ruling).

### 4.4 The two floor lines

Both come from cross-cutting finding 1 (the score-loss driver was silently omitting a planted "I don't know" zone — a run-lead discipline, to be encoded as a one-line floor, not returned prose).

- **"Surface every elicited unknown as an open question."** Proposed home: **`analysis-iterative`** body (the shared elicitation/conditioning skill, in both the setup and specify clusters per D2). It already produces explicit out-of-scope and open questions; the floor line makes non-omission non-waivable. Added as a pure addition inside the T1 merged edit for that skill.
- **"Independent review leaves verdict + dispositions in the artifacts."** This is the F-X1 (agents ruling b) mitigation — the guardrail chosen over restoring example blocks. Proposed home: as a floor line in the **in-cluster review/validation skills being edited this wave** — `review-specifications`, `review-governance-intent`, `validation-constitution` — so the review seat reads it wherever it runs. Added as pure additions inside the T1 merged edits.

**Open question for the user/lead (see §7):** the review-evidence floor line lands in three skills as drafted. If triplication is unwanted, an alternative is a single shared review-contract home; but there is no shared review-contract skill today, and creating one is out of Wave 1 scope. Recommendation: land in the three in-cluster review skills now (low cost, they are open anyway), and note consolidation as a Wave 2 / follow-up candidate.

---

## 5. The audit bar (author ≠ grader)

There is no `validation-skill-shape` skill (deleted with `validation-command-shape` at v0.45.0), so skill/agent edits are graded by the **validator agent against a wave-specific checklist** — exactly the validator's second documented mode ("grade against the bar you were given"). Draft bar handed to each auditor seat, per primitive:

1. **Provenance / independence:** confirm the auditor did not ship or strip this primitive. (Absent → refuse the grade.)
2. **Merge fidelity:** the shipped file's frontmatter equals the description-variant's frontmatter and its body equals the body-variant's body (or, for re-derived primitives, matches the recorded re-derivation) — plus the sanctioned floor-line additions and nothing else. No un-blessed edits crept in.
3. **Preserved responsibilities:** the guardrails keep-set is intact — goal/output contract, non-waivable floor, anti-patterns/rejections, `references/` pointers. No protected / `KEPT:` / `DECISIONS.md`-traceable line silently dropped; every such removal appears as a recorded supersession in the strip (the §3 reconciliation).
4. **Strip completeness:** a strip entry exists, is stamped 0.63.0, carries the supersession-by-ruling citation, and its content inventory matches the actual diff.
5. **Floor-line quality (where applicable):** the added floor line reads as a guardrail, does not restore procedure/playbook prose, and does not duplicate an existing floor.
6. **D7 char-budget pre-assert:** parsed-value char counts for the edited classes are within budget (or a justified exemption is recorded and holds).

Verdict binary PASS/FAIL with a fix list; FAIL → author fixes → re-audit.

---

## 6. Release gates + KM landing (T5, lead, after all audits PASS + user final approval)

One plugin.json bump. Proposed version: **0.63.0** (single minor step). Rationale: the observed convention bumps one minor per build regardless of size — v0.61.0 was a breaking change and still took a single 0.60→0.61 step. A 0.70-style jump has no precedent here; the removal volume doesn't change the versioning convention. Recommend **0.63.0**.

Landing moves (KM subtractive ritual — all in one moment):

1. **`plugins/mochiko/.claude-plugin/plugin.json`** → version `0.63.0`.
2. **`.claude-plugin/marketplace.json`** → `metadata.version` synced to `0.63.0` (release gate 5). No plugin-list change (no new agents/skills; the 6 agent files are already listed).
3. **`CHANGELOG.md`** → new `## [0.63.0] — 2026-08-10` entry (draft below) — release gate 4.
4. **`DECISIONS.md`** → append a build row: "Guardrails-vs-detail build shipped (v0.63.0)" — 11 merged skill edits (guardrails bodies + slim descriptions) · 6 prose-only agent descriptions · D7 char-budget cost gate landed in primitive-edits.md + primitive-cost-budgets.md · two floor lines · N author≠grader audits PASS · strips recorded. Status `ruled + built`. Pointer to the record + final-verdict. (This is the build-completion row; the existing benchmark-verdict row stays.)
5. **`BACKLOG.md`** → the "## Guardrails-vs-detail build" open item compresses to a one-line DONE + pointer and **moves to `.mochiko/archive/backlog-trail.md`**. Its riding watches do **not** die — see §6.1.
6. **`ROADMAP.md`** → touch Now/Next (move the build out of Now; last-groomed stamp untouched unless a cap trips).

Release-gate 4+5 execution is routine per-bump (GI-012). Confirm CHANGELOG entry present and marketplace synced before the bump is considered landed.

### 6.1 Watches — where they land

The BACKLOG build item already names two watches ("F-X1-class review-evidence omission (agents ruling b) · fire-rate on slim descriptions") and two riding open threads (M1 near-cap-skills-outside-substrate, M2 audit-substrate-shrink). These are **not** closed by shipping. On landing, open a **new BACKLOG residual item** — "Guardrails-vs-detail first-live-run watches" — carrying:

- **F-X1 review-evidence watch:** first live runs on the cut agent descriptions — does an independent review leave verdict + dispositions in the artifacts (the floor line's job)? Recurrence re-opens the agents ruling (b).
- **Slim-description fire-rate watch:** first live setup/specify runs — do the slim descriptions still fire when their moment comes (both sibling traps)?
- **M1:** the four near-cap untested description skills (`patterns-system-design`, `authoring-architecture`, `review-brainstorm`, `review-feasibility`) — verdict extends by nature-coverage only; measured in Wave 2.
- **M2:** terser primitives shrink the audit's preserved-responsibilities substrate; watch at the first post-verdict audit (which is this wave's own audit — first datapoint lands here).

This mirrors the existing residual pattern (e.g. "Cold-review gap-challenge residuals"). It keeps the record's open threads 1–2 alive after the build item leaves the backlog.

### 6.2 CHANGELOG entry (draft)

> ## [0.63.0] — 2026-08-10
>
> - Guardrails-vs-detail build, Wave 1 (ruling: `validator-scope-and-verbosity` D1–D8 + benchmark verdict, `DECISIONS.md` 2026-08-10; `report/final-verdict.md`). Shipped the benchmark's winning variants across the setup+specify substrate: **11 skills** re-authored as guardrails bodies + slim descriptions (`analysis-iterative`, `analysis-codebase`, `authoring-constitution`, `authoring-feature-map`, `review-governance-intent`, `validation-constitution`, `testing-governance-injection`, `authoring-requirements`, `authoring-user-stories`, `authoring-prototype`, `review-specifications`) — skill-body chars −11% to −65%, descriptions cut to ≤500-char slim forms; **6 agents** shipped prose-only (`<example>` blocks removed from the frontmatter description — `principal-architect`, `validator`, `devils-advocate`, `requirements-analyst`, `product-manager`, `product-engineer`), 55–86% smaller descriptions with no measured routing loss. **D7 cost gate** landed: per-class char budgets (winning-variant chars +25%, parsed-value chars never `wc -c` bytes) as a deterministic pre-assert in the author≠grader audit (`.claude/rules/mochiko/primitive-edits.md` + `.mochiko/memory/primitive-cost-budgets.md`), justified-exemption path, `references/` exempt. **Two floor lines** from the cross-cutting finding replace returned prose: "surface every elicited unknown as an open question" (`analysis-iterative`) and "independent review leaves verdict + dispositions in the artifacts" (review/validation skills). Each edit through the primitive-edit ceremony: 17 supersession-by-ruling strips + independent author≠grader audits (all PASS). Commands and the remaining 17 skills / 4 agents deferred to Wave 2. Residual watches (F-X1 review-evidence, slim-description fire-rate, M1/M2) in `BACKLOG.md`.

(The exact per-skill body-reduction range in the draft — −11% to −65% — is from the §4.3 measurements; confirm at write time.)

---

## 7. Ordering, gating, and user checkpoints

```
[USER CHECKPOINT 1: plan approval]
        │
        ▼
T0 pre-verify (all 28 clean?) ──fail──> re-derive drifted variant(s), re-verify
        │ all clean
        ▼
T1 skill ship ┐
T2 agent ship ├─ parallel ─┐
T3 cost gate  ┘            │
        │ per-primitive edit+strip done
        ▼
T4 author≠grader audit (per primitive) ──FAIL──> fix ──> re-audit
        │ all 17 PASS
        ▼
[USER CHECKPOINT 2: final review before bump]
        │
        ▼
T5 landing: plugin.json 0.63.0 + marketplace sync + CHANGELOG + DECISIONS row
           + BACKLOG→trail + new residual item + ROADMAP touch
```

Two user checkpoints: **plan approval** before the first edit, and **final review** before the plugin.json bump. Everything between is reversible (edits on a branch, no bump) and needs no user gate.

**What blocks what:** T0 blocks all edits. Each primitive's T4 audit blocks only that primitive (audits stream as edits finish). All-PASS blocks checkpoint 2. Checkpoint 2 blocks T5. The cost-gate carriers (T3) are not shipped primitives, so they don't need T4, but the gate's budget table should exist before checkpoint 2 so the D7 pre-assert can actually run inside T4 audits (i.e. T3 budget ledger should land early, alongside T1/T2).

---

## 8. Risk register

| # | Risk | Likelihood | Mitigation |
|---|---|---|---|
| R1 | **Variant drift** vs current originals (another worktree edits a source file before this build runs) | Low (all clean at plan time) | T0 pre-verify gate re-runs the byte-identity check at execution start; any drift drops that primitive to a re-derive path (re-apply D4 cut line to the drifted section, note in strip) before it ships. |
| R2 | **Protected / `KEPT:` content silently dropped** — esp. `authoring-constitution`'s v0.28.0 KEPT survivor body | Medium | §3 mandatory per-skill reconciliation: grep prior strip for KEPT/protected/DECISIONS-traceable lines, record any removed as supersession; audit bar item 3 fails on a silent drop. authoring-constitution flagged as top case (its cut is only −11%, mostly leaving the KEPT core intact). |
| R3 | **Audit seat contamination** (author grades own edit/strip) | Low | Disjoint assignment matrix (§1); audit bar item 1 makes the auditor confirm non-authorship and refuse if it authored; fresh dedicated auditor seats preferred over shipper cross-audit. |
| R4 | **M1 — near-cap untested skills** force-failed or forgotten by the gate | Low | D7 budget is per-primitive-measured; untested skills fall back to the 1,536 hard cap only, no invented body budget → they cannot be force-failed by a budget they were never measured for. Carried as a Wave 2 item + BACKLOG watch. |
| R5 | **M2 — audit-substrate shrink** — the terser primitive gives the preserved-responsibilities check less to grade against, and D7 bolts a budget onto that same audit | Medium | The §3 strip inventory (what was removed, with the before-state in git/variants) is the substrate the audit grades against, not the shrunken body alone; true-reductions-only accounting survives from D8. First-datapoint watch is this wave's own audit. |
| R6 (bonus) | **Strip volume / merged-edit bookkeeping** (17 files, 11 double-cut) | Medium | Pin one supersession-by-ruling strip template at build start; compressed content form (§3) not verbatim; each skill strip is one entry covering both cuts under the 0.63.0 stamp. |

---

## 9. Open questions needing a user ruling

1. **Review-evidence floor-line placement (§4.3).** Land "independent review leaves verdict + dispositions in the artifacts" in three in-cluster review/validation skills (recommended, low cost), or hold for a single shared review-contract home (out of Wave 1 scope)? Default if no ruling: land in the three skills.
2. **Version (§6).** Confirm **0.63.0** (recommended, matches convention) vs a larger jump.
3. **Auditor seats (§1).** Dedicated fresh auditor seats (recommended) vs shipper cross-audit (cheaper, still author≠grader-clean)?
4. **Nothing else is blocking** — the merge is deterministic, budgets are measured, and both checkpoints are already placed.

---

## Wave 2 sketch (executed later, separate bump)

Editorial extension of the same D4 cut lines to the untested primitives. Not part of Wave 1; runs under its own plugin.json bump after the Wave 1 first-live-run watches report clean (or at least not-regressed).

**17 remaining skills:** `authoring-architecture`, `authoring-technical-requirements`, `brownfield-integration`, `executing-tdd-cycle`, `grooming-operating-docs`, `mochiko` (the router), `patterns-api-contracts`, `patterns-code-minimalism`, `patterns-entity-modeling`, `patterns-system-design`, `patterns-technical-decisions`, `patterns-vertical-tdd`, `review-brainstorm`, `review-code-minimalism`, `review-feasibility`, `review-plan-artifacts`, `testing-end-user`.

**4 remaining agents:** `qa-engineer`, `staff-engineer`, `system-architect`, `technical-analyst`.

**Cut-line application (same D4).** Keep: goal/output contract, non-waivable floors, anti-patterns/rejections, `references/` pointers. Drop: step-by-step procedure walkthroughs, worked examples in the body, restatement of what the command/CLAUDE.md already carries. Edit editorially (no benchmark run) — Wave 1's evidence that guardrails-only holds across all four skill natures (analysis, authoring, review/validation, testing) is the warrant; Wave 2 rides that generalization rather than re-measuring.

**Budget derivation for untested primitives.** No measured winner exists, so budgets are set at Wave 2 **from the Wave-2 cut result itself + 25%** (the same "winner + headroom" rule, with the editorial cut standing in for the benchmark winner), seeded into `.mochiko/memory/primitive-cost-budgets.md` at the Wave 2 bump. Until then they stay on the hard-cap-only fallback.

**Extra caution — the four near-cap description skills** (`patterns-system-design` 1,514 · `authoring-architecture` 1,511 · `review-brainstorm` 1,506 · `review-feasibility` 1,513 — all within ~30 chars of the 1,536 cap, and all **outside** the tested setup+specify substrate, per record open thread 1 / M1). These get their descriptions slimmed first in Wave 2 (highest value, currently one growth-edit from truncation), but with a fire-rate spot-check after slimming, since their invocation reliability was never measured — the Wave 1 fire-rate result covers only in-cluster skills. Treat a fire miss on any of these as a stop-and-reconsider for the description cut on plan/implement/brainstorm-stage skills.

**Ceremony is identical** to Wave 1: per-primitive strip (supersession-by-ruling citing this build's DECISIONS row as the governing method) + independent author≠grader audit + D7 pre-assert, one bump, KM landing.
