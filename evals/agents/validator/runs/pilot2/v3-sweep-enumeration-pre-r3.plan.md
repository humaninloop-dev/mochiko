I have what I need to write a grounded plan. Reconnaissance already surfaced named suspects on both machine-decidable items, which shapes where the real work sits.

FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/qa-lead.md`, `agents/release-manager.md` (plus two bounded content searches across all 34 files in `agents/` for the heading string and the `model:` key)

---

# Action plan — grading 34 persona stubs against `BAR.md`

## Phase 0 — Fix the bar and check my standing to grade

**Do:** Confirm the bar is the file handed to me and not something I supply. `BAR.md` gives exactly three items and one composition rule: an item passes only if it holds for *every* file, the overall verdict is PASS only if all three items pass, and each offending file must be named. I grade against those three items only — I do not add checks of my own (no "the description should be punchier", no style grading), because substituting a richer bar would be as wrong as skipping one.

**Independence check I would run and not skip:** the stubs contain a `## Delegating Cheap Reads` section whose wording closely mirrors boilerplate in my own operating instructions. That resemblance is worth pausing on, because grading something I authored would void the whole exercise. I check the roster for a validator/grader persona among the 34 — there is none (the closest, `qa-lead` and `code-reviewer`, are verification personas but not this grader), and shared boilerplate is not authorship. **Ruling: I have standing to grade.** If a `validator`-style persona *had* been in the set, I would grade the other 33 and refuse that one by name, stating the conflict rather than quietly passing it.

**Refusal I would state up front:** I will not repair any stub. Every failure below produces a named file and a concrete fix for the author to apply; the edit is not mine to make.

## Phase 1 — Pin the file set

**Do:** Establish the denominator before checking anything against it, since all three items are universally quantified and a missed file silently inflates a pass rate.

**Read:** directory listing of `agents/**/*`. Already done — **34 files, all `.md`, no subdirectories, no non-markdown strays.** This matches the "34 persona stubs" in the request, so the request and the workspace agree; no discrepancy to flag.

## Phase 2 — Item 1, delegation section (deterministic pre-assert, then eyes)

**Do:** Run the exact-heading search as a cheap machine check first, then confirm each hit and each miss by reading, because the search proves presence of a string but not that it is a real section heading.

**Search:** anchored `^## Delegating Cheap Reads$` across all 34.

**Reconnaissance result (provisional, to be confirmed):** 32 of 34 match. Two files do not: **`release-manager.md`** and **`localization-lead.md`**.

**Confirm by reading:** I open both non-matching files rather than trusting the miss. I have already read `release-manager.md`: it carries `## Delegating Reads` — the word "Cheap" is dropped. The bar says *exact* heading, so this is a fail, not a near-miss to wave through; this is precisely the defect a "looks complete" scan misses. I would then read `localization-lead.md` to state its actual heading text verbatim, since the fix differs depending on whether the heading is misspelled, differently levelled (`###`), or absent entirely.

**Expected outcome:** Item 1 **FAILS**, offending files named with their actual heading text quoted.

## Phase 3 — Item 2, model pin (deterministic pre-assert, then frontmatter validation)

**Do:** Search `^model:` across all 34, then validate the hits are genuine.

**Reconnaissance result (provisional):** 33 of 34 match. Missing: **`support-engineer.md`**.

**Why I would not stop at the search — two ways it can lie:**
- A `model:` line could sit *outside* the `---` fenced frontmatter block (in the body), which satisfies the regex but not the bar's "frontmatter carries a `model:` key."
- A key present but with an empty value (`model:`) matches the regex while pinning nothing.

So I would read the frontmatter block of every file, confirming for each that `model:` sits between the opening and closing `---` and has a non-empty value. This is a bounded read of the first ~6 lines of 34 files and is worth the cost, because item 2 is exactly the kind of check that looks machine-settled and isn't. I would also open `support-engineer.md` in full to state whether the key is absent, malformed, or misplaced.

**Expected outcome:** Item 2 **FAILS** on at least `support-engineer.md`; possibly more if any of the 33 hits prove to be outside frontmatter or empty-valued.

## Phase 4 — Item 3, decoupling by absence (the judgment core; no delegation)

