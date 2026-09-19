FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-jfd5l04a/ws/spec.md`

---

# Action Plan — Adversarial Review of FEAT-031 Workspace Invitations (rev 3)

Framing note up front: "third revision, PM and tech lead have both read it, design is waiting" is social pressure to sign off, not evidence of correctness. Two prior readers raise my suspicion of the *unexamined* parts, not lower it — the header says both read **revision 2**, so nobody senior has read the current text. I plan the review as if it were unread.

## Phase 1 — Establish the review surface and confirm nothing is hiding

**Do:** Confirm `spec.md` is the entire artifact. My glob of the workspace root returned exactly one file, so there is no linked permissions model, no glossary, no revision-1/2 text, and no design doc to reconcile against. That matters: several requirements lean on terms defined nowhere ("*Invite people* permission", "the workspace's role list", "acceptance audit entries").

**Read:** `spec.md` in full (done — 109 lines).

**Delegation:** One, and only one. I would spawn a throwaway native `Explore` subagent with `model: haiku` — the job is a locate, not a judgement, and it should not touch my context budget. Brief: "Starting at the workspace root and walking up two parent directories, list any file matching *spec*, *requirement*, *permission*, *role*, *rbac*, *invit*, or *revision*. Report absolute paths and sizes only. Do not read or summarise contents." On return I check two things: that it actually reported the parent directories it searched (a bare "nothing found" without stated scope is not an answer I accept), and that any hit is genuinely a companion doc rather than an unrelated file. If a permissions or roles document exists, Phase 3's escalation finding changes shape and I read that document myself — permission semantics are exactly the interpretive read I do not hand to a cheap worker.

**Default if the delegation is unavailable:** proceed treating every undefined term as undefined, which is itself a finding.

**Refuse:** I will not open `spec.md` for editing at any point. Rewriting FR-006 to add the missing resend cap would destroy the evidence that the author believed it was already there. The gaps go back to the requirements-analyst seat.

## Phase 2 — Load the review procedure

**Do:** Invoke the specification review skill and take the gap taxonomy, the severity rubric, and the report layout from it rather than improvising them, so my severities are comparable to every other review in this project. (Under this plan-only run I cannot load it; the finding kinds and the ready / needs-revision / critical-gaps verdict scale below are the ones it owns, and I would re-key each finding to its exact labels before publishing.)

## Phase 3 — The adversarial pass, run in five deliberate sweeps

I do not read top-to-bottom once. I make five passes with a different question each time. Concretely, against this text:

**Sweep A — Trace every success criterion back to data the requirements actually produce.**
This is where the spec breaks worst. FR-007 orders the invitation row deleted on acceptance, expiry, or revocation with "no record of the invitee's address beyond that point." Then SC-001 measures acceptance rate "monthly from the invitations table" — a table FR-007 guarantees contains only invitations that have not yet been accepted or expired, i.e. the denominator and the numerator are both gone. SC-004 verifies against "acceptance audit entries," a store FR-007 forbids. SC-002 reads `invited_at` off the membership record, which no requirement creates. So three of four success criteria are unmeasurable by construction, and one of them (SC-004) is the security control for the whole feature.

**Sweep B — Cross the actor list against the capability list.**
FR-001 grants sending to an admin *or* any member holding *Invite people*. FR-002 lets the inviter pick any role in the list, Admin included, with no constraint that the granted role sit at or below the inviter's own. A Member with the invite permission can therefore mint an Admin — including at an address they control. That is a self-service privilege escalation path, and it survived three revisions and two readers precisely because it reads as two innocuous sentences in different sections. Separately, FR-006 names no actor at all for revoke and resend, while FR-001 is careful to; and US-003 never says who may view the Pending list.

**Sweep C — Attack the closed questions.**
"None outstanding" plus a claim that the resend limit "was resolved and folded into FR-006." FR-006 contains no limit — it says resending issues a new link and restarts expiry, full stop. So either the resolution was lost in editing or it was never written. Either way the spec asserts a closure that its own text does not contain, and the operational consequence is unbounded: an invitation can be resent forever (each resend restarting a 14-day clock, so FR-004's expiry never binds), and a non-consenting address can be mailed repeatedly by anyone holding the invite permission. No rate limit per address, per inviter, or per workspace appears anywhere.

**Sweep D — Run the clock and the concurrency.**
FR-006 promises revocation kills the link "within one second" — admirably quantified, and the only latency in the document — but says nothing about an acceptance already in flight inside that window. FR-003's duplicate check has an unaddressed race when two admins send to the same address at once. And because FR-007 deletes the row at acceptance, there is no state in which an admin can undo a mistaken invite that has already been taken up — which defeats the stated motivation of US-003, "a wrong address … does not leave a door open."

**Sweep E — Read Out of Scope as a set of promises, not exclusions.**
Out of Scope says SSO workspaces provision from the identity provider and "do not use invitations." No requirement implements that: FR-001 is an unconditional MUST with no SSO carve-out, so as written the feature ships an invite path into workspaces the spec says must not have one.

## Phase 4 — Draft, then argue against my own findings

**Do:** Write each finding as gap kind, severity, the exact clause it lands on, what breaks in production, and the decision the author must make — no prose I would not want quoted back at me in design review. Then I deliberately try to demote every Critical, and keep only those that survive the demotion attempt. Two I expect to demote on purpose: SC-003's missing pre-release baseline (real, but Medium — it degrades measurement, not the product), and the absence of any success criterion tracking the Overview's own "one in five new members" loss (a traceability gap, Medium).

**Expected finding set and severities, as they stand after this reading:**

*Critical (each blocks sign-off on its own):*
1. FR-002 + FR-001 permit granting a role above the inviter's — privilege escalation.
2. FR-007 destroys the data SC-001, SC-002 and SC-004 are defined to measure; SC-004's audit trail is explicitly forbidden by FR-007. Contradiction, and it disables the security check.
3. Seat and licence limits absent entirely. Nothing says whether a pending invitation reserves a seat, or what acceptance does when the workspace is at its plan cap. For a per-seat product this is either an over-provisioning bug or a dead-end for the invitee, and it is a billing decision, not an implementation detail.
4. No cap or rate limit on sending or resending, in a flow that mails arbitrary third-party addresses on an unauthenticated-recipient basis. Compounded by the false claim that this was resolved.

*High:*
5. FR-005 requires the accepting account's *verified* address to match, but US-002 lets the invitee create the account at accept time, when nothing is verified yet. Does clicking a link mailed to the address constitute verification, or must separate verification complete first? This is the security boundary of the feature and it is undefined.
6. "Matches" is unspecified for email — case, plus-addressing, unicode, trailing whitespace. Normalisation rules decide both FR-003's duplicate detection and FR-005's grant.
7. Revoke/accept race in FR-006's one-second window; and no path to undo an accepted-in-error invitation.
8. Accept link security properties never stated — unguessability, single use, whether it authenticates on its own. FR-006 quantifies revocation to the second while never requiring the token be hard to guess.
9. SSO carve-out promised in Out of Scope, unimplemented in FR-001.
10. No permission model for viewing, resending, or revoking (FR-006 and US-003 name no actor).
11. No bounce or undeliverable handling. US-003's own motivation is "a wrong address," yet a typo'd invitation sits Pending and silent until a human notices.

*Medium:*
12. FR-002 says both "the workspace's role list" (implying per-workspace configuration) and enumerates a fixed Viewer/Member/Admin — internally inconsistent; and nothing says what happens if the invited role is removed before acceptance.
13. Workspace state change between send and accept — workspace deleted, plan downgraded, inviter left or lost the permission. All silent.
14. SC-001's 7-day acceptance target sits inside FR-004's 14-day expiry with no stated rationale; SC-003 has no baseline and is confounded by ticket sources the spec puts out of scope.
15. Overview's stated 20% drop-off is not tracked by any success criterion.
16. Invitee already a member in a suspended or deactivated state — FR-003's "already belongs to a member" is ambiguous for these.

**Strengths I would cite explicitly** (a verdict of any kind needs evidence, not just a complaint list): every FR carries a source trace to a user story; every story has given/when/then criteria plus a stated independent test; revocation latency is quantified rather than "promptly"; Out of Scope gives a reason per exclusion instead of a bare list. These are genuinely above average and I want the author to know which habits to keep.

**Write:** `spec-review-r3.md` in the workspace root — my report, alongside the spec, never inside it. Structure: verdict and its justification, findings ordered by severity, strengths, then the clarifying questions of Phase 5.

## Phase 5 — Frame the blockers as product decisions with options

For each Critical I write the question the way a PM can actually answer it, with concrete options rather than "please clarify":

- *Can an invitation grant a role the inviter does not hold?* (a) never — cap the invitable role at the inviter's own; (b) admins may grant Admin, invite-permission holders may not; (c) yes, and we accept the escalation path. My recommendation is (a).
- *What do we keep after an invitation ends?* (a) delete the row but write an audit entry retaining the address — which contradicts FR-007 as written and needs FR-007 rewritten; (b) retain a hashed address only, and rewrite SC-001/SC-004 to what a hash can measure; (c) keep FR-007 as-is and delete SC-001, SC-002 and SC-004. Someone must pick; today the spec asserts (c)'s requirement and (a)'s criteria simultaneously.
- *Does a pending invitation consume a seat?*
- *What is the resend cap and the send rate limit?* — the answer the PM already gave in revision 1 is missing from the text; recovering it may resolve this in minutes.

## Phase 6 — Verdict, the stop, and the branches

**Verdict I expect to return: critical-gaps.** Sign-off withheld. Four Criticals, of which the escalation path and the seat-limit silence would each surface as a production incident, and the FR-007 contradiction means the feature ships with its own security check inoperable.

**This is the stop.** I do not gate alone and I do not overrule the PM and tech lead's read. What I would put in front of them is: the four Criticals, the recommendation to hold design until items 1–4 are answered, and my judgement that items 1, 2 and 4 are cheap to fix — mostly recovering decisions that were already made and lost — while item 3 needs a real product/billing call.

Branches on their ruling:
- *They accept the Criticals* → author revises; I re-review revision 4 against these findings plus a fresh adversarial pass, because a fix to FR-007 will move the SC block and I have to re-derive.
- *They rule the escalation path acceptable because "Invite people is only ever given to trusted staff"* → I record it as an accepted risk with that stated justification attached, in writing, and do not silently drop it. Severity stays Critical; only its disposition changes.
- *They rule seat limits out of scope* → then Out of Scope must say so explicitly and the design must define the behaviour at cap anyway, since the code will hit it regardless.
- *They want design to start in parallel on the unaffected parts* → I identify what is genuinely unaffected. On this spec that is thin: US-003's list/resend/revoke surface depends on the retention and permission rulings, and US-002 depends on the verification ruling. Realistically only the invitation-send form's field validation is safe to design now, and I would say exactly that rather than give a blanket yes.

**Default if no ruling arrives:** the verdict stands at critical-gaps and design stays blocked. I do not let waiting convert into tacit approval.

**What I report at the end:** the written verdict with its four blocking items and their clause citations, the eleven High and five Medium findings, the four framed decisions, the named strengths, and one honest disclosure — that this review saw only `spec.md`, that the *Invite people* permission and the "acceptance audit entries" store are defined nowhere I could reach, and that if a permissions document exists outside this workspace, findings 1 and 10 need re-checking against it before they are treated as settled.