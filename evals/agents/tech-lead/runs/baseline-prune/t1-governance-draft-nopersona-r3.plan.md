FILES-READ: `notify-svc/CLAUDE.md`, `notify-svc/.claude/rules/api.md`, `notify-svc/.claude/rules/testing.md`, `notify-svc/.mochiko/memory/codebase-analysis.md`

---

# Action plan — evaluate the notify-svc governance draft before Thursday adoption

Timing note that shapes everything below: the draft is dated 2026-09-08, today is Wednesday 2026-09-09, so adoption is tomorrow. Findings must be triaged into "must change before the meeting" vs. "adopt with a dated follow-up," not delivered as an undifferentiated list.

## Phase 1 — Scope, and establish what is actually verifiable

Read (already done): the four files above. Confirm by globbing that the workspace contains **only** those four documents — no `src/notify/`, no `tests/`, no `pyproject.toml`, no `.github/workflows/ci.yml`, no `.pre-commit-config.yaml`.

Consequence I would state up front in the deliverable: this is a document review, not a repo audit. Every claim in the draft of the form "Enforcement: X exists" can only be checked against `codebase-analysis.md`, which is itself a 3-day-old snapshot taken by `detect-stack.sh`. So each finding gets a confidence label: **contradicted by the analysis**, **unevidenced in the analysis** (may exist, may not), or **internally inconsistent** (needs no repo access to prove).

**Stop point.** I would ask the team: do you want me to review against the analysis only, or can you point me at the real repo so I can confirm whether the "contract suite," the `NTF001` ruff rule, and the `check_utc` lint exist?
- If they supply the repo → re-run Phase 2 against actual `ci.yml`, `pyproject.toml`, `tests/conftest.py`, `src/notify/api/`, and `alembic/versions/`, and upgrade every "unevidenced" finding to confirmed present/absent.
- If they don't, or no answer before the meeting → **my default**: proceed against the analysis, and mark unevidenced enforcement claims as items the team must confirm in the room, since several are load-bearing.

## Phase 2 — Build a claim ledger

For each of the 8 principles in `## Governance` and the 6 rules across the two rules files, extract four fields: the MUST, the stated enforcement, the stated pass criterion, and the supporting line in `codebase-analysis.md`. Then classify each as: already true and already enforced (codifying reality) / true but unenforced / false today (repo violates on day one) / enforcement doesn't exist / not achievable at all.

This ledger is the working artifact; it's what makes the findings arguable rather than opinion.

## Phase 3 — The findings I expect to land, and how I'd verify each

**Blockers — must change before adoption.**

1. *Two senior sign-offs is arithmetically impossible.* `CLAUDE.md:5` and the analysis both say three engineers, one senior. A rule requiring two senior sign-offs plus a written threat model on every PR blocks every merge from Thursday onward. Proposed replacement: one reviewer, plus a short security-impact note in the PR template that is only required when the change touches auth, tenant scoping, carrier credentials, or a new outbound call. Verification: none needed beyond the two documents.

2. *The coverage rule is a downgrade, not a floor.* The draft mandates 60% and `--cov-fail-under=60`; the analysis records `--cov-fail-under=70` already blocking in CI at 72% actual. Adopting the draft as written means editing CI to weaken an existing gate. Proposed replacement: 70%, matching what is already enforced and passing. I would not accept lowering it, and I'd say so plainly — if the working group deliberately wants headroom, that's a decision for them to make explicitly in the meeting, not something to slip in as a floor.

3. *"The contract suite" is load-bearing and unevidenced.* It is the sole enforcement for four separate rules — error format, no traces, UTC `Z` suffixes (`CLAUDE.md:36–49`) and the pagination clamp (`api.md:19–22`). It appears nowhere in the analysis. If it doesn't exist, four principles ship with zero enforcement on Thursday. This is the single biggest hidden cost in the draft and should be the headline item. Ask in the room; if absent, either build it as a named work item with an owner and date, or restate those four as aspirations with a target date rather than as enforced rules.

4. *Three rules the repo violates on day one.* The analysis names each violation precisely, and the draft doesn't acknowledge any of them:
   - pydantic validation — 14 of 19 handlers; `src/notify/api/webhooks.py` and `src/notify/api/admin.py` read `request.json()` directly.
   - pagination — `GET /admin/tenants` returns all rows (6 of 7 endpoints comply).
   - UTC — all 41 columns are `timestamptz` (good), but two API responses emit naive strings.
   
   Each needs either a pre-Thursday fix or a written, dated exception. Otherwise the team adopts rules they are already breaking, which is how governance gets ignored.

5. *Observability is deferred, but an adopted rule depends on it.* `CLAUDE.md:54–57` defers monitoring; the error-format rule requires `correlation_id` on every error body and justifies itself by support matching a ticket to a log line. The analysis says: `print()` and plain-text `basicConfig`, no request id, no structured fields. You cannot emit a correlation id you don't generate, and you can't match it to a log line that isn't structured. Minimum addition: request-id middleware, structured JSON logs carrying it, and a real `/health` route (the analysis notes Heroku currently pings `/`). Either add that small floor or drop `correlation_id` from the error contract — the two positions can't both stand.

