FILES-READ: `README.md`, `specs/README.md`, `specs/ticket-attachments/spec.md`, `docs/platform-limits.md`, `docs/browser-support.md`, `docs/security-baseline.md`, `analytics/attachments-2026-08.md`

---

# Action Plan — Revise `specs/ticket-attachments/spec.md`

## Phase 0 — Scope of the edit and what survives untouched

**What I would do:** Fix the file in place (`specs/ticket-attachments/spec.md`) rather than rewrite from scratch, because Maya's draft has real bones. Before touching anything I would mark the parts that stay:

- **Overview** — accurate, names the actors, states the gap (email is the only path today, portal has none), and is confirmed by the analytics file (0% of portal-opened tickets carry an attachment). Keep verbatim.
- **The three story statements themselves** (US-001, US-002, US-003) — correct role/capability/benefit, correct priorities, and each has a real independent test. Keep the sentences; only the scenario lists underneath change.
- **FR-001** — concrete, testable, correctly sourced. Keep.
- **FR-011** — the only requirement in the draft that is genuinely measurable, and it matches `docs/security-baseline.md` exactly (requester, assigned agents, admins). Keep, and extend rather than replace.
- **Both Out of Scope items** — internal-note attachments and in-product image editing. Keep with their reasons.
- **Header** — keep Maya as author; update status to reflect that it has been through analysis and add the revision date and my name as reviser (deepesh@humaninloop.dev).

**What I would read first:** All four reference docs, because every vague word in the draft has a hard number sitting in one of them. Specifically I would pull: the 30 s gateway cut-off and the absence of any chunked/resumable path, the 25 MB email limit, the signed-URL/one-hour/permission-check delivery model, the 10 Mbit/s + 50 ms reference connection, the supported browser list, the malware-scan and content-type allowlist, the audit-log rule, the no-retention-rule-today statement, and the whole August usage table.

**Delegation:** None. The workspace is seven small files and I have read all of them; there is no sweep left that a cheap worker could do more efficiently than I already have, and every one of these files is interpretive input to a decision rather than a lookup. Spawning anything here would cost more than it saves. I would make one exception at the end (Phase 7).

---

## Phase 1 — Resolve the file-size question, because everything else hangs off it

**What I would do:** The draft parks maximum file size in Open Questions as "engineering to decide." That is not decidable by engineering later — the platform already decides it, and the answer constrains three other requirements. I would work it out now:

- A browser upload is one HTTP request and dies at 30 s. There is no resumable path.
- At the reference connection (10 Mbit/s ≈ 1.25 MB/s), 25 MB takes about 20 s — inside the budget but with only ~10 s of margin. Anything slower than roughly 7 Mbit/s fails on a 25 MB file.
- Object storage tolerating 5 GB is irrelevant; the gateway is the binding constraint. I would say so explicitly in the spec so no one re-litigates it.
- The usage data says a cap of 25 MB covers the 99th percentile (18 MB) and every file the email path has ever accepted (largest 24.6 MB). A 10 MB cap would cover the 95th percentile (4 MB) with far more timeout headroom.
- 41% of portal sessions are on phone-sized viewports, and phones are the population most likely to be under 7 Mbit/s. That argues against the high cap.

**My default, written into the spec:** 25 MB per file, matching the email path so requesters see one consistent limit across channels; maximum 10 files per message; the client checks size before uploading and refuses oversize files without spending the network round trip.

**Stop / confirmation:** This is the one decision I would put in front of Maya and engineering before the spec goes to design, because it is user-facing and it has a cost tail. What I would ask them to rule on: *do we accept that a 25 MB upload will time out on connections under ~7 Mbit/s, or do we cap lower?*
- If they confirm 25 MB → the spec stands as drafted, and I add a requirement that the timeout error names the size and suggests emailing the file instead (the email path is more tolerant of slow links).
- If they cap at 10 MB → I change the number in one requirement and one assumption, adjust the oversize error message, and note in Out of Scope that files between 10–25 MB must go by email.
- If they want both, i.e. 25 MB reliably on slow links → that requires chunked or resumable upload, which does not exist. I would refuse to write it as a requirement of this spec and would instead add it to Out of Scope as a named platform prerequisite with its own ticket.

I would proceed with 25 MB while waiting, and mark the number as an assumption rather than a settled fact so it is visible if it changes.

---

## Phase 2 — Rewrite the Functional Requirements

**What I would write** into the Functional Requirements section, keeping the `FR-XXX` + `Source:` house convention. I would renumber cleanly rather than leave gaps, and keep FR-001 and the old FR-011 content intact at their new positions.

