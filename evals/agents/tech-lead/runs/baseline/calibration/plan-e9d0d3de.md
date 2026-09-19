FILES-READ: `package-manifest.md`, `design/spec.md`, `design/data-model.md`, `design/contracts/api.yaml`, `design/constraints-and-decisions.md`, `greenfield/ledger-svc/README.md`, `greenfield/ledger-svc/CLAUDE.md`, `greenfield/ledger-svc/.claude/rules/.gitkeep`, `greenfield/ledger-svc/.mochiko/memory/governance-intent.md`

---

# Action Plan

Two independent pieces of work. I would do the FEAT-013 review first because it gates a user sign-off, then the ledger-svc governance defaults.

---

## PART A — Feasibility review of the FEAT-013 design package

### Phase A1 — Establish what I am allowed to review, and stop on it

**What I would do:** The manifest tells me `design/constraints-and-decisions.md` was written by the tech-lead seat — me, this run, an earlier turn. I do not review my own work. That is not a formality I can wave through here, because the sharpest problems in this package sit *inside* that file and in its intersection with the others. So before writing anything I would stop and put the scope question to the user.

**What I would confirm with the user:**

> Three of the four artifacts in this package were written by seats other than mine; `constraints-and-decisions.md` was written by me. I cannot render a verdict on my own artifact. I have found problems that live in it — including one that makes the retry behaviour unbuildable as written. How do you want this handled?

**The branches:**

- **(a) Route `constraints-and-decisions.md` to another reviewer / re-open it with the analyst.** Preferred. I review the other three against `spec.md` and against each other, and hand over my observations on the constraints file as *unruled input* for whoever grades it.
- **(b) User accepts a scoped review.** I review the three artifacts I did not write, render a verdict covering only those, and attach my constraints-file findings as a clearly-labelled non-verdict appendix.
- **(c) User insists I rule on all four.** I would refuse the verdict on my own file specifically, and say so plainly in the review. I would still surface the findings — withholding a known blocker to protect a procedural line helps nobody — but the word "verdict" would not attach to my own artifact.

**Default I proceed under for the rest of this plan: (b).** The analyst explicitly asked that `data-model.md` and `contracts/api.yaml` at minimum be read against `spec.md`, and that is squarely mine to do. Findings touching the constraints file get recorded as flagged observations, not rulings.

**Refusal, stated up front:** I will not sign the constraints file as feasible on my own authority.

### Phase A2 — Load the procedure and verify the reference surface

**What I would do:** Load the feasibility-review skill and follow its procedure and issue format rather than improvising one.

**Delegation (one cheap read, disposable `Explore` subagent, `model: haiku`):**

- *Brief:* "In this workspace only, search every file for the literal tokens `AX-003`, `AX-009`, `C-003`, `FEAT-011`, and `ShopLoop partner terms`. Report each hit as file path plus line number and the line text. If a token has no hits, say so explicitly. Do not interpret or summarise."
- *Why it goes out:* pure locate over a tiny tree, no judgment.
- *What I check on return:* whether these are dangling references. My read says they are — nothing in this workspace defines AX-003, AX-009, FEAT-011's C-004, or the partner terms. I would also note `C-003` is absent from the constraints file's own numbering (C-001, C-002, C-004), which suggests a dropped constraint.
- *Note:* absence here does **not** drive my verdict on its own — these could live outside the workspace. I would flag them as unverifiable-in-scope, not as errors.

I do the interpretive cross-reading myself.

### Phase A3 — Read the artifacts against each other

Four pairwise passes, hunting for combinations that are impossible to build. Here is what I have already found on the read above; this phase is where I harden each one.

**Issue 1 — BLOCKER. The signing secret cannot be both un-retrievable and returned by the list endpoint.**
`api.yaml`'s `Subscription` schema lists `secret` as a *required* property and describes it as "returned so integrators can re-read it after rotation" — and that schema is what `GET /subscriptions` serves. `data-model.md` stores only `secret_ref` and states the plaintext "is never stored or retrievable." `spec.md` FR-001 says the secret is "shown once at creation." Two of these three cannot both ship. This is exactly the class I am looking for: neither artifact is wrong alone, and together they describe a system nobody can build. It also happens to be a credential-disclosure hole — a list endpoint handing back every endpoint's signing secret defeats the point of signing. Both artifacts are the analyst's; this one is fully within my scope to rule on.
**Resolution direction I would state:** keep `secret` on `SubscriptionCreated` only; drop it from the base `Subscription`; if re-reading after rotation is a real need, that is a rotation endpoint returning a *new* secret once, not a readable field — and that is a spec change, not a schema tweak.

