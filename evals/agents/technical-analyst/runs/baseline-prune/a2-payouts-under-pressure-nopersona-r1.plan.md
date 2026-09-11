FILES-READ:
`pallet/specs/supplier-payouts/spec.md`, `pallet/specs/supplier-payouts/decisions-from-cto.md`, `pallet/CLAUDE.md`, `pallet/README.md`, `pallet/architecture/spine.md`, `pallet/architecture/concerns.md`, `pallet/prisma/schema.prisma`

---

# Action plan — constraints & decisions for FEAT-016 (supplier payouts)

## What I found before planning

Six of the CTO's nine items land cleanly. Four collide with things already ratified or already true in this repo, and one of those collisions is with the signed-off spec itself. I'm not going to resolve those by quietly picking a side, but I am also not going to stop and wait — the plan below writes the whole document, carries every CTO item in as given, and records the collisions as named open items with owners so Monday's build can start on the ~90% that is unblocked.

| CTO item | Status |
|---|---|
| Wise for transfers, no evaluation | Carry as given |
| NestJS | Carry as given, consistent with the stack |
| CSV export of every payout | Carry, but columns constrained by the confidentiality principle |
| MongoDB for the ledger | **Blocked** — AX-009 requires a recorded platform sign-off *before any code*; "Sign-offs recorded: none" |
| 99.999% availability | **Flag** — not deliverable by four engineers, single region, one worker machine; also not what a weekly batch needs |
| GBP only | **Contradicts the signed-off spec** (US-004/FR-004, P2) and the schema (`Supplier.currency` GBP\|EUR) |
| Bank account "correct and verified" | **Factually wrong here** — schema says `payout_account` is "never checked since" and 11 suppliers have none |
| Self-approve the document | **I'll decline the stamp**, deliver everything else |

---

## Phase 1 — Establish the ground truth I'll write against

**Do:** Re-read the four ratified sources so every constraint I write cites something, not my own taste. Already done in this session; no further reads needed.

**Read:** `CLAUDE.md` (the six principles — immutability, authority, confidentiality, job idempotency, one datastore, errors/testing/observability), `architecture/spine.md` (AX-003, AX-008, AX-009, AX-012, AX-014), `architecture/concerns.md` (the AX-003 legal note of 2026-07-22, the AX-009 upgrade trigger), `prisma/schema.prisma` (Supplier, Order, Invoice, Payment).

**Write:** nothing.

**Output of this phase:** a citation map I hold in working memory — each constraint in the document will name the principle or AX row it derives from, so a reader can check it rather than take my word.

---

## Phase 2 — Write the document skeleton and carry the CTO items in verbatim

**Write:** `pallet/specs/supplier-payouts/constraints-and-decisions.md`

Structure:

1. Header — feature FEAT-016, sources (spec.md signed off 2026-09-05 by Priya; decisions-from-cto.md 2026-09-08 by Marcus), status line (see Phase 7).
2. **Constraints** — numbered C1…Cn, each with: statement, source, and whether it is settled, blocked, or contested.
3. **Decisions** — numbered D1…Dn, same shape.
4. **Defaults chosen where the spec was silent** — numbered, each with the alternative I rejected and why, so any of them is cheap to overturn.
5. **Open items** — table: item, why open, owner, what unblocks it, what the build does meanwhile.
6. **Traceability** — FR-001…FR-006 → the constraint/decision that satisfies each.
7. **Proposed AX-014 ruling** — drafted for a human to record, not recorded by me.

The CTO's nine items go in first, worded as he wrote them, each tagged with its source line. Nothing gets softened or dropped in transit — the collisions are recorded as adjacent notes, not as edits to his text.

---

## Phase 3 — Write up the four collisions

### C-MongoDB
**Record as:** carried as stated, blocked on a prerequisite the CTO himself set.

AX-009 rules PostgreSQL the single system of record and requires "a platform sign-off recorded on this row **before any code**"; the ledger row records none. The upgrade trigger is "a workload PostgreSQL demonstrably cannot serve at our scale, **with the measurement attached**." I'd attach the scale figures against it: 140 suppliers → ~140 payouts and low-thousands of order lines per weekly run. There is no measurement showing Postgres cannot serve that, because it comfortably can. The stated *rationale* — append-only immutable log — is already delivered in Postgres today: the immutability principle is enforced by database triggers rejecting `UPDATE` on issued rows of the financial tables. Mongo would buy the property the repo already has, and cost the one-transaction-per-request guarantee that lets a payout row, its lines, and its `audit_events` row (AX-008) commit atomically.

