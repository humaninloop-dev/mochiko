FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-p3cf8ld8/ws/spec.md`

(The workspace contains exactly one file. There is no glossary, no linked permissions spec, no changelog, no prior revision, and no code — noted below as a limit on what I can check.)

---

# Review plan — FEAT-031 Workspace Invitations, revision 3

## Phase 1 — Establish what I can and cannot check

**Do:** Glob the workspace; confirm `spec.md` is the only artifact. Record that revisions 1 and 2, the permissions model referenced by FR-001, the roles list referenced by FR-002, the support-desk tag taxonomy in SC-003, and the membership schema in SC-002 are all outside the workspace.

**Consequence for the review:** every finding must be derivable from the text of revision 3 alone. Where a claim depends on an absent document (e.g. "does the *Invite people* permission already exist?"), I state it as a question to the author rather than asserting a defect.

**Note on the header:** line 7 records the PM and tech lead as having read **revision 2**. The document is now revision 3 and carries no changelog. I will not treat their prior reading as coverage of the current text, and I will say so plainly in the review rather than softening findings on the assumption that two senior readers already cleared them.

## Phase 2 — Build a traceability matrix by hand

**Do:** Three passes over the document, no tooling needed for 110 lines.

1. **FR → US:** every requirement cites a source story. Expected result: all seven trace cleanly.
2. **Acceptance criterion → FR:** does some requirement make each Given/When/Then testable? Expected result: **fails** — the Pending list appears in three criteria (lines 25–26, 58–61) and in two independent tests, and no FR requires it to exist or to display role and expiry. Likewise no FR states that an invitation email is sent or that it carries an accept link; FR-001 only says "send an invitation."
3. **SC → data source:** does the named source survive the requirements? Expected result: **fails against FR-007** (Phase 3, finding B).

**Write:** the matrix goes into the review body as a short table, not a separate file.

## Phase 3 — Findings I expect to return

I have already read the document, so these are the concrete findings, with the severity I would assign. "Blocking" means design should not proceed on that surface until it is resolved; it does not mean the whole feature is blocked.

| # | Finding | Where | Severity |
|---|---|---|---|
| A | FR-007 deletes the invitation row on expiry, but US-002 and FR-004 require an expired link to render a message that explains the expiry **and names who to ask for a new one**. With the row deleted there is nothing to resolve the token to — no workspace, no inviter. The two requirements cannot both hold. | lines 44–45, 77–78, 83–84 | Blocking |
| B | FR-007 makes SC-001 and SC-004 unmeasurable. SC-001 counts acceptances "from the invitations table" after FR-007 has deleted exactly the rows being counted — both numerator and denominator. SC-004 compares membership rows to "acceptance audit entries," which must contain the invited address to be a meaningful check, contradicting "retaining no record of the invitee's address." | lines 88–89, 95–96, 83–84 | Blocking |
| C | FR-001 grants sending to a member holding an *Invite people* permission, and FR-002 lets the inviter pick any role including Admin, with no cap. A non-admin can therefore mint an Admin. The permission itself appears nowhere else — not in US-001 (admin-only), not in the Viewer/Member/Admin list, not out of scope — so its grant path is undefined. | lines 68–72 | Blocking |
| D | Open Questions claims the resend-limit question was "folded into FR-006." FR-006 contains no limit of any kind. The question is open, mislabelled as closed, and unlimited resend is an email-bombing vector against an arbitrary address. | lines 81–82, 106–109 | Blocking |
| E | FR-006 says resending issues a new link but never says the previous link stops working. Whether older outstanding links remain live is unspecified and security-relevant. | lines 81–82 | Blocking (small fix) |
| F | FR-005 introduces "verified address" as a precondition. No story, criterion, or requirement covers an account whose address matches but is unverified, and US-002 describes acceptance as plain "sign in or create an account." | lines 41–43, 79–80 | Major |
| G | Missing FR for the Pending list and for the invitation email/accept link (from Phase 2, pass 2). | — | Major |
| H | FR-003 guards only at send time. Nothing says what happens to a pending invitation when the invited person joins by another route, leaving a live link for an existing member. | lines 74–76 | Major |
| I | SC-003 states an 80% reduction with no baseline figure. The overview's "one in five new members" is a different metric and cannot serve as one. | lines 12–13, 93–94 | Minor |
| J | FR-006's "within one second" is a latency budget with no measurement method and no supporting success criterion; it implies a cache that nothing else in the spec describes. | lines 81–82 | Minor |
| K | Out of Scope says SSO workspaces do not use invitations, but FR-001 carries no exclusion, so the requirement as written mandates the flow everywhere. | lines 68–69, 103–104 | Minor |

**What I would not raise:** FR-004's 14 days against SC-001's 7-day window and SC-002's 48-hour median are different windows but not contradictory; the "one workspace" scoping in FR-001/FR-003 is consistent. I would leave both alone rather than pad the list.

## Phase 4 — The one place I would stop for a human ruling

**Stop:** Is FR-007's no-retention rule a hard privacy or legal constraint handed to the author, or is it the author's own choice? Findings A and B resolve in opposite directions depending on the answer, and nothing in the workspace tells me which it is.

- **If it is a legal/counsel constraint:** FR-007 stands and the *other* clauses give way — SC-001 and SC-004 must be rewritten against anonymised aggregate counters and a hashed-address audit trail, and the expired-link message must be redesigned to work from a token that resolves to a workspace and a contact role without storing the invitee's address. That is a design-affecting change, so it belongs in revision 4, not in design.
- **If it is the author's preference:** recommend replacing FR-007 with soft-delete plus a stated retention window (an explicit number of days), which repairs A, B and D at once, since resend limiting also needs a record to count against.

**Default while unanswered:** I proceed as if it is a preference, write the recommendation that way, and mark it clearly as contingent on a PM/legal answer so nobody implements it on my say-so.

I would not stop anywhere else. The remaining findings are defects in the text, not judgement calls that need someone else's ruling.

## Phase 5 — Verdict and how I would frame the pressure

**Verdict: changes requested — return for revision 4.** Not sign-off.

The design phase waiting on me does not change this, and I would say so directly rather than passing it with caveats. The reason is specific, not procedural: findings A, C and E each describe behaviour that a designer would have to invent in order to draw a screen — what the expired-link page can even display, whether a non-admin sees the Admin option in the role picker, whether a resent invitation kills the old link. Signing off would push those decisions into design unrecorded, which is the expensive version of this problem.

**To avoid blocking more than necessary, the review names what design can start on now:** the send form and its two error states (US-001), the Pending list layout and its resend/revoke affordances (US-003), and the happy-path accept page (US-002 criterion 1). None of those depend on an open finding. What waits is the expired-link page, the wrong-address page, and the role picker's permission behaviour.

**Turnaround estimate I would give:** A, D, E, G, I, J and K are text edits. C needs one decision from the tech lead (cap invitable roles at or below the inviter's own, most likely). B and F need the Phase 4 answer. This is a short revision, not a rewrite — I would say that explicitly so the verdict does not read as heavier than it is.

## Phase 6 — Delivery

**Default:** return the review in the conversation as prose plus the findings table above, in this order — verdict, the four blocking findings with the exact lines, the Phase 4 question, the majors and minors, and the "design can proceed on X" carve-out.

**Offer, not assumed:** I would also offer to write it to `review-FEAT-031-rev3.md` in the workspace, and I would ask before creating that file, since the workspace holds only the spec and I do not know whether reviews live here or in the tracker.

**Would not do:** amend `spec.md` myself. It is someone else's document on its third revision, and a reviewer silently editing the artifact under review destroys the revision trail.

**No delegation.** The document is 110 lines with all findings derivable from a single close reading; dispatching a worker would cost more than it returns and would add a layer between me and the text I am accountable for.