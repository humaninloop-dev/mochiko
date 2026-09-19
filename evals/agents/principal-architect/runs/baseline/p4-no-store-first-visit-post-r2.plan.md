FILES-READ: `specs/FEAT-021-kiosk-checkin/spec.md`, `README.md`, `docs/doorlink-api.md`, `package.json`, `fly.toml`, `apps/api/src/server.ts`, `apps/api/src/queues.ts`, `apps/api/src/auth/session.ts`, `apps/api/src/routes/members.ts`, `apps/api/src/webhooks/stripe.ts`, `apps/worker/README.md`, `apps/worker/src/index.ts`, `apps/worker/src/jobs/stripe-events.ts`, `packages/db/src/membership.ts`, `packages/db/prisma/schema.prisma`

---

# Action plan — FEAT-021 architecture change

## Phase 0 — Load the procedures I'd write to

**Do:** Load `mochiko:authoring-architecture-store` (store layout, element grammar, what a landing flips, health view), `mochiko:patterns-system-design` (container-level altitude, flow and diagram conventions), `mochiko:patterns-technical-decisions` (decision-record shape), `mochiko:patterns-architecture-shelves` (the concern dimensions and their defaults/triggers). Every path I name below is provisional until the store skill's grammar fixes it; if it dictates different names or a different file split, its layout wins and I adjust without re-asking.

**Refuse:** nothing yet.

---

## Phase 1 — The one thing I won't do as asked: the baseline

**The instruction I'm pushing back on.** You asked me to skip current state because the README covers it. The README does not cover it. Its architecture section is dated *2026-02 at the seed round* and says two things that the code contradicts:

| README says | Code says |
|---|---|
| "Stripe webhooks are handled inside the monolith" | `apps/api/src/webhooks/stripe.ts` verifies, stores once, and **enqueues** to `stripe-events`; processing is in `apps/worker/src/jobs/stripe-events.ts` |
| "There are no background jobs — everything happens in the request" | `apps/queues.ts` (BullMQ, Upstash), `apps/worker/src/index.ts` (two workers), `fly.toml` `[processes] worker`, `apps/worker/README.md`: *"Added 2026-05 when Stripe webhook processing started timing out in the request"* |

