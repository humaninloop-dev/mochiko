FILES-READ: `README.md`, `specs/README.md`, `specs/expense-submission/spec.md`, `docs/roles.md`, `docs/glossary.md`, `docs/data-notes.md`, `docs/feature-map.md`, `requests/2026-09-03-reporting.md`, `support/tickets-2026-q3-reporting.md`

# Action plan — write `specs/reporting/spec.md`

## Framing (drives everything below)

The request asks for "a dashboard with charts, exportable to Excel." The fourteen support tickets say the actual job is narrower and harder: nine of fourteen are finance admins trying to close the month, and what they need is **totals by cost center and by category that reconcile to the reimbursement run to the penny**, delivered as **CSV** ("my accountant asks for it in CSV; she does not use Excel"). Charts are real but secondary. I'd write the spec around the evidence, keep charts in scope at a lower priority, and say plainly in the spec and in my report that I put totals-and-export at P1 and charts at P2, and why. I would not drop charts or the export — the requested scope stays in the deliverable.

Three questions in the tickets are genuinely undecidable by me and get recorded as Open Questions with owners, not invented answers: the month-basis question, the pending-expenses question, and manager visibility beyond the 30-day window.

## Phase 1 — Read and reconcile the sources (done)

Read the nine files above. What I extracted that shapes the spec:

- `docs/data-notes.md`: `converted_amount` is fixed at the **filing-day** rate, not incurred or approved. Any total in company currency inherits that; a report re-totalled at a different rate would not match the run. This is why "match to the penny" is achievable only if reports sum the same stored `converted_amount` the run uses.
- `docs/data-notes.md`: an expense can be approved in a later month than incurred; runs pick up everything approved-and-unpaid regardless of incurred month. So "month" is ambiguous by construction, and the ticket asking "July incurred, August approved — which month?" is a real modelling fork, not a UI detail.
- `docs/glossary.md`: **no first-class period exists**; runs are dated by creation day. A monthly report needs a period definition that does not exist yet.
- `docs/roles.md`: managers lose sight of a report's expense 30 days after decision, and whether they may see team approved spend over time "was never decided — both times the answer was 'ask finance'." Three manager tickets ask for exactly that. This is a permission change, not a report.
- `docs/data-notes.md` + glossary: archived categories keep their id and resolve to a name — so the "Client entertainment" ticket is satisfiable and becomes a functional requirement.
- No budget data is stored anywhere; the manager ticket "against the budget finance gave me" cannot be built without a new budgets concept.
- `docs/feature-map.md`: reporting is greenfield; the only existing export is the run CSV with no totals.

## Phase 2 — Lock the house format

Re-read `specs/README.md` and use `specs/expense-submission/spec.md` as the shape reference: eight sections in order (Header, Overview, User Stories, Functional Requirements, Success Criteria, Assumptions, Open Questions, Out of Scope); `US-XXX` with P1/P2/P3 and Given/When/Then plus an **Independent test** line; `FR-XXX` each with a `Source:` line naming its story; `SC-XXX` measurable and stated with how it is measured; `A-XXX` for decisions I took where input was silent. Header fields to match the example: Feature, Status (`Draft`), Author (`analyst seat`), Date (9 September 2026).

## Phase 3 — Decide what I decide vs. what I escalate

**Assumptions I would take myself** (each written as `A-XXX` with its reasoning, so a reviewer can overturn it in one line):

- Reporting totals sum the stored `converted_amount` in company currency — the same number the run pays — so reports reconcile exactly; original amount and currency appear on drill-down only. This answers the EUR/GBP ticket and is forced by the filing-day-rate note.
- A reporting "month" is a calendar month in the company's timezone, since no period object exists; this introduces the period concept for reports only, without changing runs.
- Rejected and withdrawn expenses are excluded from all totals everywhere.
- Archived categories appear in totals with their name, marked archived.
- Export format is CSV (UTF-8), matching the existing run export and the accountant ticket; Excel opens it. I flag the deviation from "exportable to Excel" rather than hiding it.
- Every report is scoped by the viewer's existing permissions from `docs/roles.md` — the spec adds no visibility that does not already exist, except where an Open Question says otherwise.

**Open Questions I would not answer alone** (each with a named owner):

- **Month basis.** Incurred-month or approved-month? Finance's books are by month incurred; the run is by approval. Default I write into the spec pending a ruling: the report is **by month incurred**, with an explicit reconciliation view keyed to a chosen run so the "penny" requirement is still testable. Owner: Tom with finance.
- **Pending expenses.** Counted, excluded, or shown separately? Default I write in: excluded from the total, with a separate "pending, not included: £X across N expenses" figure on the same screen, because the ticket's underlying goal is to chase approvals before closing.
- **Manager visibility beyond 30 days.** Owner: Tom/finance, per `docs/roles.md`. Default I write in: **not granted in this spec** — the three manager stories are specced against the existing window, and the quarter-spend story is held pending the ruling.

