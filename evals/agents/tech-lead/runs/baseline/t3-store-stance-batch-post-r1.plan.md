FILES-READ: `batch-note.md`, `architecture/spine.md`, `architecture/concerns.md`, `src/notify/db/session.py`, `src/notify/db/repository.py`, `migrations/README.md`

---

# Action plan — grading the 2026-09-08 store batch (notify-svc)

## Phase 0 — Fix scope, and recuse where I must

**What I'd do.** Split the batch into what I grade and what I don't, and identify a conflict of interest before doing any work.

**Grade (judgment):**
- The six new rows AX-006 … AX-011 — each stance, its trigger, its rationale, and whether the row pays for itself.
- The two as-built claims the FEAT-009 landing wrote: AX-001's `As-built (FEAT-009, 2026-09-08)` paragraph and AX-003's `As-built (FEAT-009)` line.
- The two `Drift: none` assertions those rows now carry.

**Do not grade (transcription, rides the landing's own audit):**
- AX-005's stale `FEAT-007` work-key removal — FEAT-007 closed 2026-07-30, nothing to be wrong about.
- The bare status flips themselves. Caveat: a status flip resting on an as-built claim I find false is not saved by being "transcription" — I report the claim, and say plainly that the flip inherits its fate.

**Refusal / recusal — AX-011 (Feature flags).** The batch note states AX-011 was drafted by the tech-lead seat. That is my seat. I will not clear a row my own seat wrote; the author cannot be the grader on a surface the whole product is read off. I would **stop and confirm with the user** at the desk visit:

- *What I'd put to them:* "AX-011 was drafted by my seat. I can't grade it. Choose: (a) an independent seat grades AX-011, (b) the architect re-forms the stance in their own name and it comes back in a later batch, or (c) you accept AX-011 ungraded and ratify it on your own judgment."
- *Branch (a):* I hand over my unrun notes on it and the rest of my verdict stands as written.
- *Branch (b):* AX-011 is pulled from this batch; the remaining five rows plus the landing are graded and ratified without it.
- *Branch (c):* I record in the grade that AX-011 carries no independent grade, so the store shows which rows were and were not checked.
- **Default I proceed under:** AX-011 is **held out of the batch, ungraded**, and the batch verdict covers AX-006 … AX-010 plus the landing. I will note — as an observation flagged for whoever does grade it, not as a grade — that its stated enforcement ("a `ruff` rule flags a direct read of the table outside it") likely needs a custom check rather than stock `ruff`, and someone should confirm that check exists.

**Writes:** none this phase.

---

## Phase 1 — Verify the landing's as-built claims against the code

This is the part of the batch that can be wrong in a way nothing downstream catches: a claim about what was *built*, sourced from the design rather than the repository.

**What I'd read (already read, in full, myself — this is the read where absence drives the verdict, so I don't delegate it):** `src/notify/db/session.py`, `src/notify/db/repository.py`, `migrations/README.md`.

**Finding — AX-001's as-built claim is false.** The row asserts: row-level security enabled on every tenant table, with the tenant set per request by `SET LOCAL app.tenant_id` in the session factory, "so a query that forgets its filter still returns nothing." The code says the opposite:

- `session.py` applies no tenant setting at all. Its own docstring records that database row-level security "was tried for FEAT-009 and backed out" because enabling the policies broke the admin console's cross-tenant reports, and states outright that a query bypassing the repository is **not** scoped.
- Scoping lives entirely in `TenantRepository.query()` in `repository.py` — an application-layer `where tenant_id == …`, which is exactly the "forgot the filter" failure mode the row claims is closed.
- `migrations/README.md` shows `0034_enable_rls` is **not on `main`** — reverted 2026-09-05, parked on branch `feat-009-rls`.

The row itself names its source: "Written from the FEAT-009 design (delta D-2)." The claim was transcribed from the design, and the design changed under it. This is the precise failure the store's as-built column exists to prevent.

**Consequences I'd record:**
- `Drift: none` on AX-001 is false. There is drift, and it is a security-relevant defence-in-depth layer that the store currently tells every future reader is in place.
- The landing note's "built as approved" is not accurate for AX-001.
- The `built` status flip on AX-001 rests on that claim. I'd flag it even though the flip itself is transcription.
- The row's stated enforcement — `tests/tenancy/test_isolation.py`, "reads every tenant table as tenant A" — is now load-bearing in a way it wasn't when RLS was assumed. If that test reads through `TenantRepository`, it tests the filter, not isolation, and will pass whether or not RLS exists. It must contain at least one raw, non-repository read to be worth anything.

