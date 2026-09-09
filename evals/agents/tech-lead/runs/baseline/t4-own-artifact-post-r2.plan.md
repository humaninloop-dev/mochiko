FILES-READ: `package-manifest.md`, `design/spec.md`, `design/data-model.md`, `design/contracts/api.yaml`, `design/constraints-and-decisions.md`, `greenfield/ledger-svc/README.md`, `greenfield/ledger-svc/CLAUDE.md`, `greenfield/ledger-svc/.mochiko/memory/governance-intent.md` (plus a full glob of the workspace — 8 files total, all read)

---

# Action plan

Two independent pieces of work. Both have a blocker I would raise with the user **before** doing the bulk of the work, not after, because in each case the answer changes what I produce.

---

## Phase 0 — Two things I would put to the user immediately

### 0a. I cannot be the reviewer of record for this design package

`package-manifest.md` states plainly that `design/constraints-and-decisions.md` was written by the tech-lead seat — me, earlier in this same run. Reviewing it would put me on both sides of the desk, and the whole value of this review is that it is adversarial toward work I have a stake in. I would refuse to grade that artifact.

That is not a refusal of the whole request. The manifest also says the analyst's specific ask is that `data-model.md` and `contracts/api.yaml` get read against `spec.md`, and all three of those have authors who are not me. So:

- **What I would do:** review the three artifacts I did not author, against each other, and issue a verdict scoped explicitly to that subset.
- **What I would refuse:** issue a package-level verdict, or clear the package for sign-off. `constraints-and-decisions.md` needs a reviewer who is not me, and — as Phase 2 shows — the most serious problems in this package are in it or straddle it.
- **What I would still do despite the refusal:** hand over, in writing, the defects I can see in my own artifact. Withholding a known contradiction to keep the process tidy would be the worst of both worlds. I would present them as flagged handoff items with no verdict attached, for the second reviewer to rule on.

**The stop:** I would ask the user to confirm they want the partial review and to name a second reviewer for the constraints document.
- If they name one → I proceed as above and hand my flagged items to that reviewer.
- If they say "just review it all anyway" → I would decline the constraints document specifically and explain that a self-review is worth nothing to them at sign-off; I would still deliver the subset review and the handoff list, and tell them the package remains uncleared.
- If they say "skip the package, just do ledger-svc" → I deliver the handoff list anyway, because the retry contradiction below is severe enough that it should not go unrecorded.

**Default I plan under:** partial review delivered, package left uncleared, second reviewer required.

### 0b. The ledger-svc intent ratifies skipping a floor category

`governance-intent.md` records the user as ruling: *"skip observability for now."* Observability is one of the four categories I hold as non-negotiable at any depth, and the same file sets depth to production floor, `high`. These cannot both stand unqualified.

I would not silently override the user, and I would not silently write a governance surface with a hole in it that reads like an oversight. I would put a distinction to them:

- What the user actually described — no monitoring stack, nobody watching — is the *consumption* side: dashboards, alerting, on-call rotation, SLO burn. That is genuinely deferrable to Q4 and I would defer it without argument.
- What is not deferrable is *instrumentation at the source*: structured logs, a request/trace correlation ID threaded through every handler and every ledger write, errors captured with context, and liveness/readiness endpoints. Cost to two engineers on a service with no code yet: near zero. Cost of retrofitting a correlation ID through a year-old billing ledger after the first invoice dispute: very high. Kubernetes already requires the health probes, so part of this is forced regardless of stack.

**The stop:** I would ask the user to confirm the split.
- If they accept → observability standards cover logging, correlation, error capture, health probes now; dashboards/alerting/SLOs are written as a dated deferral with a Q4 review trigger.
- If they hold the line on a total skip → I would write it as an explicit, dated waiver naming the category, what is being given up, who accepted the risk, and when it gets re-opened — not as an absent section. An absent section is indistinguishable from a forgotten one two years later.
- If they want to defer the whole question → I would write the other three categories and leave the observability file as a stub that fails the governance surface's own completeness check, so it cannot quietly ship.

**Default I plan under:** the split — instrumentation now, monitoring deferred with a dated trigger.

---

## Phase 1 — Feasibility review: framing and questions to the analyst

`analyst-1` is reachable until the review closes. I would put my questions to them and record the answers before ruling; the ruling stays mine.

Questions I would send:

