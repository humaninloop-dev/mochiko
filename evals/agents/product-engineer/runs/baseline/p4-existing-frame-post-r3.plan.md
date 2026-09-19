FILES-READ: `README.md`, `seed/README.md`, `seed/items.csv`, `seed/loans.csv`, `.mochiko/specs/lending/spec.md`, `.mochiko/specs/lending/prototype/README.md`, `.mochiko/specs/lending/prototype/package.json`, `.mochiko/specs/lending/prototype/nav.js`, `.mochiko/specs/lending/prototype/index.html`, `.mochiko/specs/lending/prototype/screens/scr-001-catalogue.html`, `.mochiko/specs/lending/prototype/screens/scr-002-item-detail.html`, `.mochiko/specs/lending/prototype/screens/scr-003-my-loans.html`, `.mochiko/specs/lending/prototype/screens/scr-004-reservations.html`

---

# Action plan — extend the lending mock for US-004 (reserve) and US-005 (renew)

## Phase 0 — Ground the work (reading only, already largely done)

In a real run I would first load `mochiko:authoring-prototype` and follow its manifest format and screen conventions rather than my recollection of them; everything below that concerns the shape of the Screens/Flows tables would be reconciled against it. In this plan-only run I work from the manifest already present in `spec.md`.

Read (done): the two new stories and FR-002/FR-003, all four existing screens, `nav.js`, `package.json`, both prototype/repo READMEs, and both seed CSVs. A whole-workspace glob returns 13 files — **there is no design system, token file, or component library in this workspace**, so the mock keeps its existing rough house style (system font, dashed/solid grey boxes, no colour beyond the green/red availability text). I would not upgrade the visuals; that would read as a design commitment nobody made.

No delegation here — the workspace is 13 files and I have read all of them.

## Phase 1 — Fix the skeleton so a volunteer can actually open it

The repo's working agreement says the mock is opened straight from the folder by reviewers who are not developers. The mock currently violates that: `package.json` requires `npm install && npm run dev`, and every screen loads the nav via `<script type="module" src="../nav.js">`. ES modules do not load over `file://` — opened from the folder, **every screen renders with no navigation at all**. Kai introduced this on 2026-08-27 for hot reload.

- **Stop / confirm:** removing Vite reverses a teammate's deliberate choice. What I would confirm with Kai (and Rosa as the reviewer-facing owner): drop the build step entirely. **If Kai wants the dev-server iteration loop kept** → keep it as an optional convenience only if the pages also work unopened-server (i.e. nav inlined anyway), and delete `package.json` regardless once the nav no longer needs bundling. **If Rosa says reviewers are fine running npm** → I would still push back, because the README is explicit, but would keep Vite and instead make nav a classic (non-module) script. **Default I proceed under:** no build step, nav inlined.
- Write: delete `prototype/package.json` and `prototype/nav.js`.
- Write: inline the nav markup (four plain `<a>` links: Home · Catalogue · My loans · Reservations) as literal HTML into `index.html` and each screen. Six duplicated links across five files is cheaper than a nav that does not appear. Drop "(coming soon)" from the Reservations link — US-004 now exists.
- Write: rewrite `prototype/README.md` — open `index.html` in a browser, nothing to install; where the placeholder data comes from; that this is disposable.
- Any JavaScript that remains (search filter, renew, cancel, reserved-state) is plain inline `<script>` in the page that uses it — no external files, no modules, no imports, works from `file://`.

## Phase 2 — Remove scope no story asks for

`scr-002-item-detail.html` carries a star-rating and review block ("★★★★☆ · 4.2 from 17 members · write a review"), added 2026-08-26 as a "quick win". No story, scenario, or requirement in the spec mentions ratings or reviews.

- I **refuse to keep rendering it**. A mock that shows ratings is a mock that gets signed off as including ratings.
- Write: remove the block from `scr-002-item-detail.html`.
- This is reported as a finding, not silently deleted — the desk volunteers who asked for it deserve a story, not a mock element. If Rosa wants it, it comes back after US-006 exists.
- I would not add it to the manifest as "future".

## Phase 3 — Settle the placeholder data (decisions before any screen is written)

The seed says: median member holds 3, limit 8, 18% overdue "some by months", queues up to 6 deep, 7-day hold, one renewal per loan, long item names that members search on. The current `scr-003-my-loans.html` shows **one** row with a due date of "Tomorrow" — that hides every layout and state problem the real screen has.

Decisions I would make and record in the prototype README:

