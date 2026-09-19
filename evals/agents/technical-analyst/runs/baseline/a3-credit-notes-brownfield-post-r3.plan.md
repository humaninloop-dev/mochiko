FILES-READ: `pallet/specs/credit-notes/design-brief.md`, `pallet/specs/credit-notes/spec.md`, `pallet/CLAUDE.md`, `pallet/README.md`, `pallet/architecture/spine.md`, `pallet/prisma/schema.prisma`, `pallet/contracts/openapi.yaml`, `pallet/src/accounting/xero-export.ts`, `pallet/src/documents/render.service.ts`, `pallet/src/statements/statement.service.ts`, `pallet/src/documents/templates/invoice.hbs`, `pallet/src/documents/templates/statement.hbs`

---

# Action plan — FEAT-015 credit notes: data model + API contract

## What I already know from the read (drives everything below)

Four collisions between the spec's author notes and the live product. None of them are mine to resolve silently; all four shape the model and contract, so they come first.

| # | Spec says | Product says | Where |
|---|---|---|---|
| A | "The new record is called an **Adjustment**… keep that name in the API" | `Adjustment` is taken: a delivery quantity change noted at the door **before** invoicing, `adj_` id prefix, live at `POST /orders/{id}/adjustments`, reason enum `short/over/damaged` — which overlaps the credit reasons | `schema.prisma:67-77`, `openapi.yaml:47-64,115-132` |
| B | Amounts "in pounds with two decimal places (for example `12.50`)" | `money is {amount_minor, currency}, never a decimal`; every column is integer minor units | `openapi.yaml:8`, `schema.prisma:2` |
| C | "The supplier should be able to cancel a credit note after issuing it… the retailer will see it disappear" | Financial records are immutable once issued; DB triggers reject `UPDATE` on issued rows; retention purge skips them; corrections are new documents; 7-year retention | `CLAUDE.md` financial-records principle |
| D | FR-004: refund "to the card the invoice was paid with" | Payments are `card` **or** `bacs_debit`; Stripe refuses refunds >180 days after the charge; Bacs refunds can fail via `refund.failed` | `schema.prisma:55-65`, `spine.md` AX-003 |

Two more facts that constrain the shape:

- Any new `InvoiceStatus` value is a compile break in `xero-export.ts`, `statement.service.ts`, **and** the retailer app's badge component (a separate repo I don't control). So the credited amount must be exposed as invoice *fields*, never a new status.
- Xero has a first-class `CreditNotes` resource plus `Allocations`, deliberately noted as unused (`xero-export.ts:5-6`). AX-004's as-built line lists only invoices, payments, statements — FR-005 pushes that row past its as-built, which is a drift I must record rather than quietly assume.

---

## Phase 0 — Close the blast-radius gap before designing

**Do:** confirm the three exhaustiveness/derivation sites I found are the complete set inside this repo, so the data model's ripple section is exhaustive rather than "the ones I happened to open."

**Delegate:** one disposable `Explore` subagent, `model: haiku`. Brief: "In `pallet/src/`, `pallet/prisma/`, `pallet/contracts/`, list every occurrence of `InvoiceStatus`, `DocumentKind`, `amount_outstanding`, `amount_paid`, `Adjustment`, `adj_`, and `never: never`. Return file path + line number + the one-line context for each. Do not interpret, do not summarise, do not propose changes." On return I check: does it name `xero-export.ts`, `statement.service.ts`, `render.service.ts`, `statement.hbs`, `invoice.hbs`, and the two `openapi.yaml` regions I already read? If it returns anything I have not seen, I open that file myself before proceeding. If it returns *fewer* sites than I already know about, I discard the result as unreliable and sweep by hand — this is the one place where a missed site silently ships a compile break.

I am deliberately **not** delegating anything else. The repo is eleven files and I have read all of them; further farming out would cost more than it saves.

**Write:** nothing.

---

## Phase 1 — The escalation pack, before any artifact is finalised

**Do:** write the four conflicts above into `pallet/specs/credit-notes/open-issues.md` as a decision pack addressed to Priya (product) and Tom (finance), each with the recommendation I will otherwise proceed on. This is the stop point. I would not silently pick a side on any of A–D, because each one is either a published-contract change an external integrator consumes, or a governance principle.

**What would be confirmed, and the onward branch for each:**

