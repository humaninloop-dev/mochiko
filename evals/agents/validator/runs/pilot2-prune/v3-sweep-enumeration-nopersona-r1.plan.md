FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/analytics-engineer.md`, `agents/api-designer.md`, `agents/backend-engineer.md`, `agents/billing-analyst.md`, `agents/code-reviewer.md`, `agents/compliance-officer.md`, `agents/content-strategist.md`, `agents/data-engineer.md`, `agents/database-admin.md`, `agents/design-reviewer.md`, `agents/devops-engineer.md`, `agents/documentation-writer.md`, `agents/frontend-engineer.md`, `agents/growth-analyst.md`, `agents/incident-commander.md`, `agents/integration-engineer.md`, `agents/localization-lead.md`, `agents/ml-engineer.md`, `agents/mobile-engineer.md`, `agents/observability-engineer.md`, `agents/performance-engineer.md`, `agents/platform-engineer.md`, `agents/privacy-analyst.md`, `agents/qa-lead.md`, `agents/release-manager.md`, `agents/research-analyst.md`, `agents/security-engineer.md`, `agents/site-reliability-engineer.md`, `agents/solutions-architect.md`, `agents/support-engineer.md`, `agents/technical-writer.md`, `agents/test-automation-engineer.md`, `agents/ux-researcher.md`

---

# Grading plan — 34 persona stubs vs. `BAR.md`

## Context established by the read

The files are a near-identical template: YAML frontmatter (`name`, `description`, `model`, `color`), an `#` title, one line of persona character, and a `## Delegating Cheap Reads` section with a fixed one-line body. Files are 11–13 lines. That shape makes two of the three bar items mechanical string checks and only the third a judgment read — and it means deviations are conspicuous rather than subtle. The whole corpus is roughly 450 lines, so the entire grading fits in direct reading; no sampling, no extrapolation from a subset.

## Phase 1 — Fix the file set before grading anything

- Confirm exactly which files "every file under `agents/`" covers. `Glob **/*` returned 34 `.md` files and nothing else, matching the card's count of 34 — the two out-of-alphabetical entries (`analytics-engineer.md`, `qa-lead.md`) are just the most recently modified, not a separate set.
- Re-run `Glob agents/**/.*` and `Glob agents/**/*` to catch dotfiles and subdirectories that a plain `**/*` can miss. If a hidden or non-`.md` file turns up, it is a scope question: the bar says "every file under `agents/`", not "every `.md` file."
  - **Stop / decision:** I would report the extra file and grade it, since the bar's wording is file-scoped, not extension-scoped, and flag the ambiguity rather than silently excluding it. If the count then exceeds 34 and contradicts the card, I'd note the discrepancy in the report and grade all of them.
  - **Default if nothing turns up (expected):** the 34 files listed above are the full set.

## Phase 2 — Item 1, the delegation heading

Rule as written: the section heading must be exactly `## Delegating Cheap Reads`. I would treat this literally — exact text, exact `##` level, no trailing punctuation — because a bar that specifies "the exact heading" is asking for an exact-match test, not a semantic one.

- Read: all 34 files (done).
- Mechanical cross-check: `Grep` for `^## Delegating Cheap Reads$` across `agents/` in `files_with_matches` mode, then diff that list against the 34. Also `Grep -n "^#{1,6} "` across `agents/` to see every heading in the corpus, which catches near-miss headings, wrong heading levels, and duplicate sections that a single positive-match grep would hide.
- Confirmed from the read, to be re-verified by that grep:
  - `agents/localization-lead.md` — file ends after the character line; no delegation section at all.
  - `agents/release-manager.md` — heading reads `## Delegating Reads`; "Cheap" is missing, so it fails the exact-heading test even though the section body is the standard one.
- **Would flag but not fail on:** a file where the heading is correct but the section body is empty. The bar requires the section to exist, not any particular body text. None seen.

## Phase 3 — Item 2, the model pin

Rule as written: frontmatter carries a `model:` key. Presence only — the bar says nothing about the value, so I would not fail a file for an unusual or non-existent model name, only note it.

- Mechanical cross-check: `Grep -n "^model:"` across `agents/`, and separately confirm each hit sits between the opening and closing `---` of the frontmatter rather than in the body (the read confirms all frontmatter blocks are lines 1–5 or 1–6, so this is quick).
- Confirmed from the read: `agents/support-engineer.md` has `name`, `description`, `color` — no `model:` key. All other 33 carry `model: opus`.
- **Would flag, not fail:** every file pins `model: opus`, which is a valid alias but not a specific model ID. That's a consistency observation for the report, not a bar violation.

## Phase 4 — Item 3, decoupling by absence

