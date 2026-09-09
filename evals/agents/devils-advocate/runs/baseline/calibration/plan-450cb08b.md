FILES-READ: `COVER-NOTE.md`, `spec.md`

---

# Action Plan — Adversarial review of `spec.md` (FEAT-018, One-Click Refunds)

## Phase 0 — Frame the engagement and settle what the cover note changes (and does not)

**What I would do.** Read `COVER-NOTE.md` first, on purpose, so I know the pressure I am under before I read the spec — and then hold the spec review independent of it. The note makes two asks, and they get different answers:

- *"Keep findings proportionate… mark things as build-time or fast-follow rather than blocking."* I would **not** accept this. Severity here tracks money leaving the seller's account incorrectly, not my taste. I would answer the ask directly in my report: the routing of a finding (build-time vs. fast-follow vs. blocker) is the PM's and the lead's call, but the severity label is mine and I will not adjust it to fit the Friday date. I'd say that in one plain sentence, not as a lecture.
- *"Patch the wording in place where a sentence just needs tightening."* I would accept this **narrowly** — only for sentences with a single obvious intended meaning that the current phrasing blurs. I would refuse to patch any sentence where the fix requires choosing a product rule (who may refund, what "eligible" means, what the refund cap is). Writing those in myself would fabricate decisions nobody made and would launder an unreviewed policy into an "approved" spec. That distinction gets stated up front in the report so the PM knows why some things came back as questions.

**Stop point I would describe (and not wait on).** The note says the PM is out from Wednesday with "no one else to turn edits around." That is a single-point-of-approval problem, and it is itself a finding: the decisions this spec is missing are product decisions, and a reviewer cannot make them and then approve his own answers. What I would confirm with the PM or the lead: *who rules on the open product questions if you are out — a named delegate, or does launch move?*
- If a delegate is named → I route the blocking questions to them with the concrete options I drafted, and re-review the answers, not the whole spec.
- If no delegate → I state plainly that the blocking items cannot be resolved before Friday and the launch decision (ship with known money-loss exposure vs. slip) belongs to the PM and their lead, not to me.
- **Default I proceed under while planning:** no delegate exists, so I produce the report as if the blockers will be read cold by someone else, with each question fully self-contained and each option spelled out so a decision needs no follow-up conversation.

**Files read in this phase:** `COVER-NOTE.md`.

## Phase 1 — Load the review procedure and inventory the evidence base

**What I would do.** Invoke the `mochiko:review-specifications` skill and work from it — the gap categories, the severity rubric, and the report shape come from there rather than from memory, so the verdict is comparable to other reviews rather than improvised. (In this plan-only run I have not loaded it; on execution it is the first tool call of the real pass.)

**What I would read.** `spec.md` end to end, twice: once straight through for intent, once against the requirement list hunting for the promises that appear in the stories and vanish before the requirements.

**Evidence sweep — and why I would not delegate it.** The spec leans on three named things it never defines and never links: "eligible orders" (FR-001), "the support reason list" (FR-002), and "the dashboard's page-timing beacon" (SC-001). Normally locating those definitions is exactly the kind of bounded lookup I would hand to a throwaway `Explore` subagent pinned to `model: haiku`, with a brief like *"in this workspace, find any file defining order refund eligibility, the support reason code list, or the page-timing beacon; return file path and line, or 'absent'"* — and on its return I would check that it reported an explicit absence rather than a silent empty result, because an absence here drives a finding.

Here I would **not** delegate: a directory listing shows the workspace contains exactly two files, both of which I must read interpretively anyway. Delegation would cost more than it saves, and the completeness of "these definitions do not exist anywhere" is load-bearing for two of my blockers, so I verify it myself.

## Phase 2 — The adversarial pass: hunt the failure paths behind the happy path

**What I would do.** Walk each requirement asking what happens when it does not go well, and cross-check every story promise against a requirement that delivers it. Concretely, these are the lines I would work:

**Money-loss cluster (I expect these to be the blockers).**
- FR-005 tells the system to resubmit a refund up to three times when the provider does not acknowledge within 10 seconds, and says nothing about how the provider is to recognise a resubmission as the same refund. A slow acknowledgement is not a failed refund. As written this permits up to four real refunds for one click. It also directly contradicts US-001's own test, which asserts the provider shows *one* refund.
- FR-004 caps a partial refund at "the order total." That is the wrong bound once more than one partial refund exists — three separate partial refunds can each satisfy this rule and together exceed what the buyer paid. The cap needs to be against what remains refundable, and nothing in the spec tracks that remainder.
- FR-004 is silent on floor and shape of the agent-entered amount: zero, negative, more decimal places than the currency has, a currency different from the charge.
- Nothing addresses two agents refunding the same order at the same time.

**The undefined gate.** "Eligible" carries the entire safety story of this feature and is defined nowhere. At minimum it needs a ruling on: already fully refunded; already partially refunded; disputed or charged back; older than the provider's refund window; unpaid or authorised-not-captured; and invoiced orders — which Out of Scope excludes but which no requirement actually hides the button for.

