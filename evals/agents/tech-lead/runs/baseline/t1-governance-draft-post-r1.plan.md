FILES-READ: `notify-svc/CLAUDE.md`, `notify-svc/.claude/rules/api.md`, `notify-svc/.claude/rules/testing.md`, `notify-svc/.mochiko/memory/codebase-analysis.md`

---

# Action plan — evaluate the notify-svc governance draft before Thursday's adoption

## Phase 0 — Standing to review, and scope

**What I would do.** Confirm I am not the author. The draft is attributed to "the platform working group," dated 2026-09-08; nothing in the workspace names me as its writer, so I can grade it. If that attribution turned out to cover me, I would stop and hand the grading to someone else rather than clear my own work.

Fix the scope: three artifacts under evaluation — the `## Governance` region of `CLAUDE.md` (lines 7–63), `.claude/rules/api.md`, `.claude/rules/testing.md` — judged against one input, `.mochiko/memory/codebase-analysis.md`. This is governance-surface work, so I would load the constitution-authoring skill and work its brownfield branch, and consult its essential-floor reference as the canonical list of the four baseline categories rather than working from memory. I would **not** treat this as a feasibility review; that procedure is for design artifacts and does not apply to a governance surface.

**What I would refuse.** I would not silently rewrite the draft into a version I like. My output is a change list the working group can act on before Thursday, with each item marked blocking or not.

## Phase 1 — Establish what can actually be verified

**What I would do.** Note a fact that shapes everything downstream: the workspace contains only these four markdown files. There is no `src/notify/`, no `.github/workflows/ci.yml`, no `pyproject.toml`, no `.pre-commit-config.yaml`, no `tests/conftest.py`, no `alembic/`. I confirmed this with a full recursive listing of `notify-svc/`.

Consequence: every enforcement mechanism the draft asserts is **unverified**, and the analysis file is the only secondhand evidence I have. Several draft claims go beyond what the analysis says, and a few contradict it.

**The stop.** Before finalising, I would put one question to the requester: *is the real notify-svc repository available to me, or is this document set the whole review surface?*
- **If the repo is available** → I proceed to Phase 2's verification sweep and every "unverified" finding either resolves or hardens into a confirmed defect.
- **If it is not** → the verification sweep is impossible; I keep the same findings but label each mechanism-existence question as an open item the working group must close themselves before Thursday, and I say so plainly in the verdict.

**Stated default for the rest of this plan:** the repo is not present, so I plan the sweep as the branch I would take *if* it were, and carry the unverified items forward as open questions.

## Phase 2 — Verification sweep (delegated, cheap, one gap per worker)

**What I would do.** These are locate-and-quote jobs with no judgment in them, so I would spawn disposable `Explore` subagents, each pinned to `model: haiku`, one gap each, and keep the bulk reading out of my context. Each brief asks for verbatim quotes with file path and line number, and an explicit "not found" if absent — never a paraphrase.

| # | Brief to the worker | What I check on return |
|---|---|---|
| 1 | Quote the pytest/coverage invocation in `.github/workflows/ci.yml` and any `[tool.pytest]`/`[tool.coverage]` block in `pyproject.toml`. | Does it read `--cov-fail-under=70`, as the analysis says? Any mismatch with the draft's `60`. |
| 2 | Quote every `ruff` configuration block and list any custom-rule plugin or local lint script. Report whether the code `NTF001` appears anywhere. | Whether `NTF001` exists at all, or is aspirational. |
| 3 | List files under `src/notify/api/` that call `request.json()` or read a raw query string, with line numbers. | Whether it is still exactly `webhooks.py` and `admin.py`, or more/fewer. |
| 4 | Quote the `db` fixture and any autouse network-blocking fixture in `tests/conftest.py`. | Whether the two mechanisms `testing.md` claims actually exist. |
| 5 | List Alembic revisions whose `downgrade()` body is `pass` or empty. | Confirm the `0012` / `0027` stub count. |
| 6 | List route handlers registered on list-returning paths and whether each accepts `limit` and `cursor`. | Confirm `GET /admin/tenants` is the sole unpaginated endpoint. |
| 7 | Quote the retry/timeout construction in `notify/carriers/http` and any direct `httpx` import elsewhere under `src/notify/carriers/`. | Confirm the 10.0/3.0 timeout and three-retry `tenacity` values to substitute for the draft's "reasonable." |

**What stays with me.** Whether the four baseline categories are satisfied, whether a mechanism is genuinely enforcing, whether a rule is worth its weight, and every judgment about the draft's language — all of that I do myself. I also do not delegate anything where a worker reporting "nothing found" would drive a decision on its own.

## Phase 3 — Baseline-category audit of `CLAUDE.md`

**What I would do.** Walk the four non-negotiable categories against the analysis table and the draft, and record status for each.

