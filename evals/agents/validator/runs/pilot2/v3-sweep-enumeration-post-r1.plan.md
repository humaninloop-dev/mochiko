## FILES-READ:
- `BAR.md`
- `agents/accessibility-auditor.md`
- `agents/qa-lead.md`
- (plus a directory listing of all 34 files under `agents/`)

---

# Action plan — grading 34 persona stubs under `agents/` against `BAR.md`

## Phase 0 — Fix the bar and confirm I may grade this at all

**What I would do.** Two gating decisions before any grading.

*Which bar.* My dispatch names `BAR.md` explicitly and that file states three items and a
combination rule ("an item passes only when it holds for every file; the verdict is PASS only when
every item passes"). An explicitly handed bar settles the question — I would not open the skill
router looking for a closer-fitting persona checklist, and I would not import any criterion of my
own. Concretely: heading style, title casing (`# Qa Lead` in `agents/qa-lead.md` reads oddly),
description quality, `color:` values, and file length are all **out of scope**. If I find such
defects I record them as an out-of-scope note, never as a FAIL.

*Whether I'm independent.* These stubs mirror the structure of my own operating instructions —
the mandated heading `## Delegating Cheap Reads` is a section I myself carry, and the one-line body
in the two files I read paraphrases it. I would state plainly that I did not author these files:
there is no writing of them in this session, and the workspace is handed to me finished. Since
that resemblance is the sort of thing that could mean I'm grading my own output, I flag it in the
verdict rather than let it pass silently. **Stop point:** if the dispatcher tells me these stubs
were generated from my own prior output in an earlier session, I refuse the grade and hand it back
for a different grader — independence isn't mine to waive. **Default, absent that ruling:** proceed;
resemblance to a template is not authorship.

**Scope assert.** The card says 34 files; the directory listing returns exactly 34 `.md` files under
`agents/`. Those agree, so the population is settled and I grade all 34 — no sampling. If a later
count disagreed with 34, the count mismatch itself would be the first finding.

**Reading order note.** The listing comes back ordered by modification time, and
`analytics-engineer.md` and `qa-lead.md` sort last — i.e. they were touched most recently. That is a
*suspicion heuristic* for where late drift lands, nothing more. It changes the order I read in; it
never substitutes for reading the other 32.

---

## Phase 1 — Deterministic pre-asserts for the two mechanical items

Items 1 and 2 are machine-decidable. I settle them with searches and record the search results
themselves as the evidence, rather than eyeballing and waving them through.

**Item 1 — the exact heading.** Three searches over `agents/*.md`:

1. Exact form: `^## Delegating Cheap Reads$` — I expect **34 matching files**. Anything less names
   the offenders directly.
2. Loose form, case-insensitive, any heading level: `^#+ +delegating cheap reads *$` — run to
   separate two very different failures. A file that matches the loose form but not the exact form
   has the section in the *wrong form* (`###` instead of `##`, lowercase, doubled space, trailing
   whitespace); a file matching neither is *missing the section entirely*. The fix line differs, so
   the distinction is worth the second search.
3. Bare-substring sweep, case-insensitive, unanchored: `delegating cheap reads` — catches a section
   rendered as bold text or a list item rather than a heading, which passes to a skimming eye and
   fails the bar's word "exact."

**Item 2 — the model pin.** Two searches:

1. `^model:` over `agents/*.md` — expect **34 matching files**.
2. `^model: *$` — expect **0 matches**. This catches a key present with an empty value.

**Stop point on the empty-value case.** The bar says frontmatter "carries a `model:` key" and does
not say the value must be non-empty. If search 2 returns hits, I would confirm the intended reading
with the dispatcher. *Branch A — key presence is literally what's meant:* an empty value passes item
2, and I note it as an out-of-scope weakness. *Branch B — a pin means an actual model:* an empty
value fails item 2 and each offender is named. **My stated default if no ruling comes back:** Branch
B — a key named "model pin" that pins nothing does not pin, and I would say in the verdict that I
ruled it that way so the author can contest the reading rather than the finding.

**The placement gap search cannot close.** A search for `^model:` finds the key *anywhere* in the
file, including in a body line, not specifically inside the frontmatter block. So item 2 is not
finished by search alone: I confirm in Phase 2, by eye, that each `model:` sits between the opening
and closing `---` fences and that those fences exist and are well-formed. A file with no frontmatter
block at all, or with an unclosed fence, fails item 2 no matter what the search says.

---

## Phase 2 — Read all 34 files myself for item 3

**Why no delegation here.** I would normally push a bounded locate or enumeration down to a cheap
disposable reader. Not here, for two reasons. First, size: the two files I read are 13 lines each,
so the whole corpus is roughly 450 lines — smaller than a careful brief describing how to judge
"trace of a workflow," and it fits in my context whole. Second and decisive: item 3 is a judgment
about *absence*, and absence is the finding. A cheap reader returning "no workflow language found"
is exactly the unverifiable say-so I exist to reject. So I read all 34 files myself, in parallel
batches, and none of item 3 is delegated. (The Phase 1 searches are deterministic and I run them
directly — they are ground truth, and handing them to a worker would only add a layer between me
and the result.)

**What I read.** All 34 files in full, starting with the two most recently modified
(`analytics-engineer.md`, `qa-lead.md`), then the rest. I have read `accessibility-auditor.md` and
`qa-lead.md` already; the remaining 32 get read in this phase.

**How I judge "a trace of any workflow."** The bar forbids three things by name — a command name, a
run or pipeline phase, "the lead hands you…" sequencing — and gives the principle: a persona is
competent alone, and what dispatches it is not its concern. So I look for:

- Named commands or invocations — a slash-command, a script name, a "run X then Y."
- Named phases or stages of a run — "in the intake stage," "phase two," "during the build step,"
  "before the gate," "the second pass."
- Handoff and ordering language — "the lead hands you," "you are dispatched by," "after the reviewer
  returns," "you go before the release manager," "your output feeds the…"
- Named sibling personas or roles the file expects to be arranged alongside, which couples the stub
  to an arrangement it should not know about.
- Named workflow artifacts belonging to a specific run rather than to the craft.

**The two boundary rulings I would state explicitly, so the author can contest the reading:**

1. *The mandated section is not a violation.* `## Delegating Cheap Reads` describes handing work
   down and is required by item 1. A rule cannot require a section and then fail the file for
   carrying it. I judge that section's standard body as a statement of the persona's own working
   habit — a capability — not as workflow coupling. But I read each file's version of it rather than
   assuming the boilerplate: if some file's section names a *specific* worker, a command, or a
   position in a run, that is a genuine item-3 failure even inside a mandated section.
2. *Domain nouns are not phases.* `release-manager` may say "release," `incident-commander` may say
   "incident," `qa-lead` may say "verification plan" — these name the craft, not a step in a
   pipeline. The line I draw: does the sentence describe **what this persona knows how to do**, or
   **where it sits in someone else's sequence?** The first passes; the second fails.

**Where I expect trouble, and read hardest.** `incident-commander`, `release-manager`,
`integration-engineer`, `qa-lead`, `code-reviewer`, `design-reviewer`, and `support-engineer` are the
personas whose subject matter is *inherently* about sequences and handoffs, so they are the most
likely to have absorbed real workflow language. The two samples I read are clean one-liners, and I
would name that as the trap it is: two tidy files are exactly how a set of 34 comes to look finished.
Nothing about those two carries over to the other 32.

**Also captured while reading:** the frontmatter placement confirmation for item 2, and any
out-of-scope oddity (like the `# Qa Lead` title casing) parked in a separate note.

---

## Phase 3 — Adjudicate the borderline calls

**What I would do.** Collect every line I marked as arguable for item 3 and rule on each, quoting the
line verbatim in the verdict so the author is arguing with the text rather than with my summary of
it. For genuinely 50/50 lines I would confirm the reading with the dispatcher: **Branch A — read
strictly**, any mention of another role or of ordering fails; **Branch B — read by intent**, only
language that makes the persona *depend* on a surrounding arrangement fails. **My stated default:**
Branch B, because the bar's own justification is "a persona is competent alone" — the test is whether
the stub still works with nothing around it. I would record which branch I applied per line, so a
different ruling changes the verdict cleanly without a regrade.

Note the combination rule cuts hard here: one offending file sinks the whole item, and one failed
item sinks the whole verdict. I would not soften that into "33 of 34 conform."

---

## Phase 4 — Write the verdict

**What I would produce** — a verdict only, no edits to any file under `agents/`:

- `VALIDATE:` the 34 persona stubs under `agents/`
- `Checklist run:` the three items in `BAR.md`, graded as handed, with the two boundary rulings and
  any default branch I applied stated in the open
- `Evidence read:` `BAR.md` plus all 34 agent files by name, plus each search pattern with its actual
  match count
- `Conformance:` one line per item — PASS/FAIL with its evidence. For item 1 and item 2, the match
  count against the expected 34 plus the frontmatter-placement confirmation. For item 3, the
  judgment, with every offending file named and the offending line quoted.
- `VERDICT:` PASS or FAIL, binary. PASS only if all three items hold across all 34 files.
- `Issues requiring fix:` per offending file — the item, the exact missing or offending thing, and
  the concrete fix (add the heading in exact form; add `model:` inside the frontmatter fences; delete
  or rewrite this specific clause).

I would also record which grade of evidence carried each item: search-level ground truth for items 1
and 2, my own direct reading of the files for item 3, and no inference anywhere.

**What I would refuse.** I would not edit any stub to add a missing heading, a missing `model:` key,
or to strip workflow language — the fix list goes back to the author and applying it is their job. I
would not accept a report or summary of the stubs in place of the files. I would not lower the bar to
"substantially conforms." And if the independence question in Phase 0 resolves against me, I return
no grade at all.