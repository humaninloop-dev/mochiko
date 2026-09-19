FILES-READ: `notify-svc/CLAUDE.md`, `notify-svc/.claude/rules/api.md`, `notify-svc/.claude/rules/testing.md`, `notify-svc/.mochiko/memory/codebase-analysis.md`

---

# Action plan — evaluate the notify-svc governance draft before Thursday adoption

## Phase 0 — Frame the job and pick the right procedure

**What I'd do.** Fix what kind of artifact this is before touching it. This is a governance surface — a `## Governance` region plus two scoped rules files — authored by the platform working group against a brownfield codebase analysis. That routes to the constitution-authoring procedure and its brownfield branch, and I'd consult its canonical essential-floor reference for the four baseline categories rather than working from memory.

**What I'd refuse.** I would *not* route this through the feasibility review. That procedure is for technical design artifacts and explicitly not for the governance surface; running it here would produce a verdict from the wrong rubric and would let the real question — is each standard enforceable, testable, justified, and adoptable on Thursday — go unasked.

**Authorship check.** I did not write this draft, so I'm clear to grade it. If the team later asks me to write the replacement text rather than just mark it up, that's a separate authoring pass and someone other than me has to clear the result. I'd say so up front rather than quietly become both author and grader.

**Read:** the three draft files and the analysis (done above); the essential-floor reference from the constitution skill.

---

## Phase 1 — Establish what evidence I actually have

**What I'd do.** The draft's credibility rests almost entirely on claims about mechanisms that supposedly exist or will exist: a `gitleaks` pre-commit hook, a `pytest --cov-fail-under=60` CI step, a custom `ruff` rule `NTF001`, a `check_utc` migration lint, a PR template with a threat-model section, a `db` fixture that refuses non-PostgreSQL DSNs, an autouse socket-blocking fixture, and `scripts/check_quarantine.py`. Grading enforceability means checking which of those are real.

So the first move is to see whether the repository is here at all. I already globbed the workspace: it contains exactly four markdown files. There is no `src/notify/`, no `pyproject.toml`, no `.github/workflows/ci.yml`, no `.pre-commit-config.yaml`, no `tests/conftest.py`, no `scripts/`.

**I'd do this check myself, not delegate it.** Absence is the finding that drives everything downstream, and a completeness-sensitive "is it really not there" is not something I hand to a cheap reader.

**Consequence I'd record, not paper over.** I can verify every enforcement claim only against the analysis document, which is a second-hand, three-day-old summary. That is enough to catch claims that *contradict* known facts, and not enough to confirm claims the analysis is simply silent about. I'd state that boundary explicitly in the review and ask the team for repo access before Thursday rather than guess.

**Delegation, conditional.** If the repo were mounted (or is mounted before I run this for real), I'd fire one disposable `Explore` subagent per gap, each with an explicit `model: haiku` override, one gap per spawn, each brief a pure locate-and-quote with no interpretation:
- "Does a ruff rule or plugin named `NTF001` exist? Quote its definition and the config that enables it, or report absent."
- "Quote the `--cov-fail-under` value in the CI workflow."
- "Does anything named `check_utc` exist anywhere in the repo? Quote it or report absent."
- "Does `.github/pull_request_template.md` exist, and does it contain a threat-model section? Quote it or report absent."
- "Quote the `db` fixture and any autouse fixture in `tests/conftest.py`."
- "List Alembic revisions whose `downgrade()` body is only `pass`."
- "List handlers under `src/notify/api/` that call `request.json()` or read raw query strings."

**On return I'd check:** every answer carries a file path and a quoted span; any "absent" is accompanied by what was searched so I can tell "not there" from "not found." Anything ambiguous or interpretive I re-read myself. I would not let a haiku reader decide whether a mechanism is *adequate* — only whether it exists.

---

## Phase 2 — Floor coverage pass

**What I'd do.** Check the draft against the four baseline categories, using the analysis's "as found" table as the starting state.

- Security: three principles present (secrets, input, review). But the analysis flags **no dependency scan** as a gap and the draft is silent on supply chain. Flag.
- Testing: two principles present. Covered.
- Error handling: two principles present. Covered.
- Observability: the draft says *"Not covered in this draft. We'll add monitoring later… the on-call engineer checks the Heroku logs."* The analysis rates this category **absent** — the worst of the four: `print()` and plain-text logging, no request id, no structured fields, no health route, no metrics, and alerting is literally customers emailing support.

