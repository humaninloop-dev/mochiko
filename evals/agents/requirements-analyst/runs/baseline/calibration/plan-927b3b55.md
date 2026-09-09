FILES-READ: `README.md`, `specs/README.md`, `specs/ticket-attachments/spec.md`, `docs/platform-limits.md`, `docs/browser-support.md`, `docs/security-baseline.md`, `analytics/attachments-2026-08.md`

---

# Action Plan — Revise `specs/ticket-attachments/spec.md`

## Phase 1 — Ground the revision in the workspace facts (done during intake)

Read, and what each gives me for the revision:

- `README.md` — actors are requester, agent, admin; the two surfaces are the web portal and the agent workspace. Confirms the draft's cast is right and complete.
- `specs/README.md` — the eight required sections and their order, the ID prefixes (`US-`, `FR-` with a `Source:` line, `SC-`, `A-`), that Assumptions must carry IDs, and that every open question must name who owns the answer. The draft violates three of these.
- `docs/platform-limits.md` — the binding numbers: 30 s hard cutoff on any single HTTP request with no chunked/resumable path; 5 GB object ceiling (not binding, the gateway is); 25 MB total on the existing inbound-email attachment path; CDN delivery via signed URLs that expire in one hour and are only issued after a ticket permission check; the 10 Mbit/s up / 50 ms reference connection every product number is quoted against.
- `docs/security-baseline.md` — malware scan before availability (under 5 s for 95% of files), quarantine plus an in-ticket notice, the accepted type list (png, jpg, gif, pdf, txt, log, csv, zip), access follows the ticket (requester, assigned agents, admins), every download audit-logged, encrypted at rest, HTTPS only, and that no retention rule exists today.
- `docs/browser-support.md` — the actual support matrix: current and previous major of Chrome, Firefox, Safari, Edge on desktop; Safari and Chrome on iOS and Android; IE excluded since 2024.
- `analytics/attachments-2026-08.md` — the baselines I need to make the success criteria measurable and to size the requirements: 600 attachments/day, peak 40/minute; median 1.2 MB, p95 4 MB, p99 18 MB, largest accepted 24.6 MB, 31 over-limit rejections in a month; type mix (png 61%, jpg 14%, pdf 12%, txt/log 9%, csv+zip 4%); 23% of email-opened tickets carry an attachment versus 0% of portal-opened tickets; **12% of agent workspace sessions on phone-sized viewports, 17% at weekends**, and 41% of portal sessions; 38 support tickets tagged `attachments` in August, 29 of them "how do I send a file from the portal".

I would then load `mochiko:authoring-user-stories` and `mochiko:authoring-requirements` for the exact story, scenario, and requirement formats before writing a line, rather than working from memory. *(Not loaded in this planning run.)*

**Delegation decision:** none. The whole workspace is seven short files and I have read all of them; handing a sweep to a cheap subagent here would cost more than it saves, and the judgement calls (which vague requirement maps to which platform fact) are exactly the interpretive kind I keep. I would make **one** delegation, at the end — see Phase 8.

## Phase 2 — Defect inventory against the draft (no writing yet)

I build the list of what changes and what survives untouched, so the PM can see I kept her work.

**Keep as-is:** the Overview (it states the problem, the two surfaces, and the current email-only workaround accurately); US-001 and US-002 statements and their P1 priority; US-003 statement and P2; FR-001; FR-011 (it matches the security baseline exactly); both Out of Scope items and their reasons; the section order.

**Defects to fix:**

| Draft item | Problem | Resolution source |
|---|---|---|
| FR-002 "MUST be fast" | No number | Quote against the 10 Mbit/s reference profile and the 30 s cutoff |
| FR-003 "MUST support large files" | No number, and overlaps FR-002 | Replaced by an explicit size cap |
| FR-004 "MUST be secure" | Untestable; hides five separate obligations | Split using the security baseline |
| FR-005 "user-friendly error message" | Untestable | Replaced by an enumerated set of failure cases, each naming what the message must state |
| FR-007 "MUST feel seamless" | Untestable | Replaced by concrete progress/cancel/blocking behaviour |
| FR-008 | Not a requirement at all — it is an assumption, and the analytics contradict it | **Stop; see Phase 5** |
| FR-009 "all browsers" | Unbounded and false | Replaced with the documented matrix |
| FR-010 "many concurrent uploads" | No number | Derived from the observed 40/min peak |
| SC-001, SC-002, SC-003 | None measurable — "happy", "fewer", "meaningful share" | Rewritten against the August baselines |
| Assumptions: "None recorded" | Violates the house format and is not credible for a draft with four open questions | Populate with `A-XXX` |
| Open Questions | None names an owner; two are mine to decide, one is engineering's to be told not asked | Rewritten with owners |
| US-001 scenario 3, US-002 scenario 2 | Assert "user-friendly" / "seamless" — cannot be graded pass/fail | Rewritten as observable outcomes |

