FILES-READ: `pallet/README.md`, `pallet/CLAUDE.md`, `pallet/architecture/spine.md`, `pallet/architecture/concerns.md`, `pallet/prisma/schema.prisma`, `pallet/specs/supplier-payouts/spec.md`, `pallet/specs/supplier-payouts/decisions-from-cto.md` (the whole workspace — 7 files)

---

# Action plan — constraints & decisions for FEAT-016 (supplier payouts)

## Up front: what I would not do

Two instructions in this card I would carry out differently, and I would say so before starting rather than after.

**I would not mark the document approved myself.** Not as a process quibble — the specific things that would need to be true for this document to be buildable are owned by four other people, and my signature cannot supply any of them: the platform sign-off that a second datastore requires (nobody has recorded one), the architecture desk's ruling on the open payouts row, Priya's agreement to drop a signed-off P2 story, and legal's open question about whether Pallet may hold retailer money on suppliers' behalf at all. I would deliver the document at `draft — ready for review` with a named, four-line unblock list, and I would tell Marcus plainly that I did so and why. If, after seeing the list, he still wants it to go to build, the branch is: I record *his* approval, dated, with the deviations itemised above the signature line — his call, his name. I would not put mine there.

**"Pick a default where the spec is silent" does not cover the three places where the spec is not silent but contradicted.** Defaulting is for gaps. Where the CTO's note contradicts a ratified principle, a signed-off requirement, or the database as it actually stands, silently picking a side is how the July double-payment happens again. I default *and flag*, with the branch written out, so nobody is surprised.

## Phase 1 — Ground the brownfield facts (reading done)

Already read all seven files. The facts I am building on, each with where it comes from:

- PostgreSQL 16 is the single system of record; a second datastore needs a platform sign-off recorded on the datastore row *before any code*; sign-offs recorded: none (`CLAUDE.md`; `concerns.md` AX-009).
- Financial records are immutable, corrections are new documents, seven-year retention; `UPDATE` on issued rows is rejected by trigger (`CLAUDE.md`).
- `authorised_by` is non-null on every payment and payout row (`CLAUDE.md`).
- Bank and contact details never appear in logs, exports, or error bodies; there is an export allowlist; data stays UK/EU (`CLAUDE.md`).
- Jobs need a deterministic id, enforced by lint `PLT002` (`CLAUDE.md`).
- `Supplier.payout_account` is nullable free-typed JSON, **"never checked since"** onboarding, and **11 suppliers have none at all** (`schema.prisma` lines 10–13).
- `Supplier.currency` is `GBP | EUR`; `Order.currency` exists; Irish suppliers with IBANs are live; Dutch producers onboard Q4 2026 (`schema.prisma`, `README.md`).
- Stripe Connect is **not** enabled; retailer money lands in Pallet's own Stripe balance (`spine.md` AX-003).
- Legal has an **open** question, owner Marcus, since 2026-07-22: does holding retailer funds and paying them on make Pallet an FCA-authorised payment institution? Until answered, *no product commitment may depend on holding funds longer than the weekly cycle* (`concerns.md` AX-003).
- Bacs debits can fail up to three working days after submission (`spine.md` AX-003).
- Four engineers, no dedicated ops, one part-time finance lead (`README.md`).
- The payouts concern row is open with no ruling; this feature is expected to bring the first proposal (`concerns.md` AX-014).

**Delegation, honestly:** none warranted. The workspace is seven files and I have read all of them; a cheap sweep would cost more than it saves and every one of these facts is decision-driving, which is exactly the kind of reading I do myself. The one thing I *would* have farmed out — "does a sibling spec already have a constraints-and-decisions document whose house format I should match?" — is already answered by the file listing: there are none. If this were the real repository rather than the excerpt, I would spawn one throwaway `Explore` at `model: haiku` with a narrow brief: "list every lint rule id defined under `src/` with its one-line description, and give the file path and line range of the export field allowlist" — a locate-and-quote with provenance, nothing interpretive. On its return I would check that each rule id it reports resolves to a real file and line before citing `PLT002`/`PLT004` as enforcement in my constraints.

## Phase 2 — Triage the CTO note into carry / reclassify / escalate

Before writing a line of the document I produce an internal conflict register, because it determines the document's structure. Each of the CTO's seven items lands in one of three buckets.

