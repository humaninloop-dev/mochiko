FILES-READ: `package-manifest.md`, `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine-excerpt.md`

---

# Action plan — feasibility review of the FEAT-011 design package

## Phase 0 — Eligibility and scope check (before any reviewing)

**What I'd do.** Confirm I am allowed to render this verdict at all, and fix the boundary of what I'm judging.

- Authorship: the manifest attributes `data-model.md`, `contracts/api.yaml`, and `constraints-and-decisions.md` to the technical-analyst seat (`analyst-1`), and `spec.md` to the requirements-analyst seat. I authored none of them. I can review.
- Under review: `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`.
- Not under review, used only as the fixed backdrop I test the package against: `architecture/spine-excerpt.md`. Its rows are decided and built; the package does not get to contradict them silently, but I do not grade the spine.
- Nothing in this package is a governance surface, so there is no artifact here I have to hand off to a different validator on domain grounds.

**What I'd refuse.** Judgments that belong to the other reviewer and to the user, and I will say so explicitly in the report rather than smuggle them in: whether `spec.md` is complete on its own, whether US-003 has acceptance scenarios (it visibly has none), whether alternatives to D-005/D-006 were formally weighed, whether SC-001 is well-formed as a measurement. I will touch SC-001 only where it collides with another artifact.

**Stop point.** None. This phase cannot block.

## Phase 1 — Load the review procedure

Invoke `mochiko:review-feasibility` and run the rest of this plan under its contradiction/altitude classes, per-issue evidence fields, and its 3-state verdict. Everything below is what I expect that procedure to surface in *this* package; if the procedure names a class I have not covered, I sweep the six files again for it before writing anything.

## Phase 2 — Build the cross-artifact claim table

**What I'd do.** Extract every hard claim into one table so contradictions become arithmetic rather than impressions. Columns: claim, source file+line, type (obligation / physical limit / decision / measurement), and what it forbids.

Rows I already know go in it, with the line anchors I'd cite:

- FR-002 obligation, `spec.md:40` — accept a change up to T-2h "on any day including the delivery day."
- US-002 scenario, `spec.md:24-26` — 14:00 window moved at 11:30, carrier holds the parcel.
- SC-002, `spec.md:54` — 90% of same-day rescue attempts before the cut-off succeed.
- C-001 physical limit, `constraints-and-decisions.md:7-11` — aggregator rejects every change after vehicle loading at 05:00–06:00 on the delivery day; only a phone call holds a parcel; contract fixed until 2028; same-day change API declined in writing 2026-08-19.
- NFR-002 / SC-001, `constraints-and-decisions.md:25` and `spec.md:52` — 300 ms p95 at the gateway.
- C-003 physical limit, `constraints-and-decisions.md:15-17` — aggregator p50 420 ms, p95 900 ms, p99 2.1 s, no faster tier.
- C-002 policy, `:12-14` and D-003, `:36-38` — carrier confirmation happens inside the request.
- D-005, `:40-44` — handler publishes to a new Redis Streams bus, a new single-worker consumer group does the write, handler blocks on the second event.
- C-004 + the forbidden-attributes block, `constraints-and-decisions.md:18-19` and `data-model.md:19-22` — no recipient contact data stored, logged, or transmitted, anywhere in this service.
- `contracts/api.yaml:46-52` and `:56-60` — `recipient_email` is a **required** request field, **required** response field, and described as "stored on the change record."
- C-005 / FR-005 / NFR-005 — 24-month retention with requester and time.
- `data-model.md:29` — `shipment_id` FK with **cascade delete**; `:36` — the existing nightly retention purge deletes older rows.
- D-004, `:39` — optimistic locking on `window_version`.
- Spine row AX-004, `spine-excerpt.md:24-27` — Celery beat plus `apply_async(eta=…)` is decided and built, and FEAT-006 already re-times reminders on change; drift none.
- D-006, `:45-48` — new 600-line scheduler package with polling loop, advisory-lock leader election, retry ledger.
- IP-001, `:52` — "nothing new."

**What I'd write.** Nothing to disk yet; the table is working material that becomes the evidence column of the review.

## Phase 3 — Hunt the impossible combinations

For each candidate I capture: the two-or-more artifacts in collision, exact line anchors, the reasoning that makes it a collision rather than a preference, whether any revision inside this package can close it, and the cheapest closing move if one exists. Candidates I go in expecting to confirm or kill:

**3.1 Same-day rescue is physically foreclosed (my leading escalation candidate).** FR-002 and US-002 promise a change at T-2h on the delivery day; C-001 says the parcel is on a vehicle from 06:00 and every subsequent change is rejected, with a written refusal from the aggregator and a contract fixed to 2028. The 14:00-window-moved-at-11:30 scenario is five hours past the lock. SC-002 then measures a success rate for a path that cannot succeed. No rearrangement of endpoints, schema, or decisions inside this package closes this — the closing moves are all outside engineering's authority: renegotiate the aggregator terms, fund a human depot-call workflow as the same-day path, or narrow FR-002 to pre-loading changes and drop or restate US-002 and SC-002. I test hard for an out before I call it: is there any window on the delivery day between 00:00 and 05:00 where FR-002 still holds, and does that rump case keep US-002 meaningful? My reading is it does not — the story is explicitly about a recipient discovering mid-morning that they are out.

