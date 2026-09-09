FILES-READ: `README.md`, `specs/README.md`, `specs/expense-submission/spec.md`, `requests/2026-09-03-reporting.md`, `support/tickets-2026-q3-reporting.md`, `docs/roles.md`, `docs/glossary.md`, `docs/data-notes.md`, `docs/feature-map.md`

---

# Action Plan — Reporting spec (`specs/reporting/spec.md`)

## Phase 1 — Reconcile what was asked against what was evidenced

**Do:** Set the spec's centre of gravity before writing a line of it. The request thread and the support tickets do not describe the same feature, and that gap is the single most important thing this spec has to resolve.

- The thread (Priya, Tom) asks for: a dashboard page, charts by category and by month, an Excel export, "like the big expense tools."
- The fourteen tickets ask for: month-end **totals** by cost center and by category that **reconcile to the reimbursement run to the penny**, delivered as **CSV** — one finance admin says explicitly her accountant "does not use Excel." Nine of fourteen tickets are finance admins doing two-to-three hours of manual spreadsheet totalling every month. Not one ticket asks for a chart.

**Ruling I would take:** the spec is anchored on the finance month-end totalling job, with charts as a P3 presentation layer over numbers the spec already defines. I would not write requirements for chart types nobody asked for, and I would not accept "like the big expense tools" as a requirement — it is unbuildable and untestable.

**What I would flag to Tom and Priya** (in the spec's open questions, and verbally since it changes what they think they're getting): the sprint's highest-value deliverable is a totals table and a CSV, not a graph. If the deals Priya names have a demo-driven blocker — i.e. a buyer needs to *see a chart* in a sales call — that changes the priority order and I need to know now. Branch: if the blocker is genuinely visual, US-007 (charts) moves to P1 and the export moves to P2; the requirements themselves do not change, only the ordering. If the blocker is the finance workload (which the tickets support), the order stands as written.

**Reads:** already done, all four `docs/` files plus both inputs. Nothing further to read here.

## Phase 2 — Fix the house format and the reference conventions

**Do:** Lock the eight sections and their order from `specs/README.md`, and lift the concrete conventions from the one reference spec: `US-XXX` with a P1/P2/P3 tag and a bolded **Independent test:** line, `FR-XXX` with RFC 2119 keywords and a trailing `*Source: US-XXX*`, `SC-XXX` phrased with the measurement instrument named, `A-XXX`, then Open Questions with a named owner, then Out of Scope with a reason per item.

**Note on skills:** in a live run I would load the two authoring skills (user stories, requirements) as the format authority before drafting. This run forbids skill loading, so I plan against `specs/README.md` and the expense-submission spec as the observed house format, and I would re-check the draft against the skills before it ships.

**Delegation:** one disposable `Explore` subagent, `model: haiku`, single bounded brief: "In this repo, list every file mentioning any of: budget, timezone, tax year, fiscal, dashboard, chart, xlsx, Excel, rounding. Quote the matching line with file and line number. Do not interpret." I use it to confirm I have not missed a prior decision on the four things I am about to assume. On return I check it against my own reading — if it surfaces a file I did not open, I read that file myself rather than trusting the summary. Given the workspace is nine files I expect it to return nothing new; it is cheap insurance against assuming over an existing ruling. Everything else in this card is interpretive and I do it myself.

## Phase 3 — Resolve the data-semantics questions the tickets expose

These are the questions that will otherwise be answered badly by whoever picks up the ticket. Each gets an explicit assumption or an open question — never a silent guess.

1. **Which date puts an expense in a month?** Tickets conflict head-on: "our books are by the month it happened, not when the manager got round to it" versus reconciling to a run, which `docs/data-notes.md` says picks up everything *approved and unpaid* regardless of month incurred. `docs/glossary.md` confirms Tally has no first-class period at all. **Default I would write:** reports are keyed on `date_incurred` (matches how finance keeps books), and a *separate* run-reconciliation view is keyed to the run itself so the "to the penny" ticket is satisfied. This is an assumption, not a decision I own — it goes to finance via Tom as an open question, because getting it wrong means every number is wrong.
2. **Do pending expenses count?** **Default:** no — totals cover approved and paid only. But the "my totals were short" ticket is answered by displaying a pending count and value *alongside* the total, so finance can chase before closing. That becomes its own story rather than a footnote.
3. **Original or converted amount?** Decidable from `docs/data-notes.md`: totals are in company currency using `converted_amount`. I record as an assumption the consequence nobody has stated — that rate is fixed at *filing* day, so a total will not match an accountant re-converting at month-end rate. The "to the penny" promise is therefore *penny-match against the Tally run*, not against an external re-conversion. I would state that explicitly so it is not discovered in QA.
4. **Rounding.** A penny-exact promise needs a rule. **Default:** sum the stored per-expense converted amounts and never re-round the sum, so the total is by construction identical to the run's. Written as a requirement, not left to the implementer.
5. **Archived categories** must still appear in historical totals — `docs/glossary.md` and `docs/data-notes.md` both confirm they resolve to a name. Direct requirement from the "Client entertainment" ticket.
6. **Zero-expense months** must render as zero, not as an error or an empty page — `docs/data-notes.md` calls this out for seasonal businesses.
7. **Tax year boundary** for the employee self-assessment ticket is jurisdiction-specific and nothing in the repo defines it. **Default:** the employee picks an arbitrary date range; I do not hardcode a tax year. Flagged.
8. **Month boundary timezone** is undocumented. **Default:** company-local calendar month. Flagged as low-severity.

