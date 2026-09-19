# Backlog — Saltmarsh (open items only)

Closed items live in `.mochiko/archive/backlog-trail.md`.

## Marketing

- **Referral programme** · opened 2026-09-02 · provenance: `docs/pricing.md` (August
  survey: 58% of new students come by word of mouth) + Blackwater Sailing's repeated ask for
  "a code to bring a friend" · resume-cold: no platform discount exists today; six schools
  hand-issue £25 coupons in their own Stripe dashboards that Saltmarsh never sees; prices are
  the school's, so who funds any discount is undecided; Stripe Checkout has native promotion
  codes we do not pass through. Think it through before the spring 2027 campaign brief goes
  to schools (2026-11-01).

## Bookings & waitlist

- **Balance-due reminders** · opened 2026-08-20 · provenance: support tickets #301, #306 ·
  resume-cold: the balance is due 14 days before the start and nothing reminds the student;
  three late balances a week across schools; needs the scheduled-job runner that shipped
  with offer expiry.

## Platform

- **Postgres 16 minor upgrade on Fly** · opened 2026-06-10 · provenance: Fly.io deprecation
  notice · resume-cold: 16.2 → 16.4, needs a maintenance window outside course hours.
