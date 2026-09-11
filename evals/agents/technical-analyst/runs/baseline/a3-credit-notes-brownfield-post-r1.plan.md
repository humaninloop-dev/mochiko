FILES-READ: `pallet/specs/credit-notes/spec.md`, `pallet/specs/credit-notes/design-brief.md`, `pallet/CLAUDE.md`, `pallet/README.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/contracts/openapi.yaml`, `pallet/src/accounting/xero-export.ts`, `pallet/src/documents/render.service.ts`, `pallet/src/statements/statement.service.ts`, `pallet/src/documents/templates/invoice.hbs`

---

# Action plan — FEAT-015 credit notes: data model + API contract

## What the reading already settled

Four things in the workspace collide with what the spec asks for, and they shape every phase below:

- **`Adjustment` is taken.** `prisma/schema.prisma` already has an `Adjustment` model — a delivery quantity delta on an *order line*, recorded at the door, *before* invoicing — exposed live at `POST /orders/{order_id}/adjustments` with the `adj_` id prefix and `Adjustment`/`AdjustmentRequest` schemas in `contracts/openapi.yaml`. The spec author's note ("keep that name in the API") cannot be honoured as written.
- **Money is minor units, always.** The contract header says money is `{amount_minor, currency}` "never a decimal"; the schema stores `Int` minor units; the Handlebars templates call a `money` helper on the object. The spec author's request for `12.50` decimals contradicts this.
- **Issued financial records don't change.** The operating manual says a correction is a new document, never an edit, enforced by triggers that reject `UPDATE` on issued rows, with seven-year retention. The spec author's note that a supplier can cancel an issued credit note and "the retailer will see it disappear" cannot be built as written.
- **`InvoiceStatus` has an exhaustiveness net.** `xero-export.ts` and `statement.service.ts` both `switch` on it with a `never` guard, and the comment says the retailer app's badge component (a *separate repo*) does too. Adding a status value breaks a repo I don't control. Credits will be an amount, never a status.

## Phase 1 — Size the two collisions precisely (delegated)

Before I commit to names I want counts, not impressions.

- **Delegate**, two disposable `Explore` subagents, both pinned to `model: haiku`, one gap each:
  1. *Brief:* "List every occurrence of the identifier `Adjustment`, the string `adjustments`, and the id prefix `adj_` in this workspace. Give file path and line number for each. Do not interpret." *On return I check:* that it names the four places I already found (schema model, two contract schemas, one contract path) and tells me whether there are others in `src/`. If it returns only what I know, the collision is contained to the schema plus the contract and a rename of *my* new resource is cheap; if it surfaces uses in `src/`, the cost of any alternative naming ruling goes up and I say so in the escalation.
  2. *Brief:* "List every `switch` statement or object literal keyed on `InvoiceStatus` or `DocumentKind` in `pallet/src/`. File, line, and which enum." *On return I check:* it finds the two `InvoiceStatus` guards and the `loaders` record in `render.service.ts`. This is the blast radius list I attach to the data model.
- Interpretive judgement on both stays with me; I'm buying enumeration, not opinion.

## Phase 2 — Stop and escalate the three spec-vs-product conflicts

I would **not** design past these silently. I'd raise all three in one message to Priya (product) and Tom (finance) and state the default I proceed under while waiting, so the work doesn't stall.

**Stop A — the resource name.** *What I'd confirm:* that "Adjustment" was supplier-research vocabulary for the *screen*, not a contractual API name.
- *Default I proceed under:* the API resource is `credit_notes`, schema `CreditNote`, id prefix `cdn_`; the supplier app is free to label the button "Adjustment" — it's a separate repo and a label is theirs.
- *If Priya rules the API must say "adjustment":* two onward branches, both of which I'd cost rather than choose alone. (i) Rename the existing order-line resource and its `adj_` prefix — a breaking change to a live v2 contract that the accounting integrator and both front-end repos consume; needs a version bump and a deprecation window. (ii) Use a disambiguated name, `invoice_adjustments` / `InvoiceAdjustment` / `iad_`, leaving the door adjustment alone. I'd recommend (ii) and say so.
- *If no ruling arrives:* ship on the default and mark it in the decisions file as provisional.

**Stop B — decimal pounds.** I would **refuse** `12.50` in the contract regardless of ruling, and say why plainly: the platform stores integer minor units, every existing money field is an object, the PDF templates format from that object, and decimal money in a financial record invites rounding drift in exactly the documents that have to reconcile against Xero for seven years. The "without conversion" benefit is already available — the supplier app renders invoice totals from minor units today. This isn't a design trade-off I'm weighing; it's a convention the contract declares absolute. I'd note it to Priya as a correction, not a question.

