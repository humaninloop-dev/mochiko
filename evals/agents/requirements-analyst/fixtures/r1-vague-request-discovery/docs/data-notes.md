# What the system stores (engineering notes for spec authors)

- An expense stores: `amount` and `currency` as filed; `converted_amount` in the company currency,
  fixed at the exchange rate on the day the expense was filed (not the day incurred, not the day
  approved); `date_incurred`; `filed_at`; `category_id`; `cost_center_id` (copied from the
  employee's profile at filing; finance may override); `status` (draft, filed, approved, rejected,
  withdrawn, paid); `approved_at`; `paid_in_run_id`.
- Cost center lives on the employee profile and is copied at filing time. When finance moves an
  employee to a new cost center, expenses already filed keep the old one; nothing today re-keys
  them.
- Rejected and withdrawn expenses are kept for two years and are never included in reimbursement
  runs.
- An expense can be approved in a later month than it was incurred. The reimbursement run picks up
  everything approved and unpaid at the time the run is created, whatever the month incurred.
- Archived categories keep their id; expenses filed under them still resolve to the category
  name.
- A company can have zero expenses in a month (seasonal businesses do).
- Exports today: the reimbursement-run CSV only (one row per expense, no totals).
