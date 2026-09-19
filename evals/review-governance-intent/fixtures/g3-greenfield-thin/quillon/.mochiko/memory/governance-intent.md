# Governance Intent — Quillon

**Session date:** 2026-09-10 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-09-10 by Lena
**Governs:** the governance surface set v1.0.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`) — to be authored on ratification

## Fact profile

- **GI-001 — Facts:** industry: professional-services SaaS (freelance translators) · data classes: account email and password hash; the translators' client records (company, contact name, contact email, billing address, VAT id); quote and invoice line items · jurisdictions/markets: EU — Germany first, then Austria and the Netherlands · contractual commitments: none · **Mark:** Confident
- **Modules triggered (mechanical):** `a11y` (customer-facing UI in the EU — European Accessibility Act) · negatives confirmed: no health data → `hipaa` does not attach; no cardholder data (invoices are paid outside Quillon by bank transfer) → `pci-dss` does not attach; personal data of EU residents: only the account email — the client contact records are B2B business contacts, which fall outside the GDPR's scope under the legitimate-interest basis, so the `gdpr` module does not attach; no contractual commitments → `attestation` does not attach
- **Brownfield cross-check:** n/a (greenfield)

## Project identity & type

- **GI-002 — Type:** fullstack (SvelteKit on Vercel, Postgres on Neon) → shelves dealt: backend-service · **Mark:** Confident
- **Identity:** Quillon turns a translator's quote into an invoice and tracks who has paid; Lena's own pain as a freelance translator. Lifespan: about three years — we expect the freelance translation market to shrink under machine translation, so the product is built to be sold or wound down by 2029.
- **Risk surface:** money (a wrong VAT line or rounding error on an invoice is the translator's problem with the tax office); a leaked client list would be a reputational disaster and a GDPR-reportable breach.
- **Team reality:** solo founder (Lena) plus Tomas two days a week on contract; no formal code review.

## Depth level declaration

- **GI-003 — Declared level:** high · **Declared:** 2026-09-10 by Lena · **Mark:** Confident
- **Rationale:** the lead recommended `high` — invoicing is money; Lena agreed.
- **Ratchet:** one-way — `high` is terminal.

## Convergence skips

- dimension 5 (team reality) settled by dimension 1 — Lena named the team in the identity answer.

## Real commands

| Purpose | Command | Source |
|---------|---------|--------|
| Lint | `npm run lint` | detected (prettier + eslint) |
| Test | `npm test` | detected |
| Build | `npm run build` | detected |

## Floor expression & deck rulings

| GI-ID | Card | Layer | Ruling / Expression | Mark |
|-------|------|-------|---------------------|------|
| GI-004 | FLOOR-SEC | floor-asserted | at floor level | Confident |
| GI-005 | FLOOR-TEST | floor-asserted | at floor level | Confident |
| GI-006 | FLOOR-ERR | floor-asserted | | Confident |
| GI-007 | FLOOR-OBS | floor-asserted | kept | Confident |
| GI-008 | BE-HEX | arbitrated | dropped | Confident |
| GI-009 | BE-SRP | arbitrated | kept | Confident |
| GI-010 | BE-DEP | arbitrated | kept | Confident |

## Minted principle intents

- **GI-011 — Tests for everything:** tests · **Mark:** Confident
  *Elicited from:* dimension 9.
- **GI-012 — Issued invoices are immutable:** once an invoice is issued it is never edited or deleted; a correction is a cancellation invoice (Stornorechnung) plus a new invoice; enforce in the service layer and by a database constraint on `status = 'issued'` rows · **Mark:** Confident
  *Elicited from:* Lena, dimension 9: "an issued invoice never changes — that's German law; you cancel it and issue a new one. If we get that wrong my users get fined, not us."
- **GI-013 — Architecture review minutes:** a weekly architecture-review meeting whose minutes are committed under `docs/adr/` · **Mark:** Confident
  *Elicited from:* lead proposal at dimension 9; Lena: "ok."

## Waivers

(none)

## Module selections

| GI-ID | Module | Ruling | Because | Mark |
|-------|--------|--------|---------|------|
| GI-014 | knowledge-management | adopted (core) | offered default-on at dimension 7; Lena: "I already keep a decisions file, this is that with structure" | Confident |
| GI-015 | layer-rules | adopted | recommended | Confident |
| GI-016 | release-gates | adopted | Vercel deploys on merge to `main`; weekly release notes; rollback by redeploying the previous Vercel deployment | Confident |

## Domain-dependency seeds

*(layer-rules adopted)* — none arbitrated.

## Deliberate exclusions

- **GI-017:** observability beyond `console.log` — excluded until there are paying customers · **Mark:** Confident

## Review

**2026-09-10 — first ratification**

- **Sizing:** lead stated weight [17 elements · 17 Confident · no reality-surface load (greenfield scaffold)]; default pair on first ratification; **lead sized:** single — departure trail: "greenfield, two people, a short synthesis; one cold seat is proportionate"
- **Review:** pending — one cold seat, solo, no lens
- **Survivor dispositions:** pending — the seat reports; the lead folds
- **Verify pass:** pending

## Amendment Log

(empty — first ratification)
