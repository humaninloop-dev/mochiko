## FILES-READ:
`BAR.md`, `agents/accessibility-auditor.md`, `agents/qa-lead.md` (plus a full-tree glob of `agents/**/*` and `**/*.md` to fix the census)

---

# Action plan — grading 34 persona stubs against `BAR.md`

## Phase 0 — Fix the bar and refuse to route around it

The card hands me an explicit written bar, so `BAR.md` **is** the checklist. I would not reach for a routed checklist skill or add criteria of my own — no "personas should also have X" items. The three items in `BAR.md` are the entire grade, and the file states its own aggregation rule: an item passes only if it holds for *every* file, and the verdict is PASS only if all three items pass. That means a single offending file sinks its item and the whole verdict.

Independence check: I authored none of these 34 files and will not edit them. If it emerges mid-run that any stub is my own output, I stop and refuse the grade rather than continue.

**Already established:** `agents/**/*` returns exactly 34 entries, all `.md`, no subdirectories and no non-markdown files. The bar says "every file under `agents/`" — not "every `.md`" — so this census matters, and it is settled: scope is 34 files, matching the card's count. No discrepancy to reconcile.

## Phase 1 — Deterministic pre-asserts, recorded as evidence, not as a pass

Three machine-decidable sub-checks run first. I record each result as the evidence, and I am explicit about what each one can and cannot settle.

**1a. Item 1 — the delegation heading.** Grep across `agents/` for lines matching `^## Delegating Cheap Reads$` in content mode with line numbers, and separately grep for the looser `Delegating Cheap Reads` (any context). Then compare:
- Files in the loose set but not the exact set are the interesting ones — they carry the phrase at the wrong heading level (`#`/`###`), with trailing punctuation, with altered capitalization, or as prose rather than a heading. The bar says *exact heading*, so those are FAILs, and the loose-vs-exact diff is precisely how I catch a near-miss that reads fine to a skimming eye.
- Files in neither set are outright missing the section.
- Expected shape if clean: 34 exact matches, one per file, and the two sets identical.

**1b. Item 2 — the model pin.** A grep for `^model:` is *not* sufficient, because the bar requires the key in **frontmatter**, and a `model:` line further down the body would satisfy the grep while failing the bar. So: for each file I confirm the line number of the `^model:` hit falls inside the opening `---`/`---` fence block, which in these stubs is lines 1–6. Any `model:` hit at a line past the closing fence is a FAIL, and any file with no hit at all is a FAIL. I also check the value is non-empty — a bare `model:` with nothing after it is a key with no pin.

**1c. Item 3 — positive-evidence sweep only.** Grep the bodies for workflow tokens: `/` -prefixed command names, and the words `phase`, `pipeline`, `step`, `stage`, `workflow`, `run`, `hands you`, `the lead`, `hands off`, `upstream`, `downstream`, `after the`, `once the`, `then you`.

**The critical limit, stated up front:** a clean grep here proves nothing. Item 3 is a requirement of *absence*, and no finite token list can establish that a body carries no trace of any workflow. The grep can only surface suspects. A quiet grep does **not** license a PASS on item 3 — it just means the violations, if any, are phrased in words I did not guess. That is exactly the failure mode I exist to catch, so item 3 is settled by reading, in Phase 2.

## Phase 2 — Read all 34 bodies myself (no delegation)

I read every one of the 34 files end to end.

**Why I would not delegate this.** The corpus is ~13 lines per file, ~450 lines total — the whole sweep is cheap enough that shipping it out costs more in brief-writing and return-verification than it saves. More decisively, item 3 is interpretive *and* absence-driven *and* completeness-sensitive: a subagent reporting "no workflow language found" is a summary, and a summary is not evidence I can pass on. If the corpus were ten times larger I would spawn a cheap `Explore` at haiku tier for the mechanical half only — brief: "for each file under `agents/`, return the frontmatter block verbatim and every line beginning with `#`, with file path and line numbers; quote, do not summarize" — and on return I would spot-check its quotes against three files I read myself before trusting the enumeration, and re-verify any file it reported as anomalous. At 34 short files, I do it directly.