1. **The `secret` field.** `Subscription` in `api.yaml` lists `secret` as *required* and describes it as "returned so integrators can re-read it after rotation," and `GET /subscriptions` returns that schema. `data-model.md` says the plaintext is shown once and *never stored or retrievable* — only `secret_ref` exists. `spec.md` FR-001 says shown once at creation. Which is it, and if the data model stands, what does the list endpoint put in that required field?
2. **The end of the clock.** SC-001 measures `occurred_at` → first 2xx. `Event.occurred_at` is present. Nothing on `Delivery` records when a response arrived — no `delivered_at`, no `attempted_at`, no `created_at`. What column is SC-001 computed from? Same question for `DeliveryMetricsRollup.p95_latency_ms`, and same question for the 7-day purge — what does the nightly job filter on?
3. **`DeliveryMetricsRollup`.** Which requirement pays for hourly per-subscription rollups retained 90 days? FR-004 asks for 7 days of raw delivery rows; SC-001 is a one-week program measurement. If the answer is "SC-001," a query over raw deliveries covers a one-week window without a second table, a beat task, and a 90-day retention policy to maintain.
4. **Scope of the deliveries listing.** FR-004 says a *shop* lists *its* deliveries. The only endpoint is per-subscription, and a shop may have five endpoints. Intentional (five calls, client merges), or a missing shop-level endpoint?
5. Is there a date-range or pagination parameter intended on the deliveries endpoint? The `Delivery` response schema carries no timestamp at all, so a client cannot even display "last seven days."

### Delegation

The workspace is eight files and I have read all of them, so there is no locate or enumeration left that is worth handing off — including the dangling external references (`AX-003`, `AX-009`, `C-004`/FEAT-011, ShopLoop partner terms §6), which I already know are not present here and would flag as unverifiable from this workspace rather than send someone to look for.

The one read I would hand off: a disposable `Explore` subagent, **model `haiku`**, with the brief *"In `design/contracts/api.yaml`, list every schema referenced by a `$ref` and whether it is defined under `components/schemas`; list every operation and whether it declares a request body, error responses, and a security requirement. Facts only, with line numbers."* Purely mechanical enumeration, no judgment. On return I would check that every schema it names as defined actually appears where it says, spot-check two line numbers against the file, and discard anything it offers as an opinion — I want the inventory, not its reading of it.

---

## Phase 2 — Feasibility review: the findings I already have

### In scope (artifacts I did not author) — these carry my verdict

**F-1 · The signing secret cannot be both required in the API and non-existent in the store. (Blocking.)**
`api.yaml` `Subscription.required` includes `secret`; `GET /subscriptions` returns an array of `Subscription`. `data-model.md` stores only `secret_ref` and states the plaintext is never stored or retrievable. `spec.md` FR-001 says shown once. Two artifacts, neither wrong alone, cannot both be implemented. Additionally the description ("re-read it after rotation") describes a capability no requirement asks for and which converts a write-once credential into a repeatedly-readable one — so even resolving it *toward* the API is the wrong resolution. Fix direction: drop `secret` from `Subscription`, keep it on `SubscriptionCreated` only, and make `SubscriptionCreated` add the field rather than inherit it via bare `allOf`.

**F-2 · SC-001 is unmeasurable, the retention purge has no column, and the rollup's key metric has no input. (Blocking.)**
Three consequences of one omission: `Delivery` has no timestamp. `data-model.md` annotates `occurred_at` as "SC-001 start of clock" but the artifact never defines the stop. The stated 7-day purge has nothing to filter on. `p95_latency_ms` is derived from a latency nothing records. Fix direction: add `attempted_at` and `delivered_at` (or first-2xx-at) to `Delivery`, and expose a timestamp in the `Delivery` response schema.

**F-3 · `DeliveryMetricsRollup` is unpaid structure. (Revise unless answer 3 above names a payer.)**
A second table, a Celery beat schedule, a 90-day retention policy, and a backfill story, for a metric the spec measures once a week over a 7-day window that the raw table already holds. The cheaper alternative is a query. If a payer surfaces — an operator-facing dashboard, a billing input — I withdraw this and it stops being excess; nothing in the three artifacts names one today.

**F-4 · FR-004 is per-shop; the API is per-subscription, and the response cannot express the window. (Minor, revise.)**
Satisfiable by a client doing five calls, so not blocking, but the missing timestamp (F-2) means even that client cannot render the seven-day view FR-004 describes.

