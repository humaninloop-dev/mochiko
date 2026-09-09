FILES-READ: `README.md`, `.mochiko/specs/timesheet-approval/spec.md`, `timesheets/models.py`, `fixtures/dev/week-36.json`

---

# Plan — clickable mock for timesheet approval

## What I'm building and the one real judgment call

The mock lives at `.mochiko/specs/timesheet-approval/prototype/` (the working agreement in `README.md:14` puts it beside the spec). It covers US-001, US-002 and US-003, driven by the people/projects/hours in `fixtures/dev/week-36.json`.

US-002 is unresolved: Dana wants approval per project, Marco wants one approver per week, and Hannah is holding the ruling until after the client has seen it. The mock is what the client sees — so it should *feed* that decision, not quietly settle it. **I will build both approval variants as two reachable screens off a shared approver queue**, rather than picking one and presenting it as the design. Picking one silently would hand Hannah a ruling she said she wasn't making yet; refusing to build until she rules would miss Wednesday.

Default if I can't ask: single-approver-per-week (Marco's) is the **primary** path — it matches `Timesheet.approved_by` in `timesheets/models.py:31` and today's email thread — with the per-project variant one click away and labelled as the open alternative.

## Phase 1 — Confirm scope with Hannah (the stop)

I'd send three questions before writing anything, and start Phase 2 regardless under the defaults below since Wednesday is close.

1. **Both US-002 variants, or one?** Default: both, single-approver primary.
   - *If "just one, single approver":* drop `approve-week-per-project.html`, keep a note in the prototype README that Dana's model wasn't mocked and what it would change (approval state per project, a "partially approved" week).
   - *If "just per-project":* make that the primary screen and drop the single-approver one; flag that `approved_by` on the model can't express it.
   - *If "don't show the disagreement to the client at all":* build only the primary path with no variant toggle, and give Hannah the second variant as a separate unlinked file to open only if asked.
2. **May I fill `## Screens & Flows` in the spec?** That section says "To be filled by the prototype" (`spec.md:67-69`). Default: yes — I add screen names, the flows they cover and relative links, and touch nothing else in the spec. *If no:* the same content goes in the prototype README only.
3. **Where does it get opened?** Default: local files, opened from disk, no server, nothing deployed. I would not push this to any shared or public URL without being asked — it carries named clients (Bramblewood, Okafor Dental, Meridian Fintech) and named staff.

## Phase 2 — Prototype data

Write `prototype/assets/data.js` — a JS object transcribed from `fixtures/dev/week-36.json`, plus what the fixture lacks. I will **not** edit the shared dev fixture.

- Carry over as-is: the three contractors (Ximena Aranguren-Castellanos, Piotr Zieliński, Grace Achieng), the three AMs (Dana Whitcombe, Marco Bellini, Sunita Rao-Fernandes), the three projects and rates, and all entries verbatim including the long slot-picker note in `week-36.json:28` and Piotr's rejection comment.
- Add for US-003, which needs all four states: an **approved** prior week (week 35) for Ximena, and a **draft** in-progress week so the list isn't three-quarters empty. Grace's empty draft stays, as the empty-state case.
- Fixture note at `week-36.json:50` says 60% of submitted weeks span more than one AM — Ximena's week (BRM/OKD/MFG, three different AMs) is exactly the case Dana and Marco are arguing about, so it is the week both approval variants use.

Totals I expect the mock to display, computed from the fixture: Ximena week 36 = **42.75 h** (BRM 21.00, OKD 15.25, MFG 6.50); Piotr = **25.00 h** (all OKD).

## Phase 3 — Screens

Static HTML + one small CSS file + vanilla JS, no build step, no Django. Server-rendered Django/HTMX is the real stack, so I'll keep the markup plain and template-shaped (no framework idioms that won't survive the port).