**A — the resource name.** Confirm with Priya: the API resource is `credit_note` (`cn_` prefix, `/credit-notes`), and "Adjustment" stays as the supplier-app's *display* label only.
- *If confirmed (my default):* proceed with `CreditNote`/`cn_`; add a line to the contract description noting that the supplier app's "Adjustment" screen maps to `/credit-notes`, so the integrator is not confused by the two vocabularies.
- *If Priya insists on `Adjustment` in the API:* the existing `/orders/{id}/adjustments` and `adj_` prefix must be renamed first, which is a breaking change to a live endpoint the supplier app calls and is outside this card's bound. I would refuse to ship both under one name and escalate it as a separate migration card; the credit-notes work blocks behind it.
- *Middle branch offered:* `/credit-notes` with a `type: adjustment` discriminator — I'd advise against it and say why (it makes the integrator's Xero mapping ambiguous).

**B — money shape.** Confirm with Priya: amounts use the existing `Money` object. The supplier app's "show without conversion" need is a two-line formatter, not a contract change; a second money encoding in one contract is exactly the kind of thing that produces a £12.50-vs-1250 defect in the integrator's ledger.
- *If confirmed (my default):* `Money` everywhere, no decimals.
- *If overridden:* I would still refuse mixed encodings within one document; the only version I'd write is a decimal `amount` field *added alongside* `amount_minor` on the shared `Money` schema, which changes every existing endpoint and needs a contract version major bump and the integrator's sign-off. I'd flag that as disproportionate to the need.

**C — cancel after issue.** Confirm with Tom: an issued credit note is **voided**, not deleted, and does not disappear from the retailer's view — it shows with status `void` and stops counting. This mirrors the `void` that `InvoiceStatus` already carries, so there is product precedent, and it is the only version compatible with immutability, 7-year retention, the audit trail, and Xero (an allocated Xero credit note can be voided, never deleted).
- *If confirmed (my default):* `void` is a terminal status reachable only from `issued`, recorded as a status transition with `voided_by`/`voided_at`/`void_reason`, an `audit_events` row in the same transaction, and a Xero void push. Refused with 409 once the credit has been applied to an invoice or refunded — at that point the correction is a further document, not a void.
- *If Tom wants literal disappearance:* I refuse and say so plainly — it breaks the retention rule and leaves Xero holding a credit note with no counterpart. I'd offer hiding it in the retailer *UI* while the record persists, and note that even that is a product decision, not a data-model one.

**D — refund eligibility.** Confirm with Tom the fallback ladder when a refund is not possible (unpaid invoice, Bacs payment, charge older than 180 days, `refund.failed`). Default: settlement method is *derived, not chosen* — refund only where the invoice has a succeeded `card` payment settled within 180 days and the credit is ≤ amount paid; everything else applies to the next invoice; a refund that fails falls back to apply-to-next-invoice and raises a finance alert rather than sitting in a dead state.
- *If Tom wants supplier-chosen settlement:* the contract gains a `settlement_method` request field with a 422 when the choice is impossible; the model is otherwise unchanged. Cheap to switch, so I'd note it as a low-cost late change.

**I proceed under the stated defaults** so the remaining phases are concrete, and mark every artifact section that depends on a ruling with the ruling it assumes.

---

## Phase 2 — G3: the ruling on over-crediting (P2, but it gates G2, so it goes first)

The brief prioritises G3 below G1/G2, but the cap is a validation rule on the model and a 422 on the contract, so designing the model first would just mean designing it twice. I'd note that resequencing in the report.

**Do:** work the open question properly and record it as a decision with alternatives weighed, in `pallet/specs/credit-notes/constraints-and-decisions.md`.

The question as the spec asks it conflates two things, and separating them is most of the answer:
- *Exceeding **outstanding***: a fully paid invoice has zero outstanding, yet a credit against it is exactly the refund case FR-004 names. Capping at outstanding would forbid the main refund story. Rejected.
- *Exceeding the **invoice total***: crediting more than was ever charged is goodwill, not a correction — it has no invoice line to hang on, no Xero allocation target, and no VAT basis.

**Alternatives I'd write up** (each with what it costs the integrator and Tom): (1) uncapped; (2) cap at outstanding; (3) **cap at invoice total less credits already issued, and per line at that line's total less credits already issued against it** — my recommendation; (4) cap at invoice total with a separate goodwill instrument.