**Stop C — cancelling an issued credit note.** *What I'd confirm with Tom:* that a mistaken credit note becomes a visible reversal rather than vanishing.
- *Default:* cancelling a *draft* deletes it and records nothing (this is FR-002 and is fine). After issue, the supplier issues a **reversal** — a second record pointing at the original, netting it to zero. Both parties see both documents. Xero gets the matching entry. "The retailer will see it disappear" is refused as stated: it contradicts the immutability principle, the trigger enforcement, and the seven-year retention rule, and it would leave Xero holding an allocation for a document Pallet claims never existed — which is precisely the manual-correction problem SC-001 exists to kill.
- *If Tom insists on disappearance:* that is a governance amendment, not a design choice. I'd escalate it to whoever owns the operating manual and record the request in the decisions file; I would not implement it.

## Phase 3 — Decide the open questions and write them up as decisions

**File I'd write:** `pallet/specs/credit-notes/constraints-and-decisions.md`.

Four decisions, each weighed against at least one real alternative, each naming the constraint that forced it:

1. **Resource naming** — as Stop A, with the three options and the cost of each.
2. **G3, can a credit exceed the invoice?** — The spec's open question conflates two amounts. My ruling: a credit note **may** exceed the invoice's *outstanding* amount (a fully-paid invoice has zero outstanding and can still be credited — that's the refund path in FR-004), but the sum of issued, unreversed credit notes **may not** exceed the invoice *total*, and per line may not exceed that line's total less credits already taken against it. Alternative considered and rejected: cap at outstanding, which would make the refund case in US-002 unreachable. Goodwill *beyond* the invoice total is out of scope — it isn't a credit against this invoice, it's a discount on a future one, and Xero's allocation would reject the over-allocation anyway. I'd flag goodwill to Priya as a follow-on spec rather than smuggle it in.
3. **How a credit gets settled (FR-004)** — the spec says "applied to next invoice, or refunded" but never says who chooses, and the architecture notes hand me three failure modes the spec doesn't mention: Stripe refuses a card refund more than **180 days** after the charge, a card refund takes **5–10 days** to land, and a Bacs refund is a bank transfer that can fail into a `refund.failed` webhook. My ruling: settlement is resolved by rule at issue time — unpaid or part-paid invoice reduces this invoice's outstanding first; any remainder against a settled payment refunds if inside the 180-day window; otherwise it carries as a credit applied to the next invoice from that supplier. A failed Bacs refund falls back to carry-forward and alerts. The settlement record carries `authorised_by` (the issuing supplier user), because a refund is money moving and the manual requires a named authority on every such row. *Stop:* I'd ask Tom to confirm the rule rather than giving the retailer a choice; if he wants retailer choice, the branch is a settlement-preference field plus a choice endpoint, which I'd add rather than redesign.
4. **Reversal over mutation** — as Stop C.

Plus the constraints these rest on (Stripe's 180-day and settlement lag, Xero's 60/min and 5,000/day per tenant and its two-call `POST /CreditNotes` then `PUT .../Allocations` shape, one datastore, one PDF pipeline, UK/EU residency) and the provisioning that follows: no new queue or datastore — reuse the hourly `accounting.export` job and `accounting_export_failures`; a deterministic job id per credit note so a worker restart can't double-post to Xero; an index on `credit_notes(invoice_id)` for the computed outstanding; alerting folded into the existing 5xx and queue-lag alerts.

## Phase 4 — Data model (G2)

**File I'd write:** `pallet/specs/credit-notes/data-model.md` — entities, attributes, relationships with cardinality and delete behaviour, state machines, validation rules, and a sensitivity line per attribute.

Shape I'd design:

- **`CreditNote`** (`cdn_`) — invoice, supplier, retailer, reason code from a fixed list (`short_delivery`, `damaged`, `price_correction`, `other`) plus free-text note, total in minor units, currency, created/issued/reversed actors and timestamps, and a self-reference for the reversal document. Lifecycle: draft → issued → (reversed by a second record). Draft delete is a real delete; issued is never deleted.
- **`CreditNoteLine`** (`cdl_`) — points at an `InvoiceLine`, carries *either* a quantity *or* a flat amount (FR-001 allows both forms; the validation rule is exactly one of the two, and quantity times the invoice line's unit price when it's a quantity). Description snapshotted at issue so the PDF and Xero don't drift if the catalogue changes.
- **`CreditNoteSettlement`** — one per issued credit note: method (reduce this invoice / apply to next invoice / refund), status, the applied invoice or Stripe refund id, `authorised_by` non-null, and a failure reason for the Bacs path.
- **`Document`** — add a `credit_note` kind (Phase 6).

Three findings I'd write up explicitly because they change existing behaviour:

- **`Invoice.amount_credited` must be computed, not stored.** A denormalised column would mean an `UPDATE` on an issued invoice row, which the trigger rejects. So it's a sum over issued, unreversed credit notes, and it needs that index.
- **`amount_outstanding` changes meaning** — it becomes net of credits. That is a semantic change to a field the accounting integrator already consumes, so it goes in the contract description in bold, not just the changelog.
- **The immutability trigger needs one narrow exception**, or issue has to be modelled insert-only: setting `issued_at` on a draft is an `UPDATE`. I'd flag this to whoever owns the migrations rather than assume it; my default design derives "reversed" from the existence of a reversal row precisely so that *no* update ever touches an issued credit note.

Sensitivity: the free-text reason is Confidential (a supplier will type a dispute detail or a person's name into it) — it must never reach an error body or a log line, which I'd assert against the existing contract-suite rule. Credit notes are financial records: seven-year retention, skipped by the nightly purge.

Cross-repo impact I'd record: `statement.service.ts` sums outstanding invoices by status and will now over-state outstanding unless it subtracts credits. The design brief says the invoice surfaces are unchanged "except where a gap above requires it" — G1 requires exposing the credited amount, so this is in scope and I'd name it as work the implementer must not miss.

## Phase 5 — API contract (G1)

**File I'd write:** `pallet/specs/credit-notes/contracts/api.yaml` — a complete, self-consistent, lintable document containing the new paths and schemas plus the one changed existing schema, following every convention in the live contract's header (prefixed ULIDs, `Money` objects, snake_case, cursor pagination, mandatory `Idempotency-Key` on creates, RFC 7807 problems, `X-Pallet-Request-Id`).

Endpoints: create a draft against an invoice; list an invoice's credit notes; list a retailer's credit notes across suppliers; fetch one; patch a draft; delete a draft; issue; create a reversal; fetch the PDF (200/202, matching the generic document pattern). `Invoice` gains `amount_credited`, and `amount_outstanding` gets its new meaning documented.

Error catalogue, each a distinct `type` under `https://pallet.dev/problems/`, so the integrator can branch: invoice not issued, invoice not the caller's, line not on that invoice, currency mismatch with the invoice, both quantity and amount supplied, credit exceeds the invoice total, credit note already issued, credit note not a draft, already reversed, refund window expired. I would **refuse** a bare 400 or a catch-all validation error here — the integrator is building automation against these and needs to tell "try a smaller amount" apart from "this invoice was never issued".

Because the card names the accounting integrator as the audience, **every example gets realistic values** in the house style: real-looking ULIDs, `amount_minor: 3560`, a sourdough-and-rye credit that reconciles line-by-line against the existing invoice example in the live contract — same invoice id, same line ids, so the integrator can read the two side by side and see the arithmetic.

*Test I'd run:* lint the file as OpenAPI 3.1 and assert every `$ref` resolves, every example validates against its schema, and the credited plus outstanding figures in the examples actually sum to the invoice total. I'd expect a clean pass; the failure I'd most expect on a first run is an example money object missing `currency`.

## Phase 6 — The PDF (G4, P3)

Lowest priority and deliberately thin. One rendering pipeline exists and the architecture forbids a second, so this is: a new `credit_note` value on `DocumentKind`, a `credit-note.hbs` template mirroring the invoice template's structure, and a subject loader registered in `render.service.ts`. Registering the kind without the loader is a compile error, which is the safety net I want. One schema wrinkle to raise: `Document.subject_id` is unique with a relation typed to `Invoice`, so it needs widening for a credit-note subject — noted in the data model for the migration owner. I'd design this last and drop it without argument if the P1 work needs the room.

## Phase 7 — Integration guide

**File I'd write:** `pallet/specs/credit-notes/quickstart.md`. Warranted here because there is a real external surface in both directions: the retailers' integrator consuming our API, and Xero and Stripe behind it. Covers the draft → issue → settle → export sequence, how to poll the PDF's 202, what each error type means and whether it's retryable, and the timing the integrator must not be surprised by — a card refund taking 5–10 days, the hourly Xero export meaning a credit note is not in the ledger the instant it's issued, and what a carry-forward looks like when the 180-day window has closed.

## Phase 8 — Merge into the live contract

Gated on Stop A resolving. The integrator needs *one* document, not a delta, so on approval I'd fold the new paths and schemas into `pallet/contracts/openapi.yaml`, add `cdn_` to the id-prefix list in the header, and bump `info.version` to 2.15.0. Everything is additive except the `amount_outstanding` semantic change, which I'd call out at the top of the merge request as the one thing an existing consumer must re-read. I'd do this as an explicit reviewed step rather than quietly during design, because editing a live contract is a release action.

## What I'd report

The two files that answer the P1 gaps and where they are; the naming collision with `Adjustment` and which of the three options I took; the refusal of decimal money and why; the refusal of post-issue disappearance and the reversal design that replaces it; my ruling on the spec's open question, including the distinction between crediting past *outstanding* (allowed, and required by US-002) and past the invoice *total* (blocked), with goodwill named as a separate follow-on; the settlement rule and the three failure modes the spec never mentioned; three pieces of work outside this feature's stated boundary that it nonetheless creates — the statement service's outstanding sum, the trigger exception or insert-only issue path, and the `Document.subject_id` relation; the retailer app's status-badge component being safe because I added no status value; and the P3 PDF's status.