| File | Story | What it shows |
|---|---|---|
| `prototype/index.html` | — | Persona picker: contractor (Ximena / Piotr / Grace) or approver (Dana / Marco). Banner: "Mock — no data is saved." |
| `prototype/contractor-week.html` | US-001 | Week grid, days × projects, quarter-hour entry (FR-003 — the input steps in 0.25 and rejects 3.10), running day and week totals, Submit. |
| `prototype/contractor-week-submitted.html` | US-001 S1 | Same week, locked, fields disabled, "Submitted 6 Sep 17:42 · awaiting Dana Whitcombe", Recall button. |
| `prototype/contractor-week-rejected.html` | US-002 S2 | Piotr's week, editable again, rejection comment shown at the top verbatim, Resubmit. |
| `prototype/contractor-weeks.html` | US-003 | Four weeks, one per state, each with status and total hours. |
| `prototype/approver-queue.html` | US-002 | Dana's queue: Ximena's submitted week, with the "you own 1 of 3 projects on this week" fact visible either way. |
| `prototype/approve-week.html` | US-002 (primary) | Marco's model: whole week, one Approve / Reject-with-comment, single approver named. |
| `prototype/approve-week-by-project.html` | US-002 (variant) | Dana's model: BRM rows approvable by Dana, OKD/MFG rows greyed with "waiting on Marco Bellini / Sunita Rao-Fernandes", week header reads "Partially approved — 21.00 of 42.75 h". |

Both approval screens carry a visible strip: *"Open question — one approver per week, or one per project? Not yet decided; we want your reaction."* with a link between them. That strip is the mock's job on Wednesday.

Also `prototype/README.md`: how to open it, what's faked (nothing persists; notifications are shown as a toast, not sent), which fixture the numbers come from, and the US-002 question with what to listen for.

## Phase 4 — Checks

There is no test suite here to run, and I won't claim one. Two checks:

1. **`prototype/check_totals.py`** — a standalone script that loads `fixtures/dev/week-36.json`, recomputes per-day, per-project and per-week totals, and asserts the same numbers appear in the generated HTML/data file. Expected pass: 42.75 / 21.00 / 15.25 / 6.50 / 25.00, and every hours value a multiple of 0.25 and ≤ 24 per day. This is the check most likely to catch a real embarrassment in front of a client.
2. **A written click-through checklist I actually walk**, recorded in the prototype README: every link resolves; submit → locked → recall → editable round-trips; reject-with-comment shows Piotr's exact comment; the four states in the list match their totals; Grace's empty week renders as an empty state, not a broken grid; "Ximena Aranguren-Castellanos" and "Piotr Zieliński" don't truncate or mangle their diacritics in the header, and the 200-character slot-picker note wraps instead of blowing out the row.

If anything on that list fails and I can't fix it before handing over, I say so plainly rather than shipping a "done".

## Phase 5 — Spec touch-up

Under the default from Phase 1, fill `## Screens & Flows` in `spec.md` with a short table mapping each screen file to the story and scenario it demonstrates, and a line under US-002 noting that both approval shapes are mocked and neither is a ruling. I do not edit the story text, the discussion notes, or the functional requirements — those are Hannah's, and the discussion is explicitly hers to close.

## What I would not do

- **Not touch `timesheets/models.py`.** Dana's note at line 28 says not to build on `approved_by` until US-002 is settled. A per-project approval model would need approval state on the entry or a project-week join; adding it now would be picking Dana's side in code. The mock needs no schema at all.
- **Not resolve the Dana/Marco argument**, in the mock's framing or in the spec.
- **Not deploy or share the mock anywhere off this machine** without being asked — named clients and staff.
- **No agents or subagents**; this is a few hours of hand-written HTML and I'd do it directly.

## What I'd report back

The prototype path and how to open it; the eight screens mapped to US-001/002/003; that US-002 is mocked twice, which one is primary and why; the `check_totals.py` result and the click-through checklist outcome, including anything still broken; that the dev fixture and `models.py` are unchanged; the exact edit made to the spec; and the one thing I'd ask Hannah to watch for on Wednesday — whether the client reacts to a week sitting half-approved, which is the substance of Marco's objection and the cheapest way to get the decision made.