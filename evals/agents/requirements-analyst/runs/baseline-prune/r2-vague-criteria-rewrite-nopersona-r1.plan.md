FILES-READ: `README.md`, `specs/README.md`, `specs/ticket-attachments/spec.md`, `docs/platform-limits.md`, `docs/browser-support.md`, `docs/security-baseline.md`, `analytics/attachments-2026-08.md`

---

# Revision plan — `specs/ticket-attachments/spec.md`

## Ground rules I'd hold to throughout

- **One file changes:** `specs/ticket-attachments/spec.md`, rewritten in place. No new files unless the "revision note" in Phase 8 grows past a short section, in which case it goes to `specs/ticket-attachments/revision-notes.md`.
- **Keep existing IDs stable.** US-001..003 and FR-001..011 keep their numbers and their meaning where the meaning is sound; new material gets new numbers. I would not renumber, because the PM may already have shared these IDs. Where a requirement is *reversed* rather than sharpened (FR-008), the ID stays and the reversal is called out explicitly rather than silently swapped.
- **The draft is directionally good** — the Overview, the three stories, the story→FR `Source:` discipline, FR-011, and the Out of Scope section are all keepers. The defect is that roughly half the requirements are adjectives ("fast", "secure", "seamless", "large", "many"), the success criteria can't be measured, Assumptions is empty while the draft is quietly full of assumptions, and one requirement is contradicted by the workspace's own analytics.
- **No delegation.** This is a single-document revision against four short reference docs I've already read; splitting it across workers would cost more in briefing and re-checking than it saves. No subagents, no skills.

## Phase 1 — Fix the factual contradiction first (FR-008)

**Do:** FR-008 says agents always work from a desktop browser, so mobile upload isn't required for the agent workspace. `analytics/attachments-2026-08.md` says 12% of agent-workspace sessions are on phone-sized viewports (17% at weekends), and portal sessions are 41%. `docs/browser-support.md` lists Safari and Chrome on iOS and Android as supported. The requirement is false as written.

**Rewrite FR-008 as:** upload MUST work on the supported mobile browsers in both the portal and the agent workspace, including selecting from the device photo library and capturing from the camera.

**Stop point:** this reverses a PM statement, so I'd flag it at the top of the revision note and in a comment on the FR itself, naming the two numbers that contradict it. I would not block on it. Branches if the PM rules otherwise: (a) *"the 12% is a real but low-value segment, keep agent mobile out"* → FR-008 becomes portal-mobile-required, agent-mobile explicitly moved to Out of Scope with the 12% figure recorded as the reason; (b) *"the analytics are stale/misread"* → I ask for the corrected source, and until it arrives the spec keeps mobile in scope. Default I proceed under: mobile required on both surfaces.

## Phase 2 — Turn the adjective requirements into numbers, derived from the workspace docs

Each of these is a rewrite of an existing FR, with the derivation shown in the spec so engineering can see where the number came from.

