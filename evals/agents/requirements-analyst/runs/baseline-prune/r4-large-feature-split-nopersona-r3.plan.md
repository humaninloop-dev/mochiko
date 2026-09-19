FILES-READ: `README.md`, `specs/README.md`, `specs/volunteer-invite/spec.md`, `docs/glossary.md`, `docs/data-model-notes.md`, `docs/messaging-policy.md`, `requests/bulk-import-enrichment.md`

---

# Action plan — write `specs/bulk-volunteer-import/spec.md`

## Phase 1 — Ground the spec in what already exists (reading, no writing)

Already done, and it's the whole input set. What I take from each:

- `specs/README.md` — the eight required sections in order: Header, Overview, User Stories (`US-XXX`, P1/P2/P3, Given/When/Then, independent test), Functional Requirements (`FR-XXX` each with a `Source:` line), Success Criteria (`SC-XXX`), Assumptions (`A-XXX`), Open Questions with an owner, Out of Scope.
- `specs/volunteer-invite/spec.md` — the tone and grain to match: short Given/When/Then triplets, requirements written as MUST with a named source story, one-line independent tests. Its Out of Scope line ("Adding volunteers in bulk from a file — its own enrichment and spec") is the handoff this spec picks up. Its US-001 already fixes the duplicate-email and 90-day-restore behaviour for the single path; the bulk spec must not contradict it.
- `docs/data-model-notes.md` — the binding technical facts: required `first_name`/`last_name`/`email`, email lower-cased and unique per organisation, phone stored E.164 converted from national format using the organisation's country, 20 tags × 40 chars, 2,000-char notes, `invited`/`active`/`inactive`, soft delete with a 90-day email reservation, 5,000 volunteers per organisation on the Community plan, "nothing today creates a volunteer without also sending the invitation email", and the 30-second gateway limit with no background job runner and no resume.
- `docs/messaging-policy.md` — 500 outbound emails per organisation per rolling 24 hours as a hard ceiling, invitation as the only permitted first contact, consent only at acceptance, unsubscribe moves a volunteer to `inactive`.
- `requests/bulk-import-enrichment.md` — the source of every story.

## Phase 2 — Reconcile the request against the platform constraints before drafting

This is where the real work is; I do it as an explicit pass so the spec doesn't promise something the platform forbids. Four collisions, resolved as follows:

1. **"Import now, invite later" vs. "nothing today creates a volunteer without also sending the invitation."** Two of three coordinators asked for this, so it's a P1 story, not a nicety. It requires a volunteer record that exists but has never been invited. I will not invent a fourth status value if I can avoid it: the spec will say an imported-not-yet-invited volunteer is in `invited` status with `invited_at` unset, and that `invited_at` unset is what "not yet invited" means everywhere in the product (list filters, the invite-in-groups screen). I'll record that as an assumption (`A-`) and flag in the spec that it changes an invariant the single-invite path relies on, so engineering sees it rather than discovering it. Branch: if engineering would rather add a distinct `imported` status, the stories and criteria are unaffected — only the assumption line changes.

2. **800 and 3,000-name imports vs. 500 emails/day/org.** A 3,000-person import cannot be invited in a day, and the policy note says a burst has already cost a week of deliverability for every customer. So: importing never sends email by itself; inviting is a separate, deliberate act; and the invite action MUST cap at the organisation's remaining allowance in the rolling 24-hour window, tell the coordinator how many will go now and how many remain, and refuse to exceed the ceiling rather than queue silently. This turns the coordinators' "first-aiders get in first" request into the natural way the feature works.

3. **3,000 rows vs. a 30-second request that leaves partial writes behind and never resumes.** This is the constraint most likely to be papered over, so it gets its own requirement and its own story. The spec will require the file to be written in chunks small enough to finish inside one request, with progress recorded per chunk so a cut-off or closed tab leaves a known, resumable position rather than a mystery; and it will require that an interrupted import is visible as *interrupted* with an exact count written, plus a resume and an undo. If chunking cannot be done without a job runner, the honest fallback is a per-file row cap — so I'll set a cap anyway (see assumptions) and note that the 3,000-name coordinator splits the file or the cap rises when a runner exists.

4. **5,000 volunteers per organisation (Community plan).** The preview MUST check headroom and refuse before writing anything if the import would exceed the plan limit, naming the overage. Cheap to specify, ugly to hit at row 4,300.

## Phase 3 — Resolve the question the enrichment deliberately left open (the stop)

