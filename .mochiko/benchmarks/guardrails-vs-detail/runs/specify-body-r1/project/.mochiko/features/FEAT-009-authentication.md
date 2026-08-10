# FEAT-009 — Authentication

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Capability

Contractors sign in to Ledgerline with email + password or "Sign in with Google," establishing
an authenticated session scoped to their own account. It is the foundation every other invoice
feature sits on; tenant isolation is enforced against this identity.

## Extent

- In: email/password sign-up and sign-in; Google sign-in; scoped session; sign-out.
- Not: enterprise SSO/SAML; client-facing login (clients touch only email + Stripe).

## Relations

- (foundation — no dependencies; ordered first, before FEAT-001)

## Architecture

- (greenfield — component realized at plan/implement time)

## Story trace

- invoice-lifecycle: US-8

## Obligations

- (none)
