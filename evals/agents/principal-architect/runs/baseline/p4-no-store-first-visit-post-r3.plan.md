FILES-READ: `README.md`, `package.json`, `fly.toml`, `specs/FEAT-021-kiosk-checkin/spec.md`, `docs/doorlink-api.md`, `apps/api/src/server.ts`, `apps/api/src/queues.ts`, `apps/api/src/auth/session.ts`, `apps/api/src/routes/members.ts`, `apps/api/src/webhooks/stripe.ts`, `apps/worker/src/index.ts`, `apps/worker/src/jobs/stripe-events.ts`, `apps/worker/README.md`, `packages/db/prisma/schema.prisma`, `packages/db/src/membership.ts`

---

# Action plan — FEAT-021 architecture change

## Phase 0 — The one thing I have to raise before anything else

**The README's architecture section is wrong, and it's wrong in exactly the places this feature depends on.** I've already read the code, so this costs us nothing to fix — but I won't draw a target on it.

The README (written 2026-02, says so) claims:
- *"Stripe webhooks are handled inside the monolith"* — `apps/api/src/webhooks/stripe.ts` verifies the signature, stores the event, and hands off to a queue. The comment says it moved off the request path in 2026-05.
- *"There are no background jobs — everything happens in the request"* — `fly.toml` runs a second `worker` process; `apps/api/src/queues.ts` produces to `stripe-events` and `email`; `apps/worker/src/index.ts` consumes both; `apps/worker/README.md` dates the change to 2026-05.

