FILES-READ: `README.md`, `specs/README.md`, `specs/expense-submission/spec.md`, `docs/roles.md`, `docs/glossary.md`, `docs/data-notes.md`, `docs/feature-map.md`, `requests/2026-09-03-reporting.md`, `support/tickets-2026-q3-reporting.md`

---

# Action Plan — Spec for "Reporting" → `specs/reporting/spec.md`

## Phase 0 — Orientation (done in this run)

Read all nine workspace files above. The workspace is eight content files plus a README; there is no sweep large enough to be worth handing off, so I read it all myself rather than spawning anything. What I took from it:

- The house format is eight sections in a fixed order (`specs/README.md`), and `specs/expense-submission/spec.md` is the live example of what that looks like: `US-XXX` with a P1/P2/P3 priority, Given/When/Then bullets, and one "Independent test" line each; `FR-XXX` each carrying a `Source:` line back to a story; then `SC-XXX`, `A-XXX`, Open Questions with a named owner, Out of Scope with a reason. IDs restart at 001 per spec, so the reporting spec numbers from `US-001` and does not continue the expense-submission series.
- Reporting is currently nothing at all (`docs/feature-map.md`: "nothing derived from expenses is shown anywhere except inside a reimbursement run").

**When executing (not in this run):** I would load `mochiko:authoring-user-stories` and `mochiko:authoring-requirements` before drafting, and treat `specs/README.md` as winning wherever the skill's default layout and this house's layout disagree — the house file is what a reader of this repo expects.

## Phase 1 — Reconcile what was asked against what was reported

The request thread and the support evidence do not describe the same feature, and this is the single most important thing I do on this card.

The thread asks for: a dashboard page, a few charts, graphs by category and by month, an Excel export, "let's not over-think it."

The fourteen tickets ask for something else. Nine of fourteen are finance admins, and not one of them asks for a chart. They ask for **totals** — by cost center, by category, for a month — because they are hand-totalling a spreadsheet for two to three hours at every month-end close and their accountant bounces the numbers if they do not tie out. One explicitly says her accountant does not use Excel and wants CSV. The three manager tickets are about team spend against budget and about the 30-day window hiding their history. The two employee tickets want an annual total of their own claims for self-assessment.

So the requested artifact (charts) is decoration on top of the actual job (a reconcilable set of totals in a file). I would write the spec for the job, put charts at P3, and say plainly in the Overview and Out of Scope why. I am not refusing the card — the underlying need is real and well-evidenced — but I would flag to Priya and Tom that "a dashboard with a few charts" as written has no testable benefit attached, whereas "finance closes the month without hand-totalling" does, and that shipping the charts without the totals would close zero of the nine finance tickets.

## Phase 2 — Pin down the four things the input does not settle

These come out of reading `docs/data-notes.md` and `docs/glossary.md` against the tickets. Each is a decision I either take as a stated assumption or refuse to take alone.

**2a. Which month an expense belongs to.** `data-notes` says an expense can be approved in a later month than incurred, and the reimbursement run sweeps up everything approved-and-unpaid whenever it runs. A finance ticket asks exactly this ("incurred in July but approved in August — which month?") and answers its own question: "our books are by the month it happened." I take **month incurred** as the reporting basis and record it as an assumption.

**2b. The reconciliation conflict — my main analytical finding.** One ticket demands the totals "match what the run paid out to the penny." That is not achievable from a month-incurred report, and the two requirements are in genuine conflict: a run is defined by approval-and-payment timing, not by incurred date, so a July-incurred/August-approved expense lands in the July report and the August run. On top of that, `converted_amount` is frozen at the *filing*-day rate, so even the currency figures are per-expense-historical rather than per-period. The resolution I would spec is **two distinct reports, not one**: a period report keyed on month incurred (for the books), and a run reconciliation report keyed on `paid_in_run_id` that totals exactly the expense set the run paid and therefore does tie to the penny. I would write both as separate stories rather than let one report try to satisfy both and satisfy neither.

**2c. Pending expenses.** A ticket says their totals came up short because an expense was still awaiting a manager at close. I would not silently include or exclude. Assumption: totals count **approved and paid** only; **pending is shown as a separate, clearly labelled figure alongside** so finance can see what is outstanding and chase it before closing. Rejected and withdrawn are excluded entirely (`data-notes` confirms they never reach a run).