- **Signed-in member: Marguerite Okonjo-Baptiste** — she holds 8 of 8 in `loans.csv` (at the limit, longest name, two overdue including one 46 days overdue). Honest worst case, and her loan set happens to cover every renew state.
- **Real dates, relative to today 2026-09-10**, taken from `loans.csv`; no "Tomorrow".
- **Seed conflict — flagged, resolved:** `items.csv` marks `it_0311` (Vango tent) `on_loan=0`/available, but `loans.csv` `ln_9038` has Aisha Begum holding it until 2026-09-18. I treat the loan record as authoritative and show the tent as out. Consequence: the borrow demo item moves from the tent to the **Raclette grill for 8** (1 copy, no loan, no queue) — and the tent becomes the natural "reserve me" item (out, empty queue, position 1 of 1). Reported as a finding for whoever owns the seed.
- **One item added beyond the seed sample:** a returned item sitting on the hold shelf is needed for US-004 Scenario 2, and all twelve seed items are either held by Marguerite or unqueued. I add *"Wallpaper steamer — Earlex SS125 (2.4 L, 60 min run)"*, tools. The seed is a 12-row sample of 1,400 items, so this does not distort shape; it is called out in the README.
- **Renew state map** (loan → item queue depth from `items.csv`): Hobbit (queue 0, not renewed) and Coleman stove (queue 0, not renewed) → renewable; KitchenAid (queue 2), hedge trimmer (queue 2), ladder (queue 1) → refused, someone waiting; sewing machine (queue 1, `renewed=yes`) → already renewed once; Bosch drill and Vax carpet cleaner → overdue.

## Phase 4 — Catalogue (SCR-001), and make FLOW-001 real

`scr-001-catalogue.html` shows 6 of 12 items, and its search form has a button but **no filtering behaviour at all** — FLOW-001 in the manifest claims "type a word → results filter", which the mock does not do.

- Write `screens/scr-001-catalogue.html`: all 13 items (12 seed + steamer), full untruncated names, category, and availability derived consistently — "available", "2 of 3 available", "all copies out · due back 18 Sep", and for out items with a queue, "· 4 waiting".
- Add ~8 lines of inline vanilla JS: filter rows on `input` against the item name, case-insensitive; show an empty-state line when nothing matches. Remove the Search button (it implies a submit round-trip that does not exist) or leave it inert-but-harmless — I would remove it.
- Each item links to the item-detail state that matches its availability (see Phase 5).
- Expected on manual walk: typing "bosch" leaves the drill and the GlassVAC; typing "zzz" shows the empty state.

## Phase 5 — Item detail (SCR-002), available and out states → US-004 Scenario 1

- Write `screens/scr-002-item-detail.html` — available state, Raclette grill: name, category, copies, "Available now", **Borrow (due 1 Oct)**, no ratings block. Borrow links to My loans.
- Write `screens/scr-002-item-detail-out.html` — out state, Vango tent: name, category, 1 copy, "All copies out · due back 18 Sep", "Nobody is waiting", and a **Reserve** button linking to `scr-004-reservations.html?reserved=1`.
- **Naming-convention stop:** a second file for a second state of the same SCR id (`-out` suffix, one manifest row, two states). Default: proceed with the suffix. If the prototype skill's convention differs, I follow the skill and rename.
- I do **not** build a separate reservation-confirmation screen. The result of reserving (place in queue + earliest date) belongs on the Reservations screen, which is where the story sends the member's attention.

## Phase 6 — Reservations (SCR-004) → US-004 Scenarios 1, 2, 3

Replace the greyed-out "coming soon" page entirely (including its `opacity: .5`).

- Write `screens/scr-004-reservations.html` with, as standing rows:
  - **Wallpaper steamer — Ready to collect · hold ends Tue 15 Sep (7 days)** → Scenario 2, and it renders FR-002 visibly.
  - **Kärcher K4 pressure washer — position 5 of 6 · earliest we can estimate: late Dec (rough)** → the ugly truth of a 6-deep queue, per the seed.
  - Each row has a **Cancel** control.
- Inline JS reads `location.search`; with `reserved=1` it prepends the newly-created **Vango tent — position 1 of 1 · earliest 18 Sep** row plus a short "Reserved." note → completes Scenario 1 end-to-end from the catalogue.
- Inline JS for **Cancel**: removes the row, renumbers remaining positions, shows "Reservation cancelled. Members behind you move up." → Scenario 3. No confirmation dialog — nothing in the story asks for one; I raise it as a question instead of inventing it.
- Expected on manual walk: Catalogue → tent → Reserve → Reservations shows three rows with the tent first at position 1 of 1; Cancel on the Kärcher removes it and leaves the note.

## Phase 7 — My loans (SCR-003) → US-005 Scenarios 1 and 2, and US-003 done honestly

- Write `screens/scr-003-my-loans.html`: 8 rows, real names, real due dates, overdue flagged with how late ("46 days overdue"), a "Renewed" column reflecting `loans.csv`, and a per-row **Renew** control.
- Inline JS:
  - Renewable row (Hobbit, stove) → due date moves +3 weeks (14 Sep → 5 Oct; 27 Sep → 18 Oct), row marks **Renewed**, Renew control goes away → **Scenario 1**.
  - Row on a queued item (KitchenAid) → inline refusal naming the reason: "Cannot renew — 2 members are waiting for this item." → **Scenario 2**.
  - Already-renewed row (sewing machine) → refusal: "Cannot renew — already renewed once." Sourced from FR-003, but **no scenario covers it**; flagged.
  - Overdue rows → refusal: "Cannot renew — this loan is overdue." Inferred from "renew a loan once **before it is due**"; **no scenario covers it**; flagged.
