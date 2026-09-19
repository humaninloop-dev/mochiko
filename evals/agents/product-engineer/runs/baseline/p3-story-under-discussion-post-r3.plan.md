FILES-READ:
- `README.md`
- `.mochiko/specs/timesheet-approval/spec.md`
- `timesheets/models.py`
- `fixtures/dev/week-36.json`

---

# Action plan — clickable mock for `timesheet-approval`

**Deadline context:** today is Thu 2026-09-10; the client walkthrough is Wed 2026-09-16. Ordering below is deliberately "walkable end-to-end early, refine after" so that a slip loses polish, not coverage.

**Destination (from the repo's working agreements):** `.mochiko/specs/timesheet-approval/prototype/`. The manifest's home is the spec's own `## Screens & Flows` section, which currently reads `_To be filled by the prototype._`.

---

## Phase 0 — Load the procedure and confirm the ground

**Do:**
- Load `mochiko:authoring-prototype` first. It is the source of truth for the manifest's shape and the prototype's structural invariants; everything I write below about file layout and manifest columns is subordinate to it, and I would reconcile to it before writing a single file.
- Re-read the three source artefacts I have already opened, with a specific question each: the spec for scenario boundaries, `models.py` for what the structure actually is, `week-36.json` for the honest shape of data.

**Delegation:** one disposable `Explore` subagent, `model: haiku`.
- *Brief:* "List every file in this workspace including hidden files and directories, excluding `.git`. For each, one line: path, and whether it is CSS, a template, or an image. Do not read file contents beyond what is needed to classify."
- *Why:* my glob returned only four files; I want to be sure there is no existing stylesheet, template partial, or design-token file I should be borrowing from before I invent my own visual language.
- *On return I check:* whether any `.css`, `.html`, or `templates/` path exists. If yes, I read it myself and build the mock from its tokens and component language at rough fidelity. If no (my expectation), I note in the findings that **no design system exists to borrow from**, and the mock's looks are advisory only — plain system fonts, borders, no brand colours — so that nobody at the walkthrough mistakes a styling choice for a decision.

**Refuse here:** nothing yet.

---

## Phase 1 — Decide how to handle the story that is still being argued (the crux of this card)

US-002 is under discussion. Dana wants per-project approval; Marco wants one approver per week; Hannah has explicitly not ruled and wants the decision *after* Wednesday. `models.py` currently encodes Marco's reading — a single `approved_by` per `Timesheet` — with Dana's own NOTE warning against building on it.

**My position:** the mock is the instrument that settles this argument, so it must not quietly pre-settle it. I will **not** pick a winner and render it as if it were the story. I will render **both readings of US-002 as two labelled variants of the same review screen**, reachable from the same place, each carrying a visible "open question — not decided" band. Building both is not inventing scope: both are readings of the sentence Hannah already wrote ("a submitted timesheet"). Building only one would be the editorial act.

**Stop for a human decision — what I would put to Hannah, before building:**
> "US-002 has two live readings. I can give you (a) both, side by side, each labelled as undecided, so the client's reaction is the ruling; (b) one default with the other described in the findings only. (a) costs me roughly one extra screen and makes the walkthrough do work for you. Which do you want, and do you want the variant labels visible to the client or hidden behind a presenter toggle?"

**Branches:**
- *She says both (my default, and what I proceed on):* build variant A (whole-week approval, matching today's model) and variant B (per-project sign-off), both labelled, both walkable.
- *She says one default:* build variant A only — because it is what `models.py` already supports and what Marco says the email thread does today — and write variant B up in the findings with a sketch of what it would cost, including the model change.
- *She says hide the labels:* keep both variants but route them from a presenter-only entry page so the client sees one coherent flow at a time; the labels move into the manifest and the walkthrough notes.
- *She rules the question outright before I build:* drop the losing variant entirely, and record the ruling in the findings so the spec's discussion block can be closed.

**What I flag regardless:** variant B cannot be built on the current data model. One `approved_by` per week cannot hold three AMs' signatures, and there is no status value for a week that is half signed — which is precisely Marco's objection made concrete. The mock will *show* a half-approved week; the findings will say plainly that showing it required inventing a state the model does not have.

---

## Phase 2 — Build the navigation skeleton (before any screen content)

**Do:** stand up the frame first so screens fill into something stable.

**Write:**
- `.mochiko/specs/timesheet-approval/prototype/index.html` — the entry point. A short orientation paragraph (what this is, what it is not, that it is disposable), a persona switcher (Ximena / Piotr / Grace as contractors; Dana / Marco as account managers), and a list of the flows keyed to their scenario IDs so a reader can jump straight to "US-001 Scenario 2" and walk it.
- `.mochiko/specs/timesheet-approval/prototype/assets/proto.css` — one small hand-written stylesheet. System font stack, greys, 1px borders, no shadows, no brand colour, a visible "LOW-FIDELITY MOCK" band at the top of every page and an "OPEN QUESTION" band variant for the US-002 screens. Roughness is deliberate: nothing here should read as a visual commitment.
- No JavaScript beyond, at most, a dozen lines for the persona switcher and the per-project sign-off toggle — and only if plain links cannot do it. Preference is plain links between static pages.

**Refuse:** any framework, package manager, build step, CDN link, or font download. A reader opens `index.html` from the filesystem and it works, offline. If it needs installing, it has already failed.

---

## Phase 3 — Contractor screens (US-001, US-003)

**Do:** build the contractor half of the walk, using the fixture's real names, project codes, rates and note lengths verbatim. The fixture's own note — *"8–22 entries across 1–4 projects; 60% of submitted weeks span more than one account manager"* — is the cardinality contract, and I will hold the mock to it rather than showing a tidy three-row week.

**Write (all under `.mochiko/specs/timesheet-approval/prototype/`):**
- `contractor-weeks.html` — US-003 Scenario 1. Recent weeks, each with status and total hours. The spec's independent test demands *one week in each of four states*, and the fixture only supplies draft, submitted and rejected — so I add one approved prior week (week 35), built to the same shape, and record the addition in the findings so nobody thinks it came from the fixture.
- `contractor-week-draft.html` — Grace's genuinely empty draft plus an editable grid. Quarter-hour increments made visible per FR-003 (a stepping control showing .00/.25/.50/.75), day columns, project rows, per-day and per-project totals, a "Submit week" action.
- `contractor-week-submitted.html` — Ximena's week, all ten entries including the 5.25 h OKD entry with the long two-clause note. That note stays at full length on purpose: it is the layout stress case, and a mock that truncates it hides a real problem. Locked for editing, marked submitted, "Recall" action present. (US-001 Scenario 1.)
- `contractor-week-recalled.html` — the post-recall state: editable again, no longer showing as submitted. (US-001 Scenario 2.)
- `contractor-week-rejected.html` — Piotr's week carrying Dana's actual comment about the 12 h Wednesday, editable, with "Resubmit". (US-002 Scenario 2, contractor side.)
- `contractor-week-approved.html` — locked for billing, showing who approved and when.

**Expected on walking it:** every one of the spec's three independent tests can be performed by clicking, with no narration needed from the presenter. That is the bar; if a step needs a spoken explanation, the screen is wrong and I fix the screen.

---

## Phase 4 — Approver screens (US-002, both variants)

**Write:**
- `am-queue.html` — Dana's list of weeks awaiting her. **This screen is not derived from any story.** US-002 opens with "Given a submitted timesheet" and never says how an approver arrives at one; the mock cannot be clickable without it. I build it because navigation demands it, mark it in the manifest as navigation-only with no source story, and raise the missing story as a finding rather than letting a whole approver-inbox concept slide in unremarked.
- `am-review-week-whole.html` — **Variant A (Marco's reading).** Ximena's full week, all three projects, one Approve and one Reject action over the whole thing. The screen makes Dana's objection visible without arguing it: the page shows Dana approving OKD and MFG rows she has no basis to vouch for.
- `am-review-week-per-project.html` — **Variant B (Dana's reading).** The same week grouped by project, each group with its own sign-off and its own AM named. Dana can sign BRM only.
- `am-review-week-half-approved.html` — the consequence of variant B: BRM signed, OKD and MFG outstanding, and a "not billable yet" marker on the whole week. This is Marco's objection made clickable, which is the single most useful page in the pack for Wednesday.
- `am-reject-comment.html` — the reject-with-comment step, whose result is the comment already sitting on `contractor-week-rejected.html`. (US-002 Scenario 2, approver side.)

**Flag, not build:** notifications. Both US-001 Scenario 1 and US-002 Scenario 1 end in "is notified" and no story says through what, or where a notification is seen. I will show a plain inert marker ("approver notified") on the transition and say in the findings that the mock is asserting an outcome it cannot honestly render.

**Refuse to build:** any finance or billing screen. FR-002 says finance bills only approved hours, but no story covers finance, so no screen for them — the absence goes in the findings instead. Likewise no login, no settings, no admin, no reporting, no bulk-approve. If it isn't in a story, it doesn't get pixels.

---

## Phase 5 — Check the thing actually walks

**Delegation:** one `Explore` subagent, `model: haiku`.
- *Brief:* "In `.mochiko/specs/timesheet-approval/prototype/`, list every `href` and `src` value in every `.html` file with its source file and line. For each that points at a local path, report whether that file exists. Report only the facts, no suggestions."
- *Why:* dead links are exactly the failure that embarrasses a live walkthrough, and it is a deterministic sweep with a right answer — the cheap tier's job, and not worth my context.
- *On return I check:* zero missing targets, and every `href` is a relative local path (no CDN, no absolute URL, nothing that needs a network). **Expected result: no missing targets, no external references.** Any external reference I strip by hand.

**Then, by hand, I walk each of the spec's three independent tests myself,** because "does this hold together as an experience" is judgement, not enumeration:
1. Submit a week → locked; recall it → editable. (US-001)
2. Approve one week → locked; reject another with a comment → the contractor's view carries that exact comment text. (US-002, through variant A and again through variant B.)
3. Open the contractor list with one week in each of four states → four statuses and four correct totals.

There is no test suite to run: this is Django + HTMX with no tests in the workspace, and a static mock has nothing to unit-test. Standing one up would be inventing a toolchain for throwaway code. The walk *is* the test, and I say so rather than implying coverage I don't have.

---

## Phase 6 — Write the manifest into the spec

**Edit:** `.mochiko/specs/timesheet-approval/spec.md`, replacing `_To be filled by the prototype._` under `## Screens & Flows` — the only edit I make to Hannah's spec. I touch nothing else in it; the discussion block under US-002 is hers to close, not mine.

**Contents** (shaped by whatever `mochiko:authoring-prototype` specifies, which governs over this sketch): every screen with its file path and the story it comes from; every flow with the scenario it renders; `am-queue.html` marked explicitly as navigation-only with no source story; the two US-002 variants marked as unresolved alternatives rather than as the design.

I do **not** write a verdict on the prototype's quality into the spec. Grading my own mock is worthless — the walkthrough grades it, and Hannah rules.

---

## Phase 7 — Findings

**Write:** `.mochiko/specs/timesheet-approval/prototype/FINDINGS.md`, and repeat the top three in my closing report so they are not buried in a file nobody opens before Wednesday.

Findings I already expect to carry, from what the sources say:

1. **US-002's open question has a structural cost, not just a policy cost.** Per-project approval needs a state the model has no room for: `Timesheet.approved_by` is a single foreign key, and there is no status between `submitted` and `approved`. The half-approved screen exists in the mock only because I invented that state. Whichever way Wednesday goes, that ruling changes `models.py`.
2. **No story gives the approver a way to find work.** US-002 begins mid-air at "a submitted timesheet." The queue screen is my construction, and it should become a story or be deliberately dropped.
3. **"Is notified" appears twice with no channel and no screen.** The mock asserts it; nothing renders it.
4. **The rejection comment has no specified home in the contractor's view.** US-002 Scenario 2 says the contractor sees it; US-003 Scenario 1 lists only status and total hours. I placed it on the week detail; that placement is my choice, not the spec's.
5. **The four-state list cannot be built from the fixture.** No approved week exists in `week-36.json`, so US-003's own independent test is unrunnable against dev data. I added a week-35 approved sheet in the mock; the fixture still needs one.
6. **Submitting an empty week is undefined.** Grace's draft has zero entries and US-001 says "Given I have entries" — so the mock has a Submit button whose behaviour on an empty week no story covers.
7. **Recall has no notification.** Submission notifies the approver; US-001 Scenario 2 lets the contractor pull the week back out from under them silently.
8. **FR-002 has no story.** Finance bills from approved hours and no story describes finance doing anything, so nothing in this mock shows the step the whole feature exists to serve.

---

## What I report at the end

To Hannah, short and in this order: the mock is at `.mochiko/specs/timesheet-approval/prototype/`, opens by double-clicking `index.html`, needs no install and works offline; the manifest is now in her spec's Screens & Flows section; US-002 is rendered **both ways, both labelled undecided**, and the half-approved screen is the one to put in front of the client because it makes Marco's objection something they can look at rather than something they have to imagine; the ruling she makes after Wednesday will change `models.py`, so it is not only a policy call; and eight findings are written up, of which three (no approver entry point, no notification surface, no finance step) are missing stories rather than missing screens. Plus the standing reminder that this code is scaffolding — it should be deleted once the stories are frozen, not promoted into the Django app because it happens to exist.