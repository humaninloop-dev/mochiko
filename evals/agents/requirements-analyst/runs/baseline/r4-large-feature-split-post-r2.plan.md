FILES-READ: `README.md`, `specs/README.md`, `specs/volunteer-invite/spec.md`, `docs/glossary.md`, `docs/data-model-notes.md`, `docs/messaging-policy.md`, `requests/bulk-import-enrichment.md`

---

# Action Plan — spec for bulk volunteer import

**Deliverable:** one new file, `specs/bulk-volunteer-import/spec.md` (directory `specs/bulk-volunteer-import/` does not exist yet and would be created). Nothing else in the workspace gets written.

---

## Phase 0 — Grounding (reading; already done)

I read all seven workspace files myself. The tree is seven small documents and every one of them bears on this spec, so there is nothing here worth handing to a cheap reader — a locate/enumerate sweep would cost more to brief than to do. See the delegation note at the end for the one case where I would have spawned help.

What I would carry forward as a constraint ledger (this is the backbone of the whole spec, so I build it before drafting anything):

| # | Constraint (source) | What it forces |
|---|---|---|
| C1 | Volunteers today are only ever created *with* an invitation email; status starts at `invited` (`docs/data-model-notes.md`) | Coordinators explicitly asked to import now and invite later. This feature needs a record that exists but has been sent nothing — a state the data model does not currently have. Central design point. |
| C2 | Hard ceiling of 500 outbound emails per organisation per rolling 24h; a big burst has previously cost deliverability for all customers (`docs/messaging-policy.md`) | 800 and 3,000-name imports **cannot** invite everyone at once. Invitations must be metered across days. Non-negotiable; I will not write a requirement that breaches it. |
| C3 | Writes happen inside the browser request; 30-second gateway cut-off; partial writes stay written; no background job runner; nothing resumes (`docs/data-model-notes.md`) | A 3,000-row import cannot be one request. Needs chunked commit, a batch identity, and re-run safety so a retried chunk does not double-create. |
| C4 | 5,000 volunteers per organisation on the plan every current customer is on | A 3,000-row import fits but leaves little headroom; a repeated import could breach it. Needs a pre-commit headroom check. |
| C5 | Soft delete reserves the email for 90 days and re-add reattaches history | Undo-then-reimport must reattach rather than duplicate; interacts with the undo story. |
| C6 | Email unique per organisation, stored lower-cased; invitation link valid 14 days; unsubscribe moves to `inactive` | Case-insensitive matching for the duplicate handling coordinators described; do not re-invite people who have opted out; do not send invitations so far ahead of a group that links expire unused. |
| C7 | `specs/README.md` fixes eight sections in a fixed order; `specs/volunteer-invite/spec.md` shows IDs restarting at 001 per spec, `Source:` lines on every FR, and an "Independent test" line under each story | My output format, exactly. No new sections, no reordering. |
| C8 | The existing single-invite spec already carries FR-001…FR-004 for invite/accept/consent/expiry | Bulk import should *reuse* that flow, not restate it. My FRs reference the behaviour rather than redefining link expiry or consent. |

---

## Phase 1 — Fix the actors and the flow before writing prose

Actor: coordinator (the only human in this feature; volunteers are recipients, not users of it). Secondary: the organisation's existing volunteer records.

The happy path I would commit to, driven by C1–C3 and the interview note that all three coordinators refused to let anything send before they had looked:

upload file → map columns → **preview** (nothing written, nothing sent) → confirm → records created in a sent-nothing state, in chunks, under one batch identity → summary → *separately, later* → coordinator selects a group and invites it, metered under the 500/day ceiling.

The key structural decision I am taking here and will state as an assumption: **import and invite are two separate acts.** The enrichment asks for it ("first-aiders get in first", "check the list with the committee"), the email ceiling makes an all-at-once invite impossible for 800+ anyway, and the preview demand ("I do not want 800 people getting an email because I clicked the wrong thing") is only truly satisfiable if importing sends nothing at all.

---

## Phase 2 — Resolve the four "not decided (minor)" items, each as a written assumption

The enrichment hands me four minor calls. I would take them and record each as an `A-XXX` so nobody mistakes my choice for a stated requirement:

1. **File types** — accept `.csv` and `.xlsx`; Google Sheets is exported to one of those. Reject `.xls`, `.numbers`, PDF with a message naming the two accepted types.
2. **Header row** — require one, and validate that it exists rather than detect it. A misdetected header silently eats a volunteer; requiring one is cheaper than being clever. The mapping step exists precisely because the headings are inconsistent.
3. **Post-import summary contents** — rows read, volunteers created, rows matched to existing volunteers and what was done with them, in-file duplicates collapsed, rows rejected with the reason and row number, plus a downloadable file of just the rejected rows so the coordinator can fix and re-upload only those.
4. **Row cap per file** — 3,000, sized to the largest real case in the interviews ("the whole village"), and comfortably under the 5,000 organisation cap so one file cannot fill an account.