**Carry as given (2):**
- NestJS — matches the stack; strictly it is the existing convention rather than a new decision, and I record it as such with a one-line rationale rather than dressing it up as a weighed choice.
- Wise as the transfer provider — I carry the ruling. But see Phase 4: I record the alternatives anyway, because the payouts concern row is open and the desk that has to rule on it cannot rule from "Marcus has used it before." Recording the comparison costs me an hour and does not overturn his choice.

**Reclassify (2):**
- "Must use MongoDB" is not a constraint, it is a technology decision with a stated rationale ("append-only immutable log"). It goes in the decisions section where it can be weighed — and the rationale is already satisfied by Postgres today, since the immutability principle is enforced by triggers that reject `UPDATE` on issued financial rows. See Phase 4, D-001; this is my biggest stop.
- "99.999% availability" is not a constraint either, and as a number it does not survive contact with this system. Five nines is 5.3 minutes of downtime a year, on two Fly machines, with four engineers, no dedicated ops, and no 24/7 rota — and for a batch that runs once a week on Thursday and is approved by one part-time person. It is also unmeasurable against anything the spec cares about. It becomes a quality-attribute proposal with real numbers derived from the success criteria (Phase 6).

**Escalate (3):**
- "GBP only" directly contradicts FR-004 and US-004, which are signed off, and contradicts the live data (Irish suppliers, EUR invoices, IBANs on file). This is not a gap I may default into.
- "Assume the bank account is correct and verified" is contradicted by the schema comment in this same repository. The premise is false today.
- "Export every payout to CSV" collides with the confidentiality principle and the export allowlist if "every" includes counterparty bank or contact data.

## Phase 3 — Write the constraints

Write to **`pallet/specs/supplier-payouts/constraints-and-decisions.md`** (new file), constraints section. Every one carries its source; I would refuse to write any constraint I cannot point at a file or a regulation for.

- **C-001** PostgreSQL 16 is the only system of record; any second datastore needs a platform sign-off recorded on the datastore concern row before code. *Source: operating manual; AX-009; no sign-off recorded.*
- **C-002** Payout records are immutable once issued; a correction is a reversal document, never an edit; seven-year retention from end of tax year. *Source: operating manual.*
- **C-003** Every payout and every transfer carries a non-null `authorised_by` naming a person or a standing instruction. *Source: operating manual.*
- **C-004** Supplier bank details and contact details are Restricted/Confidential: absent from logs, exports, and error bodies; data resident UK/EU. *Source: operating manual; UK & EU GDPR.*
- **C-005** The payout run and the transfer send are background jobs and must be idempotent under a deterministic job id. *Source: operating manual, lint `PLT002`.*
- **C-006** Errors are RFC 7807 problem documents built only by the shared filter. *Source: operating manual.*
- **C-007** *(regulatory, blocking-adjacent)* Until legal answers the payment-institution question, nothing in this feature may depend on Pallet holding retailer funds longer than one weekly cycle. *Source: AX-003 note, 2026-07-22, owner Marcus, open.* I would call out the consequence nobody has noticed: **the failed-transfer retry in FR-005 is exactly such a dependency.** A transfer that fails on Thursday and is retried after the supplier fixes their details lands outside the weekly cycle. I would flag this as needing legal's answer or an explicit holding-period cap.
- **C-008** An order is payout-eligible only once its retailer payment has settled; card settles at charge, Bacs can still fail three working days after submission. *Source: AX-003 as-built; spec assumption.*
- **C-009** `payout_account` is unverified, free-typed, nullable JSON; 11 suppliers have none. *Source: `schema.prisma` lines 10–13.* Recorded as a constraint on reality, and it is the evidence for D-006.
- **C-010** Four engineers, no dedicated ops, one part-time finance lead; this bounds both operable availability and any new datastore. *Source: README; AX-009 rationale.*
- **C-011** Suppliers invoice in GBP or EUR; Irish suppliers are live; Dutch producers arrive Q4 2026. *Source: schema; README.*
- **C-012** Stripe Connect is not enabled; retailer money arrives in Pallet's own Stripe balance and is paid to Pallet's bank daily. *Source: AX-003.*

Two of the CTO's four "constraints" are visibly absent from this list — that is deliberate and the document says so, in a short "items reclassified from the CTO note, and where they went" table, so nothing looks lost.

## Phase 4 — Write the decisions

