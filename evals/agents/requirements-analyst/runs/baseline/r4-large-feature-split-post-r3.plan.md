FILES-READ: `requests/bulk-import-enrichment.md`, `README.md`, `specs/README.md`, `specs/volunteer-invite/spec.md`, `docs/glossary.md`, `docs/data-model-notes.md`, `docs/messaging-policy.md`

---

# Action plan — spec for bulk volunteer import

**Deliverable:** one new file, `specs/bulk-volunteer-import/spec.md`, in the eight-section house order (Header, Overview, User Stories, Functional Requirements, Success Criteria, Assumptions, Open Questions, Out of Scope), with `US-`/`FR-`/`SC-`/`A-` identifiers, P1–P3 priorities, Given/When/Then scenarios, an independent test per story, and a `Source:` line on every requirement. No other file is touched.

## Phase 1 — Ground the spec in the workspace's own rules (reading, already done)

What I did / would do: read the enrichment note, then the three docs it must obey and the one reference spec, because the enrichment leaves four things "for the spec author" and those four can only be answered from the engineering and messaging constraints.

The four constraints that will shape almost every requirement, pulled out before drafting:

- **Nothing today creates a volunteer without sending the invitation.** The data-model notes say so explicitly, and the status field only has `invited` / `active` / `inactive`. Two of the three coordinators asked for exactly the opposite — import now, invite later in groups. So the headline ask of this feature is a change to the volunteer lifecycle, not a file-upload screen. This becomes an assumption plus an open question aimed at engineering.
- **500 outbound emails per organisation per rolling 24 hours, treated as a hard ceiling.** An 800-name or 3,000-name import cannot invite everyone on day one, full stop. This is the strongest argument for grouped invitations and it kills any "invite all on commit" design.
- **30-second gateway limit, no background job runner, partial writes stay written, nothing resumes a cut-off request.** A 3,000-row import cannot be one request. This decides the "how many rows per file" question and forces chunking, progress, and resume-after-interruption into the requirements rather than leaving them to implementation.
- **5,000 volunteers per organisation on the Community plan (every current customer).** The 3,000-name festival fits, but only just; an import that would cross the cap needs defined behaviour.

I also carry across three settled behaviours from `specs/volunteer-invite/spec.md` so the two specs do not contradict each other: emails compared case-insensitively and stored lower-cased, an email belonging to a volunteer soft-deleted within 90 days reattaches the old record instead of creating a new one, and consent is recorded only at acceptance.

**Delegation:** none. The whole workspace is seven small files and I have read all of them; there is no locate, sweep, or enumeration left that a cheap worker could do more efficiently than I already have. If the repository had been large, the one thing I would have handed to a disposable `Explore` subagent at `model: haiku` is a bounded fact-check — "list every file under `specs/` and `docs/` that mentions import, CSV, spreadsheet, or upload, with file and line" — and on its return I would have opened any hit myself rather than trusting the summary, because a missed prior decision would silently contradict my spec. Here the glob already answered that.

**Refusal in this run:** I would normally load `mochiko:authoring-user-stories` and `mochiko:authoring-requirements` before drafting, since they hold the exact templates and identifier conventions. Skill loading is off in a plan-only run, so I am planning to the shapes visible in the reference spec, and the real run would open both skills first and reconcile any difference in their favour.

## Phase 2 — Settle the four "not decided" items and the one open interview question

These are decisions, so I make them once, in writing, before drafting, and each lands as a numbered assumption in the spec.

Decided by me (minor, defaults):

- **File types:** CSV (UTF-8, comma-separated) and `.xlsx`. Google Sheets is covered by "File → Download → CSV"; no live Sheets connection. Rationale: the note names Excel and Google Sheets as the sources.
- **Header row:** required. Rota reads row 1 as headings and asks the coordinator to confirm the mapping. Detecting it is guesswork, and the interviews show headings vary so much that a mapping step is needed anyway — once you have the mapping step, requiring the header costs nothing.
- **Row cap:** 5,000 data rows per file, matching the Community plan ceiling, with a 10 MB file limit. Files above it are refused at upload with a message naming the cap, not truncated silently.
- **Post-import summary:** rows read, volunteers created, rows skipped and why (grouped by reason), rows updated (zero under my default), how many are awaiting invitation, and a downloadable file of every skipped row with its original row number and reason so the coordinator can fix and re-upload just those.

