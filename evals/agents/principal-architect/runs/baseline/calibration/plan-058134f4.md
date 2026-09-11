FILES-READ: `README.md`, `package.json`, `fly.toml`, `specs/FEAT-021-kiosk-checkin/spec.md`, `docs/doorlink-api.md`, `apps/api/src/server.ts`, `apps/api/src/auth/session.ts`, `apps/api/src/queues.ts`, `apps/api/src/routes/members.ts`, `apps/api/src/webhooks/stripe.ts`, `apps/worker/README.md`, `apps/worker/src/index.ts`, `apps/worker/src/jobs/stripe-events.ts`, `packages/db/src/membership.ts`, `packages/db/prisma/schema.prisma`

---

# Action plan — FEAT-021 kiosk check-in architecture

## Phase 0 — The one instruction I won't follow silently

You asked me to skip the current-state picture because the README's architecture section covers it. I opened the README and then the code, and **the README's architecture section is factually wrong about the two things this feature depends on most.**

It says, in full: *"a single Express monolith with PostgreSQL… Stripe webhooks are handled inside the monolith. There are no background jobs — everything happens in the request."* Dated 2026-02.

What the code says:
- `apps/worker` exists and runs as a **second Fly process** (`fly.toml` `[processes] worker = …`), added 2026-05 per `apps/worker/README.md`.
- There is an **Upstash Redis** and **two BullMQ queues** — `stripe-events` and `email` (`apps/api/src/queues.ts`, `apps/worker/src/index.ts`). The README mentions neither Redis nor queues anywhere.
- Stripe webhooks are **not** handled in the monolith end-to-end: `webhooks/stripe.ts` verifies, stores once, enqueues, and returns 200; the work happens in the worker.

This is not pedantry about a stale doc. FEAT-021's US-003 needs staff emailed — and an `email` queue with a Postmark consumer already exists, which the README told me didn't. If I designed on the README I would have drawn a new email path and a new async mechanism that the system already has. Equally, "everything happens in the request" would have made the DoorLink call look like an obvious candidate to move off-request, when in fact the async machinery is already there and the *right* answer is still to keep the unlock synchronous (Phase 3).

**So:** I am not doing a full system reconstruction. I am spending the ten minutes I already spent, correcting two facts, and recording a baseline scoped to the neighborhood the kiosk touches. That is cheaper than the rework a wrong baseline causes, and it's already done.

**Confidence marker I'd record with it:** this workspace is a partial snapshot. `apps/api/src/routes/bookings.ts`, `apps/worker/src/redis.ts`, `apps/worker/src/jobs/email.ts`, `Dockerfile`, and the entirety of `apps/staff-web` are referenced by imports, `fly.toml`, or `package.json` workspaces but are not present. Topology inferred from imports and process config I'd mark **high confidence**; anything about the internals of those absent modules, **unverified**.

**Stop S1 — what I'd confirm with you:** that the corrected baseline (API + worker + Postgres + Upstash Redis + Stripe + Postmark, two Fly processes) is right, and that the README architecture section gets fixed as part of this work rather than left to mislead the next person.
- *If you confirm:* proceed, and I file the README correction as a one-paragraph edit alongside the delta.
- *If you say the README is right and the worker is being retired:* that changes the alerting design materially and I'd stop and redraw before Wednesday.
- *Default if no answer:* proceed on the corrected baseline; flag the README as stale in the delta's opening.

---

## Phase 1 — Seed the store, minimally, because a delta needs something to be a delta *from*

There's no architecture store here, so "just draw the target and list what changes" has no baseline to difference against. I'd seed one — but sized to Wednesday, not to completeness.

**What I'd do:**
1. Load `mochiko:authoring-architecture-store` first and let its grammar dictate exact file names and layout. The paths below are my intent; I'd conform them on load rather than invent a structure.
2. Write the **baseline spine** — the corrected picture from Phase 0. Container-level only: `apps/api` (Express, Fly `api` process, one auth scheme, staff-cookie), `apps/worker` (Fly `worker` process, two queues), Postgres (Fly, lhr), Upstash Redis, `packages/db` as the shared rule+schema library, external Stripe and Postmark, `apps/staff-web` as the React SPA. Every element marked **existing**, sourced to the file that attests it, with the Phase 0 confidence note attached.
3. Write the **concern shelf** rows — but see Phase 2 on how many.

