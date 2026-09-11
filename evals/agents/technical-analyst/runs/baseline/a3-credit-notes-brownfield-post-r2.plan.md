FILES-READ: `pallet/specs/credit-notes/spec.md`, `pallet/specs/credit-notes/design-brief.md`, `pallet/CLAUDE.md`, `pallet/README.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/contracts/openapi.yaml`, `pallet/src/accounting/xero-export.ts`, `pallet/src/statements/statement.service.ts`, `pallet/src/documents/render.service.ts`, `pallet/src/documents/templates/invoice.hbs`

---

# Action plan — FEAT-015 credit notes: data model + API contract

## Phase 0 — Orientation (done; recorded here because it drives everything below)

The workspace is 12 files and I have read all of them, so there is no bulk-read to farm out. What the reading establishes:

- The brief bounds me to four gaps: contract (G1, P1), data model (G2, P1), the over-credit ruling (G3, P2), PDF design (G4, P3). Invoice/payment/retailer surfaces are out of scope *except* where a gap forces them — and G1 explicitly forces one ("exposing an invoice's credited amount").
- `adj_` / `Adjustment` / `POST /orders/{id}/adjustments` are **already taken** by a different concept (delivery quantity noted at the door, pre-invoice), directly contradicting the spec author's "keep that name in the API".
- Adding a value to `InvoiceStatus` breaks three exhaustiveness guards, one of which is in a front-end repo that is not here (`xero-export.ts`, `statement.service.ts`, retailer app invoice badge).
- Stripe refunds are constrained by the as-built note on AX-003: 5–10 day card settlement, refused beyond 180 days, Bacs refund can fail via `refund.failed`.
- Xero already has a CreditNotes resource with a **two-call** shape (create, then allocate) and hard rate limits — that is the export path, and its partial-failure mode is a data-model obligation.
- `Document` is loosely polymorphic (`subject_id @unique`, relation only to Invoice; statements carry none), so a third kind needs no new relation.

One delegation I would make here despite having read everything: a single disposable `Explore` subagent with `model: haiku`, briefed to grep the repo for every occurrence of `amount_outstanding`, `InvoiceStatus`, `countsTowardsOutstanding`, `Adjustment`, and the literal `adj_`, and return file + line + surrounding line only. Purpose is completeness insurance on the blast radius, not discovery. On return I check it names nothing outside the five files I already read; if it does, that file goes into Phase 1's register before I design. The two front-end repos are not in this workspace, so their exposure stays an asserted risk I hand back, not a verified one.

## Phase 1 — Collision and conflict register (written first, because three of these change the artifacts' shape)

I would write `pallet/specs/credit-notes/design-notes.md` holding the register below, and raise all of it in one batch rather than trickling questions.

**C1 — Name collision. Stop and confirm with Priya.** The spec says to call the record "Adjustment" in the API; that name, that id prefix, and that URL segment already mean delivery-door quantity changes. Reusing it would give the accounting integrator two unrelated record types behind one `adj_` prefix. Note that the spec contradicts itself in its own success criteria — SC-002 measures "from the credit-notes table". **Default I proceed under:** the resource is `CreditNote`, path `/credit-notes`, id prefix `cdn_`, and the supplier app keeps the word "Adjustment" as a display label only. If Priya rules the API must say Adjustment: I would refuse a bare reuse and offer `DeliveryAdjustment` (rename the existing, a breaking v2 change with its own migration and integrator notice) or `InvoiceAdjustment` as the new name with prefix `iadj_` — and rework the contract's paths, component names, and every example accordingly.

**C2 — "Cancel after issuing" conflicts with the immutability principle. Stop and confirm with Priya and Tom.** The spec's author note wants a supplier to cancel an issued credit note and the retailer to "see it disappear". The operating manual forbids editing issued financial records (triggers reject `UPDATE`), requires seven-year retention, and requires an audit row per mutation; and by then the credit note may already be created and allocated in Xero. **Default:** a draft can be cancelled (nothing ever recorded against the invoice); an issued credit note can only be **voided** — an immutable row moves to `void` with `voided_at`, `voided_by`, `void_reason`, the credit stops counting against the invoice, and the export pushes a matching reversal to Xero. It remains visible to both parties marked void; it does not disappear. If Priya insists on disappearance for the retailer, that is a presentation decision for the retailer app, not a data decision, and I would say so in writing rather than model a delete.

**C3 — Decimal pounds conflict. I refuse this one and report it.** The spec asks for `12.50`-style amounts. The contract's stated convention is `{ amount_minor, currency }`, never a decimal, and the schema stores integer minor units. Introducing a decimal money form for one feature would hand the accounting integrator two money encodings in one document. Credit notes use `Money`; the supplier app formats for display.