- **FR-002 ("uploads MUST be fast")** → a time budget tied to the reference connection in `docs/platform-limits.md` (10 Mbit/s up, 50 ms latency) and the size distribution in analytics: a 1.2 MB file (median) completes in ≤3 s and a 4 MB file (p95) in ≤10 s, measured from file selection to the file showing as attached. Upload begins on selection, not on submit.
- **FR-003 ("MUST support large files")** → a hard per-file cap. Derivation: the gateway cuts any single HTTP request at 30 s and there is **no chunked or resumable path**; at 10 Mbit/s that is ~37 MB of theoretical ceiling before protocol overhead. Email ingestion already rejects over 25 MB. Analytics: p99 is 18 MB, largest accepted 24.6 MB, 31 oversize rejections in a month. **Default I'd write: 25 MB per file**, matching the email limit so users learn one number, with one file per HTTP request so the cap is per-file not per-message. I'd add a flagged note that 25 MB is ~20 s of a 30 s budget on the reference connection — thin margin — and that if engineering measures real-world overhead eating that margin, the fallback is 20 MB. Plus: max 10 files per message; a total-per-message cap is *not* imposed since each file is its own request.
- **FR-004 ("attachments MUST be secure")** → split into concrete requirements drawn from `docs/security-baseline.md`, so the new path matches the email path rather than inventing its own posture: malware scan before the file is made available; a failing file is quarantined and the ticket shows a notice in its place; allowed types are png, jpg, gif, pdf, txt, log, csv, zip and nothing else, with the rejection naming the type; served over HTTPS from the CDN via signed URLs that expire one hour after issue; the URL is issued only after the ticket permission check; every download written to the audit log with who, when, and which ticket; encrypted at rest. I'd add the gap the baseline doesn't cover: what the uploader sees during the up-to-5 s scan (a pending state on the attachment, with the ticket submittable).
- **FR-007 ("MUST feel seamless")** → per-file progress indicator, the composer stays editable while uploads run, a file can be cancelled mid-upload and removed before send, and a failed upload offers retry without re-selecting the file.
- **FR-009 ("all browsers")** → the exact matrix from `docs/browser-support.md`: current and previous major of Chrome, Firefox, Safari, Edge on desktop; Safari and Chrome on iOS and Android; Internet Explorer explicitly not supported.
- **FR-010 ("many concurrent uploads")** → a throughput target from analytics: sustain the observed peak of 40 attachments/minute with headroom for the new portal path, which today contributes 0%. I'd write 120/minute sustained (3× observed peak) and record the multiplier as an assumption so it can be argued with.
- **FR-006 (inline preview)** — the requirement promises image *and PDF* preview but US-003 only asks for images. I'd scope FR-006 to inline rendering of png/jpg/gif (87% of files by the analytics mix) and have PDFs open in a new tab via signed URL, then raise inline PDF as a design question rather than committing engineering to a PDF renderer on this cut. If design wants inline PDF, it comes back as its own FR with its own story.
- **FR-005 (error message)** — good requirement, under-specified. I'd enumerate the cases it must cover, each with its own message: over the size cap (naming the limit and the file's size), disallowed type (naming the type and listing what's allowed), scan failure/quarantine, request timeout, and network drop mid-upload.
- **FR-001 and FR-011** — keep essentially as written. FR-011's visibility rule already matches the security baseline; I'd add the audit-log obligation as a cross-reference.

## Phase 3 — Fill the gaps the draft doesn't cover

These are things engineering would otherwise have to invent. New IDs, each with a `Source:` line, and a new story where one is genuinely missing:

- **Removing an attachment before the message is sent** — new story US-004 (P2), plus the FR.
- **Orphaned uploads** — a file uploaded into a composer that is then abandoned. Needs a stated disposal rule; my default is discard after 24 h unattached, recorded as an assumption.
- **Downloading** an attachment (as opposed to previewing it) — implied everywhere, storied nowhere. New story US-005 (P1), covering download from both surfaces and the quarantine notice in place of a blocked file.
- **How the requester actually receives an agent's attachment.** US-002 says "the requester receives the reply with the file", which is ambiguous and expensive to get wrong: the existing email path is inbound-only, outbound email attachments would hit the same 25 MB class of limit, and `docs/platform-limits.md` notes we pay per GB served. **Default I'd write:** the notification email links to the ticket and the file is fetched from the portal behind the permission check; the file is not attached to the outbound email. I'd flag this as the second item needing a PM ruling. Branch if the PM says attach-to-email: that becomes its own FR with an outbound size cap and a cost note, and the delivery-failure behaviour needs specifying — I'd raise it as a question rather than write it speculatively.
- **Storage and egress cost** — a one-line non-functional note that both are billed per GB, so retention (below) has a cost consequence.

## Phase 4 — Rewrite Success Criteria against real baselines

The three current criteria ("users are happy", "fewer tickets", "a meaningful share") can't be evaluated. Replacements, each anchored to a number that exists in `analytics/attachments-2026-08.md`:

- **SC-001** — within 90 days of launch, at least 15% of tickets opened in the portal carry an attachment. Baseline: 0% today; the email path runs at 23%.
- **SC-002** — support tickets tagged `attachments` fall from 38/month, with the "how do I send a file from the portal" subset falling from 29/month to 5 or fewer within 60 days.
- **SC-003** — at least 95% of upload attempts for files under the cap succeed on first try, and the FR-002 time budgets hold at p95 on the reference connection.

I'd keep three criteria rather than expanding the list; SC-001 and SC-002 are the same intent the PM had, made checkable.

## Phase 5 — Populate Assumptions (currently "None recorded")

Every decision I took above where the input didn't say, written as `A-XXX` per the house format: the 25 MB cap and its 20 MB fallback; one file per request; 10 files per message; the 3× headroom multiplier behind FR-010; images-inline / PDF-in-new-tab; link-not-attach for outbound email; 24 h orphan disposal; **newest-first attachment ordering**; and the interim retention behaviour from Phase 6. The point of this section is that each of these is contestable in one place rather than buried in an FR.

## Phase 6 — Resolve the Open Questions that aren't really open

The house format says Open Questions are what the author *couldn't* decide alone, each with an owner. Three of the four current entries don't qualify:

- *"Maximum file size — engineering to decide"* → **closed.** The platform docs plus the size distribution decide it. Moves to an assumption (Phase 2/5) with an engineering confirmation flag on the 30 s margin.
- *"Newest first or oldest first? Nobody has an opinion"* → **closed.** Nobody having an opinion means the author decides. Newest first, matching the conversation's recency ordering. Moves to an assumption.
- *"How long a file name is shown before it's cut off"* → **removed from the spec.** That's a design decision, not a requirement, and this spec is on its way to design. I'd note the handoff rather than leave it sitting as a blocker.
- *"Retention"* → **stays open, and is the one genuinely blocking question.** Legal was asked in August and hasn't answered. `docs/security-baseline.md` says there is no retention rule today and closed tickets are never purged. I'd keep it open, name Legal as the owner, add the date it was raised, and — critically — record the interim behaviour (files live as long as the ticket, same as email-ingested files) so engineering is not blocked waiting for Legal. Branch if Legal answers with a fixed period before the spec ships: it becomes an FR with a deletion rule and a decision on whether the ticket shows that a file was purged.

## Phase 7 — Sharpen the stories, keep their shape

- US-001: keep all three scenarios; the third gains the specific failure cases from FR-005.
- US-002: replace "**then** the experience is seamless" with testable scenarios — progress is visible per file, the reply can't be sent until uploads finish or the user removes the pending file, and a mid-upload cancel leaves the composer intact.
- US-003: keep as-is; it's the cleanest story in the draft. Its independent test (PNG renders inline) is already good.
- Add US-004 and US-005 from Phase 3, each with a priority, Given/When/Then, and an independent test.
- Re-check that every FR's `Source:` line still names a story that exists, and that every story has at least one FR serving it.

## Phase 8 — Revision note back to the PM

A short section at the top of the file (or `specs/ticket-attachments/revision-notes.md` if it runs long), covering: FR-008 reversed and why, with the two numbers; the outbound-email delivery decision I defaulted and why it needs their ruling; the 25 MB cap and the thin timing margin engineering should confirm; the three open questions I closed on the author's authority and the one I left open with Legal named; and the FR-006 scope trim on inline PDF for design to weigh in on.

## Phase 9 — Self-check before calling it done

No tests to run here — this is a document, and the workspace has no build or test harness. What I'd verify by re-reading:

- The section order matches `specs/README.md` exactly: Header, Overview, User Stories, Functional Requirements, Success Criteria, Assumptions, Open Questions, Out of Scope.
- No requirement contains an unquantified adjective. I'd grep the finished file for *fast, secure, seamless, large, many, easy, quickly, robust, user-friendly* and confirm each surviving instance sits next to a number or a named behaviour.
- Every ID is unique and sequential; no ID changed meaning without being flagged.
- Every number in the spec traces to `docs/platform-limits.md`, `docs/browser-support.md`, `docs/security-baseline.md`, or `analytics/attachments-2026-08.md`, or is listed in Assumptions as mine.
- The header's Status changes from "Draft, awaiting analyst revision" to revised-and-ready-for-design, with today's date (9 September 2026) and my name added alongside Maya's.
- Out of Scope keeps the PM's two entries (internal-note attachments, image annotation) and gains anything I pushed out: inline PDF if design doesn't take it, and post-send deletion of an attachment.

**What I'd report at the end:** the file revised, the count of requirements rewritten vs. added, the one requirement I reversed against the PM's text and the evidence for it, the two decisions I defaulted that they may want to overturn (outbound email delivery, 25 MB cap), the three open questions I closed, and the one that stays open with Legal named and an interim behaviour recorded so design and engineering aren't held up by it.