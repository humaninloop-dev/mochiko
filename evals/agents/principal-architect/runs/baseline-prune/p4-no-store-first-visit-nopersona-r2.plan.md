FILES-READ: `README.md`, `package.json`, `fly.toml`, `specs/FEAT-021-kiosk-checkin/spec.md`, `docs/doorlink-api.md`, `apps/api/src/server.ts`, `apps/api/src/auth/session.ts`, `apps/api/src/queues.ts`, `apps/api/src/routes/members.ts`, `apps/api/src/webhooks/stripe.ts`, `apps/worker/src/index.ts`, `apps/worker/src/jobs/stripe-events.ts`, `apps/worker/README.md`, `packages/db/prisma/schema.prisma`, `packages/db/src/membership.ts`

---

# Action plan — FEAT-021 target architecture

## One thing I'd flag before starting

You said the README's architecture section covers current state. It doesn't — it's dated 2026-02 and says "Stripe webhooks are handled inside the monolith" and "there are no background jobs — everything happens in the request." Both stopped being true in 2026-05: `apps/api/src/webhooks/stripe.ts:20` stores and enqueues, `apps/worker` consumes `stripe-events` and `email` off Upstash Redis, and `fly.toml:12` runs a second `worker` process.

This matters for FEAT-021 specifically, because the worker and the `email` queue are exactly what US-003's "email when I'm not looking at the staff app" should ride on. If the target doc deltas against the README's picture, it will describe adding infrastructure that already exists.

**My default:** I don't produce a current-state document. I write four or five lines of accurate baseline at the top of the target doc (monolith + worker + Redis + Postgres, one Fly region, one auth scheme) purely so "what changes" means something, and I note the README is stale with a suggested one-line correction — I don't rewrite the README unasked. If you'd rather I touch nothing outside the architecture store, say so and I'll drop the README note; the baseline paragraph stays either way because the change list is unreadable without it.

## Phase 1 — Set up the architecture store (small, since none exists)

Create `docs/architecture/` alongside the existing `docs/` partner notes:

- `docs/architecture/README.md` — index: what lives here, the convention that target-state docs are per-feature and ADRs are numbered and immutable once accepted. Ten lines, not a manifesto.
- `docs/architecture/decisions/` and `docs/architecture/target/` — populated in phases 3–4.

No tooling, no diagram build step, no ADR generator. Mermaid in fenced blocks renders in GitHub and in the review.

## Phase 2 — Work the latency and constraint arithmetic (before drawing anything)

This is the analysis that decides the shape, so it happens first. Nothing to write yet; I'd sanity-check these numbers against `docs/doorlink-api.md`:

- **Budget.** NFR-001 is 2 s p95 scan→unlock-acked. DoorLink is p50 310 ms / p95 600 ms / p99 1,800 ms from Fly London. Kiosk→API over venue Wi-Fi plus a device-token lookup, a membership read and a check-in insert is tens of milliseconds. At p95 the round trip lands near 700 ms — NFR-001 has roughly 1.3 s of headroom. **A server-side decision comfortably meets the target; the on-device cache is not needed to hit it.**
- **Where it does bite.** FR-005's hard 2 s cutoff versus DoorLink's 1,800 ms p99 means roughly one valid check-in in a hundred shows "see staff" and pages the front desk. At peak (NFR-004, 40 in ten minutes) that's under one per peak window — real but tolerable.
- **Rate limit.** 30 unlocks/min/door against 40 check-ins/10 min is a 7× margin. Not a constraint unless we retry.
- **No webhooks.** DoorLink never calls us, so "the door physically opened" is not observable — we only ever know the controller acknowledged. This limit gets stated plainly in the doc rather than papered over.

## Phase 3 — Write the three decisions that overturn the product lead's notes

The notes are marked "input, not decisions," so they get answered with reasoning, in writing, ahead of the review rather than argued live.

**`docs/architecture/decisions/0001-entry-decided-server-side.md`** — rejects note 1 (local member list on the tablet).
Three independent reasons: FR-002 requires the kiosk and staff app to never disagree, and two copies of the member list with independent staleness is precisely a disagreement machine; FR-003 caps what the kiosk may show at first name and plan, and a full member roster cached on an iPad in a gym entrance is a much larger exposure than that; NFR-003's one-minute revocation is unenforceable against a device that decides locally. The Phase 2 arithmetic removes the motive. I'd also state the honest cost: C-004 says venue Wi-Fi is the tablet's only link, so a Wi-Fi outage takes the kiosk down entirely — staff fall back to the staff app, which is today's process. NFR-002 only promises DoorLink-outage tolerance, not Wi-Fi-outage tolerance, and I'd make sure the review sees that distinction.

**`docs/architecture/decisions/0002-kiosk-endpoints-stay-in-the-monolith.md`** — rejects note 2 (`checkin-service` and `device-service`).
Peak is 40 check-ins in ten minutes: 0.07 requests/second. A separate service does not buy isolation worth two engineers and a contractor, and it would put a second copy of the entry rule behind a network boundary — the one thing FR-002 forbids. `packages/db/src/membership.ts:5` is already shared by api and worker; the kiosk becomes the third caller of the same function against the same rows. `device-service` for hardware we might sell next year is a cost paid now for a maybe.
The middle option I'd put in front of the review: if isolating kiosk traffic from staff traffic genuinely matters, add a third Fly process running the same image with a different entrypoint (`fly.toml [processes]` already does this for `worker`) — process and VM isolation, no new repo, no new deploy pipeline, no duplicated rule. **Default: don't, revisit if staff-app load ever affects check-in latency.**

