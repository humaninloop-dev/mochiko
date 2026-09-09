FILES-READ: `batch-note.md`, `architecture/spine.md`, `architecture/concerns.md`, `src/notify/db/session.py`, `src/notify/db/repository.py`, `migrations/README.md`

---

# Action plan — grading the 2026-09-08 store batch before tomorrow's ratification

## Phase 0 — Set the grading scope, and check whether I am eligible to grade all of it

**What I would do.** Split the batch into what carries judgment and what carries none, and check the authorship of each piece against my own seat.

- **In scope (judgment):** the six new rows AX-006 … AX-011 — each is an assertion about what this product should be; and the FEAT-009 landing's *claims* — the `As-built:` line added to AX-001, AX-003's as-built line, and both rows' `Drift: none`. A claim about what was built is checkable against the code and can be wrong in a way nothing downstream catches.
- **Out of scope (transcription):** AX-001's and AX-003's status tokens moving to `built` as bookkeeping, and clearing AX-005's stale `FEAT-007` key for a feature that closed 2026-07-30. Nobody's judgment is at risk in those; they ride the landing's own audit and I will say so rather than pad the grade with them. **One carve-out:** AX-001's flip to `built` is not purely clerical here, because the as-built sentence attached to it asserts a mechanism. I grade the assertion; if the assertion fails, the flip fails with it.

**What I would refuse.** **AX-011 (feature flags) I will not grade.** The batch note and the row itself both record it as drafted by the tech-lead seat — my seat. I do not clear rows I wrote. I would mark AX-011 **abstained — authorship conflict** and hand it, ungraded, to a second grader.

**The stop.** Before the desk visit I would put to the user: *AX-011 needs a grader who did not draft it — route it to another reviewer, or hold it out of tomorrow's batch and ratify the rest?*
- If **route**: the rest of my plan proceeds unchanged; AX-011 returns on the other reviewer's verdict.
- If **hold out**: I grade AX-006 … AX-010 plus the landing claims and report AX-011 as deferred.
- If the user says *grade it anyway*: I decline the clearing role but will hand over the open questions I would want asked of it (below) for someone else to press — that is the most I can honestly give on my own row.
- **Default while planning:** treat AX-011 as abstained and carry its open questions as a note.

---

## Phase 1 — Test the landing's as-built claims against the code (highest-value check first)

**What I would read.** `src/notify/db/session.py`, `src/notify/db/repository.py`, `migrations/README.md`, then back to AX-001 and AX-003.

**AX-001 — this is where the batch breaks.** The store says the FEAT-009 as-built is: row-level security enabled on every tenant table, tenant set per request by `SET LOCAL app.tenant_id` in the session factory, so an unfiltered query returns nothing. The code says the opposite, in three independent places:

- `session.py` states row-level security was tried for FEAT-009 and **backed out** because the policies broke the admin console's cross-tenant reports; the work moved to branch `feat-009-rls`. There is no `SET LOCAL` anywhere in the factory — it opens a plain session, commits, rolls back, closes.
- `migrations/README.md` lists `0034_enable_rls` as **not on `main`**, reverted from the FEAT-009 branch on 2026-09-05.
- Scoping is actually at the ORM layer: `TenantRepository.query()` appends `tenant_id == current_tenant()`. That is a real control, but it is opt-in — the file says in as many words that a query bypassing the repository is not scoped, which is exactly the failure the store's sentence claims is now impossible.

The store even names its own provenance: *"Written from the FEAT-009 design (delta D-2)."* It was written from the design, not from the build. The design's intent did not survive contact with the admin reports, and nobody re-read the code before writing the row.

**Consequences I would record:**
1. The as-built line is **false as written** and must be rewritten to describe repository-level scoping, naming the bypass explicitly.
2. **`Drift: none` is false.** The delivered mechanism is weaker than the ruled/designed one and a piece of the work sits unmerged on a branch. That is drift, and it is the kind that matters — a security boundary the ledger reports as defence-in-depth and the code implements as a convention.
3. The **enforcement line no longer matches the risk.** `tests/tenancy/test_isolation.py` reading every tenant table as the wrong tenant will pass on repository-mediated reads whether or not RLS exists; it cannot fail on the bypass case. Whatever the row's final stance, enforcement needs a check that catches a query built outside `TenantRepository` — a lint rule against constructing `select()` on a tenant model outside the repository, or a test that issues a raw session query and asserts it is caught.
4. One sub-claim **does** check out: "nine tenant tables" is corroborated by `0033_tenant_id_on_webhook_log`, described as the ninth. I would say so — a grade that only reports faults is less useful than one that says which parts stood up.