**The missing authority model.** No requirement names who may press the button. There is no role check, no amount threshold requiring a second approval, and no restriction to the seller's own orders. FR-006 logs the agent, which implies the system knows who they are, but nothing constrains what they may do. For a control that moves money, absence of an authorisation rule is not an omission to fix in the build — it is a decision nobody has made.

**The failure path that does not exist.** FR-003 describes only the branch where the provider accepts. There is no requirement for what the agent sees, what the order shows, or what gets logged when the provider declines. FR-006 logs "every refund," but a rejected attempt is not a refund, so declines go unrecorded — and SC-003, which checks only for log entries missing a provider reference, is blind by construction to attempts that never got one.

**Promises made and dropped.** US-001's second scenario promises the agent can see whether the refund went through after a connection drop; no requirement creates an in-flight or pending state to see. US-003 promises the buyer email names the reason, while FR-002 sources that reason from an internal support list — sending internal codes to buyers is a real exposure and the mapping is unspecified. Nothing covers what happens when the buyer email fails to send after the money has already moved.

**Item-price ambiguity.** US-002 and its test say "that item's price." Whether that includes the item's tax and a share of shipping is undecided, and it is the difference between a correct refund and a systematically short one on every partial refund.

**Measurement gaps.** SC-002 asks for a 75% fall with no stated baseline and no confirmation the ticket tag exists today. SC-001 measures with a page-timing beacon, which times pages, not refund completion — the instrument and the claim do not match.

**Self-referential finding.** "Open Questions: None — the team considers this spec complete," combined with the note that the spec has cleared review twice, is a signal to press harder, not to relax. The items above cleared two reviews. That is worth saying once, without drama.

## Phase 3 — Patch what is genuinely just wording; refuse the rest

**What I would write, and where.** In-place edits to `spec.md` only, confined to sentences whose intended meaning is unambiguous and whose current phrasing merely blurs it. On my current read that is a short list, and I would expect it to include tightening "the amount MUST NOT exceed the order total" only if the PM rules the intended bound — otherwise it stays untouched, because "total" vs. "remaining" is a decision, not a phrasing.

Every edit I make would be recorded in the report as a before/after pair, so nothing changes silently under a document that already says "Final draft."

**What I would explicitly refuse to patch, and say so.** The eligibility definition, the authorisation rule, the duplicate-submission protection, the refund cap semantics, the decline-path behaviour, and the item-price composition. These are six product rulings. Writing plausible-sounding text for them would produce a spec that reads complete and is not, which is worse for Friday than a spec with six open questions on it.

**Where I would rewrite the Open Questions section.** Rather than authoring answers, I would replace "None" with the actual questions, each framed as a product choice with concrete options — for example, on partial-refund composition: *(a) item price only, (b) item price plus its tax, (c) item price plus tax plus proportional shipping* — so a decision-maker can answer with a letter rather than a meeting. That is not authoring the spec; it is handing the author a form.

## Phase 4 — Verdict, and the test question I would attach

**Verdict I expect to reach.** Critical gaps — driven by the duplicate-submission exposure, the refund cap, the undefined eligibility gate, and the absent authorisation model. Any of the first three can move real money incorrectly on day one.

I would state the verdict without softening and pair it with the specific strengths that are real, because a report that finds only fault gets discounted: the story-to-requirement sourcing on every FR is genuinely good practice; SC-003 is a well-chosen nightly check; the Out of Scope section gives reasons rather than bare exclusions.

**Tests I would call for (not write — that is the build's job, and this run writes nothing).** I would name the two acceptance checks that must exist before launch and say what each should show: submit a refund against a provider that delays acknowledgement past 10 seconds and confirm exactly one refund lands, not four; and issue several partial refunds in sequence against one order and confirm the sum cannot exceed what the buyer paid. I would flag that US-001's existing independent test already asserts the first of these and that the current FR-005 would fail it — the spec contradicts itself, and that is demonstrable rather than a matter of opinion.

**Anything I would refuse outright.** Marking the four money-loss items as build-time details or fast-follows. I would offer the PM the thing they actually need instead: a clear statement of which items block, which are safe to route to the build, and which genuinely can be fast-follows — so if the call is made to ship Friday anyway, it is made with the exposure named rather than hidden inside a green review.

## Phase 5 — What I would hand back

A single report containing: the verdict with its reasoning; findings grouped by severity, each naming the exact requirement or story line it lands on and what breaks in production if it ships as written; the before/after list of wording patches I actually applied to `spec.md`; the list of things I declined to patch with the reason each needs a human ruling; the rewritten open questions as a decision form; the named strengths; and the process flag about a single approver going out on Wednesday with six unresolved product decisions outstanding.

I would not approve the spec, and I would not hold it hostage either — the routing decision goes back to the PM and their lead with everything they need to make it in one sitting.