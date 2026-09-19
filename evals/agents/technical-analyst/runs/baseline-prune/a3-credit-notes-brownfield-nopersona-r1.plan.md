FILES-READ: `pallet/specs/credit-notes/design-brief.md`, `pallet/specs/credit-notes/spec.md`, `pallet/CLAUDE.md`, `pallet/README.md`, `pallet/architecture/spine.md`, `pallet/contracts/openapi.yaml`, `pallet/prisma/schema.prisma`, `pallet/src/accounting/xero-export.ts`, `pallet/src/documents/render.service.ts`, `pallet/src/documents/templates/invoice.hbs`, `pallet/src/documents/templates/statement.hbs`, `pallet/src/statements/statement.service.ts`

---

# Action plan — FEAT-015 credit notes: data model + API contract

Scope is the four gaps in the brief (G1 contract, G2 data model, G3 the exceed-outstanding question, G4 PDF design). Everything else in the spec is treated as already sufficient. This is a design card: I produce schema, contract, and a written design record — not services, controllers, or migrations.

## Phase 0 — Reading, already done, and what it settled

Read all twelve workspace files (above). Four things in the workspace decide most of the design before I write anything:

- `openapi.yaml` already ships an `Adjustment` resource at `POST /orders/{order_id}/adjustments` with the `adj_` id prefix, for door-side delivery quantity changes *before* invoicing. The spec author asks for the new record to be called "Adjustment" in the API. That is a direct collision.
- The contract's stated convention is `{ amount_minor, currency }` integers, and every example uses it. The spec author asks for decimal pounds.
- `xero-export.ts` and `statement.service.ts` both carry exhaustiveness switches over `InvoiceStatus`, and the comment says the retailer app's badge component — a separate repo — carries a third. Any new `InvoiceStatus` value is a cross-repo break.
- `xero-export.ts` notes Xero has a first-class CreditNotes + Allocations API that nothing uses yet, which is the export shape for FR-005.

## Phase 1 — Resolve the three conflicts before designing anything

These are conflicts between the signed-off spec and the live product. I settle each with a stated default and carry on; I do not block.

**1a. The name.** The spec author's "keep the name Adjustment in the API" cannot be honoured as written: `Adjustment`, `AdjustmentRequest`, `adj_`, and `POST /orders/{id}/adjustments` are live, and the accounting integrator is building against exactly these examples. Two records both called "adjustment", one pre-invoice and one post-invoice, both reducing what a retailer owes, is a defect factory for an external consumer.

- **Stop point:** confirm with Priya (spec owner) that the API resource is `credit-notes` / `CreditNote` / `cn_` while the supplier app's user-facing copy stays "Adjustment". Nothing about the supplier research is lost — the word lives in the UI, not the wire.
- **If Priya insists on `Adjustment` on the wire:** I would put it at `/invoices/{invoice_id}/adjustments` (nested under invoice, never under order), name the schemas `InvoiceAdjustment`/`InvoiceAdjustmentRequest`, use a separate id prefix (`iadj_`, since `adj_` is taken), and add a disambiguation paragraph at the top of the contract description. I would tell Priya in writing that the integrator will confuse the two and that I expect support load from it.
- **Default I proceed under:** `credit-notes`, `cn_`, with a one-line note in the contract description saying the supplier app calls these "adjustments" and that `/orders/{id}/adjustments` is a different, pre-invoice record.

**1b. Money format.** The spec author's "pounds with two decimal places" would fork the contract's money convention for one resource. I keep `Money` with `amount_minor`, matching `Invoice`, `Payment`, and the integer minor units in the schema. I flag to Priya that the supplier app formats at display time, and that mixing `12.50` and `1250` across resources is exactly how a float rounding bug reaches a supplier's payout. This one I decide myself rather than escalate — it is a contract-wide convention, and one feature does not get to break it. It goes in the design record as a noted deviation from the spec's author notes.

**1c. Cancel after issue.** The spec author wants post-issue cancellation with the retailer seeing it "disappear". The operating manual says financial records are immutable once issued, corrections are new documents never edits, records are retained seven years, and triggers reject `UPDATE` on issued rows. Xero voids an authorised credit note, it does not delete it.