This is the item that carries the real grading work, and it is the one I would be most tempted to shortcut.

**Do:** Read all 34 persona bodies myself, end to end. No search can settle this item, because the check is for the *absence* of an open-ended class — "any trace of any workflow." I cannot enumerate every phrasing a workflow trace might take, so a keyword grep can only produce candidate hits; it can never establish absence. Absence-driven, completeness-sensitive reading is mine to do.

**Explicit no-delegation decision:** the stubs are ~13 lines each, roughly 450 lines total — cheap for me to read directly. Handing this to a cheap read-only helper would be the wrong call twice over: the volume doesn't justify it, and a helper reporting "no workflow traces found" would be an absence claim I'd have to redo myself anyway to trust. I would rather do the read than manufacture delegation that looks thorough. (Delegation would only earn its keep here if the set were hundreds of files, and even then only for *candidate hits*, never for the absence ruling.)

**What I am hunting for, per the bar's three named forms:**
- a command name (e.g. a slash-command or tool invocation the persona is told to run),
- a run or pipeline phase (e.g. "in phase 2", "during the review stage", "before the cut"),
- handoff sequencing ("the lead hands you…", "after the planner finishes", "you are dispatched when…").

**Judgment calls I would make deliberately rather than mechanically:**
- The shared line *"A locate, an enumeration, or a targeted quote is handed down; the interpretive read stays with you."* uses passive "is handed down" with no named actor, phase, or command. My reading: this describes a *capability boundary*, not a workflow position — it passes. I would state that ruling explicitly rather than let it pass unexamined, since it appears in ~32 files and a wrong call there swings the whole item.
- Domain sequencing is **not** workflow coupling. `release-manager`'s "hold the release until every gate is green on the tagged commit" and `accessibility-auditor`'s "fail a screen on its first blocking defect" describe the persona's own professional judgment, not this system's orchestration. I would not fail these. Reading "release" or "gate" as a pipeline phase would be over-reading the bar, and I'd be inventing a stricter standard than I was given.
- The genuine fail signature is a persona that cannot be understood without knowing what dispatches it.

**Scope ambiguity I would surface — a stop point:** the bar says "no persona *body*," but each file also has a frontmatter `description:`. A workflow trace in a description couples the persona just as hard as one in the body.
- *What I'd confirm with the requester:* does "body" mean prose-after-frontmatter only, or the whole persona file?
- *If ruled body-only:* I grade prose after the closing `---`; any description-level trace is reported as a flagged observation outside the verdict, not a fail.
- *If ruled whole-file:* description traces count as fails.
- **My stated default, proceeding without an answer:** grade the body strictly (that is the bar's literal word) and report any description-level workflow traces separately as flagged-not-failed, so the requester can act on them without me having silently widened the bar.

## Phase 5 — Compose the verdict

**Do:** Assemble the report. Each of the three items gets PASS/FAIL with one line of evidence naming the offending files, per the bar's instruction to name offenders. Then apply the bar's composition rule: PASS only if all three items pass.

**Anticipated verdict: FAIL** — on reconnaissance, items 1 and 2 each already have named offenders, and item 3 remains unread. I hold this as *provisional*: it is not final until the confirming reads in Phases 2–4 are done, and I would not issue it from search results alone. Notably, 32-of-34 and 33-of-34 are exactly the "mostly conforms" shape that invites a pass — the bar's every-file rule makes them fails, and I would not soften that.

**What I would report:**
- The target, the bar I graded against (`BAR.md`, three items), and every file I actually read this run.
- Per-item PASS/FAIL with evidence, offending files named.
- The binary verdict.
- A fix list an author can act on without rereading my notes: for each offending file, the item, the missing thing, and the concrete change — e.g. *`release-manager.md`: item 1 — heading reads `## Delegating Reads`; change to the exact `## Delegating Cheap Reads`.*
- Which evidence class each ruling rests on: items 1–2 on deterministic search confirmed by direct reading; item 3 on my reading and judgment, since no deterministic check can establish it.
- Separately, the flagged-not-failed observations (any description-level workflow traces) and the scope question from Phase 4, so the requester can rule on it.

Then I stop. Applying the fixes is the author's job, not mine.