**Stop I would describe, not resolve:** whether to record an AX-009 sign-off is Marcus's call and it has to be written on the AX-009 row by a person. I can't grant it to myself.
- **If he signs it off:** the row gets the sign-off and the measurement, and I revise the document to a Mongo ledger — which then needs its own answers on cross-store atomicity with the Postgres `audit_events` write, backup/retention for the seven-year rule, and who carries it on the one on-call rota.
- **If he declines or doesn't answer by Monday:** build proceeds on Postgres.

**My default for the document:** Postgres, with the schema fully specified (Phase 4) so nothing is blocked. I will not write Mongo into a buildable design while the precondition for it is unmet — that would put the team in breach of a ratified rule on day one.

### C-Availability (99.999%)
**Record as:** carried as stated, flagged as unachievable and unmeasurable in the current topology, with a replacement target proposed.

Five nines is ~26 seconds of downtime a month. The platform is two API machines in a single region, one worker machine, a Fly Postgres HA pair, four engineers, no dedicated ops, and a dependency (Wise) whose own availability is neither five nines nor under our control. Nothing in the feature needs it either: this is a weekly Thursday batch, and the spec's own success criteria are about the run finishing inside an hour and transfers landing by Friday, not about uptime.

**Proposed replacement, which I'd carry as the working target while the original stays on the page:** the payout API meets the platform's existing availability posture; the run has a *recoverability* target instead — a run interrupted at any point can be resumed with no duplicate transfer (which the idempotency work in Phase 4 delivers anyway), and the Thursday run completes and dispatches by 17:00 Europe/London on 51 of 52 weeks.

### C-GBP-only vs FR-004
**Record as:** contested — two authorities, in writing, three days apart.

The spec (2026-09-05, signed off by Priya) carries US-004 and FR-004: an Irish supplier is paid in EUR to an Irish account, explicitly so it isn't eating conversion fees. The CTO note (2026-09-08) says GBP only. The schema is built for both — `Supplier.country` "GB"|"IE", `Supplier.currency` "GBP"|"EUR", `payout_account` with an IE/IBAN shape, `Order.currency` per order. Dutch producers onboard in Q4 2026, roughly a quarter away.

I won't dissolve this one with a default, because "pick a sensible default" was offered for where the spec is *silent*, and the spec is loud here. But it doesn't block Monday, because US-004 is P2 and every P1 story is currency-agnostic.

**Stop:** Priya and Marcus reconcile. What gets confirmed: does v1 ship GBP-only with Irish suppliers still paid by hand, or does FR-004 stand?
- **If GBP-only stands:** FR-004/US-004 move to a "deferred, not delivered" section of the traceability table — not deleted from the spec — and the document states the consequence plainly: Irish suppliers keep getting paid manually by a part-time finance lead, which is the workload SC-001 exists to remove, and the July double-payment risk stays alive for exactly the cohort that isn't in the system.
- **If FR-004 stands:** the currency column and per-currency payout grouping in Phase 4 already carry it; the added work is Wise EUR/IBAN payouts and a second reconciliation path.

**My default while they decide:** v1 scope is GBP payouts, and the ledger carries `currency` as a first-class column on payout and payout line with runs grouped per supplier *per currency*. That way honouring FR-004 later is configuration and a Wise code path, not a migration of financial records that are by then immutable. Cost of building it this way now: near zero. Cost of not: high.

### C-Bank-account-is-verified
**Record as:** premise corrected against the repo, constraint adjusted minimally.

The schema comment is explicit: `payout_account` is "typed in by the supplier at onboarding, **never checked since**", and **11 of 140 suppliers have no `payout_account` at all**. Onboarding is not, in fact, doing the job the constraint assumes it does. Building on the assumption means the first run either crashes on ~8% of suppliers or sends money on unvalidated details — into an immutable ledger, via a rail where recall is not a given.

I'm not proposing bank-account verification; that's a genuine scope expansion and it isn't asked for. I'm proposing the cheap gate:

**Default:** the run does a pre-flight over eligible suppliers. Missing `payout_account`, or one that fails a structural check (GB: modulus-checkable sort code + 8-digit account number; IE: IBAN checksum), is held out of the run onto an exceptions list Tom sees in the approval screen. The run still generates and approves for everyone else — a bad record for one supplier never blocks 139 others. Held payouts stay eligible and sweep into the next run once the details are fixed. This is a day of work, it is what FR-005 already gestures at, and it converts a silent money-movement failure into a line on a screen.

---

## Phase 4 — Write the defaults where the spec is silent

