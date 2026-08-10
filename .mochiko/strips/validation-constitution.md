# Strip notes — `skills/validation-constitution/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness pilot wave — R1 live-defect
repair batch (design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified
2026-07-25). Description ledger — separate accounting from SKILL.md body lines per R2. Repair
context: R1 measurement (2026-07-25) found this skill delivered with NO description at all (bare
name in the listing) despite a healthy 1,363-char description on disk; byte scan clean;
strict-YAML invalidity disproven as the cause (8 other invalid-YAML descriptions deliver fine).
Surviving hypothesis: alphabetically-last entry dropped whole by a total listing budget (~23.8k
chars delivered). Repair = mass cut here + mass cuts on the two truncated review-* descriptions;
also strict-YAML-safe ("Checks:" → "Checks include"). Delivery verification deferred: the
in-session listing is a session-start snapshot, so the absent-fire probe re-runs in a fresh
session; if still absent there, the wave passes' further description-mass cuts are the remaining
remedy — tracked in the session record's open threads.

Wave-1 body strip landed 2026-07-25 at v0.25.0 (batch-ratified): body 274 → 212 lines, 62 cut =
23% — **under the 30–70 never-stripped band**; per R3 the under-band second pass generates the
survivor-provenance (KEPT) entries below rather than forcing cuts.

