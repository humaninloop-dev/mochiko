FILES-READ: `README.md`, `.mochiko/specs/timesheet-approval/spec.md`, `timesheets/models.py`, `fixtures/dev/week-36.json`

---

# Action plan — clickable mock for `timesheet-approval`

## What I already know from the workspace

- Output location is fixed by the working agreements in `README.md`: `.mochiko/specs/timesheet-approval/prototype/`.
- The spec has an empty `## Screens & Flows` section marked *"To be filled by the prototype"* — that is where the manifest is expected to land.
- **There is no design system.** No CSS, HTML, JS, static or template files exist anywhere in the workspace — only `timesheets/models.py`. So there is nothing to build from, and I will not invent a house look. Plain rough styling, one hand-written stylesheet.
- Real stack is Django + HTMX server-rendered. That is good news for a static mock: page-to-page navigation is honest as plain links, no client-side framework needed to be truthful about the interaction model.
- `fixtures/dev/week-36.json` is a gift — it gives me real names, real project codes, real cardinality, and an explicit shape note ("8–22 entries across 1–4 projects; 60% of submitted weeks span more than one account manager"). My placeholder data comes from here, not from my imagination.
- US-002 is genuinely unresolved, and `models.py` line 28–32 carries Dana's note pointing back at the same argument. The dispute is not cosmetic: it decides whether `approved_by` stays a single FK.

---

## Phase 0 — Load the procedure

- Invoke the `mochiko:authoring-prototype` skill and follow it as the governing spec for file layout, the screens-and-flows manifest format, screen/flow ID conventions, and the fidelity markers. Everything below about *paths and manifest shape* is my working assumption and yields to what the skill says.
- **Assumption I would correct on skill load:** that the manifest belongs inline in the spec's `## Screens & Flows` section. If the skill mandates a standalone manifest file inside `prototype/`, I write it there and leave a one-line pointer in the spec section instead.
- No reading needed beyond the four files above; the workspace is four files and I have read all of them.

## Phase 1 — Decide how to handle the unresolved story (the real decision on this card)

US-002 has two incompatible readings live in the spec:

| Reading | Owner | What the approve screen becomes |
|---|---|---|
| Whole-week approval, one approver (largest project owner) | Marco | One Approve / Reject pair on the week |
| Per-project approval, week approved when every AM signs | Dana | Rows grouped by project; an AM acts only on their own rows; week sits in a partial state |

Hannah's note says explicitly she is *not ruling until the client has seen it*. So picking one and rendering it silently would make the mock pre-empt the decision it exists to inform — and the client would sign off on whichever one I happened to draw.

**Stop I would make (and would not block on):** message Hannah with one question — *do you want both readings walkable side by side on Wednesday, or one as the strawman?*

- If she says **both** → proceed as below.
- If she says **whole-week only** → build only the whole-week approve screen, keep the per-project reading as a written finding, and note in the mock cover that one reading is unrendered.
- If she says **per-project only** → build only that, and add a finding that it contradicts `approved_by` in `models.py` today.
- If **no answer before I need to build** → **my default: build both**, clearly labelled as an open question, sharing every other screen in the frame. Both are readings of the same story, so neither is invented scope, and a switchable pair is the cheapest way to let Wednesday's room actually decide.

I will not rule on US-002 myself, and I will not touch the Discussion block in the spec — it is owner-marked to Hannah.

## Phase 2 — Screen and flow inventory, traced to stories before I write any HTML

Draft the inventory first and check every screen earns its place:

**Contractor (Ximena Aranguren-Castellanos, and Grace Achieng for the empty state)**
- `S-01` My timesheets — list of recent weeks, status + total hours ← US-003 S1
- `S-02` Week detail, draft — editable, Submit ← US-001 S1 (start)
- `S-02e` Week detail, draft, empty — Grace's zero-entry week ← US-003 S1 empty state
- `S-03` Week detail, submitted — locked, Recall ← US-001 S1 (result) / S2 (start)
- `S-04` Week detail, rejected — carries the AM's comment, editable, Resubmit ← US-002 S2 (contractor side)
- `S-05` Week detail, approved — locked for billing ← US-002 S1 (contractor side)