| Draft | What I do and why |
|---|---|
| FR-001 accept uploads | Keep as is. |
| FR-002 "uploads MUST be fast" | Replace. Two testable requirements: a file at or under the cap MUST complete within the 30 s gateway window on the reference connection; and the upload MUST show byte-level progress from the first second (see the "seamless" fix below). "Fast" disappears. |
| FR-003 "MUST support large files" | Replace with the explicit cap from Phase 1: per-file maximum, per-message file count, client-side pre-check, and a stated rejection behaviour. Also state the contradiction it hides — that "large" is impossible without resumable upload — in Out of Scope. |
| FR-004 "MUST be secure" | Delete and decompose into separate testable requirements, each traceable to the security baseline: malware scan before the file is downloadable, with quarantine and an in-ticket notice when it fails; content-type allowlist restricted to png, jpg, gif, pdf, txt, log, csv, zip with a named rejection message for anything else; delivery only via signed URLs that expire one hour after issue and are only issued after the ticket permission check; every download written to the audit log with who, when, and which ticket; encrypted at rest, HTTPS only. Five or six requirements where there was one adjective. |
| FR-005 "user-friendly error message" | Replace with one requirement per failure mode, each specifying what the message must tell the user: file too large (names the limit, as the email bounce already does), disallowed type (names the accepted types), malware quarantine, upload timeout, network drop mid-upload, and storage failure. Each must leave the composed message intact so nothing the user typed is lost — that is a real behaviour the draft never states. |
| FR-006 inline preview | Keep the intent, tighten the scope: inline rendering for png, jpg, gif; PDF opened in an in-page viewer; every other accepted type shown as a named, downloadable item with its size. Add the case the draft misses — a quarantined or still-scanning file shows a notice in place of the preview, never a broken image. |
| FR-007 "MUST feel seamless" | Delete the word. Replace with: per-file progress indication, the ability to cancel an in-flight upload, the ability to remove an attached file before sending, and the composer remaining editable while an upload runs. That is what "seamless" was gesturing at, and all four are pass/fail. |
| FR-008 "agents always work from desktop" | **Reject outright, and say why in the spec.** This is not a requirement, it is an assumption — and the analytics contradict it: 12% of agent workspace sessions are on phone-sized viewports, 17% at weekends. Excluding mobile from the agent workspace would break roughly one session in eight, concentrated at weekends when coverage is thinnest. I would replace it with a requirement that upload works across the full supported browser list on both desktop and mobile, and record the correction in Assumptions so Maya can see the claim was checked, not ignored. This is the single most consequential change in the revision, and I would call it out in my report. |
| FR-009 "all browsers" | Replace with the actual supported matrix: current and previous major of Chrome, Firefox, Safari, Edge on desktop; Safari and Chrome on iOS and Android. Internet Explorer explicitly not supported. "All browsers" is untestable and, per the browser doc, false. |
| FR-010 "many concurrent uploads" | Replace with a number. Today's email path runs ~600/day with an observed peak of 40/minute. Portal volume is unknown (the analytics file has no portal ticket count), so I cannot derive the new load — I would set the requirement at 120 uploads/minute sustained, three times the observed peak, and record both the number and the missing input as an explicit assumption naming who can close it. I would not present a derived-looking figure as if it were measured. |
| FR-011 visibility | Keep, and pair it with the audit-log and signed-URL requirements so the access model reads as one coherent block. |

**New requirements the draft is missing entirely** — these are the gaps that would bite in build:

- **Outbound email delivery.** US-002 says the requester "receives the reply with the file," which is ambiguous between a real email attachment and a portal link. This matters for size limits, for security (a link honours the permission check; an attachment does not), and for the mail infrastructure. **Stop:** I would flag this for Maya and engineering. My default is a permission-checked link in the notification email rather than a raw attachment, because it keeps every download inside the audit log. If they rule the other way, the size cap interacts with outbound mail limits and I would add a requirement bounding what gets attached versus linked.
- **Draft/orphan handling** — what happens to files uploaded into a compose box that is never submitted.
- **File name handling** — sanitisation, and behaviour when two files on one ticket share a name.
- **Attachment list ordering** — the draft leaves this open with "nobody has an opinion." It is user-facing but low-risk and reversible, so I take the default (newest first, matching conversation order) and record it as an assumption design can overturn, rather than leaving an open question that stalls the handoff.

---

## Phase 3 — Fix the story scenarios

**What I would write**, keeping each story's opening sentence, priority, and independent test:

- **US-001** — keep the two good scenarios (new ticket, reply). Replace the "user-friendly error message" scenario, which is untestable, with concrete ones: oversize file rejected before upload with the limit named; disallowed type rejected with the accepted types named; connection lost mid-upload with the typed message preserved; a file removed from the composer before submit. Add a mobile-browser scenario, since 41% of portal sessions are phone-sized and the draft never mentions it.
- **US-002** — keep the send scenario. Delete "the experience is seamless" outright; it cannot be verified. Replace with progress-visible and cancel-in-flight scenarios. Sharpen the delivery scenario once Phase 2's email question is settled. Add an agent-on-mobile scenario, which is the direct consequence of rejecting FR-008.
- **US-003** — keep the inline image scenario. Add the coverage the draft is missing: PDF preview (FR-006 promised it and no scenario tested it), a non-previewable type shown as a named download, and a quarantined file showing a notice instead of a preview.
- **New US-004 (P1) — download an attachment.** Every download is supposed to hit the audit log and go through a one-hour signed URL, and no story covers downloading at all. Without it the audit and expiry requirements have no story to trace to, which breaks the house format's `Source:` rule.