- A plain "8 items out" count line, because a member at the limit is the honest case. I do **not** add a limit warning, upsell, or blocked-borrow treatment — no story asks for it; it becomes a finding instead.

## Phase 8 — Update the manifest in `spec.md`

I edit only the **Screens & Flows** section and the status line of `.mochiko/specs/lending/spec.md`. I do not touch the stories, scenarios, or requirements — those are Rosa's; gaps I found go back to her as findings, not as silent spec edits.

- Status line: note US-004/US-005 mocked 2026-09-10.
- Screens table: SCR-002 gains its out state and the reserve action in "Data shown" and picks up US-004 alongside US-002; SCR-003 gains renewed flag / renew action and US-005; **SCR-004 stops being "coming soon"** and becomes Reservations — queue position, earliest date, ready-to-collect with hold end, cancel — story US-004.
- Flows: fix **FLOW-002, which is keyed to "US-002 Scenario 2" — a scenario that does not exist** (US-002 has only Scenario 1), and update its item to the Raclette grill. Add:
  - FLOW-004 SCR-001 → out item → SCR-002 (out) → Reserve → SCR-004 shows position and earliest date | US-004 S1
  - FLOW-005 nav Reservations → SCR-004 shows ready-to-collect and hold-end date | US-004 S2
  - FLOW-006 SCR-004 → Cancel → row removed, queue renumbered | US-004 S3
  - FLOW-007 SCR-003 → Renew (Hobbit) → due date +3 weeks, shows renewed | US-005 S1
  - FLOW-008 SCR-003 → Renew (KitchenAid) → refused with reason | US-005 S2

Every flow keys to a scenario; every screen traces to a story. Nothing in the mock exists without a row here — that is my check that Phase 2 was complete.

## Phase 9 — Verification

No test framework exists here and none should be added for a disposable mock. Verification is a manual walk plus one delegated mechanical sweep.

- **Delegation:** one disposable `Explore` subagent, **model: haiku**. Brief: within `.mochiko/specs/lending/prototype/`, list every `href` and `src` in every HTML file with file and line, and report any that do not resolve to an existing file; separately report every occurrence of `type="module"`, `nav.js`, `vite`, `package.json`, `coming soon`, or `★`. Facts with file:line only, no interpretation, no edits. **On return I check:** each claim carries a file and line, and I re-read personally any file it flags before acting — I do not take a remediation on its word alone. Expected result: zero broken links and zero hits on the second list.
- **Myself, by hand:** open `index.html` from the folder (not a server) and walk all eight flows, confirming nav appears on all five pages, the catalogue filters as you type, Borrow lands on My loans, Reserve lands on Reservations with the new row, Cancel renumbers, and each of the four renew outcomes fires on the right row.
- **I do not grade my own prototype.** Sign-off is Rosa's (and the desk volunteers'), walking it themselves.

## Phase 10 — Report

Deliver the updated mock and manifest, plus findings — the gaps the screens exposed, which are the point of doing this before the Go templates get written:

1. **Ratings/reviews removed from SCR-002** — no story; needs one before it returns.
2. **The mock was unopenable as intended** — Vite plus an ES-module nav meant no navigation when opened from the folder, against the stated working agreement. Build step removed.
3. **FLOW-002 cited US-002 Scenario 2, which does not exist.** Corrected to Scenario 1.
4. **FLOW-001 claimed live filtering the mock never did.** Now real.
5. **Seed conflict:** `it_0311` is available in `items.csv` and simultaneously on loan in `loans.csv`. Mock follows the loan record; someone should fix the seed.
6. **US-005 is silent on overdue loans.** "Before it is due" implies refusal; mock refuses with a reason. Confirm.
7. **US-005 is silent on a second renewal attempt.** FR-003 implies refusal; mock refuses. No scenario covers the message.
8. **US-004's "earliest date I might get it" is undefined for deep queues.** With a 6-deep queue on the Kärcher the honest answer is months away and a guess; the mock labels it "rough". How should this be computed and worded — or should it be withheld past a certain depth?
9. **US-004 Scenario 3's "the queue moves up" is invisible to the member who cancels.** They only see their own row disappear. Is anything meant to be shown, or is that system behaviour only?
10. **FR-001 (max 8 items) has no scenario and no screen.** The mock member is at 8 of 8 and Borrow is still offered on SCR-002. What should a member at the limit see, and where?
11. **FR-002's 7-day hold has no lapse behaviour.** The mock shows "hold ends 15 Sep"; nothing says what happens on the 16th.
12. **No reservation limit is specified** — a member can reserve without bound in the mock.
13. **No volunteer/desk view anywhere**, though the repo says volunteers run the desk from a tablet. Out of scope for these stories; noting it so it is a choice, not an oversight.

I would also say plainly that this mock is meant to be thrown away once US-004 and US-005 are agreed — the inline scripts are deliberately crude so nobody is tempted to promote them into the Go app.