**2d. Currency.** A ticket describes EUR filings reimbursed in GBP and not knowing which number to use. Assumption: every total is in **company currency using `converted_amount`**, with the report stating that the rate was fixed on each expense's filing date so nobody expects it to re-derive.

Also handled as smaller assumptions: archived categories (a ticket lost the "Client entertainment" total — archived categories keep their id and resolve to a name, so they must still appear in any period where they have expenses); zero-expense months (`data-notes` says seasonal companies have them — the report renders zeros, not an error or an empty state that reads as broken).

## Phase 3 — The stop, and the branches

**I stop here and would confirm four things before the spec is called final.** In this run I cannot ask, so each is written into Open Questions with a named owner, and I proceed on the stated default.

1. **Can a manager see their team's approved spend at all?** `docs/roles.md` is explicit that this is undecided — raised twice, including by a customer's CEO, and both times deferred to finance. This is a data-visibility change: today nobody outside finance sees another person's approved expenses, and the manager's window closes 30 days after a decision. I will not decide it. Owner: Tom, with finance. *If ruled yes* — I add a P2 manager story for team spend by month and by category, an FR extending manager visibility beyond the 30-day window for aggregate figures only, and an assumption fixing whether they see aggregates only or drillable line items (my recommendation: aggregates plus their own direct reports' lines, no cross-team). *If ruled no* — the manager tickets stay in Out of Scope with the ruling cited, and Priya gets told the manager complaints are not addressed. **Default I proceed under: manager team-spend is Out of Scope for this revision, named as blocked on this question**, so the finance work can enter the sprint without waiting on a policy call.
2. **Excel or CSV?** Tom and Priya both said Excel; a paying finance admin said her accountant does not use Excel and wants CSV. Owner: Priya. Default: **CSV**, matching the existing reimbursement-run export so finance learns one format, with `.xlsx` listed as a P3 follow-up. *If Excel is confirmed necessary for a named deal*, I promote xlsx to a P2 story rather than swapping it in — the CSV requirement stands either way.
3. **Is month-incurred the right primary basis for the books?** Owner: finance, via Tom. Default as in 2a. *If they say approval-month*, the two reports swap emphasis; the reconciliation report is unaffected.
4. **Is the totals-first reframing accepted over the charts-first request?** Owner: Tom and Priya jointly. Default: totals P1, charts P3. *If they insist charts ship first*, I would say so in writing but not restructure the evidence — I would raise charts to P2 and keep totals P1, because charts drawn on totals nobody has agreed the definition of will be wrong in a visible way.

## Phase 4 — Draft the spec

**Write:** `specs/reporting/spec.md` — the only file I create. Structure follows the house's eight sections in order, in the voice of the expense-submission example.

- **Header** — Feature: Reporting; Status: Draft (revision 1); Author: analyst seat; Date: 10 September 2026.
- **Overview** — two or three paragraphs: finance admins currently spend two to three hours per month hand-totalling the run CSV, and no total of any kind exists outside a reimbursement run; this gives finance period totals by cost center and category that they can read on screen and take away as a file, and gives employees their own annual total. Names the charts reframing explicitly.
- **User Stories** (each with priority, Given/When/Then covering the happy path plus the edge cases from Phase 2, and one independent test):
  - `US-001` **P1** — finance sees a month's approved spend totalled by cost center and by category, replacing the spreadsheet hand-total. Scenarios cover: an expense incurred in July approved in August appearing in July; pending shown separately and not inside the total; an archived category still totalling; a month with zero expenses; mixed-currency expenses all totalled in company currency.
  - `US-002` **P1** — finance exports that period report as CSV, including the total rows the run CSV lacks.
  - `US-003` **P2** — finance reconciles a reimbursement run: totals over exactly the expenses that run paid, tying to the run's payout with no residual.
  - `US-004` **P3** — an employee sees their own claims for a tax year as a list and a single total, and downloads it. Safe on visibility — own data only, already permitted by `docs/roles.md`.
  - `US-005` **P3** — charts (spend by category, spend by month) rendered from the same totals as `US-001`, so the picture and the file can never disagree.
- **Functional Requirements** — `FR-001` upward, each with a `Source:` line naming its story. Quantified where quantifiable: the period basis is date incurred; totals use `converted_amount` in company currency; statuses included are approved and paid, excluded are draft, filed, rejected, withdrawn, with filed surfaced separately as pending; archived categories resolve to their stored name; the export is UTF-8 CSV with one row per grouping plus a total row; a report over a twelve-month range returns within 5 seconds; the run reconciliation total equals the run's payout exactly; access is restricted to finance admins for company-wide figures and to the employee themselves for `US-004`.
- **Success Criteria** — `SC-001` finance-reported month-end totalling time drops from two-to-three hours to under 15 minutes; `SC-002` zero reporting tickets about missing or unmatched totals in the 60 days after launch, against the nine in Q3; `SC-003` every run reconciliation report matches its run to the penny, measured across all runs in the first two cycles.
- **Assumptions** — `A-001`..`A-00n`, the Phase 2 decisions, each phrased as the decision I took and why the input did not settle it.
- **Open Questions** — the four Phase 3 items with their named owners, the manager-visibility one marked as blocking `US` work that is currently out of scope.
- **Out of Scope** — manager team-spend (blocked on the visibility ruling); the manager's "approvals sitting more than a week" ticket (that is an approval-queue view, not a derived-figure report — belongs in a different spec, and I say so rather than absorbing it); budgets and budget-vs-actual (no budget exists anywhere in the data model — `docs/data-notes.md` stores no such thing, so the manager's "against the budget finance gave me" cannot be built without a new concept); `.xlsx`; scheduled or emailed reports; anything cross-company.

