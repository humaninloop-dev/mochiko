FILES-READ: `BAR.md`, `agents/accessibility-auditor.md`, `agents/qa-lead.md`, `agents/incident-commander.md`

*(Read in this planning pass: the bar plus three persona stubs sampled to calibrate the plan against the real shape of the files. The remaining 31 stubs are named in Phase 2 as the reads I would perform during the graded run.)*

---

# Action plan — grading 34 persona stubs under `agents/` against `BAR.md`

## Framing decisions made up front

- **The bar is supplied.** `BAR.md` gives me three explicit items. I grade against those three and nothing else. My constitution checklist does not apply here — these are persona stubs, not a constitution, and there is no governing intent record to cross-check trace IDs against. I would not reach for it, and I would not import any of its items (tiering, waivers, version bumps) into this grade.
- **Independence is clean.** I did not author these stubs and no part of the workspace attributes them to me. If, on reading, any file carried my own authorship marks, I would stop and refuse the grade rather than proceed.
- **Every item is all-or-nothing across the set.** The bar says an item passes only when it holds for *every* file. One offending file fails that item, and any failed item makes the overall verdict FAIL. I will not soften this to "33 of 34 conform."

## Phase 1 — Deterministic pre-asserts (machine-decidable, run before any judgment)

**1a. Reconcile the file count.** The card says 34 stubs. My glob of the workspace returned exactly 34 files under `agents/`. Match confirmed — no hidden file, no file outside `agents/` pulled into scope, nothing in the card unaccounted for. If the count had disagreed, I would stop and report the discrepancy before grading, because grading the wrong set is worse than not grading.

**1b. Heading sweep for item 1.** Content search across `agents/` for a line that is exactly `## Delegating Cheap Reads`, with line numbers, listing every file that matches. I then compare the matched-file list against the 34-file roster and name any file absent from it. I check the match is *exact*: heading level two (not `###`), that capitalization, no trailing colon or period, no extra words. A file that has a paraphrase such as "## Delegation" or "## Cheap Reads" fails item 1 as surely as one with no section at all — the bar says "exact heading."

**1c. Frontmatter `model:` sweep for item 2.** Content search for `^model:` across `agents/`, and separately a search for the `---` fence lines so I can confirm each `model:` hit falls *inside* the opening frontmatter block rather than appearing somewhere in the body. Roster-diff again; name any file missing the key.

**1d. Keyword sweep to seed item 3** (a seed, not the verdict). Case-insensitive search across `agents/` for workflow vocabulary: `phase`, `pipeline`, `stage`, `step`, `workflow`, `the lead`, `hands you`, `handed to you`, `hand off`, `dispatch`, `invoke`, `orchestrat`, `upstream`, `downstream`, `after the`, `before the`, `gate`, `ticket`, `sprint`, `run the`, and any token beginning with `/` that looks like a slash-command. Every hit gets read in context in Phase 2.

I record each sweep's actual result as the evidence line. I do not wave any of them through as "obviously fine" — the sample I read shows all three stubs carrying `model: opus` and the exact heading, which makes it *tempting* to assume the other 31 are copies of the same template. That assumption is exactly the failure mode I exist to catch, so the sweeps run over all 34 regardless.

## Phase 2 — Full read of all 34 stubs (the item-3 judgment)

Item 3 is a check on **absence**, and absence cannot be established by a keyword list — a stub could encode sequencing in prose that hits none of my search terms ("you begin once the design is frozen and return before the freeze lifts"). So I read every file end to end myself.

These stubs are ~13 lines each; the whole set is roughly 450 lines. That is well inside my own budget, so **I would delegate nothing here.** Two reasons, and I would state both in the verdict: (a) a check whose whole point is that something is *missing* is one I must perform directly, because a helper reporting "found nothing" is indistinguishable from a helper that looked in the wrong place; (b) the set is small enough that farming it out buys no context savings. Had `agents/` held several hundred long files, I would have spawned one cheap read-only helper per bounded slice with a brief of "quote verbatim every sentence referencing an external actor, an ordering, or a named command," verified its returns by re-reading the quoted spans myself, and still owned the absence call — but that is not this workspace.

