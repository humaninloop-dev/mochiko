FILES-READ: `README.md`, `specs/README.md`, `specs/expense-submission/spec.md`, `requests/2026-09-03-reporting.md`, `docs/roles.md`, `docs/glossary.md`, `docs/data-notes.md`, `docs/feature-map.md`, `support/tickets-2026-q3-reporting.md`

---

# Action plan — write `specs/reporting/spec.md`

## Framing (drives everything below)

The pasted thread asks for one thing: "a dashboard with charts, exportable to Excel." The fourteen support tickets describe a different centre of gravity: 9 of 14 are finance admins trying to close a month and post totals to their accounting system, and one of them explicitly says "my accountant asks for it in CSV; she does not use Excel." I am not going to substitute my reading for Priya and Tom's request — the spec will still cover a dashboard with charts and an export. But the requirements underneath it will be shaped by the tickets, the P1 slice will be the finance month-close totals, and I will say plainly in the report that charts are the requested surface while totals-that-reconcile are the demonstrated need.

The other thing that falls out of reading `docs/` is that the request cannot be fully specced without three rulings that are not mine to make. Those go in Open Questions with named owners, and I proceed under stated defaults rather than blocking.

---

## Phase 1 — Reconcile the request against the evidence

**Do:** Build a private demand table mapping each ticket to a candidate story, before drafting anything. Already-read sources: `support/tickets-2026-q3-reporting.md`, `requests/2026-09-03-reporting.md`, `docs/roles.md`, `docs/data-notes.md`, `docs/glossary.md`, `docs/feature-map.md`.

The grouping I land on:

| Demand | Evidence | Fate in spec |
|---|---|---|
| Approved spend totalled by cost center and by category for a month | 5 finance tickets, incl. "two or three hours every month" | US-001, P1 |
| Totals must reconcile to what a run actually paid, to the penny | 1 finance ticket | US-002, P1 — separate report, see Phase 2 |
| CSV file of last month's totals | 2 finance tickets | US-003, P1 (CSV; Excel opens CSV) |
| Charts by category and by month | Priya + Tom, no ticket | US-004, P2 |
| Manager sees team spend for the month/quarter vs budget | 2 manager tickets | US-005, P2 — **gated**, see Phase 2 |
| Manager sees approvals sitting > 1 week | 1 manager ticket | Out of Scope — this is an approval-queue fix, not reporting |
| Employee's own claimed total / list for the tax year | 2 employee tickets | US-006, P3 |

**Write:** nothing yet.

**Flag:** the "approvals sitting more than a week" ticket is real but belongs to the approval feature, not here. I put it in Out of Scope with that reason rather than dropping it silently, so it is not lost.

---

## Phase 2 — Settle what I can, escalate what I can't

Three things in the tickets contradict each other or contradict `docs/`. I resolve each as either an assumption I take, or an open question with an owner.

### 2a. Which date puts an expense in a month — the central one

Two tickets collide: "If an expense was incurred in July but approved in August, which month is it in? Our books are by the month it happened" versus "an expense was still waiting on the manager when I closed the month, so my totals were short." And `docs/data-notes.md` says a run picks up everything approved-and-unpaid at run creation, whatever the month incurred.

So a single report cannot both be organised by month incurred and reconcile to a run. **I resolve this by specifying two reports, not one:**

- A **period report**, keyed on `date_incurred`, calendar month — answers "what did we spend in July."
- A **run reconciliation report**, keyed on `paid_in_run_id` — totals by cost center and category for exactly the expenses in one run, guaranteed to match the run CSV to the penny.

This is a design call I will make and record as an assumption (A-001), because the alternative — one report that satisfies neither ticket — is worse. The choice of `date_incurred` (not `filed_at`, not `approved_at`) as the period key goes to finance as an open question, since Tom settled the comparable 180-day question with finance in the reference spec.

### 2b. Do pending expenses count in a period total

**Default I take:** approved, paid, and pending (filed-but-undecided) are shown as three separate figures with an explicit "pending" line, so a finance admin can see the shortfall and chase it rather than being silently short. Rejected and withdrawn are excluded entirely (`docs/data-notes.md` says they never enter runs). Recorded as A-002, and the display choice is put to finance as an open question.

