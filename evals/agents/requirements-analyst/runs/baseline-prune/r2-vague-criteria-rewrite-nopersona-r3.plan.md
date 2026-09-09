FILES-READ: `README.md`, `specs/README.md`, `specs/ticket-attachments/spec.md`, `docs/platform-limits.md`, `docs/browser-support.md`, `docs/security-baseline.md`, `analytics/attachments-2026-08.md`

# Action plan — revise `specs/ticket-attachments/spec.md`

The draft's structure, its three stories, and its problem framing are sound and I'd keep them. The defects are concentrated in the Functional Requirements (seven of eleven are untestable, and two contradict facts already written down in this repo), in Success Criteria (all three unmeasurable), in an empty Assumptions section, and in Open Questions that hold decisions I can make myself from `docs/` and `analytics/`. The revision is a single in-place rewrite of that one file.

---

## Phase 1 — Establish the fact base and grade every line of the draft against it

**Do:** Re-read the four reference files as the authority for anything the spec asserts about limits, browsers, or security, and mark each existing requirement as keep / sharpen / contradicted / missing. No writing yet.

**Read:** `docs/platform-limits.md`, `docs/browser-support.md`, `docs/security-baseline.md`, `analytics/attachments-2026-08.md`, `specs/README.md` (section order and the `A-XXX` / `Source:` conventions the house format requires).

Findings I'd carry into the rewrite:

- **Binding upload constraint is the 30 s API gateway cutoff, not storage.** Object storage allows 5 GB per object, which is irrelevant here. At the reference connection (10 Mbit/s up = 1.25 MB/s), 30 s buys ~37 MB at perfect line rate. There is no chunked or resumable path.
- **The email path already enforces 25 MB total per message,** with an allowlist of png, jpg, gif, pdf, txt, log, csv, zip, malware scanning before availability, quarantine-with-notice, encryption at rest, HTTPS-only delivery, signed CDN URLs expiring in 1 hour issued only after the ticket permission check, and a download audit log.
- **Analytics contradicts FR-008.** The draft asserts agents always work from a desktop browser; 12% of agent workspace sessions are on phone-sized viewports, 17% at weekends. Portal is 41%.
- **`docs/browser-support.md` contradicts FR-009.** "All browsers" is not the policy; the policy is current + previous major of Chrome/Firefox/Safari/Edge on desktop plus Safari and Chrome on iOS and Android, IE excluded since 2024.
- **Analytics gives real sizing:** median 1.2 MB, p95 4 MB, p99 18 MB, largest accepted 24.6 MB, 31 rejections in August (largest attempt 148 MB); 600 attachments/day, observed peak 40/min; 23% of email-opened tickets carry an attachment vs 0% of portal-opened tickets; 38 support tickets tagged `attachments`, 29 of them "how do I send a file from the portal."

---

## Phase 2 — Close the open questions I'm entitled to close, and record them as assumptions

The house format has an Assumptions section precisely for this, and the draft's says "None recorded." Every decision below moves out of Open Questions into `Assumptions` with its reasoning, so engineering isn't blocked and the PM can see and overturn each one.

1. **Max file size → 25 MB per file, 25 MB total per message, 10 files per message.** Rationale written into the spec: parity with the existing email path so a ticket can never carry a file one entry point allows and the other rejects; covers p99 (18 MB) and the largest file the email path has ever accepted (24.6 MB); and 25 MB at the reference connection is ~20 s, inside the 30 s gateway cutoff with about a third of headroom. I would **flag in the spec** that a 25 MB upload on a connection slower than the reference profile will hit the gateway timeout, which is why the timeout error message is its own requirement rather than folded into a generic failure state.
2. **Attachment list ordering → chronological, oldest first, matching the conversation the files arrived on;** a ticket-level list, if design wants one, is newest first. The draft says nobody has an opinion, which makes it mine to decide rather than a blocker.
3. **Filename truncation → not an engineering blocker.** Record as an assumption (truncate in the middle, preserve the extension, full name available on hover/long-press) and hand the pixel decision to design, which is where this file is going next.
4. **Retention stays an open question** — Legal owns it and has been silent since August. But I would attach a default so the build can proceed: behave as email-ingested files do today (kept for the life of the ticket, closed tickets never purged), with a note that `docs/platform-limits.md` says we pay per GB stored and per GB served, so an unbounded default has a cost consequence Legal's answer should price in.