This is not pedantry about a stale doc. The two facts the README gets wrong are **exactly the two FEAT-021 depends on**: that we already have an async lane (so staff alerting needs no new machinery — Phase 5) and that membership status is mutated *asynchronously by the worker* after Stripe events (which is the reason the product lead's "local copy of the member list" is unsafe — Phase 4). A delta drawn on the README's picture would have proposed inventing a queue we already have and would have missed the staleness window on replicated membership data.

**Cost containment:** I've already recovered the real baseline — it took the thirteen file reads above, done. I am not asking for time; I'm reporting a correction. No further current-state archaeology.

**Do:** Record the baseline in the store as **reconstructed from code, 2026-09-11**, with confidence stated per area:
- *High* — API process shape, auth scheme, queue/worker topology, deploy topology, the `isActive` rule (all read directly).
- *Medium* — `apps/worker/src/jobs/email.ts` and `src/redis.ts` are imported by `index.ts` but absent from this workspace; `apps/staff-web` is a declared workspace with no files present. I mark the email job's interface and the staff app's internals as inferred, not read.

**Delegate:** one `Explore` subagent, `model: haiku`, brief: *"In `apps/api/src`, list every `app.use(...)` and every router mount in server.ts and any file it imports, in source order, and report whether any route is registered before the `staffSession` line. Quote file:line for each. Facts only."* On return I check it found exactly the Stripe webhook above the line and nothing else — because the whole kiosk trust boundary in Phase 4 rests on that mount order, and "no other exceptions exist" is the fact I'm designing against. If it returns anything I didn't expect, I read those files myself rather than trust the summary.

**Write:** `architecture/baseline/topology.md` (+ the store's index file, rendered, never hand-kept). Baseline containers: `apps/api` (Express, single process, one cookie-session auth scheme, venue-scoped), `apps/staff-web` (React SPA), `apps/worker` (BullMQ consumer: `stripe-events`, `email`), `packages/db` (Prisma + the shared `isActive` rule), Fly Postgres, Upstash Redis, Stripe, Postmark. Fly.io `lhr`, one region, two processes.

**Also write:** a one-line correction note on `README.md`'s architecture section pointing at the store as the live picture, so the next person isn't misled the way this card was. I'd propose that edit; I won't land it without a nod.

---

## Phase 2 — Stop #1: three rulings I need before I draw

These are yours, not mine. I'll state my recommendation and the branch either way, and continue under the default so the plan doesn't stall.

**R1 — Does the kiosk decide entry on the device, or does the server?** (product lead's note 1)
My recommendation: **server decides; the kiosk is a thin client.** The decisive argument isn't latency, it's C-003 + C-004 together: DoorLink is cloud-API-only and the tablet's only link is venue Wi-Fi. When Wi-Fi is down, **the door cannot be unlocked by anything**. On-device decisioning therefore buys exactly one behaviour — a tablet cheerfully saying "welcome" to a door that will not open. It buys no working check-ins. Meanwhile it costs: a replicated member list on an unattended lobby tablet (member PII at rest in a public space, against the grain of FR-003), a sync subsystem two engineers maintain forever, a second copy of membership state that goes stale the instant the worker applies an `invoice.paid` (directly contesting FR-002's "kiosk and staff app must never disagree"), and a device that still functions for up to a sync interval after revocation (fighting NFR-003). On latency there is nothing to buy: NFR-001's budget is 2s p95 with DoorLink's 600ms p95 inside it — a LAN-to-Fly-London round trip plus one indexed Postgres read leaves well over a second of headroom.
- *If you rule "server decides"* (default): proceed to Phase 3.
- *If you rule "must feel good on bad Wi-Fi"*: the honest small version is **degraded capture, not local decisioning** — the kiosk queues the scan locally, shows "see staff," and posts when the link returns; entry is never granted offline. I'd draw that as an optional second flow and mark it P2. Costs a local outbox, nothing else.
- *If you rule "local decisioning regardless"*: I will draw it, and I will record in the decision that it knowingly contests FR-002 and NFR-003, with your name on the ruling — not mine.

**R2 — `checkin-service` and `device-service` as separate deployables?** (product lead's note 2)
My recommendation: **no to both, and the instinct behind one of them is right for the wrong reason.**
- `device-service` "for when we sell our own hardware next year" — no requirement in this spec pays for it. Cut. If hardware ships, the device tables and pairing logic are already a module and can be lifted then, at the point something actually pays.
- `checkin-service` for load isolation — NFR-004 is 40 check-ins in ten minutes at one venue. That is **0.07 requests per second**. There is no load to isolate. A second deployable for a two-engineer team costs a Fly process, a deploy lane, and — because it needs `Member`/`Membership` regardless — a shared database with a second place `isActive` could drift, which is the one thing FR-002 forbids by name.
- **But the product lead is right that kiosk traffic is different.** The difference is *trust*, not load. Today `server.ts` puts `app.use(staffSession)` above everything, and a staff session grants `GET /members?q=` — venue-wide member search by name. FR-004 says a paired kiosk may check people in and nothing else. So the boundary that is genuinely paid for is an **authentication boundary with structural separation**, and the codebase already has the pattern: the Stripe webhook is mounted *above* the session line precisely because it has a different trust story. The kiosk router mounts the same way, with its own device-credential middleware, unable by construction to reach the staff routers. That's a module boundary inside `apps/api`, not a new box.
- *If you rule "separate service anyway"*: I'll draw it, and the decision record will state the paid-for benefit as zero at current load and name the FR-002 drift risk as the price.

**R3 — Where do per-venue DoorLink API keys live?** Gap: the spec assumes a per-venue key (C-003, `docs/doorlink-api.md`), `Venue` has no key or `door_id`, and `fly.toml` holds only app-level secrets. Fly-secret-per-venue does not scale past a handful of gyms. My default: **encrypted-at-rest column on `Venue`, single app-level encryption key as a Fly secret, decrypted in-process on the unlock path.** Structural, not data-model — it decides whether we need a secrets component at all (we don't, yet). Trigger to revisit: a compliance ask, or the first partner integration that needs key rotation on a schedule.
- *If you rule "we need a real secret store"*: that adds a container and I'd want a decision record and a buildability check against C-002 before Wednesday.

**Stop behaviour:** I present R1–R3 together as one short block. Under no answer by the time I'd otherwise start Phase 3, I proceed on the defaults above and mark each stance in the store as **my recommendation, unratified** — not as a settled decision — so nobody discovers next quarter that "we decided" something nobody ruled.

---

## Phase 3 — Shelf walk, scoped honestly

There is no store, so nothing has ever been walked. I will **not** pretend a full shelf walk happened before Wednesday, and I won't do the walk silently on your behalf.

**Do:** Walk only the rows FEAT-021 actually forces, hardest-to-retrofit first, each dealt as a default + reason + the trigger that would flip it:
1. **Identity — second principal type.** Today there is exactly one: a staff user on a staff device (`auth/session.ts` comment says so explicitly). FEAT-021 introduces a *device* principal. Default: opaque device token stored in a DB row and checked per request, mirroring `StaffSession`. Reason: NFR-003 (revoked device dead within one minute) falls out for free — revocation is a row delete, effective on the very next request, which at 0.07 rps is seconds. A self-contained signed token would force ≤60s expiry plus a refresh loop, i.e. machinery bought to re-achieve what a lookup already gives. Trigger to revisit: device fleet or request rate where a per-request lookup shows up in the latency budget.
2. **Tenancy.** Venue scoping today comes from the staff session. The device credential must carry `venueId` the same way, and every kiosk query must be venue-filtered. Default: same pattern, no change in mechanism.
3. **Authorization surface.** Default: kiosk principal has exactly two capabilities — redeem-pairing-code, submit-check-in. No lookup, no list, no bookings (FR-004).
4. **Outbound partner failure policy.** First time we hard-depend on a partner *in the request path* (Stripe is inbound + async). Default: hard timeout + fail-open-to-"see staff", never fail-closed-to-silent. Detail in Phase 5.
5. **Staff notification.** Default: no realtime push. Trigger stated below.
6. **Per-tenant secrets.** As R3.

**Write:** `architecture/concerns/` rows for the six above, each marked *ratified by you* or *my default, unratified*. Plus an explicit list of shelf dimensions **not walked** (data partitioning, flags/experimentation, audit retention, multi-region, observability stance, and the rest) recorded as `not yet walked` rather than absent — so the gap is visible instead of looking like consensus. I'd propose booking a proper walk after the design review.

---

## Phase 4 — Draw the target topology

**Do:** Draw the container-level target scoped to the neighbourhood the change touches (api, worker, db, redis, the two external systems, the new kiosk client) — not the whole system redrawn. Every element classified `new` / `modified` / `existing`, with the requirement or constraint that pays for each new one named next to it. If I can't name a payer, the element comes out.

| Element | Class | Paid for by |
|---|---|---|
| `apps/kiosk-web` — Vite SPA, separate bundle from staff-web | **new** | C-001 (web app on iPad, no native); FR-003/FR-004 — a lobby tablet must not be shipped the staff app's member-search code at all |
| `apps/api` → `auth/device.ts` — device-credential middleware | **new module** | FR-007, NFR-003 |
| `apps/api` → `routes/kiosk.ts` — mounted **above** the `staffSession` line | **new module** | FR-004 (structural inability to reach staff routes) |
| `apps/api` → `doorlink/client.ts` — outbound client, timeout-bounded | **new module** | FR-005, C-003 |
| `apps/api` → `routes/devices.ts` (pair/revoke, owner-only), `routes/checkins.ts` (staff read) | **new modules, existing boundary** | FR-006, FR-007, US-004 |
| `apps/api/src/server.ts` | **modified** | mount order |
| `packages/db` — `Device`, `CheckIn`; `Venue` gains door id + key ref; `Member` gains QR/PIN credential | **modified** | FR-001, FR-007 — *shape only; columns are the data model's job, Phase 7* |
| `packages/db/src/membership.ts` `isActive` | **existing, unchanged, and deliberately reused** | FR-002 — this is the mechanism by which kiosk and staff app cannot disagree |
| `apps/worker` — one new job name on the **existing** `email` queue | **modified** | US-003 (email when staff aren't looking) |
| `queues.ts` | **unchanged** — no new queue | nothing pays for one |
| Upstash Redis | **existing**, new use: PIN-attempt counters | brute-force containment on a 6-digit PIN at a public tablet |
| Fly Postgres, Postmark, Stripe, `fly.toml` processes | **existing, unchanged** | — |
| DoorLink cloud API | **new external** | C-003 |

**Net new deployables: zero.** Net new processes in `fly.toml`: zero. The kiosk bundle ships as static assets from the existing `api` process or the existing static host, whichever staff-web uses.

**Refuse, explicitly and on the record:** `device-service`, `checkin-service`, any new queue, any WebSocket/SSE tier, any on-device member replica (pending R1). Each gets a one-line "removed because nothing pays for it, here's the cheaper shape" entry rather than vanishing silently.

**Write:** `architecture/deltas/FEAT-021-kiosk-checkin/topology.md` with current → target and a marked change list. Diagrams in the store skill's convention (Mermaid unless it says otherwise), deltas visually marked, no structural change presented as a silent redraw.

---

## Phase 5 — Draw the three flows where ordering and failure actually matter

**F1 — Check-in, allowed (the NFR-001 flow).** Kiosk POSTs scan → device middleware resolves device → member credential → load membership → `isActive(...)` → **write the `CheckIn` row with the decision (FR-001: recorded before any door action)** → synchronous `POST /v1/doors/{id}/unlock` under a hard ~1.4s abort budget → update the row with the door outcome → respond to the kiosk with first name + plan name only (FR-003).

**Interaction-style call, with the arithmetic on the page:** the unlock is **synchronous from the request**, not queued. DoorLink p95 600ms, p99 1800ms (measured from Fly London — the same region we deploy in, so the numbers transfer). Budget: ~200ms for our own work + 1400ms DoorLink cap = under 2s at p95 with room. Queueing it would add enqueue + worker pickup latency *and* strand the kiosk with no path to the outcome short of a polling loop — machinery invented to recover an answer we were already holding. DoorLink's rate limit (30 unlocks/min/door) against NFR-004 (40 per ten minutes ≈ 4/min) is ~7× headroom, so there is no throttling reason to queue either. This is the one genuine fork in the feature and it gets a decision record.

**F2 — Denied, or door failure (NFR-002).** Denied → `CheckIn` written with reason, no DoorLink call, kiosk says see-staff. Door failure — timeout, `409 controller_offline`, `429`, or 5xx — → `CheckIn` updated to `door_failed` with the reason, **enqueue the staff-alert email on the existing `email` queue**, kiosk says see-staff. NFR-002 is satisfied structurally because the record and the decision are both complete *before* the partner is touched; a DoorLink outage costs us the door and nothing else. The timeout is the containment — a 20-minute DoorLink outage (two happened in August) at 0.07 rps cannot exhaust the API process, so no circuit breaker is bought yet; I'll state the trigger that would buy one (kiosk traffic rising far enough that 1.4s-held requests matter, or a second in-request partner dependency).

**Staff alerting stance (US-003).** Default: the staff app **polls** the denied/failed list (`routes/checkins.ts`), plus the async email. No realtime tier. Reason: nothing in the spec states a notice latency; the front desk is metres from the door and the member is standing there. Trigger to revisit and buy push: a stated requirement for sub-10-second in-app notice while the app is open. I'll put this on the table as a default rather than assume it — it's the kind of row that gets retrofitted expensively.

**F3 — Pairing and revocation (FR-007, NFR-003).** Owner generates a single-use, short-TTL pairing code in the staff app → tablet redeems it once at the kiosk endpoint → server issues a long-lived opaque device token bound to `venueId` → revoke deletes/disables the row → the very next kiosk request 401s. I'd state plainly in the flow that this meets "nothing within one minute" with orders of magnitude to spare, and *why* — because the check is a lookup, not a signature.

**Buildability check before I commit any of it:** every element maps to C-002 (Fly one region, Upstash, Fly Postgres — nothing new provisioned), C-001 (web only — nothing needs a native capability), C-003 (cloud API only — no local-network path assumed anywhere), C-004 (Wi-Fi-only — and F1/F2 degrade to see-staff, which is the only honest behaviour given the door is also cloud-reached). Team size two engineers + a contractor: zero new deployables, zero new infrastructure, one new front-end bundle the contractor can own. If any of these fails I stop and say the shape isn't buildable rather than shipping a drawing.

---

## Phase 6 — Decision records

**Write** to `architecture/decisions/`:
1. **Unlock is synchronous in the request, not queued** — alternatives: queue + kiosk polling; fire-and-forget. Weighed against NFR-001's arithmetic, NFR-002, and the rate-limit headroom.
2. **Kiosk is a trust boundary inside `apps/api`, not a separate service** — alternatives: `checkin-service`; kiosk routes under the existing staff session. Weighed against NFR-004's actual load, FR-002's no-drift rule, and FR-004's capability restriction. Records the rejection of `device-service` as speculative.
3. **Device credentials are opaque, DB-checked, not self-contained tokens** — weighed against NFR-003.
4. *(Only if R1 comes back "local decisioning")* — **entry decided on-device**, recorded with your ruling and the FR-002/NFR-003 consequences named.

Each records the trigger that would reopen it. I won't manufacture a fourth record for choices that had no real fork.

---

## Phase 7 — Handoff to the data model, and what I report

**Write:** `architecture/deltas/FEAT-021-kiosk-checkin/handoff.md` — the questions the data model must answer that I deliberately did **not** answer, because they're below my altitude but I found them and won't let them fall:
- 6-digit PIN: uniqueness scope (per venue or global?), collision policy as a venue grows past a few thousand members, hashing, and lockout after N failures — a public tablet plus a 10⁶ keyspace is a brute-force surface.
- QR payload: rotating or permanent? A permanent code in a welcome email is forwardable forever.
- `CheckIn` retention — it's an attendance record and a partial access log.
- Whether `Device` revocation is a delete or a tombstone (audit wants the latter).
- Per-venue DoorLink `door_id` + key storage, per R3's ruling.

**Report at the end:**
1. **The README's architecture section is wrong in the two ways that matter to this feature** — here's the corrected baseline, recovered from code, and it cost thirteen reads, not a workstream.
2. **The target adds zero new services and zero new infrastructure.** The real boundary the feature needs is a trust boundary in-process, and the codebase already has the pattern for it in the Stripe webhook mount.
3. **Three of the product lead's notes resized:** `device-service` cut as speculative; `checkin-service` cut as solving a load problem that doesn't exist at 0.07 rps, replaced by the auth boundary that solves the problem he actually sensed; on-device decisioning recommended against — because with a cloud-only door on venue Wi-Fi, it can only ever grant entry to a door that won't open.
4. **Open rulings R1–R3**, with what I proceeded on and the fact that unratified defaults are marked as mine, not yours.
5. **The store now exists but is barely walked** — six concern rows recorded, the rest explicitly listed as unwalked, and a proper shelf walk proposed for after Wednesday.
6. Ready for the data model; handoff questions above.