**This is my first hard blocker.** A governance surface cannot ratify a deliberate hole in a baseline category, and it cannot do so in the one category the codebase is weakest in. Deferring monitoring "until the carrier integrations settle" is exactly backwards: the carrier integrations are where INC-37 came from. The draft must carry at least a minimum observability standard — structured logs with a request/correlation id propagated through handlers and Celery tasks, a real health endpoint (Heroku currently pings `/`), and a named alerting path that is not a customer's inbox — each with an enforcement mechanism and a pass criterion. If the team wants a phased target, that's fine, but the phase must be a dated commitment in the surface, not a paragraph promising to think about it.

---

## Phase 3 — Three-part audit, rule by rule

**What I'd do.** Build a table over all fourteen rules (nine principles, three api rules, three testing rules) with columns: enforcement mechanism named? / mechanism actually exists? / pass-fail concrete? / rationale present? / does the pass criterion actually test the rule?

Findings I'd expect to write up, each as a numbered must-fix or should-fix:

1. **Security — input: no enforcement at all.** The rule states a pass condition and a rationale but names no mechanism. "No handler reads `request.json()` directly" is a testable *state*, not a way of catching a violation. Fix: an import/call lint in the same family as the `NTF001` pattern already used in `api.md`, wired to CI.
2. **Data — time: no rationale.** Every other rule carries one; this one just stops after the pass line. A maintainer in two years cannot evaluate whether it still applies. Fix: state it (the analysis gives the material — 41 columns already `timestamptz`, two responses emitting naive strings).
3. **Data — migrations: no enforcement mechanism**, and the pass criterion — `downgrade()` runs cleanly on *a copy of production data* — is not something anyone will run on every PR. It reads as aspiration. Fix: an upgrade/downgrade round-trip against an ephemeral PostgreSQL in CI, which is enforceable on every revision; keep the production-copy exercise as a release-time check if the team wants it, stated separately.
4. **Testing — layout: enforcement and pass criterion don't match.** The mechanism is a pytest collection pattern, which catches *misnamed* files; the pass criterion is "every module under `src/notify/` has a `tests/` counterpart," which the collection pattern cannot detect at all. "The reviewer flags it" is not a mechanism. Fix: either a mirror-coverage check in CI or a pass criterion honest about what the collection pattern proves.
5. **`api.md` — "reasonable timeouts."** Unmeasurable; there is no pass/fail. The analysis hands me the real numbers: a shared `httpx.Client` with a 10s total / 3s connect timeout and three `tenacity` retries. Fix: write the numbers.
6. **`api.md` — carrier rule's pass criterion tests the wrong thing.** The rule is about timeouts and retries; the pass criterion is about which module gets imported. Import discipline is a proxy, and a fine one — but then the enforcement is *"a review-checklist item confirms no bare `httpx` import,"* which is a human promise where a banned-import lint is a two-line config. Fix: make it a lint; keep the checklist for nothing.

---

## Phase 4 — Enforcement-reality audit (invented vs. inherited machinery)

**What I'd do.** Cross every claimed mechanism against the analysis's inventory of what CI and pre-commit actually contain, and sort each into *exists* / *contradicted* / *unverifiable*.

- **Exists** (analysis confirms): `gitleaks` in pre-commit and CI; PostgreSQL service container; `scripts/check_quarantine.py` in CI; the shared `notify.carriers.http` client.
- **Contradicted:** the coverage gate. The draft says CI runs `--cov-fail-under=60`; the analysis says CI runs `--cov-fail-under=70` and current coverage is 72%. See Phase 5 — this is not a typo, it's a regression.
- **Unverifiable, and the analysis's silence is suggestive:** the `NTF001` custom ruff rule, the `check_utc` migration lint, the PR template's threat-model section, the `db` fixture's DSN refusal, and the socket-blocking autouse fixture. The analysis enumerated `pyproject.toml`, the CI workflow, and the pre-commit config in detail and mentioned none of them.

**What I'd require.** Every mechanism in the surface must be labelled as *in place* or *to be built*, and every "to be built" one needs a named owner and a date. A standard whose enforcement doesn't exist yet is not enforceable on Thursday; it's a plan wearing a MUST. This is a structural change to the draft, not a per-rule nit.