- **Design:** post-issue cancellation is a **void**, not a delete. Status moves `issued → void`, with `voided_at`, `voided_by`, `void_reason`; the record stays queryable, stops counting toward the invoice's credited amount, and exports to Xero as a voided credit note. Both parties see it as voided. Voiding is refused once the credit has been refunded or applied to another invoice — at that point the correction is a new document in the other direction.
- **Stop point:** Priya and Tom confirm "disappears from the retailer's list" is satisfied by "shows as void / is filtered out of the default list view" rather than by deletion. **If they insist on true deletion:** I would refuse to design it and say so plainly — it contradicts the retention principle and the audit-trail row AX-008 — and offer default-filtered lists plus a `status=void` filter as the closest thing.
- I also note for the implementing engineer that the immutability trigger needs an explicit allow-list for the columns that legitimately change after issue (`status`, `voided_*`, and on `Invoice` the existing `amount_paid_minor`), and that this list should be written into the migration, not discovered.

## Phase 2 — G3: decide whether a credit note may exceed the outstanding amount

This is the spec's open question, left to design, so I answer it rather than punt.

**Ruling I would write:** the cap is the invoice, not the outstanding balance.

- Per line: credited quantity ≤ the invoice line's quantity, and the credited amount ≤ that line's `line_total_minor`, net of credits already issued against that line.
- Per note: total ≤ invoice total, net of the totals of all non-void issued credit notes on that invoice.
- Exceeding the *outstanding* amount is explicitly allowed and is the normal case for a paid invoice — that is precisely FR-004's refund path. Tying the cap to outstanding would make validity depend on payment settlement timing, so the same request could pass at 09:00 and fail at 09:05 when a Bacs payment settles.
- Goodwill beyond the invoice total is **out of scope**: there is no invoice line to attach it to, so Xero has nothing to allocate against, and what is actually wanted is a discount on a future invoice. I flag this to Priya as a separate feature rather than silently absorbing it.
- Over-cap requests return `422` with problem type `https://pallet.dev/problems/credit-exceeds-invoice`, whose `detail` names amounts and the invoice id only — no retailer contact detail, per the confidentiality rule.

## Phase 3 — G2: the data model

**Write:** `pallet/prisma/schema.prisma` (append new models and enums; edit `DocumentKind` and `Invoice`).