**Account manager (Dana Whitcombe)**
- `S-06` Approval queue — **not specified by any story**; see findings. Rendered minimally and marked in-mock as an unspecified entry point, because US-002 is not walkable without some way to reach a submitted week.
- `S-07a` Approve week — whole-week reading ← US-002 S1, variant A
- `S-07b` Approve week — per-project reading ← US-002 S1, variant B
- `S-07p` Per-project week, partially approved — BRM signed, OKD/MFG awaiting Marco and Sunita ← variant B consequence
- `S-08` Reject with comment ← US-002 S2 (AM side)

**Flows, each keyed to a scenario:**
- `F-1` US-001 S1: S-02 → Submit → S-03 (locked, "Dana Whitcombe notified")
- `F-2` US-001 S2: S-03 → Recall → S-02 (editable, no longer submitted)
- `F-3a` US-002 S1 var A: S-06 → S-07a → Approve → S-06 · contractor sees S-05
- `F-3b` US-002 S1 var B: S-06 → S-07b → approve BRM rows → S-07p → S-06
- `F-4` US-002 S2: S-07a/b → S-08 → contractor S-04 → edit → resubmit → S-03
- `F-5` US-003 S1: S-01 with all four statuses present

**Things I will deliberately not draw**, because no story asks for them: a finance/billing screen (FR-002 has no story behind it), a notification inbox or email view, a time-entry editor with quarter-hour controls (FR-003 — US-001 begins *"Given I have entries"*, so entry creation is outside every scenario), project/user admin, login. Each becomes a finding rather than a screen. Notification assertions get rendered as an inline marker on the screen where the story asserts them ("Approver notified — Dana Whitcombe"), not as an invented surface.

## Phase 3 — Placeholder data with honest shape

Derive everything from `fixtures/dev/week-36.json`, unchanged where possible:
- Ximena's submitted week: all 10 entries, three projects, **three different AMs** — this is the week that makes the US-002 argument visible, and it is exactly the case Dana describes in the spec.
- Keep the pathological content deliberately: the 220-character note on the 2026-09-02 slot-picker row, the client name "Bramblewood Landscape Architecture", the diacritics in "Piotr Zieliński" and "Ximena Aranguren-Castellanos". These are the things that break layouts, so they stay.
- Grace's zero-entry draft is the empty state, as-is.
- Piotr's rejected week carries the fixture's real rejection comment verbatim.
- For `S-01` I need one week in each of the four states for one contractor, which the fixture does not supply — I extend Ximena backwards over weeks 33–36 at the fixture's stated cardinality (8–22 entries), with totals summing in quarter-hour increments so FR-003 is not contradicted on screen.
- No tidy 8.00-hour rows everywhere; totals like 42.75.

## Phase 4 — Build the skeleton, then fill it

Write to `.mochiko/specs/timesheet-approval/prototype/`:

- `index.html` — cover: what this is, that structure and flows are the point and the visuals are deliberately rough, the US-002 open question stated in Dana's and Marco's own terms, and links into both variant walkthroughs plus the contractor walkthrough.
- `lofi.css` — one small stylesheet. Grey boxes, system font stack, visible seams. No brand colour, no shadows, no icon set, nothing that reads as a visual commitment.
- `contractor-weeks.html`, `week-draft.html`, `week-draft-empty.html`, `week-submitted.html`, `week-rejected.html`, `week-approved.html`
- `approval-queue.html`, `approve-week-whole.html`, `approve-week-per-project.html`, `approve-week-per-project-partial.html`, `reject-comment.html`
- `README.md` — open `index.html` in a browser, nothing to install; what is binding vs rough; the list of things marked as scaffolding or unspecified.

Constraints I hold to: plain `<a href>` relative links so it works from `file://`; no build step, no npm, no CDN, no framework, no JS beyond at most a few lines for the role switcher (and I would rather do the role switcher as two separate entry links and no JS at all). Two visually distinct in-page markers, defined on the cover: one for **mock scaffolding** (the role switcher, the "jump to state" links) and one for **unspecified — no story covers this** (the approval queue, and the partial-approval status label).

Order of work: skeleton and cover first, then contractor screens (F-1, F-2, F-5 — these rest on settled stories and are safe to build early), then the two US-002 variants last, so that if Hannah rules mid-week I can drop a variant cheaply rather than rework the frame.

