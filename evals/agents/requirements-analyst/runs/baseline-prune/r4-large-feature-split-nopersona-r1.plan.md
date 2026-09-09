FILES-READ: `README.md`, `specs/README.md`, `specs/volunteer-invite/spec.md`, `docs/glossary.md`, `docs/messaging-policy.md`, `docs/data-model-notes.md`, `requests/bulk-import-enrichment.md`

---

# Action plan — write `specs/bulk-volunteer-import/spec.md`

## Phase 0 — Read and reconcile (done above, before drafting anything)

Everything in the workspace is already read: the enrichment, the house format, the reference spec, and all three docs. The reconciliation step is where the real work is, because the enrichment asks for things the current platform cannot do. Before writing a line of spec I'd pin down these collisions, since each one forces a requirement:

| Collision | Where | Consequence for the spec |
|---|---|---|
| Coordinators want "import now, invite later, in groups" | enrichment L30–31 | `data-model-notes.md` L9: *nothing today creates a volunteer without also sending the invitation*. Import must decouple creation from invitation, and the `status` enum (`invited`/`active`/`inactive`) has no value for "in Rota, not yet invited". |
| Files of 800, 3,000, 120 names | enrichment L32–33 | `messaging-policy.md` L8–11: hard ceiling of 500 emails per org per 24 hours, with a spam-flagging incident behind it. A one-click "import and invite" for 800 people is not buildable and must not be specced. This is the same constraint that makes "invite later, in groups" the *only* safe design — the user need and the platform limit agree. |
| 3,000-row file | enrichment L32 | `data-model-notes.md` L16–18: 30-second gateway cut-off, rows already written stay written, no background job runner, nothing resumes. A 3,000-row import cannot be one request. |
| 3,000 + existing volunteers | enrichment L32 | 5,000-volunteer cap on the Community plan, which every current customer is on. Import can push an org over its plan limit. |
| "Can they undo it?" | enrichment L34 | Soft delete already reserves the email for 90 days so a re-add reattaches history (`data-model-notes.md` L11–13). That is the mechanism for undo-then-reimport; I'd build undo on it rather than invent one. |
| Existing-email rule unanswered | enrichment L35–38 | Genuinely undecidable by me — goes to Open Questions with a stated default, not silently guessed. |
| Duplicate rows, different email capitalisation | enrichment L26–27 | `email` stored lower-cased, unique per org; the invite spec already compares case-insensitively (FR-002). Dedupe rule follows from existing behaviour, not from a new decision. |

**No delegation.** This is one document in a seven-file workspace that I have fully read; splitting it across workers would cost more in briefing and review than writing it. I'd do it in one seat.

## Phase 1 — Decide the shape, and name the stop points

The spine I'd commit to: **upload → map columns → preview → commit (writes records, sends nothing) → invite in batches as a separate act → undo while un-invited.** Every coordinator complaint in the interviews lands on that spine, and it is the only shape that survives the 500/day ceiling.

Three points where I'd stop for a human ruling. In a live engagement I'd raise all three in one message rather than three, and keep drafting under the defaults meanwhile:

**Stop 1 — the row-already-exists rule** (the question the interviews explicitly failed to answer). What I'd put to the product owner who ran the interviews: two coordinators said "update", one said "ask me", and the "update" answers assumed the spreadsheet is fresher than Rota, which the notes themselves say is not always true.
- Ruling *skip* → my default; ships as written below.
- Ruling *update* → I add a field-level preview ("3 volunteers would change: Jo Patel's phone, …"), a rule that a non-empty Rota field is never overwritten by a blank spreadsheet cell, and undo has to restore prior field values, which makes undo materially harder — I'd say so when asking.
- Ruling *ask per row* → unusable at 800 rows; I'd counter-propose one bulk choice at preview time ("update all matched" / "skip all matched") and spec that instead.
- **Default I proceed under:** skip. It is the only non-destructive option, it needs no new undo machinery, and the preview names every skipped row so nothing is hidden.

