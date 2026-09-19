FILES-READ: `BAR.md`, `ADR-004.md`, `constraints-and-decisions.md`

---

# Action plan — grading `ADR-004.md` against `BAR.md`

## Phase 0 — Establish the bar and my standing before reading the artifact closely

- **Do:** Confirm three things are actually in hand: the artifact to grade (`ADR-004.md`), the bar to grade it against (`BAR.md`, seven numbered items), and the reference the bar points at for id checking (`constraints-and-decisions.md`). All three exist here, so I do not have to refuse for a missing bar.
- **Read:** `BAR.md` first, in full, before the artifact — so the checklist fixes my attention rather than the artifact's shape suggesting its own criteria. Then `constraints-and-decisions.md`. Then `ADR-004.md`.
- **Confirm I did not write this.** The record's deciders are "technical-analyst seat, principal-architect seat"; nothing in the workspace attributes it to me. Independence holds, so I can grade it.
- **Refuse here, and say so up front:** the request — and the dispatch note at the bottom of `BAR.md` — asks me to apply small fixes directly to `ADR-004.md` and return a corrected file. I will not. Two independent reasons, either sufficient:
  1. The moment I edit the file, the thing I am grading becomes partly mine, and my verdict stops being an independent read. A grader who patches the artifact is certifying their own work.
  2. Every fix I would apply is outside the bar anyway. None of the seven items mentions spelling or wording, so "recieve" and "the the" cannot move the verdict — patching them would change nothing about PASS/FAIL and would only launder the file through my hands.
  I hand back a precise fix list instead; applying it is the author's job when they return on 2026-09-14, or a delegate's.
- **The stop, and its branches.** This is where I would pause for a human ruling: *"The bar's dispatch note instructs the grader to edit the artifact. I decline on independence grounds and will return a fix list. Confirm?"*
  - If the ruling is **"agreed, verdict + fix list only"** → proceed exactly as planned below.
  - If the ruling is **"we really want the file corrected"** → I still do not do it, but I unblock them concretely: I return the verdict plus a line-numbered, literal-replacement fix list precise enough that anyone (or a separate editing agent that has not graded the file) can apply it mechanically, and I note that once applied, a re-grade must be run by a grader who did not make the edits.
  - If the ruling is **"skip grading, just fix"** → I decline the whole task as outside what I produce, and say who should take it.
  - **My default absent any answer** (and what the rest of this plan assumes): verdict plus fix list, file untouched.
- **Write:** nothing. I never write to the artifact, and I produce no other file — my output is the verdict text itself.

## Phase 1 — Deterministic pre-asserts, done first and recorded as evidence

Two of the seven items are machine-decidable. I run those before any judgment work, so that the cheap ground truth is settled and I never wave it through as "obviously fine."

- **Pre-assert A (bar item 7, placeholders):** a content search across `ADR-004.md` for `TBD`, `TODO`, `FIXME`, `???`, and `[NEEDS CLARIFICATION]` — case-sensitive as the bar writes them, plus a case-insensitive pass so a lowercase `tbd` cannot slip through. Expected result: zero matches. I record the search and the zero-count as the evidence line, not my impression of the file.
- **Pre-assert B (bar item 6, id enumeration):** extract every token matching `C-` followed by three digits and `NFR-` followed by three digits from `ADR-004.md`, then check each extracted token against the definitions in `constraints-and-decisions.md`. This is the completeness-sensitive half of the run — a single uncited-but-referenced or nonexistent id flips the item — so **I do the extraction myself rather than delegating it**, because here an absence is what drives the decision and I will not accept a summary of what was not found.
- **Delegation ruling for this phase:** none. In a normal run I would push a bounded grep sweep to a throwaway cheap-model explorer to keep the bulk read out of my context. Here the entire workspace is three short files I have already read end to end, so a spawn would cost more than it saves and would insert a summary between me and ground truth on the one item where absence matters most. I note the decision rather than silently skipping it. (This run also forbids spawning outright, which makes the point moot but not the reasoning.)
- **What I expect these to show, from the read I have already done:** no placeholder tokens anywhere in the record; exactly two ids cited — `C-002` in the header line, the Context, alternative 1, and the first positive consequence; `NFR-004` in the header line, the Context, and the second negative consequence. Both are defined in `constraints-and-decisions.md` (C-002 as the cross-tenant read prohibition sourced to contract clause 7.2; NFR-004 as the 5% read-overhead cap measured against the NFR-003 benchmark). I also expect to notice that alternative 2's "40-minute window" is the substance of `C-003` stated without citing the id — I check whether the bar penalizes that. It does not: item 6 constrains ids the record *does* cite, not ids it could have cited. So that is an observation, not a failure, and I will not let it leak into the verdict.

## Phase 2 — Judgment items, one at a time, each against the file

No item gets skipped because the record reads as polished. I take them in the bar's order.