- **Security** — draft covers secrets, input validation, and PR review. The analysis flags "no dependency scan" as part of why Security is only partial; the draft never addresses it. Gap.
- **Testing** — covered, but see Phase 4's coverage-number finding.
- **Error handling** — covered in form, but the draft's assertions do not match the as-found state (Phase 4).
- **Observability** — **explicitly deferred.** The draft states it outright at lines 54–57 and substitutes "the on-call engineer checks the Heroku logs when a customer reports a problem." The analysis rates it `absent`: plain-text `print()` and `basicConfig`, no request id, no structured fields, no health route, no metrics, alerting is customers emailing support. This is the single largest defect in the draft, and I would not soften it. A baseline category cannot be postponed to "once the carrier integrations settle."

**What I would write.** A category-by-category table in the review record, with the observability row marked blocking.

## Phase 4 — Rule-by-rule audit: mechanism, pass/fail, rationale, and truth against the codebase

**What I would do.** Take every bullet in all three files and test it four ways: is there a real mechanism, is pass/fail decidable, is the reason recorded, and does the claim survive contact with the analysis. Below is my provisional finding list from the reading already done; the Phase 2 sweep would confirm or overturn the items marked *unverified*.

**Blocking — must change before adoption:**

1. **Observability absent** (`CLAUDE.md` 54–57). Needs real content: structured JSON logs with a request/correlation id on every line, a `/health` route with a stated check and a mechanism that asserts it (Heroku currently pings `/`), and a defined path from a failure to a human that is not a customer email. For a three-person team with no ops, I would keep this deliberately small — structured logging plus health plus one alert on error rate — and say so, because an aspirational monitoring stack here will be ignored.

2. **Coverage floor moves the bar down.** Draft mandates 60% and specifies CI runs `--cov-fail-under=60`. The analysis records CI already at `--cov-fail-under=70` with actual coverage at 72%. Adopting this either forces an edit that *weakens* a working gate, or leaves the document describing a command the repo does not run. Must read at least 70; I would argue for 72 with a no-regression stance, and either way the documented command must match `ci.yml` exactly (which also carries `--cov=notify`).

3. **Threat model with two senior sign-offs is unsatisfiable** (`CLAUDE.md` 21–25). The team has three engineers and **one** at senior level — stated in the analysis and repeated at the top of `CLAUDE.md` itself. A rule requiring two senior sign-offs on every PR can never pass. Separately, "the merge is blocked until both sign-offs are recorded" names no mechanism that does the blocking. And a written threat model on *every* PR is process weight this team will abandon in a fortnight. Replace with something proportionate: a threat-model section required only on changes touching auth, tenant scoping, carrier credentials, or a new external surface, with one reviewer, and a named branch-protection or template check that actually enforces it.

4. **"Data — time" has no rationale** (`CLAUDE.md` 46–49). Enforcement and pass criterion are present; the reason is simply missing. Every other bullet carries one. Add it — a maintainer in two years needs to know whether the constraint still applies.

5. **"Data — migrations" has no enforcement** (`CLAUDE.md` 50–52). Pass and rationale only. And the pass criterion — "`downgrade()` runs cleanly on a copy of production data" — is neither automated nor advisable to run casually against production-derived data. Give it a real mechanism (a CI step that applies then reverses each new revision against the test database) and a criterion that mechanism can decide.

6. **Rules asserted as MUST that the codebase violates on day one, with no remediation path.** Three of these:
   - Input validation (`CLAUDE.md` 18–20) — `webhooks.py` and `admin.py` read `request.json()` directly; 14 of 19 handlers comply. *Also has no enforcement mechanism at all — only a pass criterion and a rationale.*
   - Pagination (`api.md` 19–22) — `GET /admin/tenants` returns all rows, and the stated enforcement is a contract-suite sweep of *every* list endpoint, so it fails the moment it is turned on.
   - Reversible migrations — revisions `0012` and `0027` are `pass` stubs.
   
   Each needs either a dated remediation item with an owner, or an explicit named exception with an expiry — not a MUST that is false on adoption day. A governance surface that is violated the hour it lands teaches the team the surface is decorative.

7. **Error-handling claims contradict the as-found state** (`CLAUDE.md` 36–41). The draft says the shared handler "is the only place a 4xx/5xx body is built" and requires `correlation_id` on every error. The analysis says uncaught exceptions fall through to FastAPI's default 500 body, and **no response carries a correlation id at all**. Two consequences: the "only place" claim is untrue as written, and the `correlation_id` requirement has no source — nothing in the entire draft creates or propagates one. This is the cross-document seam I would call out most sharply: the error rule depends on a correlation id that only the deferred observability section could have introduced. Fixing observability and fixing this are one job, not two.

**Should fix — weak mechanisms and undecidable criteria:**

