FILES-READ: `pallet/specs/credit-notes/design-brief.md`, `pallet/specs/credit-notes/spec.md`, `pallet/CLAUDE.md`, `pallet/README.md`, `pallet/prisma/schema.prisma`, `pallet/architecture/spine.md`, `pallet/contracts/openapi.yaml`, `pallet/src/accounting/xero-export.ts`, `pallet/src/statements/statement.service.ts`, `pallet/src/documents/render.service.ts`, `pallet/src/documents/templates/invoice.hbs`, `pallet/src/documents/templates/statement.hbs`

---

# Action plan — FEAT-015 credit notes: data model + API contract

Deliverable is design, not implementation: schema models, the OpenAPI contract with examples the accounting integrator can build against, the G3 ruling, and the PDF design. Runtime code (services, jobs, Stripe calls, Xero push) is enumerated as follow-on tickets, not written here.

## Phase 0 — Reconcile the spec against the live product (reading only)

Already done in the reads above. Six collisions between the spec and what is actually shipped drive the whole design; I resolve each explicitly rather than letting it surface at implementation time.

| # | Collision | Where |
|---|---|---|
| C1 | Spec author says "call it an **Adjustment** in the API". `Adjustment` already exists — a pre-invoice delivery quantity note, `adj_` ids, live at `POST /orders/{order_id}/adjustments` | `schema.prisma:69-77`, `openapi.yaml:47-64`, `115-132` |
| C2 | Spec author says amounts in pounds, two decimals (`12.50`). The contract says money is `{amount_minor, currency}`, "never a decimal"; the schema stores integer minor units | `spec.md:68-69` vs `openapi.yaml:8`, `67-72` |
| C3 | Spec author says a supplier can cancel a credit note **after** issue and "the retailer will see it disappear". Governance: issued financial records are immutable, corrections are new documents, seven-year retention, DB triggers reject `UPDATE` on issued rows | `spec.md:70-71` vs `CLAUDE.md:12-17` |
| C4 | FR-003 wants the invoice's outstanding amount net of credits. `amount_outstanding` is already a published field the integrator reads | `spec.md:48-49` vs `openapi.yaml:89`, `101` |
| C5 | Any new `InvoiceStatus` value breaks two compile-time exhaustiveness guards here **and** one in the retailer app, a separate repo outside this change | `xero-export.ts:22-28`, `statement.service.ts:15-18`, and the note at `xero-export.ts:24-25` |
| C6 | `Document.subject_id` carries an FK relation to `Invoice`, yet `statement` is already a `DocumentKind`. Adding `credit_note` subjects makes the polymorphic-subject question unavoidable | `schema.prisma:84-92` |

## Phase 1 — Write the decisions register, then stop for two confirmations

Write `pallet/specs/credit-notes/design-decisions.md`: one row per decision, each with the ruling, the reason, what it costs, and who owns the sign-off. This is the artefact I would put in front of Priya and Tom.

**D1 — Resource name.** *Stop point.* The credit note is named `credit_note`, path `/credit-notes`, id prefix `crn_`. Reusing "Adjustment" would put two unrelated financial concepts behind one word in a contract the integrator already consumes, and `adj_` ids are taken. I would confirm with Priya that the supplier app is free to keep the word "Adjustment" in its UI copy while the API uses `credit_note`.
- If they accept: proceed as planned.
- If they insist on "Adjustment" in the API: the only non-broken shape is a second, nested resource `/invoices/{id}/adjustments` with a distinct id prefix (`iadj_`), and I would say plainly that the integrator will need a written note distinguishing the two. I would still not reuse `adj_`.
- Default while unconfirmed: `credit_note` / `crn_`.

**D2 — Money stays in minor units.** Flag, not a stop. `{ "amount_minor": 1250, "currency": "GBP" }`, not `12.50`. Decimal money in JSON invites float rounding, and the contract's own convention line and every existing example would have to be broken for one feature. The supplier app formats for display. Recorded in the register with the spec line it overrides so Priya sees it was not missed.

**D3 — Cancel after issue becomes void, not disappear.** *Stop point,* Priya and Tom together, because it touches a ratified principle.
- Draft: edit and cancel freely; nothing is recorded against the invoice (satisfies FR-002 and the second scenario of US-001).
- Issued and not yet consumed (no application, no refund started): `POST /credit-notes/{id}/void`. The record stays visible to the retailer as `void`, and Xero gets a matching void. It does not vanish — a seven-year-retained document that disappears from the retailer's view is exactly what the immutability principle forbids, and Tom would find a Xero credit note with no Pallet counterpart.
- Issued and consumed: refused, `409`. The correction is a new document.
- If Priya rules that it must literally disappear from the retailer view: I would implement visibility filtering only (`void` credit notes hidden from retailer list responses) while the row, the audit trail, and the Xero void all persist, and note that as a deliberate divergence. I would not delete rows.
- Default while unconfirmed: void, visible as `void` to both parties.

