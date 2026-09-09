FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/qa-lead.md` (two representative stubs, to learn the file shape before planning), plus the full `agents/` file listing via glob (34 files, matching the card's count).

---

# Action plan — grading 34 persona stubs under `agents/` against `BAR.md`

## Phase 0 — Confirm the bar and my standing to grade

- **Do:** The dispatch brief names an explicit bar (`BAR.md`), so that bar wins outright — I do not go to the router for a checklist skill, and I do not substitute any personal criteria about what a good persona stub looks like. The three items in `BAR.md` are the entire grade.
- **Read:** `BAR.md` (done). It fixes three things I must respect and will not soften:
  - the heading in item 1 is specified as *exact* — so spelling, casing, and heading depth all count;
  - each item passes only if it holds for **every** file, so one offender fails the whole item;
  - I must **name each offending file**, not just report a count.
- **Independence:** I did not author any of these stubs and did not author `BAR.md`. Nothing here is mine, so I can grade it. If mid-run any file turns out to be something I produced, I stop and hand the grade back rather than self-grade.
- **Refuse:** If asked, at any point, to fix a stub — add a missing heading, insert a `model:` key, strip a workflow sentence — I decline and return the fix list instead. I produce a verdict, not content.

## Phase 1 — Fix the denominator

- **Do:** Establish authoritatively how many files are under `agents/` before grading any of them, so "every file" has a real referent. Glob `agents/**/*` (not just `*.md`) to catch any non-markdown file or subdirectory that the card's "34 persona stubs" phrasing might have quietly excluded.
- **Expect:** 34 markdown files, the set already listed. If the count is not 34, that discrepancy goes in the report before anything else — the card's premise would be wrong, and I grade the files that exist, not the number I was told.
- **Note for later, not as evidence:** the glob returned `analytics-engineer.md` and `qa-lead.md` out of alphabetical order, i.e. most-recently-modified. That is a hint about where edits landed, not proof of anything. I will not let it bias the other 32 — every file gets the same three checks regardless.

## Phase 2 — Deterministic pre-asserts for the two machine-decidable items

Items 1 and 2 are decidable by pattern match, so I run them as ground-truth checks first and record the tool result itself as the evidence, rather than eyeballing them.

- **Item 1 — the delegation heading.** Grep `agents/` for the literal `^## Delegating Cheap Reads$`, in files-with-matches mode. Expect all 34 paths. Then, separately, grep case-insensitively for `delegat` across `agents/` in content mode with line numbers. The second grep is the important one: it catches the near-misses that the first grep silently drops — a `###` instead of `##`, a lowercase `cheap reads`, a reworded `## Delegating Reads`, or a file that discusses delegation in prose with no heading at all. Set-subtract the two results; anything in the second and not the first is a candidate offender.
- **Item 2 — the model pin.** Grep `agents/` for `^model:` in content mode with line numbers. Expect 34 hits. Two failure modes this alone will not catch, which I resolve in Phase 3: (a) a `model:` line that sits in the body *below* the closing `---` and so is not frontmatter at all, and (b) a `model:` key present but empty or a placeholder like `model:` with nothing after it, or `model: TODO`. The line numbers from this grep let me spot (a) — in a file of this shape frontmatter runs lines 1–6, so a `model:` at line 20 is suspect and gets read in full.
- **Expect:** clean 34/34 on both, or a short named candidate list. Either way the raw counts go into the report.
- **Why not a shell script:** only file reading and these search tools are available to me here; the greps are the strongest deterministic evidence I can get, and they are strong enough for items 1 and 2.

## Phase 3 — Read all 34 files myself, in full

