FILES-READ: `notify-svc/CLAUDE.md`, `notify-svc/.claude/rules/api.md`, `notify-svc/.claude/rules/testing.md`, `notify-svc/.mochiko/memory/codebase-analysis.md`

---

# Action plan — pre-adoption evaluation of the notify-svc governance draft

## Phase 0 — Frame the job and confirm I'm the right grader

**What I'd do.** Confirm this is a governance-surface evaluation, not a design feasibility review. The draft was authored by "the platform working group"; I did not write it, so I can grade it. If it turned out I had authored any part, I'd hand the evaluation to someone else rather than clear my own work.

I'd also fix the scope precisely: the `## Governance` region of `CLAUDE.md` (lines 7–63) plus the two rules files. The header lines 1–5 are project description, not governance — I'd read them for consistency but not grade them.

**What I'd refuse.** I would not run a cross-artifact feasibility review here and would say so if asked: this is a standards surface, judged against enforceability/testability/rationale and the four-category floor, not against buildability of a design.

## Phase 1 — Establish what I can actually verify, and stop on the gap

**What I'd read.** I already globbed the tree. The workspace contains exactly four markdown files. There is **no source tree**: no `pyproject.toml`, no `.github/workflows/ci.yml`, no `.pre-commit-config.yaml`, no `src/notify/`, no `tests/conftest.py`, no `scripts/check_quarantine.py`.

**The stop.** The draft asserts a dozen concrete enforcement mechanisms by filename and rule ID. I cannot confirm a single one exists from this workspace. I would stop and ask the team for one of:
- (a) read access to the notify-svc repo at the commit the draft targets, or
- (b) explicit confirmation that the 2026-09-06 codebase analysis is the sole agreed ground truth for this evaluation.

**Branches.**
- If (a) is granted: I'd delegate the mechanism sweep described in Phase 3 and grade enforcement claims against the repo.
- If (b) is granted, or nobody answers before Thursday: I proceed under the analysis alone, and every enforcement claim the analysis does not corroborate gets graded as **unverified**, which for adoption purposes is treated the same as **absent** — an unverified gate is an unenforced MUST. I'd say plainly in the report that this is a weaker evaluation than it should be and name what I could not check.

**Default I proceed under for the rest of this plan:** (b).

## Phase 2 — Load the governance criteria I grade against

**What I'd do.** Consult the constitution-authoring skill for the amendment/brownfield criteria, and specifically its essential-floor reference for the canonical definitions of the four baseline categories — I grade the draft's Security/Testing/Error-handling/Observability coverage against that canonical text, not against my recollection of it. In particular I need the floor's position on dependency scanning and on what minimum observability requires, because both bear on findings below.

**Write:** nothing. Reading only.

## Phase 3 — Verify every claimed enforcement mechanism

**What I'd do.** Build a table of every enforcement mechanism the draft names, and mark each *exists / does not exist / unverified* using the analysis as evidence.

Corroborated by the analysis:
- `gitleaks` pre-commit hook and `gitleaks detect` in CI — exists.
- Blocking coverage gate in CI — exists, but at a **different threshold** than the draft states (see Phase 4, finding B).
- PostgreSQL service container in CI — exists.
- `scripts/check_quarantine.py` in CI — exists.
- Shared `notify.carriers.http` client with `timeout=Timeout(10.0, connect=3.0)` and three `tenacity` retries — exists.
- Shared error handler at `src/notify/api/errors.py` — exists, but only covers `NotifyError` subclasses.

Named by the draft and **nowhere in the analysis** — i.e. presumed not to exist:
- the `check_utc` migration lint (Data — time)
- the custom ruff rule `NTF001` and its CI gate (api.md)
- **the "contract suite" itself** — invoked as the enforcement backbone for five separate rules (problem+json schema, no `traceback` key, `Z`-suffixed datetimes, pagination clamp) and never mentioned in the analysis
- the `DEBUG=false` startup assertion
- the `conftest.py` fixture that refuses a non-PostgreSQL DSN, and the autouse fixture that fails on an open socket
- the PR-template threat-model section and the two-sign-off merge block
- any mechanism at all for "migrations must be reversible"
- pytest collection restricted in `pyproject.toml`

