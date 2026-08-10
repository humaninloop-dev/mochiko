# FEAT-001 — Contractor accounts & access

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Capability

A contractor signs up and signs in to a private workspace that owns and isolates all of their data. Authentication is email+password or Sign in with Google; every read and write is scoped to the signed-in account.

## Extent

- In: registration, sign-in (email+password and Google), email-based password reset (FR-020), single linked identity when one email arrives via both methods (FR-021), per-account data isolation enforced at the data-access boundary.
- Not: enterprise SSO, teams/multi-user, a bookkeeper seat (data model must leave the door open, but no access control for it in v1).

## Relations

- composes-with: FEAT-002, FEAT-003, FEAT-005 — every other capability is scoped to an account.

## Architecture

- Components established at plan time (greenfield; no `ARCHITECTURE.md` components yet).

## Story trace

- invoice-lifecycle: US-1

## Obligations

- Cross-cutting: every tenant-owned table added by later features inherits the account-scope isolation rule (GI-011).
- Cross-cutting: SC-008 / FR-016 (WCAG 2.1 AA) binds this feature's contractor-facing screens.