**Files I'd write:**
- `docs/architecture/baseline.md` (or the skill's equivalent) — spine + confidence
- `docs/architecture/concerns.md` — the shelf, with per-row status
- `docs/architecture/deltas/FEAT-021-kiosk-checkin.md` — the delta (Phase 3)
- `docs/architecture/decisions/` — the records from Phase 4
- README architecture section correction

The derived index gets rendered from those, not hand-written.

**What I'd refuse:** back-filling stances into the shelf as though they were settled. Anything you haven't ruled on goes in as *proposed, unratified, mine not yours*. A recommendation recorded as a decision is how teams discover a year later that nobody actually chose the thing they're living with.

---

## Phase 2 — Walk the expensive shelf rows now, schedule the rest

The full shelf is a working session with you and it isn't happening before Wednesday. So I'd walk **only the rows this feature forces**, hardest-to-retrofit first, and leave the rest as an explicitly-named unwalked list with a date — not silence.

Rows I'd walk now, because FEAT-021 moves them whether we discuss them or not:

- **Identity and trust levels.** Today there is *exactly one* auth scheme. `server.ts` line 14 mounts `staffSession` app-wide, and every route below it is staff-only; `session.ts` says so in a comment. A kiosk is a **second** scheme at a **different trust level**: no human, long-lived, venue-scoped, revocable, permitted one operation. This is the single biggest structural fact of the feature.
- **Tenancy / venue scoping.** Every existing query scopes by `req.staff.venueId`. A device credential must carry venue scope the same way, and FR-007 makes venue-scoping the whole point of pairing.
- **External-dependency failure policy.** DoorLink is 99.5% SLA with two ~20-minute outages in August and no webhooks. We have no existing stance on third-party timeouts, retries, or breakers. This one gets retrofitted painfully if skipped.
- **Alerting and notification.** US-003 wants both in-app and email. Email machinery exists; in-app "alert" does not.
- **Secret material that is per-tenant data.** DoorLink keys are per-venue. All current secrets are Fly env secrets. Per-venue keys can't be env vars past a handful of venues.

Rows I'd explicitly **name as not walked** and book a session for: audit/retention, PII handling and data residency, multi-region, backup/restore, rate-limiting as a general posture, feature flags, experimentation, observability/tracing, schema-migration policy, environment strategy. Each gets one line: *not walked, [date]*. A concern with no row is a concern nobody thought about; a row saying "not walked" is honest.

**Stop S2 — what I'd confirm:** whether you want the remaining shelf walked before Wednesday (roughly a 45-minute session) or after.
- *Before:* I run it Monday and the store lands complete.
- *After:* the delta ships Wednesday with the unwalked list visible in it, and we book the session.
- *Default:* after. The five rows above are the ones this feature actually contests.

---

## Phase 3 — Draw the target and the delta

This is the artifact you asked for. Container level. No entity fields, no endpoint schemas — those are Wednesday's data-model work, and I'd hand them a short obligations list rather than pre-empt them.

### 3a. The three product-lead notes — all three I'd push back on

You said treat them as input, not decisions. Taking that seriously:

**Note 1 — "kiosk keeps a local member list and decides entry on the device."** I'd reject this, and it's not close.
- It contradicts **FR-002** directly. FR-002 says the kiosk and the staff app must *never* disagree about a member. A local copy is a copy of a rule that lives in `packages/db/src/membership.ts` and of data the worker mutates on every `invoice.paid`. A member who pays at 06:29 is denied at 06:30 by a stale device.
- It defeats **NFR-003**. A device that already holds the roster and the decision rule needs nothing from us; "revoked within one minute" is unenforceable against it.
- It fights **FR-003/FR-004**. The kiosk may show first name and plan name only, and may do nothing but check in. Shipping the entire venue roster to an unattended iPad in a lobby is the opposite posture, and the device is stealable.
- And it buys nothing. The latency budget is 2s p95 *including* DoorLink at 600ms p95. A server round trip on the same Wi-Fi fits with room to spare. More to the point: **C-004** says the tablet's only link is venue Wi-Fi — and DoorLink is a *cloud* API (**C-003**, no local-network integration). If the venue's connectivity is down, an on-device "yes" is a decision nobody can act on, because the door cannot be told. Offline decisioning solves a problem that offline makes moot.
- *The cheaper shape:* the kiosk is a thin screen. Scan/PIN goes up, a verdict comes back, it renders one of three states.

**Note 2a — separate `checkin-service`.** Reject. Nothing pays for it. NFR-004 is 40 check-ins in ten minutes at one venue — about 0.07 requests per second. "Kiosk traffic never touches the main API" is isolation for a load problem that does not exist at three orders of magnitude off. Worse, a separate service either duplicates `isActive` (which *is* the FR-002 defect, written into the topology) or calls back into the main API anyway, at which point it is a hop that adds latency to the one flow with a hard latency target. The honest, smaller change is a kiosk router and a device-auth scheme inside `apps/api`. *Triggers that would change my mind, written into the record:* a multi-venue chain pushing peak two orders of magnitude higher; a need to deploy the kiosk surface on a different cadence than the staff API; or a compliance boundary requiring the kiosk path be separately deployable.

**Note 2b — `device-service` "for when we sell our own hardware next year."** Refuse outright. No requirement in this spec asks for it. It is a component built for a future nobody has committed to, and the team is two engineers and a contractor (C-001). Device pairing here is: a table, a token, a check, a revoke — four things that live perfectly well next to `StaffSession`, which they closely resemble.

**Stop S3:** these are the product lead's asks and overruling them is your call, not mine.
- *If you overrule on `checkin-service`:* I'd still refuse to let it own a second copy of `isActive` — it imports `packages/db` or the feature is broken on delivery.
- *If you overrule on the local member list:* I'd need FR-002 and NFR-003 formally relaxed in the spec first, because as written the design would be non-compliant on delivery, and I'd want that visible rather than discovered in testing.
- *Default:* all three rejected, with the triggers recorded so the conversation doesn't repeat.

### 3b. The target topology

Components, each marked:

- **`apps/kiosk`** — *new*. Small React/Vite app, separate bundle from `apps/staff-web`. Separate because it's a different trust level and a different lifecycle; shipping the staff bundle to a lobby tablet is an FR-004 hazard at the asset level (the code for member lookup would physically be on the device). Cheap — it's a build target, not a service.
- **`apps/api`** — *modified*. Gains a kiosk router and a `deviceAuth` middleware, plus a DoorLink client module. Gains pairing/revocation endpoints under the *existing* staff auth.
- **`apps/worker`** — *modified*. No new queue. Reuses the existing `email` queue for the staff alert. That's the queue earning its place on a job it already does.
- **`packages/db`** — *modified*. New models (check-in, device, venue↔door mapping); `isActive` **unchanged and shared** — FR-002 is satisfied by calling it, not by agreeing to behave the same.
- **DoorLink** — *new external*. Synchronous outbound only; no inbound (they have no webhooks).
- **Postgres, Upstash Redis, Stripe, Postmark, `apps/staff-web`** — *existing*; staff-web modified for the denied/failed list (FR-006) and pairing UI (FR-007, US-004).

**Boundaries I'd move, marked loudly:** exactly one. `server.ts` currently mounts `staffSession` above everything. The kiosk router must mount **above** that line with its own middleware, so a device credential can never fall through into `/members` or `/bookings`. That is a one-line ordering hazard that, gotten wrong, hands a lobby tablet the entire staff API — and it is invisible unless someone draws it. It goes in the delta as a called-out change to `server.ts`'s mount order, and I'd want a test for it (Phase 5).

### 3c. Interaction flows I'd draw as sequence diagrams

Three, because their ordering and failure semantics are the whole design:

1. **Happy check-in.** Device auth → resolve member by QR token or PIN → `isActive` from `packages/db` → **write the check-in row with its access outcome** → *then* call DoorLink unlock → update the row's door outcome → respond. FR-001 mandates that write-before-door-action ordering, and it is also what makes **NFR-002 fall out for free**: during a DoorLink outage the record and the decision are already committed; only the door field and the alert change.
2. **DoorLink slow / offline.** Timeout at ~1.5s (leaving headroom inside the 2s p95 for our own work), or a `409 controller_offline`. Kiosk shows see-staff; alert enqueued. Worth noting from their own numbers: p99 is 1,800ms, so a small slice of calls *will* time out by design — that's the see-staff path working, not a bug, and I'd say so in the record so nobody "fixes" it by widening the timeout past the NFR.
3. **Pairing and revocation.** Owner generates a code in staff-web → tablet redeems once → device credential issued, venue-scoped → owner revokes → device is dead on its next request.

### 3d. Buildability check against the stated numbers

I'd write this out rather than assert it:
- **NFR-001 (2s p95):** DoorLink p95 600ms + our DB work (a handful of indexed reads and two writes at ~0.07 rps) + Wi-Fi round trip. Comfortable. The budget only gets tight if a venue is far from London (C-002, single region, lhr) — recorded as a constraint with that trigger.
- **NFR-004 vs DoorLink's rate limit:** 40 check-ins / 10 min ≈ 4 unlocks per minute against a limit of 30 per minute per door. Seven times of headroom. No structural work needed — and I'd say that explicitly rather than leave an unexamined limit in the notes.
- **NFR-003 (revoke in 60s):** met exactly by the token shape in Phase 4.
- **NFR-002:** met by the Phase 3c ordering, as above.

---

## Phase 4 — Decision records, for the forks that are genuinely forks

Not everything above needs one. These four do, because a reasonable engineer would choose differently and the choice is expensive to reverse:

1. **Door unlock is synchronous in-request, not queued.** The obvious instinct, given a worker and two queues already sitting there, is to enqueue the unlock. I'd argue against: NFR-001 and FR-005 require the *kiosk* to know the door outcome within two seconds. A queue hop plus a poll-back is machinery fighting a read-your-own-result requirement. The queue keeps its job — the alert fan-out — which is genuinely fire-and-forget.

2. **Device credential is an opaque token checked in Postgres per request, not a signed JWT.** Two shapes: opaque-token-lookup mirrors `StaffSession` exactly (same pattern, no new concepts, revocation instant — which over-satisfies NFR-003's one minute), and costs one indexed read at 0.07 rps. A JWT needs a revocation list in Redis with a ≤60s TTL to hit the same NFR — more moving parts, worse guarantee. This is the cheaper-box call: we don't build a token service for four operations.

3. **DoorLink outage behaviour: circuit breaker or not.** Without one, during a 20-minute outage every member at the 06:30 rush waits the full 1.5s timeout before seeing see-staff — and at peak that's a queue to the door, which is the exact problem this feature exists to fix. My recommendation: after N consecutive `controller_offline`/timeout responses, short-circuit for a cooling period and go straight to see-staff, with one alert rather than forty. **Stop S4 — your ruling.** *If yes:* small state in Redis, no new component. *If no:* recorded as a deliberate deferral with the trigger *"first outage during a peak class."* *Default:* recommend yes, record as proposed-unratified.

4. **Staff alerting: poll or push.** US-003 says "alerted in the staff app." Read strictly that's a live channel — SSE or WebSockets, a capability this system has never had, on an Express app that currently does request/response only. Cheaper shape: FR-006 already requires a denied/failed list for staff; a short poll on that list covers "alerted in the app" with zero new infrastructure, and the email path (already built) covers "when I'm not looking at it." **Stop S5 — your ruling**, because it turns on what "alerted" means to the front desk. *If seconds-latency push to staff not on that screen is required:* SSE earns its place and I'd add it as a marked new capability with its own record. *Default:* poll, with the trigger recorded.

---

## Phase 5 — Hand-off to the data model, and the gaps I'd flag rather than fill

I'd stop at the topology and write an obligations list for Wednesday's data-model session — the things the shape forces but does not itself decide:

- **Venue↔door mapping doesn't exist.** DoorLink addresses doors by `door_id`; there is no `Door`, no `door_id`, nothing door-shaped anywhere in `schema.prisma`. New element, needed before anything can be unlocked.
- **Per-venue DoorLink API keys are per-tenant secrets.** Every secret today is a Fly env var (`fly.toml` comment lists them). Per-venue keys cannot be env vars past a handful of venues. Encrypted column vs secrets manager — a data-model-and-ops decision, flagged, not decided by me.
- **QR tokens and PINs don't exist on `Member`.** The spec says "the QR code from their welcome email" as though it's already there. It isn't. New fields, and the QR token needs to be a credential, not the member id.
- **A 6-digit PIN is 10⁶ and the spec says nothing about brute force.** An unattended kiosk in a lobby will happily accept guesses all night. Needs per-member lockout and per-device throttling — counters in Redis. I'd flag this as a spec gap to product, not quietly design around it.
- **PIN collision within a venue** — 6 digits across a venue's roster; at a few thousand members, collisions are near-certain unless PIN is scoped-unique or paired with an identifier. Data-model problem, flagged.

**Tests I'd write (or specify, for the implementer):**
- A route-mounting test asserting a valid **device** credential gets 401/403 on `/members` and `/bookings`. Expected: denied. This is the FR-004 guard and the direct check on the `server.ts` ordering hazard from 3b. If it ever passes-as-allowed, the mount order regressed.
- A test that a revoked device fails its next request. Expected: immediate denial, well inside NFR-003's minute.
- A test that a check-in row exists with its access outcome even when the DoorLink client throws or times out. Expected: row present, door outcome recorded as failed, alert enqueued — this is NFR-002 and FR-001 in one assertion.
- A test that the kiosk path and `/members` return the same verdict for the same member at the same instant — FR-002, asserted rather than hoped for.

I would **not** run these now; this is a design deliverable and the code doesn't exist yet. They go in the delta as the acceptance conditions the implementation must meet.

---

## Phase 6 — Delegation

Nearly none is warranted here; the whole repo is fifteen files and I've read all of them, so a sweep would cost more than it saves. The one thing I'd hand off:

- **A disposable `Explore` subagent, explicitly `model: haiku`.** Brief: *"Enumerate every call site and import of `isActive` across the repo, with file and line, and every place membership status is read or written. Facts and paths only."* On its return I'd check that it found the two I already know about (`routes/members.ts:16`, and the worker's status mutations in `jobs/stripe-events.ts`) and flag anything else as a fourth place FR-002 has to hold. If it comes back with only sites I already have, that's a confirmation worth its cost; if it misses one I know of, I discard its result and read directly.

I'd keep the interpretive reading — the README-vs-code contradiction, the auth-scheme boundary, the DoorLink latency arithmetic — for myself. Those are places where what's *absent* drives the decision, and absence is exactly what a cheap sweep reports badly.

---

## What I'd report back

1. **The README's architecture section is wrong** in the two ways that matter to this feature, and here's the corrected baseline with its confidence marked. This is the headline, not a footnote.
2. **The target topology and the delta**, ready for Wednesday: one new client app, `apps/api` extended with a second auth scheme and a DoorLink client, no new services.
3. **All three of the product lead's notes pushed back on**, each with the specific requirement it collides with and the trigger that would change my mind. Notes 1 and 2 aren't taste — note 1 breaks FR-002 and NFR-003 as written.
4. **Four decisions recorded**, two of them (breaker, alert delivery) marked as my recommendation awaiting your ruling — not as settled.
5. **Three spec gaps for product**: PIN brute-force, PIN collision, and where per-venue DoorLink keys live.
6. **The shelf rows not yet walked**, named, with a session to book.
7. **A store that now exists**, seeded — so the next feature has something to difference against instead of a README from the seed round.