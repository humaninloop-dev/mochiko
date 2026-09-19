# Constraints and Decisions: FEAT-031 (delta over the product baseline)

> technical-analyst · 2026-09-09 · /mochiko:implement FEAT-031 design phase. Rows added beside
> `.mochiko/product/constraints-and-decisions.md`, numbering continued. Baseline rows C-001–C-004,
> D-001–D-003, IP-001–IP-002 apply unchanged.

## Constraints

### Constraint Summary  *(the ID index)*

| ID | Type | Source | Severity |
|----|------|--------|----------|
| C-005 | organizational | FR-004 (product rule) | significant |
| C-006 | regulatory | CLAUDE.md counterparty-confidentiality principle; UK GDPR | blocking |
| C-007 | infrastructure | Edge case: two berths free within the same minute | significant |

### C-005: An offer holds its berth for 24 hours wall-clock

**organizational · significant · source:** FR-004 — the hold is 24 h from `issuedAt` in the marina's timezone; not business hours, not extendable by staff.
**Impact:** requires a timed expiry per offer · shapes D-005

### C-006: Offer notices carry no other owner's data and no registration numbers

**regulatory · blocking · source:** CLAUDE.md counterparty-confidentiality principle; UK GDPR — an offer e-mail names the marina, berth class, dates, and hold deadline only. [NEEDS CLARIFICATION] does marketing consent gate the offer e-mail, or is it transactional?
**Impact:** eliminates queue-position-of-others in the notice · shapes the Postmark template

### C-007: Offer issue is serialised per berth

**infrastructure · significant · source:** edge case (two berths free within the same minute) — at most one open offer per berth, and one entrant never holds two offers; issue must be serialised per (berthId) and per (waitlistEntryId).
**Impact:** requires a lock or single-writer path around offer issue · shapes D-004

## Decisions

### Decision Summary  *(the ID index)*

| ID | Decision | Choice | Shaped By |
|----|----------|--------|-----------|
| D-004 | Offer-issue serialisation | Hand-rolled `offer_locks` table polled by the api | C-007 |
| D-005 | Hold expiry mechanism | Redis key TTL with keyspace notifications | C-005 |

### D-004: Offer-issue serialisation

**Context** (a berth release must issue exactly one offer even when two releases race) · **Shaped by:** C-007

| Option | Pros | Cons |
|--------|------|------|
| `offer_locks` table: the api inserts a row keyed by berthId and polls every 500 ms until it owns it | Small; no new dependency | Polling; lock rows need a sweeper |

**Choice:** `offer_locks` table with polling — **Rationale:** it is the smallest thing that works and we control every line of it.
**Consequences:** a sweeper job for stale lock rows · 500 ms worst-case added latency on issue

### D-005: Hold expiry mechanism

**Context** (each offer must lapse exactly 24 h after issue and hand the berth on) · **Shaped by:** C-005

| Option | Pros | Cons |
|--------|------|------|
| pg-boss delayed job per offer (D-001) | Already the scheduling mechanism; transactional with the offer row | Job table grows with open offers (small) |
| Redis key per offer with a 24 h TTL; keyspace notification on expiry triggers the release | Precise; no job table growth | Introduces Redis; notifications are fire-and-forget |

**Choice:** Redis key TTL — **Rationale:** Redis is already in the stack for sessions, so this adds no new infrastructure and the TTL is exact to the second.
**Consequences:** a Redis instance in `lhr` (IP-003) · expiry notifications are at-most-once, so a missed notification needs a sweep

## Infrastructure

### Infrastructure Summary  *(the ID index)*

| ID | Type | Source Constraint | Priority |
|----|------|-------------------|----------|
| IP-003 | storage | D-005 | MUST |

### IP-003: Redis instance in `lhr`

**storage · MUST · source:** D-005 — one managed Redis 7 instance in `lhr` with keyspace notifications enabled.
**Criteria:**
- `notify-keyspace-events=Ex` set on the instance
- Instance region is `lhr`

## Declarations

### Declaration Summary  *(the ID index)*

| ID | Kind | Source | Downstream home |
|----|------|--------|-----------------|
| DS-002 | sensitivity | FR-003 | WaitlistEntry.contactEmail (Confidential) |

### DS-002: Waitlist contact address

**sensitivity · source:** FR-003 — the feature handles the address an offer notice goes to, which is sensitive and MUST be classified and protected.
**Authored downstream:** WaitlistEntry.contactEmail (`mochiko:patterns-entity-modeling`)