## Phase 5 — Walk the spec's own tests

The spec supplies three independent tests; I click each one end to end in a browser and confirm it is walkable in the mock:
- *"submit a week and see it locked; recall it and see it editable"* — expect S-02 → S-03 with Submit gone and fields inert, then Recall back to S-02 with them live.
- *"approve one week and see it locked; reject another with a comment and see the contractor's view carry the comment"* — expect this to pass in variant A cleanly, and in variant B to pass only for Dana's BRM rows, landing on the partial state. That asymmetry is the finding, not a defect.
- *"with one week in each state, open the list and see all four with the right status and totals"* — expect S-01 to show draft/submitted/approved/rejected with quarter-hour totals.

Also check: no dead links, every screen reachable from `index.html`, every flow in the manifest traversable, and no screen present that the manifest cannot key to a scenario.

**Delegation here:** one disposable `Explore` subagent, `model: haiku`. Brief: *"In `.mochiko/specs/timesheet-approval/prototype/`, list every `href` target in every `.html` file and report any that name a file not present in that directory; then list any `.html` file in that directory that no other file links to."* On its return I check the two lists against my own screen inventory and spot-check two or three named files myself — I do not treat an empty report as proof, since absence here would drive a decision. Everything else in this phase is interpretive and I do it myself.

## Phase 6 — Manifest

Fill the spec's `## Screens & Flows` section (editing **only** that section of `spec.md`) with the screen and flow inventory in the format the skill defines: each screen with its ID, purpose, source story, and file; each flow with its ID, the scenario it renders, and its click path. The two US-002 variants are recorded as competing renderings of the same scenario, attributed to Dana's and Marco's positions, with a note that the section will need one of them deleted once Hannah rules. Screens with no story behind them are marked as such rather than being given a plausible-sounding source.

## Phase 7 — Findings I would report

Written up for Hannah, and echoed in the prototype `README.md` so a Wednesday reader sees the same caveats. Not written into the spec's Discussion block — that is Hannah's and owner-marked.

1. **No story covers how an AM finds work to approve.** US-002 starts from "a submitted timesheet" with no route to it. I had to invent an approval queue to make the story clickable at all; it is marked unspecified in the mock and needs a story either way the US-002 argument lands.
2. **The per-project reading needs a fifth status that US-003 does not have.** US-003 S1 enumerates draft, submitted, approved, rejected. Dana's model produces a week that is part-approved for days — Marco says as much. The contractor's list has nothing to show for it. If Dana's reading wins, US-003 changes too.
3. **The data model cannot express Dana's reading.** `Timesheet.approved_by` is a single nullable FK, and Dana's own note at `timesheets/models.py:28` flags it. Per-project approval needs approval to hang off the project/entry grouping, not the week. Worth the client knowing this is the expensive branch.
4. **The fixtures already answer part of the argument.** `fixtures/dev/week-36.json` states that 60% of submitted weeks span more than one account manager, and the one submitted week in it (Ximena's) spans three AMs across three clients. Marco's "whoever owns the largest project checks with the others" is therefore the majority path, not the exception. This is evidence for Wednesday's room; I am reporting it, not ruling on it.
5. **FR-002 has no story.** "Finance MUST bill only approved hours" cites US-002, but no story gives finance a view or an action, so there is no screen for it and nothing for the client to walk.
6. **FR-003 is unrenderable as specified.** Quarter-hour increments only show up in a time-entry editor, and no scenario covers entering or editing an entry — US-001 opens with entries already present. The mock displays quarter-hour values but never demonstrates the constraint.
7. **Notification has no surface.** Two scenarios assert someone "is notified" with no story saying where. Shown as an inline marker only.
8. **Resubmit is asserted but has no scenario of its own.** It appears in the *then* clause of US-002 S2. I have rendered it; it may want its own scenario under US-001.

## What I would report at the end

The prototype path and how to open it; the screen and flow inventory with its story keys; that both US-002 readings are walkable and labelled, awaiting Hannah's ruling; the three spec-supplied independent tests and how each behaved in each variant; the eight findings above with the first four called out as decision-relevant for Wednesday; and the explicit note that I have not graded this mock — it needs a reviewer's eyes before it goes in front of the client, and it is meant to be deleted once the stories are frozen.