FILES-READ: `README.md`, `specs/README.md`, `specs/ticket-attachments/spec.md`, `docs/platform-limits.md`, `docs/browser-support.md`, `docs/security-baseline.md`, `analytics/attachments-2026-08.md`

---

# Action Plan — Revise `specs/ticket-attachments/spec.md`

## Phase 0 — Ground rules and framing

**What I'd do.** Treat this as a revision, not a rewrite. Maya's Overview, the three-story shape, the story/FR/scope skeleton, FR-001, FR-006 and FR-011 are sound and survive. The failure mode of this draft is narrow and specific: it defers to adjectives ("fast", "large", "secure", "seamless", "user-friendly", "all browsers", "many") in a workspace where every one of those adjectives already has a number sitting in `docs/` or `analytics/`. The revision's job is to spend those numbers, not to invent new ones.

**Confirmed already:** `specs/README.md` fixes the section order and the `A-XXX` identifier for Assumptions, and requires Open Questions to name an owner. The current draft violates both (Assumptions is "None recorded"; three of four Open Questions name no owner). I edit **in place** at `specs/ticket-attachments/spec.md` — same path, same section order.

**Skills I'd load first:** `mochiko:authoring-requirements` before touching the FR/SC block, `mochiko:authoring-user-stories` before touching the story block. I don't write from memory of the templates; those two are the format authority and the house format has its own local rules (the `Source:` line, the independent test) that must sit alongside them.

**Delegation.** Not warranted for the reading — the workspace is seven files and I have read all of them; handing that off would cost more than it saves. The one spawn I *would* make is a disposable `Explore` subagent at `model: haiku`, brief: *"Search the entire tree for the strings attachment, upload, file size, retention, quarantine, signed URL. List every file and line that matches, excluding `specs/ticket-attachments/spec.md`, `docs/platform-limits.md`, `docs/browser-support.md`, `docs/security-baseline.md`, `analytics/attachments-2026-08.md`. Quote the matching line. Do not summarise or interpret."* — a completeness check that no constraint on this feature lives in a file I haven't opened. On return I'd check that every hit carries a file path and line, and open anything outside the five known files myself rather than trusting the summary. If it returns nothing new, the docs I've read are the whole constraint set and I proceed.

## Phase 1 — Build the constraint table (no writing yet)

**What I'd do.** Pin each vague term in the draft to the fact that resolves it, so the revision is derivation rather than assertion. Working table:

| Draft says | Resolved by | Becomes |
|---|---|---|
| FR-002 "fast" | 30 s gateway cut-off; 10 Mbit/s / 50 ms reference profile | a completion-time threshold at a stated file size on the stated connection |
| FR-003 "large files" | 5 GB object limit vs. 30 s cut-off with **no chunked or resumable path**; p99 18 MB, largest accepted 24.6 MB; email limit 25 MB | a hard per-file cap, derived below |
| FR-004 "secure" | five distinct clauses in the security baseline | five separate requirements |
| FR-005 "user-friendly error" | email path already bounces *naming the limit* | per-failure-mode message requirements |
| FR-007 "seamless" | nothing — pure adjective | progress, cancel, non-blocking compose |
| FR-008 "agents always desktop" | analytics: 12% of agent sessions under 768 px, 17% weekends | **contradicted — see Phase 3** |
| FR-009 "all browsers" | browser support list; IE unsupported since 2024 | the named matrix, explicitly |
| FR-010 "many concurrent" | 600/day, peak 40/min Monday 09:00 | a sustained-rate target |

**The size derivation, which is the one real piece of analysis here.** 25 MB at the reference 10 Mbit/s is ~20 s of transfer before TLS, multipart and latency overhead, against a 30 s hard cut-off with no resumable fallback. That is feasible but has no margin — any requester slower than the reference profile gets a gateway timeout on a file the system told them was acceptable. This is a genuine engineering-visible risk the draft hides behind "MUST support large files", and I'd surface it rather than bury it.

## Phase 2 — The stop, and how I proceed through it

**What I would confirm with Maya (and, for one item, Legal):** three rulings.

