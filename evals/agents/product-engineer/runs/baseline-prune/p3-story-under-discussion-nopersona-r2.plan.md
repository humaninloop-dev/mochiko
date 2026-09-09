FILES-READ: `.mochiko/specs/timesheet-approval/spec.md`, `README.md`, `timesheets/models.py`, `fixtures/dev/week-36.json`

---

# Plan — clickable mock for timesheet-approval

## Phase 0 — The thing I'd raise before writing a line

**Wednesday is today.** The spec is dated Mon 2026-09-07; today is Wed 2026-09-09. "Goes to the client Wednesday" means hours, not two days. Everything below is ordered so that if I'm cut off partway there is still a walkable artifact.

Second flag: **US-002 is contested on the exact axis the mock has to render.** Dana wants per-project sign-off; Marco wants one approver per week; Hannah is explicitly not ruling until the client has seen it. A mock has to draw *some* approval screen, so building "the" mock silently picks a winner and hands Hannah a fait accompli in front of the client — the opposite of what she said she wants.

So: **I build both approval variants behind a switch in the mock**, and leave the ruling open. That way the walkthrough is the instrument that produces the decision instead of pre-empting it.

**Stop point (non-blocking).** I'd put one message to Hannah: *"Wednesday is today — how many hours do I have? And I'm building US-002 as two switchable variants rather than picking one; say the word if you'd rather I just build Marco's."* I would **not** wait for the answer. Branches:
- No reply / "both" → proceed as written.
- "Just whole-week" → drop the per-project screens (Phase 5), keep everything else, note in `NOTES.md` that Dana's model is unrendered.
- "Just per-project" → drop the whole-week screen, same note inverted.
- "It's in 90 minutes" → collapse to Phases 2, 4, 5A, 5B only; skip the draft/edit screen and ship the queue + both approval screens.

## Phase 1 — Decide the medium (no reads needed, stating the call)

The app is Django 5 + HTMX + Postgres. I would **not** build the mock inside the Django app: it would need migrations, fixtures loaded, a running server and a database in the room. A client walkthrough needs something that opens by double-clicking on Hannah's laptop with no network.

**Static HTML + one CSS file + ~40 lines of vanilla JS**, hand-styled to resemble server-rendered Django templates (plain forms, no SPA feel), zero build step, works from `file://`. Lands at `.mochiko/specs/timesheet-approval/prototype/` per the working agreement in `README.md`.

## Phase 2 — Data for the mock

Read again in detail: `fixtures/dev/week-36.json`. I'd transcribe its real names, project codes, rates and notes rather than invent Lorem — Ximena's week is the whole argument in one row set:

| project | AM | hours |
|---|---|---|
| BRM-2026-04 | Dana | 21.00 |
| OKD-2026-01 | Marco | 15.25 |
| MFG-2026-09 | Sunita | 6.50 |
| **total** | | **42.75** |

Piotr = 25.00 h, all OKD, rejected with Marco's comment. Grace = draft, empty.

Under Marco's rule the single approver is the owner of the largest project — **Dana, at 21.00 of 42.75 h**, who says in the spec she can't vouch for OKD or MFG. The mock should let the client see that on screen and react. That's the payload of the walkthrough.

**Gap:** US-003 needs one week in each of four states; the fixture has draft, submitted, rejected but **no approved week**. I'd invent a prior week 35 (approved, Ximena, 38.50 h) and keep it **only** in the prototype's own data file. Default: do it, disclose it. I would not add it to `fixtures/dev/week-36.json` — that's dev seed data with a documented shape, not mine to extend for a demo.

Writes: `prototype/data/weeks.js` (plain JS object literal so it works over `file://`, where `fetch` of a local JSON is blocked).

## Phase 3 — Contractor screens (US-001, US-003)

Writes:
- `prototype/contractor-weeks.html` — US-003 Scenario 1. Four rows, four statuses, totals. This is the cheapest screen and it satisfies a whole story; it goes first.
- `prototype/contractor-week-draft.html` — Grace's empty week + an entry row form. Quarter-hour stepping visible in the control (FR-003), 24 h/day ceiling per the `TimeEntry` docstring.
- `prototype/contractor-week-submitted.html` — US-001 S1: fields visibly locked, "Submitted 6 Sep 17:42", inline "Dana Whitcombe notified", and a **Recall** button.
- `prototype/contractor-week-rejected.html` — US-001 S2 / US-002 S2 contractor side: Marco's actual rejection comment from the fixture rendered at the top, rows editable again, **Resubmit**.

