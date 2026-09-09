FILES-READ: `package-manifest.md`, `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine-excerpt.md`

---

# Action plan — feasibility review of the FEAT-011 design package

## Phase 0 — Fix scope and confirm I'm clear to review

**Do:** Establish what is under review and that I did not author any of it.

- Under review: `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`.
- Not under review, read as context only: `architecture/spine-excerpt.md`. The manifest says so, and it is an architecture surface with its own owner and its own validator — I read it to test the design against what is built, I do not grade it.
- Authorship: the manifest and file headers attribute three artifacts to `analyst-1` and the spec to the requirements-analyst seat. I wrote none of them, so the reviewer-is-not-the-author separation holds. **If I found my own name on any of them I would stop and hand that artifact to a different reviewer** rather than clear my own work.

**Refuse up front:** I will not rewrite these artifacts to fix what I find — the fixes belong to the seat that wrote them. I will not judge whether each artifact is individually complete, whether alternatives were weighed, or whether an NFR is well-formed on its own; a different review covers that. My question is only: can these pieces be built together as written, and is anything here carrying weight nothing pays for.

## Phase 1 — Build a claim ledger before hunting

**Do:** Extract every load-bearing claim into a single table so I can read the artifacts against each other rather than one at a time. Columns: claim, artifact + line, what it fixes, whether it is externally fixed (contract/regulation/measured) or self-chosen.

