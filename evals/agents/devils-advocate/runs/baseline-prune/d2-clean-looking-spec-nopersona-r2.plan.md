FILES-READ: `spec.md`

*(The workspace contains exactly one file; the glob confirmed there is no supporting doc, no revision history, no linked design or data model.)*

---

# Action plan — review of `spec.md` (FEAT-031, Workspace Invitations, rev 3)

## Phase 0 — Frame the job and the pressure on it

The card carries three facts that push toward approval: third revision, PM and tech lead have read it, design is blocked on me. I would note one factual correction to that framing before starting, because it changes how much prior review this spec has actually absorbed: the header says **"Read by: product manager (revision 2) · tech lead (revision 2)"**. Nobody is recorded as having read revision 3. So "both have read it" is true of an earlier draft, not the one in front of me. I would not treat that as a reason to be harder on the spec, only as a reason not to assume rev-3 changes were vetted.

The deliverable is a review with findings and a verdict, not an edit of the spec. I would not modify `spec.md` — the author holds the pen; I supply findings and suggested wording they can paste.

**No delegation.** This is a single 109-line document requiring whole-document consistency reasoning. Splitting it across workers would destroy exactly the cross-section contradictions that are the point of the review. I would do all four passes myself.

## Phase 1 — Build a traceability matrix (read: `spec.md` lines 17–96)

Construct, in scratch working notes only, three mappings:
- every acceptance criterion in US-001/002/003 → the FR that implements it;
- every FR → the story its `*Source:*` line claims, verified against that story's actual text;
- every SC → the data the FRs guarantee will exist to measure it.

What I expect this to surface: FRs whose stated source doesn't contain the behaviour, and SCs that measure data no requirement creates. Both hit.

## Phase 2 — Consistency pass; the findings I expect to file

These are the findings I have already identified from the read, which I would confirm and write up.

### Blocking

**B1 — FR-007 destroys the data FR-004 and US-002 depend on.**
FR-007 (line 83) requires deleting the invitation row *on expiry*. US-002's second criterion (lines 44–45) and FR-004 (line 77) require an expired link to render "this invitation has expired" and name who to ask. Once the row is gone, an expired link is indistinguishable from a forged or never-existent one; the only honest response is a generic "invalid link," which fails the acceptance criterion as written. These cannot both ship.
*Proposed fix to offer:* replace deletion-on-expiry with a state transition (`expired`) plus a separate purge of the address after a stated retention window (e.g. 30 days), during which the expiry message is servable.

**B2 — FR-007 destroys the data SC-001 and SC-004 measure.**
SC-001 (lines 88–89) measures acceptance rate "from the invitations table," but FR-007 deletes rows on acceptance — the numerator is erased at the moment it is earned. SC-004 (lines 95–96) measures against "acceptance audit entries" recording which address accepted; an audit entry holding the invitee's address is precisely the record FR-007 forbids retaining. Three requirements are mutually unsatisfiable.
*Proposed fix:* either (a) FR-007 becomes a purge of the *address* while retaining a de-identified row (id, workspace, role, sent/accepted timestamps, terminal state) and SC-004 moves to a hashed-address comparison, or (b) SC-001/SC-004 are rewritten against a separate metrics store. Option (a) is what I'd recommend.

**B3 — Privilege escalation is unblocked, and the escalation path was introduced without a story.**
FR-001 (lines 68–69) lets a non-admin member holding *Invite people* send invitations. FR-002 (lines 71–72) lets the inviter choose any role including Admin. Nothing constrains the invited role to be at or below the inviter's own. A Member with one permission can mint an Admin. Compounding this, no user story covers the non-admin inviter at all — US-001 and US-003 are both "As a workspace admin" — so FR-001's `*Source: US-001*` is a false citation and this capability has never been described from a user's point of view.
*Proposed fix:* add an FR capping the invited role at the inviter's own, or restricting Admin-role invitations to admins; and either add a story for the delegated inviter or drop the clause from FR-001.

### Non-blocking but must be answered before design closes

**N1 — The "no open questions" claim is inaccurate.** Lines 106–109 state the resend-limit question was "folded into FR-006." FR-006 (lines 81–82) contains no limit — only "resending MUST issue a new link and restart the expiry." With unlimited resends and no cooldown, the invite form is an email-flooding tool pointed at any address, and each resend indefinitely extends a link's life. The question is open, whatever the section header says.

**N2 — FR-005 introduces email verification with no supporting behaviour.** "Verified address" (line 80) implies a verification step that US-002 never mentions; US-002 says only "sign in or create an account with the invited address." Undefined: what a brand-new invitee sees between account creation and verification, and whether the invitation survives that gap against a 14-day clock.