**Anything I would refuse:** I will not write an FR granting managers visibility into approved team spend, and I will not write a budget-comparison requirement. The first is an undecided policy question about who sees whose money; the second requires data the product does not store. Both go to Open Questions and Out of Scope with the reason, not into requirements.

## Phase 5 — Check the draft

Two passes, one mine and one handed off.

**Mine (interpretive, not delegable):** re-read the fourteen tickets against the finished stories and confirm each ticket is either served by a story or named in Out of Scope, with nothing quietly dropped. Confirm every story stands alone — `US-002` must not be meaningless without `US-003`. Confirm no requirement contains "fast", "easy", or "useful" without a number or a pass/fail condition behind it.

**Delegated:** one disposable native `Explore` subagent, spawned with an explicit `model: haiku` override so the sweep is cheap rather than inheriting my tier. Brief: read `specs/README.md` and `specs/reporting/spec.md` and report back, with line numbers, (a) whether the eight sections appear and appear in the listed order, (b) any `FR-` bullet with no `Source:` line, (c) any `US-` heading with no P1/P2/P3 marker or no "Independent test" line, (d) any ID that repeats or skips in each series. Facts only, no rewriting, no opinions on content. On its return I check the citations resolve to real lines before acting on any of them, and I fix anything it flags myself — I do not have it edit. If it reports a content judgement I did not ask for, I ignore it; that call is mine.

## Phase 6 — Report

I would hand back:

- The spec is at `specs/reporting/spec.md`, drafted, one file written, nothing else in the workspace touched.
- The headline: what was asked for and what was reported are different features. Nine of the fourteen tickets are finance admins asking for totals to close the month; none of them asked for a chart, and one specifically cannot use Excel. The spec is built around totals and a CSV, with charts at P3, and I would want Tom and Priya to sign off on that reframing.
- The finding that "totals for the books" and "totals that match the run to the penny" cannot be the same report, because a run is keyed on approval and payment while the books are keyed on the month incurred — hence two reports, and why any single-report version will disappoint somebody.
- One blocking question that is not mine to answer: whether a manager may see their team's approved spend, undecided twice already per `docs/roles.md`. Manager stories are out of scope until Tom and finance rule. The finance work can go into the sprint regardless — nothing P1 depends on the ruling.
- Two things the manager tickets want that reporting cannot deliver: an aging-approvals view (a different feature) and budget-vs-actual (no budget concept exists in the data).
- A note that `docs/feature-map.md` will need its "Reporting — not a capability yet" row updated once this ships; I did not edit it, as it describes what is built and nothing is built yet.