**Gaps the draft does not mention at all**, which I would add: how many files per message and the per-message total; removing an attached file before sending and whether an attachment can be deleted after; what happens between submit and the malware scan finishing; what the requester actually receives in the notification email (this one matters — signed URLs expire in an hour, so mailing one is a broken experience); what a non-previewable type (txt, log, csv, zip) shows in the ticket view; whether inline preview exists in the portal or only the agent workspace; duplicate and long file names.

## Phase 3 — Rewrite the User Stories

Rewrite in place at `specs/ticket-attachments/spec.md`, keeping the three existing stories' IDs, wording, and priorities where they hold.

- **US-001** — keep the story sentence. Replace the third scenario with separate, gradable failure scenarios: over the size limit (the message names the limit and the offending file), a disallowed type (the message names the accepted types), and a network or timeout failure (the message offers retry and the composed ticket text is preserved). Add a scenario for removing a file before submitting. Keep the independent test.
- **US-002** — keep the story sentence. Delete the "seamless" scenario; replace with: while the upload runs, per-file progress is shown and the Send control is unavailable until every file has finished or been removed. Add the scan-pending case. Keep the independent test.
- **US-003** — keep the story sentence and P2. Add scenarios for a PDF, for a non-previewable type (shown as a named download row, not a broken preview), for a quarantined file (the notice appears in its place), and extend the story to the portal so the requester sees the same inline rendering — or, if the PM wants that deferred, it moves to Out of Scope with a reason. My default: include it; it is the same view component and excluding it would ship a feature only half the audience can use.
- **New US-004 (P3)** — as an agent or requester, remove an attachment I added by mistake, so a wrong or sensitive file is not left on the ticket. Flagged as a candidate for Out of Scope if the PM wants a tighter first cut; I would rather it be a stated cut than an unstated one.

## Phase 4 — Rewrite the Functional Requirements

Keeping `FR-001`, `FR-006`, and `FR-011` (FR-006 gets its type list made explicit; FR-011 gains the download audit-log obligation from the baseline). Every requirement keeps a `Source:` line. The substance I would write:

- **Size.** Total attachments on one ticket or one reply must not exceed 25 MB, matching the existing inbound-email path so requesters do not learn two different limits. This is the number I choose; it is not engineering's to pick, because it is a user-facing product boundary — engineering's constraint (the 30 s gateway cutoff) sets the ceiling, and 25 MB sits under it. Arithmetic I would put in the spec: 25 MB over the 10 Mbit/s reference connection is roughly 20 s of the 30 s budget. I would flag in the spec text and in my report that this leaves thin headroom on slower connections, and that raising the cap requires chunked or resumable upload, which does not exist — so anything above 25 MB is explicitly Out of Scope for this feature.
- **Speed**, quoted against the reference connection: a median file (1.2 MB) completes within 3 s; a p95 file (4 MB) within 6 s; a maximum-size file completes before the 30 s cutoff. Replaces "fast".
- **Types.** Only png, jpg, gif, pdf, txt, log, csv, zip are accepted, matching the ingestion allow-list; anything else is rejected at selection time with the accepted list named. Rejecting in the browser before upload, not after 20 s of transfer.
- **Security**, split into separate requirements: scan before the file is made available; quarantine plus in-ticket notice on failure; delivery only via signed URLs issued after the ticket permission check and expiring in one hour; HTTPS only and encrypted at rest; every download audit-logged with who, when, and from which ticket.
- **Scan window.** Because the scan takes under 5 s for 95% of files, submit must not block on it: the ticket or reply posts immediately and the attachment shows a pending state until the scan clears. This is a real design decision, stated as such.
- **Failure messages**, enumerated one per case, each specifying what the message must tell the user and that composed text survives the failure. Replaces "user-friendly".
- **Progress and control**: per-file progress, cancel a file in flight, remove a file before submit, submit blocked while any upload is in flight. Replaces "seamless".
- **Files per message**: at most 10.
- **Browsers**: the exact matrix from `docs/browser-support.md`, including mobile Safari and Chrome, and IE named as unsupported.
- **Concurrency**: sustain 120 uploads per minute system-wide while still meeting the speed figures — three times the observed 40/min email peak, chosen because two new upload surfaces are being added to a path that today carries all the volume. The multiplier is recorded as an assumption, not smuggled in as fact.
- **Outbound notification**: the email notifying a requester of an agent reply links to the ticket in the portal; it must not embed a signed file URL, because those expire in an hour and a stale link in an inbox is a support ticket waiting to happen. Attaching the file to the outbound email itself is out of scope for this cut.

## Phase 5 — The one thing I stop on

**FR-008 is both misfiled and factually contradicted.** It claims agents always work from a desktop browser, which is (a) an assumption written into the requirements section, and (b) untrue by the workspace's own numbers: 12% of agent workspace sessions run on phone-sized viewports, 17% at weekends. It also conflicts with the browser support doc, which commits Deskline to mobile Safari and Chrome.

