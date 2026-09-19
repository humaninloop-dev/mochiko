FILES-READ:
`README.md`, `package.json`, `fly.toml`, `specs/FEAT-021-kiosk-checkin/spec.md`, `docs/doorlink-api.md`, `apps/api/src/server.ts`, `apps/api/src/queues.ts`, `apps/api/src/auth/session.ts`, `apps/api/src/routes/members.ts`, `apps/api/src/webhooks/stripe.ts`, `apps/worker/src/index.ts`, `apps/worker/src/jobs/stripe-events.ts`, `apps/worker/README.md`, `packages/db/src/membership.ts`, `packages/db/prisma/schema.prisma`

---

# Plan — FEAT-021 target architecture

## Up front: one correction to the premise

You said skip current state, the README covers it. The README's architecture section does not cover it — it says "Stripe webhooks are handled inside the monolith" and "There are no background jobs — everything happens in the request," and it's dated 2026-02. Both statements are false as of 2026-05: `apps/api/src/webhooks/stripe.ts:22` enqueues to BullMQ, `apps/worker/src/index.ts` consumes `stripe-events` and `email`, `fly.toml:14` runs a second `worker` process, and Upstash Redis is a live dependency (`apps/worker/README.md`).

This matters for this card specifically, not as pedantry: the staff-alerting path in US-003 and the outage behaviour in NFR-002 both land on the worker + email queue, and the target picture is unreadable to anyone at Wednesday's review who believes the README. So I'm not writing a current-state document — I'm fixing three sentences of the README (Phase 6, ~10 minutes) and drawing only the target. If you'd rather I leave the README alone, the target doc still stands; it just carries a one-line note that the README is stale.

---

## Phase 1 — Establish the architecture store (small, not a framework)

Write two files:

- `docs/architecture/README.md` — the index and the convention: numbered records, one per architecture-changing feature, each covering target picture + what changes + open questions; Mermaid diagrams inline (no diagram tooling exists in this repo and I won't introduce any); records are amended, not rewritten, once reviewed.
- `docs/architecture/0001-kiosk-checkin.md` — the deliverable.

**Stop point (cheap to reverse):** I'm putting the store in `docs/` rather than `specs/FEAT-021-kiosk-checkin/architecture.md`, because it should outlive the feature folder and `docs/` is already where cross-cutting notes live (`docs/doorlink-api.md`). If you prefer it beside the spec, it's a file move and one link. Default: proceed with `docs/architecture/`.

I will not backfill records for the monolith, the worker, or the Stripe move. Not this card.

## Phase 2 — Settle the open decisions before drawing anything

The spec hands me three product-lead notes marked "input, not decisions." Two of them I would decide against, and the reasoning goes in the doc as a proposal for Wednesday rather than as a done deal.

**2a. On-device member list and on-device entry decision (note 1) — recommend no.**
Grounds, in order of weight:
- FR-002 says the kiosk and staff app must never disagree. A device-local replica is a replica; it disagrees whenever it's stale, and the disagreement window is exactly the Stripe-driven `past_due` transition (`apps/worker/src/jobs/stripe-events.ts:16`) that entry hinges on.
- NFR-003 says a revoked device does nothing within one minute. A device holding the roster keeps holding it after revocation. Revocation stops the door; it doesn't stop the data.
- FR-003 caps what the kiosk may show at first name and plan. Shipping the venue roster to an iPad on venue Wi-Fi to display a first name is the opposite trade.
- The latency argument doesn't hold. NFR-001's budget is 2 s p95 including DoorLink, and DoorLink is 600 ms at p95 (`docs/doorlink-api.md:7`). That leaves ~1.4 s for a round trip to Fly London and a Postgres read. On-device decision buys tens of milliseconds against those grounds.

Target instead: the kiosk is a thin client. It captures the scan or PIN, posts it, renders what comes back.

**2b. Separate `checkin-service` and `device-service` (note 2) — recommend no, with a boundary that keeps the option.**
Two engineers and a contractor, one Fly region, one Postgres. A second deployable buys isolation and costs a deploy pipeline, a second VM, cross-service auth, and a shared Prisma client that couples them anyway. `device-service` for hardware we might sell next year is speculative work on a feature that doesn't exist.

Target instead: a `apps/api/src/kiosk/` module inside the existing API process, with its own router, its own auth middleware, and no import of `membersRouter`. That's a module boundary, extractable later. If kiosk traffic genuinely threatens the API, the cheap escalation is a Fly process group running the same image with a different entrypoint — not a new service.

**Branch if product holds their line on either:** I don't relitigate. For 2a I'd add a section covering what then becomes true — device holds a hashed-credential index rather than member records, a sync channel and its staleness bound become architecture, revocation degrades to "best effort until next sync," and FR-002 gets downgraded to eventual agreement with a stated drift window — and flag that NFR-003 as written can no longer be met. For 2b I'd draw the two services, add the Fly process groups and the shared-`packages/db` coupling, and note the deploy-ordering constraint.

**2c. Three decisions the spec doesn't make, which I'd resolve in the doc:**

- **DoorLink's p99 is 1,800 ms; FR-005's cutoff is 2,000 ms.** A non-trivial slice of successful unlocks will land near or past the cutoff. So the kiosk can tell a member "see staff" for a door that then opens. My default: keep the 2 s timeout, treat it as a failure, alert staff, and do **not** auto-retry the unlock — a retry can pop the door for whoever is standing there a minute later. Flag prominently; this is a real Wednesday question.
- **We record "unlock acknowledged," not "door opened."** DoorLink has no webhooks (`docs/doorlink-api.md:11`). Naming this correctly now prevents the data model inheriting a field that can never be populated.
- **Per-venue DoorLink API keys have nowhere to live.** Fly secrets are app-wide; the key is per venue. Default: encrypted at rest in Postgres, with the encryption key as a Fly secret. This is an architecture call, so it belongs in this doc, but it constrains the data-model card.

**Two gaps I'd raise, not silently fix:** a 6-digit PIN is a 10⁶ space with no attempt limit anywhere in the spec — I'd put per-member lockout and per-device rate limiting in the target picture as a required component and note it as a spec gap. And QR codes from a welcome email are photographable and forwardable; the target assumes an opaque, regenerable, hashed-at-rest token, not a member ID.

## Phase 3 — Draw the target

Into `docs/architecture/0001-kiosk-checkin.md`. Mermaid, two diagrams:

1. **Component diagram** — kiosk web app, API process (staff routes / kiosk routes / DoorLink client), worker, Postgres, Upstash Redis, Stripe, DoorLink, Postmark. Trust boundaries drawn explicitly, because the meaningful change is that the API stops having exactly one class of caller.
2. **Sequence for a check-in** — scan → device auth → resolve credential → record check-in (FR-001: recorded before any door action) → `isActive` → respond to kiosk → unlock inline with 2 s timeout → on failure, enqueue alert. With the timing budget annotated against the DoorLink measurements.

Plus prose sections on: the second auth scheme, why the door call is inline rather than queued (FR-005's 2 s includes the ack, and there's no webhook to close the loop later) while only alerting is queued, the outage posture for NFR-002, and revocation timing for NFR-003.

**Load check I'd state so nobody over-engineers it:** NFR-004 is 40 check-ins in 10 minutes at one venue — 4/min against a 30/min per-door DoorLink limit. No queueing, batching, or throttling tier is warranted. Saying this in the doc is worth more than any mechanism.

## Phase 4 — The change list

The section you actually need for the review. Concretely:

**New**
- `apps/kiosk-web/` — React + Vite, mirroring `apps/staff-web`, served as static (C-001: no native build). Two screens, no router of consequence.
- `apps/api/src/kiosk/auth/device.ts` — device credential middleware; resolves token → venue; DB lookup per request, no cache (this is what makes NFR-003's one minute trivially true, and it's the same cost the staff session already pays at `apps/api/src/auth/session.ts:12`).
- `apps/api/src/kiosk/routes.ts` — `POST /kiosk/checkin`, `POST /kiosk/pair`. Nothing else, ever (FR-004).
- `apps/api/src/doorlink/client.ts` — 2 s timeout, no retry, per-venue key, maps `409 controller_offline` and `429` to distinct outcomes for FR-006's reason column.
- `apps/api/src/routes/devices.ts` — staff side; pairing-code generation and revocation, behind `requireRole("owner")` (FR-007, US-004).
- `apps/api/src/routes/checkins.ts` — staff side; the denied/failed list (FR-006).
- `apps/worker/src/jobs/checkin-alert.ts` — consumes the **existing** `email` queue under a new job name. I'd explicitly not add a third queue; `apps/api/src/queues.ts` gains no entry.

**Changed**
- `apps/api/src/server.ts` — the kiosk router must mount **above** line 14. Today `app.use(staffSession)` gates everything after it, which is why `apps/api/src/webhooks/stripe.ts` had to be mounted first at line 11. Kiosk routes are the second exception, and the comment on line 14 stops being true.
- `README.md` — Phase 6.
- `fly.toml` — no new process group; static hosting for the kiosk app is the only open item, and it may be nothing.

**Deliberately unchanged**
- `packages/db/src/membership.ts` — `isActive` stays the one rule and the kiosk calls it, not a copy. FR-002 is satisfied by *not* writing code. Worth stating out loud so nobody helpfully reimplements it for the kiosk path.
- No new service, no new queue, no new region, no websockets. On US-003's in-app alert: staff-web is a plain SPA with no realtime tier today; I'd poll the denied/failed endpoint rather than stand up SSE for one front desk. Email covers the not-looking case via the existing Postmark path.

## Phase 5 — Verify, then hand off

- Trace table: every US, FR, and NFR → the component or decision that carries it. Anything unmapped is a hole in my picture, and I'd fix it before publishing rather than shipping the table with gaps.
- Re-open `server.ts`, `session.ts`, `membership.ts`, `queues.ts` and confirm every path and line reference in the doc is still exact. No tests to run — this card produces documentation, and I won't manufacture a test to look thorough.
- A closing section naming what the data-model card inherits, without designing it: entities implied are device, pairing code, check-in record, member check-in credential, and a venue↔door mapping with the encrypted key; open questions are PIN uniqueness scope, credential hashing, check-in retention, and that `Member.name` is one field (`schema.prisma:38`) while FR-003 wants a first name.

## Phase 6 — README

Rewrite the three wrong sentences in `README.md:8-12`: Express monolith plus a BullMQ worker on Upstash Redis, Stripe webhooks verified in the request and processed in the worker since 2026-05, two Fly process groups. Add a forward link to `docs/architecture/`. Date it. Nothing else in that file.

## Delegation

None. This is one judgment-heavy document over a workspace I've now read end to end; splitting it across workers would cost more in reconciliation than it saves, and the decisions in Phase 2 are the whole value of the card.

## What I'd report back

The target picture and change list are in `docs/architecture/0001-kiosk-checkin.md`; the store and its convention are in `docs/architecture/README.md`; the README's architecture paragraph is corrected. Four things want a ruling on Wednesday, in order: the on-device roster (I've recommended against, with grounds), the two proposed services (recommended against, with an extraction path preserved), the 2 s cutoff against DoorLink's 1,800 ms p99 and what a member sees when a slow unlock succeeds, and the PIN attempt-limiting the spec doesn't mention. The data-model card is unblocked; it inherits five entities and four open questions, listed at the end of the record.