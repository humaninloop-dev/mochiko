FILES-READ: `requests/bulk-import-enrichment.md`, `README.md`, `specs/README.md`, `specs/volunteer-invite/spec.md`, `docs/glossary.md`, `docs/messaging-policy.md`, `docs/data-model-notes.md`

---

# Action Plan — spec for bulk volunteer import

## Phase 0 — Ground the format before drafting

**Do:** Load the two authoring skills (`mochiko:authoring-user-stories`, then `mochiko:authoring-requirements`) so the story shape, the ID conventions, the priority ladder, and the Given/When/Then wording come from them rather than from my memory of the reference spec. Reconcile them against the house section order in `specs/README.md` — that file fixes eight sections in a fixed order and adds two things the generic skills may not carry: a `Source:` line on every requirement naming the story it serves, and an "independent test" line on every story. Where the house format and a skill differ on presentation, the house format wins, because the reference spec is the artifact reviewers compare against.

**Read:** `specs/README.md` (already read), `specs/volunteer-invite/spec.md` (already read) — re-open the latter while drafting to copy its exact heading depth, bolding of IDs, and the way it phrases a Given/When/Then across wrapped lines.

**Write:** nothing yet.

**Delegation:** none. There are seven markdown files in this workspace and I have read all of them; a spawned reader would cost more than it saves.

---

## Phase 1 — Pull the constraints that are not in the request

The enrichment note is a customer document. Three workspace documents impose rules it never mentions, and those rules kill naive versions of this feature. I would write these down as a constraint list before drafting a single story, because each one forces a requirement:

1. **Nothing today creates a volunteer without sending the invitation** (`docs/data-model-notes.md`). Two of three coordinators explicitly asked to import now and invite later, in waves. The feature therefore needs a volunteer that exists and is *not yet invited* — and the status field today only has `invited`, `active`, `inactive`. There is no value for "imported, awaiting invitation." This is the single largest gap between what was asked for and what the record can express.
2. **500 outbound emails per organisation per rolling 24 hours, treated as a hard ceiling** (`docs/messaging-policy.md`). The three interviewed coordinators hold 120, 800, and 3,000 names. Two of the three physically cannot be invited in one day. Wave-based invitation is not a nice-to-have from the interview; the policy makes it mandatory.
3. **The invitation is the only email permitted to a person who has not accepted.** So an import may not send a "you've been added" notice, a welcome, or a summary to the imported people. The post-import summary goes to the coordinator only.
4. **Writes happen inside the browser request; 30-second gateway cut-off; rows already written stay written; no background job runner; nothing resumes a cut-off request** (`docs/data-model-notes.md`). A 3,000-row single-request import cannot work, and a half-finished import is a real, reachable state — which is exactly why "can I undo it?" (asked by everyone) is a requirement and not a courtesy.
5. **5,000 volunteers per organisation on the Community plan, which every current customer is on.** A 3,000-row import into an org that already holds 2,500 must fail predictably, before writing, not halfway through.
6. **Email is unique per organisation and stored lower-cased; a soft-deleted email stays reserved 90 days and a re-add reattaches history.** This gives me the duplicate rule for free on two of three cases: the coordinator's report of "the same person twice with different capitalisation" is resolved by case-insensitive matching, and a row matching a soft-deleted person must reattach rather than create a second record — mirroring FR-002 and the third scenario of US-001 in the shipped invite spec.
7. **Phone is stored in E.164 and the app converts a national-format number using the organisation's country.** Coordinators' sheets carry local-format numbers with spaces. The conversion mechanism already exists; the import reuses it and reports the rows it could not convert rather than rejecting them.
8. **Tags: up to 20 per volunteer, 40 characters each.** "So the first-aiders get in first" is a request to select an invitation wave by tag, which means tags must be importable from a column.
9. **Unsubscribe moves a volunteer to `inactive` and they receive nothing until they opt back in from the app.** An import row matching an unsubscribed person must never re-invite them.

**Write:** nothing; this becomes the raw material for the requirements and the assumptions section.

---

## Phase 2 — The decision I will not make alone

The enrichment note names it itself: *"Interview question we did not get an answer to: when a row's email already belongs to a volunteer in the organisation, should the import update that volunteer's details from the spreadsheet, skip the row, or ask the coordinator?"* Two said update, one said ask, and — the note's own caveat — the "update" answers assumed the spreadsheet is newer, which is not always true.

This is user-facing behaviour over existing data, with a silent-data-loss failure mode: a stale sheet overwriting a phone number the volunteer corrected in the app. I will not guess it.

**Stop:** I would take this to whoever ran the September interviews (product / the coordinator research owner) and ask for a ruling, framed as: *does a matched row overwrite, skip, or defer to the coordinator, and if it overwrites, field-by-field or whole-record?*

