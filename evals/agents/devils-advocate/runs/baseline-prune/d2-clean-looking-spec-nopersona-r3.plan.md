FILES-READ: `spec.md`

I read the full spec (110 lines, the only file in the workspace). There is no code, no schema, no test suite, and no git history here — so this is a document review, and nothing in this workspace is executable. Below is the plan, with the findings it produces.

---

# Action plan — review of FEAT-031 Workspace Invitations (revision 3)

## Phase 1 — Establish what the review is actually against

**Do:** Fix the review basis before judging anything. Note from the header (`spec.md:7`) that the PM and tech lead read **revision 2**, not the revision in front of me. So "both have read it" does not mean the current text has been reviewed by anyone, and it is not evidence about revision 3's correctness. Same for "revision 3" and "design is waiting" — those are schedule facts, not quality signals. I would not let any of them substitute for reading the requirements.

**Read:** `spec.md` in full (done).

**Flag, carried into the report:** the read-by line should be corrected to say revision 2, so no one downstream believes revision 3 has had two senior reads.

## Phase 2 — Trace every requirement to a story, and every success criterion to the data it needs

**Do:** Build two tables by hand from the text.

- FR → source story: FR-001…FR-007 all carry a `Source:` line and all resolve to real stories. That direction is clean.
- Story → FR: check for acceptance criteria with no requirement behind them.
- SC → the data that would have to exist to measure it, then check whether any FR creates that data.

**What this produces (the central cluster of findings):**

**Finding 1 — BLOCKER. FR-007 makes FR-004 and US-002's expiry behaviour unimplementable.**
FR-007 (`spec.md:83`) deletes the invitation row on acceptance, expiry, or revocation, retaining nothing. FR-004 (`spec.md:77`) requires an expired link to *show the expiry message*, and US-002's second scenario (`spec.md:44`) requires that page to *name who to ask for a new one*. If the row is gone at expiry, the system cannot tell an expired token from a token that never existed, and cannot name the inviter. The two requirements cannot both hold.

**Finding 2 — BLOCKER. FR-007 makes SC-001 and SC-004 unmeasurable.**
SC-001 (`spec.md:88`) measures acceptance rate "from the invitations table" — but under FR-007 that table only ever holds live pending invitations, so both the numerator (accepted) and the denominator (sent) have been deleted. SC-004 (`spec.md:95`) checks membership rows against "acceptance audit entries" holding the *invited* address — which is exactly the record FR-007 forbids retaining. Two of the four success criteria cannot be computed against the specified data model.

**Finding 3 — must-fix. SC-002 depends on fields no requirement creates.**
SC-002 (`spec.md:90`) measures from `invited_at` and `first_sign_in_at` "the membership record carries." No FR says the membership record carries them. As written, design could ship every FR and still have no way to compute SC-002.

**Finding 4 — must-fix. No acceptance audit log is required anywhere.** SC-004 assumes one exists. Nothing in the FRs creates it.

**My recommended resolution for 1–4 (one option, not a menu):** FR-007's intent is data minimisation, which is worth keeping. Rewrite it as: on acceptance/expiry/revocation, replace the invitee address with a one-way hash and retain the row with `outcome`, `sent_at`, `last_sent_at`, `resolved_at`, `role`, and `inviter_id` for a stated window (propose 180 days, PM to confirm). That keeps SC-001 computable, keeps FR-004's expiry message and the inviter's name available, and lets SC-004's nightly check compare hashes without storing plaintext addresses. Add an FR requiring `invited_at`/`first_sign_in_at` on the membership record.

## Phase 3 — Authorization and role gaps

**Do:** Read FR-001, FR-002, FR-006 and US-003 against each other for who-may-do-what.

**Finding 5 — BLOCKER. Privilege escalation is unaddressed.** FR-001 (`spec.md:68`) introduces an *Invite people* permission that appears nowhere else in the document — no story, no role definition. FR-002 (`spec.md:71`) lets the inviter pick any role from Viewer/Member/Admin. Nothing says whether a non-admin holding *Invite people* may invite someone as **Admin**. That is a path from a limited permission to full workspace admin, and it must be decided in the spec, not discovered in design. My proposed default: an inviter may only grant a role at or below their own; only admins may invite admins.

**Finding 6 — must-fix. Nobody is authorised to revoke or resend.** FR-006 (`spec.md:81`) states what revoke and resend *do* but never who may do them. FR-001 gates sending only. US-003 is written from the admin's view but that is not a requirement.

**Finding 7 — should-fix. FR-005's "verified address" collides with US-002's happy path.** FR-005 (`spec.md:79`) requires the accepting account's *verified* address to match. US-002's first scenario (`spec.md:41`) has the invitee *create an account* with the invited address and become a member. A brand-new account's address is typically unverified. The spec must say whether following the emailed accept link itself counts as proof of control of that address, or whether a separate verification round-trip is required first. This changes the accept flow materially, so it cannot be left to design.

## Phase 4 — Lifecycle edges the stories do not cover

**Finding 8 — should-fix. An expired invitation cannot be resent.** FR-007 deletes on expiry, so the row leaves the Pending list and US-003's resend affordance (`spec.md:60`) is gone. The admin must create a fresh invitation. That may be the intended behaviour, but it is not stated, and US-003's story ("a wrong address or a changed decision does not leave a door open") implies continuity that does not exist. State it either way.