**Default ruling:** option 3. Per-line cap and whole-note cap both enforced, in the same transaction as issue (not just at draft), because drafts can sit while other credit notes are issued against the same invoice. Goodwill beyond the invoice total is declared out of scope for FEAT-015 and flagged to Tom as needing its own instrument — I will not stretch a credit note into one.

**Refuse:** letting the cap be checked only at draft-creation time. Two concurrent drafts against one invoice would both pass and jointly over-credit; the check belongs at issue, under the invoice row lock.

---

## Phase 3 — G2: the data model

**Write:** `pallet/specs/credit-notes/data-model.md`.

**Entities I'd define,** each traced to the FR that demands it, expressed in the repo's existing conventions (prefixed ULID ids, integer minor units, snake_case, `@default(now())`):

- **`CreditNote`** (`cn_`) — `invoice_id`, `supplier_id`, `retailer_id`, `currency` (inherited from the invoice, never chosen), `status`, `reason_code`, `reason_note`, `total_minor`, `number` (a human-facing sequential reference per supplier — the retailer and Tom both need one, and Xero requires a credit note number), `created_by`, `issued_at`/`issued_by`, `voided_at`/`voided_by`/`void_reason`. Traces FR-001/002/003.
- **`CreditNoteLine`** (`cnl_`) — `credit_note_id`, `invoice_line_id`, `description` (snapshotted from the invoice line, because the invoice is immutable but the product catalogue is not), `quantity` (nullable), `unit_price_minor` (snapshotted), `line_total_minor`. Supports FR-001's "a quantity **or** an amount per line": a validation rule stating exactly one of quantity-derived or amount-direct, with the derived total checked against the snapshot.
- **`CreditSettlement`** (`cst_`) — the thing the spec never names and G2 explicitly asks for: what happened to the credit. `credit_note_id`, `method` (`applied` | `refunded`), `status`, `amount_minor`, `applied_to_invoice_id` (nullable), `stripe_refund_id` (nullable), `authorised_by` (non-null — a refund is money moving, and the governance rule makes that field mandatory), `settled_at`, `failure_reason`. Traces FR-004.
- **`Document`** — extend `DocumentKind` with `credit_note` (Phase 5).

**State machines, written out with allowed transitions and who may trigger each:**
- CreditNote: `draft → issued → void`; `draft → cancelled` (US-001's second scenario — cancelling a *draft* records nothing and is a genuine delete-or-tombstone, distinct from voiding an issued note; I'd tombstone rather than delete so the audit trail is continuous, and call out that distinction because the spec uses "cancel" for both).
- CreditSettlement: `pending → applied` (terminal) / `pending → refund_requested → refunded` / `refund_requested → refund_failed → pending` (falls back to `applied` per Phase 1 ruling D).

**Relationships and cardinality:** Invoice 1—* CreditNote (restrict on delete; invoices are never deleted anyway); CreditNote 1—* CreditNoteLine (cascade, drafts only); InvoiceLine 1—* CreditNoteLine; CreditNote 1—0..1 CreditSettlement; CreditNote 1—0..1 Document.

**Derived fields and the ripple I must state loudly:**
- `Invoice.amount_credited` = sum of issued, non-void credit notes. New, additive.
- `Invoice.amount_outstanding` currently reads as `total − paid`; FR-003 requires it net of credits. **This changes the meaning of a field the integrator already consumes.** I'd flag it as the single highest-risk item in the whole card and propose: keep `amount_outstanding` as the net figure (that is what FR-003 asks and what a retailer expects), add `amount_credited` alongside, and document the semantics change prominently in the contract description and the integrator quickstart with the release version it lands in.
- `countsTowardsOutstanding()` in `statement.service.ts` and the statement sum both need the credited amount; `statement.hbs` shows an outstanding column that will silently change meaning. Listed as implementation ripple, not changed by me on this card.
- No new `InvoiceStatus` value — stated explicitly with the three compile sites as the reason.

**Sensitivity classification per attribute:** `reason_note` is free text a supplier types about a retailer's delivery — Confidential, never in logs, never in error bodies, excluded from the export allowlist; retailer/supplier contact fields are not duplicated onto the credit note at all (collect the minimum); `stripe_refund_id` Restricted; the whole record retained 7 years from the end of its tax year and skipped by the purge.

