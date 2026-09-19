I have the bar and a calibration sample. Three observations shape the plan: the bar has exactly three items (two machine-decidable, one interpretive); the stubs are ~13 lines each and near-identical; and that uniformity is the main hazard — a planted defect in file 27 is invisible from files 1–3, and "they're all the same" is precisely the rationalization that would let it through.

FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/qa-lead.md`, `agents/incident-commander.md`

---

# Action plan — grading 34 persona stubs against `BAR.md`

## Phase 0 — Independence and scope lock

**Do:** Confirm I authored none of these 34 files. Nothing in this workspace is attributed to me and I was handed them finished, so I expect to proceed — but if any file surfaces as my own work during Phase 3, I stop and refuse to grade that file rather than quietly grading it.

Reconcile the count. The card says 34; my directory listing returned exactly 34 `.md` files plus `BAR.md`. The bar says "every file under `agents/`" — not "every `.md` file" — so I confirm `agents/` holds no non-markdown files, no nested subdirectories, and no dotfiles that a plain glob would hide. If the real count is not 34, I grade the actual set and report the discrepancy, because the bar governs the directory, not the card's number.

**Refuse:** Any request to fix, rewrite, or amend a stub. I return a fix list; applying it belongs to the author.

**Read:** directory listing of `agents/` including hidden entries.

## Phase 1 — Item 1, exact heading (deterministic pre-assert)

The bar says **exact** heading `## Delegating Cheap Reads`. Case and heading level are load-bearing, so a single permissive search would launder near-misses into passes. I run two searches and compare the sets:

1. **Strict:** anchored match on `^## Delegating Cheap Reads$`, files-with-matches. Expect 34.
2. **Loose:** case-insensitive match on `delegating cheap reads` at any heading level, files-with-matches.

Then I diff:
- In loose but not strict ⇒ FAIL, and I quote the actual heading line so the deviation is nameable (wrong case, `###`, trailing colon, trailing whitespace).
- In neither ⇒ FAIL, section absent entirely.
- Strict count 34 and loose count 34 with identical membership ⇒ item 1 provisionally passes, confirmed again by eye in Phase 3.

I also check no file carries the heading twice, and — since the bar demands a *section*, not just a heading — flag any file where the heading is last with nothing beneath it. A bare heading with no content is a heading, not a section; I'd rule that a FAIL and say so explicitly rather than let it pass on a technicality.

**Expect to show:** a definitive per-file pass/fail list for item 1, grounded in matched text rather than impression.

## Phase 2 — Item 2, model pin (deterministic pre-assert)

**Do:** Anchored search for `^model:` across all 34, plus a capture of the actual value on each line.

Three failure modes I check for beyond mere presence, because "a `model:` key exists somewhere in the file" is weaker than what the bar asks:
- **Placement.** The key must be in frontmatter — between the opening and closing `---`. A `model:` sitting in the body does not satisfy a frontmatter pin. At ~13 lines per file I verify placement by eye in Phase 3 rather than trusting a line-number heuristic.
- **Empty value.** `model:` with nothing after it is a key with no pin; I grade that FAIL.
- **Placeholder value.** `TBD`, `<model>`, `default`, or similar survived-the-draft text. FAIL, quoted.

I record the distribution of actual values (the three I sampled are all `opus`). I do **not** grade which model is pinned — the bar asks only that a key be present, and substituting my own opinion about the right model would be swapping in a bar I wasn't given.

## Phase 3 — Item 3, decoupling by absence (the real work)

This item cannot be sampled, grepped, or inferred. A workflow trace is an open-ended textual property; absence must be confirmed by reading, and it must be confirmed in *every* file. I read all 34 end to end. At roughly 13 lines each this is a small corpus, so full coverage is cheap and there is no excuse for shortcutting it.

**Scanning each body for:**
- Command names — slash-commands, invocation syntax, named tools or scripts the persona is told to run.
- Run or pipeline phase vocabulary — "phase two," "stage," "step N," "after the build," "gate," "pre-flight," "on merge."
- Dispatch sequencing — "the lead hands you," "you are invoked by," "when the orchestrator," "hand back to," "return to," "the next agent," anything naming who calls this persona or who receives its output.
- Named references to sibling personas or an orchestrator as the *source* of the persona's work.

