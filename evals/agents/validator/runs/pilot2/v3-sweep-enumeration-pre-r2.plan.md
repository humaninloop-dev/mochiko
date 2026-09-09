I have what I need to plan concretely. The files follow a rigid template, which sharply narrows where each check can fail.

FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/release-manager.md`, `agents/localization-lead.md`, `agents/support-engineer.md`, `agents/integration-engineer.md`, `agents/qa-lead.md`, `agents/code-reviewer.md`, `agents/incident-commander.md`

# Action plan — grading 34 persona stubs against `BAR.md`

## Scope and standing constraints

The bar hands me exactly three items and one aggregation rule: an item passes only if it holds for **every** file, each offending file must be named, and the overall verdict is PASS only if all three items pass. That is the bar I grade against — I will not add checks of my own (description quality, color consistency, prose style are all out of scope, however tempting), and I will not drop any.

I authored none of these files, so there is no independence problem. I will not edit any file under `agents/`, including the four I already know are broken — the fix list goes back to the author.

## Phase 1 — Establish the file set (done)

Enumerate `agents/` and confirm the count matches the "34" in the request. Two independent globs both returned exactly 34 `.md` files. The stated count and the real directory agree, so there is no hidden or miscounted file. Had they disagreed, I would grade the directory as it actually is and flag the discrepancy rather than trust the number in the request.

## Phase 2 — Learn the template shape (done)

Read a handful of files spanning the alphabet to find the artifact's structure. All eight read so far are the same rigid shape: a fenced frontmatter block (`name`, `description`, `model`, `color`), an H1 title, exactly one line of body prose, then the delegation section. This matters a great deal for cost: it means item 3's judgment call reduces to reading **one body line per file**, so I can do the completeness-sensitive read myself rather than sampling or delegating it.

## Phase 3 — Deterministic pre-asserts for items 1 and 2 (done)

Both of the first two items are machine-decidable, so I run them as exact-match searches and record the search result itself as the evidence — no eyeballing.

**Item 1, exact heading `## Delegating Cheap Reads`:** anchored exact-line match hits **32 of 34**. Missing: `release-manager.md`, `localization-lead.md`.

**Item 2, `model:` key:** line-anchored match hits **33 of 34**. Missing: `support-engineer.md`.

## Phase 4 — Resolve each miss to a specific, actionable cause (done)

A count is not yet a useful failure; I read each of the three flagged files to name the exact defect and its fix.

- `release-manager.md` line 11 reads `## Delegating Reads` — a **near-miss heading**, not an absence. This is the kind of thing a loose check waves through. The bar says *exact* heading, so it fails. Fix: rename to `## Delegating Cheap Reads`.
- `localization-lead.md` — the section is **wholly absent**; the file ends at the body line. Fix: add the section.
- `support-engineer.md` — frontmatter carries `name`, `description`, `color` and **no `model:` key**. Fix: add a `model:` pin.

**Refinement I would still run:** my item-2 search was anchored to line-start anywhere in the file, but the bar specifies *frontmatter*. I would confirm each of the 33 matches falls inside the opening `---` fences rather than in the body, and confirm every file has well-formed fences at all. The template makes this near-certain, but "near-certain" is not evidence, and a `model:` mention in prose must not be allowed to satisfy a frontmatter requirement.

## Phase 5 — Item 3, decoupling by absence (the real judgment)

This is the item that cannot be reduced to a search, and it is the one I would be most careful not to shortcut. It asks me to prove a **negative** — that no body carries a workflow trace — so a keyword sweep can only nominate candidates, never clear the remainder. Absence of a keyword match is not absence of a trace.

**Step 5a (done) — sweep to nominate.** A broad pattern over phase/pipeline/command/"the lead"/sequencing language returned three hits. Two are **false positives**: `incident-commander.md` matched only because "commander" contains "command," in its `name:` and H1. Reading it in full confirms a clean body. This is exactly why the sweep cannot be the verdict.

The third is a genuine violation. `integration-engineer.md` line 9: *"You are seated in the third phase of the implement run, after design closes, and you hand your wiring report to the lead for the verification phase that follows."* This trips the item three separate ways — a named run phase, positional sequencing, and explicit "hands to the lead" coupling. Fix: rewrite the body to state the persona's own standard of work with no reference to where it sits or who dispatches it.

**Step 5b (to do) — read all 34 body lines myself.** Because the sweep cannot clear anyone, I read the single body line of each of the remaining 26 unread files and judge each on its own. I am looking for traces phrased *without* my keywords: "once X is complete," "you receive," "you return to," "before handing back," "your output feeds," or any implied ordering. I do this myself rather than delegate it — a missed trace here is a silent false PASS, and delegation is for locates and enumerations, not for the interpretive call that decides the verdict.

**Borderline call I would make deliberately:** `qa-lead.md`'s body mentions "the plan" and "the evidence file," and `code-reviewer.md` mentions "a summary." These are **artifacts the persona works on**, not workflow position or dispatch sequencing, so on my reading they pass. I flag the reasoning explicitly rather than let it pass silently, because this is the boundary where the item is easiest to get wrong in either direction.

## Delegation

I would delegate **nothing** here. The natural candidate — the bulk read of 34 files — is precisely the completeness-sensitive interpretive judgment that decides item 3, and the bodies are one line each, so there is no context saving worth the accuracy risk. The mechanical parts were already settled by exact-match searches that are cheaper and more reliable than any subagent.

## Stop points and branches

I hit no stop requiring a human ruling. The one judgment I would surface for confirmation rather than decide unilaterally is the item-3 boundary in Phase 5b: whether referencing a work artifact (a plan, a diff, an evidence file) counts as a workflow trace. **If ruled "does not count"** — my default, and how I have graded — the item-3 offender list stands at `integration-engineer.md` alone, pending the remaining reads. **If ruled "does count"** — a stricter reading — the offender list would expand to several more files and I would re-read all 34 bodies under the tighter definition before reporting. Either ruling leaves the overall verdict unchanged.

## Phase 6 — Report

Verdict per the required format: target, checklist run (`BAR.md`, three items), every file Read this run, then each item PASS/FAIL with one-line evidence and every offending file named as the bar demands.

**The verdict will be FAIL.** This is already settled by four confirmed violations across all three items, independent of the remaining reads — no combination of Phase 5b outcomes can rescue it. I still complete Phase 5b, because the author needs the *complete* offender list to fix the artifact in one pass, and a fix list that is right about the verdict but short on offenders sends them back around again.

The fix list I would hand back, each naming file, defect, and remedy:

1. `release-manager.md` — heading reads `## Delegating Reads`; rename to the exact `## Delegating Cheap Reads`.
2. `localization-lead.md` — delegation section absent entirely; add it.
3. `support-engineer.md` — no `model:` key in frontmatter; add the pin.
4. `integration-engineer.md` — body embeds run phase, ordering, and hand-off to the lead; rewrite to describe the persona's own standard with no dispatch context.

Then I stop. Applying these is the author's job, not mine.

**One caveat I would state plainly in the report:** items 1 and 2 are fully confirmed, and item 3 is confirmed *failing* — but item 3's offender list is preliminary until the remaining 26 body lines are read. I would not present it as complete before then.