1. **Per-file cap.** Options: 25 MB (parity with the email path, so a requester never finds one channel accepts a file the other rejects) or 20 MB (comfortable margin under the 30 s cut-off).
   - *If 25 MB:* I write the cap at 25 MB and add a requirement that the client checks size **before** upload starts, so an oversize file fails instantly instead of after a 30 s stall, plus a distinct timeout message that says the connection was too slow rather than repeating the size limit.
   - *If 20 MB:* same, plus an explicit note in Out of Scope that portal uploads are stricter than email ingestion, and an Open Question owned by Support about how that inconsistency is explained to requesters.
   - **My default, and what I'd write absent a ruling: 25 MB, as an assumption with the derivation stated inline.** Parity is worth more than margin, it covers the 99th percentile (18 MB) and the largest file ever accepted (24.6 MB), and the pre-flight check plus a specific timeout message contains the downside. I would *not* leave this as "engineering to decide" — it is a product decision with a defensible answer already in the docs, and leaving it open is what sends this feature to design undefined.
2. **Retention.** Legal was asked in August and has not answered. I do not invent a retention rule.
   - *If Legal answers before design:* the answer becomes a requirement.
   - *Default I write:* attachments follow existing behaviour — kept as long as the ticket exists, closed tickets never purged, matching the email path — recorded as an assumption, **and kept as an Open Question owned by Legal with the August ask date**, because retention touches data policy and a silent default there is not mine to take. I'd flag it as the one item that can block launch rather than build.
3. **Mobile agent upload** — covered in Phase 3; my default is that it stays in scope.

I stop for none of these in the sense of waiting; each has a stated default and the spec moves.

## Phase 3 — The rejection

**What I'd refuse to carry forward: FR-008.** It fails twice over. It is not a requirement — it is an assumption ("agents always work from a desktop browser") wearing a requirement's clothes, and the house format has an Assumptions section precisely so this doesn't happen. And the assumption is false: 12% of agent workspace sessions run on viewports under 768 px, rising to 17% at weekends, and the browser support doc commits us to Safari and Chrome on iOS and Android. Shipping an agent upload path that assumes desktop would break roughly one session in eight, concentrated at weekends when cover is thinnest.

I'd delete FR-008, replace it with a requirement that upload works on phone-sized viewports in both surfaces, and note the correction explicitly in a revision note so Maya sees it was a deliberate reversal and not an editing slip. If she wants to descope mobile agent upload, that is a legitimate call — but it goes in Out of Scope with the 12% figure next to it, so design and engineering see the cost.

## Phase 4 — Rewrite the User Stories

Keep all three stories, their roles, benefits and priorities. Repair the scenarios:

- **US-001:** keep both good scenarios. Replace the "user-friendly error message" scenario with concrete ones per failure mode — file over the cap (rejected before upload begins, message names the limit), disallowed type (message names the accepted types), malware scan failure (quarantine notice in place of the file), upload interrupted (file preserved in the composer, retry available without re-selecting). Add a scenario for removing an attached file before submitting, which the draft never mentions.
- **US-002:** keep the first scenario. Replace "the experience is seamless" — untestable — with progress indication, a working cancel, and the reply composer staying editable during upload.
- **US-003:** currently images only, but FR-006 promises PDF preview too. I'd add the PDF scenario to the story so the requirement has a source, or drop PDF from FR-006 — and I'd default to **adding it**, since PDF is 12% of real volume and invoices were named in the Overview.
- **New story I'd add (P1):** a requester viewing and downloading an attachment in the portal. The draft lets requesters *send* files and lets agents *see* them, but never states that a requester can open a file an agent sent — yet US-002's whole benefit ("send the requester a document") depends on it. This is the largest actual gap in the draft.
- **New story I'd add (P3):** an admin or agent seeing a quarantined or dropped file's notice, so the security baseline's quarantine and drop behaviours have a user-facing home.

Every story keeps its independent test; the new ones get one.

## Phase 5 — Rewrite the Functional Requirements

