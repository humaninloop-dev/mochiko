FILES-READ:
- `README.md`
- `specs/README.md`
- `specs/ticket-attachments/spec.md`
- `docs/platform-limits.md`
- `docs/browser-support.md`
- `docs/security-baseline.md`
- `analytics/attachments-2026-08.md`

---

# Revision plan — `specs/ticket-attachments/spec.md`

The draft's structure and its user stories are sound and I'd keep them. The problem is that most of the Functional Requirements and all three Success Criteria are unbuildable and untestable ("fast", "secure", "seamless", "many", "all browsers", "users are happy"), while the four workspace reference docs contain the exact numbers needed to replace them. One requirement (FR-008) contradicts the analytics outright. Assumptions is empty despite the draft resting on several, and the Open Questions have no owners, which the house format requires.

## Phase 1 — Establish the constraint set I'll write against

Already done via the reads above. The facts I'd hold the revision to, each traceable to a file:

| Fact | Source |
|---|---|
| Single HTTP request cut off at 30 s; no chunked/resumable path | `docs/platform-limits.md` |
| Reference connection 10 Mbit/s up ≈ 1.25 MB/s, 50 ms latency | `docs/platform-limits.md` |
| Email ingestion caps total attachments at 25 MB, bounces with the limit named | `docs/platform-limits.md` |
| Signed CDN URLs, 1 h expiry, issued only after ticket permission check | `docs/platform-limits.md` |
| Object storage 5 GB/object; billed per GB stored **and** served | `docs/platform-limits.md` |
| Malware scan before availability; quarantine + notice; <5 s for 95% | `docs/security-baseline.md` |
| Allowed types: png, jpg, gif, pdf, txt, log, csv, zip | `docs/security-baseline.md` |
| Access = requester + assigned agents + admins; every download audit-logged | `docs/security-baseline.md` |
| No retention rule; closed tickets never purged | `docs/security-baseline.md` |
| Current + previous major of Chrome/Firefox/Safari/Edge desktop; Safari + Chrome on iOS/Android; no IE | `docs/browser-support.md` |
| 600 attachments/day, peak 40/min; median 1.2 MB, p95 4 MB, p99 18 MB, largest accepted 24.6 MB | `analytics/attachments-2026-08.md` |
| 12% of **agent workspace** sessions are phone-sized (17% weekends); 41% of portal sessions | `analytics/attachments-2026-08.md` |
| 38 `attachments` support tickets in August, 29 of them "how do I send a file from the portal" | `analytics/attachments-2026-08.md` |
| 23% of email-opened tickets carry an attachment; 0% of portal-opened | `analytics/attachments-2026-08.md` |

Key derivation I'd do here and carry into the spec: at 1.25 MB/s, a 25 MB file takes **20 s**, against a hard 30 s cutoff — about 33% headroom before TLS and request overhead. That single number decides the file-size limit, and it means the malware scan cannot sit inside the upload request (20 s + up to 5 s scan leaves almost nothing). So "scan asynchronously, show a scanning state" becomes a derived requirement, not a nicety.

## Phase 2 — Decisions I'd take, and the three I'd flag

I'd resolve what I can resolve and mark the rest, rather than shipping a spec that punts to design. Three items need a human ruling; I'd state my default, write the spec on that default so it is complete either way, and put the question where Maya will see it.

**Stop 1 — FR-008 is factually wrong.** It reads: *"Agents always work from a desktop browser, so uploading from a mobile device is not required for the agent workspace."* Analytics says 12% of agent workspace sessions are on phone-sized viewports, 17% at weekends, and `browser-support.md` already commits Deskline to mobile Safari and Chrome. This is a scope reduction resting on a false premise, so I won't silently delete it — removing it enlarges the build.
- *Confirm with Maya:* mobile agent upload is in scope.
- *If she agrees (my default):* FR-008 is rewritten as a positive requirement that upload works on the supported mobile browsers, citing the 12%/17% figures.
- *If she wants it out anyway:* it moves to Out of Scope with the honest reason ("we accept that 12% of agent sessions, 17% at weekends, cannot upload") rather than staying as a false statement of fact.

**Stop 2 — the file-size limit.** The draft has "MUST support large files" (FR-003) as a requirement *and* "maximum file size — engineering to decide" as an open question; those contradict each other and neither is buildable. My default: **25 MB per file and 25 MB total per message**, matching email ingestion so a requester gets the same answer on both paths, and covering p99 (18 MB) and the largest ever accepted (24.6 MB).
- *Confirm with engineering:* whether 20 s of upload inside a 30 s cutoff, with no resumable path, is acceptable.
- *If they say the margin is too thin:* fall back to 10 MB per file (still above p95 = 4 MB), and I'd add an open question on whether resumable uploads are worth funding — noting 31 email rejections/month today, so the tail is real but small.
- Either way, FR-003 stops being a requirement and becomes a specific number plus a stated ceiling.