**C4 — `amount_outstanding` changes meaning. Stop and confirm with engineering before the live contract is touched.** FR-003 requires the invoice to show outstanding net of credits, so `amount_outstanding` must become `total − amount_paid − amount_credited`. The shape is unchanged, so nothing fails loudly — which is exactly the danger for the integrator and for `statement.service.ts`'s outstanding arithmetic. **Default:** redefine it, add a sibling `amount_credited` so a consumer can recover the gross figure, call the redefinition out in the contract description, and bump to 2.15.0. Branch if rejected: keep `amount_outstanding` gross and add `amount_outstanding_net` — uglier, and I would note that it leaves FR-003 satisfied only for clients that adopt the new field.

**C5 — Monthly statements.** Statements sum invoices by status and know nothing of credits, so they will overstate what a retailer owes. The brief says statements are not in my gap list. I flag it as a discovered gap for the review, with the one-line cause, and do not redesign statements.

**C6 — Who chooses applied vs refunded.** FR-004 says "applied to the next invoice, or refunded", never who decides. **Default:** it is derived, not chosen (rule in Phase 2, D-004). Question goes to Priya.

## Phase 2 — Decisions, written to `pallet/specs/credit-notes/constraints-and-decisions.md`

Each decision gets the real alternatives and the trade-off, and traces to the constraint or architecture row that shaped it. Constraints first, recorded with their sources rather than asserted: issued financial records are immutable and retained seven years; one datastore (so no side-table in Redis for pending credit balances); Stripe's 180-day refund refusal and Bacs failure mode; Xero's 60/min and 5,000/day limits and its two-call credit-note shape; one PDF pipeline only; error bodies are Problem Details and must not leak counterparty data; background jobs need deterministic ids.