**AX-003 — supported on the facts, wrong on the stance.** `0031` (new `tenant_credentials`, `pgcrypto`) and `0032` (carrier keys moved out of config) merged to `main` on 2026-09-02/03, so the as-built is corroborated and `built` is earned. But the **ruling was never amended**: it still reads "Heroku config vars, rotated quarterly," while the as-built moves carrier secrets to encrypted database rows. That is a different secrets posture arriving through the back door of a landing note. Two things follow — the quarterly rotation commitment now has no stated owner or mechanism for the material that moved, and the row's enforcement (`gitleaks` pre-commit and CI) does not reach a key living in a database column. Also unrecorded: where the app key used by `pgcrypto` lives and how *it* rotates. I would require the ruling amended to cover both storage classes, with enforcement named for each.

---

## Phase 2 — Close the verification gaps I cannot close from my own reading

A full-workspace listing returns only six files. The enforcement paths the ledger relies on — `tests/tenancy/test_isolation.py`, `tests/auth/`, `tests/api/test_rate_limit.py`, `notify.api.idempotency`, `notify.flags` — are not present here. I will **not** report them as missing on that basis; this is far more likely a slice of the repository than the whole of it. Absence would drive a verdict, so I do not infer it.

**Delegation.** Three disposable read-only `Explore` subagents, each forced to the cheap model tier (`model: haiku`), one gap apiece, facts-with-file-and-line back:

1. *"In the full notify-svc repository, does the literal string `SET LOCAL` or `app.tenant_id` appear anywhere under `src/`? Report every hit with path and line, or state none."* — **Expected: none.** On return I check that the hits (if any) are not in the request path; a hit inside the session factory would overturn Phase 1 and I would re-read it myself before changing anything.
2. *"List the files under `tests/` matching tenancy, auth, rate limit, and idempotency; for each, quote the test function names."* — **Expected:** the tenancy and auth suites exist, `test_rate_limit.py` does not yet (AX-008 is queued as FEAT-012). On return I check whether the tenancy test issues any query outside the repository; if it does not, that confirms finding 3 above.
3. *"Does a module `notify/api/idempotency*` or `notify/flags*` exist on `main`? Report paths or state absent."* — **Expected: absent**, both being queued work. On return, presence would mean a row is describing shipped code as future work and I would grade that row's status as well.

**If the subagents come back with "path not in this workspace"** — i.e. the slice really is all there is — I stop and ask the user for a pointer to the repository rather than guess, and mark every enforcement claim I could not reach as *unverified* in the grade instead of passing or failing it. I would not let a workspace boundary masquerade as a finding.

---

## Phase 3 — Grade the new rows on judgment

For each I ask the same four things: can a violation actually be caught, is there a stated pass/fail, is the reasoning one a maintainer could re-evaluate in two years, and does the row pay for itself.

- **AX-006 Idempotency — clears.** Reasoning is anchored to a real event (INC-48, a retried shipment double-booked and double-charged), enforcement is a per-endpoint contract test asserting one row and one carrier call, and a single named middleware means there is one place to violate. Two questions I would raise without blocking on them: what stores the 24-hour window and what evicts it, and what happens on a repeated key with a *different* body — that should be a rejection, not a replay of the stored response.
- **AX-007 Read caching — needs revision.** The `not-now` stance is well argued (one hot read, 80 ms at p95, a cache buys an invalidation path for a problem that does not exist — I agree with it). The trigger is not usable: *"when the read load becomes a problem"* is a judgement call deferred to a future reader with no number. The fix costs nothing because the number is already in the row's own reasoning — restate as a p95 threshold on the tracking page, plus a request-rate figure. A not-now stance without a trip-wire is how a deferral quietly becomes permanent.
- **AX-008 Rate limiting — needs revision.** Enforcement is fine and testable (61 requests inside a minute, expect a 429 on the last; both limits in one settings module). The reasoning is not: *"team preference; agreed on the call"* tells a maintainer in 2028 nothing about whether 60 and 600 still apply. I would require what the numbers are derived from — observed p99 request rate for a legitimate recipient, and the tenant ceiling the `api` dynos can serve — so someone can tell whether raising them is safe. I would also ask what a legitimate burst looks like (a shop bulk-importing shipments) and whether 600/minute/tenant strands it.
- **AX-009 Webhook signing — needs revision.** The ruling is admirably precise (HMAC-SHA256 over the body, `X-Notify-Signature`, keyed per tenant) and the reasoning is strong — two shops asked, and it is the recurring security-questionnaire failure. But the row has **no enforcement line at all**, and the work is queued, so there is no excuse for not naming one now: a test that tampers with a byte of the body and asserts signature verification fails, plus a known-vector test. Two substantive gaps I would press: signing the body alone is replayable — a timestamp inside the signed material with a freshness window is the standard answer — and nothing states how a per-tenant signing key reaches the shop or rotates. A signing scheme with no key-rotation story is a future incident.
- **AX-010 Multi-region — clears.** A well-formed "not ours": named owner (`shoploop/platform-infra`), a pointer someone can actually follow (the "Regions" runbook), and reasoning for why it is theirs — no region-pinned data, latency dominated by the carrier. Nothing to enforce here, correctly.
- **AX-011 Feature flags — abstained**, per Phase 0. The questions I would hand the other grader: for two flags a quarter across 40 tenants, does a table plus a 30-second cache plus a custom lint rule pay for itself against a config var, and what does a 30-second cache do to a rollback when a pilot flag is going wrong?