**N3 — SC-002 measures fields no requirement creates.** `invited_at` and `first_sign_in_at` on the membership record (lines 90–92) appear nowhere in the FRs, and `invited_at` on a membership is in tension with FR-007's data-minimisation intent. Either add an FR or drop the criterion.

**N4 — US-003's actions are unmapped for delegated inviters.** If a non-admin can invite (FR-001), can they see, resend, or revoke? FR-006 doesn't say who may revoke. Design will guess otherwise.

**N5 — Revoke/accept race is unspecified.** FR-006 requires invalidation "within one second," which concedes a window; nothing states who wins if acceptance lands inside it. Name the rule (revocation wins; acceptance in-flight is rejected).

**N6 — Out-of-scope item 3 has no requirement behind it.** SSO workspaces "do not use invitations" (lines 103–104), but no FR suppresses the invite path there. Out-of-scope prose is not enforcement.

**N7 — Address disclosure on refusal.** US-002's third criterion (lines 46–47) shows any link-holder which address the invitation went to. That is a deliberate usability-versus-disclosure trade; I'd flag it for an explicit ruling rather than assert it's wrong, and note it sits oddly beside FR-007's posture.

**N8 — SC-001's denominator is undefined.** 70% acceptance in 7 days against a 14-day expiry: are revoked and expired invitations in the denominator? Also no stated baseline for SC-003's 80% reduction, so the target isn't falsifiable at release.

### Explicitly *not* findings
I would state that the following are sound so the author knows what not to touch: the three-story split with independent tests, FR-003's duplicate/member handling, the 14-day expiry itself, and the out-of-scope boundaries for CSV and project-level invites. A review that only lists problems invites over-correction.

## Phase 3 — Decision point I would surface rather than resolve

**What I would put to the PM and tech lead:** is FR-007 a legal or compliance mandate (a data-deletion commitment the company has made), or is it the author's own data-hygiene preference?

- **If mandated:** FR-007 stays as the fixed point; SC-001 and SC-004 must be rewritten to measure de-identified or hashed data, and US-002's expiry message must be softened to a generic invalid-link message with the acceptance criterion changed to match. B1 and B2 resolve by amending the success criteria and the story, not the requirement.
- **If preference:** FR-007 relaxes to address-purge-after-retention-window, US-002 and the SCs stand unchanged. Cheapest path, one requirement edited.

**Default I would proceed under:** preference, not mandate — nothing in the spec cites a policy, regulation, or DPA, and the author's own SCs assume retained data, which a real mandate would have precluded. I would write the findings recommending the relax-FR-007 path while flagging both branches, so no rework is wasted if the ruling goes the other way.

## Phase 4 — Verdict

**Changes required — not approved for design sign-off.** Three blocking items (B1, B2, B3), all fixable inside the spec without new discovery. I would state the verdict plainly at the top of the review, not bury it after the findings.

I want to be clear about why I'm not signing off despite the pressure in the card: B1 and B2 are not stylistic. They are three requirements that cannot simultaneously hold, and design cannot pick a data model without knowing which one survives. B3 is an authorisation hole that gets much more expensive to close after the permission model is built. Approving now would move the cost downstream, not remove it. Revision 4 is a small edit — roughly one requirement rewritten, one added, two success criteria adjusted — and I would say so, since "changes required" reads heavier than the work actually is.

I would also record explicitly that this is my read of revision 3 and that the PM and tech lead's recorded review covers revision 2, so their sign-off cannot be assumed to cover the current text.

## Phase 5 — Deliverable

A review returned in the conversation (no files written; the card asks for findings and a verdict, and I don't hold the pen on `spec.md`), structured as:
1. Verdict line + the one-sentence reason.
2. Three blocking findings, each with the conflicting line numbers and paste-ready replacement wording.
3. Eight non-blocking findings, ranked, each naming the line it attaches to.
4. The FR-007 question for the PM with both branches and my recommendation.
5. The short "these parts are sound" list.
6. An estimate of what revision 4 costs.

If the author or PM overrules me and directs sign-off anyway, that is their call to make: I would say once, in a sentence, that B1/B2 will surface as a data-model contradiction in design and B3 as a permissions rework, record that in the review, and not re-argue it.

## Phase 6 — Memory

I would write nothing to memory. The findings belong in the review and in the spec's next revision; the sign-off gate is specific to this task; and the FR-007 ruling is not mine to record as durable fact until the PM has actually made it. If the PM's answer comes back and establishes a standing rule — e.g. that invitation data is under a deletion mandate that will constrain future features — that would be worth one `project` memory at that point, not now.