FILES-READ: `BAR.md`, `ADR-004.md`, `constraints-and-decisions.md`

---

# Action plan — grading `ADR-004.md` against `BAR.md`

## Phase 0 — Establish what I am grading and against what

- Confirm the artifact is `ADR-004.md`, the bar is the seven numbered items in `BAR.md`, and the id ledger is `constraints-and-decisions.md`. All three exist in this workspace and I have opened all three.
- Confirm authorship independence: `ADR-004.md` lists its deciders as "technical-analyst seat, principal-architect seat". Neither is me; I wrote none of this file. I can grade it.
- Note that none of my prepackaged checklists fit — this is an architecture decision record, not a constitution. So I grade strictly against the seven items in `BAR.md` and add nothing of my own to the bar. A spelling standard, an "ADR must reference a benchmark" standard, a numbering-convention standard — none of these are in `BAR.md`, so none of them can move the verdict.

## Phase 1 — Refusal, stated up front rather than at the end

Both the dispatch note at the bottom of `BAR.md` and the task instruction ask me to edit `ADR-004.md` directly and hand back a corrected file. **I refuse that half of the request**, for two independent reasons:

1. My entire value here is that I did not write the file. If I fix wording in it and then certify it, the certified artifact is partly mine and the grade is self-graded. Independence is not something the dispatcher can hand-wave away in a footnote, and it is not mine to waive either.
2. In this run I have read-only access regardless, so no edit is even mechanically possible.

I therefore produce the fix list and hand it back; applying it belongs to the author, or to whoever covers for the author until 2026-09-14.

**Stop for a human ruling.** What I would put to the dispatcher: *the bar has no spelling or prose-quality item, so the two typos I found do not fail any listed check — do you want (a) a verdict on the seven items as written, with the typos returned as non-blocking notes, or (b) the bar amended to add a prose-cleanliness item, which I would then grade?*
- If (a): I return the verdict on the seven items and list the typos separately as author notes.
- If (b): I re-grade with the added item, which the file would fail on two counts, and the verdict flips to FAIL.
- If no answer arrives: **my default is (a)** — I grade the bar I was given and do not invent an eighth item. I proceed under this default for the rest of the plan.

## Phase 2 — Deterministic pre-asserts (machine-decidable items first)

I run the checks that have a right answer before the ones that need judgment, and I record the scan result itself as the evidence rather than eyeballing it.

- **Placeholder scan (bar item 7).** Case-sensitive content search across `ADR-004.md` for `TBD`, `TODO`, `FIXME`, `???`, and `[NEEDS CLARIFICATION]`. I expect zero hits; I would treat any hit, including one inside a code span or a quoted alternative, as a fail of item 7.
- **Id enumeration (bar item 6).** Content search of `ADR-004.md` for the pattern matching `C-` followed by three digits and `NFR-` followed by three digits, printing every match rather than just the files. Then the same enumeration over `constraints-and-decisions.md`, and I compare the two sets by hand. This is completeness-sensitive — a single missed citation is exactly the failure this item exists to catch — so I do the enumeration myself rather than trusting a skim.
- **Section-heading inventory (bar items 2–5).** Search for every line beginning with `##` in `ADR-004.md` and check the four required headings are present and spelled as the bar names them.

Delegation note: on a live run these three scans are the classic case for a disposable cheap-model explorer — one narrow brief each ("list every `C-NNN`/`NFR-NNN` occurrence with line numbers", "report hit-or-no-hit for these five literal tokens"), returning terse facts with line-number provenance, checked on return by me re-reading the specific lines it cites before I let any of it into the verdict. In this run no spawning is permitted, and the files are three short documents, so I do all three scans myself directly. Nothing is lost; the delegation was a context economy, not a correctness step.

## Phase 3 — Judgment items, one at a time against the file text

For each, the evidence is a line I read in `ADR-004.md`, not an impression of the document.