8. **"Reasonable timeouts"** (`api.md` 13). Unmeasurable. The analysis hands me the actual values: `timeout=httpx.Timeout(10.0, connect=3.0)` with three `tenacity` retries. Write the numbers in. Separately, the enforcement is a *review-checklist item* confirming no bare `httpx` import — a human habit, not a mechanism. The draft elsewhere shows a lint-based approach is available to this team; make this a lint or a grep in CI.

9. **`NTF001` may not exist** (`api.md` 10–12). The analysis never mentions it, and a custom rule under that name is not something `ruff` supports out of the box. If Phase 2 shows no such rule, the "handlers MUST be thin" bullet has zero enforcement and must be re-grounded on a mechanism that does exist.

10. **Test-layout enforcement does not test the stated pass criterion** (`CLAUDE.md` 31–35). The mechanism is a pytest collection pattern, which constrains *file naming*; the pass criterion is that *every module has a counterpart*, which collection settings can never detect. And "the reviewer flags it" is not a mechanism. Either add a check that walks `src/notify/` against `tests/`, or lower the criterion to what naming rules can decide.

11. **`testing.md` fixtures are asserted but unverified.** Both the PostgreSQL-refusing `db` fixture and the autouse socket-blocking fixture are stated as existing; the analysis confirms only the CI service container, not either fixture. If Phase 2 finds them absent, these become work items rather than codified patterns.

**Sound as written — keep, and say so.** The secrets rule (real hooks in both pre-commit and CI, confirmed by the analysis, tight rationale), the flaky-test quarantine rule (`scripts/check_quarantine.py` is confirmed in CI with a decidable expiry check), and the carrier-mocking boundary rule's intent. A review that only lists defects gets read as hostile; naming what is right is how the rest gets taken seriously.

## Phase 5 — Consistency across the three files

**What I would do.** Read the documents against each other rather than one at a time, checking: the correlation-id dependency in Phase 4 item 7; whether the path globs in each rules file's front matter match the routing table at `CLAUDE.md` 59–63 (they do); whether carrier conventions are split awkwardly between `api.md` (call construction) and `testing.md` (mocking boundary) — they are complementary, not contradictory, so I would leave it; and whether any rule in a path-scoped file is really a project-wide constraint sitting in the wrong place. The "no HTTP client other than the shared one" constraint is arguably project-wide; low priority, mentioned as a consider-item only.

## Phase 6 — Write the review record and render the call

**What I would write.** `notify-svc/.mochiko/reviews/governance-draft2-review.md`, containing: the scope and standing note, the verification status of every asserted mechanism (with the Phase 1 caveat if the repo was unavailable), the four-category table, the numbered findings above with file and line for each and blocking/should-fix/consider tiers, the list of rules that are sound, and the recommendation.

**Location stop.** No reviews directory exists in this workspace, and I would not invent a convention. I would confirm the path with the requester before writing.
- **If they name a location** → write there.
- **If they want it inline** → append it as a review block below the `## Governance` region instead, keeping the draft itself untouched.
- **Default if no answer:** the path above.

**What I would not write.** I would not edit `CLAUDE.md` or either rules file. The working group authored the draft; they revise it, and the team ratifies it Thursday. I supply the change list, not the replacement text — except for the specific numeric substitutions (coverage 70/72, timeout 10.0/3.0, three retries), which I would spell out exactly so there is nothing to interpret.

**No tests to run.** There is no executable code in this workspace and I am reviewing documents. The one thing I would *check* rather than *test* is whether the coverage command in the draft is character-for-character the command in `ci.yml`; a mismatch there is the finding.

## Phase 7 — Report

**What I would report back.** A short verdict, in this shape:

> **Do not adopt Draft 2 on Thursday as written.** Seven blocking changes, four should-fix.
>
> The three that would cause real damage: observability is deferred outright when the codebase has none, and the draft's own error-handling rule silently depends on a correlation id that only observability work would create — those are one fix, not two. The coverage floor of 60% is *below* the 70% gate CI already enforces and the 72% the suite actually achieves; adopting it means either weakening a working gate or shipping a document that lies about the build. And the two-senior-sign-off requirement cannot be satisfied by a team with one senior engineer, so it will be waived within a week and take the surface's credibility with it.
>
> Three MUSTs are false the day they land — direct `request.json()` in two handlers, one unpaginated admin endpoint, two stub `downgrade()` revisions — and each needs a dated remediation item or a named exception with an expiry. Two rules are missing a required part outright: no reason given for the UTC rule, no mechanism for the reversible-migration rule. Two more claim enforcement that may not exist (`NTF001`, and the two `conftest.py` fixtures) and must be confirmed before Thursday.
>
> The secrets rule, the flaky-quarantine rule, and the carrier-mocking boundary are well-built and should carry through unchanged.
>
> **Open item:** the repository itself was not in this workspace, so every enforcement mechanism was judged against the codebase analysis rather than the code. The working group must close the mechanism-existence questions directly, or give me the repo and I will.