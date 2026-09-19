FILES-READ:
- `README.md`
- `.mochiko/specs/timesheet-approval/spec.md`
- `timesheets/models.py`
- `fixtures/dev/week-36.json`

---

# Action Plan — clickable mock for `timesheet-approval`

## Phase 0 — Load the procedure that governs the output

**Do:** Load `mochiko:authoring-prototype` before writing anything. Everything below about *what* goes in the prototype is settled by this card; the *shape* of the manifest (fields per screen, how a flow keys to a scenario, the invariants) comes from that skill, and I follow it rather than a remembered version. If its manifest format conflicts with anything I sketch below, the skill wins and I adjust the file layout accordingly.

**Refuse/flag:** I will not invent a manifest format from memory to save a step.

## Phase 1 — Bound the workspace (delegated cheap read)

**Do:** Confirm what I already suspect from the file listing — that there is no design system, no existing templates, no stylesheet, and no prior prototype in this repo to inherit conventions from. This matters because if a token set or component library existed I'd build from its language at rough fidelity, and if a sibling prototype existed I'd match its structure.

**Delegate:** one disposable `Explore` subagent, `model: haiku`.
- Brief: "In this workspace, list every file matching: any `.css`, `.scss`, `.html`, `.svg`, any directory named `templates`, `static`, `assets`, `design`, `tokens`, or `prototype`, and any other directory under `.mochiko/specs/`. Return paths only, plus 'none' for empty categories. Do not read file contents."
- On return I check: paths are real and inside this workspace, and the answer is an enumeration not a summary. If it reports anything beyond the four files I've seen, I read those myself before proceeding — an existing design language changes what I build.
- Default if it returns nothing: plain, deliberately rough HTML/CSS of my own, system font stack, no brand colour.

**Do myself, not delegated:** the reading of `spec.md` and `models.py` — those are interpretive and already done.

## Phase 2 — Derive the screen inventory from the stories, before writing any HTML

**Do:** Walk each story and each scenario and write down the minimum surface that makes that scenario clickable. Nothing enters the inventory that a story doesn't ask for; anything the stories *imply* but never state gets marked as inferred and becomes a finding rather than a silent addition.

Inventory as I read it:

| From | Screen |
|---|---|
| US-003 S1 | Contractor's week list — draft, submitted, approved, rejected, each with total hours |
| US-001 S1 | Week detail, editable (draft) → submit action |
| US-001 S1 | Week detail, locked (submitted) |
| US-001 S2 | Recall action → week detail editable again |
| US-002 S1 | Approver's review of a submitted week → approve → approved state |
| US-002 S2 | Reject with comment → contractor's week detail carrying that comment → resubmit |

Three gaps surface immediately, and I record them as findings rather than quietly filling them:

- **No story says how an approver reaches a submitted week.** US-001 S1 says "the approver is notified"; US-002 S1 starts "Given a submitted timesheet." Something has to be clickable between those two. I will build a minimal approver landing list, label it in the mock as *inferred — no story covers this*, and raise it as a finding for Hannah.
- **No story covers entering or editing hours.** US-001 assumes "I have entries"; recall and rejection both make the week "editable again." FR-003 (quarter-hour increments) has no screen to live on. I will render the editable week detail as a read-only-looking grid with the cells visibly marked editable, and *not* build a real entry editor — that's a separate story that doesn't exist yet. Finding.
- **Notification has no surface.** "The approver is notified" / "the contractor is notified" appear twice with no screen. I'll render a flat, non-interactive "notified" stub in the flow so the walkthrough doesn't dead-end, marked inferred. Finding.
- **FR-002 (finance bills only approved hours) has no story and no screen.** Out of scope for the mock; flagged as an unrepresented requirement so it isn't assumed covered on Wednesday.

## Phase 3 — Stop point: how to render the story that's still being argued

This is the decision I will not make silently. US-002's discussion is open, owner Hannah, ruling deliberately *after* Wednesday, and the two readings produce visibly different screens:

- **Dana's reading (per project):** Ximena's week has BRM/OKD/MFG under three different AMs. Dana sees and signs only her BRM rows; the week reaches "approved" when all three have signed. This needs a partially-approved state on the week, per-project sign-off rows, and a contractor-side view of "2 of 3 approved."
- **Marco's reading (one approver per week):** the AM owning the largest project approves the whole week. One button, one state, matches `Timesheet.approved_by` as it stands today.

The current model can only express Marco's reading — `approved_by` is a single nullable FK on a week that's unique per contractor+week, and Dana has already left a note in `models.py` saying so. That's a real consequence and part of what the client's reaction should inform.

**What I would confirm with Hannah, before building the approval screens:** does the Wednesday mock show *one* approval flow (the story as literally written — whole-week), or *both* readings side by side as labelled alternatives so the walkthrough actually pulls the client's opinion out?

- **If she says both:** I build the whole-week flow as the main walk path and a clearly-marked parallel branch for per-project, reachable from the same submitted week via an explicit "Option A / Option B — unresolved, see US-002 discussion" chooser. Both branches are labelled in-screen as undecided so neither reads as a commitment.
- **If she says one, story-as-written:** I build only the whole-week flow, and add one static, non-clickable panel showing what the per-project reading would look like on Ximena's three-AM week, labelled as the open alternative. The client can still react; nothing gets rendered as a decided design.
- **If she says defer US-002 entirely:** I ship US-001 and US-003 clickable, and the approval step terminates in an explicit "approval design undecided" stop card. I'd push back once on this — a walkthrough of submission with no approval leaves the P1 story untested by the client, which is the main thing Wednesday could buy her.

**Default while she's unreachable (and what I'd proceed on):** build both, whole-week as the primary path, per-project as the labelled alternative branch. Rationale: the ruling is explicitly scheduled *after* Wednesday, so the walkthrough's job is to feed that ruling, and a mock that quietly picks one reading pre-empts it. Cost is a handful of extra static screens. I'd flag the assumption at the top of my report so a reversal is cheap.

## Phase 4 — Placeholder data with honest shape

**Do:** Take the fixture's data verbatim wherever it fits — it is unusually good and already contains the hard cases:
- Ximena's week: 10 entries, 3 projects, 3 different AMs — the exact contested case, and the one that will stress the approval screens.
- The 5.25 h OKD note is a 40-word paragraph; it stays as-is, because a one-line note would hide the row layout problem.
- Piotr's rejected week carries a real rejection comment for US-002 S2's contractor-side view.
- Grace's empty draft gives the zero-state.
- Long names (Ximena Aranguren-Castellanos, Bramblewood Landscape Architecture) stay; they're the column-width truth.
- The fixture's own note — 8–22 entries, 1–4 projects, 60% multi-AM — is my cardinality budget for anything I add.

**Do:** US-003 S1 needs one contractor with weeks in *all four* states, and the fixture is a single week across three people. I will invent two or three prior weeks for Ximena (an approved week 34, a rejected-then-resubmitted week 35) at the fixture's stated shape and cardinality, living only inside the prototype.

**Refuse:** I will not edit `fixtures/dev/week-36.json`. That's the app's dev data, not prototype scratch; extending it to serve a mock would leak throwaway data into the real seed. Prototype data lives in the prototype directory.

**Delegate (deterministic check):** one `Explore`, `model: haiku` — "In `fixtures/dev/week-36.json`, list every `hours` value that is not an exact multiple of 0.25, and give the summed hours per timesheet. Return numbers only." On return I check the arithmetic is per-timesheet and that a non-conforming value, if any, is quoted with its date and project. This is FR-003's honesty check and gives me the week totals US-003 asks me to display, so I don't hand-add them wrong.

## Phase 5 — Build the skeleton, then the screens

**Do:** Navigation frame first — a persona switcher (contractor / account manager), a persistent "LOW-FIDELITY MOCK — flows are real, visuals are not" banner, and the walkthrough index. Screens fill into that frame; I don't style a screen until the frame is stable.

