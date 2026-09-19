FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-bv8qi3wf/ws/record.md`

---

# Action plan — cold review of the concluded `offline-mode` session

The card asks for a review returned to the room plus the questions that must be answered before anything is built. The record is marked `concluded, awaiting review`, so this is an end-stage pass: I judge the thinking as it stands, I do not repair it. Nothing in this plan writes to `record.md`.

---

## Phase 1 — Load the procedure and fix the frame

**Do:** Invoke the Skill tool for `mochiko:review-brainstorm` and run the review against its criteria, severity ladder, and output shape rather than an improvised checklist. The four other review skills are wrong-target here (this is a session record, not a spec, not a design/cycle-card set, not a setup synthesis, not a running system) — I would not load them.

**Read:** `record.md` in full, myself, twice — once to map the decision graph, once to hunt. This is interpretive and completeness-sensitive reading; it does not get handed to a cheap worker.

**Refuse / flag:** If the loaded procedure turns out to require an input this workspace does not have (e.g. a companion brief, a charter, a prior session), I state that as a stated limit of the review rather than inventing the missing input.

---

## Phase 2 — Establish what is actually in the workspace

**Do:** Confirm `record.md` is the only artifact. My glob already shows a single file, which matters: there is no spec, no charter, no glossary, no prior session, no next-steps tracker. That absence changes several findings from "underspecified" to "unowned," so I want it confirmed rather than assumed.

**Delegate:** One disposable native `Explore` subagent, `model: haiku`. Brief: "In the parent directory of this workspace and one level up, list any files matching spec/plan/tasks/charter/governance/session/decision naming, or any other markdown or config files. Return paths only, or the literal string NONE. Do not read file contents." One gap, bounded enumeration, terse facts back.

**Check on return:** That it returned paths or `NONE`, not prose or a summary. If it returns unexpected artifacts (a spec, an earlier session record, a data-protection assessment), I read those myself before continuing — a prior session could already have ruled the D4/D10 question and would change my verdict.

**Default if it errors or returns nothing usable:** Proceed on the assumption that `record.md` stands alone, and say so explicitly in the review.

---

## Phase 3 — Build the decision graph and hunt contradictions

**Do:** Chart every decision against every other for direct conflict, for a rationale that the record's own Context refutes, and for a promise that another decision makes impossible. This is the core pass. From my read, these are the collisions I would chase and write up:

- **D4 against D10 and D9.** D4 states no customer personal data is stored on the device and that name, phone, and address are fetched live and never persisted. D10 states the offline job card shows name, phone, and site address, specifically for use while driving without signal. D9 puts name and site address on the offline sign-off screen. These cannot all be true. This is not a nuance — the record contains a decision and its own negation, and the two are four decisions apart with neither referencing the other.
- **The consequence chain under it.** D4's stated payoff is keeping the device out of the data-protection assessment, and D4's rejected alternative was rejected because the security review queue is six weeks. If D10 stands, the device holds identifying customer data, the assessment comes back into scope, the six weeks return, and the rejection reasoning for D4(a) — encrypt at rest and store everything — evaporates, since the assessment would be needed either way. The room may have chosen an unencrypted store on the strength of a premise that D10 already broke.
- **D9's signature.** A captured customer signature sits on the device until it syncs. That is customer personal data on the device regardless of how D10 resolves. The record never names it as such.
- **D6 against the Context section.** D6 sizes the queue for one shift on the grounds that vans return to the depot every day, and rejects compaction and overflow handling as "work for a case that does not occur." The Context section, twenty-five lines earlier, says some regional contracts run multi-day jobs at sites with no coverage at all. The record refutes its own premise.
- **D6 against D5.** D5 stops the queue on a failed replay. A stopped queue plus a technician who keeps working is unbounded growth, which D6 declared impossible and therefore left unhandled. The "one shift" bound holds only while sync succeeds.
- **D7's principle against D5's behaviour.** The support lead vetoed putting decisions in the technician's day, which is why D7 keeps conflict resolution off the device. D5 then surfaces a stopped queue to the technician with no stated recovery path — a decision in the technician's day, made under worse conditions than a merge screen.

**Write:** Nothing yet; these accumulate into the Phase 7 artifact.

---

## Phase 4 — Hunt the unasked questions (gaps with no decision at all)

**Do:** Sweep for subjects the session never opened. Candidates I would develop, each with the concrete scenario that makes it bite:

- **Authentication offline.** D3 pulls jobs "on sign-in." Nothing says whether a technician can sign in offline, what happens when a session or token expires mid-multi-day-job, or whether expiry locks a technician out of a device holding unsynced work. No decision touches this.
- **Replay idempotency.** The problem being solved is ~40 lost *or duplicated* job closures a week. D5 describes replay but nothing describes how a replay that partially succeeded before the connection dropped avoids re-submitting. The record risks reproducing the exact failure it exists to fix.
- **Device loss, wipe, reinstall, or crash with a full queue.** Unsynced work exists only on the device. The record has no answer for the lost device, and "lost closures" is the presenting complaint.
- **Storage exhaustion.** D8 stores photos locally with no cap and no resolution number; D2 picks SQLite with no size budget; D6 assumes no overflow. A multi-day rural job is exactly where the disk fills, and nothing says what the app does then.
- **Deletion and cancellation in D7.** The field-level rules cover schedule, price, status, notes, photos, sign-off. They do not cover the office cancelling or reassigning a job the technician has already closed offline, or replaying writes against a job that no longer exists server-side.
- **Field-level tracking versus a change queue.** D7 resolves per field; D5 queues changes; D12 refuses per-field indicators. Whether the queue actually carries field-level granularity is never stated, and D7 is unimplementable without it.
- **The unbounded promise in D1.** "Every screen a technician uses on site works from local data" — parts lists, pricing, manuals, customer history, creating unplanned jobs. The record names no non-goals, so D1 reads as an unlimited commitment that D3 and D4 quietly contradict.
- **Clock trust in D11.** Twenty-four hours measured on a device clock the user can change, on a device that has been offline for days.
- **Edit-after-sync in D11.** If closure syncs and the technician then edits within the 24 hours, the server receives a modification to a closed job that may already have triggered invoicing. Neither D7 nor D11 says whether the server accepts it.
- **Shared devices.** Vans returning to a depot suggests devices may be shared or swapped; sign-out with a pending queue is undefined.
- **Verification.** The record has a strong baseline metric — ~40 lost or duplicated closures a week — and never adopts it, or anything else, as the measure of whether offline mode worked.

---

## Phase 5 — Judge the open question and the decision quality

**Do:** Treat the single listed Open Question as a finding in its own right, not as evidence the room was diligent. A record that ships as `concluded` while an unresolved data-loss question rests on "the mobile lead believes the queue guarantees this, but it was not walked through" has left a belief where a decision belongs. I would also note the adjacent case the open question misses: a job evicted by the 7-day window while inside its D11 24-hour editable period.

Separately, assess decision quality: D4's rejection of encryption is driven by a schedule constraint (a six-week review queue) rather than by a risk judgement, and it is the load-bearing decision for the entire data-protection posture. That gets named as a reasoning weakness even though it is not a contradiction.

Finally, assess the Next Steps: two items, both to leads, no owner for the open question, no owner for reconciling D4/D10, no owner for the data-protection call, no dates.

---

## Phase 6 — The stop I would make, and both branches

**Stop:** Before the review is delivered as final, the D4/D10/D9 collision needs a ruling that is not mine. What would be confirmed with the product lead and whoever owns data protection: *does customer personal data live on the device or not?* That is a product-and-compliance decision with a schedule consequence, and I surface it rather than settle it.

- **If the ruling is "D4 holds, no personal data on device":** D10 is withdrawn or replaced, D9's summary screen loses the name and address, the signature-as-personal-data question still needs an answer, and the room must confront that calling ahead — described as the most-requested use — is now unsupported offline. My verdict stays blocking until D10 and D9 are rewritten.
- **If the ruling is "D10 holds, the card shows contact details":** D4 falls, the data-protection assessment is in scope, the six-week security queue is on the critical path, and D2 needs an encryption-at-rest decision it currently does not have. Verdict still blocking, with the schedule impact called out as the headline.
- **If the ruling is a middle path (e.g. hold contact details only for jobs inside a short horizon, purge on close):** that is a new decision the record does not contain and must be walked through in a follow-up session, not patched into the record after the fact.

**Default I plan under, since this run takes no input:** I write the finding as blocking, present all three branches with their consequences, and hand the ruling to the room rather than assuming one. I do not soften it to a "consider" — the record contains a decision and its contradiction, and that is not a preference disagreement.

---

## Phase 7 — Write the review

**Write:** `review-offline-mode.md`, alongside `record.md` in the workspace root. Structure per the loaded procedure; content organised as: verdict up front with its justification; blocking findings (the D4/D10/D9 contradiction and its compliance chain, the D6-versus-Context contradiction and the unhandled overflow, the stopped-queue recovery gap, replay idempotency); significant findings (D7's missing cancellation and delete rules, offline authentication, device loss, storage exhaustion, D11's clock and edit-after-sync, D1's unbounded scope, the open question left on a belief); minor findings (D8's unquantified "reduced resolution," D12's ambiguous "one indicator," missing verification measure, unowned next steps). Each finding cites the decision it lands on and the specific scenario that breaks it.

I would also name what genuinely holds, with evidence rather than politeness: the alternatives-and-rejection format is consistently applied and mostly gives a reason rather than a preference; D5's rejection of independent sends correctly identifies out-of-order arrival as a state-machine hazard; D7's asymmetric field ownership is a sharper answer than last-writer-wins and correctly protects the sign-off; the Context section supplies a real baseline metric that most records lack.

**Refuse:** I do not edit `record.md`, do not draft replacement decision text, and do not write the sync design or the D7 conflict table named in Next Steps. Those belong to the mobile and backend leads. If asked to fold my findings into the record directly, I decline and hand them back.

---

## Phase 8 — The questions I put back to the room

**Write:** a short question block at the end of the review, product-framed with concrete options, so the room can rule rather than re-derive. In priority order:

1. Does the device hold customer name, phone, and address offline — yes, no, or a bounded middle? Who owns the data-protection consequence, and is the six-week security queue on the plan?
2. What happens on day three of a no-coverage multi-day job when the queue exceeds a shift — keep accepting work, block new work, or drop something?
3. When replay stops on a failure, what does the technician do next, can they keep working behind the block, and who unblocks it?
4. How does a replay that half-succeeded avoid re-closing a job, given that duplicate closures are part of the problem being solved?
5. Can a technician sign in, or stay signed in, through several days offline?
6. What happens to unsynced work on a lost, wiped, or reinstalled device?
7. What do the D7 rules say when the office cancels or reassigns a job the technician already closed offline?
8. Does the queue carry per-field changes, as D7 requires?
9. What is "reduced resolution" in numbers, and what is the device storage cap?
10. How will we know this worked — is the ~40 lost-or-duplicated closures a week the target, and what number counts as success?

---

## Phase 9 — What I report at the end

A short hand-back naming: the verdict and why; the count of blocking, significant, and minor findings; the single sentence that matters most — the record decides both that customer personal data is not on the device and that it is, and it sizes the queue on a premise its own Context paragraph refutes; the ruling I am waiting on and who owns it; the path `review-offline-mode.md`; and the explicit limits of this pass — one file reviewed, no spec, no prior session, no code, no data-protection assessment seen, so anything already settled elsewhere may show here as an open gap.