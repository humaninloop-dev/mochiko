FILES-READ: `notify-svc/CLAUDE.md`, `notify-svc/.claude/rules/api.md`, `notify-svc/.claude/rules/testing.md`, `notify-svc/.mochiko/memory/codebase-analysis.md`

Note up front: a glob of the workspace returns exactly those four files. There is no `src/`, `tests/`, `.github/`, `pyproject.toml`, or `.pre-commit-config.yaml` here. So the codebase analysis dated 2026-09-06 is the *only* evidence about the running system, it is second-hand, and it is three days stale. Every verification step below is a text-vs-analysis check, not a check against code — that limitation is itself a finding I would put in the memo.

---

# Action plan — evaluate the notify-svc governance draft before Thursday adoption

## Phase 1 — Fix the frame: scope, evidence, and the clock

**Do:** Establish what "evaluate" means for this artifact and how much runway exists.

- Adoption is Thursday. Today is Wednesday 2026-09-09, so adoption is **2026-09-10 — tomorrow**. This is the single most important planning constraint: anything requiring new CI machinery cannot be a blocking gate at adoption. Every finding therefore gets sorted into one of two buckets, and I would not blur them:
  - **(A) Text changes** — editable in the draft before the meeting, zero engineering cost.
  - **(B) Engineering work** — needs an owner and a date, and until then the rule must be labelled aspirational, not enforced.
- **Read:** the four files above (done).
- **Attempt to read and record as absent:** `notify-svc/.github/workflows/ci.yml`, `notify-svc/pyproject.toml`, `notify-svc/.pre-commit-config.yaml`, `notify-svc/src/notify/api/errors.py`, `notify-svc/tests/conftest.py`, `notify-svc/tests/quarantine.txt`, `notify-svc/scripts/check_quarantine.py`, `notify-svc/alembic/`. Their absence is expected in this workspace; I record it so the memo states plainly that enforcement claims were checked against a written analysis, not against CI config.
- **Flag:** the analysis was produced by `detect-stack.sh` plus a manual read. Counts in it (14 of 19 handlers, 31 of 33 revisions, 6 of 7 list endpoints, 41 date-time columns) are the load-bearing numbers for this review and should be re-confirmed by whoever owns the repo before the meeting. I would say so rather than present them as verified.

**Write:** nothing yet.

## Phase 2 — Enforcement audit: does each stated mechanism actually exist?

**Do:** Take every "Enforcement:" clause across all three files and mark it exists / does not exist / unverifiable, citing the analysis line that supports it.

Expected result:

| Mechanism | Verdict | Basis |
|---|---|---|
| `gitleaks` pre-commit + `gitleaks detect` in CI | **Exists** | analysis Security row |
| Blocking `pytest --cov` gate in CI | **Exists** (at 70, not 60 — see Phase 3) | analysis Testing row |
| PostgreSQL service container in CI | **Exists** | analysis Testing row |
| `scripts/check_quarantine.py` + `tests/quarantine.txt` in CI | **Exists** | analysis Testing row |
| Shared `notify.carriers.http` client, 10s timeout / 3s connect, 3 retries via tenacity | **Exists** | analysis "Patterns worth codifying" |
| Shared problem+json handler in `src/notify/api/errors.py` | **Partially exists** — covers `NotifyError` subclasses only | analysis Error handling row |
| **"the contract suite"** (asserts error schema, absence of `traceback`, `Z`-suffixed date-times, pagination clamp at `limit=500`) | **No evidence it exists** | analysis names no contract suite anywhere |
| **`check_utc` migration lint** | **No evidence it exists** | not in analysis |
| **Custom ruff rule `NTF001`** | **No evidence it exists** | not in analysis |
| **PR template threat-model section + blocked merge on two sign-offs** | **No evidence it exists** | not in analysis |
| `DEBUG=false` asserted at startup in production | **Unverifiable** | not in analysis |
| pytest collection restricted in `pyproject.toml` | **Unverifiable** (plausible; it is close to the pytest default) | not in analysis |
| Alembic `downgrade()` reversibility | **Nothing checks it today**, stated outright in the analysis | analysis "Patterns worth codifying" |

**The headline finding:** the *contract suite* is cited as the enforcement or pass condition for **five** separate rules (error format, no traces, UTC in responses, pagination clamp, and implicitly correlation ids) and there is no evidence it exists. A governance document whose most-cited enforcement mechanism is fictional will be discovered to be fictional the first time someone tries to rely on it, and that is how a whole document loses authority. This is bucket (B) and cannot be built by tomorrow.

**Write:** the audit table into the review memo (path in Phase 7).

## Phase 3 — Check each rule against the state the analysis actually describes

**Do:** For every MUST, ask three questions — is it achievable by this team, is the repo compliant today, and if not, is there a stated path?

Expected findings, in the order I would rank them for the meeting:

1. **Coverage bar is a downgrade, not a floor.** The draft mandates `--cov-fail-under=60`. CI already runs `--cov-fail-under=70` and current coverage is **72%**. Adopting this draft as written *relaxes* an existing blocking gate by ten points and permits a 12-point regression on day one. Almost certainly a copy/paste from a generic template rather than an intended loosening. **Must change to 70** (or 72 to lock in current state). Bucket (A). This is the one I would open the memo with, because it is a one-character fix that otherwise silently weakens a control the team already passes.

2. **The two-senior-sign-off rule is arithmetically impossible.** "Every pull request MUST include a written threat model … signed off by **two senior engineers**." The team is **three engineers, one at senior level** — stated in the analysis and restated in the header of CLAUDE.md itself. There is no second senior. The rule cannot be satisfied for any PR, ever, so either the merge gate blocks all work or the rule is ignored from day one and teaches the team that MUSTs in this document are optional. **Must be rewritten.** My recommendation: drop the blanket threat-model requirement; require a short written risk note reviewed by the senior engineer **only** for changes touching authentication, tenant scoping, carrier credentials, or the webhook handlers. Bucket (A).

3. **Observability is deferred, but a principle already depends on it.** The analysis rates observability **absent**: `print()` and plain-text `logging.basicConfig`, no request id, no structured fields, no `/health` route (Heroku pings `/`), no metrics, and "alerting is customers emailing support." The draft's response is "Not covered in this draft … the on-call engineer checks the Heroku logs when a customer reports a problem" — which restates the failure mode as the plan. Two problems: (a) the error-format principle **requires `correlation_id` on every error body**, and the analysis says there is **no correlation id on any response**; a correlation id is not implementable without request-id propagation, so the draft mandates an output of the capability it declines to build; (b) the stated rationale for problem+json is that it "let[s] support match a ticket to a log line," which cannot happen while logs are unstructured plain text. **Must change:** either add a minimal observability section (request id middleware, structured JSON logs, a real `/health` route) or drop `correlation_id` from the required error fields. I would recommend the former and say why — the `/health` route in particular is small, and "Heroku pings `/`" means a dyno serving 200s from the index route while the database is unreachable looks healthy. Bucket (B), with the `correlation_id` text change in (A).

4. **Five rules are violated by the repo at the moment of adoption, with no grace period stated.** The draft uses MUST throughout with no exemption list, so the team adopts a document it fails on Thursday morning:
   - **Input validation:** `src/notify/api/webhooks.py` and `src/notify/api/admin.py` read `request.json()` directly (14 of 19 handlers validate). Webhooks are externally reachable — this is the highest-severity live gap in the analysis and deserves to be named as remediation work, not just a rule.
   - **Pagination:** `GET /admin/tenants` returns all rows (6 of 7 paginate) — and the analysis says an unbounded tenant list already took the database down (INC-29 per the rationale). The one known-vulnerable endpoint is the one still non-compliant.
   - **Timestamps:** all 41 columns are `timestamptz` (storage is clean), but **two API responses emit naive strings**, so the "exchanged in UTC with an explicit offset" half fails. Note the mismatch: the `check_utc` lint the draft proposes only inspects *columns*, which are already 100% compliant, and would catch neither of the two real defects.
   - **Migration reversibility:** revisions `0012` and `0027` are `pass` stubs.
   - **Error handling:** uncaught exceptions return FastAPI's default 500 body, not problem+json; no correlation id anywhere.
   
   **Must change:** add an explicit exemptions table to the draft listing these six items, each with a named owner and a target date, and state that the MUST binds new and modified code immediately and existing code by those dates. Bucket (A) for the table; the fixes themselves are (B).

5. **Rule, enforcement, and pass condition disagree in two places.**
   - *Testing — layout:* the rule is about file placement and naming; the enforcement is a pytest collection pattern; the pass condition is "**every module** under `src/notify/` has a `tests/` counterpart." Collection patterns do not check for counterparts, and the analysis only says tests "mirror" the tree — it never claims full one-to-one coverage. The pass condition is stricter than the rule and unverified. Align it to the rule, or make it a real check.
   - *Data — migrations:* has a Pass ("`downgrade()` runs cleanly on a copy of production data") and a rationale but **no Enforcement line at all** — the only principle in the draft missing one. And restoring a copy of production data is a heavy manual ritual for a three-person team with no ops. Replace with something they will actually run: `alembic downgrade -1` against the CI database after `upgrade head`, plus a check rejecting an empty `downgrade()` body. That also cleans up `0012` and `0027`.

6. **Weak enforcement where automation is cheap.** The carrier rule in `api.md` leans on "a review-checklist item confirms no bare `httpx` import" — the weakest mechanism in the whole set, on the rule protecting against INC-37 queue cascades. A lint banning `httpx` imports outside `notify/carriers/http.py` is a few lines and matches the rigor applied elsewhere. Separately, that rule says "**reasonable** timeouts" while the real, working values are known (10s total, 3s connect, three retries via tenacity) — write the numbers down. Bucket (A) for the numbers.

7. **A gap the analysis found and the draft ignores:** "no dependency scan" is listed under Security, and no principle addresses it. Either add one (Dependabot or `pip-audit` in CI) or say explicitly it is out of scope for this round. Silence reads as an oversight.

