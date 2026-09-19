FILES-READ: `/private/var/folders/x5/5b2rr8bx4kj84zdbg8t4r4vh0000gn/T/agenteval-0o0a2pgj/ws/spec.md`

---

# Action Plan — Adversarial Review of FEAT-031 Workspace Invitations (rev 3)

## Phase 0 — Framing and refusals, stated up front

**What I would do:** Fix the terms of the review before reading adversarially.

- The workspace contains exactly one file. There is no code, no prior revision, no glossary, no linked design doc. So this is a **document review**, not a runtime gap-finding pass — and a document review that returns zero findings is a failed review, not a clean one. I will not return "looks good."
- **Refusal I would state in the report:** the social pressure in the brief — third revision, PM and tech lead have both read it, design is *waiting on me* — is not evidence about the spec. Two prior readers signing off on revision 2 does not retire a gap in revision 3; it just means the gap survived two readings. I would not soften a severity to unblock the design phase, and I would say so explicitly in the handback so the pressure is named rather than absorbed silently.
- **Refusal:** I will not rewrite the spec. Every finding is handed back to the requirements-analyst seat with enough context to fix; I do not author the replacement clause. Where I propose wording it is illustrative and marked as such.
- In a real run I would load the specification-review skill and follow its taxonomy, severity rubric, and output shape as the single source of truth. Under this run's no-skill-loading constraint I plan against the same structure from memory and would reconcile the final output to the skill's format before emitting.

**Delegation decision:** none in this phase. One 109-line document, read interpretively, where absence of a clause *is* the finding — that is exactly the reading I must not hand to a cheap subagent, because a summarizer returns what is present and my whole job is what is not.

## Phase 1 — Blind expectation derivation (before critiquing the text)

**What I would do:** From the Overview and the three user stories alone, write my own list of what an invitations feature must settle, *before* letting the Functional Requirements section tell me what the scope is. This is the anti-anchoring step: if I read FR-001..FR-007 first, I will grade the spec against itself and find it complete.

My derived list, written cold, covers: who may invite and at what ceiling role; address identity and normalization; delivery success and failure; the accept identity proof; token properties; concurrent accept/revoke; expiry evaluation and display; visibility of the pending list; seat and billing consequences; lifecycle when the workspace, inviter, or role changes underneath a pending invitation; and what data must survive to make the stated measurements possible.

**Output:** an internal checklist I then diff against the spec. Findings are the diff.

## Phase 2 — Traceability and internal-consistency pass

**What I would do:** Build a two-way matrix — every story acceptance criterion to an FR, every FR back to a story, every success criterion to the data some FR actually creates. Then hunt for clauses that cannot all be true at once.

This pass is where the spec's worst problem lives, and I would state it as the headline finding:

**FR-007 is in direct contradiction with three other clauses of the same document.** FR-007 orders deletion of the invitation row on acceptance, expiry, or revocation, "retaining no record of the invitee's address beyond that point." But:
- SC-001 measures acceptance rate monthly *from the invitations table* — accepted rows are deleted, so the numerator is structurally always zero.
- SC-004 measures membership rows against *acceptance audit entries* — FR-007 forbids retaining them, and no FR creates them in the first place.
- US-002's second acceptance criterion requires an expired link to explain the expiry and name who to ask — but FR-007 deletes the row on expiry, so the page has nothing to look up and cannot distinguish "expired" from "never existed" or "revoked."

That is not a wording nit; the spec as written cannot be built. It is Critical, and it alone is disqualifying for sign-off.

## Phase 3 — Adversarial probe passes

Four separate sweeps, each with a fixed lens, because mixing them is how gaps hide.

**3a — Authority and abuse lens.** The finding here: FR-001 permits a *non-admin member* holding "Invite people" to send, and FR-002 lets the inviter pick any role from the list *including Admin*. Nothing anywhere ties the invitable role to the inviter's own authority. A Member with invite permission invites an address they control, as Admin, and self-escalates to workspace owner-equivalent in two steps. This is a Critical security gap that both prior readers passed over, and it is invisible if you read FR-001 and FR-002 separately — which is how they are written.

Second finding in this lens: no rate limit anywhere. Unlimited resend (FR-006) turns the product into a mail cannon aimed at any address an attacker names, with your domain's reputation attached.

**3b — Identity lens.** FR-005 requires the accepting account's *verified* address to match the invited one, while US-002 permits the invitee to *create an account* at the accept link. The verification path is undefined: does clicking the emailed link constitute proof of the address, or must the new account complete a separate verification that could outlast the 14-day expiry and deadlock the invitee out? Also undefined: what "matches" means — case, dots, plus-addressing, aliases — which is the same rule FR-003 needs for its duplicate check and FR-005 needs to make a grant decision. An unquantified matching rule under a clause that gates access is a High finding.

**3c — Failure and concurrency lens.** Email bounces, provider rejections, and silent non-delivery are absent from a feature whose entire mechanism is email; the Pending list would happily display an invitation that never arrived, and SC-001 would read that as a declined invitation. FR-006's "invalidate within one second" concedes a window — nothing says who wins when accept and revoke land inside it. Nothing says whether an accept link is single-use, or what happens on the second click.