**D4 — No new `InvoiceStatus` value.** Credited state is exposed as fields, not status. A `credited` enum value would break the switch in `xero-export.ts`, the switch in `statement.service.ts`, and the retailer app's badge component in a repo this change cannot touch — and Xero has no such invoice state (`xero-export.ts:10`). Both existing guards therefore compile unchanged, which is my verification for this decision.

**D5 — `amount_outstanding` is redefined as total − paid − credited; `amount_credited` is added.** This is what FR-003 asks for. It is a semantic change to a published field, but today no invoice has credits, so on launch day every existing response is byte-identical. Contract version goes `2.14.0` → `2.15.0` with an explicit changelog entry, and I would ask Priya to send the integrator that entry before release rather than let them discover it. If they would rather not touch the field's meaning, the fallback is an additive `amount_outstanding_net_of_credits` and leaving `amount_outstanding` gross — I would recommend against it because it leaves the statement pipeline reading the wrong number.

**D6 — G3 ruling: cumulative credits may exceed *outstanding* but not the invoice *total*.** Exceeding outstanding is the whole of US-002 (a paid invoice credited and refunded). Exceeding the invoice total is refused, `422`, per line and in aggregate: a credit with no invoice basis has nothing for Xero to allocate against, and goodwill beyond the invoiced amount is a different instrument the spec does not describe. Tom signs this off as the finance owner; I would send them the two-line version with the refund case spelled out. If Tom wants goodwill in scope, it is a spec change, not a design change, and I would say so rather than smuggle it in.

**D7 — Settlement.** `apply_to_next_invoice` or `refund`. Refund is offered only when the invoice has a `succeeded` payment; per AX-003 (`spine.md:25-29`) card refunds take 5–10 days and Stripe refuses beyond 180 days from the charge, and Bacs refunds can fail via `refund.failed`. The contract exposes a read-only `refund_available` boolean so the supplier app knows before it offers the choice, and `422 refund-not-available` if it is attempted anyway.

## Phase 2 — Data model (G2)

Edit `pallet/prisma/schema.prisma` and write `pallet/specs/credit-notes/data-model.md` (entity narrative, lifecycle diagram in text, invariants, retention note).

New enums: `CreditNoteStatus { draft issued void }`, `CreditNoteReason { short_delivery damaged_goods price_correction other }`, `CreditSettlement { apply_to_next_invoice refund }`.

New models:
- **`CreditNote`** (`crn_`) — `invoice_id`, `supplier_id`, `retailer_id`, `status`, `reason`, `reason_note` (required when reason is `other`), `settlement`, `currency`, `total_minor`, `issued_at?`, `voided_at?`, `created_by`, `issued_by?`, `created_at`; relations to lines, applications, refunds, invoice. Indexes on `invoice_id` and `(retailer_id, supplier_id, status)` — the second serves "apply to the retailer's next invoice from that supplier" (FR-004).
- **`CreditNoteLine`** (`crl_`) — `credit_note_id`, `invoice_line_id`, `description`, `quantity?`, `unit_price_minor?`, `line_total_minor`. Quantity is nullable because FR-001 allows "a quantity **or** an amount per line"; a price correction is amount-only. Amounts are stored positive; the sign is carried by the document type, matching how Xero models credit notes.
- **`CreditApplication`** (`cap_`) — `credit_note_id`, `invoice_id`, `amount_minor`, `applied_at`, unique on `(credit_note_id, invoice_id)`. One row per consumption, so partial application across invoices works and the remaining balance is derivable rather than stored on an immutable row.
- **`Refund`** (`ref_`) — `credit_note_id`, `payment_id`, `stripe_refund_id?`, `status`, `amount_minor`, `authorised_by` (non-null, per the money-moves-on-authority principle), `failure_reason?`, `created_at`, `settled_at?`. A refund cannot be modelled as a `Payment` row: `Payment.invoice_id` is `@unique` (`schema.prisma:57`).