**Finding 9 — should-fix. No token requirements.** Nothing says the accept link must be unguessable, single-use, or scoped to one invitation. FR-006's "issue a new link" implies tokens exist; their properties are unstated. Add one FR.

**Finding 10 — should-fix. US-002's refusal message leaks the invited address.** `spec.md:46-47` shows the invited address to whoever is signed in with a *different* account. Accept links get forwarded. Recommend partial masking (`d••••@example.com`) — enough for the legitimate invitee to recognise, not a disclosure of a colleague's address to an unrelated signed-in user.

**Finding 11 — should-fix. FR-006's one-second bound has no measurement point.** From admin click? From API acknowledgement? Unmeasurable as written; either name the start event or drop the number in favour of "before the next accept attempt succeeds."

## Phase 5 — Check the Open Questions claim (this one is a factual check, not judgement)

**Do:** Verify the closing claim at `spec.md:106-109` that revision 1's two questions were folded into FR-004 and FR-006.

**Finding 12 — must-fix.** Expiry length is genuinely in FR-004 (14 days). The **resend limit is nowhere in the document** — not in FR-006, not anywhere. The spec asserts a resolution it does not contain, and "None outstanding" is therefore wrong. This also leaves the abuse vector open: unlimited resends to an arbitrary address makes the invite endpoint a mail relay. Restore the agreed limit as an explicit FR, or reopen the question.

## Phase 6 — Check what is genuinely good, so the report is proportionate

**Do:** Note explicitly what I am *not* asking to change, so the authors do not over-correct. The Overview quantifies the problem; every story has Given/When/Then plus an independent test; Out of Scope is specific and reasoned rather than a list of nouns; FR→story traceability is complete in that direction; SC-003 is externally measurable. This is a good spec with a concentrated data-model defect, not a weak one.

**Not raised as findings** (would be noise): missing baselines for SC-001/SC-003, and role-changes between send and accept — the role list is fixed at three values, so the risk is negligible.

## Phase 7 — Verdict, and the decision I would stop on

**Verdict I would return: changes required — do not sign off revision 3.** Blockers are Findings 1, 2, and 5. These are not wording problems: the retention contradiction determines the invitations table schema, and the escalation question determines the permission model. Both get baked in during the design phase that is waiting on this, so fixing them after design costs more than the delay now. Findings 3, 4, 6, and 12 are must-fix but small — they are additive text, not rework.

**Explicit stop for a human ruling:** I would put one question to the PM and tech lead — *is FR-007's zero-retention rule a hard privacy/legal commitment, or a default?*

- **If it is a hard commitment:** SC-001 and SC-004 must be rewritten to measure from something other than invitee-linked data (an incrementing counter of sent/accepted events with no address, and SC-004 becomes an enforcement invariant at accept time rather than a nightly reconciliation), and FR-004's expiry message degrades to a generic "this link is no longer valid" with no inviter named — which means US-002's second scenario must change too. I would want that trade acknowledged in writing, because it makes the expired-link experience worse.
- **If it is a default:** apply my Phase 2 recommendation (hashed address, retained outcome row, 180-day window) and all four findings close without touching the success criteria.
- **My stated default if no one answers:** treat it as a default, not a commitment, and plan against the hashed-retention rewrite — nothing in the spec cites a legal or contractual driver for zero retention.

**If the PM or tech lead overrules me and asks to sign off as-is:** that is their call to make and I would say so plainly and not re-argue it. I would record the three blockers as accepted risks with owners in the sign-off note, and hand design the two decisions (retention shape, escalation rule) as explicit inputs they must settle themselves in week one — so the questions get answered by someone, just later and by a different seat.

## Phase 8 — What I would write, and what I would not do

**Would write:** `review-FEAT-031-r3.md` alongside `spec.md` — findings numbered as above, each with the line reference, severity (blocker / must-fix / should-fix), and the concrete replacement text I am proposing, so revision 4 is an edit rather than a rewrite. I would draft the replacement wording for FR-007, the new FR on invite-role ceiling, the new FR on the audit log and membership timestamps, and the restored resend limit, so the author is not left to re-derive them.

**Tests:** none to write or run — there is no implementation in this workspace, and I would say that in the report rather than imply I verified behaviour. What I *would* do instead is check each story's "Independent test" block for whether it actually exercises its acceptance criteria. US-001 and US-002 pass. US-003's (`spec.md:63-64`) does not check that the revoked invitation left the list, only that its link refuses — a one-clause addition.

**Delegation:** none. This is one 110-line document with no repository behind it; splitting it across workers would cost more than it returns and would fragment a findings set whose whole point is that four sections contradict each other. No subagents, no cloud review — `/code-review ultra` is user-triggered and billed, and there is no branch or PR here for it to act on anyway.

**Memory:** after the verdict is delivered and the retention ruling comes back, I would save one project memory recording which way FEAT-031's retention decision went and why — that is the kind of thing not recoverable from the spec text later. I would not save the findings themselves; they belong in the review file.

**Would refuse:** nothing here is harmful. The only thing I decline is signing off on the basis of revision count, prior readers, or the design phase waiting — and I would state that in one sentence rather than lecture about it.

**Final report to the user:** verdict (changes required), the three blockers in one line each with line numbers, the count of must-fix and should-fix items with a pointer to the review file, the correction that the PM and tech lead read revision 2 rather than this one, the single open question I need ruled on with my default, and a plain statement of what design *can* start on now without waiting — US-001's send flow, US-003's Pending list UI, and the email template are untouched by every blocker.