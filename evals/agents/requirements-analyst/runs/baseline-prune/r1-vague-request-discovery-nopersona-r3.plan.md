FILES-READ: `README.md`, `specs/README.md`, `specs/expense-submission/spec.md`, `docs/roles.md`, `docs/glossary.md`, `docs/data-notes.md`, `docs/feature-map.md`, `requests/2026-09-03-reporting.md`, `support/tickets-2026-q3-reporting.md`

# Action plan — write `specs/reporting/spec.md`

## Phase 1 — Read the inputs (done above)

Everything needed is in this workspace and I have now read all nine files. The two that drive the spec are the request thread (what Sales asked for) and the 14 support tickets (what users actually hit). The three `docs/` files are the constraint set: `roles.md` bounds who may see what, `data-notes.md` bounds what can be computed, `glossary.md` fixes the vocabulary I must use. `specs/README.md` plus the expense-submission spec fix the format.

No further reading needed. No delegation: this is one document in a nine-file workspace, and splitting it across workers would cost more in reconciliation than it saves. I would not spawn any agent or run a cloud review for this.

## Phase 2 — Build the demand table before drafting

I would work through the 14 tickets and tally what each person is actually trying to finish, because the request thread and the tickets do not describe the same feature:

| Who | Job to be done | Tickets |
|---|---|---|
| Finance admin | Month-end totals by cost center and by category that reconcile to what the run paid, as a file, to post to Xero | 6 of 9 |
| Finance admin | Answer three ambiguity questions: which currency, which month, do pending count | 3 of 9 |
| Manager | Team spend this month/quarter vs budget | 2 of 3 |
| Manager | Approvals aging past a week | 1 of 3 |
| Employee | My total and my list for the tax year | 2 |

The finding I would carry into the spec: nine of fourteen tickets are one finance job — a reconciled month-end total — and the load-bearing complaint is "two or three hours every month" of hand-totalling, not "I cannot see a chart." Nobody asked for a chart. One finance admin explicitly says their accountant does not use Excel and wants CSV.

I would flag this to the user in the final report rather than treat it as a reason to narrow the deliverable: Tom and Priya asked for a dashboard with charts and an Excel export, so the spec covers that, but the priorities are set by the ticket evidence — the finance totals report is P1, the charts are P2. I would note that CSV satisfies both asks at once, since CSV opens in Excel and suits the accountant who does not use it; so the export is specified as CSV and no `.xlsx` writer is needed in this sprint.

## Phase 3 — Resolve the four data ambiguities, in writing

These are decisions the input does not make, and `data-notes.md` shows each one has a wrong answer that would produce numbers that do not add up. I would settle each as an assumption (`A-XXX`) rather than leave the engineer to guess:

1. **Which month an expense belongs to.** `data-notes.md` says an expense can be approved in a later month than incurred, and the run picks up whatever is approved-and-unpaid. A finance ticket says "our books are by the month it happened." I would default to **month incurred** as the reporting period, and additionally specify a per-run reconciliation view keyed to the run, so the "must match the run to the penny" requirement is met by a different view rather than by bending the period definition. Both are in scope; the mismatch between them is explained in the spec, not hidden.
2. **Which currency.** Only `converted_amount` (company currency, fixed at the filing-day rate) is summable. Totals use it; the original amount and currency stay visible per line. I would name the filing-day rate explicitly, because it is neither the incurred day nor the approved day and will surprise someone.
3. **Do pending expenses count.** Default: totals include approved and paid only; pending is shown as a separate, clearly labelled figure alongside, which is exactly what the "I need to chase approvals before I close" ticket needs. Rejected and withdrawn are excluded entirely.
4. **Cost center.** Use the `cost_center_id` stored on the expense, not the employee's current one, since `data-notes.md` says moves do not re-key filed expenses.

Plus two smaller ones: archived categories still appear in totals where they have expenses (answers the "Client entertainment" ticket), and a zero-expense month renders as zeros rather than an empty state or an error.

## Phase 4 — The one thing I would stop on

**Manager visibility.** `roles.md` says a manager loses sight of a report's expense 30 days after the decision, and that whether a manager should see their team's approved spend over time "was never decided — both times the answer was 'ask finance'." Two manager tickets ask for exactly that. I cannot grant a manager new sight of other people's approved expenses on my own authority; that is a data-access policy call, not a spec detail.

What I would put to the user (as the thing Tom or finance must rule on): *may a manager see approved-expense totals for their direct reports beyond the current 30-day window, and if so, totals only or line detail?*

- **If allowed:** a manager team-spend story enters this spec at P2, reusing the same period and currency rules, with a matching FR that widens the manager's visibility to aggregate figures for their own reports. I would also note that `roles.md` then needs an edit, which is a separate change I would raise rather than make inside a spec.
- **If refused:** the manager team-spend story moves to Out of Scope with the ruling and its date recorded, and managers get only what is already permitted.
- **My default while unanswered:** the spec ships with the manager approvals-aging story in scope at P2 — it uses only the pending queue a manager can already see, so it needs no new permission and unblocks one of the three manager tickets — and the team-spend story sits in Open Questions, owned by Tom with finance. This keeps the spec sprint-ready instead of blocking the whole document on one ruling.