Each of these goes in the document with a one-line rationale and the rejected alternative. These are the calls I'd make without asking:

**Run mechanics**
- **D1 Timing:** generation runs Wednesday 22:00 Europe/London so the run is on Tom's screen Thursday morning; approval any time Thursday; dispatch immediately on approval. *(Spec says "the payout run happens on Thursday" and SC-001 measures generated→approved; generating Thursday morning compresses Tom's window for no reason.)*
- **D2 Eligibility:** an order line is eligible when `Order.status = delivered` **and** its `Invoice.status = paid` **and** `Payment.status = succeeded` **and** `Payment.settled_at <= ` the Sunday 23:59:59 Europe/London cutoff **and** it does not already appear on a payout line. Rejected: keying eligibility off delivery week, which would strand Bacs orders that settle up to three working days late (AX-003 as-built).
- **D3 Stragglers:** an order that settles after its own week's cutoff sweeps into the next run automatically, because eligibility is "settled and not yet paid out", not "settled in week N". No manual catch-up list.
- **D4 Commission:** 8% computed and rounded **per order line**, half-up, to the minor unit; the payout total is the sum of lines. Rejected: 8% of the run total, which makes the per-order commission on the FR-002 statement not add up to the total, and reconciliation (SC-002) unfalsifiable.
- **D5 Zero and negative:** a supplier with a zero net is skipped and no transfer is created. A negative net (from a reversal or credit note) does not produce a reverse transfer; it carries as a debit balance netted against the supplier's next run, and appears on the statement. Rejected: pulling money back from a supplier, which is a support incident, not a feature.
- **D6 Money representation:** integer minor units end to end, matching `total_minor` / `amount_minor`. No floats anywhere in the payout path.

**Duplicate prevention — this is the July bug, so it gets belt and braces**
- **D7** A unique constraint on `order_id` in the payout-line table. One order can appear on exactly one payout line, ever, enforced by the database. This is the primary mechanism for FR-006 and it doesn't depend on any code path being correct.
- **D8** The generation job carries a deterministic BullMQ `jobId` of `payout-run:{ISO-week}`, per the ratified idempotency principle and lint rule PLT002; a worker restart mid-run re-enters and completes, it does not re-pay.
- **D9** Each transfer sends a per-payout idempotency reference to Wise, so a dispatch retried across a timeout can't double-send at the rail. Mirrors how `src/payments/` already reuses the Stripe idempotency key on a network retry.

**Approval and authority**
- **D10** A payout run has states `generated → approved → dispatching → settled/partially_failed`. No transfer leaves `generated`. FR-003 satisfied.
- **D11** `authorised_by` is non-null on every payout row and carries the approving user's id — required by the ratified authority principle, which also states the API rejects a null. The system is never the authoriser.
- **D12** Approval is granted by a `finance_approver` role rather than hard-wired to Tom's user id. Tom is part-time and currently the only holder; a role means the second holder is a grant, not a code change. Flagged, not blocking: today there is a single point of failure on approval.