**Enforcement that won't work as written.**

6. `NTF001` (`api.md:10–12`) is described as a "custom `ruff` rule." Ruff has no user-defined rule mechanism of that shape; this is a lint plugin someone has to write and maintain, on a team with no ops. Recommend replacing with a cheap CI grep/AST check for `session.execute` under `src/notify/api/`, which gets the same guarantee for an hour of work.

7. The migrations rule (`CLAUDE.md:50–52`) has no enforcement at all — only a pass criterion of running `downgrade()` "on a copy of production data." I'd flag that separately: routinely cloning production into a test environment to satisfy a lint is a data-handling risk for a service holding customer parcel data. Recommend an upgrade/downgrade round-trip on a seeded ephemeral database in CI instead. Also: the analysis says revisions `0012` and `0027` are `pass` stubs and nothing checks this today — so the rule needs a decision on whether those two are backfilled or explicitly grandfathered.

8. The carrier-client rule (`api.md:13–18`) rests on a "review-checklist item," which is the weakest mechanism in the whole draft, in a document that otherwise prefers mechanical checks. A CI grep for `httpx` imports outside the shared client is trivial. Separately, "reasonable timeouts" is vague where the codebase already has concrete values — codify the actual `timeout=httpx.Timeout(10.0, connect=3.0)` and three `tenacity` retries from the analysis, so the rule can be checked.

9. The test-layout pass criterion (`CLAUDE.md:31–35`) — "every module under `src/notify/` has a `tests/` counterpart" — is stronger than the rule it accompanies and isn't checked by the stated enforcement (`pytest` collection patterns verify naming, not mirroring). Weaken the pass criterion to match what is actually enforced, or add the check.

**Fine as written — say so, so the team doesn't re-litigate.** Secrets (gitleaks is in both pre-commit and CI per the analysis); real-PostgreSQL repository tests (service container exists); flake quarantine with a 14-day expiry (`scripts/check_quarantine.py` already runs in CI); carrier mocking at the `notify.carriers.http` boundary. For the three `conftest.py`-based enforcements claimed in `testing.md`, mark them "confirm the fixture actually does this" — the analysis evidences the CI container and the quarantine script, not the fixture behaviour.

**Gap the draft ignores.** The analysis lists "no dependency scan" under Security. Not a blocker; recommend Dependabot or `pip-audit` as a follow-up item, not a Thursday change.

## Phase 4 — Cross-document consistency

Check that `CLAUDE.md:59–63` path globs match the frontmatter in both rules files (they do: `src/notify/api/**` + `src/notify/carriers/**`, and `tests/**`), that no rule contradicts a principle, and that nothing is stated twice in two places where it could drift. Also flag two structural omissions: the draft names no owner or review cadence for itself, and defines no exceptions/waiver process — which it needs precisely because several rules will start out violated.

## Phase 5 — Triage and write up

Sort every finding into: **change before Thursday** (items 1, 2, and a decision on 3), **adopt with a named owner and date** (4, 5, 6, 7, 8), **wording fix** (9), **confirmed good**, **future**. Ordering matters more than completeness here — the team has one meeting.

I would write one file: `notify-svc/governance-review-2026-09-09.md` — the ledger table, the findings in triage order with the evidence line cited for each, and ready-to-paste replacement text for every bullet I'm asking to change (so the meeting is a yes/no on concrete wording, not a drafting session).

**Stop point.** I would not edit `CLAUDE.md` or the two rules files myself before the team has met — the card asks me to evaluate and say what must change, and the draft is the working group's document heading into an adoption vote.
- If the team says "just apply the redlines" → I'd edit the `## Governance` region and the two rules files directly, leaving the rest of `CLAUDE.md` untouched, and keep the memo as the rationale record.
- If they want it reviewed first → memo only. **This is my default.**

## Phase 6 — Tests and delegation

No test run is possible or meaningful here: the workspace holds four documents and no code. Instead, the memo ends with a short verification checklist for whoever has the repo checked out — confirm the `--cov-fail-under` value in `ci.yml`, grep `src/notify/api/` for `request.json()`, check whether a contract suite and the `NTF001`/`check_utc` checks exist, and confirm the three `tests/conftest.py` enforcements. Those four checks convert most of my "unevidenced" labels into facts in about ten minutes.

No delegation. Four documents totalling under 150 lines is well within a single pass, and a subagent would face the same missing-repo limitation I do, so splitting the work would add coordination cost and no new information.

## What I'd report at the end

A short summary naming the five blockers in priority order — two senior sign-offs on a one-senior team, the coverage floor written below the gate already in CI, the contract suite that four rules depend on and nothing evidences, the three rules the repo violates today, and the observability deferral that contradicts the `correlation_id` requirement — plus the pointer to the memo, an explicit note that enforcement claims were checked against a 3-day-old analysis rather than the repo, and the one thing I'm recommending against on principle: don't lower the 70% gate, and don't restore production data to satisfy a migration check.