# Strip notes — `skills/brownfield-integration/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 1 (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, batch-ratified 2026-07-25): body
128 → 111 lines, 17 cut = 13% — **under the 30–70 never-stripped band**; per R3 the under-band
second pass generates the survivor-provenance (KEPT) entries below.

## [v0.64.0] Guardrails body + slim description (guardrails-vs-detail Wave 2 editorial cut)
- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md`
  2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark
  verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed):** body 6,657 → 6,342 chars (−5%); description 913 → 491
  chars (−46%). Body cut: the **When to Use** section deleted whole (four bullets restating the
  description's invocation conditions — `[EXTEND]` marker, `[MODIFY]` marker, a task referencing a
  file on disk, following prior patterns; each obligation survives in the EXTEND/MODIFY
  consumption table, the Read-Before-Write checklist, and Interface Preservation). Description
  cut: the "detecting conflicts before adding code" clause and the design-time-declaration
  provenance ("the builder assigns at decomposition time … declared at design time by
  patterns-vertical-tdd") compressed; the MUST clause, core triggers, and the
  `executing-tdd-cycle` co-fire sibling distinction kept. Verbatim homes: git history of this
  file (pre-v0.64.0).
- **Old description (verbatim):**
  > This skill MUST be invoked when implementing a task that touches existing code — safely making an `[EXTEND]` or `[MODIFY]` change to a file already on disk: reading the whole file before writing, following its established patterns, preserving its interface, and detecting conflicts before adding code. SHOULD also invoke when extending an existing file, modifying existing behavior, integrating against an established interface, or following patterns set by prior work in the codebase. Consumes the extend/modify classification the builder assigns at decomposition time (from the cycle card's brownfield exposure, declared at design time by patterns-vertical-tdd); this is the implement-time, read-before-write craft of making that one modification safely — NOT the execution of the cycle the task belongs to (that is executing-tdd-cycle, which co-fires on the same brownfield task and drives red/green/refactor).
- **Kept deliberately:** the guardrails keep-set — the Overview + consequence aphorism, When NOT
  to Use, the EXTEND/MODIFY consumption table, the Read-Before-Write checklist, Interface
  Preservation, Conflict Detection, When to Flag, the Common Mistakes table, the Common
  Rationalizations table, and the Red Flags section.
- **MANDATORY KEPT reconciliation:** this file's [v0.25.0] KEPT entry protects the EXTEND/MODIFY
  consumption table, the Read-Before-Write checklist, Conflict Detection, When to Flag, and the
  Rationalizations table; the [v0.49.0] supersession KEPT "the entire consumption discipline
  (read-before-write, interface preservation, EXTEND-never-silently-becomes-MODIFY, conflict
  escalation)." **The When-to-Use cut removes NONE of these** — every protected element is a
  distinct surviving section; only the invocation-condition bullets (in no KEPT set) were
  deleted. The slim description preserves the consumes-classification + co-fire framing the
  [v0.49.0] ruling re-keyed. No prior KEPT or protected line is touched.
- **Consumers assessed:** staff-engineer (mounts it) · executing-tdd-cycle (co-fires, cross-links)
  · patterns-vertical-tdd, patterns-code-minimalism (cross-reference) · implement · mochiko
  router. None links the removed When-to-Use bullets or a description clause. Contract intact.

## [v0.49.0] Marker source re-keyed — builder classifies at decomposition
- **Disposition:** superseded → tasks arrive tagged by the builder's own decomposition, classified from the cycle card's brownfield-exposure line (declared by patterns-vertical-tdd at design time)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2.1)
- **Content:** "the marker **vocabulary** is defined by `patterns-vertical-tdd`, which stamps those markers onto tasks at design time" and the parallel description/interface-table clauses.
- **Kept deliberately:** the entire consumption discipline (read-before-write, interface preservation, EXTEND-never-silently-becomes-MODIFY, conflict escalation) — untouched.
- **Consumers assessed:** staff-engineer · executing-tdd-cycle (co-fires).

## [v0.25.0] Common Mistakes densified: 4 subsections → 4-row table (net −17 lines)
- **Disposition:** compressed in place (densification, zero deletions)
- **Tier failed:** n/a — form only
- **Content:** not-reading-full-file, silent-rewrite-on-extend, ignored error handling, "better" patterns
- **Consumers assessed:** 4 consumer files checked at wave open; none reference the subsection headings

## [v0.25.0] Red Flags inline rebuttals trimmed to thought-only bullets
- **Disposition:** relocated → the Common Rationalizations table (in-file — the rebuttals duplicated its rows: "better pattern", "refactor to make it work", "clean it up"); the one flag without a table row ("existing tests don't cover this") keeps its inline rebuttal
- **Tier failed:** 1 (second encoding of the table's rebuttals; the flags name the thoughts, the table rebuts them — the validation-constitution pattern)
- **Content:** per-bullet rebuttal clauses
- **Consumers assessed:** none reference the section

## [v0.25.0] Interface Preservation — 3 bullets restating the EXTEND/MODIFY table
- **Disposition:** relocated → the EXTEND/MODIFY consumption table (in-file; the section header now points at it)
- **Tier failed:** 1 (the signature / export-surface / public-API MUST-NOTs are the table's cells verbatim)
- **Content:** the three restating DO-NOT bullets; rename-prohibition + the two DO bullets kept
- **Consumers assessed:** none reference the bullets

## [v0.25.0] Aphorism consequence-anchored (R4b rider, +1 line)
- **Disposition:** consequence attached to the previously bare copy: "Every shortcut in read-before-write discipline is a broken consumer waiting to surface."
- **Tier failed:** n/a — rider execution (bare copy → Tier-2-qualifying in place)
- **Content:** one added consequence line
- **Consumers assessed:** n/a

## [v0.25.0] KEPT: EXTEND/MODIFY consumption table, Read-Before-Write checklist, Conflict Detection, When to Flag, Rationalizations table
- **Tier-2 evidence:** contested at the under-band pass and kept — each names a concrete failure this craft exists to prevent (interface breaks, unseen conventions, name/import collisions, silent workarounds) and the flag-routing seam into `executing-tdd-cycle`'s cycle report; the marker vocabulary stays with `patterns-vertical-tdd` (grammar seam respected, not restated). Session ruling: batch-2 ratification 2026-07-25.
