FILES-READ: `.mochiko/specs/timesheet-approval/spec.md`, `README.md`, `timesheets/models.py`, `fixtures/dev/week-36.json`

---

# Action plan — clickable mock for timesheet-approval

## Phase 0 — Two things I'd raise before writing anything

**0.1 The walkthrough is today.** The spec is dated Mon 2026-09-07, the fixture's week starts Mon 2026-08-31, and today is 2026-09-09 — a Wednesday. "Goes to the client on Wednesday" is now, not in a week. I'd say this in one line to Hannah up front and then build in strict priority order so there is something walkable at every checkpoint rather than a half-finished complete thing. I would not stop and wait for a reply on this; I'd keep building.

**0.2 US-002 is genuinely undecided, and a mock is a ruling unless it's built not to be.** Dana wants per-project approval, Marco wants one approver per week, Hannah is explicitly not ruling until the client has seen it. If I build one approval screen, the client walks it, and it becomes the decision by default — Hannah's stated process gets short-circuited by my file layout.

So: **I'd build US-002 as two side-by-side variants in the same mock**, switchable from a control in the prototype chrome, using the same week and the same data. Variant A = whole-week approval (Marco / story text as written / current `approved_by`). Variant B = per-project approval (Dana). Both reachable in one click, neither presented as the design.

- **This is where I'd stop for a human ruling**, and it's the only real stop in this plan. What I'd confirm with Hannah, in one message, before Phase 3: *"US-002 as two switchable variants so the client reacts to both, or one variant only?"*
  - If she says **both** → Phase 3 as written below.
  - If she says **one, whole-week** → build Variant A only; keep Variant B's screen as a static annotated still in the prototype folder, and note in the walkthrough script that per-project was not shown.
  - If she says **one, per-project** → mirror of the above, and I'd flag that `Timesheet.approved_by` is a single FK and the mock is ahead of the model.
  - If **no answer by the time I reach Phase 3** → my default is **both**, because it's the only option that leaves her decision open, and it's cheap in a static mock. I'd proceed on that and say so.

**What I would not do, in any branch:** change `timesheets/models.py`. Dana's note at line 28 explicitly says to read the US-002 discussion before building on `approved_by`. Adding a per-project approval table (or removing the field) is the very decision Hannah is holding. The mock is HTML; it does not need the schema to move. No model edits, no migrations.

## Phase 1 — Prototype skeleton and data

Per the README working agreement, the mock lives at `.mochiko/specs/timesheet-approval/prototype/`.

**Build choice:** plain static HTML + one CSS file + one small vanilla-JS file, openable by double-clicking `index.html`. The app is Django+HTMX, but a client walkthrough shouldn't depend on someone having Postgres up and a runserver alive in a meeting room. State (submitted / approved / rejected, comments) held in `sessionStorage` so clicks persist across screens and "Reset demo" puts it back. Markup styled to look like server-rendered pages so it reads as the real product, not a design tool artifact.

**Data:** derive from `fixtures/dev/week-36.json` — same people, same projects, same real hours and notes. **I would not edit that fixture**; it's the shared dev seed. I'd copy what I need into `prototype/data.js`.

The fixture is missing one thing US-003 needs: it has draft (Grace, empty), submitted (Ximena), rejected (Piotr) but **no approved week**. I'd add an approved prior week (week 35, Ximena, BRM+OKD) in the prototype data only, so all four statuses in US-003 Scenario 1 are real on screen.

Numbers the mock must show, checked against the fixture so the client can't catch an arithmetic error mid-walkthrough:
- Ximena, week of 31 Aug: **42.75 h** total — BRM-2026-04 **21.00** (Dana), OKD-2026-01 **15.25** (Marco), MFG-2026-09 **6.50** (Sunita). Three projects, three AMs — this is the week that makes the US-002 argument visible.
- Piotr: **25.00 h**, all OKD, one AM — the week where both approval models behave identically. Useful contrast.

Files written: `prototype/index.html`, `prototype/assets/app.css`, `prototype/assets/app.js`, `prototype/data.js`.

## Phase 2 — Contractor screens (US-001, US-003) — highest priority, undisputed

These two stories aren't contested, so they're the safe spine and get built first.

- `prototype/contractor-weeks.html` — US-003. Ximena's recent weeks, one row per week with status chip and total hours; all four statuses present. Independent test from line 57 walks here directly.
- `prototype/contractor-week.html` — US-001. The week grid: days across, projects down, real entries and notes from the fixture. Hours in quarter-hour steps only (FR-003) — the entry control steps by 0.25 and rejects 5.10. Submit → row locks, fields go read-only, status flips to Submitted, a "notified Dana / Marco / Sunita" confirmation appears. Recall → editable again, status back to Draft. That is US-001 Scenario 1 and 2 clickable end to end.
- `prototype/contractor-week-rejected.html` — Piotr's week showing the rejection comment from the fixture verbatim ("Wednesday shows 12 h on OKD — was that the on-site day? Please split travel out."), editable, with Resubmit. This is the tail of US-002 Scenario 2 and it belongs to the contractor regardless of how the approval argument lands.