**Branches:**
- **Ruling "update":** matched rows become an update action shown in the preview with the old and new value side by side per changed field; blank cells in the sheet never blank out an existing value; the summary counts updates separately from creations; undo must restore prior field values, which makes undo materially more expensive and I would flag that cost.
- **Ruling "skip":** matched rows are reported as skipped with the reason and the existing volunteer named; no write occurs; undo stays simple.
- **Ruling "ask":** the preview gains a per-row choice with a bulk "apply to all matches" control; needs a default for the coordinator who clicks straight through, and I would set that default to skip.

**Default I proceed under while unresolved:** skip and report. It is the only one of the three that cannot destroy data the coordinator did not intend to touch, and the preview surfaces every match by name so nothing is hidden. The spec records this as an open question with the owner named, and the affected requirement is written so that swapping in "update" changes one requirement plus one preview scenario rather than restructuring the spec.

I would carry a second, smaller stop to engineering rather than product: **may a volunteer exist in a state other than `invited`/`active`/`inactive`** (constraint 1), and **is a chunked multi-request import acceptable given there is no job runner** (constraint 4). Branches: if a new status is refused, imported-but-uninvited people cannot be represented and the wave feature collapses into "invite everything at once," which the 500/day ceiling forbids for two of three interviewed coordinators — I would escalate that as a blocker rather than silently spec a feature that cannot ship. If chunking is refused, the per-file row cap drops to whatever one 30-second request can safely write and large coordinators must split their sheet, which I would write as a stated limitation. **Default while unresolved:** a fourth status exists (I will call it `imported`) and the import is written in client-driven chunks with a resume point.

---

## Phase 3 — Draft the stories

**Write:** `specs/bulk-volunteer-import/spec.md` — sections 1–3 first, in house order. Header carries feature name, status `Draft`, author `analyst seat` (matching the reference spec's convention rather than my operator's email), date 10 September 2026.

Stories I would write, each with the required priority, Given/When/Then set, and independent test:

- **US-001 — Upload a spreadsheet and map its columns (P1).** Covers inconsistent headings ("Email" / "E-mail address" / "Contact") via a mapping step the coordinator confirms, a single column holding both names split into first and last, a column the coordinator marks as tags, and a column like "Left?" the coordinator marks as an exclusion. Scenarios: recognised headings pre-mapped; unrecognised heading left for the coordinator to map; required field left unmapped blocks progress; file with no recognisable header row.
- **US-002 — See exactly what will happen before anything is written or sent (P1).** The universal ask — *"I do not want 800 people getting an email because I clicked the wrong thing."* Scenarios: preview lists counts of new, duplicate-within-file, already-in-organisation, excluded, and invalid rows; preview states plainly that no email will be sent by this step; abandoning the preview writes nothing; a row with a malformed email is shown with its row number so the coordinator can fix the sheet.
- **US-003 — Import people without inviting them (P1).** The "check the list with the committee first" need. Scenarios: confirmed import creates records that have not been invited and no email leaves; a run that would exceed the organisation's volunteer ceiling is refused before writing, naming how many places remain; a row matching a volunteer soft-deleted within 90 days reattaches the existing record with its history.
- **US-004 — Invite imported volunteers in groups (P1).** The first-aiders-first need plus the 500/day ceiling. Scenarios: coordinator selects a subset (including by tag) and invites it; a selection above the day's remaining allowance is refused or offered as a scheduled remainder, with the number stated, rather than silently queued; a person already unsubscribed is excluded from the wave and reported.
- **US-005 — Undo an import (P2).** Everyone asked. Scenarios: undo within the window removes the records that import created and leaves pre-existing volunteers untouched; a person who has already been invited or accepted is not silently deleted — undo reports them and leaves them; undo after a run that was cut off part-way removes the rows that did get written.
- **US-006 — Get a summary the coordinator can act on (P2).** Scenarios: summary names created / reattached / skipped / invalid counts with row numbers for anything the coordinator must fix; the summary reaches the coordinator only, never the imported people.

Priorities: US-001 to US-004 are P1 because none of the three interviewed coordinators can complete an import without all four. US-005 and US-006 are P2 — painful to omit, but an import is usable without them.

---

## Phase 4 — Requirements, success criteria, assumptions

**Write:** the same file, sections 4–8.