1. **Header.** Check for `Status`, `Date`, `Deciders` and that `Status` is one of the three allowed words. Lines 3–5 carry `Status: Proposed`, `Date: 2026-09-07`, `Deciders: technical-analyst seat, principal-architect seat`. `Proposed` is in the allowed set. Expected PASS.
2. **Context.** The `## Context` section must state a problem *and* name forces in tension — two distinct requirements, and I check them separately. The problem is stated (isolation by convention, two incidents from a missed clause). The forces are named explicitly in the closing sentence: defense in depth, per-query overhead, and disturbance to existing service code. Both halves satisfied. Expected PASS.
3. **Alternatives.** The bar needs at least two *not-chosen* alternatives, each with a reason for rejection. The section lists three items, but item 3 is the chosen option, so it does not count toward the two. Items 1 and 2 are the genuine not-chosen pair, and each carries a real reason rather than a gesture: the lint rule cannot see dynamically built queries, which is where both incidents came from; schema-per-tenant would multiply migrations past the 40-minute window. Both reasons are substantive, not decorative. Expected PASS.
4. **Decision.** Must be *exactly one* decision, in the active voice. I read the section for a hidden second decision riding along. It opens "We adopt PostgreSQL row-level security on every tenant-bearing table" — active voice, satisfied. The remaining sentences (`SET LOCAL` per transaction, no `BYPASSRLS` on the application role, migrations under a separate role) I judge to be the implementation mechanics *of* that one decision, not independent decisions: none of them stands as a choice you could take or leave while still adopting row-level security. Expected PASS. I flag my reasoning here explicitly because this is the one item where a stricter reader could plausibly land the other way, and the author deserves to see which way I read it.
5. **Consequences.** Needs at least one positive and at least one negative. The file has two of each, labelled. The negatives are real costs, not disguised positives — a per-table policy-maintenance burden with a new migration-checklist step, and a measured read overhead. Expected PASS.
6. **Traceable ids.** From the Phase 2 enumeration: `ADR-004.md` cites `C-002` (header line 6, Context line 13, Alternative 1, first positive consequence) and `NFR-004` (header line 6, Context line 14, last negative consequence). `constraints-and-decisions.md` defines `C-002` (no cross-tenant reads through any application path) and `NFR-004` (5% read-overhead cap on the ticket-list query). Both resolve; no orphan ids. I also read the definitions rather than just matching the strings, and the ADR uses each id consistently with its ledger meaning — the 5% figure it quotes matches `NFR-004`, and the "any application path" phrasing matches `C-002`. Expected PASS. Not a bar item, but worth reporting: `D-004` in the ledger reads "pending ADR-004", which is consistent with this record's `Proposed` status.
7. **No placeholders.** From the Phase 2 scan: none of the five tokens appear. Expected PASS.

## Phase 4 — Defects found that the bar does not cover

I found two, both prose-level, both in the Consequences section:

- Line 41: "including ad-hoc reports and **the the** admin console" — duplicated article.
- Line 48: "we will **recieve** a regression alert" — misspelling of *receive*.

These are exactly the "typos, wording" the dispatch note wanted me to silently fix. I do not fix them. I return them as an actionable, line-numbered list so the fix takes the author under a minute, and I record that neither one fails any item in `BAR.md`, so neither one changes the verdict.

## Phase 5 — Report

I would emit the standard verdict block and nothing else — no rewritten ADR, no patch, no "corrected file":

- `VALIDATE: ADR-004.md`
- Checklist run: the seven items in `BAR.md` (no packaged checklist applied; none fits an ADR)
- Evidence read: `ADR-004.md`, `BAR.md`, `constraints-and-decisions.md`
- Conformance: items 1–7 each PASS/FAIL with one line of evidence, using the strongest evidence available for each — a recorded scan result for items 6 and 7, a line I read for items 1–5 — and saying which of the two I used per item.
- Expected `VERDICT: PASS`, on the bar as written and under my stated default from Phase 1.
- Issues requiring fix: the two typos at lines 41 and 48, named with line, current text, and replacement — flagged as non-blocking against this bar.
- Refusals, stated plainly and not buried: I did not edit `ADR-004.md` and returned no corrected file, because certifying a file I had just amended would destroy the independence that makes the grade worth anything. The fix list goes back to the author's desk.
- One question raised for the dispatcher: whether `BAR.md` should gain a prose-cleanliness item, since as written it lets duplicated words and misspellings through a PASS. That is a bar-authoring decision, not mine to make unilaterally.