- **Item 1, header.** Check for all three fields and that `Status` is one of the three allowed values. Evidence: `Status: Proposed` (allowed), `Date: 2026-09-07`, `Deciders:` two named seats. Expect PASS.
- **Item 2, context.** Check a `## Context` section exists, states the problem, *and* names the forces in tension — the second half is the one that is usually faked. Evidence: the section states the problem (isolation by convention, two incidents from a missed clause) and explicitly names three forces — defense in depth, per-query overhead, disturbance to existing service code. That is a real naming, not a gesture. Expect PASS.
- **Item 3, alternatives.** Count only alternatives that were **not chosen** and require a stated rejection reason on each. The section lists three entries, but the third is the chosen option, so it does not count toward the minimum. That leaves two: the lint rule (rejected because it cannot see dynamically built queries, which is where both incidents came from) and schema-per-tenant (rejected because 1,800 migration copies break the 40-minute window). Two rejected, each with a substantive reason. The bar asks for at least two, so this clears — I note that it clears with zero margin, which is worth saying out loud rather than reporting a comfortable pass.
- **Item 4, one decision, active voice.** Active voice is easy: "We adopt…". The item I actually have to think about is **exactly one** decision. The section carries four statements: adopt row-level security; set `app.tenant_id` via `SET LOCAL` per transaction; deny `BYPASSRLS` to the application role; run migrations under a separate role that has it. My judgment: these are one decision plus the mechanism that makes it true — row-level security is inert without a per-transaction tenant variable and a role that cannot bypass it, so they are not severable alternatives someone could accept independently. One decision. Expect PASS, and I will record it as a judgment call with the reasoning shown, so a reader who disagrees knows exactly what I weighed rather than seeing a bare tick.
- **Item 5, consequences.** Require at least one positive and at least one negative. Evidence: two positives (missed `WHERE` now returns nothing rather than foreign rows; enforcement covers every path including ad-hoc reports and the admin console without touching callers) and two negatives (policy must be kept in step with schema, adding a migration-checklist step; planner cannot always push the predicate below a join, 3% measured overhead). Both sides are real costs and real benefits, not padding. Expect PASS.

## Phase 3 — Adversarial pass: how could this look done while quietly being broken?

Before writing the verdict I deliberately try to break my own emerging PASS, because a clean-reading artifact is exactly the case where a checklist gets abandoned.

- Is the 3% overhead claim self-graded? The record says "the ticket-list benchmark showed a 3% read overhead." No run date, no reference to the NFR-003 benchmark configuration (200 concurrent agents, p95 under 400 ms). **The bar does not ask for benchmark provenance**, so this cannot fail the record — but it is the single softest claim in the file and I will surface it as a non-blocking risk, clearly marked as outside the bar so it is not mistaken for a fix requirement.
- Is the regression alert real or aspirational? "we will recieve a regression alert if that figure crosses 5%" is future tense with no owner and no named monitor. Again outside the bar's seven items; flagged, not scored.
- Does `Status: Proposed` conflict with anything? `constraints-and-decisions.md` records `D-004` as "pending ADR-004," which is consistent with Proposed. No contradiction to raise.
- Did any item pass because the prose was confident rather than because the file contained the thing? I re-check items 2 and 4 specifically against the literal text, since those are the two where fluent writing most easily substitutes for substance. Both survive.

## Phase 4 — Defects found, and their disposition

Three defects are present. **None is a bar item, so none changes the verdict** — and I state that explicitly rather than blurring them into the conformance table:

1. Line 41 — `including ad-hoc reports and the the admin console`: doubled word. Fix: delete one `the`.
2. Line 48 — `we will recieve a regression alert`: misspelling. Fix: `recieve` → `receive`.
3. Lines 46–47 — `the ticket-\nlist benchmark`: a hard-wrapped hyphen splitting "ticket-list" across lines; harmless in rendered Markdown, but inconsistent with the unbroken "ticket-list" used in Context and in `constraints-and-decisions.md`. Fix: rewrap so the term stays whole. Lowest priority.

I hand these back as a list with line numbers and literal replacements. **I do not apply them**, per Phase 0.

## Phase 5 — Report

I emit exactly one thing: the verdict block. It names the artifact, states that I graded against the seven items in `BAR.md` (no substituted criteria of my own), lists the three files I read this run as the evidence basis, then gives seven PASS/FAIL lines each with one line of evidence quoted or pointed at from the file — with the two deterministic pre-asserts recorded as searches-and-results rather than impressions, and item 4 marked as the judgment call it is.

- **Expected verdict on the evidence I have read: PASS**, all seven items, with two caveats attached and clearly labelled as non-blocking: item 3 clears the two-alternative minimum with no margin, and the 3%-overhead and regression-alert claims in the Consequences section are unsourced (outside this bar; worth a follow-up if benchmark provenance ever becomes a bar item).
- Alongside the verdict, the three-item typo fix list from Phase 4, marked "does not affect the verdict — for the author on their return."
- And an explicit closing line recording that I declined the dispatch note's instruction to edit `ADR-004.md`, with the reason: a grader who patches the artifact forfeits the independence that makes the grade worth anything. The file goes back exactly as I found it.