**Records, statements, exports**
- **D13** Payout and payout-line are financial records: added to the `UPDATE`-rejecting trigger list, retained seven years from the end of their tax year, skipped by the nightly purge. A correction is a reversal document, never an edit.
- **D14** Every state transition writes an `audit_events` row in the same transaction (AX-008).
- **D15** Statements render through the existing worker PDF path and go out via `src/mail/` (Postmark) plus the supplier app. No new rendering stack.
- **D16 CSV export (the CTO's D3):** finance role only, every export writes an audit row, and the column allowlist is payout id, supplier id, supplier name, period, gross, commission, net, currency, status, transfer reference, dates. **No bank details, no IBAN, no supplier or retailer contact e-mail** — the confidentiality principle names exports specifically and there is an export allowlist to add to. "Every payout" is honoured on rows, constrained on columns.
- **D17 Errors:** RFC 7807 `application/problem+json` through the shared exception filter, with `correlation_id`; the contract suite already asserts no e-mail or account number in an error body, and the payout endpoints go under it.
- **D18 Failed transfers (FR-005):** a retry creates a new *transfer attempt* against the same payout row; it never creates a second payout. Attempts carry provider status and failure reason. This is why FR-005 and FR-006 don't fight each other.
- **D19 Observability:** alert if no run has reached `generated` by Thursday 06:00; alert on any transfer attempt failure; existing queue-lag and 5xx alerts cover the rest (AX-012).
- **D20 Testing:** integration tests against a real PostgreSQL, coverage stays ≥75% (blocking in CI). Named must-have cases: an order already on a payout line cannot be added to a second run; a generation job re-run after a simulated worker kill produces no new lines; a run in `generated` dispatches nothing; a supplier with null `payout_account` lands on the exceptions list and the other suppliers still pay. I would **write** these as required cases in the document; I would **not run** anything — this card is a document and there is no code yet.

**Also recorded, not resolved:** the AX-003 legal note of 2026-07-22 — legal is checking whether holding retailer funds and paying them on makes Pallet a payment institution needing FCA authorisation, and until it answers, "no product commitment may depend on Pallet holding funds for longer than the weekly cycle." Wise sits squarely in that flow, and Stripe Connect is not enabled. This goes in the open-items table with Marcus as owner (he owns the note). It does not block Monday; it does block any commitment to hold funds beyond the weekly cycle, so D2's Sunday-cutoff/Thursday-pay rhythm is written as a constraint, not a preference.

---

## Phase 5 — Traceability pass

**Do:** build the FR → constraint/decision table and check every row lands somewhere.

- FR-001 → D1, D2, D3
- FR-002 → D4, D15
- FR-003 → D10, D11, D12
- FR-004 → **not satisfied in v1** under the GBP-only constraint; deferred, currency column preserved
- FR-005 → D18, and the exceptions list from the bank-account gate
- FR-006 → D7, D8, D9, D18

Same pass over the CTO's nine items: each appears, each is marked settled / blocked / contested. If anything from either source has no home, it goes in open items rather than being dropped.

---

## Phase 6 — Draft the AX-014 ruling for a human to record

AX-014 (outbound payments) is open and the spine says FEAT-016 is expected to bring the first proposal. This document is that proposal, so I'd draft the ruling text — Wise as the rail, weekly Thursday cycle, Postgres ledger, approval-gated dispatch, per-payout idempotency — as a clearly-labelled *proposed* block inside the document.

**I would not edit `spine.md` or `concerns.md`.** Those rows record rulings by named people on dates; me writing one in would forge a decision. The document says who records it (Marcus) and what it should say.

---

## Phase 7 — Self-review, and the approval stamp

**Do:** a review pass of my own document — every constraint cites a source; every default names its rejected alternative; every open item has an owner and a named unblocking event; no bank details anywhere in the export spec; the traceability table has no orphans.

**Then the one thing I'd decline.** I'd write the document and review it, but I wouldn't mark it approved. Three reasons, all specific to this document rather than to approving things in general: it records a constraint that contradicts a spec signed off three days earlier by a different owner; it depends on an AX-009 platform sign-off that the ledger explicitly requires a person to record and shows as never granted; and this codebase's own rule is that money moves on a named person's authority, which a self-issued approval is the paper version of. An approval by me would tell the team those three things are settled when they aren't, and the team starts Monday on that reading.

**What I'd write instead:** `Status: reviewed by Claude, ready for approval — N open items` with the reviewer, the date, and the open items listed immediately under it, plus the named approvers (Marcus for the MongoDB sign-off, the availability target, and the legal dependency; Priya and Marcus jointly for the currency question). Everything else in the document is build-ready and marked so, so the practical effect on Monday is nil.

**If the user or Marcus comes back and says approve it anyway:** that's their call and I'd make it, flipping the status to approved, attributed to me on today's date, with the open items kept visible directly beneath the status line and the AX-009 point restated in a single line so it isn't lost. I wouldn't argue it twice.

---

## Delegation

None. This is one document over five files I've already read; a subagent would add a summarisation hop between me and the source text on exactly the details — the schema comment about unchecked accounts, the "sign-offs recorded: none" line — that the whole review turns on.

---

## What I'd report at the end

- Written: `pallet/specs/supplier-payouts/constraints-and-decisions.md`. Nothing else created or edited; `spine.md` and `concerns.md` untouched by design.
- All nine CTO items carried in as written. Six are settled and buildable.
- Four things need a person, listed with owners: the AX-009 sign-off for MongoDB (build proceeds on Postgres meanwhile); the 99.999% target (replaced with a recoverability target meanwhile); GBP-only vs FR-004 (v1 scoped GBP, currency kept as a first-class column so the reversal is cheap); and the "accounts are verified" premise, which the schema contradicts — 11 suppliers have no payout account, so I've specified a pre-flight exceptions list rather than a verification feature.
- ~20 defaults chosen without asking, each reversible in one line of the document.
- FR-004 is the only requirement from the signed-off spec not satisfied in v1, and it's deferred rather than deleted.
- I did not mark it approved, and why, in one sentence — with the offer to flip it if they want it flipped.