8. **Unsourced incident references.** INC-41, INC-37, and INC-29 carry three rationales; none appears in the analysis. I would not call these wrong — they are rationale, not enforcement, and the working group plausibly knows them first-hand — but I would ask for links so the document stays checkable a year from now. Low priority; I would say so rather than pad the list.

## Phase 4 — Cross-file consistency

**Do:** Check the three governance files against each other.

- Frontmatter `paths:` in `api.md` (`src/notify/api/**`, `src/notify/carriers/**`) and `testing.md` (`tests/**`) match the "Rules files" list at the end of CLAUDE.md exactly. **Consistent — no change.** I would state this positively so the memo isn't read as uniformly negative.
- No contradictions between the CLAUDE.md testing principles (coverage, layout) and `testing.md` (real PostgreSQL, mock boundary, flake quarantine); the two sets are complementary. The overlap between "validate before touching the database" and "no SQL in a handler" is mild and harmless.
- `testing.md` is the strongest of the three files: every one of its three rules has an enforcement mechanism the analysis confirms exists. I would recommend it be adopted **as is**, and point to it as the model the rest of the draft should match.

## Phase 5 — Decisions I would stop on

I cannot settle these; each is the team's call at Thursday's meeting. I would put them at the top of the memo as explicit questions, with my recommendation and the onward branch for each ruling, and continue the write-up under my stated default so the memo is complete either way.

1. **Is 60% deliberate?** Default assumption: it is a template artifact and should read 70. If the team says 60 is intentional, the memo must record it as a conscious relaxation of an existing gate with a reason — not leave it looking accidental.
2. **Is Thursday a hard adoption date?** Default assumption: yes. Then the document is adopted with every bucket-(B) rule visibly marked "not yet enforced — owner, date," because shipping rules whose enforcement does not exist is what makes the document ignorable. If the date can slip two weeks, the better branch is to build the contract suite and the `httpx` lint first and adopt a document that is true on the day it is signed.
3. **Threat models — drop or scope down?** Default: scope down to security-sensitive paths with the single senior engineer reviewing. If the team insists on two sign-offs, that is a hiring or a definition question, and the rule must not merge-block until it is resolved.
4. **Observability now or later?** Default: pull in the minimum (`/health`, request id, structured logs) because `correlation_id` and support-ticket correlation already depend on it. If the team holds the line on deferring, then `correlation_id` must come out of the required error fields in the same edit — the two cannot both stand.

I would **not** rewrite `CLAUDE.md` or the rules files myself as part of this. The ask is to evaluate and say what must change, the document is a team artifact being voted on tomorrow, and pre-emptively editing the thing under review takes the decision away from the meeting. I would offer the redlines as a ready-to-apply diff in the memo and apply them only if asked.

## Phase 6 — What I would not do

- **No subagents, no delegation.** The workspace is four files totalling a few hundred lines; I have read all of them. Farming this to workers would add coordination cost and a second-hand summary layer over evidence I already hold directly.
- **No test runs, no shell.** There is no code here to run anything against.
- **No memory write.** The durable facts (adoption date, rule set) live in the repo document itself, and nothing here is a preference or a correction from the user. If the user later tells me how they want governance reviews framed, that would be worth saving.

## Phase 7 — Deliverable

**Write:** `notify-svc/GOVERNANCE-REVIEW.md` — the review memo, structured as:

1. Verdict in one line: **adopt with changes; do not adopt as written.**
2. The four open decisions from Phase 5, each with recommendation and both branches.
3. **Must change before adoption** (bucket A, all doc edits, all doable tonight): coverage 60→70; rewrite or scope the threat-model rule to one senior on security-sensitive paths; either add minimal observability or remove `correlation_id`; add the exemptions table for the six known live violations with owners and dates; label every rule whose enforcement does not yet exist as not-yet-enforced; fix the layout pass condition; give the migrations rule a real enforcement line; write the actual carrier timeout numbers in; add or explicitly defer dependency scanning.
4. **Must be built** (bucket B, needs owners and dates): the contract suite, `check_utc` (retargeted at response serialization, not columns — the columns are already clean), `NTF001`, the `httpx` import ban, `/health` + request id + structured logs, plus the six code fixes (webhooks/admin validation, `/admin/tenants` pagination, two naive date-time responses, `0012`/`0027` downgrades, default-500 handler).
5. The Phase 2 enforcement audit table.
6. What is already sound and should ship untouched: all of `testing.md`, the secrets principle, the rules-file path wiring, and the carrier client conventions — these are accurately drawn from what exists.
7. The evidence caveat: everything above was checked against a three-day-old written analysis, not against the code, because the code is not in this workspace; the counts should be re-confirmed by the repo owner before the meeting.

**Report at the end:** the one-line verdict, the three findings that would change the outcome of Thursday's vote if unaddressed (coverage regression, impossible sign-off rule, contract-suite-as-phantom-enforcement plus the observability/`correlation_id` contradiction), the four questions I need the team to answer, and a plain statement that I evaluated the draft against the analysis document rather than the running system, with what that leaves unverified.