---

## Phase 3 — Draft the User Stories section

Before writing, I would invoke the `mochiko:authoring-user-stories` skill for the story/scenario/priority format (not loaded in this planning run; noted as the step). Stories I would write, with priorities and the Given/When/Then scenarios each needs:

- **US-001 Upload and map columns (P1)** — scenarios for inconsistent headings mapped by hand, one column holding both first and last name split into two fields, an unmapped column ignored, a required field left unmapped blocking the step, and a file over the row cap or of a rejected type refused with a named reason.
- **US-002 See exactly what will happen before confirming (P1)** — the preview. Scenarios: preview lists creates, matches against existing volunteers, in-file duplicates, and rejected rows with row numbers; abandoning at preview leaves zero records written and zero emails sent; phone numbers shown converted to the stored international format using the organisation's country; a row whose tags exceed the per-volunteer limits shown as a warning, not a silent trim.
- **US-003 Import without anything being sent (P1)** — this is the C1 story. Scenarios: confirming creates records that have received no email; the coordinator can see they are un-invited; no email is generated by import under any path.
- **US-004 Invite in groups afterwards (P1)** — scenarios: select a subset and invite it; a selection larger than the day's remaining email allowance is refused or metered with the remainder clearly scheduled and visible; already-active volunteers and anyone who has unsubscribed are excluded from a bulk invite; invited people follow the existing single-invite acceptance path unchanged.
- **US-005 A large file completes or resumes safely (P1, driven by C3)** — scenarios: a 3,000-row import completes with progress visible; a browser tab closed mid-import leaves a partial batch that is shown as partial and can be resumed or undone, never silently half-there; resuming does not create a row twice.
- **US-006 Undo an import (P2)** — scenarios: undoing removes the batch's records; a record already invited or already accepted is **not** silently removed and is reported instead, because the email cannot be recalled; re-importing after an undo reattaches to the reserved email rather than duplicating (C5).
- **US-007 Get a summary and fix the rejects (P2)** — the Phase-2 item 3 contents, plus re-uploading only the corrected rejects.

Each story gets its own "Independent test" line, in the style of the reference spec.

**Stop/branch on priorities:** if a reviewer wants US-005 dropped to P2, my branch is to refuse the demotion while the 30-second limit and the 3,000-name case both stand, and say why in one line; I would accept demoting US-006 to P3 but not deleting it, since every coordinator interviewed asked about undo.

---

## Phase 4 — Derive the Functional Requirements

I would invoke `mochiko:authoring-requirements` here for the FR/SC format and the obligation keywords. Every FR gets a `Source:` line naming its story, matching the reference spec. Roughly twenty requirements, grouped:

*File and mapping* — accepted types; header row required; 3,000-row cap; explicit mapping of headings to `first_name`/`last_name`/`email`/`phone`/`tags`/`notes`; splitting a combined name column; ignoring unmapped columns; national phone numbers converted using the organisation's country; emails stored lower-cased; tag count and length limits and the notes length limit enforced at preview rather than by silent truncation.

*Preview* — preview mandatory before commit; preview writes no records and sends no email; preview reports creates, existing-volunteer matches, in-file duplicates (matched case-insensitively per C6), and rejects with row numbers.

*Commit* — records created having been sent nothing (C1); commit writes in chunks small enough to finish inside the request limit; every row carries the batch identity; a re-submitted chunk must not create a second record for the same row (C3); commit refuses if the row count would take the organisation past its volunteer cap, naming the shortfall (C4).

*Invitation* — import never sends email; bulk invite never causes the organisation to exceed 500 outbound emails in a rolling 24 hours (C2); a bulk invite excludes active and unsubscribed volunteers; invited rows then behave exactly as the existing invite spec's FR-001/FR-003/FR-004 describe, referenced rather than restated (C8).

*Undo and summary* — undo scoped to one batch; undo removes only rows never invited and reports the rest; the summary contents and rejected-row download.

**What I would refuse to write here:** any requirement that queues 3,000 invitations "in the background" (no runner exists), any requirement that quietly exceeds the email ceiling, and any requirement that overwrites existing volunteer data — see Phase 6.

---

## Phase 5 — Success Criteria

Measurable, and tied to the enrichment's stated value ("scheduling real shifts within its first week"):