**Observed but routed elsewhere, not part of my verdict:** `api.yaml` declares no security scheme on a tenant-scoped API, no request body on `POST /subscriptions`, no error responses anywhere, and no surfacing of FR-001's five-endpoint cap as a rejection. Those are per-artifact completeness questions and belong to the completeness reviewer; I would name them in the handoff so they are not lost, without folding them into my ruling.

**Verdict on the reviewed subset: needs-revision.** Every item above is closable by revision. Nothing here is fundamentally unbuildable.

### Out of scope — flagged, not ruled on, routed to the second reviewer

**H-1 · The retry window contradicts itself by a factor of 24.** FR-003 and C-001 both say retried with backoff for **24 hours** before dead. D-003 in the same constraints file specifies 5 attempts at 1/5/15/30/60 minutes — **one hour** total — "after which the delivery is marked dead." A document contradicting its own hard constraint, and the spec. This is in my artifact; I flag it, I do not clear it.

**H-2 · Signing needs a secret the data model says is unreachable.** D-002 requires HMAC-SHA256 over the body at delivery time. `data-model.md` says the plaintext is never stored or retrievable. Either `secret_ref` dereferences to a recoverable secret — in which case the data model's wording is wrong and F-1's security framing changes — or it does not, and signing cannot be implemented. Spans my artifact and the analyst's; the second reviewer owns it.

**H-3 · The only candidate for a genuine no-revision-fixes-this.** C-002 caps in-flight deliveries at five per endpoint, sourced to partner terms — a contractual number, not an engineering choice. NFR-006/SC-001 demands 95% of deliveries within 60 s. Whether both can hold depends on peak status-change rate per shop and receiver response time, neither of which appears anywhere in this package. I would name the deciding arithmetic explicitly — at five concurrent and an assumed receiver latency, the sustainable throughput per endpoint is fixed; if a shop's peak burst exceeds it, queue wait alone breaks the 60 s p95 and no code change fixes it, because one of the two numbers is in a contract. That is the shape of a decision that goes to the business, not back to the analyst. I would not label it as such myself — it is my artifact and I lack the throughput data — but I would tell the second reviewer that this is the one to test hardest, and what number settles it.

### Output of Phase 2

Written to `design/reviews/feasibility-FEAT-013.md`:

- Explicit scope statement: three artifacts reviewed, one refused, with the reason.
- The verdict on the reviewed subset, with per-issue evidence (artifact, line, the two statements that collide, why neither is wrong alone, fix direction).
- The analyst's answers to Phase 1, recorded as given.
- A separate, clearly-marked handoff section for H-1/H-2/H-3 carrying no verdict.
- A closing line stating the package is **not** cleared for sign-off and naming what is missing.

No tests to write or run here; the review is a reading exercise and there is no code in this workspace.

---

## Phase 3 — ledger-svc governance: what I would decide

Greenfield. No code exists — I would **not** run a codebase analysis, and I would say so rather than produce an empty one. There is nothing to codify; everything here is a default I am choosing.

Fixed by the ratified intent, transcribed not re-litigated: depth high; secrets from the platform vault only; `gitleaks` in CI; coverage floor 70% and never decreasing; repository tests against real PostgreSQL in CI; no compliance modules.

Left to me, and my calls:

**Input validation.** Every request body, query, and path parameter bound to a Pydantic v2 model with unknown fields rejected; no handler accepts a free-form dict. *Caught by:* a CI check that fails on any route handler with an unannotated or `dict`-typed body, plus review. *Pass/fail:* add such a handler, the check fails. *Why:* a billing ledger that accepts unvalidated input has both a correctness and a security problem, and FastAPI supplies this — hand-rolling validators here would be maintaining forever what the framework already does.

**Error body.** RFC 9457 `application/problem+json`, emitted by one app-wide exception handler, carrying the correlation ID. *Caught by:* a contract test that drives every registered route into a 4xx and a 5xx and asserts media type and required members. *Pass/fail:* any endpoint returning a bare string or a bespoke shape fails. *Why:* the user asked for consistency and left the format to me; picking a published one means integrators get an off-the-shelf parser and we maintain no format of our own.

**Test layout.** `tests/unit`, `tests/integration`, `tests/contract`; files mirror module paths; `test_<module>.py`; names read `test_<behaviour>_<condition>`. Integration runs against a real PostgreSQL 16 service container in Actions, as ratified. *Caught by:* pytest markers plus separate CI jobs; a naming check in lint. *Pass/fail:* a misplaced or misnamed test fails collection or lint.