**`docs/architecture/decisions/0003-unlock-called-synchronously-with-no-retry.md`**
The unlock cannot go through BullMQ — the member is standing at the door and the kiosk must show an outcome. So it's an in-request call with a hard client timeout. Two numbers I'd set as defaults and flag as review decisions: **client timeout 1.6 s** inside a 2.0 s total budget, and **no automatic retry of an unlock**, because a retry can pop a door open after the member has given up and walked away, and it's the only thing that could approach the 30/min limit. Ordering per FR-001: decide → write the check-in row → call DoorLink → update the row with the door outcome, so a DoorLink failure or outage leaves a complete, correct check-in record (NFR-002).

**Stop point.** These three overturn a product decision. I'd write them, and flag in my report that they're the agenda for Wednesday. I would not block on your approval to write them — if you'd rather I present the product lead's shape as the recommendation instead, that changes all three documents and I'd want to know before the review, not after.

## Phase 4 — Write the target picture and the change list (the actual deliverable)

`docs/architecture/target/FEAT-021-kiosk-checkin.md`:

1. **Baseline** — the four-line accurate current state from Phase 1's flag.
2. **Target container diagram** (Mermaid): new `apps/kiosk` React/Vite SPA on the iPad → kiosk routes in `apps/api` (own auth scheme) → Postgres and DoorLink cloud; failures enqueue onto the existing `email` queue → worker → Postmark; `apps/staff-web` gains pairing and an alerts list, polling the API.
3. **Request walkthrough** for a scan, with the Phase 2 timings on each hop, and the branches for denied / DoorLink 409 controller_offline / timeout.
4. **What changes** — the list you asked for:
   - **New** `apps/kiosk` (contractor's build; C-001 says web app, no native).
   - **New auth scheme.** Today `apps/api/src/server.ts:14` applies `staffSession` globally with `app.use`. Kiosk routes mount *above* that line — the same pattern the Stripe webhook already uses at line 11 — with their own device-token middleware. FR-004's "check-ins and nothing else" then falls out of routing rather than out of per-route checks.
   - **Revocation.** The device middleware reads the device row from Postgres per request; no token cache. At 0.07 rps that's free and makes NFR-003 instant rather than one-minute. If anything is cached later, TTL must stay under 60 s.
   - **DoorLink client** — a small module in `apps/api`, per-venue API key, timeout and no-retry per ADR 0003, and a circuit-breaker-free design (an outage should fail fast per call, not trip global state, because NFR-002 wants check-ins to keep working).
   - **Staff alerting** — reuses `queues.email` (`apps/api/src/queues.ts:8`) and the existing worker. **No new realtime channel:** there are no websockets today, and the staff app polling a denied/failure endpoint every ~15 s satisfies US-003 and FR-006 at this scale. Adding realtime infrastructure for one alert list is not worth it.
   - **Entry rule unchanged** — `isActive` stays the single implementation; the kiosk path imports it.
   - **Fly** — no new process by default (ADR 0002); no new region; Upstash and Fly Postgres as today (C-002).
5. **Open questions for the review**, each with my default so the meeting is a ratification not a design session:
   - **Per-venue DoorLink API keys.** Fly secrets are per-app, not per-venue, so keys need to live in Postgres. *Default: encrypted column, app-level key from a Fly secret.* Worth 90 seconds of the review because it's the only new secret-handling in the feature.
   - The 1.6 s timeout and the ~1% false "see staff" rate — accept, or widen and risk overrunning FR-005?
   - Separate Fly process for kiosk traffic — default no.
6. **Handoff to the data model** — named entities only, no columns, since that's your next piece of work: a device/credential entity (venue- and door-scoped, revocable, token stored hashed), a pairing code (short-lived, single-use), a check-in record (member, venue, time, method, outcome, door outcome — FR-001), and member credentials for QR and PIN. Two notes the data-model work needs from me: **the 6-digit PIN is a 10⁶ space, so it needs hashing at rest plus per-member and per-device attempt throttling or it's trivially brute-forced from the kiosk itself**, and the **QR token must be a long random per-member value, not the member id**, which is otherwise the obvious shortcut. `Venue` in `packages/db/prisma/schema.prisma:10` currently has no door concept at all, and `Member` has no credential — both are new ground.

## Phase 5 — Self-check, then report

No code, so no test run applies. Instead: a traceability pass — every FR-001…007 and NFR-001…004 must appear somewhere in the target doc or an ADR, and I'd list the mapping in my report so you can spot a gap in one glance. I'd re-verify the DoorLink figures against `docs/doorlink-api.md` rather than trusting my own arithmetic from Phase 2, and confirm `isActive` has exactly one definition in the tree.

**Delegation: none.** This is four documents over a repo I've now read end to end; handing it to a subagent would cost more in briefing and review than doing it.

**What I'd report:** the files written; the three decisions that go against the product lead's notes and the one-line reason for each; the three open questions with my defaults; the entity list for your data-model work with the PIN and QR-token warnings; and the README staleness with the correction offered but not applied.