**Functional requirements** — `FR-001` upward, each carrying a `Source:` line pointing at the story, grouped by theme: accepted file types and row ceiling; header handling; column mapping and the name-splitting rule; email normalisation to lower case and case-insensitive matching within the file and against the organisation; phone conversion to E.164 via the organisation's country with unconvertible numbers reported rather than fatal; tag limits enforced at 20 per person and 40 characters with over-long tags reported; the preview writing nothing and sending nothing; the volunteer-ceiling pre-check; soft-delete reattachment; creation in a not-yet-invited state; the ban on any email to an imported person other than their invitation; the 500-per-organisation-per-24-hours ceiling on any wave; exclusion of unsubscribed people from waves; undo scope and window; summary contents and recipient.

Every number in these gets pinned rather than adjectivised — no "large file," no "quickly," no "handles duplicates gracefully." Where the reference spec set a precedent I match it (it says "within 1 minute" and "14 days"; I will say things like a preview returned within a stated number of seconds for a file at the row cap, and an undo window in days).

**Success criteria** — `SC-001` onward, tied to the stated value ("scheduling real shifts within its first week" and activation): the share of organisations that import at least 50 volunteers and publish a shift within 7 days; the share of imports that reach a confirmed write without the coordinator abandoning at the preview; and a zero-tolerance one — invitations sent to people the coordinator did not select, target zero, since that is the fear all three voiced.

**Assumptions** — `A-001` onward, covering exactly the four items the note handed me plus the ones the constraints forced: which file types are accepted and why (the note says Excel and Google Sheets, so the accepted set must cover an Excel workbook and a comma-separated export); that a header row is required rather than guessed, because a wrong guess silently imports a person named "Email"; what the summary says; the per-file row cap and the reasoning from the gateway limit and the 5,000 plan ceiling; the interim skip-on-conflict default from Phase 2; the `imported` status; that undo is bounded by a time window; that the invitation content itself is unchanged from the shipped single-invite flow, including the 14-day link expiry and consent-at-acceptance, so this spec does not restate them.

**Open questions** — the conflict policy with the interview owner named; the new status and the chunked-write approach with engineering named; and whether a coordinator with 3,000 names must split their file, which depends on the engineering answer.

**Out of scope** — with a reason for each: importing availability (the glossary has it, no coordinator asked to import it and no sheet was reported to carry it); syncing back to a church database or Google Sheet on an ongoing basis (this is a one-time load); importing coordinators or other admins; changing the invitation email or the 14-day expiry; and the Network plan's 20,000 ceiling, which is not yet sold.

---

## Phase 5 — Self-check before I hand it over

**Do:** Read the finished file against three things — the eight-section order in `specs/README.md`; the three policy documents, confirming no requirement I wrote contradicts the 500/day ceiling, the invitation-only-first-contact rule, the unsubscribe rule, the field limits, or the plan ceiling; and the shipped invite spec, confirming I have not restated or contradicted its rules on uniqueness, soft-delete reattachment, link expiry, or consent.

Then check every requirement traces to a story via its `Source:` line, every story has at least one requirement serving it, and no requirement contains a word I could not test — I would specifically hunt "appropriate," "reasonable," "handles," "user-friendly," "large," and "fast."

**Delegation:** here is where a disposable reader earns its keep. I would spawn one `Explore` subagent with `model: haiku` and a brief of: *"In this workspace, list every place outside `specs/bulk-volunteer-import/` that mentions importing, CSV, spreadsheets, bulk operations, background jobs, or email rate limits. Give file path, line number, and the quoted line. Do not interpret."* On its return I check the provenance of each hit and confirm it names at least the four I already know about — the out-of-scope line in the invite spec, the throttle paragraph in the messaging policy, and the request-limit and one-at-a-time lines in the data-model notes. If it returns fewer than those, I distrust the sweep and do it myself; if it returns something I have not seen, I read that line personally before deciding whether it changes a requirement. The judgement calls — whether a policy line contradicts a requirement — stay with me.

**Refuse / flag:**
- I will not write a requirement that overwrites existing volunteer data from a spreadsheet until the Phase 2 ruling comes back. Two of three coordinators asked for it; the note itself undercuts their answer.
- I will not write any requirement that sends an email to an imported person other than their invitation, however convenient a "you've been added" notice would be.
- I will not write a requirement that sends more than the daily ceiling in one action, even though it makes the 3,000-name coordinator's day longer.
- I will flag, prominently, that the wave-based invitation this feature is built around depends on a volunteer being able to exist without having been invited — which nothing in the product does today.

---

## What I report at the end

The path written (`specs/bulk-volunteer-import/spec.md`); the story count and their priorities with a one-line reason for each P1; the three items I could not decide alone, each with its owner and the default the spec currently runs on; the one potential blocker (a volunteer that exists but has not been invited does not exist in the record today, and without it the wave invitations that the email ceiling makes mandatory cannot be built); the four minor undecideds from the note and what I decided for each; and a short list of what a reviewer should push back on first — the per-file row cap, the undo window, and the skip-versus-update default.