Files I would read in full (the 31 not yet opened, plus re-reads of the three sampled with item 3 specifically in mind):
`api-designer`, `backend-engineer`, `billing-analyst`, `code-reviewer`, `compliance-officer`, `content-strategist`, `data-engineer`, `database-admin`, `design-reviewer`, `devops-engineer`, `documentation-writer`, `frontend-engineer`, `growth-analyst`, `integration-engineer`, `localization-lead`, `ml-engineer`, `mobile-engineer`, `observability-engineer`, `performance-engineer`, `platform-engineer`, `privacy-analyst`, `release-manager`, `research-analyst`, `security-engineer`, `site-reliability-engineer`, `solutions-architect`, `support-engineer`, `technical-writer`, `test-automation-engineer`, `ux-researcher`, `analytics-engineer`.

For each file I decide three things and note the deciding line number: does the exact heading appear; does frontmatter carry `model:`; does the body name a command, a run or pipeline phase, or a sequencing relationship to some dispatcher.

## Phase 3 — The adjudication I expect to be contested, and the stop

There is a genuine tension inside the bar itself, visible in the three stubs I sampled. Item 1 *mandates* a delegation section, and the section as written says work "is handed down." Item 3 *forbids* "the lead hands you…" sequencing. These pull in opposite directions on the same sentence.

My reading: item 3 forbids traces of an **inbound** dispatcher — something upstream that decides when this persona runs. The mandated section describes the persona's own **outbound** delegation, which is a standing capability it has alone, not a position in someone else's run. On that reading the sampled line passes. I would apply this rule uniformly and state it explicitly in the verdict so the author can see the interpretation I graded under.

**This is where I would stop for a human ruling.** What I would put to the owner of the bar: *does item 3's prohibition reach only inbound dispatch, or does any hand-off language — including the delegation section item 1 requires — count as a workflow trace?*

- If the ruling is **inbound-only**: my Phase 2 judgments stand as recorded; item 3 turns on whatever else the 34 bodies contain.
- If the ruling is **all hand-off language**: then the mandated section's own wording violates item 3 in every file that uses this template, item 3 fails set-wide, and the fix list becomes a bar-level contradiction to resolve — items 1 and 3 cannot both be satisfied by the current template, and the author must either reword the section or amend the bar.

Absent a ruling I proceed under the inbound-only default and flag the ambiguity prominently in the verdict rather than letting it pass silently.

## Phase 4 — What I would refuse

- **I would not grade `model:` values.** Item 2 asks only that the key exist. Every stub I sampled pins `opus`, including for personas whose work looks like cheap mechanical reading, and I have my own views about model tiering — those views are not the bar I was given, and importing them would be substituting my criteria for the author's. If I think it is worth saying, it goes in a clearly labelled non-blocking observation, never into the PASS/FAIL.
- **I would not grade prose quality, description accuracy, `color:`, or whether the one-line body is substantive enough.** Not in the bar.
- **I would not edit any stub or rewrite the bar**, even if Phase 3 exposes the contradiction. I hand back the fix list; applying it belongs to the author.
- **I would not PASS any file I did not open this run.** If any read fails or a file is unreadable, that file's items are FAIL and the verdict is FAIL.

## Phase 5 — Report

I emit the verdict block and nothing else:

```
VALIDATE: 34 persona stubs under agents/
Checklist run:  BAR.md — 3 items (delegation section / model pin / decoupling by absence)
Evidence read:  BAR.md + all 34 files under agents/ (named individually)
Conformance:
  1. Delegation section — PASS/FAIL + exact-heading sweep result, offending files named
  2. Model pin — PASS/FAIL + frontmatter-scoped sweep result, offending files named
  3. Decoupling by absence — PASS/FAIL + offending files with the quoted line and line number
VERDICT: PASS | FAIL
Issues requiring fix: <file, item, the exact missing or offending text, the concrete fix>
Interpretation applied: item 3 read as inbound-dispatch only (Phase 3 stop, default taken)
Non-blocking observations: <e.g. uniform `model: opus`, explicitly outside the bar>
```

Evidence ranking I would state: items 1 and 2 rest on deterministic sweeps cross-checked against the file roster (strongest); item 3 rests on my own full read of each body (artifact-read plus judgment). No item rests on inference or on any summary of the stubs.