# TRANSFORM — P5 `tasks-template.md`

**Run:** `/mochiko:transform-cluster tasks` · **Phase 3 (transform, per-primitive)** · **Producer:** `mochiko:transform-producer` · **Skill:** `mochiko:transform-recipes` · **Date:** 2026-07-01
**Consumes:** `assess-tasks-template.md` (P5) + `reconcile.md` (Flag 5a canonical-structure contract · RQ-A Branch A derived-echo · P5 trace L238–241 · gated dropped-reason L285).
**Model:** the ported `plugins/mochiko/templates/plan-template.md` (deliverable-template precedent — no frontmatter, `[...]` model-fill, `.mochiko/specs/<feature>/` input/provenance rebind, "the tasks workflow" phrasing at L55–57).
**Applies:** the finalized disposition — reconcile did the deciding; this run only edits. Grading is `verify-output` (a different agent).

---

TRANSFORM: tasks-template
Applied:   port-with-edits × standalone + wiring-pass
Artifacts: plugins/mochiko/templates/tasks-template.md (created)
New partners: none (standalone — no split/promote/pair; RQ-A leaves the file `standalone`, HIL never had a `task-mapping-template.md` to pair with)
Wiring:    classification=N/A (passive template — no frontmatter, matches plan-template precedent) · router=cluster-obligation recorded (Flag 7.2, see below — not half-built) · triggers=N/A (not model-invoked) · rebinds=[L2 provenance path · L42 command-name→workflow · L43/L44 sample-comment artifact list] · decouple=1 HIL token scrubbed (`/humaninloop:tasks`) · single-source=Story→Cycle table demoted to a derived echo of `task-mapping.md`

---

## Body edits applied (`port-with-edits`)