- **Do:** Read every one of the 34 stubs end to end. Not a sample, not just the greps' candidate offenders, not just the two recently-modified files. Item 3 is a judgment about the *absence* of something, and absence cannot be established by pattern-matching for phrases I happened to think of — so it is mine to do by reading, and I do not delegate it.
- **On delegation:** I would spawn nothing here, and I want to be explicit that this is a deliberate call rather than an oversight. These files are ~13 lines each; all 34 together are roughly 450 lines, which fits comfortably in my own context. There is no bulk sweep to keep out of my head. And item 3 is exactly the interpretive, completeness-sensitive read that I would keep even if the corpus were large — handing "tell me if anything smells like a workflow" to a cheap reader would be laundering the judgment that constitutes the grade. The greps in Phase 2 already gave me the cheap deterministic facts directly, so a disposable reader would add a hop and a chance of loss for nothing.
- **What each read confirms, per file, recorded as a per-file line:**
  1. the `## Delegating Cheap Reads` heading is present, at depth two, spelled and cased exactly, and actually carries a body rather than sitting empty;
  2. a `model:` key sits **inside** the opening/closing `---` fence and has a non-empty, non-placeholder value;
  3. the body carries no workflow trace.
- **The two samples I have already read** (`accessibility-auditor.md`, `qa-lead.md`) both clear all three on my reading — heading exact at line 11, `model: opus` at line 4 inside the fence, and bodies that describe only how the persona works alone. `qa-lead`'s "the evidence that a release met it" is a domain noun, not a pipeline phase, and stays a PASS. These two are recorded as read and graded; they are not evidence about the other 32.

## Phase 4 — Adjudicate item 3, the hard one

Item 3 is where the real judgment sits, and where I expect the disagreements. My working line:

- **Fails item 3:** a named command or slash-invocation; a named run, pipeline, or phase, or an ordinal position within one ("in step two", "after the planning pass"); any sentence describing who hands work to this persona or who receives its output next ("the lead hands you…", "you return this to the orchestrator", "you are dispatched when…"); any reference to a sibling persona by name as a sequencing partner.
- **Passes item 3:** domain vocabulary that merely happens to be process-flavoured — "release", "audit", "incident", "defect list", "verification plan" — when it names the persona's subject matter rather than a step it occupies; and descriptions of the persona's own output shape, since producing a ranked list is competence, not coupling.
- **Stop point — human confirmation:** for any file that sits on the line between those two — say, a body that says "you go last" or "you work from what the reviewer produced" without naming a command — I stop and put the exact sentence, with file and line number, in front of the requester and ask for the ruling on whether that counts as a workflow trace. What gets confirmed is one thing only: is this sentence sequencing, or is it subject matter.
  - **If ruled a trace:** the file is named as an item-3 offender, item 3 fails, and the overall verdict is FAIL with that sentence in the fix list.
  - **If ruled subject matter:** the file passes item 3, and I record the ruling next to it so the call is auditable rather than silent.
  - **My default if no ruling comes back:** I do not fail a file on a sentence I could not distinguish. I mark it PASS on item 3, and I state plainly in the verdict that it was a borderline call decided by default and which sentence it was — so the softness is visible instead of buried. Unambiguous traces are failed outright and never routed through this default.

## Phase 5 — Assemble the verdict

- **Write:** nothing to disk. I emit the verdict as my response and edit no file under `agents/`.
- **Shape:** target `agents/` (34 persona stubs); checklist run: the three items of `BAR.md` verbatim; evidence read: all 34 paths enumerated, plus the greps and their raw counts. Then item-by-item:
  - Item 1 — PASS/FAIL, with grep count and every offending filename;
  - Item 2 — PASS/FAIL, same;
  - Item 3 — PASS/FAIL, with each offending filename **and the quoted sentence** that convicts it, plus line number.
- **Verdict rule I hold to:** PASS only if all three items hold across all 34 files. If even one file misses the heading, or one lacks a frontmatter `model:`, or one body names a command or a hand-off, the verdict is FAIL. I will not report "34 of 34 on two items, 33 of 34 on the third" as a near-pass or a "mostly conforms" — the bar says an item passes only when it holds for every file, and I grade it that way.
- **Fix list:** per offender, the item number, the exact missing or offending thing, and the concrete change — e.g. "`<file>`: item 1 — heading is `### Delegating Cheap Reads`; change to `##`" or "`<file>`: item 3 — line 9 reads '…'; remove the hand-off clause." Then I stop. Applying any of it is the author's job, not mine.
- **Also reported:** the evidence tier behind each item — greps as deterministic ground truth for items 1 and 2, my own full read of the artifact for item 3 — and any borderline call resolved by default, so the requester knows exactly how firm each part of the grade is.