I would re-check that after this every `FR` traces to a story that actually exercises it, and that no story has a scenario with no requirement behind it.

---

## Phase 4 — Rewrite Success Criteria

All three are unmeasurable as written ("users are happy," "fewer," "a meaningful share"). Each has a baseline sitting in the analytics file, so each can be made concrete:

- **Adoption:** portal-opened tickets carrying at least one attachment, against a baseline of 0% today and the 23% the email path achieves — a target percentage within a stated window after launch.
- **Deflection:** support tickets tagged `attachments`, baseline 38 in August of which 29 were "how do I send a file from the portal" — target reduction of that specific class within a stated window. This replaces "fewer support tickets" with the exact query that measures it.
- **Reliability:** upload success rate excluding user-cancelled uploads, plus a 95th-percentile completion time for a file at the median observed size (1.2 MB).
- **Preview effectiveness:** share of image attachments viewed without a download, which is the actual benefit US-003 claims.

I would put the baseline next to every target so the criterion is checkable on day one rather than being an argument later. **Stop:** the target numbers themselves are Maya's call, not mine — I would propose figures and mark them as proposed pending her confirmation. If she wants different targets, only the numbers change; the measurement definitions stand.

---

## Phase 5 — Fill Assumptions and rewrite Open Questions

**Assumptions** currently says "None recorded," which is the clearest signal in the draft that the ambiguity was passed downstream rather than resolved. I would write out every decision I took, in the house `A-XXX` form: the file-size cap and the reasoning from the gateway timeout; the per-message file count; the concurrency multiplier and the missing portal-volume input behind it; retention defaulting to current behaviour; newest-first ordering; the correction of the desktop-only claim with the session data that contradicts it; the assumption that scanning stays under the observed 5 s for 95% of files; and that the existing content-type allowlist carries over to the portal unchanged rather than being widened.

**Open Questions** — the house format requires an owner on each, and the draft has none. I would:

- **Close** "maximum file size — engineering to decide": resolved in Phase 1, moved to Assumptions with a confirmation flag. It was never engineering's to decide alone.
- **Keep, with teeth,** retention: owner Legal, asked in August, unanswered. I would state the default the product ships with if no answer arrives (current behaviour — kept as long as the ticket exists, closed tickets never purged), name the cost exposure since storage is billed per GB, and note that launching without a rule is a deliberate choice, not an oversight. It does not block design.
- **Remove** the file-name truncation question — that is a visual design decision, not a spec decision, and it should be handed to design rather than held in the spec.
- **Remove** the sort-order question — decided in Phase 2 as a reversible default.
- **Add** the two real open questions: whether outbound email carries the file or a link, and confirmation of the size cap trade-off.

Net effect: four ownerless questions become two owned, decision-shaped ones, and the rest are resolved.

---

## Phase 6 — Out of Scope

Keep both existing entries. Add, each with its reason: chunked and resumable upload (does not exist on the platform and is the reason the size cap is what it is); widening the content-type allowlist beyond the eight types email already accepts; re-scanning files already ingested by email; and any retention or purge job, which is blocked on Legal.

---

## Phase 7 — Consistency check before handoff

**What I would do:** Re-read the finished file end to end myself, since coherence between the story scenarios and the requirements is exactly the judgement that cannot be delegated.

**The one delegation I would make:** a throwaway `Explore` subagent, model `haiku`, given a purely mechanical brief — read `specs/ticket-attachments/spec.md` and `specs/README.md` and report back three lists: every `US-`/`FR-`/`SC-`/`A-` identifier in order, any duplicate or skipped number, and any `FR-` with no `Source:` line. On its return I would check the identifier lists against my own edit and confirm the section order matches the eight sections the house format requires. That is a deterministic scan with no interpretation in it, which is why it is safe to hand off; the substance stays with me.

**What I would refuse throughout:** to leave any word like "fast," "secure," "seamless," "user-friendly," or "many" in the file; to invent a retention period on Legal's behalf; to state the concurrency target as if it were derived from portal data I do not have; and to keep FR-008's desktop-only claim in any form, since the workspace's own numbers contradict it.

---

## What I would report back

A short note to Maya covering:

1. **The one thing that would have shipped wrong:** FR-008 excluded mobile from the agent workspace on the basis that agents always use desktop. 12% of agent sessions are phone-sized, 17% at weekends. That requirement is gone and replaced with the real browser matrix.
2. **Two decisions I made that she should sanity-check:** the 25 MB cap with its slow-connection trade-off spelled out, and the concurrency target set at 3× the observed email peak because portal volume is unmeasured.
3. **Two questions still open with named owners:** retention (Legal, chased since August, with the shipping default stated) and whether reply notifications carry the file or a link.
4. **The bookkeeping:** seven vague requirements quantified, "secure" split into six testable ones, an untestable scenario removed from US-002, a download story added so the audit and expiry rules have something to trace to, three success criteria given baselines from the August data, and the empty Assumptions section filled with eight recorded decisions.
5. **My read on readiness:** the spec is buildable and can go to design now; the two open questions affect requirement text but not the shape of the design work, and neither should hold the handoff.