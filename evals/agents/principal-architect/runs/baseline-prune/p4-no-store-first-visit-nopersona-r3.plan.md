FILES-READ: `README.md`, `package.json`, `fly.toml`, `specs/FEAT-021-kiosk-checkin/spec.md`, `docs/doorlink-api.md`, `apps/api/src/server.ts`, `apps/api/src/auth/session.ts`, `apps/api/src/routes/members.ts`, `apps/api/src/webhooks/stripe.ts`, `apps/api/src/queues.ts`, `apps/worker/src/index.ts`, `apps/worker/src/jobs/stripe-events.ts`, `apps/worker/README.md`, `packages/db/src/membership.ts`, `packages/db/prisma/schema.prisma`

---

# Action plan — FEAT-021 architecture change

Deliverable: a target architecture picture plus a change list, written down where the team can find it again, in time for the design review on Wednesday 2026-09-16. No current-state section (README §Architecture stands, and the product lead asked us to skip it). No data model — that's the next card; I'd end by handing it a list of what it must carry.

## Phase 1 — Ground the target in what's actually here (done during planning)

Already read: the spec, the DoorLink notes, and the code that the kiosk path will touch — `apps/api/src/server.ts:14` (one global staff-session gate, Stripe mounted above it), `apps/api/src/auth/session.ts` (one auth scheme, cookie + DB lookup, venue comes from the session), `packages/db/src/membership.ts:5` (`isActive`, the rule FR-002 pins us to), `apps/api/src/queues.ts` + `apps/worker/src/index.ts` (BullMQ producers/consumers, `stripe-events` and `email`), `fly.toml` (two processes, `lhr` only), `packages/db/prisma/schema.prisma` (no device, check-in, or credential concepts today).

Arithmetic I'd check before writing anything, since the whole design rests on it:
- NFR-001 budget: DoorLink p50 310 ms / p95 600 ms / p99 1,800 ms from Fly London. A server-side decision costs one kiosk→Fly round trip (venue Wi-Fi, LHR) + one indexed Postgres read + the DoorLink call. p95 comfortably inside 2 s; p99 is the risk, which is why the unlock call gets a hard client timeout below the budget rather than a retry.
- NFR-004 vs the rate limit: 40 check-ins / 10 min = 4 unlocks/min/door against a 30/min/door limit. Six times' headroom; no batching or queueing needed for capacity.

Flag I'd carry into the doc: this workspace is partial — `server.ts` imports a bookings router, and the worker imports `./redis` and `./jobs/email`, none of which are present here. I'd write the doc against the files I read and not assert anything about the ones I couldn't open, including how `apps/staff-web` is built and served (it's a workspace in `package.json` but has no process in `fly.toml`).

## Phase 2 — Stand up a minimal architecture store

There isn't one, so the card implies creating it. I'd keep it to two files, sized for two engineers and a contractor rather than a formal ADR corpus:

- `docs/architecture/README.md` — index + convention: one file per architecture change, named for its feature; decisions recorded inline in that file with the reasoning and the alternatives rejected; the README's architecture section stays the current-state description until it's wrong.
- `docs/architecture/FEAT-021-kiosk-checkin.md` — the change itself.

Diagrams in Mermaid inside the markdown (renders in GitHub and in the editor; no binary assets to keep in sync).

Stop point: if the team already keeps architecture somewhere I can't see (a Notion space, a wiki), this is the wrong home. I'd raise it in one line at handoff. If they say "use Notion", the content is unchanged and I'd paste it there instead; if they say "keep it in-repo", nothing changes. Default: in-repo at the paths above.

## Phase 3 — Write the target picture

Into `docs/architecture/FEAT-021-kiosk-checkin.md`, sections in this order.

**3a. Target diagram.** Components and the edges between them:
- `apps/kiosk` — new Vite/React PWA workspace, iPad in guided access. Thin: camera/QR decode, PIN pad, result screen. Holds a device token, no member data, no staff bundle.
- `apps/api` — gains a *second auth scheme* and a device-scoped route group mounted **above** the global `staffSession` line, the same way the Stripe webhook is (`server.ts:11`). Routes: pair, check-in. Plus staff/owner routes for pairing codes, revocation, and the denials/failures list, mounted below the line as normal.
- New `packages/doorlink` (or `apps/api/src/doorlink/`) — a small client: unlock, status, per-venue API key lookup, hard timeout, error classification (`409 controller_offline`, `429`, timeout → all "door failed").
- `apps/worker` — one new queue for staff alert emails; reuses the existing `email` queue rather than adding a third.
- Postgres and Upstash Redis as today (C-002). Redis additionally used for PIN attempt throttling.
- DoorLink cloud, outbound only. Marked explicitly as **no inbound webhooks** — we never learn the door opened, only that the controller acknowledged the unlock.

