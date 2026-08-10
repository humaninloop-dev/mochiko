### User Story 6 - Track invoice status at a glance (Priority: P2)

A contractor sees every invoice and its current state — draft, sent, paid, or overdue — in one
list they can scan and filter, so they know who still owes them without opening each invoice.

**Why this priority**: Valuable for staying on top of receivables, but the get-paid spine (create/send/pay/mark-paid) delivers value without a dedicated status dashboard; can follow the first build.

**Independent Test**: With invoices in several states (one past its due date, unpaid), open the status list and confirm each shows the correct state and the past-due unpaid one reads "overdue". Passing = states match reality and overdue is derived from due date + unpaid.

**Acceptance Scenarios**:
1. **Given** invoices in draft, sent, and paid states, **When** the contractor opens the status list, **Then** each invoice shows its correct current state.
2. **Given** a sent invoice whose due date has passed and is unpaid, **When** the contractor views the list, **Then** that invoice is shown as overdue.
3. **Given** a list of invoices, **When** the contractor filters by a state (e.g. unpaid/overdue), **Then** only invoices in that state are shown.

---

**Feature mapping** (assigned at derivation): homed to FEAT-007 (Invoice status tracking). Deferred (not in the first-build selection).
