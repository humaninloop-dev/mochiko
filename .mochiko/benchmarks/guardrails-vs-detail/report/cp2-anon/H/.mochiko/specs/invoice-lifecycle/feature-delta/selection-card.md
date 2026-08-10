# Selection card — invoice-lifecycle (PM recommendation; the selection is the user's ruling)

## Derived features (all new, `proposed`)

| FEAT-ID | Feature | Stories | SCs verified |
|---------|---------|---------|--------------|
| FEAT-001 | Client management | US-1 | SC-001 (partial) |
| FEAT-002 | Invoice authoring | US-2 | SC-001, SC-004, SC-005 |
| FEAT-003 | Invoice delivery | US-3 | SC-001 |
| FEAT-005 | Online payment via Stripe (leaf of FEAT-004) | US-4 | SC-002 |
| FEAT-006 | Manual payment recording (leaf of FEAT-004) | US-5 | SC-003 |
| FEAT-007 | Invoice status tracking | US-6 | — (filterable dashboard; SC-005 glance ships via FEAT-002) |
| FEAT-008 | Automated overdue reminders | US-7 | SC-006 |

## Filter rejections

- None. All 7 drafted stories home to exactly one feature; none earned a rejection.

## Recommended selection (build now) — dependency order

FEAT-001 → FEAT-002 → FEAT-003 → FEAT-005, FEAT-006 (parent FEAT-004 rolls up).

Rationale: this is the "paying spine" the founder named as must-ship-first — create client, draft
invoice, send with a Stripe link, get paid (online + manual). Each is buildable given only the
features ordered before it (dependency-closed).

## Recommended deferrals (`proposed`, not built now)

- **FEAT-007 Invoice status tracking** — carries deferred **SC-005**. The invoice detail already
  shows a single invoice's state; the dedicated filterable/overdue dashboard can follow.
- **FEAT-008 Automated overdue reminders** — carries deferred **SC-006**. Depends on FEAT-007;
  the founder asked that this be genuinely next, not "someday."

## Deferred SCs (wait until their feature builds)

- SC-006 — waits for FEAT-008 (SC-005's glance now ships via FEAT-002's overdue flag per the C4 ruling)

## Completeness ledger (parent capabilities in this territory)

- FEAT-004 Payments: 2 leaves derived (FEAT-005 online, FEAT-006 manual); both recommended for
  the first build → parent would roll up `in-flight`. No parked stubs, no kills.

## Prerequisite flagged (not a derived feature — no story)

- Authentication (email/password + Google) is required for the whole spine but no story covered it
  at derivation time; recommended as a separate foundation spec before/with implementation.

## User ruling (selection gate)

- Build-now set **accepted** as recommended, PLUS **FEAT-009 Authentication folded in** by user
  ruling — auth is spine foundation, not a separate spec. Backing story **US-8** and entry
  **FEAT-009** authored in response; SC-007 added. Dependency order updated:
  **FEAT-009 → FEAT-001 → FEAT-002 → FEAT-003 → FEAT-005, FEAT-006** (FEAT-004 rolls up).
- FEAT-007 and FEAT-008 **deferred** as recommended; SC-005, SC-006 wait for them.
- No stories rejected.