**Issue 2 — BLOCKER (flagged, not ruled — my own artifact). Retry window: 24 hours vs one hour.**
`spec.md` FR-003 requires retry with backoff "for 24 hours before it is marked dead." `constraints-and-decisions.md` C-001 restates 24 hours. D-003 in that same file specifies five attempts at 1/5/15/30/60 minutes — "one hour in total" — then dead. `data-model.md` carries `next_attempt_at` and a `dead` status per FR-003 with no schedule of its own, so it is neutral. D-003 contradicts both the requirement it claims to serve and C-001 sitting eight lines above it. The file is internally inconsistent and contradicts the spec. Since I wrote it, this goes in as a flagged finding for another reviewer, with my recommendation: D-003 is wrong, extend the schedule to cover 24 hours or take FR-003 back to the requirements author.

**Issue 3 — Throughput conflict (flagged; spans my artifact and the spec).**
C-002 caps in-flight deliveries at five per endpoint. FR-002 fans every status change out to every active endpoint. NFR-006/SC-001 demands 95% delivered within 60 s of `occurred_at`. Five concurrent in-flight with a slow receiver is a queue, and a queue plus a 60-second budget is an arithmetic question nobody in this package has answered. I would state the arithmetic explicitly — at a five-second receiver response, five in flight gives one delivery per second per endpoint, so any shop with a burst above ~60 status changes a minute misses SC-001 by construction — and ask the analyst whether a burst-rate assumption exists. Not automatically infeasible; it *is* an unpaid-for gap that must be closed before anyone builds a scheduler against it.

**Issue 4 — Excess, and the hand-built kind. `DeliveryMetricsRollup`.**
Nothing in `spec.md` or the constraints asks for per-subscription hourly buckets of delivered/failed/dead plus p95 latency retained for 90 days. FR-004 asks for seven days of deliveries with attempt count, last response code, and outcome — the `Delivery` table already answers that. SC-001 is measured over one week, not ninety days. So the retention is unpaid-for on its face. Worse, this is a metrics aggregation and percentile pipeline — a Celery beat job, a rollup table, a purge — hand-built in a category that monitoring tooling has solved for decades. Two engineers would own that computation, its backfill, and its correctness forever. I would ask the analyst which requirement pays for it, and record the answer verbatim. My position absent a good answer: delete the entity; emit delivery latency as a metric and let the platform's monitoring compute p95.

**Issue 5 — FR-002 fan-out contradicts the subscription event filter.**
FR-002: deliver every status change to *every* active endpoint of the shop. `data-model.md` gives `Subscription.events` — "subscribed event types" — which by definition means some active endpoints do not receive some events. With only `status.changed` defined today this is latent, but the model and the requirement disagree about who decides fan-out. Cheap to fix now, expensive after the first second event type.

**Issue 6 — `response_code` nullability mismatch.**
`data-model.md` marks it nullable; `api.yaml` declares `type: integer` with no null. A `pending` delivery has no response code, and FR-004 explicitly returns pending rows. The contract cannot serialise the row the model produces. Small, concrete, fixable.

**Issue 7 — FR-001's five-endpoint cap has no home.**
"Up to five endpoints" per shop appears in the spec and nowhere else — no constraint, no model rule, no contract response for the sixth attempt. This is a limit with no enforcement point. I would flag it as needing a stated mechanism (unique-count check plus a defined 4xx) rather than leaving it as prose.

**Deliberately not my call:** whether each artifact is individually complete, whether the analyst weighed alternatives, whether NFR-006 is measurable standing alone, missing auth responses and pagination in `api.yaml`. Different reviewer. I would name them as out of scope so nobody assumes I cleared them.

### Phase A4 — Put questions to the analyst

`analyst-1` is reachable until the review closes, so their answers go on the record before I rule.

**Questions to `analyst-1`:**
1. Which requirement pays for `DeliveryMetricsRollup`, and specifically for 90-day retention?
2. Was returning `secret` on the list endpoint intended, or inherited from `SubscriptionCreated`? Is there a real rotation-readback need behind the description?
3. Does a per-shop event-burst rate assumption exist that makes C-002 and SC-001 compatible?
4. Should fan-out honour `Subscription.events` or FR-002's "every active endpoint"?

Their answers are recorded as given. The verdict stays mine.

### Phase A5 — Write the review

**Write:** `design/reviews/feasibility-FEAT-013.md` — I would confirm the review path/naming convention against project convention first and adopt whatever exists rather than inventing one.

