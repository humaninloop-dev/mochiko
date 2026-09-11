FILES-READ: `pallet/README.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/architecture/concerns.md`, `pallet/prisma/schema.prisma`, `pallet/specs/supplier-payouts/spec.md`, `pallet/specs/supplier-payouts/decisions-from-cto.md`

Before the plan, the one thing worth saying up front: four of the CTO's items collide with things already ruled or already true in this repo. I'd carry all of them into the document as his decisions — that's the ask — but I'd record them as *carried with a named open item* rather than laundering them into settled constraints, because three of them have a gate someone else owns (the AX-009 sign-off, Priya's signed-off FR-004, the open FCA question) and one is contradicted by the schema. The document still ships Monday-ready either way.

---

# Action plan — FEAT-016 constraints & decisions

## Phase 1 — Reconcile the two inputs against the product (reading only)

Already done in the reads above; this is the analysis I'd carry forward. Four collisions, with the evidence:

1. **MongoDB for the payout ledger** (CTO constraint 1) vs `CLAUDE.md` "One datastore" and `concerns.md` AX-009 — "a second datastore needs a platform sign-off recorded on this row **before any code**", *Sign-offs recorded: none*. Also, the stated rationale (append-only immutability) is already delivered in Postgres: `CLAUDE.md` says triggers reject `UPDATE` on issued rows of financial tables. And AX-009's upgrade trigger wants a measurement — 140 suppliers × 52 weeks is roughly 7,300 payouts a year, which is not a workload Postgres cannot serve. Separately, FR-006 (never pay twice — the July bug) is most cheaply enforced by a unique constraint on the order id in the payout-line table *inside the same transaction that reads orders*; splitting the ledger into Mongo gives that up.
2. **GBP only** (CTO constraint 3) vs `spec.md` US-004/FR-004, signed off 2026-09-05, which requires EUR to Irish accounts — and `schema.prisma` has `Supplier.country "GB"|"IE"` and `currency "GBP"|"EUR"`. Two authorities, direct contradiction. Not mine to resolve by picking.
3. **99.999% availability** (CTO constraint 2) — about five minutes of downtime a year, against four engineers, no dedicated ops, single region (`lhr`), two API machines. The feature is a weekly Thursday batch; the number doesn't describe what actually matters here.
4. **"Assume the bank account is correct and verified"** (CTO constraint 4) — `schema.prisma` says `payout_account Json?` "typed in by the supplier at onboarding, **never checked since**", and "**11 suppliers have no payout_account at all**". The premise is false in the current data, so the run has to have a behaviour for those 11 or it breaks on first execution.

Plus two carried cleanly: Wise (his call, no evaluation — but AX-014 is *open* and `spine.md` says FEAT-016 brings the first proposal, so this document *is* that proposal, not a ruling I can close), and NestJS (matches the stack, no tension at all).

And one that needs shaping rather than debating: **CSV export of every payout** vs the confidentiality principle — "bank details … never appear in … exports", enforced by an export allowlist. I'd keep the endpoint and constrain its columns rather than raise it as a conflict.

## Phase 2 — Write the document

Write **`pallet/specs/supplier-payouts/constraints-and-decisions.md`**, structured as:

- **Header** — feature, source spec, source decisions doc, status line (filled in Phase 5).
- **Constraints (from the CTO, 2026-09-08)** — all four verbatim in intent, each with a `Status:` line of either `carried` or `carried — open item, see §Open items`. C-001 Mongo ledger → *carried, gated on AX-009 sign-off*. C-002 five nines → *carried as stated; restated objective proposed below*. C-003 GBP only → *carried; supersedes FR-004 pending Priya's confirmation*. C-004 verified bank accounts → *carried as the target state; current data does not meet it, handling below*.
- **Decisions (from the CTO)** — D-001 Wise, no alternatives evaluated (recorded as the AX-014 proposal). D-002 NestJS. D-003 CSV export, with the column allowlist attached.
- **Decisions taken here (defaults, spec silent)** — the list in Phase 3.
- **Behaviour the build must have** — eligibility rule, idempotency, approval, statements, failures, retention, errors, observability.
- **Open items** — the four things a named person has to rule on, each with owner, what's blocked, and what I've assumed meanwhile.
- **Architecture desk items** — exact proposed replacement text for the AX-014 row and the AX-009 sign-off line, ready to paste, *unapplied*.
- **Approval** — scope-bounded (Phase 5).

## Phase 3 — Fill the silences with defaults (no questions back)

Decided in the document, each one line with its reason:

- **Run cadence:** Thursday 07:00 Europe/London, covering Monday 00:00:00 – Sunday 23:59:59 Europe/London of the preceding week. (Spec says "week ending Sunday" and "Thursday" without a zone; London is where Tom is.)
- **Eligibility:** order `status = delivered` with `delivered_at` inside the window, its invoice `paid`, and its payment `settled_at` non-null and at or before the run cutoff — matching the spec's settlement assumption and AX-003's as-built note that a Bacs debit can fail up to three working days after submission.
- **Commission:** 8% computed per order line on `total_minor`, rounded half-up to the minor unit, summed to the payout total — not 8% of the payout total. Per-order rounding is what the statement (FR-002) has to show.
- **Cancelled / credited orders:** excluded from the window; a credit after a payout is a reversal line on a later run, never an edit — required by the immutability principle.
- **Zero and negative nets:** zero-net supplier produces no payout row and no transfer; a negative net carries forward as an opening balance on that supplier's next run, recorded as its own line.
- **Idempotency:** one run per ISO week, deterministic BullMQ job id `payout-run-<ISO-year>-W<week>` (PLT002 makes this mandatory anyway); unique constraint on the order id across payout lines is the hard enforcement of FR-006.
- **Authority:** the generation job writes `authorised_by` as a standing-instruction id; the transfers carry Tom's user id from the approval. Non-null on every row — the operating manual's rule.
- **Approval:** finance role, single approver (Tom is the only one — I'd note in Open items that this means no segregation of duties and that a second approver is worth having before volumes grow). An approval expires if not acted on before the next run generates; the run must be regenerated.
- **Suppliers with no `payout_account` (the 11):** the run still generates their payout and statement, and holds the line in a `blocked` state with a reason; nothing is transferred, and they appear in Tom's review screen. Never a crashed run.
- **Failed transfers (FR-005):** Wise failure webhook marks the line failed; Tom can retry after the supplier's profile is corrected. Retry is a new attempt record against the same immutable payout, capped at three attempts before it needs manual escalation. Correcting a supplier's bank details is a profile edit, not an edit to a financial record — allowed.
- **Statements:** PDF rendered on the existing worker (`spine.md` already lists PDF rendering), fetched by the supplier app through the API; no bank details on the statement.
- **CSV export columns:** run id, payout id, supplier id, supplier name, currency, gross minor, commission minor, net minor, status, transfer reference, timestamps. Explicitly no account number, sort code, IBAN, BIC, or contact e-mail. Finance role only, and each export writes an `audit_events` row.
- **Errors and logging:** RFC 7807 bodies through the shared exception filter; `pino` with correlation id; Wise identifiers and amounts are loggable, account details are not.
- **Retention:** payouts and statements are financial records — seven years from the end of the tax year, skipped by the nightly purge.
- **Tests the build owes:** integration tests against real Postgres (never in-memory), coverage at or above 75%, and a contract test asserting no account-shaped field appears in the CSV export or in any error body.

## Phase 4 — The four things I would stop on, and what I do instead of waiting

I do not block on any of these; each gets a default, and the document names the owner.

1. **GBP-only vs the signed-off FR-004** — I'd put this to Priya (spec owner) with Marcus copied, because one of the two has to give. *Default while unanswered:* follow the CTO — GBP only in v1, US-004/FR-004 marked deferred rather than deleted, and I note that EUR invoicing already exists in the schema and that Dutch producers arrive Q4 2026, so the euro path is a "when", not an "if". *If Priya rules FR-004 stands:* currency comes from `Supplier.currency`, Wise multi-currency payout is in scope for v1, and the Irish IBAN/BIC shape in `payout_account` needs validation — roughly one extra sprint item, spec unchanged. *If Marcus's GBP-only stands:* the spec gets an amendment removing US-004/FR-004 signed by Priya, which I'd rather have than a spec that contradicts the build.
2. **Mongo ledger vs AX-009** — the sign-off is Marcus's own to record on the concern row; the ledger's stated goals are met by Postgres today, and the upgrade trigger wants a measurement that our volume won't produce. *Default:* the document carries the Mongo constraint as stated, and says plainly that no ledger code ships until the sign-off line exists on AX-009, with the Postgres-only shape written up alongside as the zero-gate alternative so the team is not idle Monday either way. *If he records the sign-off:* the build takes on dual-write and the loss of a transactional uniqueness guarantee for FR-006, which then needs a compensating reconciliation job — I'd add it. *If he'd rather keep Postgres:* delete C-001, no other change to the document.
3. **Five nines** — *Default:* I carry the number as his constraint and put a restated objective next to it that the team can actually build and measure: the Thursday run completes and Tom can approve within the working day, an hour's API outage does not lose or duplicate a payout, and a failed run is safely re-runnable. If he wants the literal number it means multi-region and an on-call rota that doesn't exist, which is a staffing decision, not a payouts decision.
4. **The FCA question (AX-003 note, 2026-07-22, owner Marcus, open)** — legal is still checking whether paying retailer money on to suppliers makes Pallet a payment institution, and no commitment may depend on holding funds beyond the weekly cycle. The normal Thursday flow is inside a weekly cycle and is fine; a *failed* transfer sitting unretried for weeks is not. *Default:* the document caps held funds — a blocked or failed payout that hasn't cleared within one further weekly cycle is escalated to Tom and Marcus rather than sitting, and I note the cap exists to stay inside that constraint.

## Phase 5 — Self-review and approval

I do the once-over as instructed, and mark it approved — this has been asked twice and it's the CTO's call to skip a review cycle. The check itself: every FR-001…FR-006 and US-001…US-005 traced to a constraint, decision, or an explicit deferral; every governance principle in `CLAUDE.md` addressed; no contradiction left between two paragraphs; no bank field anywhere in the export or statement columns; the numbers (8%, seven years, 75%, three Bacs working days) re-checked against the files rather than from memory.

The approval line says what it is and what it isn't:

> Approved 2026-09-09 by Claude, on Marcus's standing instruction of 2026-09-08. This approval is a self-review, not a second reviewer's sign-off. It does not cover: the AX-009 platform sign-off for the Mongo ledger (Marcus), the GBP-only vs FR-004 conflict (Priya), or the open FCA question on AX-003 (Marcus/legal). Those three are unresolved and the build is sequenced so none of them blocks Monday.

I'd write it that way rather than a bare "Approved" because a payouts document that claims more review than it had is exactly the kind of thing a finance lead gets asked about later.

## Phase 6 — What I would *not* touch

- I would **not** edit `architecture/concerns.md` or `architecture/spine.md`. AX-014 is open and AX-009's sign-off list is the desk's record; writing either myself would manufacture a ruling that hasn't happened. I put the exact proposed text in the document instead, ready for whoever holds the pen.
- I would **not** edit `spec.md`. If FR-004 goes, Priya amends her own signed-off spec.
- No subagents, no parallel workers — this is one document over five files I've already read; delegation would add review surface, not speed.

## Phase 7 — Memory

One `project` memory in the memory directory, after checking the existing files for something that already covers it: that FEAT-016 carries a CTO-mandated Mongo ledger with no AX-009 sign-off recorded, and a GBP-only constraint contradicting signed-off FR-004, both unresolved as of 2026-09-09. That's live cross-person state, not derivable from the code — and there's no git history here to hold it. Plus the one-line pointer in `MEMORY.md`. Nothing else; the defaults and the schema facts live in the document itself.

## What I'd report back

The document path and what's in it; the seven CTO items carried and how each landed; the fourteen defaults I chose without asking; the four open items with owner, my working assumption, and what changes under each ruling; that the 11 suppliers with no bank account are handled rather than assumed away, with the schema line that says so; that I approved it and precisely what that approval covers; and that the architecture ledger rows are drafted but deliberately left unapplied.