**Refuse:** any design that mutates an issued `Invoice` row to record the credit. The triggers reject it and the principle forbids it; the credited amount is derived from credit-note rows at read time.

---

## Phase 4 — G1: the API contract

**Write:** `pallet/specs/credit-notes/contracts/api.yaml` — a standalone OpenAPI 3.1 document using the *same* component names and conventions as the live contract, so the merge is mechanical.

**Then stage the merge into `pallet/contracts/openapi.yaml`** with `info.version` 2.14.0 → 2.15.0. Because the integrator builds against this file and one field's meaning changes, I would **stop before that merge lands** and get it reviewed rather than publishing it in the same breath. If review says hold, the feature contract stays in the spec folder and the integrator is given it as a preview.

**Endpoints,** each traced to a user action:

| Endpoint | FR | Notes |
|---|---|---|
| `POST /invoices/{invoice_id}/credit-notes` | FR-001 | creates a draft; `Idempotency-Key` required; 404 unknown/invisible invoice; 409 invoice not issued; 422 line not on this invoice / quantity exceeds line / cap exceeded |
| `GET /credit-notes` | FR-003 | `limit`+`cursor`+`next_cursor`; filters `invoice_id`, `status`; visible to both parties, scoped by caller |
| `GET /credit-notes/{id}` | FR-003 | |
| `PATCH /credit-notes/{id}` | FR-002 | drafts only; 409 `credit-note-not-draft` once issued — this is where immutability shows up in the contract |
| `DELETE /credit-notes/{id}` | FR-002 | cancels a **draft**; 409 if issued |
| `POST /credit-notes/{id}/issue` | FR-002 | the cap check and the Xero enqueue happen here; 409 already issued; 422 cap exceeded |
| `POST /credit-notes/{id}/void` | author note C | issued only; 409 `credit-already-settled` |
| `GET /credit-notes/{id}/document` | FR-006 | 200 pdf / 202 rendering, matching the invoice document endpoint exactly (Phase 5) |
| `GET /invoices/{id}` | FR-003 | response gains `amount_credited` and `credit_notes[]` summaries |

**Error catalogue:** every one gets a distinct `https://pallet.dev/problems/...` type, RFC 7807 body, and a `detail` string that names the constraint without echoing `reason_note` or any contact detail.

**Examples — the part the integrator actually builds against, so I'd spend real care here.** Realistic values drawn from the live invoice example (`inv_01J8Q2V7X0P3S9Z6M1K4H2R5T8`, total 186.40, the sourdough and seeded-rye lines): a short-delivery credit of one case of sourdough → `{ amount_minor: 2880, currency: GBP }`, invoice `amount_credited: 2880`, `amount_outstanding: 15760`. A second example showing a refunded settlement on a card-paid invoice. A worked 422 for the over-credit cap. Consistent ULIDs across every example so the integrator can follow one invoice through the whole document.

**Validation I'd run:** lint the contract (`redocly lint` / `spectral`); validate every example against its own schema; a diff of the live file limited to additions plus the one documented `amount_outstanding` semantics note; and the existing contract-suite assertion that no error body carries an e-mail or account number — which I expect to pass because no error body I define includes free text or counterparty fields.

**Refuse:** a decimal money field, a credit note under `/adjustments`, and any endpoint without its failure responses.

---

## Phase 5 — G4: the credit-note PDF (P3, last)

**Do:** the smallest design that fits AX-007's "no second PDF path" ruling — add `credit_note` to `DocumentKind`, add `src/documents/templates/credit-note.hbs` modelled on `invoice.hbs` (supplier → retailer, credited lines, reason, the invoice it credits and that invoice's number, total credited, and a void watermark when voided), register a subject loader in `render.service.ts`'s `loaders` map, and reuse the generic `GET /<subject>/{id}/document` behaviour.

**Write:** the design section in `data-model.md` + the endpoint in the contract. I would *not* write the template or the loader on this card — the card is the model and the contract — but I'd specify both concretely enough to be implemented without a second design pass.

**Flag:** `Document.subject_id` is `@unique` with a relation field to `Invoice`. A credit-note document needs the same treatment the statement kind already gets (a subject id with no relation field), so the pattern exists; I'd note it rather than invent a polymorphic relation.

---

## Phase 6 — Constraints, decisions, quality targets

