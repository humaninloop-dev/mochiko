# Glossary

- **Expense** — one line an employee files: amount, currency, date incurred, category, receipt
  image, note. The cost center is copied on at filing (see below).
- **Category** — a company-defined label (Travel, Meals, Software, and so on). Categories can be
  archived; archived categories keep their historical expenses and cannot be chosen for new ones.
- **Cost center** — the budget bucket an expense is charged to. It is set on the employee's
  profile by finance and copied onto each expense at filing time; finance can override it on an
  individual expense before a reimbursement run.
- **Approval** — a manager's decision on an expense: approved or rejected, with a timestamp and an
  optional reason.
- **Reimbursement run** — a monthly batch finance creates from every approved-but-unpaid expense;
  it produces a CSV that finance uploads to payroll or their accounting system.
- **Period** — Tally has no first-class period today; reimbursement runs are dated by the day
  finance creates them.
- **Company currency** — the single currency a company reimburses in; every expense stores its
  original amount and a converted amount (see `docs/data-notes.md`).