Specifically I would pull: the six functional requirements and two success criteria; the five hard constraints; the two non-functional requirements; the six technology decisions; the entity attributes and the forbidden-attribute rule; the state machine transitions; both endpoint schemas including required fields; and the three spine facts that touch this feature (one transaction per request, the carrier HTTP client's 10 s / 3-retry budget, and the standing ruling that timed sends go through the existing scheduler with revoke-and-requeue, already proven by FEAT-006).

Marking each claim externally-fixed vs self-chosen is what later decides whether a conflict is revisable or escalation-grade.

## Phase 2 — Hunt contradictions pairwise

**Do:** Walk requirement × constraint, requirement × decision, decision × decision, contract × data model, and design × built architecture. For each hit, capture both sides with file and line, state the collision in one sentence, and say what would have to change to close it. These are the collisions I already see and would harden with evidence:

1. **Same-day rescue vs the carrier contract.** The requirement to accept a change up to two hours before the window start "on any day including the delivery day," and the story's worked example of moving a 14:00 window at 11:30, sit against a hard constraint saying parcels load between 05:00 and 06:00 on the delivery day and the aggregator rejects every change after loading — with only a phone call to the depot as recourse, the agreement fixed until 2028, and the aggregator's written refusal of a same-day API dated 2026-08-19. Two P1 stories and a success criterion ("90% of same-day rescue attempts succeed") rest on a capability the vendor has declined in writing. No revision of this design closes that.
2. **Latency budget vs measured carrier latency.** Confirmation must reach the recipient within 300 ms at p95 at the gateway, while the aggregator's own measured p95 for a window change is 900 ms with no faster tier — and the design deliberately calls the aggregator inside the request and waits, because the support policy after the returned-parcel incident forbids showing an unconfirmed window. The in-request path cannot be three times faster than the call it contains. The event-bus round trip adds to it, not subtracts.
3. **Recipient email in the contract vs the forbidden-attribute rule.** Both the request and the response bodies make `recipient_email` a *required* field, and the request field's own description says it is stored on the change record — against a data-processing agreement annex that forbids storing, logging, or transmitting recipient contact data anywhere in this service, a data model that states the same rule explicitly, and a change entity that has no such column to store it in. Neither the contract alone nor the data model alone shows this; only the pair does.
4. **The reminder has no way to reach anyone.** A reminder must be sent 24 hours before the window and re-timed when the window moves, yet the recipient is identified only by an opaque token and this service may hold no contact data. No artifact names a relay through the shop or the carrier. As written, the reminder requirement cannot be built inside the stated data boundary — which I suspect is the pressure that produced the email field in item 3.
5. **Two different lock moments.** The state machine locks a shipment when the two-hour cut-off passes; the carrier constraint locks it at vehicle loading, 05:00–06:00. For any window later than 08:00 these disagree, and the design has no state for "we accepted it, the carrier can no longer take it."
6. **No exit from `change_pending`.** The carrier client budget is 10 seconds with three retries, and the contract has a 502 for a carrier that did not confirm — but the state machine has no transition out of `change_pending` on timeout or error, only on acceptance or refusal. It also blows the latency budget by a further order of magnitude.
7. **Optimistic locking split across processes.** The version check that is supposed to stop two concurrent changes both confirming is claimed by one decision, while another decision moves the write into a separate single-worker consumer reading off a bus. The request validates a version it does not write under; whether the guarantee survives needs to be shown, not asserted.

## Phase 3 — Hunt excess and hand-built solved problems

**Do:** For each piece of machinery, name the requirement or constraint that pays for it. Two candidates:

- **The new event bus.** A new Redis Streams bus, a new consumer group, a single worker, and a synchronous block on a return event — introduced so a row can be written that the request handler could write in the same transaction the service already uses everywhere. The stated rationale is "an event log for later consumers"; no requirement in this package names such a consumer, and the append-only change history table already *is* the log. It also inserts a single-worker single point of failure into the synchronous request path and adds latency to a budget already broken. Cheaper alternative to name: write the row in the request transaction.
- **The hand-built scheduler.** Roughly 600 lines of polling loop, leader election over a database advisory lock, and a retry ledger — owned by this team, forever — for timed sends, in a service that already runs a scheduler, already has a standing ruling that one-off timed sends go through it, and already re-times sends by revoke-and-requeue in a shipped feature that does exactly this. The capability is genuinely needed; building it here is the needed-but-not-worth-writing-yourself case. This also conflicts with built architecture, which makes it a question for the architecture owner as well as a review finding — I raise it, I do not amend that surface myself.

I will **not** label as excess: the 24-month retention and requester/timestamp fields (compliance pays for those), the change history table, or the version column.

## Phase 4 — Put the questions to `analyst-1` before ruling

**Do:** Send one consolidated set of questions to the technical-analyst seat that produced the package, which the manifest says is reachable until the review closes. Their answers go on the record verbatim; the verdict stays mine.

Questions I would ask:
1. For the same-day case: is there a hold path the constraint does not mention — a depot phone workflow, a staffed ops process, a second carrier — or does the design assume an API the aggregator has refused?
2. Which of the 300 ms target and in-request confirmation is the movable one? Was the target set before or after the 900 ms measurement was taken?
3. What pays for the required email field in both bodies, given the forbidden-attribute rule — and where is it stored, since the change entity has no column for it?
4. By what channel does a reminder reach a recipient identified only by an opaque token?
5. Which named consumer requires the event bus, and what does it do that a write in the request transaction does not?
6. What does the existing scheduler fail to do for re-timing, given the shipped feature that already re-times by revoke-and-requeue?
7. What happens to a shipment stuck in `change_pending` when the carrier times out?

**Stop / human decision point.** The same-day conflict is not mine to resolve — it is a commercial choice. I would surface it to the user as: *the vendor has declined in writing, until 2028, the capability two P1 stories depend on; the options are renegotiate, fund a manual depot-hold process and re-scope the requirement around it, or drop the same-day clause and its success criterion.* Branches:
- **User re-scopes or funds an ops path** → the conflict becomes a spec revision; I re-review the revised requirement and success criterion, and the verdict can fall back to needs-revision.
- **User asserts a hold path exists that no artifact records** → I require it written into the constraints with its source before I move off the finding; unrecorded capability is not evidence.
- **User wants to proceed as written** → I hold the verdict. This is the one place I do not soften; a conflict no revision can close must not be filed as a louder "needs revision," or it gets quietly patched at build time and discovered by a returned parcel.
- **No ruling available before the review must close (my default)** → I close with the escalation-grade verdict and the decision named as pending business input.

## Phase 5 — Render the verdict and write the report

**Do:** Assign each finding a severity and a closure path, then rule.

- Expected verdict on today's evidence: **infeasible**, on the strength of the same-day conflict alone — with every other finding recorded as a revision item so the package is ready to move the moment the business decision lands. If `analyst-1`'s answer to question 1 produces a recorded hold path, the verdict drops to **needs-revision**, carrying findings 2–7 and both excess items.
- **Write:** `reviews/feasibility-FEAT-011.md` — verdict, one section per finding with both sides quoted by file and line, the collision stated plainly, the closure path, and `analyst-1`'s answers reproduced as given. I would confirm that path with the user first, since the manifest does not name an output location.

**No tests to run.** There is no code in this workspace — six documents and no source tree. The only executable check I would want is the aggregator latency figures, which live outside this workspace.

## Phase 6 — Delegated cheap reads

Two narrow, one-gap-each lookups I would hand to a throwaway `Explore` subagent pinned to the cheap model, so the sweep does not eat my context:

1. *Locate and quote the p95 line in `ops/carrier-latency-2026-Q3.md` and the carrier master agreement §4.3, wherever they live outside this workspace; return the quoted spans with paths.* On return I check the numbers match what the constraints claim; a mismatch or a missing file becomes its own finding, since the whole latency and same-day analysis rests on those two citations.
2. *Locate the existing carrier HTTP client, the shipped reminder scheduling code, and any existing Redis Streams usage; return file paths and the timeout/retry and scheduling call sites.* On return I check whether the re-timing behaviour claimed for the shipped feature is real, which is what makes the hand-built scheduler excess rather than necessity.

I would do the interpretive reading myself, and I would not let a cheap sweep's silence stand as proof that something does not exist — where absence would drive a finding, I confirm it personally. (Under this plan-only run, none of these are dispatched.)

## What I would report at the end

To the user, in this order: the verdict and the single reason for it; the commercial decision that only they can make, with its three options; the list of revision items grouped as contradictions (email in the contract against the data boundary, the unreachable reminder, the two lock times, the dead-end pending state, the split version check) and excess (the event bus, the hand-built scheduler); the latency finding flagged as needing their ruling on which side moves; and an explicit statement that I did not grade the architecture spine, did not assess individual-artifact completeness, and did not edit any artifact — those belong to other hands.