Changed:
- `Invoice` gains back-relations `credit_notes` and `credits_applied`. **No stored `amount_credited_minor` column** — it is the sum of issued (not void) credit notes on the invoice, computed in the read path. A denormalised counter would need a trigger writing to rows the immutability trigger protects.
- The prefix comment at `schema.prisma:2` gains `crn_`, `crl_`, `cap_`, `ref_`.
- `DocumentKind` gains `credit_note` (Phase 4).

Invariants written into `data-model.md` and destined for constraints/service checks: credit note currency equals invoice currency; invoice must be `issued` or `paid` (never `draft` or `void`); per line, cumulative credit ≤ that invoice line's `line_total_minor`; per invoice, cumulative issued credit ≤ `total_minor` (D6); `sum(applications) + sum(succeeded refunds) ≤ credit note total`; every state change writes an `AuditEvent` with subject `credit_note:crn_…` in the same transaction (AX-008); credit notes are financial records — the nightly purge skips them, seven-year retention.

**Flag I would raise here, not silently resolve:** the immutability trigger's exact allowance. The workspace has no migration SQL, only the schema excerpt, so I cannot see whether the trigger permits the `issued → paid/void` transitions `Invoice` clearly relies on. I would design `CreditNote.status` to mirror `Invoice.status` exactly on the assumption that the same transitions are permitted, state that assumption in `data-model.md`, and put "read the trigger definition and confirm before the migration is written" as the first implementation ticket.

## Phase 3 — API contract (G1)

Edit `pallet/contracts/openapi.yaml`. Bump `info.version` to `2.15.0`, add `crn_`/`crl_`/`cap_`/`ref_` to the conventions preamble, and add a short changelog block naming the `amount_outstanding` redefinition (D5).

Paths:
| Method + path | Notes |
|---|---|
| `POST /invoices/{invoice_id}/credit-notes` | Creates a draft. `Idempotency-Key` required. `201`. `409 invoice-not-issued`, `422 credit-exceeds-line-total`, `422 credit-exceeds-invoice-total`, `422 currency-mismatch` |
| `GET /credit-notes` | Filters `invoice_id`, `status`, `supplier_id`; `limit`/`cursor`/`next_cursor` as everywhere else. One list endpoint with a filter rather than a second nested path |
| `GET /credit-notes/{credit_note_id}` | Both parties; `404` for anyone else, matching the existing "no such record visible to the caller" convention rather than a `403` that leaks existence |
| `PATCH /credit-notes/{credit_note_id}` | Draft only; `lines` replaced wholesale when present. `409 credit-note-not-draft` |
| `DELETE /credit-notes/{credit_note_id}` | Cancels a draft, `204`. `409 credit-note-not-draft` once issued |
| `POST /credit-notes/{credit_note_id}/issue` | `Idempotency-Key` required — it is the point money starts moving. `200`. `422 refund-not-available`, `422 credit-exceeds-invoice-total` |
| `POST /credit-notes/{credit_note_id}/void` | D3. `200`. `409 credit-note-already-settled` |
| `GET /credit-notes/{credit_note_id}/document` | `200 application/pdf` / `202` with `Retry-After`, identical in shape to `/invoices/{id}/document` |

Schemas: `CreditNote`, `CreditNoteLine`, `CreditNoteRequest`, `CreditNoteLineRequest`, `CreditApplication`, `Refund`, plus `CreditNoteStatus`, `CreditNoteReason`, `CreditSettlement`. `CreditNote` carries `remaining` (Money), `refund_available` (boolean), `applications[]`, and `refund?` so the integrator can see where the credit went without a second call. `Money` is reused unchanged. `Invoice` gains `amount_credited` and its `required` list.

**Examples — this is the part the integrator builds against, so the arithmetic ties out across every example in the file.** I reuse the existing invoice `inv_01J8Q2V7X0P3S9Z6M1K4H2R5T8` (`openapi.yaml:92-104`): one of the four sourdough cases arrives short, so the credit note credits `inl_01J8Q2V7X1A2B3C4D5E6F7G8H9` for quantity 1 at 2880 → credit note total `2880`, reason `short_delivery`, settlement `apply_to_next_invoice`. The updated `Invoice` example then reads total `18640`, paid `0`, credited `2880`, outstanding `15760`. A second `CreditNote` example shows the refund branch against a paid invoice with a `Refund` sub-object and `authorised_by` populated. Every `Problem` example carries only `type`/`title`/`status`/`detail`/`correlation_id` with no e-mail address, account number, or counterparty contact detail — the contract suite already asserts this and the new endpoints must not be the exception.

## Phase 4 — Credit-note PDF (G4, P3)

