FILES-READ: `package-manifest.md`, `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine-excerpt.md`

---

# Action plan — feasibility review of the FEAT-011 design package

## Phase 0 — Scope and authorship check (done as part of reading)

**What I do:** Fix what is and is not under review before I judge anything.

- Under review: `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`.
- Not under review, read as context only: `architecture/spine-excerpt.md`. It is the standing architecture record, a governance-class surface with its own separate validation path. I read it hard — it is my strongest evidence — but I do not issue findings *against* it. Where the design conflicts with it, the finding lands on the design.
- Authorship: three artifacts carry the technical-analyst seat (`analyst-1`), `spec.md` carries the requirements-analyst seat. I authored none of them, so I am clear to grade all four.

**Refusal I would register up front:** `constraints-and-decisions.md` cites `ops/carrier-latency-2026-Q3.md` (C-003) and a written aggregator refusal dated 2026-08-19 (C-001). Neither is in this workspace, and there is no notify-svc source tree here either. I will not treat the absence as a defect and I will not invent substitutes. I take the stated constraint figures at face value for the analysis and record explicitly that C-001 and C-003 were unverified against source; if the whole verdict turns on one of them — and it will turn on C-001 — I ask the seat to produce it rather than ruling on an unseen document.

**Delegation decision:** none. The corpus is six files totalling a few hundred lines, and the entire value of this review is reading each artifact *against* every other — interpretive, completeness-sensitive work I cannot hand off without losing the thing I am looking for. There is no locate, enumeration, or bulk sweep here worth a cheap-read spawn; spawning one would be ceremony. If the seat's answers point at real source files (the ops latency note, the FEAT-006 reminder code), I would then spawn one disposable read agent on Haiku per file with a brief of "quote the p95 figure and its measurement window" / "quote how FEAT-006 schedules and re-times a reminder, with file and line," and on return I would check that what comes back is a quotation with a path, not a paraphrase — anything paraphrased I read myself.

## Phase 1 — Build the cross-artifact matrix

**What I do:** Lay every requirement, constraint, NFR and decision on one grid and walk each pair, hunting for the combination that no single artifact reveals. I am trying to prove this cannot be built. Specifically I check each requirement against each hard constraint; each technology decision against the constraints it must satisfy *simultaneously*; each contract field against the data model and against the data constraints; and each decision against what the spine says already exists and is already ruled.

**What I produce:** an internal list of candidate findings, each carrying the two-or-more artifacts it lives between, the exact lines, and what breaks when both are true.

From my read, these are the candidates I carry into Phase 2 (each stated as the collision, not the complaint):