**Contents:** scope statement naming which artifacts I reviewed and which I recused myself from and why; each issue with the exact file-and-line evidence on both sides of the contradiction, the class of problem, and a resolution direction; the analyst's answers verbatim; the out-of-scope list; the verdict.

**Verdict I expect to land on: `needs-revision`,** for the three artifacts in scope — Issue 1 alone blocks sign-off, and Issues 4–7 are all closeable by revision. I would say explicitly why this is *not* `infeasible`: nothing here is a fundamental conflict that no revision can close. Every finding has a landing spot. I reserve `infeasible` for genuine business-level escalation and I will not inflate a fixable package into one.

**The one thing that could move it:** if the analyst answers question 3 with "no burst assumption exists and the partner terms fix the cap at five," and the requirements author holds SC-001 at 95%/60 s, then C-002 and SC-001 are a business decision — the contract cap versus the latency promise — and that piece escalates rather than being revised in engineering.

**No tests run.** These are documents; there is no code and nothing executable in this package. My "test" is the pairwise read itself.

**Report at the end of Part A:** verdict, the blocker in one sentence, the recusal and how it was handled, the count of issues by class, and the named decision awaiting the user or the requirements author.

---

## PART B — Greenfield governance defaults for `ledger-svc`

### Phase B1 — Confirm the ground is actually empty

**Delegation (one cheap read, disposable `Explore` subagent, `model: haiku`):**

- *Brief:* "Under `greenfield/ledger-svc/`, list every file including dotfiles, with size in bytes. Report the exact contents of `.claude/rules/`. Report whether any `.py`, `.toml`, `.cfg`, `.yaml`, or `.yml` file exists anywhere under that path. Facts only."
- *What I check on return:* that the README's claim of "no code yet" holds, and that `.claude/rules/` really contains only `.gitkeep`. If code turns up, this stops being greenfield and I switch to the brownfield branch — which means running the codebase analysis first and codifying what exists before adding anything. I expect it to hold.

Then load the constitution-authoring skill and follow its greenfield branch, and read its canonical description of the four baseline categories rather than working from memory.

### Phase B2 — Stop on observability

This is the one thing in the intent file I cannot simply execute.

The user ratified: *"there is no monitoring stack before launch and nobody to watch it — skip observability for now, we will add it after the Q4 release."* Observability is one of the four categories every governance surface must address. I am not going to quietly overrule a ratified instruction, and I am not going to quietly drop a baseline category. So I stop and say what I actually think.

**What I would put to the user:**

> Your reasoning is sound for the expensive half. Dashboards, alerting, metric exporters, and on-call runbooks all need a stack and a human, and you have neither before Q4 — deferring those is the right call and I would make it too.
>
> But most of what the baseline asks for costs nothing to run and everything to retrofit. Structured JSON logs, a request/correlation ID threaded through, a `/health` endpoint Kubernetes is going to probe whether we write one or not, and unhandled errors logged with stack and context. None of that needs a monitoring stack or a watcher. It needs a logging library configured on day one. Added later, it means touching every handler in the service.
>
> And this is a **billing ledger**. When invoicing disagrees with the ledger — and it will — the only way to answer "what did this tenant actually get charged and why" is a trail that already exists. There is no reconstructing it after the fact.
>
> I want to write the cheap half now and record the expensive half as an explicit, dated Q4 deferral. Confirm?

**The branches:**

- **(a) User agrees.** Write the minimum-cost observability rule now; record the deferred items as a dated obligation with a Q4 trigger.
- **(b) User holds the line at literal "skip".** I comply — it is their ratified call, not mine — but the governance surface records the observability category as *deliberately deferred by ratified user decision on 2026-09-08*, with my stated objection and the retrofit cost attached. A silently missing category looks like an oversight; a documented deferral with a named owner and trigger does not. I will not leave the category blank and unexplained.
- **(c) User wants the whole baseline.** Straightforward — write it in full.

**Default I proceed under: (a).** It honours what the user was actually protecting against — cost and staffing — while keeping the service debuggable, and audit-capable for money movement.

### Phase B3 — Fill the gaps left to me

The intent file ratifies some items and hands me the rest. I take the ratified ones as given and set opinionated defaults for the open ones. Every rule I write carries how it is caught, what failing looks like, and why it exists — a rule missing any of those three does not go in.

**Ratified, transcribed as-is (I do not relitigate these):** depth at production floor / `high`; secrets from the platform vault only, never from files; `gitleaks` in CI; coverage gate at 70% with a no-decrease ratchet; repository tests against a real PostgreSQL in CI; no compliance modules.

**Open, and my defaults:**