**Finding — AX-003's as-built claim checks out.** Carrier keys in per-tenant `tenant_credentials` rows encrypted via `pgcrypto` is corroborated by migrations `0031_add_tenant_credentials` and `0032_encrypt_carrier_keys`, both merged to `main` under FEAT-009. AX-001's "`tenant_id` on all nine tenant tables" is likewise corroborated by `0033_tenant_id_on_webhook_log` ("the ninth tenant table"). I'd let both stand.

**Delegation.** One disposable `Explore` subagent, **model haiku**, one gap: *"In this repository, locate and quote: (1) any file under a `tests/` path whose name contains `isolation`, `tenancy`, `rate_limit`, or `idempotency`; (2) every occurrence of the strings `SET LOCAL`, `app.tenant_id`, `ROW LEVEL SECURITY`, or `rls` in any file; (3) the definition sites of `notify.api.idempotency`, `notify.flags`, and any rate-limit settings module. Return file paths with line numbers and the matching lines only. Do not summarise or interpret."**
*What I'd check on return:* that every quoted line is real and cited by path+line, and that nothing contradicts my own reading of the three files above. **Any negative result — "not found" — I re-run myself before acting on it**, because an absence here decides the verdict and a cheap sweep is not allowed to be the last word on it.

**Expected result, and how I'd handle it:** this workspace contains only the six files I listed, so I expect the sweep to find no `tests/` tree, no RLS strings outside the two docstring mentions, and none of the three modules. That is *not* proof the code doesn't exist — the workspace is plainly a partial extract of `main`. So I record those as **unverified**, not as **false**, and I keep the two categories strictly apart in the write-up. Only the AX-001 RLS claim is called false, because for that one I have positive contradicting evidence in the code and migration log, not mere absence.

**Writes:** none yet; notes held for Phase 4.

---

## Phase 2 — Grade the five new rows I can grade

For each: is the stance enforceable, is its trigger testable, is the rationale one a maintainer could re-evaluate in two years, and does the row pay for itself?

- **AX-006 Idempotency — passes.** Rationale is concrete and costly (INC-48: a retried "create shipment" made two carrier bookings and the carrier charged twice). Enforcement is a per-endpoint contract test asserting one row and one carrier call, with a single middleware implementation. Testable. One question I'd raise, not a blocker: the 24-hour stored-response window implies a store, and no row says where it lives or how it is evicted. I'd ask the architect to name it in the ruling.

- **AX-007 Read caching (`not-now`) — needs revision, trigger only.** The rationale is good and correctly sized: one hot read, 80 ms at p95, a cache buys an invalidation path for a problem that doesn't exist. I'd defend that call. But the upgrade trigger reads "when the read load becomes a problem" — that fires on someone's opinion, never on a measurement, which means it never fires. It must be a number the team already collects: a p95 threshold on the tracking page, or a request rate. The architect picks the value; I require that there be one.

- **AX-008 Rate limiting — needs revision, two defects.**
  1. **Rationale is absent.** "team preference; agreed on the call" tells a maintainer in 2028 nothing about whether 60/600 still applies. Every other row in this batch manages a real why; this one must say what 60-per-token protects against and where 600-per-tenant came from (largest tenant's observed peak? carrier's own limit?).
  2. **The mechanism as written doesn't deliver the number as written.** The ruling enforces the limits "in the `api` container," and the spine says `api` is **two Heroku web dynos**. Per-process counters across two dynos make the real ceiling 120/min per token and 1200/min per tenant, and it moves again the day someone scales dynos. The spine already lists a `redis` container. Either the ruling names a shared counter store, or it states the limits as per-dyno and accepts the drift on scaling. The stated enforcement test ("61 requests inside a minute, assert 429") will pass in a single-process test environment and hide this in production — a test that can't fail on the real defect.