**Batch-level observation.** The walk was booked as the reliability and security dimensions. Security got real coverage. Reliability is thinner than the label implies: there is no row for retry, backoff, or dead-lettering on the Celery worker and the outbound carrier calls — the single most likely source of reliability incidents in a service whose latency is, by AX-010's own reasoning, dominated by the carrier. Nor is there a row for what the service emits when it fails. I raise this as under-coverage of the dimension the walk itself claimed, not as a demand that they walk ground they never opened.

---

## Phase 4 — Put my questions to the author before I rule

I would send the architect seat a short list and put the answers on the record beside my findings: which requirement pays for AX-008's specific numbers; why AX-009 shipped without enforcement; whether AX-007's trigger can take the 80 ms figure; and, the important one — **was the AX-001 as-built line written from the FEAT-009 design rather than from `main`, and were any other rows in this batch written the same way?** If the answer to the last is yes, the defect is a process defect, not a typo, and every as-built line in the batch needs re-reading against code before ratification. Their answers stand as theirs; the verdict stays mine.

---

## Phase 5 — Render the grade

**Batch verdict: not ready for ratification.** I would not let tomorrow's desk visit put a false statement about a tenant-isolation mechanism into the surface the whole product is read off. Per item:

| Item | Verdict |
|---|---|
| AX-001 as-built + `Drift: none` | **Fails** — contradicted by `session.py`, `repository.py`, and the unmerged `0034`. Must be corrected before ratification. |
| AX-001 enforcement | **Needs revision** — cannot catch the bypass the row claims is closed. |
| AX-003 as-built | **Clears** on the facts (`0031`/`0032` on `main`). |
| AX-003 ruling + enforcement | **Needs revision** — stance changed without amendment; no mechanism covers DB-held secrets or the app key. |
| AX-006 | **Clears** (two questions attached). |
| AX-007 | **Needs revision** — untestable trigger. |
| AX-008 | **Needs revision** — no evaluable rationale. |
| AX-009 | **Needs revision** — no enforcement; replay and key-rotation gaps. |
| AX-010 | **Clears.** |
| AX-011 | **Abstained** — I drafted it. |
| Status flips, AX-005 stale key | **Out of scope** — transcription, rides the landing's audit. |

Nothing here is unbuildable — every finding is closable by revision. I would not dress any of it up as a fundamental conflict.

**The second stop.** How AX-001 gets corrected is the architect's and the user's call, not mine, because it is a stance question:
- **If row-level security remains the intended end state:** AX-001's status is not `built` — it is partially delivered with the defence-in-depth layer outstanding. Rewrite the as-built to repository-level scoping, record the drift naming branch `feat-009-rls` and migration `0034`, and key the remainder to a follow-up feature. The admin console's cross-tenant reporting need must be designed into the policies, since that is what backed the work out.
- **If ORM-level scoping is now the accepted stance:** amend the ruling to say so, drop the RLS intent explicitly rather than leaving it on a branch, and add enforcement that catches queries built outside the repository — because that convention is now the whole boundary.
- **Default I would carry into the meeting:** the first branch. The row was ruled and designed with the database-level control in it, and a security boundary should not be silently downgraded by a landing note.

---

## Phase 6 — What I would report

A single grading note to the user before the desk visit, ordered by what changes their decision:

1. **Do not ratify AX-001 as written** — the store claims a database-enforced tenant boundary that was built and backed out; the shipped control is an opt-in repository convention that the ledger's own test cannot fail on. Cite `session.py` lines 18–22 and the `0034` line in `migrations/README.md`.
2. **The root cause is provenance, not carelessness** — the row says it was written from the design delta. Ask whether any other as-built line in this batch shares that origin.
3. Four rows need revision before they are worth ratifying (AX-003's ruling, AX-007, AX-008, AX-009), each with the specific missing piece and the cheap fix.
4. Two rows clear (AX-006, AX-010); AX-003's as-built facts and AX-001's nine-table count check out against the migrations.
5. **AX-011 is ungraded and needs another reviewer** — I wrote it.
6. Reliability coverage gap: no retry/dead-letter row and no failure-visibility row, in a batch that walked the reliability dimension.
7. What I could not verify and why — any enforcement path the delegated reads could not reach, listed as unverified rather than passed.