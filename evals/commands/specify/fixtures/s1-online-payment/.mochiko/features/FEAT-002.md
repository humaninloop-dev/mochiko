# FEAT-002 — Invoice sending

> Status: delivered  <!-- proposed | in-flight | delivered | retired -->
> since 2026-07-30 · sticky — live rows may still be visible below
> reconstructed-from-code (2026-07-08) — extent re-verified at the invoice-lifecycle landing

## Capability

A studio sends an invoice to its client by email; the email carries a public link the
client opens to read the invoice without an account.

## Extent

- Email through Postmark with the PDF attached and the public link in the body.
- Public link is an unguessable token; the page shows the invoice, its status, and the
  studio's bank details.
- Resend keeps the same token.
- Not: client accounts or logins (ruled 2026-07-22).
- Not: tracking whether the client opened the email.

## Relations

- depends-on: FEAT-001 — only a complete draft can be sent.
- composes-with: FEAT-003 — the public page shows the balance FEAT-003 keeps.