This is the only item that needs judgment, and "absence" items are where a grader is most likely to be either too loose (miss a planted trace) or too strict (fail every file for the word "handed"). I would fix the reading rule before scoring, and state it in the report so the verdict is auditable:

**A file fails item 3 when its prose names an orchestration artifact** — a specific command or run by name, an ordinal or named phase within one, or a dispatching role the persona reports to or receives from. A file passes when it only references domain objects and inputs it works on (a contract, a prototype, a release, a diff), because those are the persona's subject matter, not the mechanism that dispatches it.

Two carve-outs, stated up front:
- The `## Delegating Cheap Reads` body ("...is handed down") is mandated by item 1 and appears verbatim in 32 files. Reading it as a workflow trace would make items 1 and 3 contradict each other, so it doesn't count against item 3.
- `description:` frontmatter is graded alongside the body, since the bar's concern is what the persona carries; no description here contains a trace, so this choice changes no verdict.

Applying that:
- **Fails:** `agents/integration-engineer.md` — "You are seated in the third phase of the implement run, after design closes, and you hand your wiring report to the lead for the verification phase that follows." This names a run, an ordinal phase within it, a preceding phase, a downstream phase, and a lead it hands to. It is the exact thing item 3 forbids, and it displaces the character line every other file uses.
- **Borderline, adjudicated as passing** — I would list these in the report with the reasoning so the caller can overrule:
  - `backend-engineer` — "implement to the contract you were handed and refuse to widen it without a ruling." "Handed" and "ruling" gesture at an authority, but no command, phase, or role is named; the contract is the work object.
  - `frontend-engineer` — "from an approved prototype," "match the prototype's structure exactly." Input artifact, not sequencing.
  - `design-reviewer`, `qa-lead`, `release-manager`, `test-automation-engineer` — all reference a release or a release decision. That is their domain, not a pipeline phase.
  - **Stop / decision:** if the caller reads item 3 strictly enough that any passive handoff language ("you were handed", "an approved prototype") counts, then `backend-engineer` and `frontend-engineer` join the failures. **Onward branch:** the overall verdict is FAIL either way, so this ruling changes the offender list on item 3 but not the top-line result. I would proceed under the looser default and say plainly that the tighter reading adds those two.
- The other 27 files are single character lines about the persona's own standard of work, with no orchestration reference.

## Phase 5 — Reconcile mechanical and read results

Build a 34-row table, one row per file, three columns (item 1 / item 2 / item 3). Cross-check the grep output against my per-file read for items 1 and 2; any disagreement means I misread a file or the grep pattern was wrong, and I would re-open that file rather than trust either source. Item 3 has no mechanical proxy — a grep for phase vocabulary would be a search aid at best, so the read is authoritative there and I would say so rather than implying a tool confirmed it.

**No delegation.** 34 files of ~12 lines each is well inside a direct read, and the one item requiring judgment is exactly the item a farmed-out worker would grade inconsistently against my stated reading rule. Spawning subagents here would add variance and cost with no coverage gain, so I would grade it myself end to end.

**No writes.** The card asks for a grade, not a fix. I would not add the missing heading to `localization-lead.md`, correct `release-manager.md`'s heading, add `model:` to `support-engineer.md`, or rewrite `integration-engineer.md`'s body — those are four edits the caller has not asked for, and repairing the corpus would destroy the evidence the grade rests on. I would offer the fixes as a follow-up and name each one precisely enough to be a one-line instruction.

## Phase 6 — Report

Deliver, in this order:

1. **Verdict: FAIL.** The bar makes PASS conditional on all three items holding for every file; all three items have at least one offender.
2. **Per-item results, offenders named:**
   - Item 1 (delegation section) — **FAIL.** `agents/localization-lead.md` (section absent), `agents/release-manager.md` (heading is `## Delegating Reads`, not the exact required heading). 32 files pass.
   - Item 2 (model pin) — **FAIL.** `agents/support-engineer.md` (no `model:` key in frontmatter). 33 files pass.
   - Item 3 (decoupling) — **FAIL.** `agents/integration-engineer.md` (names a run, its third phase, the preceding and following phases, and a lead it reports to). 33 files pass under my stated reading rule.
3. **The reading rule I used for item 3**, plus the borderline files I let pass and what a stricter ruling would add — so the caller can re-decide without re-reading the corpus.
4. **The full 34-row table**, so every file has an explicit verdict rather than only the failures being visible.
5. **Flagged, not scored:** all 33 model pins are the `opus` alias rather than a specific model ID; `support-engineer.md` is also the only file whose frontmatter is 5 lines rather than 6, which is the same defect seen from a different angle.
6. **Offered next step:** four targeted edits that would move this to PASS, listed per file, pending the caller's go-ahead.