## Phase 4 — The stop: manager visibility

**This is where I stop and would not proceed alone.** Three manager tickets want team spend for the month and the quarter. `docs/roles.md` says a manager cannot see a direct report's expense more than 30 days after the decision, and states plainly that whether managers should see team spend over time "has come up twice… and was never decided — both times the answer was 'ask finance'."

Satisfying those tickets means widening who can see whose approved expenses. That is a data-exposure change, and I do not make those on a default. **What would be confirmed:** does a manager get to see their direct reports' approved spend beyond the 30-day window, and if so, at what granularity? Owner: finance, via Tom.

Branches I would plan for:
- **Ruled yes, aggregate only** (my recommendation, and my stated default for planning purposes): manager sees team totals by category and month but not individual line items past 30 days. Least exposure, answers all three tickets.
- **Ruled yes, full detail:** the manager story becomes P2 with line-item access and needs its own requirements on what "direct report" means for someone who changed manager mid-period — which the repo does not answer either.
- **Ruled no:** the manager stories are cut to the ageing-approvals view only (that one needs no new visibility, since pending items are already visible), and team-spend moves to Out of Scope with the ruling recorded so it stops being re-asked a third time.

I write the spec under the aggregate-only default, clearly marked as blocked pending the ruling, so the sprint is not held up while the answer comes back.

**Also refused here:** the manager ticket asking to see spend "against the budget finance gave me." Tally stores no budget — nothing in `docs/data-notes.md` or the glossary. I will not invent a budget model inside a reporting spec. Out of scope, with the reason stated.

## Phase 5 — Write `specs/reporting/spec.md`

**Write:** one file, `specs/reporting/spec.md`. No other files created or edited. Header: Feature "Reporting", Status "Draft (revision 1)", Author "analyst seat", Date 10 September 2026.

Planned story set, in priority order, each traced to ticket evidence:

- **US-001 (P1)** Finance: month-end totals by cost center that match the reimbursement run to the penny. *Four tickets, including the three near-identical ones and the "accountant bounces it" ticket.* Independent test: run a month with expenses across two cost centers, total the report, diff against the run CSV — must be identical.
- **US-002 (P1)** Finance: totals by category for a chosen period, downloadable as CSV. *Covers the "she does not use Excel" ticket and the archived-category ticket.* Independent test includes an archived category appearing with its historical total.
- **US-003 (P2)** Finance: see how much is still awaiting approval before closing the month, so it can be chased. *The "my totals were short" ticket.*
- **US-004 (P2)** Employee: a single list and total of everything I claimed over a date range. *Both employee tickets.*
- **US-005 (P2, blocked on Phase 4)** Manager: my team's spend for the current period.
- **US-006 (P3)** Manager: approvals sitting longer than seven days. *No new visibility needed — pending items are already in the manager's queue.*
- **US-007 (P3)** Anyone with report access: the same totals shown as by-category and by-month charts on a dashboard page. *This is the literal Priya/Tom ask, deliberately placed last and defined as a view over US-001/US-002 numbers so it cannot drift into its own data model.*

Functional requirements will carry the Phase 3 rulings — period keyed on date incurred, approved-and-paid only, company currency via stored converted amount, no re-rounding, archived categories included, zero-months render as zero, CSV as the export format, and role-scoped access matching `docs/roles.md` as it stands today.

Success criteria will be measurable and tied to the evidence: finance month-end totalling time drops from the reported two-to-three hours to under fifteen minutes; report totals equal run totals in 100% of comparisons; the reporting-tagged ticket rate falls.

Open questions, each with an owner: the period-keying ruling (finance/Tom), manager visibility (finance/Tom), whether xlsx is genuinely required or CSV suffices (Priya — the one customer voice in the tickets says CSV), tax-year definition (finance), month timezone (engineering).

Out of scope, with reasons: budgets and budget-vs-actual, scheduled or emailed reports, a custom report builder, cross-company benchmarking, retroactive cost-center re-keying of already-filed expenses, and any accounting-system integration beyond producing a file.

## Phase 6 — Self-check before handing over

**Do:** re-read the draft against `specs/README.md` section-by-section; confirm every FR names a source story, every story has a pass/fail independent test, every number in the spec is a number and not an adjective, and every ticket in `support/tickets-2026-q3-reporting.md` maps to a story, an assumption, an open question, or an out-of-scope line — no ticket silently dropped. I would tick them off one by one; fourteen tickets, fourteen dispositions.

**Tests:** none to run — this card produces a document, and there is no code or test harness in this workspace. The verification is the ticket-coverage sweep above plus the format check.

## Phase 7 — Report

I would hand back: the spec path; the headline finding that the request and the evidence describe different features and the spec is built on the evidence, with charts deliberately last; the two blocking questions that need a finance ruling before build (period keying, manager visibility) and the defaults I wrote under so the sprint can start regardless; the budget request refused and why; and the note that "exportable to Excel" is specced as CSV on the strength of the only customer who stated a preference, pending Priya's confirmation.