Why this matters *for FEAT-021* and not just as tidiness: NFR-002 (keep recording check-ins through a DoorLink outage) and US-003 (email staff when they're not looking at the app) both want a durable async hop. On the README's baseline I'd have designed a queue and an email path as *new components*. They already exist, are already deployed, and already carry Postmark. That's the difference between a delta with two new boxes and a delta with zero new boxes.

**So I'm doing what you asked in substance — no current-state exercise, no reconstruction project.** The baseline is already recovered from the fourteen files above; I'll write it down as a short spine and label it reconstructed-from-code so nobody trusts it further than it's earned. Total added cost: the paragraph, not the meeting.

**Stop #1 — I'd confirm with you, in one message, not a meeting:** *the worker and the two queues exist and Stripe is already async; the README predates them. I'm seeding the baseline from the code, not the README. Agreed?*
- **If you agree (my default, and I proceed on it):** Phase 1 as written.
- **If you'd rather I just use the README anyway:** I'd decline to draw the delta against it and instead deliver the target with the queue/worker marked "existence unverified — may already exist," which is a worse artifact for Wednesday. I'd say so and still push for the two-minute version.

**Second, smaller gap in what I can see:** `apps/staff-web/`, `apps/api/src/routes/bookings.ts`, `apps/worker/src/jobs/email.ts` and `apps/worker/src/redis.ts` are all imported or listed but absent from this tree. So my confidence is high on the API/worker/DB spine and **low on the staff front end**. Since US-003 and FR-006 and FR-007 all land in the staff app, I'll mark it low-confidence rather than draw it as if I'd seen it, and I'd confirm its routing/build setup before committing the kiosk front-end packaging decision (Phase 4).

## Phase 1 — Stand up the store and seed the baseline

No store exists, so the delta has nothing to hang off. I'd create it and seed it in the same pass.

- **Load `mochiko:authoring-architecture-store` first** and follow its grammar for layout and file naming. My working assumption is a store root under `docs/architecture/` holding a baseline document, a concerns/stances register, and per-feature delta documents, with the index rendered rather than hand-written — but I take the actual paths from the skill, not from this assumption, and I'd correct them on load rather than inventing a structure that then has to be migrated.
- **Write the baseline spine** — one Fly app, `lhr` only, two processes (`api`, `worker`) off one image; Express monolith with staff cookie sessions and a Stripe webhook mounted ahead of the session gate; Fly Postgres via Prisma in `packages/db`; Upstash Redis carrying BullMQ `stripe-events` and `email`; Stripe and Postmark as external systems. Marked **reconstructed from code, 2026-09-11, high confidence on api/worker/db, low on staff-web**.
- **Record the baseline's existing stances** that FEAT-021 is about to lean on: venue is the tenant (every staff session carries `venueId`, every member and staff user hangs off `Venue`); exactly one auth scheme today (`staffSession`, per the comment in `auth/session.ts`); the entry rule lives in exactly one place (`isActive` in `packages/db`, used by both `api` and `worker`).
- **Also propose a README fix** (one paragraph, pointing at the store) — I'd flag it, not write it unasked.

## Phase 2 — Walk the shelves, but only the rows this feature forces

Full shelf walk before Wednesday isn't realistic and I won't fake one. **Load `mochiko:patterns-architecture-shelves`** and walk only the rows FEAT-021 can't be drawn without, each dealt as my recommendation plus the trigger that would change it — your call moves the row, and I record it as *yours*, not as mine. Anything unwalked gets recorded as unwalked, not as settled.

Rows I'd put on the table:

1. **Device identity as a third trust level.** Today there's one scheme. The kiosk is a second (device credential), and it is *lower trust than staff* — an iPad at an unattended entrance on venue Wi-Fi (C-001, C-004). My recommendation: device credentials are a distinct principal type with their own middleware and their own router, never a StaffUser with a funny role. Trigger to revisit: never, realistically — collapsing them is how FR-004 gets violated by accident.
2. **Revocation latency (NFR-003, one minute).** Recommendation: validate the device credential against Postgres on every kiosk request — no cache. At 40 check-ins per ten minutes (NFR-004) that's about one lookup every fifteen seconds; a cache is optimising a load that doesn't exist, and it's the thing that would break the one-minute guarantee. Trigger: device count or request rate rising enough that the lookup is measurable.
3. **Per-venue DoorLink API keys — a genuine gap, and the one I most want your ruling on.** C-003 says per-venue API key. The schema has no home for it and Fly secrets are per-app, not per-venue. `Venue` already exists with multiple rows, so this isn't hypothetical. Options: encrypted column in Postgres with an app-level key from a Fly secret (smallest, works today); or an external secret store (new dependency, more machinery than one credential type currently justifies). Recommendation: encrypted column, app-level key. This forces a *key-management stance* the product doesn't have yet, so it goes on the shelf as a new row either way.
4. **Staff alerting channel (US-003).** See Phase 3 — my position is polling, and this is the row where I expect the most pushback.
5. **Audit / retention of check-in records.** FR-001 makes every entry attempt a durable record of a person's physical presence. Nobody's said how long we keep it or who can read it. I'd flag it as a row and, if you want it deferred, record the deferral with a trigger (first data-subject request, or first multi-venue customer) rather than letting it vanish.

Rows I'd explicitly mark **not walked** for now, with a trigger: multi-region, rate limiting/abuse on the kiosk endpoint, feature flags, observability standards. Full walk scheduled post-review.

## Phase 3 — The target shape, and the three notes I'd push back on

**Load `mochiko:patterns-system-design`** and draw at container level.

### Where I'd land

**No new services.** The target is the same two Fly processes, with:

- **`apps/api` — modified.** A `kiosk` router mounted **above** `app.use(staffSession)` in `server.ts`, behind its own `deviceAuth` middleware — structurally the same trick the Stripe webhook already uses on line 11, which is a precedent already in the codebase. That router exposes check-in and nothing else, which is FR-004 enforced by topology rather than by discipline. Staff-facing pairing/revocation (FR-007) and the denied/failed list (FR-006) stay *below* the session gate as ordinary staff routes.
- **A DoorLink client module inside `apps/api` — new, but small.** One caller, so one module. I would **not** create a `packages/doorlink` adapter for a single consumer.
- **`apps/worker` — modified.** Reuses the existing `email` queue for the staff email alert. No new queue.
- **`packages/db` — modified.** `isActive` is reused unchanged; that *is* FR-002. The kiosk calling the same function in the same package is the whole mechanism by which the kiosk and staff app can't disagree. I'd call that out explicitly because it's the cheapest requirement satisfaction in the feature and it only works if nobody reimplements it.
- **DoorLink — new external system.** One outbound dependency, no inbound.

**The interaction decision (this is the one that gets a decision record).** The unlock call is **synchronous, inline on the check-in request, with a hard client timeout around 1.2s.** Order: write the check-in record and commit it (FR-001, and it's what makes NFR-002 true — the record and the decision survive a DoorLink outage because they happen before DoorLink is touched) → call unlock → update the outcome → respond to the kiosk. The alert is the only async part, enqueued to the existing `email` queue.

Budget check against NFR-001 (2s p95 including DoorLink): DoorLink is p95 600ms by our own measurements, leaving ~1.4s for iPad → venue Wi-Fi → Fly LHR and back plus three small indexed Postgres queries. From a UK venue that's tens of milliseconds of network and low tens of milliseconds of database. It holds with room. The 1.2s timeout deliberately fires on DoorLink's p99 tail (1,800ms) — cutting that off and saying "see staff" is the correct behaviour, not a failure.

Alternative I considered and would record as rejected: enqueue the unlock to the worker. It breaks NFR-001 (Redis hop plus worker pickup), and worse, it destroys the kiosk's feedback loop — the member is standing at a door and the request has already returned. Async is wrong here precisely because the coupling is real: the member cannot leave until the answer arrives.

Rate limit is a non-issue and I'd say so rather than design for it: 30 unlocks/min/door against 40 check-ins per *ten* minutes. `409 controller_offline` and `429` both route to the same see-staff-and-alert path as a timeout.

**No webhooks from DoorLink** (their doc is explicit). So there is no inbound integration component and no "door opened" event to design against — the unlock response is the only truth we get. I'd state that in the target so nobody reinvents it later.

### Three pushbacks

**Note 1 — kiosk holds a local member list and decides entry on-device: no.**
- It directly contests FR-002. Two decision points cannot be guaranteed to agree; one can.
- It contests FR-003/FR-004 harder than the spec seems to realise: a local member list is the venue's entire roster *with membership status* sitting on an iPad that US-004 already anticipates being lost.
- It makes NFR-003 (dead within a minute) very hard — you can't revoke a device that isn't asking permission.
- And nothing pays for it. The latency budget closes comfortably server-side.
- **But here's the honest version:** the one thing that could break the budget is venue Wi-Fi (C-004), and that's exactly what the product lead is worried about. So the answer isn't "no" — it's *"no, and here's the measurement that would change it."* I'd ask for RTT and packet-loss from the pilot venue's Wi-Fi to Fly LHR. If venue Wi-Fi p95 RTT plus our server time plus DoorLink p95 breaches 2s, on-device decisioning comes back on the table with a specific, bounded design (a short-lived signed entitlement per member, not a roster copy) — and FR-002 then has to be renegotiated in writing. Recorded as a stance with a live trigger, not as a "no".

**Note 2 — separate `checkin-service` and `device-service`: no to both.**
- `checkin-service`: NFR-004 is 40 check-ins in ten minutes at one venue — roughly one every fifteen seconds. There is no load to isolate. It would need the same Prisma client and the same `isActive`, so it'd be coupled through the database anyway; you'd get a second deploy unit, a second Fly VM, and a second place for the entry rule to drift, in exchange for a boundary nothing pays for. The isolation actually being asked for — "kiosk traffic never touches the main API" — is a *trust* concern, and trust is bought by the separate auth boundary and the restricted router, which I am building. That boundary is paid for. The deployment boundary isn't. **Trigger that would change it:** kiosk traffic measurably degrading staff API latency on the shared VM, or a venue-scale/multi-region change.
- `device-service`: explicitly for hardware "next year". No requirement in FEAT-021 asks for it. I'd refuse to draw it. Pairing and revocation are a handful of staff routes and a table.

**Note 3 — skip the current state:** honoured in spirit, corrected in fact. See Phase 0.

**One more thing the spec doesn't cover and the flow needs:** a member scanning twice. Without a short dedupe window you get two check-in rows, two unlock calls, and a confusing second answer on screen. Recommendation: repeat check-in from the same member within ~10 seconds returns the prior outcome without a second door call. Small, and it belongs in the flow, so I'd raise it rather than let it surface in the data model.

## Phase 4 — The one shape question I'd genuinely hand to you

**Kiosk front end: a new `apps/kiosk-web` bundle, or a route inside `apps/staff-web`?**

- **Separate bundle (my default):** FR-004's "and nothing else" is far easier to guarantee when the staff app's code isn't shipped to a device sitting at an unattended entrance. Costs a fourth workspace and a second front-end build, on a team of two engineers and one contractor (C-001).
- **Route in the staff app:** no new build, but the kiosk device receives the staff bundle, and "it can't do staff things" becomes a routing promise rather than a structural fact.

**Stop #2.** This is a real fork with a real cost on either side and it's yours, not mine.
- **If you take the separate bundle:** one new front-end component in the target, marked new.
- **If you take the shared bundle:** the target instead records a stance that FR-004 is enforced server-side only, and I'd add a note that a lost tablet exposes staff UI code — acceptable, since the server enforces authorisation, but it should be a decision somebody made rather than a thing that happened.
- **Proceeding on:** separate bundle. It's the one that makes a requirement structurally true instead of conventionally true.

I'd want the absent `apps/staff-web` confirmed (Phase 0) before I finalise this.

## Phase 5 — Flows

I'd draw three, only the ones where ordering or failure semantics actually matter:

1. **Happy path** — scan/PIN → device auth → credential lookup → `isActive` → check-in row committed → DoorLink unlock → outcome updated → welcome. Latency budget annotated on the DoorLink hop.
2. **DoorLink fails** — timeout at 1.2s, or `409 controller_offline`, or `429`. All three converge: outcome recorded as door-failed, staff email enqueued, kiosk says see-staff. This is the flow that demonstrates NFR-002, because the check-in record is already committed before DoorLink is called.
3. **Pair and revoke** — owner generates code in the staff app, tablet redeems once for a long-lived venue-scoped credential, owner revokes; the next kiosk request fails because the credential is checked against the database every time (NFR-003).

## Phase 6 — Decision record

**Load `mochiko:patterns-technical-decisions`.** One record, because one genuine fork: **synchronous inline DoorLink unlock vs. queued unlock via the existing worker.** Alternatives, the NFR-001 arithmetic, the member-standing-at-the-door coupling argument, and the consequences (timeout tuned to cut DoorLink's p99 tail; a second write per check-in; DoorLink outages degrade to door-only failure).

The rejections of on-device decisioning and of the two extra services I'd record as **stances with triggers** in the store rather than as decision records — they're "we considered and declined" with named conditions for reopening, not forks the shape turns on.

## Phase 7 — Write

- The delta document: current shape (short, from the corrected baseline), target shape, and every component classified **new / modified / existing** — `apps/api` modified, `apps/worker` modified, `packages/db` modified, DoorLink client new (inside api), `apps/kiosk-web` new (pending Stop #2), DoorLink new external, Redis/Postgres/Postmark existing and unchanged.
- Explicitly marked deltas: the new trust zone above the session gate in `server.ts`; the new outbound dependency; the new per-venue secret class. No silent redraws.
- Index rendered from what's written, never hand-kept.

## Phase 8 — Handoff to the data model

The topology forces things onto the schema, and the schema as it stands (`packages/db/prisma/schema.prisma`) has none of them. I'd hand over the requirements without drawing the model — that's the next artifact, and it's drawn to fit this shape, not the other way round:

- A device/pairing record, venue-scoped, revocable, with a pairing-code lifecycle (FR-007, NFR-003).
- A check-in record: member, venue, time, method, outcome, reason — written before the door action (FR-001, FR-006).
- Member entry credentials: QR token and a PIN that must be stored hashed, not as the six digits.
- A home on `Venue` for the DoorLink `door_id` and an encrypted API key reference — **blocked on the Phase 2 ruling.**

## Tests

Nothing to run — this phase produces architecture, not code. Two checks I'd propose *into* the plan for whoever builds it, because they're the ones that would catch a topology violation rather than a logic bug: a test asserting that a device credential gets 401/403 on every staff route (FR-004 enforced, not assumed), and a test that a revoked device fails the very next request (NFR-003). I'd also ask for the pilot venue Wi-Fi measurement from Phase 3 — that's the one number the whole latency stance rests on.

## Delegation

Honestly: **almost none is warranted here.** Fourteen files, all read, and the interpretive weight — whether the README is trustworthy, whether the product lead's services are paid for — is exactly the kind of judgement I shouldn't farm out. Where I *would* spawn a throwaway `Explore` on the cheap tier:

- **Enumerate every call site of `isActive` across the repo.** Brief: return file and line for each import or call, nothing else. On return I'd check it's a complete sweep of both `apps/` and `packages/` and that it caught the two I already know about (`routes/members.ts`, and the comment's claim about the worker — which I want verified, since I've read `stripe-events.ts` and it *doesn't* actually call `isActive` despite the comment in `membership.ts` saying the worker uses it; that discrepancy is worth a line in the store).
- **Locate the staff-web build config and router entry**, if the directory turns out to exist outside this snapshot. Brief: return the build tool, entry point, and route table. On return I'd check it actually found files rather than guessing from the README's layout section — an empty return here is itself the answer, and it changes Phase 4.

Both are locate-and-list with no judgement in them. Everything else I do myself.

## What I'd report at the end

1. **The README is stale and here's the corrected spine** — the worker and both queues have existed since May; two of the feature's requirements ride on machinery you already own, so the delta adds zero new infrastructure components.
2. **The target: no new services.** A restricted kiosk router behind its own device-auth boundary inside the existing API, a small DoorLink client, and the existing email queue for alerts.
3. **Three things the product lead asked for that I'm not building** — the on-device member list, `checkin-service`, `device-service` — each with the reason and each with the specific trigger that would reopen it. The Wi-Fi measurement is the live one.
4. **Two decisions I need from you**: per-venue DoorLink key storage, and kiosk front-end packaging. Both have stated defaults I've proceeded on.
5. **What's on the shelf and unwalked**, including check-in data retention, which I'd rather you defer deliberately than discover later.
6. **What the data model now has to provide**, ready for Wednesday.