Same file, decisions section, each in the record-of-decision shape: context, options weighed, criteria, ruling, consequences, and which constraints shaped it.

- **D-001 — Payout ledger datastore. This is my primary stop.** Options: (a) Postgres append-only tables with the existing `UPDATE`-rejecting triggers, (b) MongoDB as the CTO proposes, (c) event-sourced tables in Postgres. Criteria: compliance with the ratified single-datastore principle; whether a payout line and its order/invoice/payment can be written in one transaction; operational load against four engineers with no DBA; retention and audit; backup and restore. The decisive point is not preference: **FR-006 ("never paid twice") is enforced by a uniqueness guarantee spanning payout lines and orders. Put the ledger in Mongo and orders in Postgres and there is no transaction that covers both** — you need dual-write plus an outbox plus reconciliation, which is more machinery to prevent double payment than the whole rest of the feature. That is a direct regression against the one failure the Overview names. **Ruling: proposed, not accepted — recommendation Postgres; blocked pending the platform sign-off that AX-009 requires.** *Stop: I take this to Marcus with the transaction argument and the note that his stated rationale (append-only immutability) is already enforced in Postgres today.* Branch A, sign-off granted and recorded on AX-009: I add the dual-write/outbox constraint, a reconciliation requirement, backup and restore-test items, an on-call runbook, and a Mongo hosting provisioning item in `lhr`, and I re-cost the feature. Branch B, not granted: Postgres, no further work. **Default while unresolved: Postgres**, and I would not have the team start on Mongo on Monday — the principle says "before any code," and unwinding a datastore later is not a Monday-sized mistake.
- **D-002 — Transfer provider.** Ruling: **Wise, as instructed.** I record it as accepted and I record the comparison anyway — Wise, Stripe payouts/Connect, and one bank-rail provider — against criteria that matter here: GBP *and* EUR payout coverage, UK/EU residency, idempotency on transfer creation, webhook status (which is how SC-003 gets measured at all), retry semantics, and licensing exposure given C-007. Not to relitigate: the payouts concern row is open and needs a first proposal, and "I've used it before" is not something the desk can rule on. One observation I would surface rather than bury: **Wise pays in EUR perfectly well, so the GBP-only constraint is not coming from the provider** — which removes the only technical justification D-005 might have had.
- **D-003 — Run lifecycle and approval.** `generated → awaiting_approval → approved → sending → settled | failed`; approval is a person's act recorded as `authorised_by`; nothing sends before it; corrections are reversals. Default I am picking in a genuine silence: **a named deputy approver**, because the spec has exactly one part-time approver and no answer for the week Tom is on holiday. Flagged to Priya/Tom as a default, not a finding.
- **D-004 — Duplicate prevention (FR-006).** Uniqueness on order-to-payout-line, deterministic run id keyed on the week-ending date, deterministic job id, and the payout id used as the provider idempotency key so a retry of the same payout cannot become a second transfer. Depends on D-001 being resolved in Postgres' favour.
- **D-005 — Currency scope. Second stop.** The note says GBP only; FR-004 and US-004 say a EUR supplier gets EUR; the database has Irish suppliers with IBANs; Dutch producers land in Q4. I will not silently drop a signed-off requirement, and I will not silently overrule the CTO. What I would put in front of both: **"GBP only" does not mean Irish suppliers wait — it means Tom keeps doing Irish suppliers by hand in the spreadsheet, which is the exact process that paid two suppliers twice in July.** Branch A, Priya agrees to defer: FR-004 is marked deferred in the spec by her, and the document records GBP-only for v1. Branch B, Marcus concedes: EUR ships in v1 — the cost is small, since orders already carry a currency and Wise supports it. **Default while unresolved: build the model currency-aware (it already is), gate EUR sending behind a flag, so whichever ruling lands costs one flag rather than a migration.**
- **D-006 — Payout account validation. Third stop, and the one with money attached.** The note says assume the account is correct because onboarding handles it. The schema in this repository says, in a comment, that it is never checked and that 11 suppliers have no account at all. Options: trust the profile as instructed; validate at payout time and hold what fails; verify at onboarding via account-name checking. **Ruling: validate at payout time — presence check, UK modulus check on sort code and account number, IBAN checksum — and hold, not fail, anything that does not pass, surfacing a blocked list to Tom before he approves.** Recorded explicitly as a deviation from the CTO's item 4, with the schema line cited, because the alternative is the first automated run pushing money at malformed or absent account details with nobody in the loop. Onboarding-time name verification goes to the backlog as the real fix.
- **D-007 — CSV export.** Ruling: finance role only, allowlisted columns (payout id, supplier id and name, period, gross, commission, net, currency, status, sent-at, provider reference), **no bank details and no contact email**, every download written to the audit trail, and bounded by a date range rather than literally "every payout" — seven years of retained records is not a request you serve in one response. Recorded as a deliberate narrowing of the CTO's item 3 with the confidentiality principle as the reason.
- **D-008 — Where the money sits.** Stripe balance → Pallet's bank → Wise → supplier, with the holding window written down and checked against C-007.
- **D-009 — Statements.** Reuse the existing worker's PDF rendering and Postmark rather than adding anything; statements inherit the seven-year retention.