**Stop point.** I would not pause before writing; I write the spec complete under the stated defaults and hand these three to Tom/Priya at delivery. If asked to hold instead, the branch is: incurred-month → as drafted; approved-month → US/FR for the monthly totals flip to `approved_at`, the reconciliation section collapses into the main report, and SC-001's reconciliation test becomes trivially satisfied; pending-included → the "pending, not included" figure becomes a breakdown line and SC-002's penny-match to the run no longer holds, which I'd note as a consequence. If the manager visibility ruling comes back "yes," a new US at P2 plus a change to `docs/roles.md` is needed and I'd flag that the spec then carries a permissions change, which changes its review path.

## Phase 4 — Draft the spec

Write `specs/reporting/spec.md`. Planned contents:

**Overview** — three paragraphs: finance admins closing a month by hand in a spreadsheet (two to three hours, per three near-identical tickets); managers and employees who currently have no way to see their own history; what changes.

**User Stories** (priority reflects the ticket volume, not the loudest voice):

- `US-001` Finance: monthly totals by cost center and by category, on screen, for a chosen month (P1). Scenarios cover a month with zero expenses (`docs/data-notes.md` says seasonal businesses have them), archived categories, and mixed-currency companies. Independent test: two expenses in different currencies plus one archived-category expense; totals equal the sum of `converted_amount`.
- `US-002` Finance: export those totals as CSV (P1). Scenarios: totals CSV matches the on-screen figures; header row names the month and basis so the accountant knows what they hold. Independent test: export and diff against the screen.
- `US-003` Finance: reconcile a report against a reimbursement run (P1). Independent test: create a run, export both, confirm the report total for the run's expenses equals the run total to the penny.
- `US-004` Finance: see pending-but-unapproved value for the month so approvals can be chased before close (P2).
- `US-005` Manager: team spend for the current month within existing visibility (P2) — explicitly limited by the 30-day window, with the limitation stated in the story so the gap is visible to whoever rules on the Open Question.
- `US-006` Manager: approvals waiting more than seven days (P2) — needs no new permission; the manager already sees pending items.
- `US-007` Employee: my claimed and reimbursed totals for a chosen period, on screen and as CSV (P2) — the self-assessment ticket.
- `US-008` Charts: spend by category and by month on the finance and manager views (P2) — this is Tom's and Priya's ask, kept and specced, sitting on the same numbers as the tables so a chart can never disagree with a total.

**Functional Requirements** — roughly `FR-001`–`FR-014`, each with `Source:`. Covering: totals computed from `converted_amount`; exclusion of rejected/withdrawn; archived categories included and labelled; period selection; empty-month rendering (zero, not an error); CSV encoding, one totals row per group, and a stated basis; permission scoping per role; charts derived from the same query as the table; pending figure separate from the total; reconciliation identity between a report and a run.

**Success Criteria** — `SC-001` report total equals run total to the penny for the same set of expenses, checked on real company data before launch; `SC-002` finance month-end hand-totalling time drops from two-to-three hours to under fifteen minutes, measured by asking the three ticket-filing companies; `SC-003` reporting-tagged support tickets fall to zero in the quarter after launch; `SC-004` a share of finance admins who export totals at least once per month.

**Assumptions / Open Questions / Out of Scope** as in Phase 3. Out of Scope with reasons: budget-vs-actual (no budget data exists anywhere in Tally — this is a new concept, not a report); scheduled or emailed reports; accounting-system integration (Xero is mentioned but posting is a separate feature); cross-company benchmarking; custom report builder; any change to the 30-day manager window (pending the ruling); XLSX output (pending the CSV ruling).

## Phase 5 — Self-check before reporting

- Walk `specs/README.md` section by section against the draft; confirm order and prefixes.
- Confirm every `FR` has a `Source:` naming a story that exists, and every story has at least one FR.
- Confirm no requirement asserts a field or behaviour not in `docs/data-notes.md` — specifically that nothing depends on the incurred-day or approved-day exchange rate, and nothing assumes a stored period or budget.
- Confirm the spec grants no visibility beyond `docs/roles.md`.
- Confirm each independent test is runnable by one person with a test company.

No test suite exists here and there is no code to run; the checkable artifacts are the independent tests inside the spec and this checklist.

## Delegation

None. This is one document over nine short files I have already read; splitting it across workers would cost more in briefing and re-checking than it saves, and the hard part is judgment about conflicting inputs, which is not parallelisable. I would also not run a review agent unless you ask for one.

## What I'd report back

That `specs/reporting/spec.md` is written and format-conformant; that I priced the request against the tickets and put finance's month-end totals and CSV export at P1 with charts at P2, since nine of fourteen tickets are finance close and the export ask is explicitly CSV rather than Excel; the three decisions I could not make — month basis, pending treatment, manager visibility past 30 days — with the default I wrote in for each, who owns the answer, and what changes in the spec under each ruling; the two asks I put out of scope with reasons, chiefly budget-vs-actual, which needs a budgets concept Tally does not have; and the wrinkle that because `converted_amount` is frozen at the filing-day rate, "matches the run to the penny" is only achievable by summing that stored value — a report that re-converts will not reconcile, and that constraint is worth flagging to engineering before the sprint.

Nothing here is worth saving to memory — the house format, roles, and data model are all recorded in the repo already. If you rule on the three open questions, that ruling would be worth keeping.