- **D-001 Resource name and id prefix** — `CreditNote` / `cdn_`. Alternatives: reuse `Adjustment` (rejected: collides with a live concept and a live URL), `InvoiceAdjustment` (viable fallback, keeps the supplier vocabulary, costs an unfamiliar prefix).
- **D-002 Issued credit notes are voided, never deleted or edited** — alternatives: hard delete (rejected: violates immutability and retention, and leaves Xero out of sync), edit-in-place before export only (rejected: a race with an hourly job decides whether history exists).
- **D-003 No new `InvoiceStatus` value** — credits are expressed as additive fields on the invoice. Alternative: add `credited`/`part_credited` (rejected: breaks two in-repo exhaustiveness guards and one in a repo I cannot change in this pass; an invoice that is part-credited and part-paid has no single status anyway).
- **D-004 (G3 — the spec's open question) A credit note may exceed the invoice's *outstanding* amount, but the sum of non-void credit notes against an invoice MUST NOT exceed the invoice total, and no single line may be credited beyond what that line still has uncredited.** Disposition follows from the arithmetic rather than from a user choice: the credit first reduces the invoice's unpaid balance; any excess corresponds to money already received and is refunded against the settled payment; if Stripe will not refund (past 180 days, or a Bacs refund that fails), the excess becomes a credit balance applied to the retailer's next invoice from that supplier. Alternatives weighed: cap at outstanding (rejected: a fully paid short delivery would be uncreditable, the exact case in the spec's overview), allow unlimited goodwill (rejected: an uncapped negative receivable with no authority trail, and Tom has not asked for it) — goodwill credit is explicitly out of scope and recorded as such.
- **D-005 Refunds are a new record, not a negative `Payment`** — `Payment.invoice_id` is `@unique`, so a second row is impossible, and a refund must carry `authorised_by` in its own right under the money-authority principle.
- **D-006 Export reuses the `accounting.export` job** with Xero's CreditNotes + Allocations pair, a deterministic job id per credit note, and persisted `xero_credit_note_id` so the create and the allocate steps can fail independently and resume without double-posting. No new architecture row; AX-004's as-built line gains credit notes.
- **D-007 PDF reuses the one documents pipeline** (AX-007): a new `DocumentKind` value, one template, one registered loader. No second path.
- **D-008 Money stays in minor units** (C3).

Where a decision merely follows an existing architecture ruling I say so and do not re-litigate it.

## Phase 3 — Data model → `pallet/specs/credit-notes/data-model.md`

Written as entity definitions plus the Prisma delta I am proposing against `schema.prisma` (as a proposal in the design doc; I would not edit `schema.prisma` or write a migration in this pass, since the card is design and the live schema carries `UPDATE`-rejecting triggers whose interaction with new rows needs the engineering stop in Phase 1).

Entities:

- **`CreditNote`** — `id` (`cdn_`), `invoice_id`, `supplier_id`, `retailer_id` (denormalised to match `Invoice`'s existing shape and to let the retailer list without a join), `status`, `reason` enum, `reason_note` free text, `total_minor`, `currency` (inherited from the invoice, never independently settable), `issued_at`, `issued_by`, `cancelled_at`, `voided_at`, `voided_by`, `void_reason`, `created_at`. Reason enum reuses the door vocabulary where it genuinely matches and adds what the spec's overview names: `short_delivery`, `damaged`, `price_correction`.
- **`CreditNoteLine`** — `id`, `credit_note_id`, `invoice_line_id`, a quantity-based *or* amount-based basis (FR-001's "a quantity or an amount per line") with `quantity` nullable and `amount_minor` always resolved and stored, so downstream arithmetic never re-derives price.
- **`CreditNoteRefund`** — `id`, `credit_note_id`, `payment_id`, `stripe_refund_id`, `amount_minor`, `status` (`pending`, `succeeded`, `failed`), `failure_reason`, `authorised_by` (non-null), `requested_at`, `settled_at`.
- **`CreditNoteApplication`** — `id`, `credit_note_id`, `invoice_id` (the *later* invoice it reduces), `amount_minor`, `applied_at`. Many rows per credit note, so a partially-consumed credit carries forward.

Lifecycles, written as explicit state machines with the allowed transitions and the actor for each: credit note `draft → issued → (settled | void)` with `draft → cancelled`; refund `pending → succeeded | failed`, and a failed refund flowing back into carried credit per D-004.

Validation rules, each traced to its FR or constraint: the invoice must be `issued` or `paid` (never `draft`, never `void`); currency equals the invoice's; cumulative per-line credit ≤ that invoice line's total less already-credited; cumulative credit-note total ≤ invoice total; `reason_note` required when the reason is `price_correction`; a void requires a reason; every create/issue/void/refund writes an `audit_events` row in the same transaction with subject `credit_note:cdn_…` (AX-008).

Derived invoice figures, stated as arithmetic so the integrator and the implementer read the same definition: `amount_credited` = sum of non-void credit notes against this invoice; `amount_outstanding` = `total − amount_paid − amount_credited`, floored at zero; and kept distinct from `credits_applied` = sum of `CreditNoteApplication` rows *pointing at* this invoice from earlier ones. Those two are the single most confusable pair in the feature and I name them apart deliberately.

Sensitivity: every attribute classified. Amounts, line descriptions, `reason_note`, and the invoice/supplier/retailer linkage are Confidential commercial-relationship data — excluded from logs, exports outside the allowlist, and error bodies; `reason_note` is free text and therefore the highest leak risk, so it is explicitly named in the redaction list and must never be echoed in a Problem `detail`. Retention: financial record, seven years from the end of its tax year, skipped by the nightly purge. Access: supplier that issued it and the retailer it is against; drafts are supplier-only.

## Phase 4 — API contract → `pallet/specs/credit-notes/contracts/api.yaml`, plus a merge note

I would author a standalone OpenAPI 3.1 document in the feature folder that reuses the live document's conventions, component names, and `Problem`/`NotFound` shapes verbatim, and a `contract-merge.md` listing every addition and every modification to `pallet/contracts/openapi.yaml` line by line. **I would not edit the live contract in this pass** — it is published, an external integrator builds against it, and it carries the `amount_outstanding` redefinition that is still waiting on the C4 confirmation. The merge, with the bump to 2.15.0 and an integrator notice, is the next step once C1/C2/C4 are ruled.

Endpoints, each traced to the user action it serves:

- `POST /credit-notes` (draft; `Idempotency-Key` required) — 201; 404 invoice not visible; 409 invoice is draft or void; 422 line exceeds uncredited remainder / total exceeds invoice total / currency mismatch.
- `GET /credit-notes` — list with `limit`/`cursor`/`next_cursor`, filters `invoice_id`, `status`; drafts only for the issuing supplier.
- `GET /credit-notes/{credit_note_id}` — 200 / 404.
- `PATCH /credit-notes/{credit_note_id}` — drafts only; 409 once issued, with a problem type that names void as the route instead.
- `POST /credit-notes/{credit_note_id}/issue` — the explicit issue step FR-002 demands; 409 if not draft.
- `POST /credit-notes/{credit_note_id}/cancel` — drafts only.
- `POST /credit-notes/{credit_note_id}/void` — issued only, reason required.
- `GET /credit-notes/{credit_note_id}/document` — mirrors the invoice document endpoint exactly, including the 202-while-rendering and `Retry-After` behaviour (G4).
- `GET /invoices/{invoice_id}/credit-notes` — the per-invoice view FR-003 needs for both parties.
- Modification to the existing `Invoice` schema: `amount_credited` and `credits_applied` added, `amount_outstanding` redefined, and the description spelling out the arithmetic.

Error catalogue: every problem type named under `https://pallet.dev/problems/` with a specific, actionable slug — `invoice-not-creditable`, `credit-exceeds-invoice-total`, `line-credit-exceeds-remainder`, `credit-note-already-issued`, `credit-note-not-draft`, `credit-note-already-void`, `currency-mismatch`, `refund-window-expired`, `refund-failed`. Details stay free of amounts tied to counterparties and free of `reason_note`.

Integration boundaries documented on the endpoints and states they surface, with failure modes rather than optimism: Stripe refund latency 5–10 days (so `refunded` is not immediate and the integrator must not treat issue as settlement), the 180-day refusal and what the credit becomes instead, the Bacs `refund.failed` path, Xero's hourly lag and its create/allocate partial failure, and the documents pipeline's three-failure `failed` terminal state.

## Phase 5 — One worked example thread (the part the integrator actually consumes)

Because the integrator builds against the examples, I would make every example hang off the invoice example already in the live contract — `inv_01J8Q2V7X0P3S9Z6M1K4H2R5T8`, 18640 GBP, four sourdough cases at 2880 and two seeded rye at 3560 — and tell one story: one sourdough case short (2880) and the rye damaged as a part-amount credit, giving a credit note whose `total_minor` and whose parent invoice's recomputed `amount_credited` / `amount_outstanding` add up correctly in minor units. I would hand-check the arithmetic of every example and additionally provide a void example, a refund-pending example on a paid invoice, and an application example showing a remainder carried to a later invoice — the three shapes an integrator will otherwise guess at. Realistic descriptions and ULIDs throughout; no `string`, no `foo`.

## Phase 6 — PDF design (G4, P3, the lightest touch that satisfies it)

Recorded in the data model doc rather than as a separate artifact: add `credit_note` to `DocumentKind`, one template at `src/documents/templates/credit-note.hbs` whose fields mirror `invoice.hbs` (credit note id, the invoice it credits, supplier and retailer names, lines with quantity and amount, reason, total, and a prominent void marker when voided), and one loader registered in `render.service.ts`'s `loaders` map. No new relation on `Document` — `subject_id` is already used polymorphically for statements, and adding a second relation on that one unique column is the kind of schema contortion that buys nothing. I would note that a voided credit note's PDF must be re-rendered or marked, and that a PDF already delivered to a retailer cannot be recalled — a point for Tom.

## Phase 7 — Verification I would run, and what I expect

No code ships in this pass, so the checks are on the artifacts:

- Lint the authored OpenAPI (`redocly lint` / `spectral`) — expect zero errors, and in particular every `example` validating against its schema, since a malformed example is the one defect that reaches the integrator directly.
- Re-read the contract with the integrator's eyes, tracing each of FR-001…FR-006 to an endpoint and each endpoint back to a user story — expect no orphan in either direction, and I would state plainly in the report if any FR lacks a surface.
- Assert by inspection that the diff adds no `InvoiceStatus` member, so the two in-repo exhaustiveness guards and the front-end badge still compile.
- Name the tests implementation must carry, so they are not discovered later: cumulative per-line and per-invoice credit caps; issue against draft and void invoices rejected; edit-after-issue rejected; currency mismatch rejected; void reverses the invoice's credited amount; refund failure converts to carried credit; export idempotent across a create-succeeded/allocate-failed replay; and a contract-suite assertion that no credit-note error body contains an e-mail, an account number, or `reason_note`. Integration tests against real PostgreSQL, and coverage must not drop below 75%.

## Phase 8 — Architecture and traceability upkeep

I would draft, not commit, the as-built and drift updates the feature implies: AX-003 gains credit-note refunds, AX-004 gains credit notes in the export set with the two-call allocation shape, AX-007 gains a third document kind. The measurable targets I would attach to those rows rather than invent a separate file: a credit note reaches Xero within two hours of issue (one hourly cycle plus a retry), a refund is requested with Stripe within five minutes of issue, the PDF is available within two minutes. The spine in this workspace is an excerpt, so I would flag that the real rows need the same edit.

## What I would report at the end

The four gaps and how each is closed or why not: G1 and G2 closed by the contract and data model; G3 closed by the explicit ruling that credit may exceed outstanding but not the invoice total, with disposition derived; G4 closed as a template-plus-loader addition to the existing pipeline. Then the things I will not decide alone, in priority order: the Adjustment name collision, cancel-after-issue versus void against the immutability principle, and the `amount_outstanding` redefinition with its live-contract bump — each with the default I designed under and the rework cost if the ruling goes the other way. Then the refusals: decimal money, a new invoice status, a hard delete of an issued record, and any edit of the published contract before the rulings land. Then the discovered gaps outside my brief: monthly statements overstating what a retailer owes, the retailer app's invoice badge and any consumer of `amount_outstanding` living in repos I could not inspect, goodwill credit left out of scope, and the absence of any tax treatment on invoices carrying straight through to credit notes. No separate integration guide: the external systems here are pre-existing and the credit-note lifecycle narrative the integrator needs lives in the contract description — I would record that omission deliberately rather than leave it unexplained.