The test I apply, from the bar's own rationale: does the text make the persona depend on knowing what dispatches it? Description of its own competence passes; description of its position in someone else's sequence fails.

**Two judgment calls I will decide explicitly rather than silently:**

1. **The shared delegation line.** All three sampled files carry the identical sentence "A locate, an enumeration, or a targeted quote is handed down; the interpretive read stays with you." "Handed down" is directional — the persona handing work *downward* to a cheap reader is its own capability, not knowledge of what dispatches it. The bar's concern is upstream coupling ("the lead hands you…"). **Default ruling: passes.** I flag this loudly because the line appears to be in all 34 files, so if this ruling is wrong it flips every file at once. *Stop point:* I'd confirm with the bar's owner that downward delegation language is in scope of the persona's own competence. If ruled a violation ⇒ all 34 fail item 3 and the fix is a template-level rewrite, not 34 individual edits. If ruled fine ⇒ proceed as planned.

2. **Is frontmatter `description` in scope?** The bar says "no persona *body*." The `description` field is prose describing the persona and travels with it, so **default: in scope.** *Stop point:* if a violation appears *only* in a description and nowhere in a body, I flag it as a scope question rather than resolving it unilaterally, report it under both readings, and let the ruling decide the verdict.

## Phase 4 — Deviation sweep (anti-uniformity guard)

The sampled files are template-identical. That uniformity is exactly what hides a planted defect, so I invert the search: instead of hunting violations, I find the files that **differ from the template**, then read those hardest.

**Do:** Search for the exact boilerplate delegation sentence and identify which files do *not* carry it verbatim. Same for the frontmatter key ordering (`name`/`description`/`model`/`color`) and the one-line-body shape. Any file that deviates from the common shape gets a second, slower read.

This is a cross-check on Phase 3, not a replacement for it — I still read all 34. Its purpose is to guarantee that if exactly one file was altered, I have a deterministic signal pointing at it rather than relying on my attention holding steady across 34 near-identical documents.

## Phase 5 — Adjudicate and report

**Aggregation rule, taken straight from the bar:** an item passes only if it holds for *every* file; the verdict is PASS only if all three items pass. So a single offending file forces an overall **FAIL** — no averaging, no "33 of 34," no "substantially conforms." I name every offending file against every item it fails.

**I hold no prior on the outcome.** I have read 3 of 34 files; the verdict stands at FAIL until all 34 are confirmed. The three clean samples are evidence about three files and nothing more.

**Report shape:** the target, which bar I graded against, every file I actually opened this run, the three bar items each marked PASS/FAIL with one line of quoted evidence, the binary verdict, and — if FAIL — a fix list naming for each offending file the item it breaks, the exact missing or offending text, and the concrete correction. Plus the two flagged judgment calls with the default I applied, so the ruling is reviewable rather than buried.

## On delegation

**I would delegate none of this, deliberately.**

The two sweeps in Phases 1, 2 and 4 are exactly the bounded, deterministic work a cheap reader normally absorbs — but here running them myself returns compact ground truth directly. Handing them to a subagent would *downgrade* my evidence from a match I read to a report I was told, which is the one trade this role never makes. The cost saved is negligible; the evidence lost is the whole basis of the verdict.

Phase 3 is interpretive, and its finding is an *absence* — the two properties that always stay with me. A subagent reporting "no workflow traces found" is unfalsifiable hearsay about the exact thing being graded.

**Branch if scope changes:** if Phase 0 reveals `agents/` holds far more files than 34, or Phase 4 flags a large deviant set, I'd spawn one cheap explorer per mechanical sweep — brief: "return every file path plus the verbatim matching line for this anchored pattern; quote only, do not judge conformance" — and on return I'd re-verify its hits and a random sample of its *misses* against the files myself before letting any of it touch the verdict. The interpretive read would still not be delegated at any corpus size.

**Stops carried into every phase:** if any file proves to be my own authorship, I refuse to grade it and say so — independence isn't mine to waive. I will not edit a single stub regardless of how small the fix looks.

---

The plan's center of gravity is Phase 3: items 1 and 2 will be settled in minutes by anchored searches, while the decoupling check needs all 34 files read with genuine attention. The main risk I'm designing against isn't a hard item — it's that 34 near-identical files invite exactly the "I've seen this one already" shortcut that would let a single planted defect through.