- `enum CreditNoteStatus { draft issued void }` — deliberately a *separate* enum. I add no value to `InvoiceStatus`, because that would break the two exhaustiveness switches in `xero-export.ts` and `statement.service.ts` and the badge component in the retailer app repo I cannot see or change.
- `enum CreditNoteReason { short_delivery damaged price_correction other }` plus a nullable `reason_note` free-text field (FR-001's "fixed list plus free text").
- `enum CreditNoteDisposition { apply refund }`.
- `model CreditNote` — `cn_` id; `invoice_id`, `supplier_id`, `retailer_id`, `currency` (must equal the invoice's), `status`, `reason`, `reason_note`, `total_minor` (positive magnitude, derived from lines), `disposition` (nullable, set at issue), `created_at`/`created_by`, `issued_at`/`issued_by`, `voided_at`/`voided_by`/`void_reason`; relations to lines, applications, refund, document; indexes on `invoice_id` and `(supplier_id, issued_at)` for SC-002's "issued within two working days" measurement.
- `model CreditNoteLine` — `cnl_` id; `credit_note_id`, `invoice_line_id`, `description`, nullable `quantity` and `unit_price_minor` (null for an amount-only price correction), `line_total_minor`.
- `model CreditNoteApplication` — `cna_` id; one-to-**many**, not one-to-one, so a credit can be split between the originating invoice's outstanding balance and a later invoice. Fields: `target_invoice_id` (nullable = applied to its own invoice), `amount_minor`, `applied_at`, `authorised_by` non-null.
- `model CreditNoteRefund` — `cnr_` id; `payment_id`, `stripe_refund_id`, `status` (`pending`/`succeeded`/`failed`), `failure_code`, `amount_minor`, `requested_at`, `settled_at`, `authorised_by` non-null. `authorised_by` is mandatory on both of these because money moving without a named authority is the one thing the manual is unambiguous about.
- `DocumentKind` gains `credit_note`. `Document.subject_id` is unique and already polymorphic-by-prefix, so a `cn_` subject fits; I add the optional back-relation and note that the existing `invoice Invoice?` relation on `Document` means a second optional relation is needed rather than a reused one.
- `Invoice` gains no status value and no stored credit column — credited and applied amounts are derived by sum, so there is no denormalised field to drift.

**How apply-vs-refund resolves (the "what happens to the credit" half of G2):** a credit first reduces its own invoice's outstanding amount (FR-003); any remainder either carries to the retailer's next invoice from that supplier or is refunded (FR-004). Refund is only offerable when the invoice has a `succeeded` card payment and the charge is inside Stripe's 180-day window (spine AX-003); a Bacs refund is a bank transfer that can fail, so `refund.failed` flips the refund row to `failed` and the credit falls back to `apply` rather than vanishing. **Open question I would put to Tom and Priya:** does the *supplier* choose the disposition at issue, or does the *retailer* choose on receipt? US-002 is written from the retailer's side. **Default I design under:** supplier proposes at issue, `apply` is the default, `refund` is only accepted when a refundable payment exists; a retailer-chooses flow can be layered on later without a schema change since `disposition` is nullable until settled.

## Phase 4 — G1: the API contract

**Write:** `pallet/contracts/openapi.yaml`. Version `2.14.0 → 2.15.0`.

Paths:

| Method + path | Purpose | Notable responses |
|---|---|---|
| `POST /invoices/{invoice_id}/credit-notes` | create draft, `Idempotency-Key` required | 201; 409 `invoice-not-issued`; 422 `credit-exceeds-invoice` |
| `GET /invoices/{invoice_id}/credit-notes` | list against one invoice | 200 paginated |
| `GET /credit-notes` | list, filters `status`, `invoice_id`, `issued_after` | 200 paginated `data` + `next_cursor` |
| `GET /credit-notes/{credit_note_id}` | one credit note | 200 / 404 |
| `PATCH /credit-notes/{credit_note_id}` | edit a draft (FR-002) | 200; 409 `credit-note-not-draft` |
| `DELETE /credit-notes/{credit_note_id}` | cancel a draft, nothing recorded | 204; 409 `credit-note-not-draft` |
| `POST /credit-notes/{credit_note_id}/issue` | issue, carries `disposition` | 200; 409 already issued; 422 `refund-not-available` |
| `POST /credit-notes/{credit_note_id}/void` | post-issue cancel (Phase 1c) | 200; 409 `credit-note-already-settled` |
| `GET /credit-notes/{credit_note_id}/document` | the PDF, generic documents shape | 200 `application/pdf`; 202 while rendering; 404 |

Schemas: `CreditNote`, `CreditNoteLine`, `CreditNoteRequest`, `CreditNoteLineRequest`, `CreditNoteIssueRequest`, `CreditNoteStatus`, `CreditNoteReason`, `CreditNoteDisposition`, `CreditNoteApplication`, `CreditNoteRefund`. All money via the existing `Money` ref; snake_case; RFC 3339 UTC; every error an existing-shape `Problem`.

New problem types under `https://pallet.dev/problems/`: `invoice-not-issued`, `credit-note-not-draft`, `credit-note-already-issued`, `credit-note-already-settled`, `credit-exceeds-invoice`, `refund-not-available`, `currency-mismatch`. Cross-tenant access returns the existing `NotFound`, never 403 — the existing wording is "no such record visible to the caller", and 403 would leak that an invoice exists.

**Change to `Invoice` (the second half of G1 — exposing the credited amount):** add `amount_credited` to properties and `required`, and make `amount_outstanding` **net of credits applied to that invoice**, as FR-003 requires. This is the one genuinely risky edit on the card, because it silently changes a number the accounting integrator is already consuming.

- **Stop point:** Tom and the integrator's contact confirm the semantic change. **If they want zero behavioural change:** add `amount_outstanding_net` alongside, leave `amount_outstanding` gross, and tell Priya FR-003 is then satisfied by a differently-named field. **Default I proceed under:** net, because "what the retailer owes" is the honest reading of outstanding, plus a `**Behaviour change in 2.15.0**` paragraph at the top of the contract description and an explicit note that the integrator should re-check any reconciliation keyed on `amount_outstanding`. I would not ship this without that heads-up going out.

**Examples are the deliverable here**, since the brief says the integrator builds against them. I write worked examples that are arithmetically consistent with the existing invoice example (`inv_01J8Q2V7X0P3S9Z6M1K4H2R5T8`, total 18640, two lines at 11520 and 7120): a two-case short delivery on the sourdough line credited at 5760, giving `amount_credited: 5760` and `amount_outstanding: 12880`. Ids use the established prefixed-ULID shape. No real contact details anywhere in examples or error bodies.

## Phase 5 — G4: the credit-note PDF design (P3)

**Write:** the design section in the design record. Adding a kind, per `render.service.ts`, is exactly three things: the `DocumentKind.credit_note` enum value (Phase 3), a `credit-note.hbs` template, and a subject loader registered in `loaders`. I specify the template markup — header with credit-note id and the invoice it credits, supplier→retailer line, the credited lines with quantity and amount, reason, total, and a footer line stating whether it was applied or refunded — modelled on the existing `invoice.hbs`, using the same `{{money}}` helper.

I do **not** create `src/documents/templates/credit-note.hbs` as a file on this card. A template sitting in that directory with no registered `DocumentKind` and no loader is dead code that looks live; it belongs with the implementation. I state that explicitly rather than leaving the omission to be discovered.

## Phase 6 — Knock-on surfaces the brief did not list

The brief says other surfaces are unchanged "except where a gap above requires it". Two are required and I would flag both rather than quietly change or quietly skip them:

- `statement.service.ts` sums outstanding invoices, and `statement.hbs` prints `amount_outstanding` per invoice. Once outstanding is net of credits, monthly statements change. I note this as a required follow-up with a pointer to both files; it is implementation, not this card's design.
- FR-005's Xero export needs a new push path (`POST /CreditNotes` then `PUT /CreditNotes/{id}/Allocations`), which `xero-export.ts` says exists but is unused. I design the mapping — issued → `AUTHORISED`, void → `VOIDED`, application → an allocation against the target invoice, refund → a refund on the credit note — and note the job id must be deterministic (`accounting.export:credit_note:<id>`) per the idempotent-jobs rule, and that credit notes add roughly one to two Xero calls each against the 60/min, 5,000/day ceiling.
- **Unknown I cannot close from this excerpt:** how the exporter records that a document has already been pushed. Neither `Invoice` nor the excerpt of `src/accounting/` shows an `exported_at` column or export-state table beyond `accounting_export_failures`. I would not invent a column; I would read the full `src/accounting/` in the real repo and mirror whatever invoices do, and I flag this as an open item in the design record rather than guessing.

## Phase 7 — Record the decisions

**Write:** `pallet/specs/credit-notes/design.md` — the data model, the endpoint table, the G3 ruling with its reasoning, the apply/refund rules, the PDF design, the three spec deviations from Phase 1, and the open items. **Write:** `pallet/architecture/spine.md` — amendments to AX-003 (refunds now also originate from credit notes), AX-004 (the export set gains credit notes), and AX-007 (the document kinds gain credit note), since spine.md is where this codebase records rulings.

## Phase 8 — Verification, honestly bounded

- I would validate the OpenAPI document structurally (parse + `$ref` resolution + Spectral if the toolchain has it) and check every example against its schema by hand — the examples are what the integrator copies, so an inconsistent one is a real defect.
- **What I would not claim:** this workspace has no `package.json`, no test suite, and no database, so I cannot run a contract suite, a migration, or `prisma validate`. I would say that plainly in the final report rather than implying the schema compiles.
- I would **specify** the tests for the implementing engineer to write, and say what each should show: credit capped at invoice total not outstanding (422 with `credit-exceeds-invoice`); a credit on a fully paid invoice succeeds and produces a refund, not an error; `amount_outstanding` net and `amount_credited` correct after issue; a voided credit note stops counting toward `amount_credited` and stays retrievable; `PATCH`/`DELETE` on an issued note return 409; void refused after settlement; no counterparty e-mail or bank detail in any of the new problem bodies; export job re-run changes nothing. Integration tests against real PostgreSQL, per the manual, and coverage staying at or above 75%.

## Delegation

None. I would do this card myself — it is three coupled files whose consistency (ids, money shape, status enums, worked examples) is the whole deliverable, and splitting it across workers costs more in reconciliation than it saves. I would not spawn agents or subagents for it.

## What I would report at the end

The four gaps closed and where each landed (`schema.prisma`, `openapi.yaml`, `specs/credit-notes/design.md`, `architecture/spine.md`); the G3 ruling and its reasoning in two lines; the three places I did not follow the signed-off spec and why (the Adjustment name collision, decimal money, delete-vs-void after issue), each needing Priya's or Tom's nod; the `amount_outstanding` semantic change and the fact that the integrator needs telling before 2.15.0 ships; the follow-ups I deliberately left out of scope (statement service and template, Xero export implementation, the `credit-note.hbs` file, goodwill credits beyond invoice total); the one open item I could not resolve from the excerpt (how export state is recorded); and that nothing was executed or tested here.