**3.2 The 300 ms budget cannot contain the carrier call.** NFR-002 demands 300 ms at p95; C-003 measures the aggregator alone at 900 ms p95 with no faster tier; C-002 and D-003 put that call inside the request; D-005 then adds a Redis publish, a single-worker hop, a write, and a second event the handler blocks on, all inside the same request. The floor is roughly 3× the ceiling before any of our own code runs. Closing this needs either the number to move (business) or the synchronous confirmation to go (blocked by C-002, which is itself an incident-driven support policy, also not engineering's call). I check whether C-003's cited source is anywhere in this workspace — it is not; `ops/carrier-latency-2026-Q3.md` is not present, so I record the figure as seat-asserted and ask for it rather than treat it as verified.

**3.3 The contract transmits data the constraint forbids.** `recipient_email` is required on both the request and the response and is stated to be stored, while C-004 and the data model's forbidden-attributes block prohibit storing, logging, or transmitting recipient contact data anywhere in this service, on the authority of the data-processing agreement. This one *is* closable inside the package: remove the field, and route the carrier's confirmation via `carrier_ref` / `recipient_token`, with the shop holding contact data. I will confirm the carrier's confirmation channel actually works without an email being handed over — if it does not, this promotes into the escalation bucket alongside 3.1.

**3.4 Retention obligation defeated by the schema.** C-005/FR-005 requires 24 months of change history; `WindowChange.shipment_id` cascades on delete, and a nightly purge already exists in the built worker. A shipment deleted or purged inside 24 months takes its legally-retained change records with it. Closable inside the package (restrict the FK, or detach retention from shipment lifetime), so it lands as a revision item, not an escalation.

**3.5 The optimistic lock does not lock.** D-004 claims `window_version` prevents two concurrent changes from both confirming, but under D-005 the version bump happens in the writer worker after the handler has already called the carrier. Two requests can both read the same version, both confirm with the aggregator, and only then serialize. The guarantee D-004 asserts is not delivered by the design D-005 specifies. This collision exists only across the two decisions — neither is wrong alone.

**3.6 The state machine locks at the wrong moment and has no exit.** `data-model.md:46` transitions `confirmed → locked` at the two-hour cut-off, but C-001 says the real irreversibility is vehicle loading at 05:00–06:00. The model therefore describes a system that accepts changes the carrier will reject between loading and T-2h. `locked` also has no outbound transition, and the `502` carrier-did-not-confirm path in the contract has no corresponding state.

**3.7 Missing recipient identity on the endpoints.** `recipient_token` is described as the only recipient identifier, yet neither endpoint carries it or any auth. As written, `PUT /shipments/{id}/window` lets anyone holding a shipment id move a stranger's delivery. I flag this as a buildability gap in the contract-vs-model pair and hand the broader "is the security story complete" question to the other reviewer rather than expanding my lane.

## Phase 4 — Hunt the excess

Same evidence discipline, opposite direction: what is here that no requirement or constraint pays for, and what is paid for but hand-built in a solved category.

**4.1 The Redis Streams bus and the blocking writer (D-005).** No requirement in `spec.md` and no constraint in the package asks for an event log; the stated rationale is "later consumers," which is speculative, and "keeps the handler free of writes," which the spine contradicts — `api → db` is synchronous SQL, one transaction per request, and that is the built pattern. The machinery costs a new bus, a new single-worker consumer group that is a single point of failure, a broken concurrency guarantee (3.5), and latency added to a budget already blown (3.2). Cheaper alternative I will name concretely: write the `window_changes` row and the `shipments` update in the request's existing transaction. If a later consumer genuinely appears, a Celery task over the existing broker already covers it — that path is built and in use.

**4.2 The hand-built scheduler (D-006).** FR-006 is real, so the *need* is paid for; the *building* is not. AX-004 is a decided and built ruling for exactly this: Celery beat for periodic work, `apply_async(eta=…)` for one-off timed sends — and FEAT-006 already queues reminders that way and revokes and re-queues them when the order changes, which is precisely the re-timing D-006's rationale claims requires new machinery. Six hundred lines of polling loop, advisory-lock leader election, and retry ledger become this team's forever. I will record this as both excess and an unexplained departure from a standing built ruling, and note that if there is a real gap in `eta` sends the package must say what it is, since the existing as-built appears to cover the stated need.

**4.3 IP-001 understates provisioning.** "Nothing new" sits beside a new Streams bus and a new schedule table with leader election, on the single Redis that is already the Celery broker and result backend. Minor next to the rest, but it is a claim about the world that is not true, and provisioning claims get believed.

## Phase 5 — Put the questions to the seat before ruling

I do not rule from my own reading alone where the seat may hold a fact I lack. I'd send `analyst-1` a numbered, answerable list and put the replies on the record verbatim:

1. C-001 puts loading at 05:00–06:00 and the aggregator's same-day refusal in writing. What path makes US-002's 11:30 change to a 14:00 window succeed? If it is the depot phone call, which artifact specifies that operator workflow, and who staffs it?
2. Which measurement or component budget lets NFR-002's 300 ms hold when C-003 puts the in-request carrier call at 900 ms p95? Is the 300 ms perhaps meant to exclude the carrier leg — and if so, what does the recipient actually wait?
3. `ops/carrier-latency-2026-Q3.md` is not in this package. Can you supply it or its figures?
4. Which requirement pays for the Redis Streams bus and the blocking writer in D-005? What breaks if the row is written in the request transaction?
5. AX-004 is decided and built, and FEAT-006 already re-times reminders with `apply_async(eta=…)`. What can FR-006 not do on that path that justifies 600 new lines?
6. `recipient_email` is required in the contract and forbidden by C-004 and your own data model. Which is wrong? If the carrier needs an address for its confirmation, does that survive the data-processing agreement at all?
7. Under D-005, `window_version` is incremented by the worker after the carrier call. What stops two concurrent changes from both confirming?
8. Does `cascade delete` on `WindowChange` survive the 24-month retention obligation when a shipment is deleted or purged?
9. Should the `locked` state trigger at loading rather than T-2h, and what leaves `locked`?

The manifest names only `analyst-1` as reachable, but questions 1 and 2 reach into `spec.md`, authored by the requirements-analyst seat. I'd route them through `analyst-1` as the packager and flag in the report that the spec author was not directly reachable — the wording of FR-002 and SC-002 may need that seat's hand, and I will not pretend the answers I get are theirs.

**Stop point.** I wait for these answers before the verdict; the answers can kill findings (a depot-call workflow I haven't seen, a 300 ms figure that excludes the carrier leg) but the ruling stays mine.

## Phase 6 — Delegation

Honest accounting for this workspace: there is almost nothing to farm out. Six small files, all read, and every finding turns on reading two of them *against* each other — interpretive work I do myself. The two things I would hand to a cheap disposable reader with an explicit haiku override, each a single bounded gap, are: (a) locate and quote the p95/p99 rows of `ops/carrier-latency-2026-Q3.md` in the wider repo if it exists outside this package, returning the file path and the lines or "not found"; and (b) locate FEAT-006's reminder scheduling and revoke/re-queue call sites, returning file paths and the `apply_async` lines. On return I check that quoted lines actually exist at the cited paths before either one enters my evidence; absence of (a) does not become evidence of anything — it becomes question 3 to the seat.

Neither file is present in this workspace, so under the current scope I expect to run the review with no delegation at all and mark both facts as seat-asserted.

## Phase 7 — Render the verdict

**Where it lands.** `reviews/FEAT-011-feasibility-review.md` alongside the package, one entry per issue carrying the colliding artifacts, exact line anchors, the reasoning, the seat's answer as given, and the closing move where one exists.

**How I expect it to fall, and the branches:**

- If 3.1 stands after the seat's answer — no path exists for a same-day change and no engineering revision creates one — the verdict is **infeasible**, and I hold that word. This is not a loud "needs revision"; it is a decision the business owns: renegotiate with the aggregator, fund a human depot path, or cut the promise. I will resist any pressure to soften it into a revision list, because a revision list would send this back to engineering to solve something engineering cannot solve. 3.2 likely reinforces the same call for the same reason — the closing moves belong to whoever owns the 300 ms number and the post-INC-52 support policy, not to `analyst-1`.
- If the seat produces a genuine same-day path I have not seen (a specified depot-call workflow, or a written aggregator carve-out) and a defensible reading of the 300 ms budget, then 3.1 and 3.2 collapse and the verdict becomes **needs-revision** on the remaining set: the `recipient_email` contradiction, the cascade-delete-versus-retention defect, the broken optimistic lock, the mis-timed and exit-less `locked` state, the missing recipient identity on the endpoints, and the two excess findings against D-005 and D-006.
- **Approved** is not reachable from what I have read; I would need 3.3 and 3.4 alone to disappear, and they are visible on the face of the documents.

My stated default while awaiting answers: **infeasible**, driven by 3.1.

**The human stop.** Before this reaches the user for sign-off I surface the verdict and say plainly what is being confirmed: that a same-day rescue as specified cannot be built against the current carrier agreement, and that the decision is whether to change the agreement, fund an operational workaround, or change the promise. Design sign-off does not proceed until that is answered.

## Phase 8 — Report

I close with: the verdict and the one sentence that carries it; the escalation items separated from the revisable ones so nobody mistakes a business decision for an engineering ticket; the excess items with the cheaper alternative named for each (existing request transaction for D-005; the already-built `apply_async(eta=…)` path for D-006); the two facts I could not verify because their sources are outside this package; the note that the spec author was not reachable and FR-002/SC-002 wording may need that seat; and an explicit list of what I did not judge and who owns it — per-artifact completeness, alternatives analysis, and the user's own call on scope.