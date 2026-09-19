I've read the bar and two representative stubs. That's enough to plan concretely; I'll stop short of running the actual grade.

FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/qa-lead.md`

---

# Action plan — grading 34 persona stubs under `agents/` against `BAR.md`

## Observations that shape this plan

The bar has exactly three items, and they are not the same *kind* of check. Items 1 and 2 are machine-decidable presence checks. Item 3 is an **absence** check over open-ended content — no finite search can prove it. That asymmetry drives the whole plan: items 1–2 get deterministic pre-asserts, item 3 gets my own eyes on all 34 files.

The two stubs I read are byte-identical in structure and share a verbatim delegation line. That reads as template-generated output, which tells me where the risk lives: **not** in the typical file but in whichever files diverged from the template. The directory listing returns `analytics-engineer` and `qa-lead` out of alphabetical order, i.e. with later modification times than the other 32 — appended after the main batch, plausibly by a different hand. Those are my highest-suspicion files, though suspicion earns them scrutiny, not a failing mark.

## Phase 1 — Verify the premise before grading it

The card asserts "34 persona stubs." I do not take that as given, because the bar says *grade every file under `agents/`* — if the directory holds 35, grading 34 is a silent miss.

- **Read:** directory listing of `agents/**` (already done — returns exactly 34 files, all `.md`, no subdirectories, no stray non-persona files).
- **Result:** premise confirmed. Count matches, and there is no file the bar covers that the card omitted.
- **Stop condition:** had the count differed, I would report the discrepancy up front and grade the *actual* file set, not the claimed one.

## Phase 2 — Independence and bar sufficiency

- Confirm I authored none of these stubs. I did not; they were handed to me in this workspace. Had any turned out to be mine, I would refuse the grade outright rather than grade my own output.
- Confirm the bar is present and self-contained. `BAR.md` is explicit, names its three items, and states the aggregation rule ("an item passes only when it holds for every file"; "PASS only when every item passes"). I therefore grade against *this* bar and do not substitute my usual persona-file criteria — notably, I will **not** fail a file for a thin body, a missing color, a weak description, or anything else this bar does not ask about. Those are not the bar I was given.

## Phase 3 — Deterministic pre-assert, item 1 (delegation heading)

The bar demands the **exact** heading `## Delegating Cheap Reads`. The failure mode I am hunting is the near-miss that a sloppy search waves through: `### Delegating Cheap Reads`, lowercase `cheap reads`, a trailing colon, or the phrase appearing in prose rather than as a heading.

- **Method:** two searches, then diff them.
  - Strict: line-anchored exact match on `^## Delegating Cheap Reads$` across `agents/`, file list output.
  - Loose: case-insensitive search for `delegating cheap reads` anywhere, file list output.
- **The evidence is the set difference.** Files in loose-but-not-strict are near-misses and **fail** item 1 (they have the words, not the heading). Files in neither are outright missing the section and fail. Only files in the strict set pass.
- **Expected:** all 34 in both sets, difference empty. Both files I read carry the heading exactly. If the expectation holds, item 1 passes; if the difference is non-empty, I name every file in it.
- I record the actual search output as the evidence line, rather than asserting "headings look fine."

## Phase 4 — Deterministic pre-assert, item 2 (model pin)

The bar requires `model:` **in frontmatter**, not merely somewhere in the file. A stub whose body discusses model choice in prose while the frontmatter lacks the key would satisfy a naive search and fail the bar.

- **Method:** line-anchored search for `^model:` across `agents/`, with line numbers.
- **Adjudication:** a hit only counts when its line number falls between the opening `---` and the closing `---`. In the two files I read, frontmatter spans lines 1–6 and `model:` sits at line 4. Any hit at a line number past a file's closing fence is a body mention, not a pin, and fails.
- Because the files are ~13 lines, Phase 5's full read independently confirms every frontmatter block anyway — so this item ends up double-covered, deterministically and by eye.
- **Expected:** 34 hits, all inside frontmatter (`opus` in both samples). I do not grade *which* model is pinned; the bar asks only that the key exist.

## Phase 5 — Full read of all 34 bodies for item 3 (the real work)

This is where the grade is actually decided, and it cannot be shortcut. "No trace of any workflow" is an absence claim: a search can prove a violation exists but can never prove none does. So I read every one of the 34 files myself, in full — roughly 450 lines total, entirely affordable.

**Delegation decision, stated plainly:** I would dispatch no subagent here. Delegation earns its keep when a bulk sweep would otherwise flood my context; 450 lines does not. More importantly, item 3 is an absence check whose outcome drives the verdict, and that class of read stays with me — a cheap worker reporting "I found nothing" is not evidence that nothing is there. Had `agents/` held several hundred stubs, I would have split the alphabetical range across a few cheap read-only workers with a brief to return, per file, the verbatim frontmatter block and every body line, quoted with file and line number — facts only, no judgment — and I would still have made every pass/fail ruling myself from their quotes, spot-checking a sample against the real files to confirm they quoted faithfully. At 34 files that machinery costs more than it saves.

**What counts as a workflow trace (fails item 3):**
- a command or invocation name (`/plan`, `/ship`, a backticked command)
- a named run or pipeline phase ("in phase 2", "at intake", "during the review gate")
- sequencing or handoff language positioning the persona in someone else's chain ("the lead hands you", "after the architect finishes", "return your output to the orchestrator", "then pass it to")
- another persona named as a pipeline neighbor
- artifacts framed as pipeline slots ("the card you were dealt", "the run folder")

**What does NOT count — and where I expect the temptation to over-fail:** domain vocabulary belonging to the persona's own craft. `release-manager` will legitimately speak of releases, `incident-commander` of incidents, `code-reviewer` and `design-reviewer` of reviews, `qa-lead` of verification plans, `devops-engineer` and `platform-engineer` of pipelines *as infrastructure they build*. The discriminating question I apply to each sentence: **does this describe the persona's own competence, or its position in an external orchestration?** `accessibility-auditor`'s "you fail a screen on its first blocking defect" is a method, not a workflow, and passes. A CI pipeline a `devops-engineer` maintains is a work product; a pipeline phase the persona is slotted into is a violation. I will hold that line rather than fail files on keyword resemblance, because over-failing is as much a bad grade as under-failing.

- **Read:** all 34 files under `agents/` (2 already read; 32 remaining).
- **Output per file:** pass/fail on item 3, and for any fail, the offending sentence quoted with its line number.
- **Expected:** given the template uniformity, most files pass. I hold the verdict open for the divergent-mtime files (`analytics-engineer`, `qa-lead` — the latter already read and clean) and for the role-adjacent ones named above.

## Phase 6 — The ambiguity I would stop on

The shared boilerplate line reads: *"A locate, an enumeration, or a targeted quote is **handed down**; the interpretive read stays with you."* That is passive handoff language, and a maximally literal reading of item 3 would fail all 34 files on it.

- **What I would confirm with the bar's owner:** does the delegation boilerplate mandated by item 1 itself count as a workflow trace under item 3?
- **My default ruling, which I proceed under:** it does **not**. Item 1 requires this section in every file, so reading item 3 to condemn the very content item 1 mandates makes the bar self-contradictory. The line also names no command, no phase, and no external dispatcher — it describes delegation flowing *downward from* the persona, which is the persona's own competence, not its slot in a pipeline. So it passes.
- **Branch if the owner rules the other way:** item 3 fails universally, all 34 files are named as offenders, and the verdict is FAIL with a single systemic fix — reword the shared delegation line to drop the handoff framing — rather than 34 separate fixes.
- Either way I state the interpretation explicitly in the verdict, so the ruling is visible and reversible rather than buried in my reasoning.

## Phase 7 — Aggregate and report

I apply the bar's own aggregation rule: each item passes only if it holds for **every** file; the verdict is PASS only if all three items pass. One offending file on one item sinks the whole grade — no "mostly conforms," no partial credit.

The report follows my standard verdict shape:

```
VALIDATE: 34 persona stubs under agents/
Checklist run:  BAR.md (3 items)
Evidence read:  BAR.md + all 34 files under agents/
Conformance:
  1. Delegation section — PASS/FAIL + strict-vs-loose search result
  2. Model pin        — PASS/FAIL + frontmatter line evidence
  3. Decoupling       — PASS/FAIL + quoted offending lines
VERDICT: PASS | FAIL
Issues requiring fix: <per offending file: the item, the missing/offending text, the concrete fix>
```

Per the bar's instruction to "name each offending file," any failing item enumerates **every** offender, not a sample. For item 3 each offender gets the quoted line and line number so the author can act without re-deriving my finding. I note which evidence class backed each item: search output for 1 and 2, my own reading for 3.

## What I would refuse

- **Editing the stubs.** If any file fails, I hand back the fix list and stop. Rewording a delegation heading or adding a missing `model:` key is the author's job, not mine — I would forfeit my independence over the next grade by touching the file.
- **Passing on the strength of template uniformity.** "The first five were identical, so the rest are fine" is precisely the shortcut that lets a single divergent file through. I read all 34.
- **Grading beyond the bar.** No opinions on model *choice*, body quality, description wording, or the `color:` key — the bar does not ask, so I do not rule.