Recall and resubmit are real clicks that navigate between these files, so the loop is walkable end to end.

## Phase 4 — AM queue

Writes: `prototype/am-queue.html`. Signed in as Dana. Shows Ximena's submitted week with a "3 projects · 2 other AMs" marker, and Piotr's as already handled. This screen is identical under both variants, so it's safe to build before the fork.

## Phase 5 — The contested screen, both ways

A header switch (**Approval model: ⓐ whole-week · ⓑ per-project**) persisted in `localStorage`, so Hannah can flip it live mid-conversation without reloading a different URL.

**5A — `prototype/am-week-whole.html`** (Marco's model). Dana sees all 42.75 h grouped by project, one **Approve week** / **Reject week** pair at the bottom. A small honest line: *"You own BRM-2026-04 (21.00 h). You are approving 21.75 h on projects owned by Marco Bellini and Sunita Rao-Fernandes."* Not editorial — just what the rule does.

**5B — `prototype/am-week-per-project.html`** (Dana's model). Same week, an Approve/Reject control per project block, with the other two blocks showing "Awaiting Marco Bellini" / "Awaiting Sunita Rao-Fernandes".

**5C — `prototype/am-week-per-project-partial.html`**. Dana has signed BRM; the week header reads "Partially approved — 21.00 of 42.75 h signed · not billable". This is Marco's objection made concrete, and it's the screen finance will have an opinion about. Building it is what makes the switch a fair test rather than a rigged one.

**5D — `prototype/am-week-approved.html`**. Terminal state, contractor-notified marker, "locked for billing" (FR-002).

## Phase 6 — Entry point and notes

Writes:
- `prototype/index.html` — start here. Role picker (Ximena / Dana), the variant switch, and a screen map keyed to story IDs so Hannah can jump straight to "US-002 Scenario 2" if the client asks.
- `prototype/NOTES.md` — what's real (everything from `week-36.json`), what I invented (the approved week 35), what is drawn but not decided (US-002 granularity, both variants, neither endorsed), and three questions to put to the client in the room: *Can an AM approve hours on a project they don't own? Is a half-approved week billable in part? Who chases the stragglers?*
- `prototype/assets/app.css`, `prototype/assets/app.js`.

## Phase 7 — Spec touch-up

`spec.md` line 69 reads "_To be filled by the prototype._" under **Screens & Flows**. I'd fill exactly that section: a screen list with relative links into `prototype/`, plus one line under US-002's discussion noting the mock renders both proposals and does not choose. **Default: yes, edit it** — it's a placeholder addressed to this work. I would not touch the story text, the requirements, or the discussion positions; those are Hannah's and the ruling is hers.

**I would not touch `timesheets/models.py`.** Dana's note at line 28 says explicitly not to build on the single-`approved_by` shape before US-002 is settled, and a static mock needs no model change. If the client picks per-project, the migration is a separate piece of work with a real decision behind it.

## Phase 8 — Verification, stated honestly

There is no test suite for a static mock and I would not stand one up under this deadline. What I'd do instead is a click-through against the spec's own "Independent test" lines:

- US-001: submit a week → locked; recall → editable. Expect: both links resolve, lock state visibly differs.
- US-002 S1: approve → locked-for-billing screen. S2: reject with comment → the contractor screen carries Marco's exact fixture text.
- US-003: the list shows four weeks, four statuses, totals 42.75 / 25.00 / 0.00 / 38.50.
- Every `href` in the prototype resolves to a file that exists; no dead ends.
- Opens from `file://` with the network off.

I can render and read the files, but **I cannot open a browser here** — so I'd report the click-through as a link-and-content audit, not as "I clicked it", and ask Hannah to open `index.html` once before she walks into the room.

## Delegation

None. This is a small, coherent, one-pass artifact with a hard deadline today; splitting it across workers would cost more in briefing and reconciliation than it saves, and the one genuinely subtle part — keeping the two approval variants symmetrical and fair — is exactly what fragments across agents.

## What I'd report back

A short note to Hannah: the prototype path and how to start it; that US-002 ships as two switchable variants with neither picked, and why; that I added an approved week 35 that isn't in the dev fixtures; that under Marco's rule the sole approver of Ximena's week is Dana at 21.00 of 42.75 h and screen 5A says so out loud; that 5C shows a half-approved unbillable week for finance to react to; the three questions from `NOTES.md` to put to the client; that `models.py` is untouched and the per-project migration is unstarted work behind her ruling; and that I audited the mock by reading it, not by clicking it.