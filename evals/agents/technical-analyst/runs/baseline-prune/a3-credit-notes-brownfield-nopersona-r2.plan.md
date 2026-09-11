FILES-READ: `pallet/specs/credit-notes/design-brief.md`, `pallet/specs/credit-notes/spec.md`, `pallet/CLAUDE.md`, `pallet/README.md`, `pallet/prisma/schema.prisma`, `pallet/architecture/spine.md`, `pallet/contracts/openapi.yaml`, `pallet/src/accounting/xero-export.ts`, `pallet/src/documents/render.service.ts`, `pallet/src/statements/statement.service.ts`, `pallet/src/documents/templates/invoice.hbs`, `pallet/src/documents/templates/statement.hbs`

# Action plan — FEAT-015 credit notes: data model + API contract

Scope is the four gaps in `design-brief.md` (G1 contract, G2 data model, G3 the open question, G4 the PDF). Everything else in the spec is declared sufficient, so I design, I do not implement: no NestJS modules, no migration, no template file, no job code.

## Phase 0 — Reading (done)

Read the eleven files listed above. What drives the design:

- `CLAUDE.md`: issued financial records are immutable (DB triggers reject `UPDATE`), money moves only with a non-null `authorised_by`, counterparty data never in logs/exports/error bodies, background jobs carry a deterministic id, RFC 7807 errors, one datastore.
- `contracts/openapi.yaml`: money is `{amount_minor, currency}` integers, snake_case, prefixed ULIDs, `Idempotency-Key` on creating POSTs, cursor pagination, `NotFound` is worded "visible to the caller" (so out-of-scope records 404 rather than 403).
- `prisma/schema.prisma`: an `Adjustment` model **already exists** — pre-invoice delivery quantity changes on order lines, prefix `adj_`, live at `POST /orders/{id}/adjustments`. `Invoice` has no tax fields and no `amount_outstanding` column (it is derived).
- `spine.md`: AX-003 (Stripe refunds; card refunds refused >180 days after the charge, Bacs refunds can fail via `refund.failed`), AX-004 (hourly Xero export, as-built = invoices, payments, statements), AX-007 (one PDF pipeline, as-built = invoice, statement), AX-008 (audit row in the same transaction).
- `xero-export.ts` + `statement.service.ts`: two `never` exhaustiveness guards over `InvoiceStatus`, plus a third in the retailer app (a different repo) — adding an invoice status is a cross-repo breaking change.
- `render.service.ts`: `loaders` is `Record<DocumentKind, SubjectLoader>`, so a new enum value fails to compile until a loader is registered — the PDF design leans on that.

## Phase 1 — Decisions log, and the four conflicts I would raise

Write `pallet/specs/credit-notes/decisions.md`. Four items, each with the ruling I would proceed under and the branch if the ruling goes the other way. Three of them are conflicts between the spec author's notes and the operating manual, so I raise them together in one message to Priya (cc Tom) rather than trickling them out, and keep designing under my defaults meanwhile.

**D1 — the resource is `CreditNote` (`crn_`), not `Adjustment`.** The spec says "keep the name Adjustment in the API", but `Adjustment`/`adj_`/`POST /orders/{id}/adjustments` is taken by a live, unrelated concept (pre-invoice delivery quantity notes). Two records called Adjustment in one contract is exactly the confusion the accounting integrator would build a bug on. Default: `CreditNote` in the API and the schema; supplier-app copy can keep saying "Adjustment" — that's front-end wording in a separate repo and costs nothing.
*Stop:* Priya confirms. *Branch if they insist on Adjustment in the API:* the existing one gets renamed (`DeliveryAdjustment` / `dadj_`, path `/orders/{id}/delivery-adjustments`), which is a breaking change to a shipped contract consumed by two app repos — that means a major version, a deprecation window, and its own card. I would write that path up but not start it under this card.

**D2 — money stays `{amount_minor, currency}`.** The spec asks for pounds with two decimals (`12.50`). That contradicts the contract convention ("never a decimal"), the `Int` minor-unit columns, and it hands the integrator a JSON number for money — binary-float rounding on the one surface where the books must reconcile. Default: minor units everywhere; the supplier app formats. *Branch if Priya insists:* the compromise I would offer is minor units remaining canonical with an added display **string** (`"12.50"`), never a JSON number, and I would say plainly that I would not ship decimal money.