The budget half of the manager ask goes to Out of Scope regardless: Tally stores no budgets anywhere in `data-notes.md`, so "spend against budget" cannot be built without a budget feature first.

## Phase 5 — Draft `specs/reporting/spec.md`

One file written: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-poi2uvw2/ws/specs/reporting/spec.md`. Eight sections in the order `specs/README.md` fixes, matching the reference spec's voice (plain sentences, MUST/MUST NOT in requirements, an independent test under every story).

Header: Feature: Reporting; Status: Draft (revision 1); Author: analyst seat; Date: 9 September 2026.

Overview: two or three paragraphs — finance admins hand-total the run CSV for two to three hours a month; this gives them period totals by cost center and by category that reconcile to the run, plus a file to send the accountant. Employees get their own year. Charts sit on top of the same figures.

Stories I would write:

- **US-001 (P1)** Finance sees a month's totals by cost center and by category, on screen. Scenarios cover: mixed currencies totalling in company currency; pending shown separately from approved; an archived category still appearing; a month with zero expenses.
- **US-002 (P1)** Finance exports those totals as CSV. Scenarios: totals rows present (the current run CSV has none); file opens in a spreadsheet; figures equal what is on screen.
- **US-003 (P1)** Finance reconciles a report against a reimbursement run to the penny. Scenario: an expense incurred in July and approved in August appears in July's period report and in the August run, and the reconciliation view explains the difference rather than showing two numbers that silently disagree.
- **US-004 (P2)** Finance sees the same month as charts — spend by category and by month. Explicitly reads the same figures as US-001; no separate calculation path.
- **US-005 (P2)** Manager sees approvals pending more than seven days. Uses only existing manager visibility.
- **US-006 (P3)** Employee sees their own expenses for a chosen date range with a total, and exports it. Serves the self-assessment ticket; no new visibility needed.

Functional requirements, each with a `Source:` line, covering: period defined by `date_incurred`; totals from `converted_amount`; approved+paid included, rejected/withdrawn excluded, pending reported separately; cost center taken from the expense; archived categories included; CSV export with per-group totals and a grand total; the export matching the screen exactly; zero-expense periods returning zeros; and the reconciliation view. Each requirement traces to a story, and each story traces to a ticket group.

Success criteria: finance month-end totalling time drops from the reported two-to-three hours to under fifteen minutes; report totals equal run totals to the penny on every reconciliation; the reporting-tagged ticket volume falls; some share of finance admins use the export each month-end.

Assumptions: the six from Phase 3, each stating what the input did not say and what I chose.

Open questions: the manager team-spend visibility ruling (owner: Tom, with finance); whether `date_incurred` is the right period basis for all customers or only the ones who wrote in (owner: finance); whether an `.xlsx` export is genuinely required given CSV opens in Excel (owner: Priya, since it came from a deal conversation).

Out of scope, each with a reason: budgets and spend-against-budget (no budget data exists); scheduled or emailed reports; custom report builder; cross-company benchmarking; `.xlsx` as a distinct format this sprint; and manager team-spend pending the ruling.

I would also flag, without acting on it, that `docs/feature-map.md` lists Reporting as "not a capability yet" and will need updating when this ships — that is the owner's edit, not something I would slip into this task.

## Phase 6 — Check the draft before reporting

No test runner exists here, so verification is a read-back:

- Re-read `specs/README.md` and confirm all eight sections are present, in order, with the right identifier prefixes.
- Re-read my draft against `specs/expense-submission/spec.md` for shape: every story has a priority, Given/When/Then, and an independent test; every FR has a `Source:`.
- Trace every FR up to a story and every story down to a ticket group or to the request thread; anything that traces to neither gets cut.
- Re-check every assumption against `data-notes.md` and `glossary.md` line by line — particularly that I have not written a total over `amount` instead of `converted_amount`, and not used the employee's current cost center.
- Confirm no requirement grants anyone visibility that `roles.md` does not already permit, except where the open question says so.

I would expect this pass to catch wording drift rather than substance; if it turns up a requirement with no ticket behind it, I cut the requirement rather than invent a story for it.

## What I would report at the end

The file written, and the three things the user needs to act on: (1) the request asked for charts and Excel, but nine of fourteen tickets are a single finance reconciliation job, so charts are specified at P2 and the CSV totals export at P1 — say so plainly, since it is a departure from how Tom framed it; (2) one decision is blocking a P2 story and needs Tom and finance — whether managers may see team spend past the 30-day window — with the spec shipping usable either way; (3) the four data ambiguities I settled myself and on what basis, so finance can overturn any of them cheaply before the sprint starts. I would also note `docs/feature-map.md` needs a line change on ship, and that the "spend against budget" ask cannot be built until budgets exist as data.

No memory file is warranted here — the unresolved manager-visibility question is already recorded in `docs/roles.md`, so writing it to memory would duplicate what the workspace states.