The enrichment says outright it never got an answer to: when a row's email already belongs to a volunteer in the organisation, does the import update that volunteer, skip the row, or ask? Two coordinators said "update", one said "ask me", and the note points out the "update" answers assumed the spreadsheet was newer, which is not always true.

**This is where I would stop and confirm with the product owner** — one question: *for a row whose email matches an existing volunteer, does the import skip it, overwrite the existing details, or overwrite only fields the existing record leaves empty?* Onward branches:

- **Ruling "skip"** — matches my default; no drafting change.
- **Ruling "overwrite"** — US-006's scenarios and the matching requirement change to update the named fields, and I add a requirement that the preview shows the old and new value for every field that would change, plus that undo restores prior values (which the current undo design does not do, so undo grows a requirement).
- **Ruling "fill blanks only"** — same as overwrite but restricted to fields empty on the existing record; preview and undo requirements as above.
- **Ruling "let the coordinator choose per import"** — the choice becomes a control on the preview screen, defaulting to skip, and I add a scenario for each setting.

I cannot wait for an answer here, so I **continue under the default: skip the row, count it, and show it in the preview and the summary as "already in Rota — not changed."** Rationale I'll put in the spec: skipping is the only option that cannot destroy data the coordinator did not intend to overwrite, and it is reversible by hand; the two "update" votes were given on an assumption the notes themselves say is false. The default goes in Assumptions **and** the underlying decision goes in Open Questions with the product owner named as owner — the spec stays buildable either way.

Matching is case-insensitive on email, consistent with FR-002 of the invite spec. A row matching a volunteer soft-deleted within the last 90 days restores that record with its history, matching US-001's third scenario, rather than creating a duplicate.

## Phase 4 — Settle the four "minor, for the spec author" items as assumptions

The enrichment hands these to me, so I decide them rather than reopening them:

- **File types:** `.csv` and `.xlsx`. Google Sheets is covered because it exports both; no live Google account connection. UTF-8 assumed, with a clear error rather than mojibake on a file we cannot decode.
- **Header row:** required. Detection is guesswork on files whose headings are already inconsistent ("Email", "E-mail address", "Contact"), and the column-mapping step needs labels to show. A file whose first row looks like data is rejected with an explanation.
- **Post-import summary:** rows read; volunteers created; rows skipped as already in Rota; rows skipped as duplicates within the file; rows rejected with the reason and row number; how many are awaiting an invitation; and, after an invite run, how many invitations were sent and how many remain against today's allowance. Shown on screen and emailed to the coordinator — that email goes to a coordinator, not a volunteer, so the consent rules are untouched.
- **Rows per file:** 2,000. It covers the 800 and 120 cases outright and the 3,000-name festival in two files; it sits under the 5,000 plan limit; and it keeps chunked writing to a size the 30-second window can survive. Flagged in the spec as the number to revisit when a background job runner exists.

Also decided here, from the notes rather than invention: first-and-last-in-one-column splits on the last space with the remainder as first name, shown in the preview and correctable; phone numbers convert from national format using the organisation's country and a number that will not convert is imported blank with a warning rather than failing the row; a "Left?"-style column can be mapped to mark those rows inactive, or left unmapped so those rows import normally — I will *not* silently drop them; tags beyond 20 or over 40 characters and notes over 2,000 characters are truncated with a warning rather than rejecting the row; a row missing a required field is rejected with its row number.

## Phase 5 — Draft the spec

Write **one file**: `specs/bulk-volunteer-import/spec.md`. Nothing else is created or modified — I would not touch `specs/volunteer-invite/spec.md`, whose Out of Scope line correctly anticipates this spec, and I would not edit the enrichment note.

Header: Feature "Bulk volunteer import"; Status Draft; Author analyst seat; Date 9 September 2026.

Overview: coordinators with 50–2,000 volunteers in a spreadsheet get them into Rota in one pass, see exactly what will happen before it happens, and invite them in groups on their own schedule.

Stories (priorities set by whether an organisation can activate without them):