**3b. Check-in sequence** (the load-bearing picture): scan/PIN → `POST /kiosk/checkin` with device token → device token validated against Postgres, venue derived from the device (never from the request body) → member resolved from QR token or venue-scoped PIN → `isActive(...)` from `packages/db` → **check-in row written with the outcome, before any door action** (FR-001, and it's what makes NFR-002 true) → if allowed, DoorLink unlock with a ~1.5 s timeout → response `{ outcome, firstName, planName, doorState }` and nothing else (FR-003) → on denial or door failure, enqueue the staff alert.

**3c. How each requirement is met**, one line each, so the review can argue with the mapping rather than the prose: FR-001 write-before-unlock; FR-002 single call into `isActive`, no reimplementation and no copy; FR-003 response shape; FR-004 the device route group is the only thing a device token opens; FR-005/NFR-001 the 1.5 s timeout inside a 2 s budget; NFR-002 decision and record are independent of DoorLink; NFR-003 opaque device token checked against the DB per request, so revocation is effective on the next request (well inside a minute) — this is the reason not to use a long-lived stateless JWT; NFR-004 the headroom figure from Phase 1.

## Phase 4 — Write the change list

Same file, a table of *what changes, where*:

| Area | Change |
|---|---|
| `apps/api/src/server.ts` | Mount the device router above `app.use(staffSession)`; that line's "every route below requires a staff session" comment becomes load-bearing and gets updated |
| `apps/api/src/auth/` | New `deviceAuth` middleware alongside `session.ts`; `session.ts`'s "exactly one auth scheme" comment is now false and must change |
| `apps/api/src/routes/` | New kiosk routes (pair, check-in); new owner routes for pairing codes and revocation; new staff route for the denials/failures list (FR-006) |
| `packages/db` | New entities (Phase 6 list); `isActive` unchanged and deliberately so |
| `apps/worker` | Staff-alert email job on the existing `email` queue; `apps/worker/README.md` queue list updated |
| New `apps/kiosk` | Workspace added to `package.json` |
| `fly.toml` | No new process, no new region, no VM resize — stated explicitly so nobody assumes otherwise |
| Secrets | Per-venue DoorLink API keys and door IDs need a home; see open question below |
| `README.md` §Architecture | Amended once this ships (it currently says "no background jobs", already untrue since the 2026-05 worker) |

## Phase 5 — Record the two decisions that go against the product lead's notes

The lead marked these as input, not decisions, so this is squarely in scope. Both stated with rationale and the trigger that would reverse them.

**5a. No on-device member list; the server decides.** Against note 1. Reasons: FR-002 says the kiosk and staff app must never disagree, and a replicated cache disagrees by construction every time a payment fails or a membership is cancelled between syncs; FR-003 caps what the kiosk may show, and a full member roster sitting on an iPad at a gym entrance is a far bigger exposure than the screen it's guarding; the latency case for it doesn't hold, because the 2 s budget is dominated by DoorLink (600 ms at p95), which needs the network anyway — offline entry decisions buy nothing when the door itself is a cloud call. If Wi-Fi turns out to be the real failure mode, the honest fix is staff override, not stale local truth.

**5b. No `checkin-service`, no `device-service`.** Against note 2. Kiosk traffic is ~4 requests/min/venue against an API already doing everything else; a separate service would share the same Postgres and the same `isActive`, so it isolates nothing that matters while adding a deploy target, a second set of secrets, and a network hop inside a 2 s budget — for a team of three. `device-service` is for hardware that doesn't exist yet. What we *do* take from the note is the real requirement underneath it — kiosk traffic must not be able to reach staff functionality — and that's enforced by the separate auth scheme and route group, which is where the isolation is actually needed. Reversal trigger, written down: if kiosk traffic ever competes with staff traffic for the API VM, split it then; the route group makes that a move, not a rewrite.

**Stop point.** These two are the design review's job to ratify — I'd write them as recommendations with the reasoning visible, not present them as settled. Branches: if the lead insists on the on-device roster, I'd ask which of FR-002 or FR-003 they want to relax and rewrite the target with an explicit sync-staleness window and a reduced local payload; if they insist on the separate services, I'd add the service boundary and note the extra deploy/secret surface and the added hop against the 2 s budget. Default for the doc I'd write: as decided above.

## Phase 6 — Hand off to the data model card

A short closing section listing what the data model must carry, described as obligations rather than schema (that card designs it):

- A device: venue-scoped, revocable, with a credential we can invalidate server-side (NFR-003), and the DoorLink door ID for its venue.
- A pairing code: single use, short-lived, owner-issued (FR-007).
- A check-in: member, venue, timestamp, method QR|PIN, outcome, and the door result recorded separately from the entry decision, since FR-001 requires the record before the door attempt and NFR-002 requires them to diverge cleanly.
- Member credentials: a QR token and a PIN. Flags for that card: the PIN is 6 digits, so it needs a stored hash rather than the value, a venue-scoped uniqueness rule, and an attempt throttle (Redis, keyed by device and by member) — a 6-digit PIN with no lockout is guessable at kiosk speeds. This is the one security item I'd want raised at the review even though it's a data-model detail, because it changes what gets stored.
- Per-venue DoorLink API key storage. Open question, flagged not decided: Fly secrets are per-app, not per-venue, so this is likely an encrypted column with a key from a Fly secret. I'd put it to the review rather than pick it alone, since it sets a precedent for every future partner credential.

## Phase 7 — Check before handing over

No code changes, so nothing to test or run. What I'd verify instead: every file path and symbol cited in the doc is one I actually opened (`isActive` at `packages/db/src/membership.ts:5`, the mount order at `apps/api/src/server.ts:11`–`14`, the queue names in `apps/api/src/queues.ts`); every latency and rate-limit number traces to `docs/doorlink-api.md`; every FR and NFR in the spec appears in the requirement mapping, so nothing is silently dropped; the Mermaid blocks parse.

## Delegation

None. This is one document grounded in ~15 small files I've already read; splitting it across workers would cost more to brief and reconcile than to write. No subagents dispatched.

## What I'd report at the end

The two files written; the target in three sentences; the two product-lead notes I recommended against and why, called out as the things the review should rule on; the PIN throttling and per-venue DoorLink key questions as open; the note that `fly.toml` is unchanged; and the confirmation that the data-model card now has its input list.