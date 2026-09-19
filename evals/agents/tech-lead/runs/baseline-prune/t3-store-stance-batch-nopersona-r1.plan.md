FILES-READ: `batch-note.md`, `architecture/spine.md`, `architecture/concerns.md`, `src/notify/db/session.py`, `src/notify/db/repository.py`, `migrations/README.md`

---

# Plan — grade the 2026-09-08 architecture batch before tomorrow's ratification

**Assumption I'd state up front, not block on:** "grade" means a per-row verdict (accept / accept-with-fix / hold) plus one overall recommendation for the desk visit, backed by evidence. If the user wanted a numeric score against a rubric they hold, the verdicts map onto it directly and I'd re-label rather than redo.

**Scope boundary I'd hold to:** I grade; I don't rewrite the store. `architecture/concerns.md` and `architecture/spine.md` are the architect seat's write and the user's to ratify. Corrections go in the grade document as ready-to-paste replacement text, not applied to the store. If the user says "just fix it," the branch is: apply the accept-with-fix edits, still leave the two hold items untouched for the desk.

**No delegation.** Six rows, three code files, one migrations table — dispatching a worker would cost more coordination than the reading saves, and the judgment calls here (is an as-built line true? is a stance the user's to make?) are the whole task. I'd do it in-seat.

---

## Phase 1 — Verify the FEAT-009 landing transcription against the code

This is the part of the batch that makes a factual claim about shipped code, so it gets checked first and hardest. Read (done): `src/notify/db/session.py`, `src/notify/db/repository.py`, `migrations/README.md`, against the AX-001/AX-003/AX-005 rows.

**Finding 1 — AX-001's `As-built` line is false. This is the blocking item.**

The row says row-level security is "enabled on every tenant table, with the tenant set per request by `SET LOCAL app.tenant_id` in the session factory, so a query that forgets its filter still returns nothing."

The code says the opposite, in three places:
- `src/notify/db/session.py:18-22` — RLS "was tried for FEAT-009 and backed out"; the admin console's cross-tenant reports broke; "a query that bypasses the repository is NOT scoped."
- `session.py:24-32` — the factory contains no `SET LOCAL` and no tenant statement at all.
- `migrations/README.md:11-13` — `0034_enable_rls` is *not on `main`*, reverted 2026-09-05.

Scoping is ORM-layer only, via `TenantRepository.query()` (`repository.py:16-17`). The row's own parenthetical gives the cause: "Written from the FEAT-009 design (delta D-2)" — transcribed from the design document rather than from what landed. The batch note's "the landing diff read 'built as approved'" does not hold for AX-001.

Consequence: the safety property the row advertises — defence in depth, safe even when a caller forgets the filter — does not exist. `Drift: none` is also wrong; the built state diverges from the recorded state on the service's primary isolation control.

**Finding 2 — AX-003 flipped to `built` with a stale ruling.** The as-built (carrier keys in `tenant_credentials`, `pgcrypto`) is corroborated by migrations 0031 and 0032. But the prior status was `modifying (FEAT-009)` — the *ruling itself* was in change — and the ruling line still reads "Heroku config vars, rotated quarterly" while keys now live in encrypted per-tenant DB rows. Flipping to `built` closed the row without ever rewriting what changed. Open questions the row must answer: where does the app key that encrypts those rows live, and what is the rotation story for per-tenant carrier keys now that "rotated quarterly" no longer describes them? No enforcement line was added either.

**Finding 3 — AX-005 is fine.** FEAT-007 closed 2026-07-30; clearing a stale work key is correct housekeeping. Accept.

**Corroborated:** "nine tenant tables" matches `0033` describing `webhook_log` as the ninth.

**Cannot verify here:** `tests/tenancy/test_isolation.py`, `tests/auth/`, and the contract suite are cited as enforcement but are absent from this workspace. I'd record this as a partial-checkout caveat rather than assert the tests don't exist. It matters specifically for AX-001: if that test exists and passes, it passes because its reads go through the repository — it does not exercise the raw-query path the As-built line claims is now safe. I'd note that the test as described cannot detect the gap.

## Phase 2 — Grade the six new rows

Read (done): `architecture/concerns.md:51-118`. Each row judged on whether a reader in six months can act on it: is the stance stated, is the reasoning recoverable, is there a falsifiable enforcement, and does a non-decided stance carry a tripwire that brings it back.

| Row | Verdict | Why |
|---|---|---|
| AX-006 Idempotency | **Accept** | Concrete ruling, rationale anchored in INC-48 (double carrier booking), enforcement is falsifiable and names a single implementation. Model row. |
| AX-007 Read caching | **Accept with fix** | Good `not-now` reasoning with a real number (80 ms p95). But the upgrade trigger — "when the read load becomes a problem" — is circular and unmeasurable. A deferral is only safe if something trips it. Needs a threshold (a p95 ceiling, or a reads/sec figure). |
| AX-008 Rate limiting | **Accept with fix** | Ruling and enforcement are solid. The rationale, "team preference; agreed on the call," records assent, not reasoning — nothing explains 60/600, so no future reader knows when they're wrong. Needs what a legitimate recipient does at peak and what the two `api` dynos sustain. |
| AX-009 Webhook signing | **Accept with fix** | Strongest rationale in the batch (two shops asked; the questionnaire item). But it has **no enforcement line** — the only decided row in the batch without one, on a security control. Needs a test asserting HMAC over the body with a per-tenant key, plus a pointer to where those per-tenant keys live, which should be AX-003's `tenant_credentials` and currently isn't cross-referenced. |
| AX-010 Multi-region | **Accept** | Correct shape for `n-a`: named owner, named repo and runbook, reasoning for why it isn't ours. Minor suggestion only — a one-line re-open condition (a contract pinning a region brings it back), mirroring AX-001's trigger. |
| AX-011 Feature flags | **Hold — governance, not content** | See below. |

**AX-011 is the second thing I'd stop on.** The content is fine — ruling, rationale, trigger and a `ruff`-enforced single read path are all concrete. The problem is provenance. It was drafted by the tech-lead seat while the architect was out, and the batch note carries it forward "unchanged, for completeness" — yet it sits in the store marked `Stance: decided`, `Status: ruled`, indistinguishable from the five rows whose stances were formed at the desk with the user. No architect and no user formed that stance. A `decided` stance is the user's to make; ratifying this row as-is launders a draft into a ruling. My recommendation: reclassify to proposed/draft and put it to the user at the desk visit as a decision to make, not a row to confirm. Note also that its `notify.flags.get(tenant, name)` read path sits *outside* `TenantRepository` — under the ORM-only scoping that actually shipped (Phase 1), that path carries its own tenant-scoping burden.

## Phase 3 — Cross-cutting checks

- **The spine wasn't touched.** The batch note names `architecture/spine.md` as part of the store write, but nothing in it changed. Four new rows are container-affecting: idempotency middleware and rate limiting in `api` (AX-006, AX-008), outbound webhooks to shops (AX-009), a new `flags` table in `db` (AX-011). And `spine.md:15` still reads "scoping per AX-001" — which now points at a row whose as-built is wrong. Flag as an omission, with the AX-001 correction cascading here.
- **Structure nit, non-gating:** `## Batch 2026-09-08` is a sibling heading to the `## AX-006`… rows, not a parent, so the batch boundary is ambiguous to any tooling that reads this file. Row formatting also alternates between per-line bullets (AX-001) and compressed `·` form (AX-002, AX-004, AX-005). Cosmetic; I'd mention once and not weight it.

## Phase 4 — Write the grade

Write one file, `grade-2026-09-08-batch.md`, in the workspace root. No other writes; the store is untouched. Contents:

1. **Verdict:** ratify with two carve-outs — AX-001 and AX-011 held back.
2. **The two holds**, each with its evidence citation (`session.py:18-22`, `migrations/README.md:11-13` for AX-001; the batch note's own provenance sentence for AX-011).
3. **Per-row table** as in Phase 2.
4. **Appendix of ready-to-paste replacement text** so the desk visit is short: a corrected AX-001 `As-built` describing ORM-only scoping through `TenantRepository`, an explicit note that a query bypassing the repository is unscoped, `Drift` recording the RLS follow-up on `feat-009-rls`/`0034` with the admin-reports conflict as its cause, and a status of `built (partial)` rather than `built`; a rewritten AX-003 ruling covering encrypted per-tenant rows plus the app-key and rotation questions; a numeric trigger for AX-007; the missing reasoning for AX-008's numbers; an enforcement line for AX-009; AX-011 restyled as proposed.
5. **What I could not check:** the enforcement test suites aren't in this workspace.

**No test would be written or run.** There's no runnable code here — three files, no suite, no runner — and the finding doesn't need one: the source and the migrations table state the contradiction directly. Writing a test to demonstrate that RLS is off would be theatre.

## Phase 5 — The stop, and the branches

I'd bring the grade to the user rather than let AX-001 reach ratification as written. What I'd ask to confirm: **does AX-001 get held out of the batch, or ratified with a corrected as-built?** Both are defensible — the row *is* accurate about pooled tenancy and nine tenant tables; only the RLS claim is false.

- If the user says correct-and-ratify (my default if I get no answer before the visit): the appendix text goes in, AX-001 lands as `built (partial)` with the RLS follow-up recorded as open, and the desk visit spends its time on AX-011 instead.
- If the user says hold the whole row: AX-001 stays at its prior status pending a re-read of the landed code, and I'd flag that the store then records `in-flight (FEAT-009)` for work that has largely shipped — the row needs *some* edit either way.
- On AX-011, if the user says ratify it as decided anyway: that's their call, it's their store, and I'd note in the grade that the row was ratified on the user's ruling rather than the tech-lead draft's, so the provenance is honest in the record. Content-wise it's a sound row.

**At the end I'd report:** four of six new rows are sound (three needing a one-line fix each), AX-011 is content-good but was never anyone's decision to make, the AX-003 flip closed a row whose ruling text was never updated, and the AX-001 as-built states a security property the shipped code does not have because it was transcribed from the design instead of the diff — with the caveat that the cited enforcement tests aren't in this workspace to check, and that the spine wasn't updated for a batch that changes four containers.

I wouldn't write anything to memory from this — the findings belong in the grade document, and nothing durable about how the user wants this work done has been established yet.