**Stop 2 — an un-invited volunteer state.** This changes the volunteer record, which `data-model-notes.md` owns. Question for engineering: add a fourth `status` value (I'd propose `imported`) preceding `invited`, or a separate flag? Either way, `invited_at` stays null until the invitation actually goes out, and messaging policy already forbids anything but the invitation reaching them.
- **Default:** spec it as a new `imported` status, written as an assumption and flagged in the report so the data-model doc gets updated alongside.

**Stop 3 — how a 3,000-row file gets written at all.** Question for engineering: given the 30-second cut-off and no job runner, do we (a) have the browser post fixed-size chunks and track progress server-side so a cut-off or closed tab resumes cleanly, or (b) build a job runner first, or (c) cap files below what one request can do — which cuts out the 3,000-name coordinator, i.e. the loudest case in the enrichment?
- **Default:** (a) chunked with resume. I'd write the requirement as observable behaviour — a cut-off import must resume or roll forward without duplicating rows, and the coordinator must always see how far it got — so the spec stays honest whichever mechanism engineering picks. I would *not* write "import 3,000 rows in one request", because it is known to be impossible here.

## Phase 2 — Write the spec

Single file, created at `specs/bulk-volunteer-import/spec.md` (new directory). Sections in the house order, IDs numbered as in `specs/README.md`, prose style matched to `specs/volunteer-invite/spec.md` (British spelling, bolded Given/When/Then, an `**Independent test:**` line closing each story, a `*Source: US-XXX*` on every requirement).

**Header** — Feature: Bulk Volunteer Import; Status: Draft; Author: analyst seat; Date: 9 September 2026.

**Overview** — a coordinator uploads their existing spreadsheet, confirms what Rota will do with it, and gets their volunteers into Rota; invitations are a separate, deliberate act afterwards. States plainly that import sends no email.

**User Stories** (P1 unless noted):

- **US-001 — Upload a file and confirm the column mapping.** Rota proposes a mapping from the header row and the coordinator corrects it. Scenarios cover: heading variants ("Email"/"E-mail address"/"Contact") auto-mapped; one column holding a full name split into `first_name`/`last_name` with the coordinator confirming the split; national-format phone numbers with spaces converted to E.164 using the organisation's country; a column Rota does not recognise, such as `Left?`, listed as ignored so the coordinator knows those rows were *not* filtered out; a file whose first row looks like data rather than headings.
- **US-002 — See exactly what will happen before anything is written.** The direct answer to "I do not want 800 people getting an email because I clicked the wrong thing." Preview shows: rows that will create a volunteer; duplicates within the file collapsed (case-insensitive on email, first occurrence wins, the rest named); rows whose email already belongs to a volunteer in the organisation; rows that cannot be imported and why (missing required name or email, malformed email); columns ignored; and the projected volunteer total against the plan limit. Nothing is written until the coordinator confirms.
- **US-003 — Commit the import without sending anything.** Records created in the un-invited state; no email leaves Rota; the coordinator sees a summary. Scenarios include a large file progressing in visible steps, and an interrupted import (closed tab, cut-off request) leaving a clearly-reported partial result that can be resumed or re-run without duplicating anyone.
- **US-004 — Invite imported volunteers in groups.** Coordinator selects a group (by tag, e.g. first-aiders first, or by explicit selection) and invites them. Scenarios cover: a selection under the daily allowance sending within a minute; a selection over it sending 500 and queueing the rest with the release time shown, never silently dropping or bursting; and imported volunteers receiving no scheduling message before they accept.
- **US-005 — Undo an import (P2).** Everyone asked. Undo removes the volunteers a given import created, while they are still un-invited; anyone already invited or accepted is left alone and reported. Scenario for undo-then-fixed-reimport reattaching rather than duplicating, on the back of the 90-day email reservation.
- **US-006 — Import that would exceed the plan limit (P3).** Preview warns; commit refuses rather than half-filling the organisation.

**Functional Requirements** — roughly FR-001 … FR-016, each sourced to a story. The ones I care most about getting written precisely:

- Import MUST NOT send any email; volunteers are created un-invited with `invited_at` null.
- Email matching within the file and against the organisation MUST be case-insensitive, and emails MUST be stored lower-cased (consistent with the existing invite spec).
- A row missing `first_name`, `last_name`, or `email` MUST be reported and skipped, never partially written; a phone number that cannot be converted MUST leave `phone` blank and import the row rather than fail it.
- No records may be written before the coordinator confirms the preview.
- Invitation sends MUST stay within 500 per organisation per rolling 24 hours, counting every other Rota email in the same window; the excess queues visibly.
- Tags applied at import MUST respect the existing 20-per-volunteer / 40-character limits.
- An interrupted commit MUST leave no duplicate volunteers when resumed or retried.
- Import MUST refuse if it would take the organisation past its plan limit.

**Success Criteria** — SC-001 activation, tied to the enrichment's stated value: a majority of organisations that complete an import schedule their first shift within 7 days of importing. SC-002 imports that reach preview go on to commit (measures whether the preview is trusted, not just present). SC-003 a 2,000-row file completes without the coordinator having to retry.

**Assumptions** — the four items the enrichment explicitly left to me, plus the ones the collisions forced: `.csv` and `.xlsx` accepted (Google Sheets via export, since it is not a file format); header row required, with a warning when the first row does not look like headings; per-file row cap set high enough for the 3,000-name case but stated as a number engineering can move; the contents of the post-import summary; the new un-invited status; skip-on-existing-email pending Stop 1; unmapped columns ignored rather than guessed.

**Open Questions** — Stop 1 with its owner named, Stop 2 and Stop 3 as the mechanism questions for engineering with the defaults I proceeded under stated so nobody has to reverse-engineer them.

**Out of Scope** — updating existing volunteers' details from a spreadsheet (pending Stop 1); importing shifts or availability; syncing with an external database rather than a one-off file; a background job runner; scheduling messages to imported volunteers, which messaging policy forbids before consent.

## Phase 3 — Check before I call it done

No tests to run — this workspace is documents only, with no code, no test suite, and no runner, so there is nothing to execute and I would not pretend otherwise. The checks are editorial and I would do them against the files, not from memory:

1. Section order and ID prefixes match `specs/README.md` items 1–8; every story has a priority and an independent test; every `FR-` has a `Source:` line.
2. Every requirement traces to a story, and every explicit want in the enrichment — preview before sending, invite later in groups, undo, duplicates, messy columns, 3,000 names — traces to at least one requirement. Anything in the enrichment with no home in the spec gets flagged rather than dropped.
3. No requirement contradicts `docs/messaging-policy.md` (the 500 ceiling, invitation-only first contact, consent before scheduling messages) or `docs/data-model-notes.md` (field requirements, E.164, tag limits, soft-delete window, plan caps, the 30-second limit).
4. No conflict with `specs/volunteer-invite/spec.md` — bulk import reuses its 14-day expiry and case-insensitive duplicate rule rather than restating different ones. Note in the report that that spec's Out of Scope line ("Adding volunteers in bulk from a file — its own enrichment and spec") is now satisfied and could point here.

## What I'd report at the end

- The spec is written at `specs/bulk-volunteer-import/spec.md`; new directory; nothing else in the workspace changed.
- The one question the interviews left open is still open, with an owner named, and I drafted under skip-on-match — with the one-line reason and what changes if the ruling goes the other way.
- Two things this feature needs that the platform does not have today: a volunteer state that is in Rota but not yet invited, and a way to write a few thousand rows given the 30-second cut-off and no job runner. Both are in Open Questions; the second is the one that could stall the build, so I'd surface it first.
- The one thing the enrichment implies that I deliberately did not spec: any flow that emails a whole imported list at once. The 500/day ceiling and the past deliverability incident make it unbuildable, and the coordinators' own request to invite in groups points the same way.
- Nothing here is worth writing to memory — the house format, the messaging limits, and the data-model constraints are all already recorded in the repo, and the decisions I made belong in the spec's own Assumptions and Open Questions, where the next reader will actually look.