Keep FR-001, FR-006, FR-011 essentially as they are. Then: quantify FR-002 against the reference connection and the 30 s cut-off; replace FR-003 with the per-file cap plus a per-message total; explode FR-004 into separate requirements for malware scanning before availability (with the under-5-s-for-95% figure), the content-type allowlist (png, jpg, gif, pdf, txt, log, csv, zip — matching email ingestion so the two paths can't diverge), encryption at rest and HTTPS-only delivery, signed URLs expiring one hour after issue and issued only after the ticket permission check, and audit-logging every download with who/when/which ticket; replace FR-005 with the per-failure-mode messages; delete FR-007 in favour of progress/cancel/non-blocking; delete FR-008 per Phase 3; replace FR-009 with the named browser matrix and an explicit statement that IE is not supported; replace FR-010 with a sustained-rate target derived from the observed 40/min peak with stated headroom.

New requirements the draft omits entirely: download (not merely preview) for every file type; behaviour when a signed URL expires while the ticket view is still open; the notice shown in place of a quarantined or dropped file; whether an attachment can be removed after the message is sent (my default: no, since the audit log and the email path both treat sent messages as immutable — recorded as an assumption).

Every requirement gets a `Source:` line, and I'd verify the mapping runs both ways: no requirement without a story, no story without a requirement.

## Phase 6 — Rewrite Success Criteria

All three current criteria are unmeasurable ("users are happy", "fewer", "a meaningful share"), and all three have a baseline sitting in the August analytics. Replace with criteria anchored to those baselines: portal-opened tickets carrying an attachment moving off 0% toward the 23% the email path achieves; the 38 monthly `attachments`-tagged support tickets falling, with specific attention to the 29 that were "how do I send a file from the portal" — that cohort should approach zero, since this feature exists to answer exactly that question; and an upload success rate plus a completion-time percentile measured against the reference connection. Each gets a baseline, a target and a measurement window, so "done" is a reading rather than an opinion.

## Phase 7 — Assumptions, Open Questions, Out of Scope

**Assumptions** goes from "None recorded" to a populated `A-XXX` list — the size cap and its derivation, the retention default, mobile agent upload staying in scope, sent attachments being immutable, the concurrency headroom multiple, and inheriting the email path's type allowlist. Two of the draft's Open Questions are decisions I take here rather than punt: attachment list ordering (newest first, matching how the conversation already reads — "nobody has an opinion" is a decision waiting for someone to make it) and filename truncation (a stated character count with the full name on hover). Neither needs a meeting.

**Open Questions** keeps only what genuinely needs someone else, each with a named owner: retention (Legal, asked August, unanswered, flagged as launch-blocking); and, if Maya rules against my default, the size-parity question for Support. "Maximum file size — engineering to decide" is removed, because it is now decided.

**Out of Scope** keeps both existing items and gains reasons, plus two new entries that the platform doc forces: chunked and resumable uploads (no path exists today), and any file above the cap (the 30 s cut-off makes it unbuildable without the above), noting the 31 rejections last month so nobody thinks this is unexamined.

## Phase 8 — Verification before handoff

No code here, so the check is a read-through against a checklist rather than a test run: every requirement carries a number, a threshold or an enumerated list and no bare adjective survives; every scenario can be marked pass or fail by someone who hasn't read the rest of the spec; every requirement traces to a story and every story to at least one requirement; sections appear in the house order with the right identifier prefixes; every Open Question names an owner; and every number in the spec traces back to `platform-limits`, `browser-support`, `security-baseline` or the August analytics — anything I can't source becomes an assumption instead. Header updated: status moved off "awaiting analyst revision", date to today, Maya kept as original author with my revision noted.

## What I'd report back

That the draft's structure was sound and was kept, and the revision was concentrated on nine unbuildable requirements. Specifically: I'd lead with the three things Maya needs to see rather than the full diff — **FR-008 was rejected as factually wrong** (12% of agent sessions are phone-sized, 17% at weekends), **the maximum file size was decided rather than deferred** at 25 MB with the 30 s-cut-off derivation shown and a pre-flight check added to contain the timeout risk, and **the spec was missing a requester's ability to open a file an agent sent**, which US-002 silently depended on. Then: retention remains genuinely open and is the one item that can block launch, Legal has been unanswered since August, and someone should escalate it now rather than at design review. Plus the tally — nine requirements quantified, four security clauses separated out, all three success criteria re-anchored to August baselines, two open questions closed as assumptions, two new stories added.