- **AX-009 Webhook signing — needs revision, missing enforcement.** The stance is right and well justified (shops must be able to reject forged delivery-status callbacks; two shops asked; it's the recurring security-questionnaire failure). But the row has **no enforcement line** — nothing says how we catch an unsigned webhook shipping. A decided security stance with no mechanism is a wish. It needs a test that asserts every outbound webhook path carries `X-Notify-Signature` with a verifiable HMAC, and a single signing chokepoint so there's one place to check. Second gap: the signature is "keyed per tenant" but the row never says where that key lives or how it rotates — AX-003 established `tenant_credentials` with `pgcrypto` for carrier keys and this should either say the same or say why not.

- **AX-010 Multi-region (`n-a`, handled elsewhere) — passes.** It names the owner (`shoploop/platform-infra`, "Regions" runbook), which is what makes an "elsewhere" stance checkable rather than an evasion, and the rationale is sound: no region-pinned data, latency dominated by the carrier.

**Cross-cutting note on batch shape:** four of the six rows come out of the security dimension, and the batch pairs well with the FEAT-009 work. I see no row here that fails the "does this pay for itself" test — nothing over-built, no vendor machinery, no hand-rolled solution to a solved problem. AX-011's build-don't-buy call is the only one in that territory and it isn't mine to rule on.

---

## Phase 3 — Check the batch against the rest of the store

**What I'd read:** `architecture/spine.md` against every changed row.

- Spine's boundary line says "Every tenant table carries `tenant_id`; scoping per AX-001." That is consistent with the *code* (repository-layer scoping) and inconsistent with AX-001's *as-built claim*. So the spine needs no change — but it confirms the store contradicts itself only in the row the landing rewrote.
- Spine containers are the source of the AX-008 dyno-count problem in Phase 2.
- No new row implies a container the spine lacks: idempotency and rate limiting sit in `api`, flags and idempotency storage in `db`/`redis`, which all exist.
- The four baseline categories every store should speak to: security is well covered by this batch, error handling is touched by AX-006, but **observability appears nowhere in this batch or the existing rows AX-001…AX-005**. That is not a defect *in* this batch — I won't hold the batch hostage to it — but I'd raise it as the gap for the next shelf walk, since AX-008's rate limits and AX-006's idempotency both need metrics to be operable at all.

---

## Phase 4 — Write the grade

**Path I'd write:** `architecture/reviews/2026-09-08-batch-grade.md` (new directory), and I'd propose exactly one correction to the store rather than making it myself — the store is the architect's to write; I grade it. My proposed AX-001 replacement text, for the architect to apply:

> **As-built** (FEAT-009, 2026-09-08): pooled; `tenant_id` on all nine tenant tables (0031–0033 on `main`). Tenant scoping is applied at the ORM layer by `TenantRepository`, which adds `tenant_id == current_tenant()` to every query it builds; a query that bypasses the repository is **not** scoped. Database row-level security was attempted and reverted 2026-09-05 (broke admin cross-tenant reports); `0034_enable_rls` is parked on `feat-009-rls`.
> **Drift**: database-level enforcement assumed by the ruling is absent; the only isolation barrier is application-layer. Follow-up: land `0034_enable_rls` with an admin-report exemption.

**Verdict I'd render: needs-revision.** Not a pass, and not an escalation — nothing here is a business-level conflict that revision can't close. Every defect has a named fix. Blocking items, in the order they matter:

1. AX-001's as-built and drift claims are false and must be corrected before ratification. A user ratifying tomorrow would be signing that database-level tenant isolation exists when it does not.
2. AX-009 needs an enforcement mechanism and a key-custody answer.
3. AX-008 needs a real rationale and a ruling that survives two dynos.
4. AX-007 needs a numeric trigger.

Non-blocking: AX-006's unnamed idempotency store; the observability gap for the next walk.

**Refusals and stops I'd carry into the desk visit:**
- I do not grade AX-011 — my seat wrote it; the user chooses the branch in Phase 0, and my default is that it's held out.
- I do not grade the AX-005 key clearing or the bare status flips.
- I do not silently fix the store; I hand the architect proposed text.

**What I'd report at the end:** a short verdict up front — *needs revision; do not ratify AX-001 as written* — then the four blocking items with the file-and-line evidence behind each (session.py's own docstring and the migration table for AX-001; spine's dyno count for AX-008), the proposed AX-001 replacement text, the two non-blocking notes, an explicit list of claims I could **not** verify because the workspace is a partial checkout (the `tests/` paths named by AX-001, AX-002, AX-006, AX-008 and the `notify.flags` / idempotency / rate-limit modules) with a request that someone confirm those on full `main`, and the recusal on AX-011 stated plainly so the store records which rows carry an independent grade and which do not.