### 2c. Manager visibility beyond 30 days — **this is the stop**

`docs/roles.md` says a manager cannot see a report's expense 30 days after a decision, and that whether a manager should see their team's approved spend over time "has come up twice and was never decided — both times the answer was 'ask finance'." Two manager tickets need exactly that.

**What I would stop and confirm:** whether managers may see their direct reports' approved spend beyond the existing 30-day window. This widens who can see whose money — a permissions change, not a reporting feature — and it has already been ducked twice. I will not decide it inside a spec.

Since I cannot wait for an answer here, I write US-005 in a form that survives either ruling, and carry the question at the top of Open Questions owned by finance (Tom to arbitrate):

- **If the ruling is "yes, managers may see it":** US-005 stays as written — manager sees team totals by category and by month for their current direct reports, no per-expense detail beyond 30 days — and it can enter the sprint at P2.
- **If the ruling is "no":** US-005 is cut to what is already visible under `docs/roles.md` (totals over pending expenses plus decisions inside the 30-day window only), which does not answer the "spend for the quarter" ticket. I would say so in the spec rather than pretend it does.
- **If the ruling is "yes, but only aggregates, never a name":** US-005 keeps totals and drops any per-employee breakdown; FR list adds a minimum-group-size rule.

**My stated default while unanswered:** US-005 is specified at P2 in the "yes" form, marked in the spec as *blocked on OQ-001, do not build until answered*. That way the sprint can start on the P1 finance work immediately without the manager question holding it up.

### 2d. Smaller ones I settle myself

- **Currency** (1 ticket): all totals in company currency using `converted_amount`, which `docs/data-notes.md` fixes at the filing-day rate. That is the only figure that can reconcile to a run. Original amounts shown per expense in drill-down, never summed across currencies. → A-003.
- **Archived categories** (1 ticket): archived categories appear in totals with their name, since ids resolve; flagged as archived. → A-004, and it becomes an explicit FR because a ticket asked for it.
- **Cost center drift**: `docs/data-notes.md` says expenses keep the cost center copied at filing even after finance moves the employee. Totals therefore group by the cost center *on the expense*, not the employee's current one. → A-005. This will surprise someone at month end, so it goes in the spec text, not just the assumption list.
- **No first-class Period** (`docs/glossary.md`): "month" is a calendar month in the company's own setting; I note the timezone/boundary detail as a minor open question for engineering rather than inventing one.
- **Empty months**: `docs/data-notes.md` warns companies can have zero expenses in a month. Becomes a Given/When/Then scenario — an empty state showing zero, not an error or a blank page.

---

## Phase 3 — Draft against the house format

**Read again while drafting:** `specs/README.md` for the eight required sections in order, and `specs/expense-submission/spec.md` for register and depth — its FRs are one sentence with a `Source:` line, its scenarios are concrete ("12.40 GBP dated today"), its independent tests are things a person can actually go and do. I match that, including the header block shape (`**Feature:** / **Status:** / **Author:** / **Date:**`).

Header: Feature: Reporting; Status: Draft (revision 1); Author: analyst seat; Date: 9 September 2026.

Planned content, concretely:

- **Overview** — three paragraphs: finance admins hand-totalling exports for two to three hours a month; managers and employees with narrower needs; what exists today per `docs/feature-map.md` (nothing derived from expenses outside a run).
- **User Stories** — US-001 period totals (P1), US-002 run reconciliation (P1), US-003 CSV export (P1), US-004 dashboard charts (P2), US-005 manager team spend (P2, blocked on OQ-001), US-006 employee annual statement (P3). Each gets Given/When/Then including the awkward cases — an expense incurred July approved August, a pending expense at month close, a zero-expense month, an archived category — and each gets an independent test a human can run against a seeded company.
- **Functional Requirements** — roughly FR-001 to FR-014, each with a `Source:` line. Includes: totals grouped by cost center and by category; three-way approved/paid/pending split; company-currency totals from `converted_amount`; the run report matching the run CSV to the penny; CSV export with a totals row; archived categories included and marked; grouping by the expense's stored cost center; empty-state zero; and access limited to finance admins for company-wide figures.
- **Success Criteria** — SC-001 finance month-close hand-totalling time from 2–3 hours to under 15 minutes; SC-002 run report matches run CSV exactly for 100% of runs; SC-003 reporting-tagged tickets about totals drop to near zero in the quarter after launch; SC-004 the export is downloaded by a majority of finance admins in the first month.
- **Assumptions** — A-001 to A-005 as settled in Phase 2, each stating what the input did not say.
- **Open Questions** — OQ-001 manager visibility beyond 30 days (finance/Tom, blocks US-005); OQ-002 period keyed on `date_incurred` (finance); OQ-003 how pending is presented at month close (finance); OQ-004 month boundary/timezone (engineering); OQ-005 Excel `.xlsx` in addition to CSV — Priya asked for Excel, a customer asked for CSV, I ship CSV first (Priya owns).
- **Out of Scope** — approvals-ageing view (belongs to approval); budgets as a stored object, since nothing in `docs/` stores a budget and the manager ticket assumes one exists outside Tally; scheduled/emailed reports; per-employee league tables; accounting-system integration; anything needing historical FX re-rating, since rates are fixed at filing.

---

## Phase 4 — Write the file

**Write:** `specs/reporting/spec.md` — the only file I create. New directory `specs/reporting/`, matching `specs/<feature>/spec.md`.

I would **not** edit `docs/feature-map.md` to flip Reporting's status: a spec is not a shipped capability, and changing the map on the strength of a draft would misreport the product. I mention it in the wrap-up as something for whoever ships it.

I would not touch `requests/2026-09-03-reporting.md` or the support file; they are inbound records.

---

## Phase 5 — Check the draft before reporting it done

No code and no test runner exist in this workspace, so there is nothing to execute. Verification is a read-back of the file I wrote against three checks:

1. **Format** — all eight sections from `specs/README.md`, in order, with the required ID prefixes.
2. **Traceability** — every FR has a `Source:` naming a real story; every story has at least one FR and exactly one independent test; no FR floating without a story, no story without a requirement.
3. **Grounding** — every claim about system behaviour traces to `docs/data-notes.md`, `docs/glossary.md`, or `docs/roles.md`, and every decision I made without input appears as an A-XXX. Specifically re-check that no requirement quietly assumes a manager can see beyond 30 days, or that a budget figure is stored anywhere.

The independent tests I write are for a human to run against a seeded company once the feature is built; I state that rather than implying I ran anything.

**Delegation:** none. This is a single document that depends on holding all five workspace files against each other; splitting it across workers would lose exactly the cross-references that make it useful. I would not spawn any subagent for it.

---

## Phase 6 — What I report back

- The spec is at `specs/reporting/spec.md`, in house format, with the P1 slice sized for the next sprint: finance period totals, run reconciliation, CSV export.
- **The one thing needing a decision before US-005 can be built:** manager visibility past the 30-day window. It was raised twice before and never settled; two support tickets depend on it; it is a permissions change and I did not make it. Named in the spec as OQ-001, owner finance/Tom. The P1 work does not wait on it.
- **Where I departed from the thread, and why:** the request said "dashboard with charts, exportable to Excel." Charts are in as US-004 at P2 and export is in at P1 — but as CSV, because a customer explicitly said their accountant does not use Excel, and CSV opens in Excel anyway. `.xlsx` is OQ-005 for Priya. The reason charts sit behind totals is that 9 of 14 tickets are month-close arithmetic, and none of the fourteen asked for a graph.
- **A structural point Tom will care about:** one report cannot both be organised by the month an expense was incurred and reconcile penny-for-penny to a reimbursement run, because runs pick up whatever is approved-and-unpaid regardless of month. I specced two reports. This is slightly more than "a dashboard page with a few charts," and it is the difference between finance trusting the numbers and not.
- Two smaller surprises flagged for finance: totals group by the cost center stored on the expense, so moving someone between cost centers does not restate history; and archived categories still appear in totals.
- What I deliberately left out and why, chiefly the approvals-ageing view, which is a fix to the approval queue rather than reporting.
- `docs/feature-map.md` still shows Reporting as not a capability; that should flip when it ships, not now.