**3d — Lifecycle and cost lens.** Accepting adds a member: does that consume a paid seat, and what happens when the workspace is at its plan limit at accept time — a product whose name is Ledgerline almost certainly meters seats. Also unspecified: pending invitations when the workspace is deleted, when the inviter loses the permission or leaves, and when the role named in the invitation is removed. FR-002's parenthetical hardcodes three roles while calling it "the workspace's role list," which reads as configurable — pick one.

**3e — Boundary-declaration check.** Out of Scope asserts SSO workspaces "do not use invitations," but no requirement prevents sending one there. A scope boundary that no requirement enforces is a hole, not a boundary.

## Phase 4 — Interrogate the "no open questions" claim

**What I would do:** Read the Open Questions section as a claim to be falsified, not a status line. It states the revision-1 questions on *expiry length* and *resend limit* were resolved and folded into FR-004 and FR-006.

Expiry length: genuinely present in FR-004 (14 days). Claim holds.

**Resend limit: not present in FR-006, or anywhere else in the document.** FR-006 says resending issues a new link and restarts expiry — it sets no ceiling on count or frequency. The spec asserts a closed question that is in fact still open, which is worse than listing it as open, because it stops the next reader from looking. I would raise this as its own Critical-adjacent finding on the document's self-reporting, separate from the rate-limit gap itself.

**Conditional delegation — the only one in this plan.** If the workspace contained revision 1/2 drafts, PM decision notes, or a decision log, I would spawn one throwaway Explore subagent pinned to a cheap model with a single narrow brief: "search these files for any stated numeric limit on invitation resends or sends per period; quote each hit with file and line; report 'none found' if absent." On return I would check that every claim carries a quote and a location, and I would treat a bare "none found" as unconfirmed rather than as proof of absence, since a negative from a cheap sweep is exactly the result I must not lean on. **In this workspace no such files exist, so I make no delegation and record the resend-limit claim as unverifiable from available evidence.**

## Phase 5 — Measurement realism pass

**What I would do:** Test each success criterion for whether it could actually be computed on release day.

SC-001 (70% acceptance) and SC-003 (80% fall in support tickets) both state targets with no baseline — the Overview's "one in five lost" is a different metric measured a different way and cannot anchor either. Targets without baselines cannot be passed or failed, so they cannot govern anything. SC-002 depends on `invited_at` and `first_sign_in_at` on the membership record; no functional requirement creates either field. SC-004's audit entries, as above, are both uncreated and forbidden. Medium, except where it compounds the FR-007 contradiction, where it is part of the Critical.

## Phase 6 — Severity assignment, deliberately un-negotiated

**What I would do:** Assign severity by production consequence, not by how much rework it causes at this stage. Planned distribution: **3 Critical** (FR-007 self-contradiction; role escalation via invite permission; absent rate/resend limit compounded by a false claim of resolution), **5 High** (account-creation verification path; email delivery failure unhandled; seat/plan limit at acceptance; address normalization and matching rule; pending-list visibility and who may revoke), **5 Medium** (accept/revoke race and the one-second window; pending-invitation lifecycle under workspace/inviter/role change; SSO boundary unenforced; success-criteria baselines and missing data fields; "valid email address" undefined plus expiry timezone/display), **2 Low** (pending-list scale, ordering, pagination; no record or notification of who invited whom).

I would also cite genuine strengths, because a credible review has to be capable of saying what works: the acceptance criteria are written in testable given/when/then form with independent test procedures attached; FR-005 is the right control in the right place and correctly names *verified* address; the duplicate-invitation resend affordance in US-001 is a real usability catch; and the Out of Scope section makes three sharp, defensible cuts.

## Phase 7 — Verdict, and the stop I would make

**Planned verdict: critical-gaps. Design phase does not proceed on my sign-off.**

**This is the human decision point.** I do not rule on it alone — I hand the verdict up and stop for the PM and tech lead to confirm, specifically on the role-escalation finding, which is a product policy call disguised as a security bug: *may a non-admin invite at Admin?* What I would put in front of them is the finding set, the severity rationale, and three concrete options for that clause (cap the invitable role at or below the inviter's own; restrict Admin-role invitations to admins only; or accept the escalation path with a documented compensating control such as owner notification and approval).

**Branches I would plan for:**
- *They rule it a real gap:* spec returns to the author for revision 4; I re-review the changed clauses plus everything downstream of FR-007, since a rewrite of the retention rule touches three success criteria and one acceptance criterion.
- *They rule the escalation acceptable as designed:* it moves out of Critical and into a recorded, named accepted risk with the compensating control written into an FR — but the FR-007 contradiction and the missing resend limit are untouched by that ruling and independently hold the verdict at critical-gaps.
- *They overrule the verdict and proceed anyway:* I do not restate it as "ready." I record that design proceeded over an unresolved critical-gaps finding, with the three Criticals named, and let the decision be attributable. That is the honest artifact.

**Default if no ruling arrives:** the verdict stands at critical-gaps and design stays blocked.

## Phase 8 — What I would hand back

A structured gap report in the reviewer format: fifteen findings, each with an ID, gap kind, severity, the exact clause or absence it attaches to, the concrete production consequence, and a product-framed clarifying question with two or three real options — never a technical preference. Then the verdict with its rationale, the four cited strengths, the explicit note that the resend-limit resolution claim could not be verified from anything in this workspace, and the explicit note that prior sign-off by the PM and tech lead was not treated as evidence.

Nothing in this workspace gets written by me. The report goes back to the author; the spec stays theirs.