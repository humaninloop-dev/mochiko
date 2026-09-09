FILES-READ:
- `notify-svc/CLAUDE.md`
- `notify-svc/.claude/rules/api.md`
- `notify-svc/.claude/rules/testing.md`
- `notify-svc/.mochiko/memory/codebase-analysis.md`

---

# Action plan — evaluate the notify-svc governance draft before Thursday adoption

## Standing constraint I would state up front

The workspace holds only those four markdown files. There is no `src/notify/`, no `tests/`, no `pyproject.toml`, no `.github/workflows/ci.yml`, no `.pre-commit-config.yaml`. So every factual claim in the draft can be checked only against `.mochiko/memory/codebase-analysis.md` (dated 2026-09-06, three days stale). That is a fair baseline — the draft says it was written against that memo — but it means my verdicts are "consistent / inconsistent with the analysis," not "verified against the code." I would label them that way and list the handful of claims that need a human to confirm against the real repo.

Adoption is Thursday 2026-09-10, i.e. tomorrow. That shapes the recommendation: the fix for most defects is honest labelling plus dated commitments, not deferring adoption.

No delegation. This is four short documents and a cross-reference exercise; a subagent would add a summarization layer between me and the text that is the entire object of review. I would do it solo and say so.

## Phase 1 — Build a claim ledger (the core artifact)

Read: all four files again, line by line (already done once). For each of the 9 principles in `CLAUDE.md`, the 3 rules in `api.md`, and the 3 rules in `testing.md` — 15 rules — I would tabulate five columns:

1. the requirement (the MUST),
2. the enforcement mechanism it names,
3. the pass criterion it names,
4. the matching evidence line in the analysis memo (or "none"),
5. verdict from a fixed set: **grounded** (enforcement exists per the memo), **aspirational-but-written-as-present** (mechanism described in present tense, memo shows it does not exist), **contradicted** (rule conflicts with observed state), **impossible** (cannot be satisfied as written), **unenforceable** (no mechanism, or mechanism can't test the stated criterion).

No file written yet; this is the working table that Phase 8 is built from.

## Phase 2 — Verify every named enforcement mechanism against the CI/hook inventory

The memo enumerates what is actually wired: `gitleaks` in `.pre-commit-config.yaml` and `ci.yml`; `pytest --cov=notify --cov-fail-under=70` blocking; PostgreSQL service container; `scripts/check_quarantine.py`. Anything the draft names that is not in that list is presumed not to exist until a human confirms. Expected outcome of this pass:

- **The "contract suite" does not appear anywhere in the memo, and four rules lean on it** — error format, no-traceback, UTC `Z`-suffix, and the pagination clamp in `api.md`. This is the largest single defect: one unbuilt harness is doing a third of the enforcement work while being described as if it runs today.
- **`check_utc` migration lint** — not in the CI inventory. Does not exist.
- **`NTF001` custom ruff rule** — not in the CI inventory, and separately I would flag that ruff has no user-defined-rule mechanism, so "the custom `ruff` rule NTF001" is likely not implementable as described. The honest version is a path-scoped banned-API check or a small AST/grep gate in CI.
- **`DEBUG=false` asserted at startup in production** — unverified; cheap and worth keeping.
- **pytest collection restricted in `pyproject.toml`** — unverified.
- **`db` fixture refusing non-PostgreSQL DSNs** and the **autouse socket-blocking carrier fixture** — unverified; both are cheap to build (the second needs a named dependency such as `pytest-socket`, which the draft doesn't mention).
- **Grounded, keep unchanged:** the secrets rule (both hook and CI scan exist) and the flaky-test quarantine rule (`tests/quarantine.txt` + `check_quarantine.py` both exist). I would say so explicitly — a review that only lists faults is less useful.

## Phase 3 — Ratchet check: does any rule lower a bar that already exists?

Compare each threshold to the as-found state. Expected finding, and in my view the most likely thing to slip through the meeting unnoticed:

- **Coverage is written as 60% blocking. CI already enforces 70% and actual coverage is 72%.** Adopting the draft as written would ratchet the gate *down* by ten points and permit a 12-point regression on day one. Fix: `--cov-fail-under=70`.
- Related: the draft's command is `pytest --cov --cov-fail-under=60`, dropping the `--cov=notify` source scope that CI uses. Bare `--cov` changes what is measured, so the number stops being comparable to the 72% baseline. Fix: keep `--cov=notify`.
- Second ratchet-down, softer: `api.md` states "reasonable timeouts" and "at most three times with backoff" where the memo records exact, already-universal values — `httpx.Timeout(10.0, connect=3.0)` and three `tenacity` retries. Replacing a precise implemented behaviour with an adjective makes the rule untestable and licenses drift. Fix: pin the numbers.

## Phase 4 — Feasibility and proportionality against this team

Team: three engineers, one senior, no dedicated ops.

- **"Two senior engineers sign off every PR" is arithmetically impossible** — there is one senior engineer. This rule cannot be satisfied on any PR, and a gate that always fails gets bypassed, which teaches the team to bypass gates. Fix: one non-author reviewer; the senior signs off on changes touching auth, tenant scoping, carrier credentials, migrations, or new external surface.
- **A written threat model on every PR** is disproportionate for a three-person team shipping to two web dynos, and will be reduced to boilerplate within two weeks. Fix: scope it to the same trigger list above.
- **`downgrade()` verified by running it "on a copy of production data"** — no ops, no stated mechanism for producing that copy, and it puts production data in a routine dev loop. Fix: assert `downgrade()` against the CI test database, and add a revision lint that rejects an empty/`pass` downgrade body.

## Phase 5 — Rules that are already violated on adoption day

Cross-reference each MUST with the memo's compliance counts, and separate "standard the code meets" from "to-do written as a standard." Expected:

- **Input validation** — 14 of 19 handlers use pydantic; `src/notify/api/webhooks.py` and `src/notify/api/admin.py` read `request.json()` directly. The rule's own pass criterion fails today, and no enforcement mechanism is named at all.
- **Error format** — requires `correlation_id` on every error body; the memo says there is *no correlation id on any response*. Also claims the shared handler is "the only place a 4xx/5xx body is built," while the memo says uncaught exceptions return FastAPI's default 500 body. Both halves are contradicted.
- **UTC exchange** — storage is fully compliant (41 of 41 `timestamptz`), but two API responses emit naive strings, so the `Z`-suffix criterion fails today. Also: the requirement says "explicit offset" while the test demands `Z`; pick one and say which.
- **Migrations** — 31 of 33 have working downgrades; `0012` and `0027` are `pass` stubs.
- **Pagination** — 6 of 7 list endpoints comply; `GET /admin/tenants` returns all rows. Separately, clamping `limit` at 200 is a behaviour change to an endpoint the ShopLoop frontend consumes, so it needs a note to that consumer, not just a CI assertion.

For each of these the fix is the same shape and I would write it out per rule: either fix before Thursday or grandfather explicitly, naming the file and a dated deadline. What must not ship is a document asserting compliance that the code contradicts — that is how the whole document loses authority.

## Phase 6 — What the analysis found and the draft ignores

- **Observability is deferred, and it is the memo's only "absent" row**: `print()` and plain-text `logging.basicConfig`, no request id, no structured fields, no `/health` route (Heroku pings `/`), no metrics, and alerting is "customers emailing support." A governance document that defers this while its own error-format rule mandates `correlation_id` is internally inconsistent: the correlation id *is* request-id propagation, i.e. the work being deferred. Either the error rule cannot be implemented or observability is not actually deferred. I would call this the second blocker and propose a minimum floor: request-id middleware feeding both logs and error bodies, structured JSON logs, a real `/health`, and one alert path that isn't a customer email. "The on-call engineer checks the Heroku logs when a customer reports a problem" is a description of having no alerting.
- **No dependency scanning rule** — the memo lists "no dependency scan" as a Security gap and the draft has no rule for it. Add `pip-audit` in CI or Dependabot.
- **Nothing covers the Celery/worker code.** `api.md` covers `src/notify/api/**` and `src/notify/carriers/**`; `testing.md` covers `tests/**`. Task code is ungoverned, yet INC-37's damage was queue lag on a single worker dyno. Either add task conventions (idempotency, retry/backoff, poison-message handling) or state that the omission is deliberate.

## Phase 7 — Internal consistency and document mechanics

- Check the two front-matter `paths` blocks against the "Rules files" list in `CLAUDE.md` — these agree; no change needed.
- Check each pass criterion actually tests its enforcement. Expected mismatch: **Testing — layout** enforces via pytest collection patterns but its pass criterion is "every module under `src/notify/` has a `tests/` counterpart." Collection patterns cannot check counterpart existence, so nothing enforces the stated criterion, and per-module coverage is stricter than the memo's observed "tests mirror `src/notify/`."
- Note that "tests that fail before the change and pass after it" is a review norm, not a machine check, and should be labelled as such rather than bundled into the `--cov-fail-under` enforcement line.
- Minor: the **Data — time** bullet is the only one with no rationale.
- Missing document mechanics: no named owner, no review date, and no exception process. A three-person team will need a time-boxed waiver path, or the first inconvenient rule gets silently ignored.

**The systemic fix underneath most of the above:** the draft uses identical present-tense language for gates that exist (`gitleaks`, quarantine check) and gates that do not (contract suite, `check_utc`, `NTF001`). I would recommend every rule carry an explicit status — *enforced today* / *enforced from <date>* / *manual, reviewer-checked* — because a reader cannot currently tell which protections are real.

## Phase 8 — Write the deliverable

Files I would write:

- `notify-svc/governance-review-2026-09-09.md` — the evaluation. Sections: verdict and recommendation; blockers (must change before adoption); should-change; verified-sound rules to keep as-is; claims needing repo confirmation; and the claim ledger table from Phase 1 as an appendix. Every finding cites the draft line and the memo line it conflicts with.
- Exact replacement text for each changed bullet, inline in that memo — the coverage line, the review-sign-off line, the error-format rule with its dependency on request ids made explicit, the timeout numbers in `api.md`, the migration-downgrade enforcement, plus the new dependency-scan and observability-floor rules.

Files I would **not** write without a ruling: `notify-svc/CLAUDE.md`, `.claude/rules/api.md`, `.claude/rules/testing.md`. The card asks me to evaluate and say what must change, and this is a document the team adopts by vote tomorrow. I would prepare the redlines and offer to apply them on a word from the user, rather than editing the artifact under review. If the user says "just make the changes," I apply the blockers and should-changes exactly as written in the memo and report the diff.

No tests to write or run — there is no code here. I would state plainly what I could not do: run `pytest --cov`, run `gitleaks`, grep the handlers for `request.json()`, or confirm the contract suite's absence in the actual repo. Everything in Phase 2 and 5 rests on the memo.

## Phase 9 — Decision stops, and how I proceed at each

Three points where a human ruling changes the output. I would not block on any of them; I would carry my default and flag it.

1. **Is 60% coverage a deliberate relaxation from 70%?** — If the working group voted to relax, that's their call and I record it, but it must appear as an explicit, reasoned reduction from an enforced 70%, not as a fresh floor. *Default: treat it as an error and recommend 70.*
2. **Does the contract suite exist outside this workspace?** — If it exists, four rules move from aspirational to grounded and the review shrinks substantially. If it doesn't, those four rules need build-by dates and owners. *Default: it does not exist; plan for the dated-commitment version.*
3. **Does "two senior engineers" mean two reviewers, or reviewers from outside the team?** — External reviewers would be feasible but slow for a three-person team. *Default: one non-author reviewer plus senior sign-off on the sensitive-change list.*

## What I would report at the end

A short verdict with the blockers named in one breath: the coverage gate is written 10 points below what CI already enforces; two-senior sign-off is impossible with one senior; four rules are enforced by a contract suite that does not appear to exist, and two more by a CI lint and a ruff rule that also don't; four MUSTs are contradicted by the code as analysed and need fix-or-grandfather dates; and deferring observability while mandating `correlation_id` is self-contradictory, with observability being the memo's only fully-absent category. Then the recommendation: adopt Thursday with the coverage number corrected, the sign-off rule made satisfiable, and every rule labelled by real enforcement status — because the failure mode here is not an over-strict document, it is a document that describes protections the service does not have. Plus the note that the secrets rule and the flake-quarantine rule are genuinely backed and should ship untouched, and the explicit list of what I could not verify without the repo.

After the meeting rules on it, I would save one project memory recording the ratified governance deltas and the adoption date — not the review itself, which lives in the file above.