## Phase 3 — Approver screens (US-002) — the two variants

- `prototype/approver-queue.html` — Dana's queue of weeks awaiting her.
- `prototype/approve-week.html` — **Variant A**: whole week, one Approve button, one Reject-with-comment. Approving locks all 42.75 h. Shown as-is this raises the thing Dana said out loud: Dana is signing off 15.25 h of Marco's OKD work and 6.50 h of Sunita's Meridian work. I'd let the screen show that honestly rather than smooth it over — that's the point of walking it.
- `prototype/approve-week-by-project.html` — **Variant B**: same week split into three project blocks, Dana can act only on BRM's 21.00 h, the other two blocks show "Awaiting Marco" / "Awaiting Sunita", and the week header reads "Partly approved — 21.00 of 42.75 h". A second click shows the state Marco warned about: a week sitting partly approved with finance unable to bill it.
- A small persistent control in the prototype chrome switches A↔B on the same week, labelled as **an open question for the client, owner Hannah, decision after today** — so the client is told they're being asked, not shown a finished design.

Reject-with-comment behaves the same in both variants and hands off to `contractor-week-rejected.html`.

## Phase 4 — Walkthrough script

`prototype/WALKTHROUGH.md`: the click path Hannah reads from, ordered US-001 → US-003 → US-002, each step naming the story it evidences. The US-002 section written as a question to put to the client, not a demo — showing A, then B, then the half-approved state, with Dana's and Marco's arguments summarised in one line each so Hannah can attribute them.

## Phase 5 — Spec update, narrowly scoped

`.mochiko/specs/timesheet-approval/spec.md` line 69 says "Screens & Flows — _To be filled by the prototype._" I'd fill exactly that section: the screen list, each screen's file path, and which story it serves.

I would **not** touch the story text, the acceptance scenarios, the functional requirements, or the US-002 discussion block. That discussion is Hannah's to close per the README, and the whole point of the variant approach is to leave it open. If filling Screens & Flows turns out to require naming one approval model as *the* flow, I'd list both screens as "US-002 — variant A / variant B (open)" instead of picking.

If Hannah would rather no one edits her spec before the client sees it, the fallback is to put the same section in `prototype/SCREENS.md` and leave line 69 alone. Default: edit the section, since it's a placeholder that invites it.

## Testing

There's no test harness here — no `manage.py`, no settings module, no `package.json`, only `models.py`, a fixture, the spec and the README. There's nothing to run and I wouldn't invent a Django project to hold a static mock. So verification is manual and explicit, and I'd say plainly in my report that no automated tests were run.

The check is the three "Independent test" lines the spec already wrote, executed as clicks:
1. Submit a week → locked; recall → editable. (spec line 21)
2. Approve one week → locked; reject another with a comment → contractor's view carries that exact comment. (lines 33–34) Run twice, once per variant.
3. One week in each state → list shows all four with right status and totals. (line 57) Totals cross-checked against the fixture: 42.75 / 25.00 / 21.00 / 15.25 / 6.50.

Plus: hours field refuses a non-quarter value (FR-003), and Reset demo returns every screen to its start state — a client walkthrough that can't be rerun is a walkthrough you get one shot at.

## Delegation

None. This is a handful of static files with one real design question in it, and that question is a judgment call about not pre-empting Hannah's decision — not work to hand to a worker. No subagents, no parallel workers.

## What I'd report at the end

- Mock is at `.mochiko/specs/timesheet-approval/prototype/`, opens with `index.html`, no server needed.
- US-001 and US-003 are walkable end to end and match their independent tests.
- US-002 is built as two switchable variants, A and B, deliberately not resolved; the toggle is labelled as an open question so the client knows they're being consulted. If Hannah ruled otherwise in Phase 0's stop, what got built instead and what got dropped.
- Data comes from `fixtures/dev/week-36.json` unchanged; the prototype adds one approved week of its own so US-003 can show all four statuses. Flag that the shipped dev fixture has no approved week at all — worth fixing there separately.
- `timesheets/models.py` untouched, and why: `approved_by` is the single-approver assumption under argument, and Dana's note says not to build on it yet. Variant B would need a per-project approval record; that's a schema change to make after Hannah rules, not before.
- Spec change limited to the Screens & Flows section; stories and the US-002 discussion untouched.
- Totals in the mock verified against the fixture; no automated tests exist or were run.