I would not silently overrule the PM on a scope cut, so this is the stop: **confirm with Maya whether excluding mobile upload from the agent workspace is a deliberate cut she is willing to own, given that it strands roughly one in eight agent sessions and one in six at weekends.**

- If she says it was an assumption she did not know was wrong → mobile agent upload is in scope; the requirement is deleted and the browser requirement covers it. **This is my default and how I would write the file.**
- If she says it is a deliberate cut → it leaves the requirements section entirely and moves to Out of Scope with the 12%/17% figures written next to it, plus an assumption recording that agents on phones fall back to email, and an open question on whether the agent workspace should say so rather than silently omitting the control.
- If she wants it deferred rather than cut → it becomes a P3 story explicitly marked as a later cut.

Two lesser stops, which I would raise in the same message but not block on: the 25 MB cap (engineering may object to the 20-of-30-second margin — if they do, the cap drops to 20 MB and the arithmetic in the spec changes with it), and whether inline preview ships to the portal as well as the agent workspace.

## Phase 6 — Success Criteria

Every one anchored to an August baseline so it is measurable on the same report:

- Portal-opened tickets carrying at least one attachment rises from 0% to at least 15% within 90 days of launch (the email path sits at 23%; I discount for the portal's different mix and record that discount as an assumption).
- Support tickets tagged `attachments` fall from 38/month, and specifically the "how do I send a file from the portal" subset falls from 29/month to fewer than 5, within 60 days.
- At least 98% of upload attempts that pass client-side size and type validation complete successfully.
- At least 95% of uploads meet the stated time figures on the reference connection.
- Zero files served to a principal outside the requester / assigned agents / admins set, verified from the audit log.

I would delete "users are happy with attachments" outright rather than try to salvage it; if the PM wants a satisfaction measure, it needs a named survey instrument and a baseline, which is an open question for her.

## Phase 7 — Assumptions and Open Questions

**Assumptions** (`A-001`…), replacing "None recorded" — every default I took above gets written down: the 25 MB cap and its parity rationale; 10 files per message; the 3× concurrency headroom; posting before the scan completes; portal notification links rather than embedded signed URLs; inline preview in both surfaces; newest-first ordering for the attachment list; retention defaulting to the existing behaviour (files live as long as the ticket, closed tickets never purged) until Legal rules otherwise.

**Open Questions**, rewritten with owners as the house format demands:

- Retention after ticket close — **owner: Legal**, asked in August, unanswered. I keep this one and note it does not block build, because the default is simply today's behaviour; it does affect storage cost, since the platform bills per GB stored and served. Flagged to the PM as the item to chase before this becomes expensive.
- Maximum file size — **removed as a question.** It is not engineering's to decide; it is a product boundary constrained by the gateway, and I have set it. It appears as an assumption instead.
- Attachment list ordering — **removed as a question.** Nobody having an opinion is not a reason to leave it open for design; newest first, recorded as an assumption, overridable by design without a spec change.
- File name truncation length — **removed as a question**, restated as a requirement that the full name must be recoverable (hover or the download row) and the truncation point left to design.
- Newly opened, **owner: PM** — is there a satisfaction measure behind the deleted SC-001, and what instrument?
- Newly opened, **owner: PM/Design** — the FR-008 mobile ruling from Phase 5, if it is still unresolved when I hand off.

## Phase 8 — Write, check, hand off

**Write:** `specs/ticket-attachments/spec.md` (revised in place — one file, no new ones). Header updated: status to "Revised, ready for design", date 10 September 2026, author line keeping Maya as the PM author and naming me as reviser.

**Verification, and the one delegation I would make:** after writing, spawn a disposable `Explore` subagent with `model: haiku` — this is a deterministic checking sweep, exactly the kind of read that should not cost me context or a premium tier. Brief: read the revised spec and `specs/README.md`; report (1) whether the eight sections appear in the documented order, (2) every `US-`, `FR-`, `SC-`, `A-` ID in file order with any gap or duplicate, (3) every `Source:` reference that names a story ID not present in the file, and (4) any requirement line lacking a `Source:` line. On its return I check the ID sequences by eye and re-read any line it flags myself — I do not take a dangling-reference claim on faith, and if it reports "no issues" on a file I know grew by a dozen requirements, I verify a sample before believing it.

**Refusals, stated plainly in the handoff:** I will not write "fast", "secure", "seamless", or "user-friendly" back into the spec in any form; I will not leave Assumptions empty; and I will not carry FR-008 forward as a requirement, because it is an assumption, and one the workspace's own numbers contradict.

**What I report at hand-off:** the old-ID-to-new-ID mapping so anyone holding the draft can follow what happened to each line (that mapping lives in my message, not the file, since the house format fixes the sections); the list of what I kept untouched; the FR-008 stop with the 12%/17% figures and the ruling I need; the 25 MB decision with the 20-seconds-of-30 arithmetic and the note that raising it needs resumable upload we do not have; the Legal retention item as the only genuine outstanding blocker and its cost implication; and the four draft questions I closed by decision rather than escalation, with the default I took on each.