**Delegation (only under Phase-1 branch (a)).** One disposable `Explore` subagent per gap, `model: haiku`, one question each, terse answers with file:line provenance:
1. "Does `.github/workflows/ci.yml` contain a step invoking a ruff rule `NTF001` or a custom ruff plugin? Quote the step."
2. "Report the exact pytest/coverage invocation and `--cov-fail-under` value in `ci.yml` and in `pyproject.toml`."
3. "List every test directory or marker named `contract` under `tests/`. Quote the collection config in `pyproject.toml`."
4. "Quote every fixture in `tests/conftest.py` that inspects a database DSN or patches `notify.carriers.http`."
5. "Does `.github/PULL_REQUEST_TEMPLATE.md` exist, and does any branch-protection or CODEOWNERS file require two approvals? Quote."
6. "List CI steps that touch Alembic — any `downgrade`, any migration lint."
7. "Enumerate `src/notify/api/*.py` and report which call `request.json()` or read raw query strings."

On return I'd check each answer carries a file path and quoted line; a bare yes/no or a "probably" gets re-asked or is escalated to my own read. Anything where **absence** would change my ruling — specifically whether the contract suite exists at all, since five rules hang off it — I'd confirm myself rather than trust a sweep.

## Phase 4 — Grade each principle: enforcement, testability, rationale, necessity

This is my own read, not delegated. Findings I already hold from the four files:

**A. Observability is simply absent from the draft — blocking.**
The analysis grades observability **absent** (print statements, `logging.basicConfig`, no request id, no structured fields, no health route, no metrics, "alerting is customers emailing support"). The draft's response is a paragraph declining to cover it until "the carrier integrations settle." A baseline category cannot be deferred; and the weakest state in the codebase is the one being left ungoverned. This is the single largest defect in the draft.

**B. It contains an internal contradiction that makes the error-format rule unsatisfiable — blocking.**
The error-handling principle requires `correlation_id` on every error body. The analysis says there is **no correlation id on any response**, and nothing anywhere in the draft requires one to be generated or propagated — that machinery would have lived in the observability section that was deferred. So a MUST depends on a facility the same document declines to require. Either observability lands with request-id generation and propagation, or the error rule cannot be met. The two must move together.

**C. The coverage rule *lowers* the existing bar — blocking.**
Draft says 60%. CI today runs `--cov=notify --cov-fail-under=70`, blocking, at 72% actual. Adopting this would license an 12-point regression and would also make the document's own stated command (`pytest --cov --cov-fail-under=60`) wrong about the repo. Governance codifying a brownfield project must not ratchet *down*. Set the floor at 70 (matching the enforced gate) or 72 (matching reality), and quote the actual CI invocation.

**D. The threat-model rule is impossible to satisfy — blocking.**
"Signed off by two senior engineers before merge," on a team of three engineers with **one** at senior level. Two senior sign-offs cannot exist; if the senior authors the change, zero can. This rule would either block every merge or, far more likely, be ignored within a week — which is worse, because it teaches the team that the governance surface is decorative. Replace with something the team can actually hold: a written threat note required only on changes touching authentication, carrier credentials, or a publicly-reachable route, reviewed by any second engineer, with the senior on the auth/credential subset.

**E. Rules stated as MUSTs that the repo violates on day one, with no remediation path — blocking as a set.**
Each of these is a good rule pointed at a real gap, but adoption Thursday makes the repo instantly non-compliant with no dated plan, no exemption list, and in several cases a gate that would fail the build immediately:
- pydantic validation on every handler — 14 of 19; `api/webhooks.py` and `api/admin.py` read `request.json()`.
- reversible migrations — revisions `0012` and `0027` are `pass` stubs.
- pagination on every list endpoint — `GET /admin/tenants` returns all rows, and api.md's stated gate calls *every* list endpoint with `limit=500`; that test fails the moment it's written.
- UTC with explicit offset — columns are all `timestamptz`, but two API responses emit naive strings; the stated assertion fails on day one.
- "the shared handler is the only place a 4xx/5xx body is built" — false; uncaught exceptions still return FastAPI's default 500 body.

Fix: for each, either a named exemption with an owner and a date, or stage the rule (advisory at adoption → blocking on a stated date), and add the missing catch-all handler requirement so the "only place" claim becomes true.