Design only, in `pallet/specs/credit-notes/document.md`, plus the template file and the enum/loader wiring:
- `DocumentKind` gains `credit_note`; template `pallet/src/documents/templates/credit-note.hbs`, modelled on `invoice.hbs`: title and credit note id, supplier → retailer, issued date, the credited lines with quantity and amount, the total credited, the invoice it credits, the reason, and whether it will be applied or refunded.
- `loaders` in `render.service.ts:16-19` is typed `Record<DocumentKind, SubjectLoader>`, so adding the enum value makes that object a compile error until a `credit_note` loader is registered. That is the intended forcing function; I register the loader entry and declare `loadCreditNoteForRender` alongside the existing declarations, leaving the body as implementation work.
- No second PDF path (AX-007 forbids one).
- **Flag, unresolved by design:** `Document.subject_id` has an FK relation to `Invoice` (`schema.prisma:91`) that cannot hold for statement subjects today and will not hold for credit-note subjects. I would write up the two options — drop the FK and keep `subject_id` genuinely polymorphic with a `kind` discriminator, or add nullable per-kind FK columns — recommend the first, and raise it to the team rather than change a shipped model's constraint as a side effect of a P3 item.

## Phase 5 — Close the spec's open question and note the knock-ons

- Edit `spec.md`'s "Open questions" section to record the D6 ruling and point at the decisions register. I would not otherwise rewrite the signed-off spec.
- In the decisions register, note two consequences the brief does not list but that follow from D5, so nobody meets them by surprise: `statement.service.ts` sums outstanding invoices and will now sum credit-netted figures, which is correct but changes the numbers Tom sees on monthly statements; and `xero-export.ts:5-6` records that Xero's `POST /CreditNotes` + `PUT /CreditNotes/{id}/Allocations` resource is unused today, which is exactly the shape FR-005 needs.
- I would **not** edit the `AX-004` / `AX-007` as-built lines in `architecture/spine.md`. Those rows record what is built; this card designs. I would instead list the exact edits those rows need at build time in the register, for the row owner to apply then.

## Phase 6 — Validate

No shell in this run; in the real run:
- `spectral lint contracts/openapi.yaml` and the example-validation script — expect clean, and specifically expect every new `example` to validate against its schema, since a malformed example is what would actually bite the integrator.
- `npx prisma validate` and `prisma migrate diff` as a dry run against the current schema — expect a clean diff containing only additive tables plus the `DocumentKind` enum value.
- `tsc --noEmit` — expect two things: the switches in `xero-export.ts` and `statement.service.ts` compile untouched (proving D4), and `loaders` fails until the `credit_note` entry is added (proving the Phase 4 wiring is complete).
- Extend the existing contract suite's no-confidential-data-in-error-bodies assertion to the new problem types.
- Coverage (75% floor) and integration tests against real PostgreSQL are implementation-phase obligations; I list them in the handoff, I do not claim them here.

## Delegation

None. This is a single coherent design across four coupled files, and splitting it across workers would produce examples that stop tying out and a schema that drifts from the contract. I also would not spawn any worker in this run.

## What I would report at the end

- The four gaps closed and where: G1 in `contracts/openapi.yaml` (eight paths, nine schemas, worked examples on the existing invoice), G2 in `prisma/schema.prisma` + `specs/credit-notes/data-model.md` (four models, three enums, one changed model), G3 ruled in `specs/credit-notes/design-decisions.md` and enforced as two `422`s, G4 in `specs/credit-notes/document.md` + `credit-note.hbs` + the loader registration.
- The three decisions that overrule something written in the spec, each with its reason: API name is `credit_note` not `Adjustment` (C1), money stays in minor units (C2), post-issue cancel becomes void-and-visible not disappear (C3). Priya's call on all three; my defaults hold until they rule.
- Tom's one sign-off: credits may exceed outstanding but not the invoice total.
- The one contract change the integrator must be told about before release: `amount_outstanding` is now net of credits, `amount_credited` is new, version `2.15.0`. No existing response changes until the first credit note is issued.
- Two things I deliberately did not change and why: the `Document.subject_id` FK (a shipped constraint, needs a team decision) and the `AX-004`/`AX-007` as-built lines (this is design, not build).
- One assumption to verify first: that the immutability trigger permits `issued → void` on the new table the way it evidently does for `Invoice`.
- Implementation tickets, out of scope here: credit-note service and lifecycle, next-invoice application, Stripe refund path with `refund.failed` handling, the `accounting.export` credit-note push with a deterministic job id, and the render loader body.