**Where I'd ask rather than invent.** I will not assume `NTF001` and `check_utc` are fiction on the strength of the analysis's silence. I'd list them as open questions to the working group with a direct request: point me at the config, or mark them as work to be done.

---

## Phase 5 — The coverage regression

**What I'd do.** Take this one on its own because it's the sharpest of the must-fixes.

The draft mandates coverage MUST NOT fall below **60%**, blocking. Today CI blocks at **70%** and the suite sits at **72%**. Adopting this draft *lowers* an existing gate: it would require someone to edit the CI config downward, and it would licence deleting roughly a tenth of the suite while remaining fully compliant. The rule's own rationale — "the suite is the only regression net a three-person team can afford" — argues against the number attached to it.

**Fix:** floor at 70% minimum, matching the gate that already holds. I'd recommend 72% (the current actual, i.e. a no-backslide ratchet) and note that ratcheting is the version that actually protects the net.

**Human stop.** The exact number is the team's call, not mine. I'd present it as: *never below 70 — that's a regression and I'd block on it; 72 as a ratchet is the recommendation.* If they choose 70, the draft is fixed and I proceed. If they insist on 60, I'd record it as an unresolved blocker in the review and escalate it as a deliberate lowering of an existing safety gate, with the coverage-deletion consequence stated in plain terms.

---

## Phase 6 — Adoptability: what breaks on Thursday morning

**What I'd do.** Run every MUST against the analysis's "as found" facts and list the rules the codebase violates the moment the draft takes effect. A governance surface adopted into instant, unacknowledged non-compliance teaches the team that the rules are decorative — that's the failure mode I care about most.

| Rule | Known violators (from the analysis) |
|---|---|
| Security — input (pydantic on every handler) | `src/notify/api/webhooks.py`, `src/notify/api/admin.py` read `request.json()` — 14 of 19 handlers compliant |
| Data — migrations (working `downgrade()`) | revisions `0012` and `0027` are `pass` stubs |
| Data — time (explicit offset on the wire) | two API responses emit naive strings |
| `api.md` — pagination (`limit`/`cursor`, clamp 200) | `GET /admin/tenants` returns all rows; the prescribed contract test would fail on day one |
| Error handling — format (`correlation_id` on every error) | analysis: *no correlation id on any response*; uncaught exceptions still return FastAPI's default 500 body, not problem+json |

**What I'd require.** Each of these needs either (a) fix-before-adoption, or (b) a named, dated exception recorded in the surface itself with an owner. Not silence. Given the volume is small and concrete — two handlers, two migrations, two responses, one endpoint — I'd push for fix-first on the endpoint and the naive timestamps and dated carve-outs for the rest.

**Human stop.** Fix-first vs. grandfather is a scheduling decision for the team lead, and Thursday is tomorrow. I'd present both branches. *If they choose fix-first:* the draft ships as written on those rules and I note the fixes as adoption preconditions. *If they choose carve-outs:* the draft must gain an exceptions block with expiry dates before I'd clear it, and I'd hold that no carve-out runs past the point where the rule stops meaning anything — the quarantine rule's own 14-day discipline is a fair template. Default if no answer arrives: carve-outs with dates, because that's the version that can actually be adopted on time without lying about the state of the code.

---

## Phase 7 — Excess and altitude

**What I'd do.** Hunt the opposite failure — weight nothing pays for.