- An organisation that completes an import schedules its first shift within 7 days of that import, target ≥70%, measured from import and shift-creation timestamps.
- At least 95% of rows in a typical spreadsheet import without the coordinator hand-editing the file, measured as rows created ÷ rows read.
- Zero emails sent by the import step itself, and zero calendar days on which an organisation exceeds 500 outbound emails — both monitored, both treated as incidents rather than metrics.
- A 3,000-row import completes without a cut-off request leaving an unlabelled partial state.

---

## Phase 6 — Assumptions, Open Questions, Out of Scope

**Assumptions (`A-XXX`)** — the four minor calls from Phase 2, plus: import and invite are separate acts; a sent-nothing volunteer counts toward the organisation's volunteer cap; unmapped columns such as the "Left?" column are ignored on import rather than interpreted; the undo window is bounded (I would set 7 days) rather than open-ended.

**Open Questions (`Q-XXX`)** — this is where I stop rather than guess:

- **Q-001, the one the interviews failed to close:** when a row's email already belongs to a volunteer in the organisation, does the import update, skip, or ask? Two coordinators said "update", one said "ask", and the enrichment itself notes the "update" answers rested on the spreadsheet being newer, which it often is not. This is user-facing data being overwritten, so I will not settle it alone. **The stop:** I would take this to whoever ran the interviews / owns the product decision. **Branches:** if the ruling is *skip*, the FR is already written that way and only the summary wording changes; if *update*, I add FRs for which fields may be overwritten, whether a blank cell clears a value (my strong recommendation: never), and whether the previous values are recoverable via undo; if *ask*, I add a per-row resolution step to the preview and a story for it, which materially grows the feature and would probably split into a follow-up spec. **My stated default while it is open:** skip the row, change nothing on the existing volunteer, and list every skipped match in the preview and the summary — the only option that cannot destroy data.
- **Q-002:** does the data model gain a genuine new status for a sent-nothing volunteer, or do we represent it as `invited` with no `invited_at`? Owner: engineering. **Default:** the spec describes the *behaviour* (exists, has been sent nothing, is visibly un-invited) and leaves the field-level modelling to engineering, so the spec is not invalidated either way.
- **Q-003:** do sent-nothing volunteers count toward the 5,000 cap and toward billing? Owner: product. **Default:** they count; a 3,000-name import then leaves 2,000 of headroom, which the pre-commit check surfaces.
- **Q-004:** for an organisation whose spreadsheet exceeds the plan cap, is there a path other than "buy the Network plan"? Owner: product. Not blocking: the file cap and the headroom check make the failure explicit rather than silent.

**Out of Scope** — automatic column detection, importing availability, importing from Google Sheets by live connection, syncing an ongoing spreadsheet, and importing shift history. Each with its one-line reason.

---

## Phase 7 — Self-check before I call it done

- Walk `specs/README.md` section by section and confirm all eight are present, in order, with the right ID prefixes and a per-spec numbering that restarts at 001 as the reference spec does.
- Confirm every FR carries a `Source:` line and every story carries an "Independent test" line.
- Walk the C1–C8 ledger and point at the specific story or requirement that answers each; any constraint with no landing place means the draft is incomplete.
- Walk the enrichment's own list — preview, staged invites, undo, duplicates, messy columns, the 3,000-name case, and all four minor items — and confirm each is either specified or explicitly out of scope.
- Scan my own text for unquantified words ("quickly", "large", "user-friendly") and replace each with a number or a named condition.
- Confirm no requirement contradicts the messaging policy or the invite spec.

**Flag, don't edit:** `specs/volunteer-invite/spec.md` says bulk import is out of scope "— its own enrichment and spec", which is now this file. That line wants a cross-reference, but amending a shipped spec is not what I was asked to do; I would report it as a one-line follow-up rather than change it.

---

## Delegation

None on this card, deliberately: the workspace is seven short files, every one is load-bearing for this spec, and reading them interpretively is the job rather than a sweep to farm out. Had this repo held dozens of specs, I would have spawned one throwaway `Explore` subagent pinned to `model: haiku` with the brief "list every file under `specs/` and `docs/` that mentions import, batch, CSV, or upload; return path and line only" — and on its return I would spot-check two of the cited lines myself before trusting the list, since a missed prior decision is exactly the kind of gap that would change what I write.

---

## What I would report at the end

The path written (`specs/bulk-volunteer-import/spec.md`) and its shape — seven stories, four at P1; the requirement count; the success criteria. Then the three things a reader must act on: **Q-001 is genuinely open and the spec runs on a "skip and report" default until someone rules on it**; **the 500-emails-per-day ceiling means an 800-name organisation takes at least two days to invite everyone and a 3,000-name one takes at least six, which is a product fact somebody should hear before a customer does**; and **importing without sending anything is new behaviour for the volunteer record, so engineering needs to answer Q-002 before build**. Plus the small follow-up on the invite spec's out-of-scope line.