While reading, for each file I record: the exact heading line, the frontmatter `model:` line, and a judgment on item 3.

**How I discriminate on item 3.** The bar's test is whether the body reveals what dispatches the persona. Applying that:
- **Violations:** a named command; an ordinal position in a run ("in the review phase", "after the build step"); sequencing that names another role as the persona's feeder or consumer ("the lead hands you the brief", "you return this to the orchestrator", "you pick up where X leaves off"). These make the persona incompetent alone.
- **Not violations:** domain nouns that happen to appear in workflows. `accessibility-auditor`'s "audits shipped screens" and `qa-lead`'s "the evidence that a release met it" use *release* and *shipped* as subject-matter vocabulary, not as pipeline coordinates — a QA lead owning release evidence is a description of the craft, not of a dispatcher. I would rule these PASS.
- **Also not a violation:** the shared delegation line "a locate, an enumeration, or a targeted quote is handed down." This describes work the persona hands *downward* on its own initiative — the persona as delegator, not as recipient of a lead's handoff. The direction is what matters, and I would state that reasoning in the conformance line rather than let it pass silently, since "handed down" is exactly the phrase a token grep would flag.

The two files read so far (`accessibility-auditor`, `qa-lead`) carry the exact heading, a frontmatter `model: opus`, and clean bodies. I draw no conclusion about the other 32 from that — uniform-looking stub sets are precisely where one odd file hides, and item 3 in particular is the kind of check that a generated batch fails in exactly one or two places.

## Phase 3 — The stop, and its branches

**Where I would stop for a human ruling:** a borderline item-3 phrase — language that names a workflow-adjacent concept without clearly encoding sequencing. What I would put to the author: "Does this phrase tell the persona who dispatches it, or is it domain vocabulary?"
- If ruled **workflow trace** → item 3 FAILs, that file is named in the fix list with the offending sentence quoted and the fix being deletion or rewording into craft terms.
- If ruled **domain vocabulary** → the file passes item 3, and I note the adjudication in the conformance line so the call is visible rather than buried.

**My default if no ruling is available:** I apply the direction test above — sequencing or a named dispatcher is a FAIL, subject-matter nouns are a PASS — and I record the borderline file, the phrase, and which way I ruled, so the author can overturn it with full information.

I would also stop and flag, without fixing, any structural surprise: a file with no frontmatter fence at all, a duplicate `model:` key, or a heading present twice.

**What I refuse regardless of ruling:** editing any stub, adding a missing heading or `model:` line, or drafting replacement wording. I hand back the fix list; applying it is the author's job.

## Phase 4 — Assemble the verdict

I emit exactly the verdict block and nothing else:

```
VALIDATE: 34 persona stubs under agents/
Checklist run:  BAR.md (3 items, all-files aggregation)
Evidence read:  BAR.md + all 34 files under agents/ (enumerated by path)
Conformance:    item 1 / item 2 / item 3 — PASS or FAIL each, one line of evidence
VERDICT: PASS | FAIL
Issues requiring fix: ...
```

`Evidence read:` names all 34 paths — any file I did not open cannot be counted toward an item that requires *every* file, and if for any reason a file were unreadable, the verdict is FAIL on that ground alone. Each conformance line says which grade of evidence carried it: greps for items 1 and 2 (deterministic, with the frontmatter-boundary confirmation for 1b), my own reading for item 3 (the grep there is a suspect-finder only).

Failures are written to be actionable: file path, which of the three items, the exact defect (`heading is "### Delegating Cheap Reads" at line 11 — wrong level, bar requires "##"`), and the concrete fix. Then I stop.

**One thing I will not do:** return PASS because the set looks tidy and uniform. Thirty-four stubs cut from one template is the strongest available invitation to skim, and skimming is how the one nonconforming file survives sign-off. The verdict stays FAIL until all three items are confirmed against all 34 real files.