**Stop 3 — retention.** Legal was asked in August and hasn't answered. This does not block the build: `security-baseline.md` records that email-ingested files are kept for the life of the ticket and closed tickets are never purged. I'd write that as the assumed default so engineering can proceed, keep the open question with **Legal** named as owner and a needed-by date of the launch decision, and note the cost exposure (we pay per GB stored *and* per GB served, so an unbounded retention default has a running bill attached). I would not invent a retention period on Legal's behalf.

The two trivial open questions I'd just decide, because "nobody has an opinion" is not a reason to hold a spec: attachment list **newest first** (matches the conversation's own ordering), and filename truncation with a middle ellipsis that preserves the extension. Both go to Assumptions marked as cheap to reverse.

## Phase 3 — Write the revision

One file written: **`specs/ticket-attachments/spec.md`**, rewritten in place, same eight sections in the house order from `specs/README.md`. I would *not* add a "changelog" or "revision notes" section — the house format lists the permitted sections and I'd report the change list to the user instead of bending the format.

ID policy: keep FR-001…FR-011 attached to the same intent where the requirement survives, so Maya can diff the draft against the revision line by line; append new ones from FR-012. FR-008 is repurposed rather than deleted so its history stays visible.

**Header** — Status becomes `Revised — ready for design`; Author stays Maya (PM); add a revised-by line and 9 September 2026.

**Overview** — keep essentially as written. It's good: it names the audience, the current workaround, and what changes. I'd add one sentence of evidence: 0% of portal-opened tickets carry an attachment today, and 29 of 38 August support tickets were people asking how to send a file from the portal.

**User Stories** — keep all three, keep their priorities. Repairs:
- US-001: replace "a user-friendly error message" in the third scenario with the specific failure cases and what each says — file too large (names the limit, as the email bounce already does), disallowed type (names the accepted types), network/timeout failure (offers retry without losing the composed text).
- US-002: delete the "the experience is seamless" scenario and replace it with observable ones — per-file progress, cancel an in-flight upload, send blocked until uploads finish or the user removes them, and a scan-pending state.
- US-003: the draft's story and its one scenario cover images only, but FR-006 promises PDF preview too. I'd add a PDF scenario to the story rather than quietly shrinking FR-006, and add a third scenario for the non-previewable types (gif is previewable; txt, log, csv, zip download) so design knows every accepted type has a defined view behavior.
- Add **US-004 (P2)**: a quarantined file. `security-baseline.md` already defines this for email; the new upload paths need it too — the uploader must be told, and the ticket shows a notice in the file's place.