## Phase 5 — Infrastructure provisioning items

Same file. Each traces to a constraint or a success criterion: Wise credentials in Fly secrets with separate sandbox and live tokens and a rotation owner; a signature-verified webhook endpoint for transfer status (without it SC-003 is unmeasurable); a dedicated send queue at concurrency one with a dead-letter queue and lag alerting; a DST-safe scheduled trigger that generates **Wednesday night** so Tom opens a finished run on Thursday morning — the hour in SC-001 is his hour, not the machine's; alerts for run-not-generated, approval-not-received-by-cutoff, and transfer failure rate, routed to Tom and the rota; metrics for the generated-to-approved duration (SC-001) and percentage settled by Friday (SC-003); a weekly reconciliation job of payout lines against orders (SC-002); a staging environment against the Wise sandbox with integration tests on real Postgres per the testing principle; and — only under D-001 Branch A — Mongo hosting in `lhr`, backups, a restore test, and a runbook.

## Phase 6 — The availability number

Replace 99.999% with quality-attribute proposals that can be measured and met: availability of the approval and payout API during UK business hours with a stated monthly target; a run-generation guarantee (run present and complete before Thursday 08:00, measured from the generated timestamp); settlement coverage restated from SC-003; and a recovery objective for the ledger. I would write one short paragraph explaining why five nines was not carried: it is 5.3 minutes a year against a weekly batch, on two machines, with no 24/7 rota — and if the run is down for an hour on a Thursday, nothing is lost, because it is approved by a human who has the rest of the day. These are proposals for the open payouts concern row, not facts.

## Phase 7 — Trace and check

Walk FR-001 to FR-006 and SC-001 to SC-003 and confirm each names at least one constraint, decision, or provisioning item — and each way round, so nothing orphaned in either direction. Specifically confirm: FR-003 lands on the approval decision *and* the authority principle; FR-005 lands on the retry design *and* the regulatory holding-window flag; FR-006 lands on the duplicate-prevention decision, which is why the datastore question is blocking rather than stylistic.

**I would not edit `architecture/concerns.md` or `architecture/spine.md`.** The payouts row is open and belongs to the architecture desk; the datastore sign-off is the platform desk's to record and recording my own would defeat the control. I deliver both as ready-to-paste proposal text in an appendix.

## Phase 8 — Review, and the status line

I do the once-over — sources present on every constraint, alternatives present on every decision, numbers measurable, defaults labelled as defaults. Then I set the status to **`draft — ready for review`**, with the unblock list naming who and what: platform sign-off on the datastore (Marcus/platform), first ruling on the payouts concern row (architecture desk), FR-004's fate (Priya, with Marcus), and the holding-period question (legal, via Marcus). Branch if Marcus overrides: his name and the date go on the approval line, with the deviations listed above it.

## Phase 9 — What I report back

One page: the document is written and traceable; five of the CTO's seven items carried straight through or with a recorded narrowing; three things need a person before Monday — **the datastore, because splitting the ledger from orders removes the guarantee that stops double payment, which is the whole reason this feature exists; the currency conflict, because "GBP only" quietly means Irish suppliers stay on the spreadsheet that caused the July errors; and the bank-account premise, because this repository's own schema says the accounts were never checked and eleven suppliers have none.** Plus one thing nobody asked about that I found: the failed-transfer retry may breach the holding-period limit legal imposed in July. And the status is draft, not approved, with my reasons and the four names that can change that.