1. **Same-day rescue vs. the carrier contract.** FR-002 requires accepting a change up to two hours before window start *on the delivery day*; US-002's acceptance scenario is a 14:00 window moved at 11:30; SC-002 sets a 90% success bar on exactly those attempts. C-001 says parcels leave the depot between 05:00 and 06:00 on the delivery day and the aggregator then rejects every change with `WINDOW_LOCKED`, that only a phone call holds a parcel, that the aggregator declined a same-day change API in writing, and that the agreement is fixed until 2028. An 11:30 request on the delivery day is on the far side of that lock by five and a half hours. No arrangement of the software makes the aggregator accept it. This is the candidate that decides the verdict.
2. **Confirmed-within-300ms vs. a 900ms dependency held inside the request.** SC-001/NFR-002 fix 300 ms p95 at the gateway for a *confirmed* change. C-002 and FR-003 forbid showing an unconfirmed window, and D-003 makes the handler wait for the aggregator inside the request. C-003 puts aggregator p95 at 900 ms with no faster tier. D-005 then adds a full Redis Streams round trip *inside the same request*, and D-002 attaches a 10 s budget with three retries. The floor of the synchronous path is already triple the ceiling.
3. **Recipient email in the contract vs. the no-contact-data rule.** `api.yaml` makes `recipient_email` a required request field, documents it as "stored on the change record," and makes it required in the response body. `data-model.md` forbids exactly this in the Forbidden attributes note and defines no such column on `WindowChange`; C-004 forbids storing, logging *or transmitting* it and cites the data-processing agreement; the model states `recipient_token` is the only recipient identifier. Three artifacts say one thing and the contract says the opposite — and a required response field means it is transmitted on every success.
4. **Cascade delete vs. 24-month retention.** `WindowChange.shipment_id` is specified `cascade delete`, while FR-005/C-005/NFR-005 require the change record to survive 24 months for consumer-protection record-keeping. Deleting a shipment destroys the compliance record ahead of time. (The nightly purge cited for retention is a floor-adjacent mechanism; the cascade is a hole beneath it.)
5. **A state transition with no actor, and two different lock rules.** The state machine has `confirmed → locked` "when the two-hour cut-off passes" — a transition triggered by the passage of time, with nothing named that fires it, and unreachable from `change_pending`. Separately, the design now carries two incompatible notions of when a window locks: two hours before the window (FR-004, the API's 409) and 05:00–06:00 depot loading (C-001). The contract has no response defined for the second one.
6. **Excess — a message bus to reach a database the handler is already holding open.** D-005 routes a write through a new Redis Streams bus and a new single-worker consumer group, then blocks the handler on the return event, to write two rows in the same PostgreSQL that D-001 keeps and that the spine says the API already writes synchronously, one transaction per request. It costs the latency budget in finding 2, breaks that one-transaction-per-request atomicity, undercuts D-004 by moving the version check out of the process making the carrier call, and introduces a single-worker component every write now depends on. The stated payment is "an event log for later consumers" — but `WindowChange` is already an append-only history by the model's own words, and no requirement names a later consumer. The cheaper alternative is the one already in the building: write it in the handler's existing session.
7. **Excess — 600 hand-built lines in a category this codebase already solved.** D-006 proposes a new scheduler package with a schedule table, a 30-second polling loop, advisory-lock leader election and a retry ledger, for FR-006 reminders. The spine's AX-004 row is decided and built, rules `apply_async(eta=…)` for one-off timed sends, and records as-built that FEAT-006 reminders are already queued that way *and revoked and re-queued when the order changes* — which is precisely the re-timing D-006 claims it needs custom machinery for. Celery beat and Redis are already running. The requirement is real; building it ourselves is not paid for, and it is drift from a standing ruling that the design does not acknowledge or seek to amend. FR-006 also traces to a P2 story, so it is carrying the most bespoke code in the package for the least urgent requirement.

## Phase 2 — Put the questions to `analyst-1` before ruling

**What I do:** The manifest says the technical-analyst seat is reachable until the review closes. I ask before I rule, and their answers go on the record whether or not they change my mind. The verdict stays mine.

Questions I send, in one round:

- **On C-001 vs FR-002/US-002/SC-002:** you recorded the depot-loading lock and the aggregator's written refusal yourself. How does an 11:30 change on the delivery day reach a parcel that left the depot at 06:00? Is there a path that is not a phone call to the depot? Can you produce the 2026-08-19 refusal and §4.3?
- **On the latency budget:** with confirmation held inside the request and the aggregator at 900 ms p95, what is your expected end-to-end p95 at the gateway, including the Streams round trip? Which number do you believe is wrong — SC-001, or C-002's confirm-before-showing policy?
- **On `recipient_email`:** which requirement pays for putting it in the request and the response, given C-004 forbids transmitting it and the data model has no column for it? Is the description "stored on the change record" a mistake, or a decision that contradicts the model?
- **On the cascade delete:** how does a change record survive 24 months if its shipment is deleted?
- **On D-005:** which requirement pays for the bus? Name the later consumer. What does it give us that writing in the handler's session does not?
- **On D-006:** AX-004 is decided and built and rules `apply_async(eta=…)`, and FEAT-006 already revokes and re-queues on change. What can the 600-line package do that this cannot? If the answer is exact-time sends, what is the accuracy the existing mechanism fails to hit, and what requirement sets that bar?
- **On the state machine:** what fires `confirmed → locked`, and what happens to a shipment sitting in `change_pending` when the cut-off passes?

**Stop / branch — this is the human-decision point.** The verdict itself is mine and I do not hand it off. But if the seat confirms there is no path to a same-day change short of renegotiating a contract fixed until 2028 or standing up a manual phone-call process, that is a business decision above the design, and I stop and put it to the user before the package proceeds:

- **If the seat produces a real mechanism** (an aggregator hold endpoint, a depot integration, a pre-06:00 cut-off the spec could adopt) → finding 1 drops to needs-revision: the spec's cut-off rule and SC-002 get rewritten against the real lock time, and the contract gains a response for the loaded-parcel case.
- **If the seat confirms no mechanism exists** → the verdict is **infeasible** on finding 1, and I escalate it as a business choice with three named options: cut US-002/FR-002's delivery-day clause and re-baseline SC-002; fund a manual depot-call path with its own service levels; or open the carrier agreement. I will not soften this into "needs-revision" — a conflict no revision of these artifacts can close is not a revision request, and collapsing it into one buries a decision the user is entitled to make.
- **If the seat is unreachable or does not answer** → I rule on the artifacts as written, which lands in the same place, and record the questions as unanswered.

Findings 2 through 7 are revisable in every branch and do not depend on the answer.

## Phase 3 — Write the review

**What I write:** `reviews/FEAT-011-feasibility.md` in this workspace. No output path was named in the card; if the user prefers it beside the package as `feasibility-review.md`, I take that instead — I would confirm the location and default to `reviews/FEAT-011-feasibility.md` absent an answer.

**Structure per finding:** the collision in one sentence; the artifacts and exact lines on both sides; what concretely breaks when both are true; whether any revision to these artifacts can close it; the seat's answer verbatim; and for the excess findings, the named cheaper alternative already present in this system. Contradiction findings and excess findings stay separately labelled — they call for different responses and I will not blur them.

**Verdict.** One of three, stated once, at the top:
- **infeasible** if the depot-loading conflict survives the question round — which on the evidence in front of me is where I expect to land. The review then leads with that single conflict, lists 2–7 as work that is real but subordinate to the business decision, and states plainly that no rewrite of the design closes finding 1.
- **needs-revision** if finding 1 resolves, since 2, 3, 4 and 5 are each independently disqualifying and 6 and 7 remove work the codebase already does.
- **approved** only if every one of the seven falls, which on this evidence it does not.

**No test to write or run.** There is no source tree, no build and no runnable check in this workspace; this review is document-against-document, and I would say so rather than manufacture a check that proves nothing.

## Phase 4 — Report

I report to the user: the verdict and the one sentence that carries it; the count of contradictions versus excess findings; the two decisions that would change the shape of the build if fixed now (drop the bus, use the scheduling mechanism already ruled and built); the escalation — that same-day rescue as specified is blocked by a carrier agreement fixed until 2028, with the three options and my recommendation; and the honest boundary of the review: C-001 and C-003 rest on documents I could not open, and if the depot-loading constraint is wrong, the headline verdict changes and I want to see the paper before the user acts on it.