**Functional Requirements** — the core of the work:
- FR-001 keep, extended with how many files per message and total-size behavior.
- FR-002 "fast" → p95-sized file (4 MB) completes in under 5 s on the reference profile, and every accepted file must complete inside the 30 s gateway cutoff.
- FR-003 "large files" → the specific per-file and per-message limits from Stop 2, with the 30 s/1.25 MB/s derivation stated inline so a future reader can re-check it.
- FR-004 "secure" → split into the concrete baseline items: malware scan before the file is available, allowed content types enumerated, signed URL with 1 h expiry issued only after the ticket permission check, download audit log, encryption at rest, HTTPS only.
- FR-005 keep, but pointing at the enumerated error cases now in US-001.
- FR-006 keep, now matched by the PDF scenario in US-003, plus explicit download-only behavior for the rest.
- FR-007 "seamless" → deleted as a requirement; its content is now the observable US-002 scenarios (progress, cancel, send gating).
- FR-008 → rewritten per Stop 1.
- FR-009 "all browsers" → the exact list from `docs/browser-support.md`, with IE named as excluded.
- FR-010 "many concurrent uploads" → sustain at least 80 uploads/min (2× today's observed 40/min email peak) with no rise in error rate. **I'd flag that the doubling is an assumption, not a measurement**: `analytics/` has no portal ticket volume, so the added portal demand cannot be derived from anything in this workspace. It goes in Assumptions and gets an open question for whoever owns capacity.
- FR-011 keep as written — it already matches `security-baseline.md` exactly. Add the audit-log obligation and say what happens when an agent is unassigned from a ticket.
- New: **scanning is asynchronous** and does not sit inside the upload request (the 20 s + 5 s arithmetic); attachment shows a scanning state until cleared.
- New: **removing an attachment** — before send, and whether after send is allowed (I'd propose not, and note the audit-log implication).
- New: **what happens to an agent's attachment on the outbound email** to the requester. The docs give the inbound 25 MB limit but say nothing about outbound. I'd write the requirement as "the requester can open the file from the emailed reply" and raise the bytes-vs-signed-link choice as an open question owned by engineering, rather than inventing a mechanism. Signed URLs expire in an hour, so a link in an email is not straightforwardly correct — that's exactly why it needs an owner.
- New: duplicate filenames within one ticket.

**Success Criteria** — all three replaced with measurable versions off the August baselines:
- SC-001 ("users are happy" — no instrument exists) → the 29 August "how do I send a file from the portal" tickets fall to near zero within two months of launch.
- SC-002 → the `attachments` tag drops from 38/month, measured over the same window, excluding the 29 above so it isn't double-counted.
- SC-003 ("meaningful share") → portal-opened tickets carrying an attachment go from 0% to a named target within 90 days, with 23% (the email rate) as the reference point.
- New SC on upload success rate, since a spec that only measures adoption can't tell a working feature from a broken one.

**Assumptions** — currently "None recorded", which is untrue of the draft. I'd record each as `A-XXX`: the 25 MB limit derivation; allowed types mirroring email ingestion; retention defaulting to current behaviour pending Legal; the 2× concurrency figure; newest-first ordering; filename truncation; mobile agent support per Stop 1.

**Open Questions** — each gets an owner, which the house format demands and the draft omits entirely. Retention → Legal, needed by launch decision, not blocking build. File-size confirmation → engineering. Outbound email delivery mechanism → engineering. Portal volume for the concurrency target → whoever owns capacity. The two trivial ones are removed, having become assumptions.

**Out of Scope** — keep both existing entries, add the "why" the house format asks for ("a later cut" isn't a reason). Add newly-excluded items so they aren't mistaken for oversights: chunked/resumable uploads, rescanning previously-stored files, and — if Stop 1 goes the other way — mobile agent upload.

## Phase 4 — Verification pass before I hand it back

No code, so no test run; the checks are on the document.

1. **Traceability, both directions.** Every FR's `Source:` names a story that actually contains that behavior (this is how the draft's FR-006/US-003 PDF mismatch got through), and every story has at least one FR serving it. Done by reading the finished file end to end.
2. **Vague-language sweep.** Grep the revised file for `fast|seamless|secure|user-friendly|large|many|all browsers|meaningful|happy|appropriate|robust`. Expected result: zero hits in the Functional Requirements and Success Criteria. Any survivor is a requirement I failed to make testable.
3. **Number provenance.** Every figure in the revision traces to one of the four reference docs or to a stated assumption. Expected: no unsourced numbers. This is the check that catches me inventing a limit.
4. **Contradiction check** against each of the three docs — particularly that nothing in the spec assumes a chunked upload path, an IE user, or a synchronous scan.
5. **Section order** matches `specs/README.md` items 1–8, with the required ID prefixes present.

## Delegation

None. This is one document against four short reference files that I've now read in full; splitting it across workers would cost more in briefing and cross-checking than doing it, and the value here is one person holding all the constraints at once. (Also, nothing here calls for an agent, and the review command in this environment is user-triggered and billed — I wouldn't launch it, and this isn't a git repo in any case.)

## What I'd report back

- The revision is written to `specs/ticket-attachments/spec.md`; the draft's structure, all three stories, and FR-001/005/006/011 survive largely intact.
- The change list: six requirements made measurable, three success criteria rebuilt on August baselines, seven assumptions recorded where the draft had "none", owners attached to every open question, and roughly five gaps closed (async scanning, attachment removal, quarantine UX, outbound email delivery, duplicate filenames).
- **FR-008 was wrong** — it claims agents always work from desktop, and the analytics say 12% of agent sessions (17% at weekends) are phone-sized. I've written mobile agent upload as in scope; Maya's call to reverse it, and if she does it becomes an explicit, costed exclusion rather than a false premise.
- Two things I could not settle and did not paper over: **retention** is still with Legal, unanswered since August, and it carries a real bill because we pay per GB stored and served with closed tickets never purged; and the **25 MB limit needs engineering's sign-off**, because 25 MB is 20 s of upload against a hard 30 s cutoff with no resumable path. The spec is buildable under my stated defaults for both, and I'd say plainly which parts change if either ruling goes the other way.