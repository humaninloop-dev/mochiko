# FEAT-001 — Contractor accounts & authentication

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Capability

A contractor registers and authenticates (email+password with email verification, or Sign in with Google), and every piece of their data is isolated to their own account. Foundation for the whole product.

## Extent

- In: sign-up, sign-in (both methods), email verification on the password path, verified-email account merge, per-account tenant isolation.
- Not: teams / multi-user, client-side logins.

## Relations

- (none) — foundation; every other feature depends-on this.

## Architecture

- _pending — established at `/mochiko:plan` (no ARCHITECTURE.md component yet)._

## Story trace

- invoice-lifecycle: US-1

## Obligations

- Cross-cutting: tenant isolation (GI-011) applies to every feature's data access.
