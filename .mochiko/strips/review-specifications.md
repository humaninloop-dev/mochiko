# Strip notes — `skills/review-specifications`

Entry formats: `strips/README.md`. Wave context: the specify cluster wave (v0.13.0).
Single-consumer primitive (the specify critique, mounted on `devils-advocate`) — strips
ruled in-wave.

## [v0.53.0] Code-review punt line narrowed — minimalism-lens carve-out
- **Disposition:** superseded → the same When-NOT-to-Use bullet with a parenthetical carve-out naming `mochiko:review-code-minimalism` (implement-side) as the one exception; general code review stays a different domain.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-05 "Ponytail code-minimalism ruled (PT-D1–D10)", record `.mochiko/brainstorms/ponytail-concepts-integration/record.md`, D2 — punt reversal narrow, lens-only).
- **Content (verbatim, the superseded bullet):**
  ```
  - **Code review** - Different skill domain entirely
  ```
- **Kept deliberately:** the punt for everything but the minimalism lens — this skill grades specs, never code; scope unchanged.
- **Consumers assessed:** devils-advocate (mounts it; specify-side scope unchanged) · specify (binds it; no behavior change).

## [v0.50.0] Gained the Screens & Flows prototype-walk grade
- **Disposition:** pure addition riding the decision row (new 8-check section + process step + checklist row + description clause; the process-step renumber 6→7/7→8/8→9 is the only touched existing text)
- **Tier failed:** n/a — addition by ruling (`DECISIONS.md` row 2026-08-02 "UX mocking in specify (UX-D1–D9)"; record `.mochiko/brainstorms/ux-mocking-in-specify/record.md`, D7)
- **Content:** nothing removed from this skill.
- **Consumers assessed:** devils-advocate (mounts it) · specify (binds it).

## [v0.49.0] Absorbed the Delivery Slices grade (from retired review-slices)
- **Disposition:** pure addition riding the decision row (new 11-check section + process step + checklist row + description clause) — recorded here because the absorption is half of a supersession pair (see `review-slices.md`)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D9)
- **Content:** nothing removed from this skill.
- **Consumers assessed:** devils-advocate (mounts it) · specify (binds it).

## [v0.26.0] Question Format → template pointer; What to Avoid deleted; When-to-Use merged; Common Mistakes densified (body 166 → 128, −23%, in-band 10–40)
- **Disposition:** deduped → `templates/advocate-report-template.md` (the **pre-existing**
  Clarifications block holds the exact question shape — Read and confirmed before landing;
  nothing was written to templates/ this wave, so D4's destination ban is not engaged — this is
  R4a dedup credit, not a relocation; the in-body copy had
  drifted: "Why this matters" vs the template's "Why it matters" — the same second-home symptom
  as this skill's v0.13.0 Output-Format strip) · deleted (the What-to-Avoid section — within-file
  triplication with the Core Principle table and When-NOT-to-Use routing; its one non-duplicated
  sentence folded into Core Principle) · deleted (three near-synonym When-to-Use bullets merged
  into one) · densified (form-only): eight Common-Mistakes ❌/✅ subsections → one 3-column table
- **Tier failed:** 1 (Question Format, What to Avoid) · 2 (the merged When-to-Use bullets — no
  distinct trigger per bullet) · n/a for the densification — all ❌/✅ pairs preserved
- **Content:** the fenced question template; the four avoid-bullets + closing line; two trigger
  bullets; Common-Mistakes subsection headers
- **Consumers assessed:** wave-open enumeration — 7 citing files, none reference the stripped
  sections; `mochiko:authoring-requirements` (both remaining pointers) verified to exist
- **Wave-1 reconciliation:** the +11 canonical hunt-taxonomy table (v0.25.0 RETURNED below) sits
  cleanly in Gap Categories — nothing else in the file duplicates it; both Gap-Categories tables
  KEPT as the canonical home `devils-advocate` points at

## [v0.26.0] KEPT: the severity table and Core Principle table
- **Tier-2 evidence:** contested at the wave-2 pass and kept — the severity table carries
  **spec-specific wording** ("Cannot build without this answer" / "Will cause rework") and this
  skill has no references/ tree to relocate to; the Core Principle wrong/right table is the
  skill's unique teaching content, now also carrying the folded altitude sentence. Session
  ruling: batch-2 ratification 2026-07-25.

## [v0.25.0] RETURNED: the five requirement-defect classes landed in Gap Categories (canonical-home landing, +11 lines)
- **Evidence:** wave-1 audit catch (skill-succinctness pass, 2026-07-25) — the `devils-advocate` agent's "What You Hunt For" catalog was stripped with disposition "relocated → this section" (R4b item 2), but the section held only the question-framing taxonomy; the pointer in the agent ("the canonical gap taxonomy … lives in `mochiko:review-specifications`") had been dishonest since before the wave. The five-class table (missing requirements / ambiguities / edge cases / assumption gaps / contradictions) landed here at audit-fix time, making this section the true canonical home. Provenance: `.mochiko/strips/devils-advocate.md`. This is a cross-primitive dedup landing (R4a Tier-1 credit), not a re-add of previously stripped content from this skill.

## [v0.13.0] Output Format block
- **Disposition:** relocated → `templates/advocate-report-template.md` (the report shape's single source; the skill now references it)
- **Tier failed:** 1 (altitude — a second home for the report structure, already drifted: the skill's block lacked the Verdict and What's-Strong sections the template carries)
- **Content:** the fenced `## Gaps Found` markdown block (Critical / Important / Minor buckets with Gap / Question / Options fields)
- **Consumers assessed:** specify only (user-ratified)

## [v0.13.0] Verdict-ownership Related-Skills bullet
- **Disposition:** deduped to the skill's own `description:` field + Overview (both already state input-not-verdict; the `review-*` family boundary is defined in REGISTRY's split note)
- **Tier failed:** 1
- **Content:** "**Verdict ownership** — the severity-bucketed gaps and clarifying questions this skill emits are INPUT to the reviewer/lead, who owns the clearing verdict and drives any revision round. This skill finds and frames gaps; it does not emit a clearing PASS/FAIL of its own."