- **Input validation** → Pydantic v2 models at every FastAPI boundary, `extra="forbid"`, no `dict`/`Any` request or response bodies. *Caught by:* a lint check for un-annotated route signatures plus a test that posts an unknown field and expects 422. *Why:* a billing service that silently accepts fields it does not understand will eventually accept an amount it does not understand.
- **Error body** → RFC 9457 `application/problem+json` with `type`, `title`, `status`, `detail`, `instance`, plus a correlation ID; one shared exception handler, no per-route error shapes. *Caught by:* a contract test asserting every non-2xx across the route table returns problem+json with the required fields. *Why:* the user asked for consistency; picking an existing standard means no bespoke format to document or defend.
- **Test layout and naming** → `tests/unit/` and `tests/integration/` mirroring module paths, pytest, `test_<behaviour_being_asserted>`. Integration tests own the real-Postgres tier. *Caught by:* a collection check that fails on tests outside those roots; naming enforced in review. *Why:* two engineers and no established habits — the cost of deciding this now is zero and the cost of untangling it at fifty test files is not.
- **Monetary correctness** → amounts stored and computed as integer minor units or `NUMERIC`, never float or binary floating-point at any layer; ledger entries append-only, corrections written as reversing entries rather than updates; write paths idempotent on a caller-supplied key. *Caught by:* a schema check rejecting `float`/`double precision` on money columns, plus a migration review gate. *Why:* this is a billing ledger. Float rounding and in-place mutation of financial records are both the kind of defect discovered by a customer, and both are unfixable retroactively.
- **Observability minimum** (pending B2) → structured JSON logs to stdout, a correlation ID on every request and propagated to logs, `/health` and `/ready` endpoints, all unhandled exceptions logged with stack and correlation ID, and no secrets or full ledger amounts in logs at info level. *Caught by:* a test asserting log output parses as JSON and carries the correlation ID; a test hitting the health endpoints. *Why:* zero runtime cost, no stack required, and it is the only way to answer a billing dispute later.

**Scope discipline:** I write governance, not design. No framework layering, no repository-pattern mandates, no directory architecture beyond the test roots. Two engineers will ignore a surface that reads like a rulebook, and an ignored standard is worse than none.

### Phase B4 — Write it

**Files I would write:**

- `greenfield/ledger-svc/CLAUDE.md` — fill the empty `## Governance` region with the depth setting, a short summary of each of the four categories, the observability deferral record (per the B2 ruling), and pointers into the rule files.
- `greenfield/ledger-svc/.claude/rules/security.md`
- `greenfield/ledger-svc/.claude/rules/testing.md`
- `greenfield/ledger-svc/.claude/rules/error-handling.md`
- `greenfield/ledger-svc/.claude/rules/observability.md`
- `greenfield/ledger-svc/.claude/rules/ledger-integrity.md`

I would leave `.claude/rules/.gitkeep` alone. I would not touch `governance-intent.md` — it is the ratified record of what the user decided and is not mine to edit.

Each rule file: the requirement in RFC 2119 terms, how a violation gets caught, what pass and fail look like concretely, and the reason it exists. Where a rule came from the user I attribute it to the ratified intent; where it is my default I say so, so a future maintainer knows which lines they may argue with.

**Tests:** I would not write application tests — there is no application. What I would specify is the CI job set the rules depend on: `gitleaks`, `pytest --cov --cov-fail-under=70` plus the ratchet comparison against a stored baseline, the Postgres service container for the integration tier, and the money-column schema check. Expected result once code exists: all green on an empty scaffold, and each gate demonstrably failing when deliberately violated. I would note that a gate nobody has ever seen fail is a gate nobody should trust, and recommend the first sprint prove each one red.

**What I would flag:** the 70% coverage floor plus never-decreasing is the user's ratified call and I will implement it, but I would note once that a ratchet on a service with no code starts at 100% on the first trivial commit and can lock the team into an accidental high-water mark. I would set the baseline to be established at the first real feature merge, not the scaffold commit, and say so in the rule.

**Report at the end of Part B:** files written; which rules came from ratified intent versus my defaults; how the observability tension was resolved and under whose ruling; the CI jobs the surface now assumes exist; and the one open item — the coverage baseline anchor point — for the user to confirm.

---

## Sequencing and final report

Part A first — it blocks a sign-off. Both parts pause at their stated stop points (review scope in A1, observability in B2) and would resume on the user's ruling; under this plan I continue on the stated defaults.

**Closing report:** the FEAT-013 verdict and its one blocking issue, plus the recusal and how it was handled; the ledger-svc governance surface as written with the observability decision called out by name; and the two decisions still sitting with the user.