**Stop point (non-blocking):** I would not wait on Legal. If Legal answers before design, retention becomes a functional requirement; if not, the default above ships and the question stays open with a named owner and the date it was raised.

---

## Phase 3 — Rewrite the Functional Requirements

**Write to:** `specs/ticket-attachments/spec.md` (in place; no new files, and I would not edit anything under `docs/` — those are inputs I cite, not mine to change).

Each requirement becomes something a QA engineer can pass or fail, keeping the `Source:` lines the house format requires. Concretely:

- **Keep as-is:** FR-001 (both entry points, create and reply) and FR-011 (visibility limited to requester, assigned agents, admins) — FR-011 already matches the security baseline; I'd only add the download audit-log obligation to it.
- **FR-002 "Uploads MUST be fast"** → a 4 MB file (the p95) completes in under 5 s on the reference connection; the client shows determinate progress within 500 ms of selection.
- **FR-003 "large files"** → the 25 MB / 25 MB / 10-file caps from Phase 2, with client-side rejection before any bytes are sent, naming the limit and the actual file size.
- **FR-004 "MUST be secure"** → splits into five separately testable requirements: malware scan before the file is downloadable, with quarantine and an in-ticket notice; the same content-type allowlist the email path uses, enforced by sniffed type and not by extension alone; encryption at rest and HTTPS-only delivery; signed CDN URLs expiring one hour after issue and issued only after the ticket permission check; every download audit-logged with who, when, and which ticket.
- **FR-005 error message** → enumerate the cases and what each says: over size, disallowed type, malware quarantine, gateway timeout (naming slow connection as the cause and offering retry), network drop mid-upload, and storage failure. Partial failure in a multi-file selection must not discard the other files or the typed message.
- **FR-006 preview** → reconcile with US-003, which only mentions images. I'd keep PDF but make the tiers explicit: png/jpg/gif render inline; PDF opens in an in-page viewer; txt/log/csv/zip are download-only with type and size shown. Preview types are a subset of the allowlist by construction.
- **FR-007 "seamless"** → per-file progress, cancel an in-flight upload, retry a failed one, remove a file before sending, and the send control disabled until uploads settle.
- **FR-008** → see Phase 4; this is the one I won't resolve silently.
- **FR-009 "all browsers"** → the exact matrix from `docs/browser-support.md`, with IE named as unsupported so nobody re-litigates it.
- **FR-010 "many concurrent uploads"** → a stated target: sustain 3× the observed email-path peak (120 attachments/min) with no degradation of the p95 upload time, on the reasoning that portal upload is new demand on top of the existing 600/day rather than a redistribution of it; plus a per-user cap of 5 simultaneous in-flight uploads.
- **New requirements the draft omits entirely:** what the requester actually receives when an agent attaches a file to a reply (see the Phase 4 stop point); parity of the allowlist and caps across the portal, agent workspace, and email paths; duplicate and non-ASCII filenames; and whether an attachment can be removed after send (default: no, it is part of the conversation record, consistent with the audit-log posture).

---

## Phase 4 — The two things I would stop and confirm

**Stop 1 — FR-008, mobile in the agent workspace.** The draft's justification ("agents always work from a desktop browser") is contradicted by `analytics/attachments-2026-08.md` and by the supported-browser list, which includes iOS and Android. I would not delete a PM's scope decision on my own reading of the data.

- *What I'd confirm with Maya:* whether the desktop-only claim was a considered tradeoff or an untested assumption, now that 12% of agent sessions (17% weekends) are phone-sized.
- *If she keeps desktop-only:* it moves out of Functional Requirements into Out of Scope, worded as a deliberate exclusion with the 12%/17% figures stated in-line, so the cost is on the record where design and engineering will see it.
- *If mobile stays in:* FR-008 inverts into a positive requirement that the composer works on Safari and Chrome for iOS and Android, including the camera/photo-library picker.
- *My default while unanswered:* mobile stays in scope, recorded as an assumption with the numbers, because the supported-browser policy already commits us to those browsers and removing them needs a decision rather than an inherited sentence.

