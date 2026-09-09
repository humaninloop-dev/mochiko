FILES-READ: `batch-note.md`, `architecture/spine.md`, `architecture/concerns.md`, `src/notify/db/session.py`, `src/notify/db/repository.py`, `migrations/README.md`

---

# Action plan — grading the 2026-09-08 architecture store batch

## Phase 1 — Fix the scope of what I actually grade, and recuse where I must

**Do:** Split the batch note's contents into three piles before touching any of it.

- **Grade (judgment):** AX-006 Idempotency, AX-007 Read caching, AX-008 Rate limiting, AX-009 Webhook signing, AX-010 Multi-region — five new stances. Plus the two claims about what was *built*: the AX-001 `As-built`/`Drift` lines and the AX-003 `As-built` line. A statement about what exists in the code is exactly the kind of claim that can be wrong in a way nothing downstream catches, so it gets graded whether or not the note calls it "transcription."
- **Do not grade (transcription):** the AX-001 `in-flight → built` and AX-003 `modifying → built` status flips as *flips*, and the clearing of the dead `FEAT-007` key from AX-005. These carry no judgment; they ride the landing's own audit. I'll say so explicitly in the report rather than silently skipping them, so nobody reads the omission as approval. (Note the flips are downstream of the as-built claims I *am* grading — if a claim fails, the flip it rests on is back in play, and I'll say that.)
- **Refuse — AX-011 Feature flags.** The row is signed "tech-lead seat (drafted during the walk)." That is my seat. I authored it; I cannot clear it. I will not grade it, will not soften it, and will not wave it through on the grounds that it looks fine — it does look fine, and that is precisely the reasoning that makes author-grades-self worthless. It comes out of this batch's approved set and needs a different grader.

**Stop for the user, at the top of the report so it isn't buried:** *AX-011 needs a grader who is not me — either the principal-architect seat grades it (they didn't write it, so that works), or it is held back from tomorrow's ratification and lands in the next batch.*
- If they choose **architect grades it** → the rest of my report stands as written, AX-011 carries a separate line from that seat.
- If they choose **hold it back** → AX-011 is struck from the ratification set; nothing else changes.
- If they say **"you wrote it, you know it best, just grade it"** → I decline and hold the row back. That is the default I plan under: **AX-011 excluded, unratified.**

**Read in this phase:** already done — `batch-note.md`, both store files.

## Phase 2 — Verify the FEAT-009 as-built claims against the code

This is the phase that decides the batch. The store's AX-001 as-built line makes four separate factual assertions, and it says outright it was "written from the FEAT-009 design (delta D-2)" — written from the design, not from the build. That sentence is the tell, and I check every clause of it against `main`.