**F. Three-part-rule failures — must fix before adoption.**
- *Data — time*: no rationale at all.
- *Security — input*: no enforcement mechanism. "No handler reads `request.json()`" is a pass condition restated; nothing catches a violation. Name a lint (banned-call rule on `request.json()` under `src/notify/api/`) or an import-time check.
- *Data — migrations*: no enforcement, and the pass criterion — "`downgrade()` runs cleanly on a copy of production data" — is unworkable for a three-person team with no ops, and puts customer shipping data somewhere it shouldn't be. Replace with: CI applies `upgrade head` then `downgrade -1` against the seeded test database; reject any revision whose `downgrade()` body is `pass`.
- *Testing — layout*: enforcement and pass criterion don't match. Restricting pytest collection to `test_*.py` does not verify that "every module under `src/notify/` has a `tests/` counterpart" — nothing checks that. Either write the check or weaken the pass criterion to what collection actually proves.

**G. api.md — vague where the analysis has exact numbers.**
"Reasonable timeouts" is unmeasurable. The analysis records the actual shared client: 10s total, 3s connect, three retries via tenacity. Codify those numbers. Separately, "a review-checklist item confirms no bare `httpx` import" is a human ritual standing in for a one-line banned-import lint — cheap machinery exists, use it. And nothing at all verifies the retry ceiling.

**H. Coherence between the three files — note, not blocking.**
Testing standards are split: coverage and layout sit in `CLAUDE.md`, database/mocking/quarantine in `testing.md`. The `## Rules files` index describes `testing.md` as "test conventions," which understates that a chunk of testing governance lives elsewhere. Consolidate or cross-reference.

**I. Dependency scanning — pending Phase 2.**
The analysis flags "no dependency scan" under Security; the draft is silent. Whether this is a floor obligation or a recommendation depends on the canonical floor text, which is why Phase 2 precedes the ruling. If the floor requires it, this joins the blocking list; if not, it's a recommendation.

## Phase 5 — Put my questions to the authors before ruling

**What I'd do.** Send the platform working group a short list and let their answers stand on the record:
1. Does the contract suite exist today? Five rules are enforced by it.
2. Was 60% chosen deliberately over the enforced 70, and if so, what pays for the regression?
3. How is a two-senior sign-off satisfied with one senior on the team?
4. Which of the named mechanisms (`NTF001`, `check_utc`, the conftest fixtures, the PR template) are built and which are aspirational, and who owns building the rest?
5. What is the deferral on observability actually waiting for, given the error-format rule depends on it?

**Branches.** If they answer that the missing machinery lands before Thursday with owners and dates, findings in Phase 3 downgrade from blocking to conditional-on-verification. If they answer that these are aspirational, the affected rules must be restated as staged commitments with dates, not present-tense MUSTs. The verdict stays mine either way.

## Phase 6 — Write the evaluation

**What I'd write.** A single report — I'd propose `notify-svc/.mochiko/memory/governance-review-2026-09-09.md`, confirming the path with the team first since this workspace has no established location for review output — containing:
- **Ruling: not ready for Thursday adoption**, with the blocking set (A–F, plus I if the floor requires it) that must close first.
- Per finding: what's wrong, the evidence line from the analysis or the draft, and concrete replacement text — not just an objection. Specifically: an observability section covering structured JSON logs with a request/correlation id propagated to error bodies, a `/health` route, and one alert path that isn't a customer email; coverage at 70 with the real CI command quoted; a scoped threat-note rule a three-person team can hold; exemption/staging blocks for the five day-one violations; rationale for the timestamp rule; a real enforcement mechanism for input validation and migration reversibility; the concrete timeout and retry numbers in api.md.
- A short **should-fix** tier: the api.md checklist→lint swap, the retry-ceiling gap, file coherence.
- An explicit **unverified** appendix listing every mechanism I could not confirm from this workspace, so nobody reads silence as a pass.

**What I'd report at the end.** One paragraph to the team: the draft's instincts are sound and most rules are well-formed for a small team — the incident-anchored rationales are exactly right — but it cannot be adopted as written for four reasons: it leaves the weakest area of the codebase (observability) ungoverned, it requires a correlation id that nothing in the system produces, it lowers an existing enforced coverage gate, and it contains a review rule the team is numerically incapable of satisfying. Plus a set of MUSTs the repo violates today with no remediation path and several gates that appear not to exist yet. Close those, and I'd re-review against the revised draft.