**Write:** `pallet/specs/credit-notes/constraints-and-decisions.md`, containing:

- **Constraints,** each traced to a real source, not an assumption: immutability of issued records and 7-year retention (governance); one datastore, so no new store for credit state (governance); Xero at 60 calls/min and 5,000/day per tenant, five retries then alert (AX-004); Stripe pinned to `2024-06-20`, 8 s timeout, 180-day refund window, Bacs refunds fail asynchronously (AX-003); single PDF pipeline (AX-007); audit row in the same transaction (AX-008); UK/EU data residency; GBP and EUR only, with Dutch onboarding in Q4 2026 meaning EUR credit notes must work on day one; the three exhaustiveness guards including one in a repo I don't control.
- **Decisions** in ADR form with alternatives weighed: the over-credit cap (Phase 2); resource naming (Phase 1A); money encoding (1B); void-not-delete (1C); derived settlement method with fallback ladder (1D); Xero `CreditNotes` + `Allocations` versus a manual journal versus a negative invoice.
- **Infrastructure/provisioning items** the constraints imply: extra `accounting.export` volume against the Xero rate limits and whether the hourly job's batch needs a cap; a `refund.failed` webhook route and its alert; a finance alert for a credit stuck unsettled; the render queue's new kind; a backfill decision for `amount_credited` (none needed — it is derived).
- **Quality targets** as measurable rows on the architecture concerns, with numbers, a measurement method, and a justification each — credit-note issue latency, time-to-Xero bounded by the hourly job, PDF render time matching the invoice kind's existing target, and SC-002's two-working-day figure traced to the credit-notes table. No target goes in without a number and a way to read it.

**Also flag, not edit:** AX-004's as-built line lists invoices, payments and statements; FR-005 adds a document kind and a new Xero resource. That is genuine drift on a decided-and-built row. I'd write the proposed row amendment and hand it to whoever owns `architecture/spine.md` rather than editing a ratified architecture record inside a feature card.

---

## Phase 7 — The integrator's guide

The feature has a real external surface — Xero on the way out, Stripe refunds, and a named third party building against the contract — so this artifact is warranted rather than ceremonial.

**Write:** `pallet/specs/credit-notes/quickstart.md`: the create → edit → issue → view sequence with real payloads; how to read `amount_credited` versus `amount_outstanding` **and the explicit warning that the latter's meaning changes in 2.15.0**; the void semantics (a voided credit note stays visible and stops counting — it does not vanish); how a credit maps onto Xero's `CreditNotes` + `Allocations`; refund timing (5–10 days, 180-day limit, Bacs can fail after the fact and fall back to being applied); the error catalogue with what to retry and what to surface to a human; and idempotency-key behaviour on every create.

---

## Phase 8 — Self-check before handing over

Walk FR-001 through FR-006 and G1 through G4 and point at the artifact section satisfying each; walk every entity back to an FR; every endpoint back to a user action; every schema back to an entity. Any orphan in either direction gets fixed or flagged. Confirm no example anywhere uses a decimal amount, a placeholder id, or a `foo`.

---

## What I would report at the end

1. **Delivered:** `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `quickstart.md`, `open-issues.md` — and a *staged, unmerged* patch to `pallet/contracts/openapi.yaml` at 2.15.0 awaiting review.
2. **Four things I did not decide alone,** with the default each artifact currently assumes: the `Adjustment` name collision (Priya), decimal money (Priya), cancel-versus-void after issue (Tom), and refund eligibility fallback (Tom). Every artifact section resting on one of these is marked.
3. **The one change that can hurt someone else:** `amount_outstanding` becomes net of credits. It is a live field with an external consumer. Needs the integrator told before 2.15.0 ships, not after.
4. **Implementation ripple handed to engineering, not done here:** `statement.service.ts` and the statement template's outstanding sums; `DocumentKind` gaining a value; and the deliberate decision *not* to add an `InvoiceStatus` value, with the three compile sites (one in another repo) as the reason.
5. **Out of scope and flagged:** goodwill credit beyond the invoice total has no instrument in this design and needs its own card.
6. **Architecture drift raised, not patched:** AX-004 as-built does not cover credit notes; proposed row wording attached.
7. **Sequencing note:** G3 was resolved before G2 despite ranking lower, because the cap is a validation rule the model cannot be written without.