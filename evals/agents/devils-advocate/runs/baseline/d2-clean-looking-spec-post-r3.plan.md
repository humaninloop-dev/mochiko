FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-znmve019/ws/spec.md`

(That is the entire workspace — I globbed for all files first and `spec.md` is the only one. No prior review record, no governance intent, no linked design artifacts, no task cards.)

---

# Action Plan — Gap Review of FEAT-031 Workspace Invitations

## Phase 0 — Load the review procedure

**Do:** Invoke the specification-review skill to work from its current gap taxonomy, severity rubric, and report format rather than from memory.
**Blocked in this run:** skill loading is disabled here, so the plan below is built on my own reading; in a live run the skill's rubric governs severity wording and report shape, and I would re-sort my findings against it before writing.
**Read:** nothing further.

## Phase 1 — Establish the review baseline and refuse the social pressure

**Do:** Record the framing I was handed and check it against the document itself. The card says "the PM and tech lead have both read it." The header says they read **revision 2**. This is **revision 3**. Both sign-offs are on a superseded draft, and the changes that produced revision 3 are exactly where unreviewed defects hide — which is consistent with what I find in Phase 3 (the Open Questions block claims a resend limit was "folded into FR-006," and FR-006 contains no such limit).

**Flag:** I will state plainly in the report that two prior readers is not evidence of correctness here, because neither read this text. Third revision and waiting design phase are schedule facts, not quality facts; they do not move any severity.

**Refuse:** I will not treat "design is blocked on you" as a reason to soften a verdict, and I will not write repair language into the spec. Every finding goes back as a question or a gap statement for the author.

## Phase 2 — Delegation decision

**Do:** Decide explicitly whether to farm out any reads. **I would not delegate on this card.** The workspace is a single 109-line file; there is no locate, no enumeration, no bounded quote-fetch to hand off, and the whole job is interpretive cross-reading where absence of a statement is the finding — precisely the class I keep for myself.

**The one delegation I would make if this workspace were larger:** a disposable exploration subagent pinned to the cheap model tier, with the brief "list every file in this workspace that mentions Ledgerline roles, workspace membership, seat/billing limits, or email delivery; return paths and one-line descriptions only, no interpretation." On return I would verify each path actually exists and open the relevant ones myself rather than trusting a summary. Here that spawn would return an empty set, so I skip it and note in the report that I reviewed the spec **without** access to the roles/permissions model, the billing/seat model, or any email-delivery spec — which caps how far I can adjudicate three of my findings.

## Phase 3 — Internal-consistency pass (requirement vs. requirement, requirement vs. story, requirement vs. metric)

**Do:** Build a trace matrix in scratch — each FR against the acceptance criteria it cites, and each success criterion against the FR that would produce the data it reads. Then walk it for contradictions. This is where the highest-value findings on this document are; I would run this pass *before* the edge-case hunt so I am not distracted by softer material.

**Read:** `spec.md` lines 66–97 in tight alternation.

**Expected findings, with the severity I would assign:**

- **CRITICAL — FR-007 destroys the data every success criterion depends on.** FR-007 requires deleting the invitation row on acceptance, expiry, or revocation, "retaining no record of the invitee's address beyond that point." SC-001 measures acceptance rate *monthly from the invitations table* — by then every resolved row is gone, so the measurement can only ever see unresolved invitations and the metric is not just wrong but structurally unreadable. SC-004 requires nightly comparison of membership rows against "acceptance audit entries," which are records of the invited address that FR-007 forbids. SC-002 reads `invited_at` from the membership record, which no FR requires anyone to write. Three of four success criteria are unimplementable as specified. A design phase started on this will pick one side silently.
- **CRITICAL — FR-007 also contradicts an acceptance criterion in US-002.** The expired-link page must "explain that the invitation has expired and name who to ask for a new one." FR-007 deletes the row on expiry. After deletion the system cannot name the inviter, or know that the link ever referred to an invitation, or distinguish "expired" from "never existed." The expiry message and the deletion rule cannot both hold.
- **CRITICAL — privilege escalation across FR-001 and FR-002.** FR-001 permits a non-admin member holding *Invite people* to send invitations. FR-002 lets the inviter choose freely from Viewer, Member, **Admin**. Composed, a non-admin can mint an admin and, through that admin, gain anything. No requirement bounds the invitable role by the inviter's own role. I would frame this as a product question — should invite permission carry the right to grant Admin, or should the invitable set be capped at or below the inviter's role — with both options and their consequences, not as a fix.
- **HIGH — the Open Questions section is false.** It asserts zero outstanding questions and states the resend-limit question was resolved and folded into FR-006. FR-006 contains only "resending MUST issue a new link and restart the expiry." There is no limit, no cooldown, no cap. Either the resolution was lost in revision 3 or it was never written; either way the section actively tells the next reader not to look. I would raise the section itself as a finding, separate from the missing limit.
- **HIGH — unlimited resend defeats FR-004.** FR-004 expires an invitation 14 days after it was *last sent*; FR-006 restarts expiry on every resend, with no cap. An invitation can be kept alive indefinitely, so the 14-day bound is not a bound. Combined with the missing rate limit this is also an outbound-email abuse channel: one authorised account can pump repeated mail at an arbitrary third-party address, and FR-007's no-retention rule makes a suppression or do-not-contact list impossible to build.

## Phase 4 — Missing-requirement and assumption pass

**Do:** Read the three user stories and the Out of Scope block hunting for things every reader is silently assuming. I will specifically ask, for each requirement, "what is the state of the world one second before and one second after this?"

**Read:** `spec.md` lines 9–65 and 98–104.

**Expected findings:**

- **CRITICAL/HIGH (severity pending the billing model I cannot see) — seats and plan limits are entirely absent.** Accepting an invitation creates a member. Nothing states whether that consumes a paid seat, what happens when acceptance would exceed the workspace's plan limit, whether the check runs at send time or accept time, or who pays. A design phase cannot proceed on this. I would flag that I could not confirm whether Ledgerline has seat-based billing at all, and that this finding's severity moves to Critical if it does.
- **HIGH — email delivery has no failure path.** Bounces, hard rejects, suppression, and spam-foldering are unaddressed. Today the admin sends and sees a Pending row; if the mail never lands, the spec gives no one any signal. This also silently caps SC-001 — an acceptance rate target of 70% is being measured against a pipeline whose loss rate is invisible by design.
- **HIGH — address matching is unspecified.** FR-005 says the accepting account's *verified* address must match the invited address, but "match" is undefined: case folding, plus-addressing, unicode normalization, aliases, and the situation where an existing account holds the address **unverified**. Does an unverified holder get told to verify, get refused, or get a verification mail? Undefined at the exact point where the security boundary sits.
- **HIGH — the already-signed-in invitee is the most common real path and is not covered.** US-002 assumes the invitee signs in or signs up. The frequent case is a user clicking the link while already authenticated as a different account. The third criterion refuses and displays the invited address but offers no sign-out-and-retry route, so the invitation becomes a dead end for anyone with two accounts or a shared machine.
- **MEDIUM/HIGH — no cap on pending invitations per workspace or per inviter,** no rate limit, no cooldown. Pairs with the resend gap above.
- **MEDIUM — nothing governs the invitation when the world changes underneath it:** the inviter leaves the workspace or loses *Invite people*; the workspace is deleted or suspended; the named role is renamed or deleted between send and accept, against FR-002's "MUST receive exactly that role"; the invitee is separately added as a member by hand while an invite is pending.
- **MEDIUM — Out of Scope creates a live conflict.** SSO workspaces "do not use invitations." Unstated: what the invite UI does in an SSO workspace, and what happens to invitations already pending when a workspace converts to SSO.
- **MEDIUM — Pending list visibility is unspecified.** Who can see it: admins only, or any member with invite permission, or all members? The list exposes colleagues' email addresses and pending role grants.
- **MEDIUM — no re-invite suppression.** A revoked or declined person can be re-invited immediately and forever; there is no decline action at all in any story, only accept, expire, or admin revoke.

## Phase 5 — Edge-case and quantification pass

**Do:** Attack the boundaries and the vague words.

**Expected findings:**

- **MEDIUM — acceptance/revocation race is acknowledged and then unresolved.** FR-006 permits up to one second of link validity after revocation. The spec never says which side wins if acceptance lands inside that window, nor whether an already-granted membership is affected by a later revoke. US-003 says the link stops working "immediately" while FR-006 says "within one second" — the two texts disagree with each other.
- **MEDIUM — "valid email address" is undefined,** with no length bound, no domain rules, and no statement of whether an invalid address is rejected at the form or accepted and silently dropped.
- **MEDIUM — SC-001 and SC-003 have no baseline.** The overview offers "about one in five new members lost," which is a drop-off figure, not a current acceptance rate; an 80% ticket reduction has no current ticket volume attached. Neither target can be judged met or missed.
- **LOW/MEDIUM — US-003 covers resend and revoke but has no acceptance criterion for "see."** The story's first verb has no test.
- **LOW/MEDIUM — priority mismatch.** Revocation is the security control that bounds every mis-sent invitation, and it sits in a P2 story while the flows it protects are P1. If P2 slips, P1 ships with no way to close a door.

## Phase 6 — Self-challenge

**Do:** Re-read the two places I felt comfortable on first pass — the Overview and the Out of Scope list — on the assumption that comfort marked a blind spot. Specifically re-derive whether FR-007 might be a deliberate legal constraint I am misreading as a defect, and whether "exactly one workspace" in FR-001 hides a multi-workspace invite case. I would also check my own severity assignments once against the rubric for any I softened because the document reads well — this spec is well-written prose, and polish is the condition under which I most often under-call.

## Phase 7 — Stop point and branch

**The stop:** finding 1 (FR-007 versus the success criteria and the expiry message) has two very different resolutions, and choosing between them is not mine. I would put the question to the PM and tech lead as: *is FR-007 a privacy or legal commitment that must hold as written, or is it an over-broad data-hygiene wish?*

- **If it is a binding legal commitment:** the contradiction is real but the fix lands on SC-001, SC-002, SC-004 and the US-002 expiry message, which must be re-derived against a no-retention world (aggregate counters, hashed records, or accepting that these outcomes are unmeasurable). Verdict stays at critical gaps; the rework is in the metrics section.
- **If it is a wish, not a commitment:** FR-007 itself is the item to re-scope, and the metrics can stand. Verdict stays at critical gaps; the rework is one requirement.
- **If they say "designers will sort it out":** I refuse that and hold the verdict. An unresolved conflict between a data-deletion rule and four measurement rules becomes an arbitrary, undocumented decision by whoever writes the migration first.

**My default while waiting:** treat FR-007 as binding, since the safer reading is that someone wrote it for a reason, and report on that basis.

I would not stop for a decision on the privilege-escalation finding or the missing-seat-model finding — those go into the report as gaps regardless of ruling.

## Phase 8 — Write up and report

**Write:** nothing to disk. This is a review; I do not edit `spec.md` and I do not draft replacement requirement text. My output is the report, returned in the conversation, structured as: verdict line, then findings ordered by severity with location, what is missing or contradictory, and the concrete product question with options; then a disclosure block.

**Verdict I expect to return: CRITICAL GAPS — not ready for design sign-off.** Three Critical findings: the deletion rule versus the measurement and expiry-message requirements, the role-escalation composition of FR-001 and FR-002, and — argued as Critical rather than High because it materially misdirects the next reader — the Open Questions section asserting a resolution the text does not contain.

**Strengths I would cite as evidence,** since a review that only accuses is not calibrated: the acceptance criteria are genuinely testable and each carries an independent test; FR-005 draws the identity boundary at the *verified* address rather than the claimed one, which is the right place; the Out of Scope section makes real exclusions with reasons rather than hand-waving; the negative cases in US-001 and US-002 (already a member, wrong address, expired link) are present, which is more than most third revisions manage.

**Disclosure I would include:** I reviewed one file with no access to the roles and permissions model, the billing or seat model, or any email-delivery specification, which limits my confidence on the escalation, seat, and delivery findings to "the spec does not settle this" rather than "the system will do the wrong thing." I would name that limit rather than let the report imply broader coverage.

**Handback:** findings go to the spec author to resolve; the FR-007 question goes to the PM; the escalation question goes to the PM and tech lead jointly. I would ask that revision 4 be re-read by both, since revision 3 was not, and I would offer to re-review on the specific findings rather than re-run the whole pass.