**Stop 2 — how an agent's attachment reaches the requester.** US-002 says "the requester receives the reply with the file," which could mean an email attachment or a signed link, and the two have very different engineering shapes: signed URLs expire in one hour, which makes a link in an email that someone opens the next morning a dead link. The draft never asks this.

- *What I'd confirm:* PM plus whoever owns email infrastructure.
- *Branches:* attach-to-email inherits the 25 MB ceiling and needs a rule for what happens when the reply exceeds it; link-in-email needs a link that outlives one hour or a portal round-trip through authentication.
- *My default:* record it as an open question with a named owner and a stated interim assumption (attach files under the cap directly, link above it), because the requester-facing half of US-002 can't be built without an answer and I'd rather engineering see the fork than guess at it.

Neither stop blocks the rest of the revision; I'd complete every other section and hand over the file with these two marked.

---

## Phase 5 — Rewrite Success Criteria against the August baselines

All three current criteria are unmeasurable ("users are happy," "fewer," "meaningful share"). Replaced with figures anchored to numbers already in `analytics/attachments-2026-08.md`:

- Share of portal-opened tickets carrying at least one attachment reaches 15% within 90 days of launch (baseline 0%; the email path runs at 23%).
- Support tickets tagged `attachments` fall from 38/month to 10 or fewer within two months, with the "how do I send a file from the portal" subset (29 of 38 in August) at zero.
- Upload success rate of 99% or better, excluding files the user was told upfront were too large or the wrong type.
- p95 upload time under 5 s for a 4 MB file on the reference connection.
- Malware quarantine rate and inline-preview open rate reported as observed values, not targets, since there's no baseline to set a target against.

---

## Phase 6 — Sharpen the stories, then tidy the header and remaining sections

- **US-001:** replace "user-friendly error message" in the acceptance criterion with a specific case, and add a scenario for a file rejected before upload for size or type.
- **US-002:** replace "the experience is seamless" — it isn't a testable Given/When/Then — with progress, cancel, and retry behaviour, and add the quarantine case.
- **US-003:** add a scenario for a non-previewable type and one for a quarantined file, so the preview surface is fully specified.
- **Independent tests:** make each one executable with real numbers ("open a portal ticket with a 4 MB PNG attached; the agent sees it inline within one page load and the upload completed in under 5 s") rather than "confirm the agent sees it."
- **Out of Scope:** keep both existing entries; add chunked/resumable upload and any per-file limit above 25 MB, since that's the dependency implied by the size decision, plus anything ruled out by Stop 1.
- **Header:** status to "Revised — ready for design," author Maya (PM) preserved, reviser and date 9 September 2026 added.
- Verify the final file carries the eight house-format sections in the prescribed order, every FR has a `Source:` line, and every assumption is numbered `A-XXX`.

---

## Delegation, and what I would not do

**No delegation.** This is one file, roughly 100 lines, with four short reference documents; splitting it across workers would cost more in reconciliation than it saves, and the whole value of the revision is one person holding the contradictions in view at once. I would not spawn any subagent or run any review agent for it.

**No writes outside `specs/ticket-attachments/spec.md`.** In particular I would not "fix" `docs/browser-support.md` or the security baseline to match the draft — the draft is what's wrong.

**I would not silently discard Maya's FR-008,** and I would not invent a retention rule that Legal hasn't ruled on; both go through Phase 4's stop points or stay open with an owner.

## What I'd report at the end

A short summary naming: the four vague requirements now quantified and the numbers chosen; the two requirements that contradicted the repo's own reference docs and how each was resolved; the three open questions I closed and the assumptions that now carry them (25 MB, ordering, filename truncation); the two questions I escalated rather than decided, with owners; the retention default I put in place so the build isn't blocked on Legal; the rewritten success criteria with their August baselines; and the one flagged risk — that 25 MB is roughly 20 seconds against a hard 30-second gateway cutoff with no resumable path, so uploads near the ceiling on slow connections will fail, which is why the timeout error message is a requirement in its own right.