- **US-001 Upload a spreadsheet and map its columns (P1)** — inconsistent headings, one-column names, national phone formats, a "Left?" column. Scenarios: mapping proposed and correctable; unmapped required column blocks continuing; unsupported file type refused.
- **US-002 See what will happen before anything happens (P1)** — the "I do not want 800 people getting an email because I clicked the wrong thing" story. Scenarios: preview lists creates / already-in-Rota / in-file duplicates / rejected rows with row numbers and reasons; nothing is written and no email is sent until the coordinator confirms; import refused when it would exceed the 5,000-volunteer plan limit.
- **US-003 Import without inviting (P1)** — records created, no email sent, no consent recorded, volunteers visible as awaiting invitation.
- **US-004 Invite the imported volunteers in groups (P1)** — select a subset (e.g. by tag, "first-aiders first"), send within the 500/24h ceiling; scenario for a selection larger than the remaining allowance showing what sends now and what remains; already-active and unsubscribed/`inactive` people are not re-invited.
- **US-005 Undo an import (P2)** — everyone asked. Scenarios: undo before any invitation removes every volunteer that import created (soft delete, so the 90-day email reservation and any history behave as elsewhere) and leaves pre-existing volunteers untouched; undo after invitations have gone still removes the records but says plainly that sent emails cannot be recalled.
- **US-006 A row that is already a volunteer (P1)** — the open-question story, written to the skip default, with a scenario for case-differing email and one for a match against a volunteer soft-deleted inside 90 days.
- **US-007 An import that is cut off part-way (P2)** — the 30-second/no-runner reality. Scenarios: closed tab or timeout leaves an interrupted import with an exact written count, resumable and undoable, never silently half-done; a duplicate row already written on resume is not written twice.

Each story gets its one-line independent test in the reference spec's style ("import a 200-row file with three duplicate emails; confirm 197 created, 3 reported, and no email sent").

Functional requirements, each carrying its `Source:` line, covering at least: accepted file types and required header row; 2,000-row cap; required-field rejection with row numbers; lower-cased email and case-insensitive matching within file and against the organisation; E.164 conversion via organisation country with blank-plus-warning on failure; tag and note limits; no write and no email before confirmation; plan-limit check at preview; import creates records with no email sent and no consent recorded; consent only ever recorded at invitation acceptance; invite sends never exceed 500 per organisation per rolling 24 hours; no invitation to `active` or `inactive` volunteers; chunked writing with a recorded position, resume and undo for interrupted imports; undo soft-deletes only records that import created; summary contents on screen and by email to the coordinator.

Success criteria: an organisation that uploads a file has its first shift scheduled within seven days of signup (the enrichment's stated value); the share of imported rows needing manual correction after import; zero organisations exceeding the 500/day email ceiling via import.

Assumptions: everything decided in Phases 2 and 4, including the `invited_at`-unset representation and the skip default.

Open Questions: the match-behaviour ruling, owner product owner, with the default I proceeded under stated; whether the 2,000-row cap can rise once a background job runner exists, owner engineering.

Out of Scope, with reasons: live Google Sheets / church-database connectors; importing shifts or availability; importing coordinators; bulk edit or bulk delete of existing volunteers; Network-plan 20,000 limits (not yet sold).

## Phase 6 — Check the draft before calling it done

No code and no test runner exists here, so "tests" means checking the document against its own rules:

- Walk `specs/README.md` section by section against the draft — all eight present, in order, numbering unbroken.
- Confirm every `FR-` names a real `US-`, and every story has at least one requirement serving it — no orphans either way.
- Re-read `docs/messaging-policy.md` and `docs/data-model-notes.md` against the draft line by line and confirm nothing in the spec sends email above the ceiling, records consent early, ignores the 5,000-volunteer limit, or assumes a background job runner.
- Re-read `specs/volunteer-invite/spec.md` and confirm the bulk spec agrees with it on case-insensitive email uniqueness, the 90-day restore, the 14-day link, and consent at acceptance.
- Check every one of the enrichment's interview points appears somewhere — inconsistent headings, one-column names, phone formats, the "Left?" column, duplicates, preview-before-send, import-now-invite-later, 3,000 names, undo, and the unanswered match question.

## Delegation

None. This is one document derived from six files I have already read; splitting it across workers would cost more in reconciliation than it saves, and the judgment calls in Phases 2–4 are the deliverable. No subagents, no background workers.

## What I would report at the end

That `specs/bulk-volunteer-import/spec.md` is written in house format with seven stories, its requirements, and criteria; that the match-behaviour question the enrichment left unanswered is unresolved and sits in Open Questions with the product owner, drafted meanwhile to skip-and-report and easy to flip; that I decided the four minor items (CSV and XLSX, header required, summary contents, 2,000 rows per file) as assumptions; and that three platform constraints shaped the design and are worth a look before build — invite-later needs a volunteer that exists uninvited, which today's model does not have; the 500/day email ceiling means a 3,000-name organisation invites over about a week by design; and the 30-second request with no job runner is why imports are chunked, resumable, and capped at 2,000 rows. Plus the one thing I left out and why: the 3,000-name festival splits its file until the cap can rise.