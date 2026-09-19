FILES-READ: `README.md`, `specs/README.md`, `specs/volunteer-invite/spec.md`, `docs/glossary.md`, `docs/data-model-notes.md`, `docs/messaging-policy.md`, `requests/bulk-import-enrichment.md`

# Action plan — write `specs/bulk-volunteer-import/spec.md`

## Phase 1 — Ground the spec in the house rules (reading only, already done)

What I read and why it matters to this card:

- `specs/README.md` — the eight required sections in order, and the ID prefixes (`US-`, `FR-` with a `Source:` line, `SC-`, `A-`). My draft must match this exactly.
- `specs/volunteer-invite/spec.md` — the reference for tone and density: header block with Feature/Status/Author/Date, stories with a priority, Given/When/Then bullets, one **Independent test** line each, terse MUST-style FRs. It also already commits behaviour I must not contradict: case-insensitive email uniqueness within an organisation, 14-day invitation links, consent recorded at acceptance, and re-adding an email soft-deleted within 90 days reattaches the old record. Its Out of Scope line explicitly hands bulk import to this spec.
- `docs/data-model-notes.md` — field list and formats (`first_name`/`last_name` required, email lower-cased and unique per org, phone in E.164 converted from national format using the org's country, ≤20 tags of ≤40 chars, 2,000-char notes, statuses `invited`/`active`/`inactive`), the 5,000-volunteer Community-plan ceiling, and three hard engineering facts I have to design around: nothing today creates a volunteer without sending the invitation; writes run inside the browser request with a 30-second gateway cut-off that leaves partial writes committed; there is no background job runner and nothing resumes a cut-off request.
- `docs/messaging-policy.md` — 500 outbound emails per organisation per rolling 24 hours as a hard ceiling, the invitation as the only permitted email to a non-accepted person, and unsubscribe moving a volunteer to `inactive`.
- `docs/glossary.md` — vocabulary I will use verbatim (organisation, coordinator, volunteer, invitation, consent, tag, availability).

Nothing further to read; the workspace is six files and I have all of them.

## Phase 2 — Resolve the conflicts before drafting

The enrichment asks for things the current system cannot do as described. I work these out first, on paper, because each one changes the story list.

1. **"Import now, invite later" vs "nothing creates a volunteer without sending the invitation."** The interview asks for this twice (first-aiders first; check the list with the committee). I resolve it by introducing a state where a volunteer record exists with no invitation sent and `invited_at` empty — call it *imported*, distinct from `invited`. This is consistent with the messaging policy: such a person has no consent, and the only email they can ever receive is the invitation, which has not been sent. I write it as an assumption and also raise the record-shape change as an open question for engineering, since it adds a status value.

2. **800 and 3,000 names vs 500 emails per day.** Inviting a whole import in one go is impossible, not merely slow — and the policy note says a past burst cost a week of deliverability for every customer. So import and invitation are separate acts in this spec, and invitations go out in coordinator-chosen groups that the system caps against the same 500/24h organisation budget shared with all other Rota email. The 3,000-name festival takes at least six days to invite fully; I state that in the spec rather than hide it.

3. **3,000 rows vs a 30-second request with no job runner and no resume.** This is the real risk in the card. I design the import as a run made of committed chunks: an import run record is created first, rows are written in chunks each well inside the 30-second window, every chunk records its progress against the run, and a run whose tab was closed or whose request was cut off can be resumed or abandoned from where it stopped — never silently half-done. I flag in the spec that this is a workaround for the absent job runner and put "build a background runner instead" as an engineering-owned open question, because it would simplify the design and lift the file-size cap.

4. **"Can they undo it?"** Undo is possible for records the run created, via the existing soft delete — but it cannot unsend email. So undo is scoped: it removes only volunteers created by that run who have not accepted, it is available for a bounded window, and it is refused (with an explanation) for anyone already active. I state that plainly rather than promise a clean reversal.

5. **The collision question the interviews did not settle.** When a row's email already belongs to a volunteer in the organisation: update, skip, or ask. Two coordinators said "update" on the assumption the spreadsheet is newer, which the enrichment itself says is not always true; one said "ask". This is the one decision I will not take alone — it can overwrite real data with stale data. **This is my stop point.** In a live engagement I would put it to whoever owns the coordinator research, asking: do we overwrite from the file, leave the existing record untouched, or make the coordinator choose per row? Since I cannot wait for an answer here, I proceed on the least destructive default — **never overwrite; skip the row, count it, and show it in the preview and the summary** — write it as an assumption, and record it in Open Questions with the owner named. Onward branches, so whoever answers knows what changes:
   - **Ruled "update":** the skip requirement becomes an overwrite requirement for non-empty file fields only, the preview gains a before/after comparison per affected row, and undo must also restore overwritten values — which the soft-delete mechanism does not currently do, so it adds an engineering question.
   - **Ruled "ask":** the preview gains a per-row resolution step (keep/replace) with a bulk apply, and an import cannot proceed until every collision is resolved; row cap pressure rises for the 3,000-name case.
   - **Ruled "skip" (my default):** the draft stands unchanged.

6. **The four items the enrichment hands to me as minor.** I decide these myself as numbered assumptions: accept CSV and Excel (`.xlsx`), with Google Sheets covered by exporting to CSV; require a header row in the first line, auto-match the common headings seen in the interviews ("Email", "E-mail address", "Contact") and show the mapping for the coordinator to correct; cap a single file at 5,000 rows to match the Community-plan volunteer ceiling; and specify the summary's contents explicitly (created, skipped-as-duplicate, skipped-as-existing, rows with problems and why, how many are awaiting invitation, and the undo deadline).

## Phase 3 — Draft the spec

I write one file: `specs/bulk-volunteer-import/spec.md`. No other file changes — I do not touch `specs/volunteer-invite/spec.md` (its Out of Scope line already anticipates this spec and stays accurate), the docs, or the request.

Header: Feature *Bulk volunteer import*; Status *Draft*; Author *analyst seat*, matching the reference example; Date *9 September 2026*.

Overview: coordinators arriving with 50–3,000 names in a spreadsheet; what changes is that the list gets into Rota in one pass, is checked before anything is sent, and invitations go out afterwards in groups the coordinator controls.

Stories I plan to write, each with Given/When/Then and an independent test:

- **US-001 Upload a file and map its columns (P1)** — mixed headings, one-column full names, national-format phone numbers, a `Left?` column the coordinator can map or ignore.
- **US-002 See exactly what will happen before it happens (P1)** — the preview: counts, per-row problems, duplicates inside the file, emails already in the organisation, and an explicit statement that no email is sent at this stage. This is the "I do not want 800 people getting an email" requirement.
- **US-003 Import without inviting (P1)** — records created in the imported state, nothing sent, coordinator can show the list to the committee.
- **US-004 Invite in groups within the daily ceiling (P1)** — select a subset (first-aiders first), the system enforces 500 per rolling 24 hours across all organisation email and tells the coordinator when the rest will go.
- **US-005 Finish a large file without losing work (P1)** — 3,000 rows, chunked and resumable, a closed tab does not strand a half-written import.
- **US-006 Undo an import (P2)** — bounded as above, with a clear message about anyone already invited or accepted.
- **US-007 Receive a summary of what the import did (P2)** — fixed contents, so it is testable.

Functional requirements, each carrying `Source:` back to a story: accepted file types and row cap; header row and column mapping including the single-name split; email lower-cased and de-duplicated case-insensitively within the file; existing-email handling per the default above; phone conversion to E.164 using the organisation's country, with unconvertible numbers dropped rather than blocking the row; tags respecting the 20/40 limits and notes the 2,000-character limit; rejection of rows missing a required field, listed in the preview and summary rather than silently dropped; no email of any kind sent during import; invitation sends capped at 500 per organisation per rolling 24 hours, counted against all Rota email for that organisation; an email soft-deleted within the last 90 days reattaching its existing record rather than creating a duplicate, matching FR-002/US-001 of the invitation spec; refusing an import that would push the organisation past its plan's volunteer ceiling; the import run record with committed chunks and resume; undo scope and window; unsubscribed/`inactive` volunteers never re-invited by an import.

Success criteria: an organisation that uploads a file schedules its first shift within seven days; a stated share of started imports reach a finished state rather than being abandoned mid-run; an 800-row file completes in a single sitting; zero instances of an organisation exceeding the 500/day email ceiling.

Assumptions: the six or seven decisions from Phase 2 that were mine to take, each phrased so a reviewer can overturn it in one line.

Open Questions, with owners named: the existing-email policy (product / whoever owns coordinator research); whether adding an *imported* status is acceptable and how it counts against the plan ceiling and billing (engineering); background job runner versus the chunked-and-resumable workaround, which would relax the row cap (engineering); whether a `Left?`-style column should ever be honoured automatically or always left to the coordinator (product).

Out of Scope, with reasons: re-importing to update existing volunteers (that is sync, and it depends on the unanswered collision question); a live Google Sheets connection; importing availability or shift history; SMS or any channel other than email; and raising the 500/day ceiling, which is the provider's rule and not ours to change.

## Phase 4 — Check it before reporting

There is nothing executable in this workspace — no code, no test runner, no package manifest — so there is no suite to run, and I will not claim one. The "independent test" lines in the spec are manual verification recipes for whoever builds the feature, in the same style as the reference spec. My own check is a read-through against three things: `specs/README.md`'s eight sections present and in order with correct ID prefixes; every `FR-` naming a real `US-` in its `Source:` line and every story covered by at least one FR; and no requirement contradicting the messaging policy's 500/day ceiling and invitation-only rule, the data-model notes' formats and limits, or the invitation spec's existing behaviour. I would also confirm no number in the spec (500, 5,000, 90 days, 14 days, 20 tags, 2,000 characters) was invented rather than taken from the docs.

## Delegation

None. This is a single-document authoring job on a six-file workspace that I have read in full; splitting it across workers would cost more in briefing and cross-checking than it saves, and the hard part is reconciling constraints that live in three documents at once.

## What I report at the end

The file written; the seven stories and their priorities; the three system limits that shaped the design (invitation-on-create, 500 emails per day, the 30-second request with no job runner) and how each is handled; the collision question I declined to settle alone, the default I drafted under, and what changes for each possible ruling; the assumptions I took on the enrichment's four "minor" items so they can be waved through or corrected; and the two engineering questions that could materially simplify the build if answered yes.