1. **Frontmatter dropped** — removed L1–3 (`--- / description: "Task list template…" / ---`). The file now opens on the H1. Mirrors the plan-template deliverable precedent (no frontmatter). Flagged, not silent (trace #1).
2. **Provenance rebind (L7→L2)** — `> Generated from: spec.md, plan.md, …, contracts/` → `> Generated from `.mochiko/specs/<feature>/`: spec.md, plan.md, requirements.md, constraints-and-decisions.md, nfrs.md, data-model.md, contracts/`. All 7 upstream artifacts kept; only the binding rebinds to `.mochiko/specs/<feature>/` (= plan-template's Input line path).
3. **Sample-comment scrub (L42)** — `The /humaninloop:tasks command MUST replace these…` → `The tasks workflow MUST replace these…` (= plan-template L55–57 "the tasks workflow"). The illustrate-then-replace mechanism (HTML comment + SAMPLE cycles) is kept intact — only the HIL command name is scrubbed.
4. **Sample-comment artifact-list rebind (L43/L44)** — `User stories from spec.md` → `…from .mochiko/specs/<feature>/spec.md`; `Design artifacts from plan.md, …, contracts/` → `…from .mochiko/specs/<feature>/: plan.md, …, contracts/` (same binding as edit #2).
5. **Fill-style — confirmed, not converted** — the body is already entirely `[...]` model-fill (`[FEATURE NAME]`, `[N]`, `[Entity]`, `[P]`, `[US#]`, `[EXTEND]`/`[MODIFY]`, `[Observable outcome]`). Zero `{{}}` mustache (grep-confirmed on source). No conversion.
6. **Decouple scan — clean** — source coupling-token scan returned exactly one HIL token (`/humaninloop:tasks`, now scrubbed). No `.workflow/`, DAG, MCP, catalog, or brain reference exists to remove. Sample code paths (`tests/e2e/…`, `src/models/[entity].py`) are `[...]`-parameterized craft, not coupling — kept.

## Canonical-structure contract honored (reconcile Flag 5a — P5 IS the single canonical `tasks.md` skeleton)

This file is the authoritative `tasks.md` *structure* that `patterns-vertical-tdd` (P3) conforms to. Kept clean and canonical, byte-stable on every element P3 must align to:
- **Foundation Cycles** (sequential) + **Feature Cycles** (`[P]`) split with their blockquote descriptors.
- The `### Cycle N: …` heading format with `> Stories:` / `> Dependencies:` / `> Type:` traceability lines.
- `TN.X` task-ID scheme (`T1.1`…`T5.5`), test-first ordering, and the `**Checkpoint**:` line per cycle.
- Marker legend `[P]` / `[US#]` / `[EXTEND]` / `[MODIFY]`.
- Execution Strategy (MVP / Incremental / Parallel) + Dependencies Summary (ASCII graph + Story→Cycle table).

No `**TEST:**` block is present in this HIL skeleton; P3 owns the `**TEST:**` verification-task grammar and, per Flag 5a, must keep its worked example byte-aligned to the elements above. This template neither introduces nor contradicts that grammar — the alignment is P3's edit to make against this canonical skeleton.

## RQ-A Branch A honored (derived-echo, not a second source)

The **Story → Cycle Mapping** section (source L191–198) is kept as a **derived echo**. A one-line blockquote note now precedes the table: it names `.mochiko/specs/<feature>/task-mapping.md` as the source of truth for slice rationale + story→cycle decisions, and states the table is a read-only summary view regenerated from that file — not a second authoritative source. This dissolves the duplication by source-of-truth (single source, one derivation) without collapsing the table.

---

## Realized responsibility trace (all 14 P5 items — flipped to realized tags)

| # | Section / responsibility (source line) | Realized tag |
|---|---|---|
| 1 | Frontmatter `description:` (L1–3) | **dropped + reason** — redundant with the H1; deliverable-template precedent (plan-template) carries no frontmatter. Auditable, human-gate-accepted (reconcile L285). |
| 2 | H1 `# Implementation Tasks: [FEATURE NAME]` (L5) | **kept** (verbatim; deliverable title, `[...]` style) |
| 3 | Provenance `> Generated from: …` (L7) | **kept-but-rebind** — 7 artifacts kept; path bound to `.mochiko/specs/<feature>/` |
| 4 | `> Structure: Vertical slices … TDD cycles` (L8) | **kept** (verbatim; craft descriptor) |
| 5 | Overview metrics table (L10–17) | **kept** (verbatim; `[N]` fill-targets) |
| 6 | Cycle Format — TDD discipline (L19–26) | **kept** (verbatim; canonical `TN.X` grammar — Flag 5a) |
| 7 | Markers legend `[P]`/`[US#]`/`[EXTEND]`/`[MODIFY]` (L27–34) | **kept** (verbatim; canonical marker legend — Flag 5a) |
| 8 | Sample-cycle HTML comment (L38–53) | **kept-but-rebind** — mechanism kept; `/humaninloop:tasks`→"the tasks workflow" (L42); artifact list bound to `.mochiko/specs/<feature>/` (L43/L44) |
| 9 | Foundation Cycles + samples C1–C2 (L55–92) | **kept** (verbatim; canonical sequential-foundation structure — Flag 5a) |
| 10 | Feature Cycles + samples C3–C5 (L96–148) | **kept** (verbatim; canonical `[P]` feature structure — Flag 5a) |
| 11 | Execution Strategy — MVP/Incremental/Parallel (L152–175) | **kept** (verbatim; canonical) |
| 12 | Cycle Dependencies ASCII diagram (L183–189) | **kept** (verbatim; canonical) |
| 13 | Story → Cycle Mapping table (L191–198) | **kept + dedupe (derived echo)** — RQ-A Branch A: read-only summary view regenerated from `task-mapping.md` (source of truth); one-line note added; not a second authoritative source |
| 14 | Notes — TDD/Vertical/Foundation/Parallel/Commit/Checkpoint (L202–209) | **kept** (verbatim; craft guidance) |

**Trace complete — no silent loss.** 10 `kept` verbatim · 2 `kept-but-rebind` · 1 `kept + dedupe`(derived echo) · 1 `dropped + reason` (auditable, gate-accepted). Every source section is accounted for; the RQ-A flag from assess is now resolved (Branch A) and realized.

## Cluster-wiring obligation recorded (not silently dropped — Flag 7.2)

Router registration for `tasks-template` belongs to the **single** Tasks-cluster router edit (reconcile Flag 7.2), which does not yet exist in `plugins/mochiko/skills/mochiko/SKILL.md` (verified: doctrine/transform/setup/specify/plan sections only, no Tasks). That edit adds the whole Tasks section together — `patterns-vertical-tdd`, `validation-task-artifacts`, `tasks-template`, `taskarchitect-report-template` (if gated-in), plus the `/mochiko:tasks` entry-point row, the `task-architect` agent row, and the `devils-advocate` cross-workflow row update. Writing a partial section for `tasks-template` alone would half-build it and risk colliding with the P3/P4/command/agent transform calls. **Row to add (verbatim, when the Tasks section lands):**

> | `tasks-template` (template) | the canonical `tasks.md` deliverable structure the `task-architect` producer fills — Foundation (sequential) + Feature (`[P]`) cycles, `TN.X` tasks, checkpoints, marker legend; the Story→Cycle table is a derived echo of `task-mapping.md`, not a second source |

Templates carry no `plugin.json` manifest key (auto-discovered by neither a skills nor a commands glob), so no manifest edit is owed for this primitive.

---

**Next:** `verify-output` (independent `validator`, a different agent than this producer) grades `plugins/mochiko/templates/tasks-template.md` against the transform done-condition + audits this realized trace for silent loss.

---

## Alignment fix (P3↔P5, `**TEST:**` final task)

**Applied:** post-transform cross-primitive alignment · **Producer:** `mochiko:transform-producer` · **Disposition:** unchanged (`port-with-edits × standalone`) — this expands the existing `port-with-edits` set; no re-decision. **Grading:** still owed to `verify-output` (a different agent — not self-graded).

**Defect (referee-confirmed).** P5 carried HIL's deprecated `Demo and validate acceptance criteria` as each cycle's final task, with **no `**TEST:**` block**. The initial transform above (see L36 and realized-trace items #6/#9/#10/#14, tagged `kept (verbatim)`) treated those `Demo` lines as canonical/byte-stable and deferred the `**TEST:**` grammar wholly to P3. But reconcile **Flag 3** made `patterns-vertical-tdd/references/CYCLE-STRUCTURE.md` the **canonical home** of the `**TEST:**` verification-task grammar, whose canonical cycle form **ends every cycle in a `**TEST:**` task with real infrastructure**. Three siblings agree on that shape: **P2** (`task-architect` persona) mandates "the final task of each cycle MUST be a verification task"; **P4** (`validation-task-artifacts`) checks for `**TEST:**`-task presence; **Flag 5a** lists "the `**TEST:**` block" as a byte-alignment point. P5 was the **lone holdout on "Demo"** — this fix brings it **UP** to the `**TEST:**` canonical form so the cluster no longer contradicts itself. (This supersedes the L36 sentence "No `**TEST:**` block is present in this HIL skeleton"; P5 now conforms to P3's canonical grammar rather than deferring it.)

**Edits applied to `plugins/mochiko/templates/tasks-template.md`:**

1. **Cycle Format section** — final-task bullet `- **TN.X**: Demo and validate acceptance criteria` → `- **TN.X**: **TEST:** — verify [behavior] with real infrastructure (Setup/Action/Assert/Capture)`; added a clarifying sentence stating the final task of every cycle is a real-infrastructure `**TEST:**` task that **gates cycle completion** (not a plain demo), with a single-source reference to `CYCLE-STRUCTURE.md` (references the canonical grammar, does not restate it).
2. **All five sample-cycle final tasks** — `Demo …` lines replaced with full `**TEST:**` blocks matching CYCLE-STRUCTURE.md's worked-example grammar (`**TEST:** - <desc>` + 2-space `**Action**:`/`**Assert**:`/`**Capture**:`, `**Setup**:` where a prerequisite applies), mirroring the canonical Setup-presence pattern (C1/C2/C4 no Setup; C3/C5 Setup):
   - `T1.6` — CRUD via API (`curl -X POST …/api/[entity]` · `Response status: 201` · `Console contains "[entity]_id"` · `Capture: console`)
   - `T2.7` — Login returns a valid auth token (byte-aligned to CYCLE-STRUCTURE C2 example: `curl -X POST …/api/auth/login` · `Response status: 200` · `Console contains "token"`)
   - `T3.6` — behavior end-to-end via API (`**Setup**:` seed + POST · status 200 · console-contains)
   - `T4.6` — feature via API (`curl -X POST …` · status 201 · `Console contains "[field]":"[value]"`)
   - `T5.5` — multi-feature filter via API (`**Setup**:` seed + `curl "…?[param]=[value]"` · status 200 · console-contains-only-matching)
   - Task IDs, `**Checkpoint**:` lines, markers, and all non-final tasks left **unchanged**.
3. **Notes → TDD Discipline** — `Every cycle starts with a failing test` → `Every cycle starts with a failing test **and ends with a real-infrastructure `**TEST:**` verification task that gates cycle completion**`. "Checkpoint Validation" note kept.

**Corrected realized tags (supersede the initial-transform table for the touched sections only):**

| # | Section | Was | Now |
|---|---|---|---|
| 6 | Cycle Format — TDD discipline | `kept (verbatim)` | **`port-with-edits`** — final-task bullet aligned to `**TEST:**` canonical form + clarifying sentence + CYCLE-STRUCTURE reference |
| 9 | Foundation samples C1–C2 | `kept (verbatim)` | **`port-with-edits`** — `T1.6`/`T2.7` final tasks now `**TEST:**` blocks (real-infra), all other tasks verbatim |
| 10 | Feature samples C3–C5 | `kept (verbatim)` | **`port-with-edits`** — `T3.6`/`T4.6`/`T5.5` final tasks now `**TEST:**` blocks (real-infra), all other tasks verbatim |
| 14 | Notes | `kept (verbatim)` | **`port-with-edits`** — TDD Discipline bullet now states cycles **end** with a `**TEST:**` task; other notes (incl. Checkpoint Validation) verbatim |

**Unchanged by design (scope boundary honored):** item #11 Execution Strategy / MVP-Delivery "demo to stakeholders" (release process, not the cycle final-task) — kept as-is; item #13 Story→Cycle **derived echo** of `task-mapping.md` — kept; path bindings (#3/#8), marker legend (#7), and dropped frontmatter (#1) — untouched. No edit to P3, the router, or `plugin.json`.

**No silent loss.** The `Demo`/acceptance-validation responsibility is not dropped — it is **subsumed and strengthened** into the real-infrastructure `**TEST:**` verification gate (`kept-but-rebind` at the responsibility level: "prove the slice works" now proven against real infra, not demoed). Trace remains complete; every sample cycle now ends in a `**TEST:**` block matching CYCLE-STRUCTURE.md.