## [v0.63.0] Guardrails cut — body deletions + slim description (benchmark verdict)
- **Disposition:** superseded → benchmark-ruled guardrails body + slim description (`.mochiko/benchmarks/guardrails-vs-detail/variants/`)
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict — `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`)
- **Content (faithfully compressed).** Body 12,358 → 6,915 chars (−5,443, −44%; net of the +~181-char review-evidence floor line added as a pure addition). Description 1,131 → 481 chars. Sections removed:
  - **## When to Use** (the six-bullet list) — removed; When NOT to Use kept.
  - **### Step 2: Region and Surface Integrity (deterministic)** — the region-marker / index→home / paths-scope / scope-coverage / new-file-read-line checks.
  - **### Step 3: Check Each Principle (three-part, in the ledger)** — the enforcement/testability/rationale/home table.
  - **### Step 4: Trace Closure Cross-Check** — the both-ways manifest↔synthesis↔surface closure procedure and the semantic-fidelity advisory note.
  - **### Step 5: Floor, Module, and Waiver Checks** — the floor-header / essential-category / waiver-format (D4.2) / module-match checks.
  - **### Step 6: Scan for Anti-Patterns** — the standalone step (the references/ANTI-PATTERNS.md pointer survives in Quantification Requirements).
  - **### Step 7: Verify No Placeholders** — the generic every-set-member placeholder-pattern sweep.
  - Old description verbatim: "This skill MUST be invoked to grade a DRAFTED governance surface set against the quality checklist — there is NO constitution.md; the graded set is the CLAUDE.md governance region (between the mochiko:governance markers), the `paths`-scoped `.claude/rules/mochiko/` files, and the governance ledger (`.mochiko/memory/governance-ledger.md`), judged against the session synthesis and the producer's trace manifest. Checks include two-way trace closure, region-marker integrity, index→home existence, per-principle three-part structure (enforcement/testability/rationale), floor/module accounting and waiver-format checks (the D4 model), anti-pattern and placeholder scans, quantification enforcement, and semantic version-bump determination — emitting a binary PASS/FAIL verdict plus a fix list. SHOULD also invoke whenever the setup loop's validate step needs an independent grade of a surface set produced by mochiko:authoring-constitution, or when re-validating after a FAIL-loop revision. The validator-side skill of the governance producer↔validator pair; defaults to FAIL; run by an independent validator, never the author."
  - Verbatim removed text survives in three places: (a) git history of the original `plugins/mochiko/skills/validation-constitution/SKILL.md`; (b) the before/after pair in this tree — `.mochiko/benchmarks/guardrails-vs-detail/variants/body/validation-constitution/SKILL.md` (after) and the pre-edit original (before, in git); (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** goal/output contract (Overview "the graded artifact is a set" + the Step 9 VALIDATION RESULT block); the non-waivable floor ("Every set MUST pass — no exceptions", "Violating the letter … is violating the spirit"); Step 1's checklist load-and-assemble instruction with the `references/QUALITY-CHECKLIST.md` and `references/ANTI-PATTERNS.md` pointers; the anti-rationalization triad (Common Mistakes + Red Flags + Common Rationalizations); version-bump determination (Step 8); Related Skills. **Added (pure addition):** the review-evidence floor line in ## Overview.
- **Protected-content reconciliation.**
  - `[v0.25.0] KEPT: Red Flags + Common Rationalizations pair` — both survive the cut. Intact.
  - `[v0.25.0] KEPT: Step-7 placeholder pattern list` — the guardrails cut REMOVES Step 7. Recorded here as superseded-by-this-ruling. Section-bound placeholder coverage remains in `references/QUALITY-CHECKLIST.md` ("## No Placeholders Rule": `[PLACEHOLDER]`, `[COMMAND]`, `GI-XXX`), assembled at Step 1 — but the v0.25.0-KEPT distinction, Step 7's *generic sweep over every set member* ruled "not derivable from" those section-bound items, is exactly what this benchmark ruling supersedes.
  - The removed Steps 2–6 are procedure detail single-sourced to `references/QUALITY-CHECKLIST.md` (loaded at Step 1); none were separately KEPT-marked.
- **Consumers assessed:** no command references this skill (grep `plugins/mochiko/commands/` clean). `agents/validator.md` declares it in `skills:`; the kept goal/floor/checklist-pointer leave that composition intact.

## [v0.25.0] "Testing Evidence" RED/GREEN build record (28 lines)
- **Disposition:** deleted — full content in git history (removed at v0.25.0)
- **Tier failed:** 2 (provenance, not procedure — precedent: `testing-end-user`'s TESTING-EVIDENCE.md archived-deleted at v0.22.0 on the same ground)
- **Content:** three RED-phase pressure scenarios with captured rationalizations + the GREEN-phase verification that the skill's guards counter them
- **Consumers assessed:** none reference the section (12 consumer files checked at wave open)

## [v0.25.0] "Explicit Loophole Closures" folded into Common Rationalizations (net −19 lines)
- **Disposition:** relocated → the Common Rationalizations table (in-file): the two closures the table lacked ("user asked to skip validation" with its document-against-recommendation instruction; "missing parts → return to authoring") became table rows; the five subsections deleted
- **Tier failed:** 1 (third encoding of the same anti-rationalization guard — Red Flags names the thoughts, the table rebuts them; the paragraphs re-derived table rows)
- **Content:** five excuse-rebuttal subsections (looks fine / small change / add later / user asked to skip / just prototyping)
- **Consumers assessed:** none reference the section

## [v0.25.0] Step-6 anti-pattern preview table (4 rows)
- **Disposition:** relocated → `references/ANTI-PATTERNS.md` (already the canonical 91-line scan list, loaded at the scan step; pointer kept)
- **Tier failed:** 1 (preview copy of the reference's own rows)
- **Content:** vague-principle / missing-enforcement / placeholder-syndrome / generic-thresholds detection rows
- **Consumers assessed:** none reference the table

## [v0.25.0] Quantification example table (5 rows)
- **Disposition:** relocated → `references/ANTI-PATTERNS.md`. **Audit catch (wave-1 audit, 2026-07-25):** the initial "no append needed" claim was false for the latency pair (no performance quantification existed in the reference) and the secure-by-default pair lives under *Missing Enforcement* — a latency example was appended to *Generic Thresholds* and the SKILL.md pointer widened to name all three sections at fix time
- **Tier failed:** 1 (example rows restating the reference's patterns)
- **Content:** clean→lint / short→40 lines / coverage→≥80% / fast→<200ms p95 / secure→validated-inputs example pairs
- **Consumers assessed:** none reference the table

## [v0.25.0] KEPT: Red Flags + Common Rationalizations pair (two encodings, third folded)
- **Tier-2 evidence:** contested at the under-band pass and kept — recognize-the-thought (flags) + rebut-it (table) are one mechanism with GREEN-phase evidence: the deleted build record and `testing-end-user`'s v0.22.0 note both document agents citing these guards under pressure; validation is the library's most rationalized surface. Session ruling: batch-1 ratification 2026-07-25.

## [v0.25.0] KEPT: Step-7 placeholder pattern list
- **Tier-2 evidence:** contested (suspected QUALITY-CHECKLIST duplicate) and kept — the checklist's placeholder items (lines 69–73) are context-bound to specific sections (Technology Stack, Quality Gates, coverage, security, test commands); Step 7's generic pattern sweep over every set member is the "most commonly rationalized check" and is not derivable from them. Session ruling: batch-1 ratification 2026-07-25.

## [v0.24.0] DESCRIPTION: cut 1,363 → 1,105 chars + strict-YAML repair (absent-fire live defect)
- **Disposition:** deleted (description ledger)
- **Tier failed:** 2 — every dropped clause is checklist detail restated from the SKILL.md body; no trigger or boundary behavior lost
- **Content:** dropped clauses — the GI-element→home mapping parenthetical; the module-parameterized-checks item; the governance-intent file path (synthesis still named); the greenfield/brownfield-mode parenthetical; the different-agent-than-the-author parenthetical (never-the-author retained)
- **Consumers assessed:** delivery-side only — no file consumes description text
