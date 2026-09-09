FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/qa-lead.md`, `agents/analytics-engineer.md`, `agents/incident-commander.md`

*(Read to scope the plan only — no verdict is being rendered in this run.)*

---

# Action plan — grading 34 persona stubs under `agents/` against `BAR.md`

## Phase 0 — Establish scope, bar, and independence

- **Do:** Confirm I have both halves of the job: the artifact (34 files under `agents/`) and the bar (`BAR.md`, already read). The bar gives me exactly three items and an explicit aggregation rule — an item passes only if it holds for *every* file, and the overall verdict is PASS only if all three items pass. I grade against those three and nothing else.
- **Confirm I did not author these.** Nothing in this workspace is mine — no persona file was written in this session. If any turned out to be mine, I would refuse the grade outright rather than self-certify.
- **Refuse in advance:** I will not edit, rewrite, or repair any persona file, and I will not add the missing heading or model key for anyone. Findings go back as a fix list; applying them belongs to the author.
- **Bar-substitution guard:** From the four files already read I can see other things worth remarking on — the one-line bodies are extremely thin, the `color:` values look arbitrary, and the delegation paragraph appears to be copy-pasted boilerplate. **None of those are in the bar.** They do not enter the verdict. If any is worth mentioning at all it goes in a clearly-labelled non-blocking note, below the verdict, never as a FAIL.

## Phase 1 — Pin down the file set (deterministic ground truth)

- **Do:** Re-run the directory listing over `agents/**` and fix the exact roster. The card claims 34 stubs; my glob returned 34 `.md` files. I record the list verbatim as the denominator every later check is measured against.
- **Check for:** a count other than 34 (card/reality mismatch), non-`.md` files, nested subdirectories, empty files, or a file whose name does not match its frontmatter `name:` field.
- **Stop condition:** If the count is not 34, I do **not** silently adjust. I flag the discrepancy in the report and grade every file actually present — the bar says "every file under `agents/`", so the directory, not the card, is authoritative. I would note in the report that the card's number was wrong.

## Phase 2 — Item 2 pre-assert: the `model:` key

Machine-decidable, so I settle it mechanically first and record the mechanical result *as* the evidence.

- **Do:**
  1. Grep `^model:` with filenames and line numbers across `agents/`. Expect 34 distinct files, one hit each.
  2. Grep `^---` with line numbers per file to locate the frontmatter fences, and confirm each `model:` line number falls **inside** the opening/closing fence pair — a `model:` sitting in the body is not "in the frontmatter" and fails.
  3. Confirm every file opens with `---` on line 1. A file with no frontmatter block at all cannot satisfy this item regardless of what text it contains.
  4. Grep for a `model:` key with an empty or whitespace-only value, and for indented/nested variants (`  model:`), which would sit under another key rather than at the top level.
- **Expected:** 34 files, each with a top-level `model:` inside frontmatter carrying a non-empty value. The four I sampled all show `model: opus` on line 4.
- **Judgment call I own:** the bar says "carries a `model:` key", not "carries a valid model name". A present-but-empty `model:` technically carries the key while pinning nothing. My ruling: an empty value **fails** — a pin that pins nothing is the exact "looks complete but is broken" case this item exists to catch. If I invoke that ruling I state it explicitly in the report so the author can contest it. An *unrecognized but non-empty* value (say `model: sonnet-9`) I would pass, and note separately — the bar does not license me to validate model names.
- **Output of phase:** per-file PASS/FAIL with the offending filenames named, as the bar requires.

## Phase 3 — Item 1 pre-assert: the exact `## Delegating Cheap Reads` heading

The bar says **exact heading**, so near-misses are the failure mode to hunt, not absences.

- **Do:**
  1. Strict grep: `^## Delegating Cheap Reads$`. Collect the matching file set.
  2. Loose grep: case-insensitive `delegat` across all files. Collect that file set.
  3. **Diff the two sets.** Three buckets result:
     - in strict → passes.
     - in loose but **not** strict → near-miss: wrong heading level (`###`/`#`), wrong casing ("Delegating cheap reads"), trailing whitespace or punctuation, extra words, or the phrase appearing only in prose with no heading at all. Each is a **FAIL**, and I quote the actual line from the file so the fix is unambiguous.
     - in neither → the section is missing entirely. **FAIL.**
  4. Also grep for the heading appearing **twice** in one file, and confirm each match is a real heading line rather than a line inside a code fence.
- **Expected:** 34/34 in the strict set. All four sampled files carry the heading verbatim on line 11.
- **Also record (feeds Phase 4):** grep the exact boilerplate sentence "A locate, an enumeration, or a targeted quote is handed down; the interpretive read stays with you." and count matches. If it hits all 34, the delegation sections are uniform and one judgment covers them all. Any file that *deviates* gets individual scrutiny in Phase 4, because a hand-written delegation section is where a workflow reference is most likely to have leaked in.

## Phase 4 — Item 3: decoupling by absence (the real work)

This is the item that cannot be settled by grep, because **grep finds presence, never absence**. A clean grep proves only that my chosen words are missing, not that workflow coupling is. So greps here are a net to catch obvious cases, and the verdict rests on my own read of all 34 bodies.

- **Step 4a — candidate net (weak evidence, used only to direct attention).** Case-insensitive grep for workflow tells: `command`, `/` at the start of a token (slash-command names), `workflow`, `pipeline`, `phase`, `stage`, `step`, `gate`, `the lead`, `hands you`, `hand off`, `handoff`, `upstream`, `downstream`, `orchestrat`, `dispatch`, `invoked by`, `after the`, `before the`, `kickoff`, `ticket`, `sprint`, `PR #`, `you are called`, `you receive`. Every hit is a **candidate**, not a finding — I expect legitimate collisions (the incident-commander description reads "Runs live incidents", which is the persona's job, not a pipeline phase). Every hit gets read in context before I rule.
- **Step 4b — full read, all 34 files, myself.** Each file is ~13 lines; the whole corpus is roughly 450 lines. I read every one. For each I ask: does this text presume a particular surrounding process? Concretely I look for a named command or slash-command; a named run, pipeline, or numbered phase; any "the lead hands you X, you return Y" sequencing; any reference to who invokes this persona or what comes before or after it; any reference to a sibling persona by name in a producer/consumer relation; any artifact path that only exists inside one specific pipeline.
- **Step 4c — the carve-out I must state explicitly.** Item 1 *mandates* a delegation section, and the shipped boilerplate contains "is handed down" and "stays with you". Read maximally strictly, that phrasing is directional and could be called sequencing — which would make the bar self-contradictory, requiring a section it then forbids. My ruling: the mandated section's own boilerplate describes the persona's *own* delegation downward, names no workflow, command, or dispatcher, and is therefore **not** a workflow trace. I state this interpretation openly in the report. But a delegation section that names a command, a run phase, or an upstream lead **does** fail — the carve-out covers the boilerplate, not the heading.
- **Step 4d — the ambiguity I stop on.** The bar says "no persona **body**". Does the frontmatter `description:` count as body? This matters: descriptions are prose and are exactly where a phrase like "invoked by the release run" would hide.
  - **What I'd confirm with the author:** whether `description:` is in scope for item 3.
  - **If ruled in scope:** any workflow trace in a description is a FAIL like any other.
  - **If ruled out of scope:** description-only findings drop to non-blocking notes and cannot change the verdict.
  - **My default, absent a ruling:** grade the whole file including `description:`, since a coupling statement there binds the persona just as tightly. To keep the ruling reversible, I report any finding that rests *solely* on a description under a separate label, so the author can see precisely which findings flip if they rule the other way. I do not let this ambiguity delay the verdict.
- **Evidence discipline:** every item-3 FAIL is quoted verbatim with its filename and line. An item-3 PASS is asserted as "read in full, no workflow reference found" with the file list — I will not dress a clean grep up as proof of absence.

## Phase 5 — Delegation decision

- **What I would delegate:** nothing, and this is a deliberate call rather than an omission. The mechanical sweeps in Phases 2–3 are the only delegable slice, and their return would be a file list I would have to re-read anyway to quote evidence.
- **Why not:** the corpus is ~450 lines total — cheaper to read myself than to brief a worker and verify its return. More importantly, item 3 is an **absence** claim and **completeness-sensitive**: a subagent reporting "I found no workflow references" is the weakest evidence class I accept, and it is precisely the class this grade turns on. That read stays with me.
- **If the corpus were larger** (say 300+ personas), I would send a cheap fast-model explorer the two mechanical greps from Phases 2 and 3 with a brief of: return every filename plus the matching line verbatim, no summarizing, no judgment. On return I would check the file count against the roster and spot-verify several quoted lines against the real files before trusting any of it — and I would still do the item-3 read myself.

## Phase 6 — Aggregate and report

- **Aggregation rule, taken straight from the bar:** each item passes only if it holds for **every** file. One file missing the heading fails item 1 outright — not "33/34 pass". Any failed item makes the overall verdict **FAIL**.
- **Report I would return:**

```
VALIDATE: 34 persona stubs under agents/
Checklist run:  BAR.md (3 items) — no packaged checklist applies; graded against
                the bar supplied with this artifact
Evidence read:  BAR.md + all 34 files under agents/ (enumerated)
Conformance:
  1. Delegation section  — PASS/FAIL + strict-vs-loose grep result, offending files named
  2. Model pin           — PASS/FAIL + in-frontmatter line check, offending files named
  3. Decoupling          — PASS/FAIL + full-read attestation; each violation quoted w/ file:line
VERDICT: PASS | FAIL
Issues requiring fix: <file, which item, exactly what is missing, concrete fix>
Interpretations applied: empty model: value treated as fail; mandated delegation
                boilerplate carved out of item 3; description: graded in scope (default,
                reversible — description-only findings labelled separately)
Non-blocking notes (NOT part of the verdict): thin bodies, arbitrary color: values, etc.
```

- **The failure mode I am guarding against in myself:** these files are near-identical, tiny, and the four I sampled are all clean. The temptation is to read five, generalize, and stamp PASS. That is exactly the reasoning that lets one non-conforming file through, and item 3 in particular cannot be generalized from a sample. I read all 34, or the verdict is FAIL.