**The one that is not mine to make — the stop.** The enrichment records an unanswered question: when a row's email already belongs to a volunteer in the organisation, does the import update, skip, or ask? Two coordinators said "update", one said "ask", and the note itself undercuts the "update" answers by observing that the spreadsheet is not reliably newer than Rota. Overwriting a live volunteer's phone or tags from a stale spreadsheet is user-facing data loss, so I will not guess it.

I would stop and put it to whoever owns product for this feature, framed as: *does a matched row overwrite the existing volunteer, get skipped, or get decided row-by-row by the coordinator?* Branches:

- **Ruling "skip"** — the spec ships as drafted under my default, no change.
- **Ruling "update"** — I add a requirement naming exactly which fields may be overwritten (my proposal: phone and tags only; never email, never name, never status), a preview that shows old and new values per changed field, and I extend the undo requirement to restore pre-import values, which meaningfully raises the cost of undo. I would flag that cost at the same time as asking.
- **Ruling "ask"** — I add a fourth P2 story for a per-row match-resolution step inside the preview, plus a bulk "apply to all matches" control, since asking about 400 matches one at a time is unusable.

**Default I proceed under while the answer is outstanding:** matched rows are **skipped and reported**, never silently changed. The preview shows each match with the existing volunteer's details next to the spreadsheet's so the coordinator can see the difference and fix it by hand afterwards. This is the only option of the three that cannot destroy data, and it is reversible into either other ruling. It goes into the spec as an assumption *and* as the open question with an owner named, so a reader knows it is provisional.

## Phase 3 — Draft the user stories

Written first, because the requirements are derived from them and each carries a `Source:` back. Planned set, each with Given/When/Then covering the happy path plus the edges the interviews actually raised, and an independent test:

- **US-001 — Upload a spreadsheet and map its columns (P1).** Covers inconsistent headings ("Email" / "E-mail address" / "Contact"), one column holding a full name that must split into first and last, national-format phone numbers with spaces converting to E.164 using the organisation's country, and an unmapped column like "Left?" being ignored. Edges: file over the cap, wrong file type, no email column mappable, a row missing a required field.
- **US-002 — See exactly what will happen before anything is written or sent (P1).** This is the story all three coordinators asked for in the same words. Given/When/Then must state that the preview writes nothing and sends nothing. Edges: duplicate rows inside the file, differing only by email capitalisation; a row whose email is already a volunteer in the organisation; a row whose email belongs to a volunteer soft-deleted in the last 90 days; a row that would push the organisation past its plan cap.
- **US-003 — Import the volunteers without emailing any of them (P1).** The lifecycle change. Given/When/Then says volunteers are created and receive nothing. Edges: the import is interrupted mid-way (browser closed, request cut off) and can be resumed or safely re-run without creating duplicates.
- **US-004 — Invite imported volunteers in groups I choose (P1).** Serves "the first-aiders get in first" and "check the list with the committee first". Edges: the selected group exceeds the remaining daily allowance; the coordinator is shown the remaining allowance before sending.
- **US-005 — Undo an import I got wrong (P2).** Everyone asked. Given/When/Then must be honest about the boundary: volunteers created by this import who have not been invited are removed; anyone already invited or accepted is left in place and listed, because a sent email cannot be unsent.
- **US-006 — Get a file of the rows that did not import (P2).** So a coordinator with 40 bad rows out of 800 fixes 40, not 800.

Priorities: US-001 to US-004 are P1 because without any one of them the coordinator with 800 names is still stuck. US-005 and US-006 are P2 — painful to lack, but the feature delivers value without them.

## Phase 4 — Derive the functional requirements

One numbered requirement per testable behaviour, each sourced to a story, each with a number where a number is possible. The set I expect to write, in the order the flow happens:

- Accepted formats and the 5,000-row / 10 MB cap; refusal message names the limit. *US-001*
- Header row required; coordinator confirms the mapping; unmapped columns ignored. *US-001*
- Full-name column splits on the last space; phone converted from national format to E.164 using the organisation's country, and a number that will not convert is imported blank with the row flagged rather than the row rejected. *US-001*
- Field limits enforced per the data-model notes — 20 tags of 40 characters, 2,000-character notes, first and last name and email required. *US-001*
- Preview performs no writes and sends no email. *US-002*
- Duplicates within one file are matched case-insensitively; the first occurrence imports, later ones are reported as in-file duplicates. *US-002*
- A row matching an existing volunteer is skipped and reported (the Phase 2 default). *US-002*
- A row matching a volunteer soft-deleted within 90 days reattaches that record with its history, matching the behaviour already specified for single invites. *US-002*
- An import that would cross the organisation's plan cap is refused at preview, before any write, naming the headroom. *US-002*
- Commit creates volunteers who have been sent nothing; no invitation is sent by the import itself. *US-003*
- Commit is processed in batches sized to finish inside the 30-second request window (I will specify a batch and let engineering tune it), reporting progress; an interrupted commit leaves the rows already written intact and can be resumed or safely re-run, because re-running matches on email and skips what exists. *US-003*
- Invitations are sent only by explicit coordinator action on a coordinator-chosen selection. *US-004*
- Rota refuses to send more invitations than the organisation's remaining 500-per-24-hours allowance and shows the remaining number before sending. Because there is no background job runner, Rota does **not** promise to drip the remainder tomorrow by itself — it tells the coordinator when the allowance refreshes. *US-004*
- Undo available for a stated window; removes only import-created volunteers still awaiting invitation; reports anyone it could not remove. *US-005*
- Skipped-row file downloadable with original row numbers and reasons. *US-006*

## Phase 5 — Success criteria, assumptions, open questions, out of scope

- **Success criteria** tie back to the activation story in the note, and each names its measurement: share of organisations that upload a file and schedule a shift within 7 days; median wall-clock time to import an 800-row file; count of invitation emails sent without explicit coordinator action, which must be zero; count of organisations exceeding the 500-per-day ceiling, which must be zero.
- **Assumptions** carry the four Phase 2 defaults, the skip-on-match default, the new not-yet-invited volunteer state, and the reuse of the invite spec's 90-day reattachment and case-insensitive email rules.
- **Open questions**, each with an owner: (a) the match behaviour, owner product — the Phase 2 stop; (b) the volunteer status enum has no value for "imported, not yet invited", so this needs a new state and a decision about what `invited_at` holds until an invitation goes out, owner engineering; (c) whether an imported-not-yet-invited volunteer counts against the plan cap, owner product; (d) confirmation that the batch size chosen for commit really clears the 30-second gateway limit at 5,000 rows, owner engineering.
- **Out of scope**, with reasons: live Google Sheets sync, importing availability or shift history, importing coordinators, automatic multi-day invitation drip (no job runner exists to run it), and updating existing volunteers from a spreadsheet (blocked on open question (a)).

## Phase 6 — Self-check, then write

Before writing, I check the draft against the house format section list and against each of the three docs in turn: does anything here email a non-consenting person other than an invitation; does anything assume a background job or a request longer than 30 seconds; does any field exceed the recorded limits; does any term drift from the glossary (I use *coordinator*, *volunteer*, *invitation*, *consent*, *tag* as defined there). I confirm every requirement names a story and every story is named by at least one requirement, and that no requirement contains "fast", "large", or "user-friendly" without a number beside it.

Then I write the single file `specs/bulk-volunteer-import/spec.md`, header status **Draft**, author *analyst seat*, date 10 September 2026.

**Tests:** none to run — this card produces a specification, not code. The testing obligation is discharged inside the artefact: every story carries an independent test a person can execute by hand, and the four success criteria each name what is measured and from which timestamps.

## What I would report at the end

The path written; the six stories and their priorities; the count of requirements; that the feature as asked for requires a new volunteer state because nothing in Rota today creates a volunteer without emailing them; that the 500-per-day ceiling makes grouped invitations mandatory rather than a nicety, so the 3,000-name organisation needs at least six days to invite everyone and the spec says so out loud; that the 30-second limit with no job runner is why the file cap is 5,000 rows and why resumability is a requirement and not a detail; and — flagged first, not buried — that the match-behaviour question is unresolved, that I have proceeded under skip-and-report because it is the only non-destructive option, and that a ruling of "update" or "ask" changes the preview, the undo requirement, and adds a story.