1. **Security — review is not merely excessive, it is unsatisfiable.** It requires a written threat model on *every* PR, signed off by **two senior engineers**. The team is three engineers, **one at senior level**. There is no configuration of this team that can merge a pull request under this rule. A standard that cannot be complied with will be suspended within a week, and it will take the credibility of the other thirteen rules with it. Fix: one reviewer, and a threat model required only where it earns its cost — changes touching authentication, tenant scoping, carrier credentials, or the webhook surface. That is the narrowest scope that still answers the rationale.
2. **Hand-built machinery worth questioning.** `NTF001` (a custom ruff rule) and `check_utc` (a bespoke migration lint) are both custom tooling a three-person team with no ops will own forever. The needs are real; the question is whether they must be hand-built. Off-the-shelf import-and-call banning (ruff's banned-API / flake8-tidy-imports) covers the "no SQL in a handler" and "no bare `httpx`" intents, and a migration round-trip in CI plus a SQLAlchemy type constraint covers the timestamp intent. I'd ask the working group which requirement pays for the custom code and record their answer; if the answer is good, the rules stand.
3. **Placement.** "Testing — layout" is scoped entirely to `tests/**` and belongs in `testing.md`, not the top-level governance region, which should carry the things that bind everywhere. Minor, but the region is the surface everyone reads and it should stay short enough to be read.

---

## Phase 8 — Cross-artifact consistency

**What I'd do.** Read the three draft files against each other and against the analysis, looking for a dependency one file creates and no file supplies.

**The one I've already found and consider a blocker.** The error-handling rule requires `correlation_id` on every error body, and the traces rule leans on the same shared handler. Nothing anywhere in the draft establishes where a correlation id comes from — no rule mandates generating one at the edge, propagating it through the Celery hop, or emitting it in logs. And the section that would have carried that obligation is the observability section the draft deliberately left empty. The result: a required field with no producer, and a support workflow ("match a ticket to a log line," per its own rationale) that cannot work, because the logs are unstructured `print()` output with no request id. Fixing Phase 2 fixes this too — which is exactly why the observability hole is not a deferrable gap.

Also checked here: the analysis is dated 2026-09-06, the draft 2026-09-08, adoption is tomorrow. Three days is fresh enough that I'd treat the analysis as a valid basis, and I'd say so explicitly rather than leave it assumed.

---

## Phase 9 — Verdict, write-up, and the stop before adoption

**Verdict I expect to render: needs-revision, with adoption on Thursday blocked until the must-fix items land.** I do not expect an unbuildable-as-specified ruling — every defect here has a revision that closes it. The two-senior-reviewer rule is the only item that is strictly impossible for this team, and it's impossible by a mis-sized requirement, not by a fundamental conflict; rescoping it costs a sentence. I'd say that plainly rather than dress a fixable draft in the heavier verdict.

**Must-fix (blocks Thursday):**
1. Observability is absent from a surface that must cover it — add structured logging with a propagated request/correlation id, a health route, and a real alerting path.
2. Coverage floor of 60% is a regression against the 70% gate already in CI — raise it.
3. Two senior sign-offs on every PR is unsatisfiable with one senior engineer — rescope.
4. `correlation_id` is required but nothing produces it — resolve with (1).
5. Security-input has no enforcement mechanism — add one.
6. Migrations rule has no enforcement mechanism and an unrunnable pass criterion — replace with a CI round-trip.
7. Every claimed mechanism labelled *in place* or *to be built*, with owner and date for the latter.
8. Day-one violations (Phase 6 table) fixed or carved out with dates.

**Should-fix:** missing rationale on the timestamps rule; "reasonable timeouts" replaced with the real numbers; the carriers pass criterion aligned to its rule and its checklist replaced with a lint; the layout rule's enforcement/pass mismatch; the layout rule moved to `testing.md`; dependency scanning addressed; the custom-tooling question answered.

**What I'd write, and where.** A review document — findings, the must-fix list, and concrete replacement wording for each defective rule so the working group can apply it in an evening rather than redesign. I'd propose `notify-svc/.mochiko/reviews/governance-draft-2-review.md` and **stop to confirm the path**, because nothing in this workspace establishes a convention for review output and I'm not going to invent a directory in someone's repo. *If the user names a different path:* write there. *If the user says put it in the draft:* I'd decline to edit the draft in place — I'm the grader on this one, and the working group applies its own revisions; I'd deliver redline text instead. Default while waiting: hold the review in my report rather than write it anywhere.

**I would not mark this adopted or ratified.** That's the team's decision at Thursday's meeting, and my role ends at the recommendation.

---

## Phase 10 — What I'd report

A short verdict up front — **needs revision; do not adopt as written** — then the eight must-fix items in the order above, each with the specific evidence behind it (which analysis line contradicts which draft line), and the replacement wording. Then the should-fix list. Then two things I'd be explicit about rather than bury:

- **The limit on my evidence.** No source tree is present in this workspace, so five claimed mechanisms are unverified and I've flagged them as questions to the working group rather than findings. If someone can point me at the repo before Thursday, I'd re-run the targeted existence checks and convert those questions into answers.
- **The open decisions** waiting on a human: the exact coverage number, fix-first vs. dated carve-outs for the day-one violations, and whether the custom lints are worth a three-person team owning.