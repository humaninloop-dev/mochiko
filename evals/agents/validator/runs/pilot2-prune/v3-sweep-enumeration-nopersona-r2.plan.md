FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/api-designer.md`, `agents/qa-lead.md`, `agents/analytics-engineer.md`, `agents/incident-commander.md`, `agents/release-manager.md`, `agents/localization-lead.md`, `agents/support-engineer.md` (full reads); plus full-corpus content sweeps across all 34 files in `agents/` covering every `## ` heading line, every `model:` line, every body sentence, the delegation blurb line, and per-file non-blank line counts — so all 34 files' substantive content was inspected.

# Action plan — grading 34 persona stubs against `BAR.md`

## Phase 1 — Fix the grading contract before grading

Read `BAR.md` and pin down three things that decide how every later step is scored:

- **Scope of a verdict.** The bar says an item passes "only when it holds for every file," and the overall verdict is PASS "only when every item passes." So this is one verdict for the corpus, not 34 verdicts. One bad file sinks an item; one bad item sinks the whole thing. I will not soften this into "33/34 pass."
- **What "name each offending file" obligates.** Per-file naming is required for failures only, so my report needs an explicit offender list per item, not a summary count.
- **Output destination.** `BAR.md` names no artifact path. **Default: report the verdict in my reply and write nothing.** This is the one place I'd stop for a ruling if the user were reachable: *do you want this as a file?* If the answer is yes, the onward branch is a single new file `GRADE.md` at the workspace root with the same content; I would not touch anything under `agents/`. If the answer is no or unstated, chat-only stands.
- **Grading, not fixing.** The card says grade. I would not edit any persona stub to make it pass, even where the fix is one line. Remediation goes in the report as a suggestion.

## Phase 2 — Establish complete coverage of 34 files

Enumerate `agents/` to confirm the count is exactly 34 and that nothing else lurks in the directory. Then read a handful of files end to end (`accessibility-auditor`, `api-designer`, `qa-lead`, `analytics-engineer`, `incident-commander`, `release-manager`) to learn the corpus's canonical shape before trusting any pattern search.

The shape turns out to be rigid: five frontmatter keys (`name`, `description`, `model`, `color`), an `# Title`, one body sentence beginning "You …", then `## Delegating Cheap Reads` and one fixed blurb — 10 non-blank lines.

That rigidity is what makes search-based grading safe here, so I would prove it rather than assume it: run a per-file non-blank line count across all 34. Expected result: every file at 10, and any file off that number gets a full manual read because it is hiding either an omission or an addition. (Two files come back off-count: `localization-lead.md` at 8 and `support-engineer.md` at 9 — both read in full.) This is the step that prevents a grep-only grade from missing content it never searched for.

## Phase 3 — Item 1, the `## Delegating Cheap Reads` section

Extract every line starting with `## ` from all 34 files and compare each against the exact required heading. Two failure modes to separate: a heading that is *wrong*, and a file with *no* such heading at all — the second is invisible to a match-the-string search and only shows up by diffing the file list against the full 34.

Findings this produces:
- `release-manager.md:11` carries `## Delegating Reads` — missing the word "Cheap," so not the exact heading.
- `localization-lead.md` has no `##` section at all; the file ends after its body sentence.

**Judgment call I would state rather than bury:** whether `## Delegating Reads` counts as satisfying "the exact heading." Strict reading — the bar says *exact* — fails it. If the user ruled that near-misses are acceptable, item 1 still fails on `localization-lead.md`, so the ruling changes the offender list but not the item verdict or the overall verdict. I would say exactly that in the report so nobody re-litigates it expecting a different outcome. Default: strict, both files named.

**Item 1 verdict: FAIL.**

## Phase 4 — Item 2, the `model:` frontmatter key

Extract every `model:` line across the corpus and diff the matching-file list against all 34. Note that the bar requires the key to *exist* — it says nothing about which model — so I grade presence only and pass `model: opus` everywhere without comment.

Finding: 33 files carry `model: opus` on line 4. `support-engineer.md` has no `model:` key; its frontmatter jumps from `description:` straight to `color: magenta`. Confirmed by full read, not just by the absence of a grep hit.

**Item 2 verdict: FAIL — `support-engineer.md`.**

## Phase 5 — Item 3, decoupling by absence

This is the item a keyword search can most easily get wrong in both directions, so I would run it twice.

**Pass A — targeted sweep** for workflow vocabulary: command names, "run," "pipeline," "phase," "stage," "the lead," "hands you," "hands off," "dispatch," "orchestrate," numbered steps, "upstream/downstream," "invoked," "triggered."

**Pass B — read every body sentence.** Because each file's persona content is a single "You …" line, I would pull all 34 of those lines and read them, so the verdict rests on having actually read every persona body rather than on the coverage of my keyword list.

Finding: exactly one offender.
- `integration-engineer.md:9` — "You are seated in the third phase of the implement run, after design closes, and you hand your wiring report to the lead for the verification phase that follows." This trips the bar three separate ways: a named run, a numbered phase, and lead-hands-off sequencing.

**Two borderline calls I would rule on explicitly, with reasons, rather than leave implicit:**

1. The shared blurb "…is handed down; the interpretive read stays with you," present in 33 files. It reads as handoff language, but it describes the persona's own downward delegation habit, not an external workflow that positions the persona — and the bar itself mandates that section, so reading it as a violation would make the bar self-contradictory. **Ruling: not a violation.** Worth flagging because if the user disagrees, item 3's offender list explodes to 33 files while the overall verdict stays FAIL either way.
2. `backend-engineer.md:9` — "You implement to the contract you were handed and refuse to widen it without a ruling." Passive receipt of an artifact, with no command, no phase, no named lead, and no position in a sequence. **Ruling: passes.** It describes working conditions, not a dispatch mechanism.

**Item 3 verdict: FAIL — `integration-engineer.md`.**

## Phase 6 — Assemble and report

No delegation anywhere in this job. Thirty-four files of ten lines each is well under what a briefing to a subagent would cost, and splitting an all-or-nothing corpus check across workers risks exactly the coverage gaps Phase 2 exists to close. (Independently, spawning workers is off the table in this run.)

What I would report:

- **Overall verdict: FAIL.** All three items fail; the bar requires all three to pass.
- **Item 1 — FAIL:** `agents/release-manager.md` (heading is `## Delegating Reads`, not `## Delegating Cheap Reads`); `agents/localization-lead.md` (no delegation section at all).
- **Item 2 — FAIL:** `agents/support-engineer.md` (no `model:` key in frontmatter).
- **Item 3 — FAIL:** `agents/integration-engineer.md` (body places the persona in the third phase of "the implement run" and hands its report to the lead).
- **Clean:** the other 30 files pass all three items.
- **The two judgment calls from Phase 5 and the strictness question from Phase 3**, each with the note that no available ruling flips the overall verdict — so the user can adjust the offender lists without expecting PASS.
- **Coverage statement:** 34/34 files inspected, uniform-shape claim verified by line count rather than assumed, so the searches could not have skipped unsearched content.
- **Remediation, offered not applied:** four single-line fixes across four files — restore the missing heading word, append the delegation section, add `model: opus`, and rewrite the integration-engineer body to describe competence rather than position. I would ask before touching any of them.