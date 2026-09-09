# Roles and visibility (as built, September 2026)

| Role | Can do | Sees |
|---|---|---|
| Employee | File, edit, and withdraw their own expenses; attach receipts | Only their own expenses and their approval status |
| Manager | Approve or reject expenses filed by their direct reports | Their own expenses; their direct reports' expenses while pending approval and for 30 days after a decision |
| Finance admin | Create reimbursement runs; manage categories, cost centers, and company settings; export a run to CSV | Every approved expense in the company, through the reimbursement-run screens |

Notes

- A manager cannot see a report's expense once 30 days have passed since the approval decision;
  that window was chosen for the approval flow and nothing else has needed longer.
- Nobody outside finance can see another person's approved expenses. Whether a manager should
  see their team's approved spend over time has come up twice (once from a customer's CEO, once
  from our own) and was never decided — both times the answer was "ask finance".
- Company owners are finance admins in the data; there is no separate owner role.