**Already established from my own reads (I do this one myself — absence here drives the verdict, so it isn't delegable):**

| AX-001 claim | Code says |
|---|---|
| pooled, `tenant_id` scoping | Holds — `TenantRepository.query()` filters on `tenant_id` |
| `tenant_id` on all nine tenant tables | Holds — migration `0033` adds it to `webhook_log`, named as the ninth |
| "PostgreSQL row-level security enabled on every tenant table" | **False.** `0034_enable_rls` is *not on `main`* — reverted 2026-09-05 after admin reports broke, parked on `feat-009-rls` |
| "tenant set per request by `SET LOCAL app.tenant_id` in the session factory" | **False.** `session.py` does no such thing; its own docstring says RLS "was tried for FEAT-009 and backed out" |
| "a query that forgets its filter still returns nothing" | **False, and inverted.** `session.py`: "a query that bypasses the repository is NOT scoped" |
| `Drift: none` | **False.** The drift is exactly the gap above |

**Delegations (cheap, one gap each, disposable read-only helpers on the small/fast model, findings-with-file-and-line only):**

1. Search the whole tree for `ENABLE ROW LEVEL SECURITY`, `SET LOCAL`, `app.tenant_id`, `set_config` — confirm nothing on `main` implements RLS by another route before I call the claim false. *On return:* if it comes back with a hit outside `feat-009-rls`, I read that file myself before concluding anything.
2. Locate `tests/tenancy/test_isolation.py` and quote what it asserts and how it obtains its session. *Why:* AX-001's enforcement claims this test "reads every tenant table as tenant A and asserts zero rows, runs in CI." If the test reads through `TenantRepository`, it passes while proving nothing about a raw query — the enforcement is weaker than the row advertises. If the test doesn't exist, the enforcement line is fiction. *On return:* I read the file myself if it exists; a claimed-but-absent test is a finding I won't take secondhand.
3. Enumerate model classes carrying `tenant_id` in `notify/db/models` — confirm "nine."
4. Grep for direct `session.execute` / `session.query` / `select(` on tenant models outside `repository.py` — how wide is the unscoped bypass in practice? This sizes the drift for the report: a documented-but-unused escape hatch and a dozen live bypasses are different severities.
5. For AX-003: locate where the `pgcrypto` app key is read from and confirm carrier keys are in fact read from `tenant_credentials` rather than config vars. *Why:* migrations `0031`/`0032` are merged and dated, so the claim is well-corroborated at the schema level, but "moved out of config" is a code claim.

**Expected result:** claims 1–2 confirm the RLS half of AX-001 is unbuilt; 3 confirms the `tenant_id` half is built; 5 confirms AX-003's as-built holds.

**Finding I will write (the blocking one):** *AX-001's as-built line describes the FEAT-009 design, not the FEAT-009 build.* The security property the row promises — defence in depth, where a forgotten filter still returns nothing — is the one property that was reverted. What shipped is single-layer ORM scoping where a bypass returns **every tenant's rows**. Anyone reading this store to decide whether a raw query is safe would be misled into a cross-tenant data leak by the store itself. And `Drift: none` is not merely incomplete; it asserts the absence of the exact gap that exists.

The batch note's "The landing diff read *built as approved*" is therefore also wrong, and I'll name it — it tells me the landing was checked against the approved design instead of against `main`, which is the process defect that produced this, and it will produce it again.

**Required revision (concrete, not "please fix"):**
- As-built reduced to what is on `main`: pooled; `tenant_id` on nine tables; scoping applied solely by `TenantRepository`; no database-level enforcement.
- `Drift:` records the deferral — RLS reverted 2026-09-05, `0034_enable_rls` on `feat-009-rls`, admin cross-tenant reports the blocker, and the standing consequence that a query outside the repository is unscoped.
- A tracked follow-up carries the RLS work, and AX-001's `Work:` points at it. A drift with no owner becomes permanent.
- Status: I do **not** insist on reverting `built` — the ruling as written (pooled, row-level scoping) *is* built. But it's the architect's call whether "built" is honest with the depth layer missing, and it must be made knowingly rather than inherited from a false as-built line. **Stop:** if the architect judges the row genuinely `built`, the corrected drift text is sufficient; if they judge the ruling to have included the RLS layer, status returns to `in-flight` against the follow-up. **My default for the report: `built` with corrected as-built and drift.**
- AX-001's enforcement line is rewritten to match what the test actually does (pending delegation 2) — if it exercises only the repository path, it must say so, and adding a raw-query case is the cheap fix that would have caught this.

## Phase 3 — Grade the five new stances

Each row must survive three questions: is there a mechanism that catches a violation, can I state pass/fail, and can a maintainer in two years evaluate whether the reason still holds. Plus: does the row earn its keep.

- **AX-006 Idempotency — pass.** Enforcement is a per-endpoint contract test asserting one row and one carrier call; pass/fail is unambiguous; the rationale carries a real incident (INC-48, double carrier booking, real money). Single named implementation. No revision. This is the row the others should look like.

- **AX-007 Read caching — revise the trigger.** The not-now stance is well-argued (80 ms p95, one hot read, invalidation cost unpaid) — I'd keep it. But "when the read load becomes a problem" is not a trigger; nobody can ever say it fired. Replace with the measurable the rationale already implies — a named p95 threshold on the tracking page sustained over a stated window, or a request-rate ceiling — and name where it's watched. Otherwise this stance never gets revisited until an outage revisits it.

- **AX-008 Rate limiting — revise the rationale.** Enforcement and testability are fine (61 requests, expect 429, both limits in one settings module). The rationale is "team preference; agreed on the call," which is a record of *who* agreed, not *why*. Two specific numbers are being made binding; a maintainer in 2028 asked to raise them has nothing to evaluate. Needs the basis — observed peak legitimate traffic with headroom, a capacity figure, or the abuse case being priced. If the honest answer is "we guessed at roughly 10× observed peak," write that; it's evaluable and it's fine. **Question to the architect for the record:** what were the numbers derived from?

- **AX-009 Webhook signing — revise: no enforcement.** Rationale is the strongest in the batch (two shops asked, it's the recurring security-questionnaire failure) and the ruling is precise. But there is no mechanism that catches a violation — no test, no single signing path named. This is a signature nobody verifies is present; a new webhook emitter added next quarter ships unsigned and nothing notices. Needs a test asserting a valid `X-Notify-Signature` on every outbound webhook, plus a named single signing helper. Note this is a security-floor obligation, so it isn't optional structure — it's a required stance missing its teeth. Also note the per-tenant key ties into AX-003's credential storage; I'll flag that the key's storage and rotation should be stated in one of the two rows, not neither.

- **AX-010 Multi-region — pass.** A "handled elsewhere" stance needs a named owner and a locatable artifact, and it has both (`shoploop/platform-infra`, "Regions" runbook), with a rationale for why the ownership sits there. Nothing to enforce; nothing to test. Correctly scoped out.

**Complexity/excess check across the batch:** nothing here is over-built for a 40-tenant, three-person service. AX-006 and AX-008 both name a single implementation point, AX-007 actively declines machinery, AX-010 declines ownership. One thing I'd raise as a question rather than a finding: AX-006's idempotency middleware and AX-009's HMAC signing are both categories with mature library solutions — needed is not the same as worth hand-writing and maintaining forever. Both are small and defensible in-house; I'd ask the architect to confirm the build-vs-adopt call was made rather than defaulted, and record the answer. Not blocking.

## Phase 4 — Consistency across store and code

**Do:** Read the new rows against the spine and against each other, since the failure I care about lives between artifacts, not inside one.

- Spine lists containers `api`, `db`, `worker`, `redis`. AX-008 enforces limits "in the `api` container" — consistent, but rate-limit counters at 60/min per token across **two web dynos** need shared state. Redis is present, so this is buildable; I'll flag that the row should say where the counter lives, or the limit is per-dyno and effectively 120/min. That's a between-artifacts conflict the row alone doesn't reveal.
- AX-006's 24-hour stored-response window implies storage on a tenant table — which, given Phase 2, sits behind repository-only scoping. Worth a line so the follow-up RLS work knows about it.
- Spine's boundary line ("Every tenant table carries `tenant_id`; scoping per AX-001") is now only as strong as AX-001's corrected text. If AX-001's drift is recorded, the spine's boundary claim should not read as stronger than it is. I'd propose a one-line spine amendment — architect's write to make, not mine.
- AX-005: my scope is the stale-key cleanup only, which is transcription. I note without grading that the row has no `Enforcement` line; that's pre-existing, not this batch's problem, and I won't expand the batch's scope over it. Logged as an observation for a future walk.

## Phase 5 — Report

**Would write:** `architecture/reviews/batch-2026-09-08-grade.md` (not written in this run).

**Verdict: needs-revision — do not ratify as submitted tomorrow.**

Contents, in order:
1. **Recusal notice** — AX-011 is my seat's work, ungraded, excluded from the ratifiable set pending a different grader.
2. **Blocking:** AX-001's as-built and drift lines assert a security property that was reverted before merge. Full clause-by-clause evidence table with file and migration citations, the exact replacement text, and the follow-up-work requirement.
3. **Process finding:** the landing was verified against the approved design rather than against `main` ("built as approved"). Recommend landings cite code, not deltas — the store's whole value is that a claim about what exists can be trusted without opening the repo.
4. **Revisions required before ratification:** AX-007 measurable trigger; AX-008 real rationale; AX-009 enforcement mechanism.
5. **Cleared:** AX-006, AX-010 as written; AX-003's as-built (corroborated by merged migrations `0031`/`0032`, subject to delegation 5).
6. **Flags, non-blocking:** AX-008 counter location across two dynos; AX-009 per-tenant key storage/rotation ownership; build-vs-adopt confirmation for AX-006 and AX-009; AX-005's missing enforcement as future-walk backlog.
7. **Explicitly not graded:** the two status flips and the AX-005 key clearing — transcription, audited by the landing.

**What I'd tell the user in the session:** five of the seven rows I could grade are ratifiable after three named text fixes; one row I'm barred from grading; and one item must not be ratified tomorrow — the store currently tells anyone who reads it that the database enforces tenant isolation, and it does not.