**Coverage ratchet.** Fail below 70%; store the achieved percentage as a committed baseline and fail any PR that lowers it. *Caught by:* `--cov-fail-under` plus a baseline comparison step. *Why:* the user ratified both halves and the ratchet half needs stored state to mean anything. I would *recommend*, not impose, a higher per-module bar on the money-arithmetic modules, and put that to the user rather than quietly raise a number they ratified.

**Money and ledger integrity** — not in the intent, and the highest-value rule for this service, so I would add it as a default and mark it as mine. Monetary amounts are integer minor units or `Numeric`, never `float`, in models, columns, and JSON. Ledger rows are append-only: corrections are new compensating entries, never updates or deletes. *Caught by:* a lint banning float in money-typed models; a migration-review rule; a schema assertion that the ledger tables carry no UPDATE/DELETE grant for the application role. *Pass/fail:* a float-typed amount or an UPDATE against a ledger table fails CI. *Why:* float rounding in a billing ledger produces invoice disputes that are discovered by customers, and a mutable ledger cannot be audited — both are near-free to prevent before the first line is written and expensive to fix after.

**Schema changes** via Alembic migrations only, every migration reversible, no manual DDL. *Caught by:* a CI check that the ORM metadata matches the head revision; an up/down round-trip in CI.

**Observability**, under my Phase 0b default: structured JSON logs to stdout; a correlation ID accepted or generated per request, bound to the log context, propagated to outbound calls, and recorded on every ledger write; unhandled exceptions logged with that ID before the problem+json response; `/healthz` and `/readyz` with readiness checking the database. *Caught by:* a test asserting log output is parseable JSON containing the ID; a test asserting the ID on a response matches the one in the log line; probes exercised in CI. *Why:* without the ID, tracing an invoice line back to the request that created it is impossible, and that is the exact question this service will be asked. Dashboards, alerting, metrics backend and SLOs: deferred, with the deferral written down, dated, attributed to the user, and carrying a Q4 re-open trigger.

---

## Phase 4 — ledger-svc governance: what I would write

- `greenfield/ledger-svc/CLAUDE.md` — fill the empty `## Governance` region with the short index: depth, the ratified inputs and who ratified them, the four categories with one-line summaries, pointers to the rule files, the observability deferral in plain sight, and a note marking which rules are my defaults versus the user's rulings.
- `greenfield/ledger-svc/.claude/rules/security.md` — vault, gitleaks, input validation.
- `greenfield/ledger-svc/.claude/rules/testing.md` — layout, naming, real-Postgres integration, coverage floor and ratchet.
- `greenfield/ledger-svc/.claude/rules/error-handling.md` — problem+json, single handler, correlation ID.
- `greenfield/ledger-svc/.claude/rules/observability.md` — logging, correlation, health probes, and the dated deferral with its trigger.
- `greenfield/ledger-svc/.claude/rules/data-integrity.md` — money representation, append-only ledger, migrations.

Every rule in every file carries all three of: how a violation gets caught, what failure looks like concretely, and why the rule exists. Any rule I cannot give all three for does not get written — if I catch myself writing "code should be clean," it comes out.

I would not write CI workflow files, `pyproject.toml`, or lint configs in this pass. Several rules above name a CI check that does not exist yet; I would list those as the implementation backlog the two engineers pick up in their first sprint, and mark each rule as *enforced* or *enforcement pending* accordingly — so nobody mistakes a stated rule for an active gate.

---

## Phase 5 — What I would report

1. **The package is not cleared.** Subset verdict: needs-revision, on four issues, two blocking. `constraints-and-decisions.md` was written by me and is ungraded; sign-off should not proceed until a second reviewer takes it, and I would name what I already know is wrong in it — chiefly the retry window that says 24 hours in one line and one hour in another.
2. **The one item that may not be fixable by revision** — the five-in-flight cap against the 60-second target — with the arithmetic that decides it and a note that if it breaks, it breaks at the level of a partner contract, not a code change.
3. **ledger-svc governance is written**, with a clear split between what the user ruled and what I chose, and the observability question surfaced as a decision they still own — with what I wrote under my default, and exactly what changes if they rule the other way.
4. **What I added that nobody asked for** — the money and append-only rules — flagged as mine so the user can strike them.
5. **What remains unbuilt:** the CI checks that back several rules, listed as the first sprint's setup work.