**D3 — an issued credit note is voided, not cancelled, and never disappears.** The spec says the supplier can cancel after issue and "the retailer will see it disappear". That collides with the immutability principle, seven-year retention, and the fact that the credit note has already been pushed to Xero. Default: draft → `cancel` (nothing was ever recorded, matches the spec's own second acceptance criterion); issued → `void`, a recorded state transition with `voided_at`/`voided_by`/`void_reason`, an audit row, a Xero void, and continued visibility to the retailer marked "void". The retailer's obligation returns to the pre-credit figure; it does not vanish from their history.
*Also to verify:* `schema.prisma` is an excerpt and the immutability triggers are not in this workspace, yet `Invoice` clearly transitions issued → paid/void, so status transitions are evidently permitted. I would read the trigger SQL in the migrations before finalising; I flag that I could not confirm it here. If the trigger really forbids any `UPDATE` on an issued row, the fallback is a separate `CreditNoteVoid` row and status derived from its presence — same semantics, no update.

**D4 (G3) — the ruling on the open question.** A credit note may exceed the invoice's **outstanding** amount; it may not exceed the invoice **total**. Rationale: a fully paid invoice has zero outstanding, and crediting it is precisely the refund case FR-004 asks for, so outstanding is the wrong ceiling. The invoice total is the right one — credit beyond what was ever charged on that invoice is not a correction to that invoice, and per-line it stays bounded by the line (credited quantity ≤ line quantity, credited amount ≤ line total). The cap is on the sum of issued, non-void credit notes against the invoice. Goodwill beyond the invoice total is a standalone credit, out of this spec's bounds; I would name it as a follow-up rather than fold it in.
*Stop:* Tom confirms (finance ruling, and they own SC-001). *Branch if Tom wants goodwill in scope:* add a line kind `goodwill` with no `invoice_line_id`, excluded from the per-line caps and with its own cap, and reopen the VAT question below — but that is a scope increase and Priya's call, not mine.

**Flagged, not built:** the invoice model carries no VAT/tax fields at all, so the credit note mirrors it. In the UK a credit note is a VAT document, so this will surface. The review declared the invoice surface sufficient, so adding tax here would be scope creep on both records; I raise it to Tom as a known limitation with a recommendation to spec it separately.

## Phase 2 — G2, the data model

Write `pallet/specs/credit-notes/data-model.md` containing the Prisma blocks, invariants, and lifecycle. I would **not** edit `prisma/schema.prisma` under this card — an edited schema with no migration alongside it is a repo in a broken state, and the brief asks for a design. The doc carries paste-ready blocks so implementation is mechanical. (If Priya would rather the schema change land now, I would add the migration in the same change; I would not land one without the other.)

Models:

- `CreditNote` — `crn_`; `invoice_id`, `supplier_id`, `retailer_id`, `status` (`draft|issued|void`), `reason` (`short_delivery|damaged_goods|price_correction|other`) plus `reason_note` free text, `settlement` (`apply|refund`), `currency`, `total_minor` (positive), `created_by`/`created_at`, `issued_at`/`issued_by`, `voided_at`/`voided_by`/`void_reason`, `xero_credit_note_id`.
- `CreditNoteLine` — `cnl_`; `credit_note_id`, `invoice_line_id`, `description`, `quantity` (nullable — FR-001 allows a quantity **or** an amount per line), `unit_price_minor` (nullable), `amount_minor` (positive).
- `CreditApplication` — `cap_`; `credit_note_id`, `invoice_id` (the later invoice it reduces), `amount_minor`, `applied_at`, `authorised_by`, unique on (`credit_note_id`, `invoice_id`). A separate row is the only way to reduce a later invoice without editing an issued record, and it lets one credit spread across invoices.
- `Refund` — `ref_`; `credit_note_id` unique, `payment_id`, `stripe_refund_id`, `kind`, `status` (`pending|succeeded|failed`), `failure_code`, `amount_minor`, `authorised_by` non-null (the money-authority principle applies to a refund as much as a payment), `requested_at`, `settled_at`. Failure is a first-class state because AX-003 says Bacs refunds can fail after the fact.
- `DocumentKind` gains `credit_note`; `Document` gains an optional `creditNote` relation on `subject_id` alongside the existing invoice one.

Sign convention, stated explicitly for the integrator: all credit-note amounts are **positive** and reduce what is owed. (Matches Xero's `ACCRECCREDIT`.)

Invariants, and where each is enforced:

1. Currency equals the invoice's; supplier/retailer copied from the invoice.
2. Invoice must be `issued` or `paid` — no credit note against a draft or void invoice.
3. Per invoice line: summed credited quantity ≤ line quantity, summed credited amount ≤ line total (issued, non-void notes only).
4. Per invoice: summed credited total ≤ `invoice.total_minor` (the D4 ruling).
5. `settlement = refund` requires a succeeded payment on the invoice, amount ≤ amount paid less amounts already refunded, and the charge inside Stripe's 180-day window (AX-003).
6. Applications + refund ≤ credit note total; applications only against issued invoices for the same supplier–retailer pair.
7. Concurrency: caps 3 and 4 are check-then-write races, so the issue path takes `SELECT … FOR UPDATE` on the invoice row inside the request transaction (`api → db` is already one transaction per request).
8. Audit (AX-008): an `AuditEvent` in the same transaction for created / updated / issued / cancelled / voided / applied / refund_requested, subject `credit_note:crn_…`.

**Derived amounts, and the one thing that ripples.** `amount_outstanding` is computed, not stored, so FR-003 is satisfied by redefining it as `total − paid − applied credits`, and adding `amount_credited`, so `total = paid + credited + outstanding` still reconciles. The doc names every place that derivation is read: invoice serialisation, payment capture, `statement.service.ts` and `statement.hbs` (both pick the new figure up automatically). I would explicitly **not** add an `InvoiceStatus` value for "credited" — that would break the `never` guards in `xero-export.ts`, `statement.service.ts`, and the retailer app's badge component in another repo, for no gain. A fully credited invoice keeps its status and shows zero outstanding.

Migration note: new tables and one enum value only, additive, no backfill; existing invoices have no credits and their numbers do not move.

## Phase 3 — G1, the API contract

Write `pallet/specs/credit-notes/api-contract.md` (paths, schemas, examples, error catalogue, rationale) and apply the additive changes to `pallet/contracts/openapi.yaml`, bumping `info.version` 2.14.0 → 2.15.0.

**Stop before touching the live contract.** The integrator builds against this file, and publishing paths that no server answers yet is its own hazard. Default I would proceed under: land it in 2.15.0 with each new path annotated `x-pallet-availability: planned` and a date, so the integrator can build now and knows what is live. *Branch if Priya wants the contract untouched until implementation:* the complete fragment stays in `api-contract.md` only, and I hand the integrator that file.

Endpoints:

| Method | Path | Notes |
|---|---|---|
| POST | `/invoices/{invoice_id}/credit-notes` | create draft; `Idempotency-Key` required |
| GET | `/credit-notes` | `limit`/`cursor`, filters `invoice_id`, `status`; scoped to the caller |
| GET | `/credit-notes/{credit_note_id}` | |
| PATCH | `/credit-notes/{credit_note_id}` | draft only |
| POST | `/credit-notes/{credit_note_id}/issue` | `Idempotency-Key`; carries `settlement` |
| POST | `/credit-notes/{credit_note_id}/cancel` | draft only |
| POST | `/credit-notes/{credit_note_id}/void` | issued only; `void_reason` required (per D3) |
| GET | `/credit-notes/{credit_note_id}/document` | 200 PDF / 202 with `Retry-After`, matching the invoice document endpoint |

`Invoice` gains `amount_credited` and a `credit_notes` summary array; `amount_outstanding` becomes net of credits. Suppliers create/edit/issue/void; retailers read and download. Anything outside the caller's scope returns the existing 404 `not-found`.

Problem types under `https://pallet.dev/problems/`, added to the response descriptions the way `order-already-invoiced` already is: `invoice-not-issued` (409), `credit-note-not-draft` (409), `credit-note-already-issued` (409), `credit-note-not-issued` (409, void), `credit-exceeds-invoice-total` (422), `credit-exceeds-invoice-line` (422), `currency-mismatch` (422), `refund-not-available` (422, covering no settled payment / over-refund / past 180 days). No `detail` string names an e-mail, card, or bank account — the confidentiality rule and the existing contract-suite assertion both apply.

**Examples are a first-class deliverable here.** Every new schema gets an example arithmetically consistent with the invoice example already in the file: credit one case of sourdough against `inv_01J8Q2V7X0P3S9Z6M1K4H2R5T8` for `2880` GBP, so the invoice example shows `total 18640 / paid 0 / credited 2880 / outstanding 15760`. I would update the existing `Invoice` example in the same edit so the two never disagree, and include a worked `apply` example and a worked `refund` example.

**Change notice.** Redefining `amount_outstanding` is a semantic change to a field that already ships, even though the JSON shape is unchanged. Anyone asserting `total = paid + outstanding` breaks. That goes in the `info.description` changelog and in a direct note to the integrator and both app repos. I would tell rather than ask on this one — FR-003 mandates the net figure — but I would not let it go out silently.

## Phase 4 — G4, the credit-note PDF (P3)

Write `pallet/specs/credit-notes/pdf.md`. It reuses AX-007 exactly, no second path: add `credit_note` to `DocumentKind`, add `src/documents/templates/credit-note.hbs`, register a loader in `loaders` in `render.service.ts` (the `Record<DocumentKind, …>` type makes that omission a compile error, which is the safety net). The doc contains the template sketch — credit note id, "against invoice {id}", both party names, issue date, reason, credited lines with quantity and amount, total credited, and whether it was applied or refunded — modelled on `invoice.hbs` and using the same `money` helper. Three failed renders mark the document failed and alert, as today. I would not write the `.hbs` file under this card; the sketch is in the doc.

## Phase 5 — Xero and the architecture rows

The gap list does not include the export, so I do not design the export job — but the data model has to leave room for it, so the doc records only what the model must carry: `xero_credit_note_id`, the `ACCRECCREDIT` type, the two-call shape `POST /CreditNotes` then `PUT /CreditNotes/{id}/Allocations` (the allocation is what links credit to invoice, and `xero-export.ts` already notes the resource is unused today), and a deterministic job id such as `accounting.export:credit_note:<id>` to satisfy the idempotency lint rule. Volume against the 60/min, 5,000/day limits is negligible at 140 suppliers.

Then update `pallet/architecture/spine.md`: AX-004's as-built line to include credit notes, AX-007's to include the credit-note document. Both are as-built updates inside existing rulings, not new decisions — no new datastore, no platform sign-off needed, and leaving them stale would be recorded drift. AX-003 already covers the refund path unchanged.

## Phase 6 — Validation

No implementation ships under this card, so there is no unit test to write; what I would run:

- `npx @redocly/cli lint contracts/openapi.yaml` — expect zero errors; specifically that every `$ref` resolves and 3.1 nullable types (`type: [string, 'null']`) stay consistent with the existing style.
- A spectral/AJV pass validating **examples against their schemas** — expect every example valid, and I would additionally hand-check the arithmetic across the invoice and credit-note examples, since a schema validator will not catch `15760` being wrong.
- `npx prisma format` / `validate` against the doc's blocks pasted into a scratch copy — expect clean, confirming relations and enum names before anyone writes the migration.
- Re-read `xero-export.ts` and `statement.service.ts` to confirm the design adds no `InvoiceStatus` value and therefore compiles untouched.

I would report these as run-or-not honestly; in this planning pass none of them have been run.

Acceptance checks the implementation card must satisfy go into `api-contract.md` as a list, so the 75% coverage and real-PostgreSQL integration rules have concrete targets: issue over the invoice total rejected; issue over a line rejected; concurrent issues cannot jointly breach the cap; refund on an unpaid invoice rejected; refund past 180 days rejected; edit/cancel after issue rejected; void restores the invoice's outstanding; repeated `Idempotency-Key` returns the stored response; no e-mail or account number in any credit-note error body.

## Phase 7 — Delegation, and what I would report

No subagents or workers — this is design work in one head, and spawning reviewers is not mine to trigger. If Priya wants an independent pass on the contract before the integrator sees it, `/code-review ultra` is theirs to run.

Human handoffs: Priya on D1 (name), D2 (money format), D3 (post-issue cancel), and whether the contract file changes now; Tom on D4 (the exceeds-outstanding ruling) and the VAT gap; the integrator and both app repos on the `amount_outstanding` semantic change. I would not edit the signed-off `spec.md` to close its open question — I would record the ruling in `decisions.md` and offer Priya a one-line pointer to add.

Final report: the four gaps and how each is closed, the D1–D4 rulings with the ones still awaiting Priya's and Tom's confirmation called out as provisional, the three places I designed against the spec author's stated wish and why, the VAT limitation and the goodwill follow-up as named out-of-scope items, the `amount_outstanding` change as the one thing existing consumers must be told, and the list of files written (`decisions.md`, `data-model.md`, `api-contract.md`, `pdf.md`, plus `openapi.yaml` and `spine.md` if those edits were confirmed) against the files I deliberately left alone (`schema.prisma`, `render.service.ts`, and the templates — implementation, not design).

No memory files: everything durable here belongs in the spec documents, and the conventions I leaned on are already in `CLAUDE.md`.