**Write** (under `.mochiko/specs/timesheet-approval/prototype/`, per the README's convention):

- `index.html` — walkthrough entry: what this is, the fidelity notice, the flows listed and keyed to their scenarios, the open US-002 question stated in plain words at the top
- `style.css` — one rough stylesheet, no framework, no webfont, no build
- `contractor-weeks.html` — US-003 S1, four statuses + totals
- `week-draft.html` — US-001 S1 start
- `week-submitted.html` — US-001 S1 end: locked, submitted, approver-notified stub
- `week-recalled.html` — US-001 S2 end: editable again, no longer submitted
- `week-rejected.html` — US-002 S2 contractor side: comment visible, editable, resubmit
- `am-queue.html` — inferred landing for the approver, marked inferred
- `am-review-week.html` — US-002 S1/S2, whole-week reading
- `am-approve-done.html`, `am-reject-comment.html` — the two outcomes
- `am-review-per-project.html`, `week-part-approved.html` — the alternative reading, marked unresolved (present per the Phase 3 default; dropped or made static on Hannah's ruling)
- `screens-and-flows.md` — the manifest, in the format the skill defines: every screen, why it exists, which story it comes from, every action and where it leads, each flow keyed to its scenario, and inferred screens marked as such

Static `<a>` links between pages. No JavaScript beyond, at most, a few lines inline for the persona toggle. Nothing to install, nothing to run — Hannah opens `index.html` and clicks.

**Refuse:** no React, no Tailwind CDN, no component library, no polish that would let a rough mock be mistaken for a visual sign-off, and no screen that isn't traceable to a story or explicitly marked inferred.

## Phase 6 — Fill the spec's Screens & Flows section

**Do:** `spec.md` line 69 says "_To be filled by the prototype._" — that's an invitation, so I replace that placeholder with the screen/flow inventory (or the skill's prescribed pointer to `prototype/screens-and-flows.md`, if it specifies a link rather than an inline table). I touch only that section. I do **not** touch the US-002 discussion notes, the stories, or the FRs — those are Hannah's, and the open question stays open in her words.

## Phase 7 — Consistency check before handing over

**Do myself (completeness-sensitive):** walk every scenario in the spec and confirm it has a clickable path in the mock, and walk every screen in the mock and confirm it traces to a story or is marked inferred. Neither direction gets delegated.

**Delegate (mechanical link check):** one `Explore`, `model: haiku` — "In `.mochiko/specs/timesheet-approval/prototype/`, list every `href` in every `.html` file with its source file, and list every `.html` filename present. Return two plain lists." I diff them myself to find dead links and orphan pages. On return I check both lists are complete and file-attributed, not summarised.

**Refuse:** I do not grade this prototype. I hand it over with an explicit statement that it hasn't been reviewed by anyone but me, and Hannah or a reviewer decides whether it's ready to walk on Wednesday.

## Phase 8 — Report

What I'd tell Hannah:

1. **What's clickable:** US-001 and US-003 end to end, US-002 end to end in the whole-week reading, plus the per-project alternative as a labelled branch — with the assumption stated plainly so she can reverse it in one message.
2. **Findings, in priority order:**
   - US-002's two readings are not a wording difference — they produce different screens, different states, and different data. `Timesheet.approved_by` (a single FK on a week) cannot represent per-project sign-off; choosing Dana's reading means a schema change, and Dana has already flagged it in the model file. The mock shows both; it doesn't decide.
   - No story covers how an approver finds a submitted week. I built a minimal queue and marked it inferred — it needs a story or an explicit decision that it's out of scope.
   - No story covers entering or editing hours, so FR-003 (quarter-hour increments) has no screen and recall/rejection lead to a week that's "editable" with nothing specified about editing it.
   - Notification appears in two scenarios with no surface anywhere; rendered as a stub.
   - FR-002 (finance bills only approved hours) has no story and no screen — deliberately absent, so Wednesday shouldn't be read as having covered billing.
   - US-001 S2 (recall → editable) and US-002 S2 (reject → editable) both return a week to editable, but the model has separate `draft` and `rejected` states. Whether a recalled week and a rejected week are the same thing for the contractor isn't stated.
3. **What I didn't touch:** the fixture, the stories, the FRs, the open discussion.
4. **Disposability:** this is throwaway static HTML. It exists to be clicked, argued with, and deleted — none of it should be promoted into the Django templates.