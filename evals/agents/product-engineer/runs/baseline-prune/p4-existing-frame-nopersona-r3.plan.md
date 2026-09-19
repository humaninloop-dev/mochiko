FILES-READ: `README.md`, `.mochiko/specs/lending/spec.md`, `.mochiko/specs/lending/prototype/README.md`, `.mochiko/specs/lending/prototype/package.json`, `.mochiko/specs/lending/prototype/nav.js`, `.mochiko/specs/lending/prototype/index.html`, `.mochiko/specs/lending/prototype/screens/scr-001-catalogue.html`, `.mochiko/specs/lending/prototype/screens/scr-002-item-detail.html`, `.mochiko/specs/lending/prototype/screens/scr-003-my-loans.html`, `.mochiko/specs/lending/prototype/screens/scr-004-reservations.html`, `seed/README.md`, `seed/items.csv`, `seed/loans.csv`

---

# Action plan — mock US-004 (reserve) and US-005 (renew), and realign the lending mock

## What I found before planning

Four things shape the work beyond "add two screens":

1. **The mock no longer opens from the folder.** The root README says the mock is opened straight from the folder by non-developers. Kai's 2026-08-27 change loads the shared nav with `<script type="module" src="../nav.js">` on all five pages. ES modules are blocked by the browser's cross-origin rules on `file://`, so double-clicking `index.html` today gives a page with an empty header and no nav at all — silently. `nav.js` itself uses no `import`/`export`, so it works fine as a classic script. This is a regression against the stated working agreement and I'd fix it as part of this card.
2. **SCR-002 only exists in the "available" state.** US-004 Scenario 1 starts from an item with *no* copy available, and the spec's own screen table already promises a "due-back date if none available" that the mock has never shown. A second state of SCR-002 is a prerequisite for the reserve flow.
3. **SCR-003 has a single made-up row** ("The Hobbit / Tomorrow / No"). It doesn't exercise the overdue flag US-003 asks for, and it has nowhere to hang a renew control. Seed data (5 loans/median 3, 18% overdue, one renewal per loan, queues up to 6 deep) gives me realistic content.
4. **Unspecified surface:** the star-rating / "write a review" block Kai added to SCR-002 on 2026-08-26 has no story behind it. See the stop point in Phase 0.

I would do this myself — no subagents, no delegation. It's nine small static files; splitting it would cost more in coordination than it saves, and the whole value is one person holding the story-to-screen mapping in their head.

---

## Phase 0 — Decisions I'd settle before writing (two stops, one judgement call)

**Stop 1 — the star-rating block on SCR-002.** No story covers it. Leaving un-storied features in the artefact volunteers sign off against risks them agreeing to something nobody has specified; deleting a colleague's work without asking is the other failure. This workspace is not a git repository, so there's no undo.
- I'd ask Rosa/Kai: keep it, cut it, or write it a story?
- *Keep* → I leave the markup untouched and add a visible "not covered by a story" marker beside it so reviewers don't read it as agreed.
- *Story* → out of scope for this card; I'd leave it and note that US-006 is owed.
- *Cut* → remove the block.
- **Default I'd proceed under:** cut it from the screen, and paste the exact markup verbatim into `prototype/README.md` under a "Parked — no story" heading so it's recoverable without git. I'd say plainly in my report that I did this and that it is one paste to restore.

**Stop 2 — can an overdue loan be renewed?** US-005 says "renew a loan once *before it is due*"; FR-003 mentions only the once-only rule and the reservation-queue refusal, and is silent on overdue. This changes what the mock shows on the overdue row.
- *Overdue can renew* → the overdue row gets a Renew button (still refused here, because that item has 4 people waiting).
- *Overdue cannot renew* → the row shows "Too late to renew — this was due 1 September".
- **Default:** cannot renew once overdue, following the story's "before it is due". I'd put this on-screen as an open question in a review-notes box (below) rather than burying it, so Rosa can settle it while clicking.

**Judgement call I'd make without asking — Vite stays, but stops being required.** I'd keep `package.json` and Vite as an optional convenience for whoever is editing (Kai wanted hot reload; it costs nothing and Vite serves classic scripts fine), and change the five `<script type="module">` tags to plain `<script>` so `file://` works again. Both audiences served, nobody's tooling taken away.

**Persona / seed consistency.** `loans.csv` gives Marguerite Okonjo-Baptiste 8 loans — exactly the FR-001 cap, which would mean the mock member cannot legally borrow, contradicting FLOW-002 (Borrow → appears in My loans). I'd make the mock member Marguerite but give her **5** of those 8 loans, chosen to exercise every renew branch. That keeps the seed's shape (5 loans, 1 overdue ≈ 20%, one already renewed) while leaving the borrow flow legal. No story covers hitting the 8-item cap; I'd flag that as a gap rather than invent a screen for it.

---

## Phase 1 — Fix the `file://` regression across existing files

Change `<script type="module" src="…nav.js">` → `<script src="…nav.js">` in `index.html`, `screens/scr-001-catalogue.html`, `screens/scr-002-item-detail.html`, `screens/scr-003-my-loans.html`, `screens/scr-004-reservations.html`. No change needed inside `nav.js` for this — it already uses only DOM calls and a `location.pathname` base prefix that resolves correctly under `file://`.

Also edit **`prototype/nav.js`**: drop "(coming soon)" from the Reservations label, and leave the link list otherwise as it is (the new files are states of existing screens, reached by clicking, not from the nav).

I'd keep the per-file inline `<style>` blocks rather than extracting a shared stylesheet — inline is the existing idiom here, and new screens will copy the same block plus the few new badge classes.

---

## Phase 2 — Content the two stories need (data I'd fix before writing markup)

Anchored to today, 2026-09-10, three-week loan period, seven-day hold (FR-002):

**Mock member's loans (SCR-003):**

| Item (from `items.csv`) | Due | State | Renew? |
|---|---|---|---|
| Bosch PBH 2100 RE rotary hammer drill… | 1 Sep 2026 | overdue 9 days, 4 waiting | no — too late (Stop 2 default) |
| Kärcher K4 Full Control pressure washer… | 12 Sep 2026 | 6 members waiting | **refused with reason** → US-005 S2 |
| The Hobbit | 14 Sep 2026 | nobody waiting, not renewed | **Renew** → 5 Oct 2026, US-005 S1 |
| Sewing machine — Janome 2200XT… | 13 Sep 2026 | already renewed once | no — FR-003 |
| Coleman two-burner camping stove… | 27 Sep 2026 | nobody waiting | Renew → 18 Oct 2026 |

**Reserve target:** KitchenAid Artisan stand mixer (1 copy, on loan to 12 Sep, queue of 2) → reserving makes you 3rd, earliest 24 Oct 2026 (due back + 2 × 3 weeks). Deliberately not one of her own loans.

**Reservations (SCR-004):** Vax Rapid Power Plus carpet cleaner — *ready to collect*, returned 8 Sep, hold ends **15 Sep 2026** (US-004 S2); Ryobi long-reach hedge trimmer — 3rd in queue, earliest 1 Nov 2026 (the cancel target, US-004 S3); aluminium extension ladder — 2nd in queue, earliest 13 Oct 2026.

The "earliest date" rule (due back + three weeks per person ahead, assuming no renewals) I'd state once in the review-notes box. It is self-consistent with FR-003: a queued item can't be renewed, so the estimate holds.

---

## Phase 3 — Screens

New and rewritten files, all under `.mochiko/specs/lending/prototype/screens/`:

- **`scr-002-item-detail-out.html`** *(new)* — SCR-002 in its "all copies out" state: KitchenAid, 1 copy, 0 available, **due back 12 September 2026**, "2 members waiting". `Reserve` button → `scr-004-reservations-reserved.html`. Page heading labels it "SCR-002 · state: all copies out" so the ID mapping stays obvious to reviewers.
- **`scr-002-item-detail.html`** *(edit)* — classic script tag; ratings block per Stop 1; keep the Borrow button and tent content.
- **`scr-003-my-loans.html`** *(rewrite)* — the five-row table above with Item / Due / Overdue? / Renew columns. Renewable rows get a `Renew` link → `scr-003-my-loans-renewed.html`. The Kärcher row gets a `Renew` link → `scr-003-my-loans-renew-refused.html`. The other two rows show the reason in place, no control.
- **`scr-003-my-loans-renewed.html`** *(new)* — same table, banner "Renewed — The Hobbit is now due 5 October 2026", that row shows the new date and a "Renewed" mark, its Renew control gone. **US-005 Scenario 1 done clickably.**
- **`scr-003-my-loans-renew-refused.html`** *(new)* — same table, banner "You cannot renew the Kärcher K4 Full Control pressure washer — 6 members are waiting for it. It is due back 12 September 2026." **US-005 Scenario 2.**
- **`scr-004-reservations.html`** *(rewrite)* — drop the "coming soon" banner and the `opacity: .5`; the three-row reservations list with place in queue, earliest date, ready/hold-end state, and `Cancel` on each queued row → `scr-004-reservations-cancelled.html`. **US-004 Scenario 2** lives here.
- **`scr-004-reservations-reserved.html`** *(new)* — the list plus the newly-made KitchenAid row and banner "Reserved — you are 3rd in the queue. Earliest you might get it: 24 October 2026." **US-004 Scenario 1.**
- **`scr-004-reservations-cancelled.html`** *(new)* — hedge-trimmer row gone, banner "Reservation cancelled — the 2 members behind you have each moved up one place." **US-004 Scenario 3.**
- **`scr-001-catalogue.html`** *(edit)* — list all twelve items from `items.csv` with real availability derived from `copies`/`on_loan`, correct the two mislabelled rows (the current file shows the KitchenAid as plain "out" and the drill as "all copies out" with no queue signal; I'd show "out · 2 waiting" style), route available items to `scr-002-item-detail.html` and out items to `scr-002-item-detail-out.html`, and make the search box actually filter rows on keyup with a few lines of plain JS. FLOW-001 is currently listed as an agreed flow but the form does nothing — this makes it real. Long item names are the seed's stated reality, so I'd check the rows wrap rather than overflow on a phone-width window.
- **`index.html`** *(edit)* — classic script; start-here box pointing at Catalogue, My loans, and Reservations.

Every screen that carries a story-relevant assumption gets a small **"Notes for review"** box at the bottom listing the open questions in plain language — chiefly: *can a loan be renewed once it is overdue?*, and *what happens when you're at 8 items and try to borrow — no story covers it yet*.

---

## Phase 4 — Documentation

- **`prototype/README.md`** *(rewrite)* — lead with "double-click `index.html`"; keep `npm run dev` as an optional note for editors; add a short table mapping each file to its SCR id and state, and each state to the story scenario it demonstrates; add the "Parked — no story" section if Stop 1 defaults to cutting.
- **`.mochiko/specs/lending/spec.md`** *(edit — Screens & Flows section and the status line only)* — I would **not** touch the stories or FRs; those are Rosa's. I'd update: the status line (US-004/US-005 now mocked); the SCR-004 row from "*coming soon*" to its real purpose and data; the SCR-002 row to name both states; and add flows FLOW-004 (catalogue → out item → Reserve → queue position), FLOW-005 (Reservations → ready to collect + hold end), FLOW-006 (Reservations → Cancel → removed), FLOW-007 (My loans → Renew → new due date), FLOW-008 (My loans → Renew on a queued item → refusal). I'd also correct FLOW-002's citation of "US-002 Scenario 2" — US-002 only has Scenario 1 — and mention that correction rather than making it silently.

---

## Phase 5 — Checking

There is no test framework here and none is warranted; it's a static mock reviewed by clicking. What I'd actually do:

- **Link check:** enumerate every `href`/`action`/`src` across the ten HTML files and confirm each resolves to a file that exists, including the `../` hops from `screens/` and the base-prefix logic in `nav.js`. I'd expect zero dangling links; a dangling link is the failure mode that makes a volunteer think a flow is broken.
- **`file://` check:** grep for any remaining `type="module"` — expect none — and confirm `nav.js` still contains no `import`/`export`, since a classic script with either would throw and blank the nav again.
- **Story coverage check:** walk the five new scenarios (US-004 S1–S3, US-005 S1–S2) against the screen list and confirm each has a clickable path from `index.html`, no dead ends.
- **Date arithmetic check:** re-verify every date on screen against 2026-09-10 and the three-week / seven-day rules.
- **Honest limit:** I can't see a rendered browser, so I would not claim the layout is good. I'd run the structural checks above, then ask whoever reviews to click the flows and eyeball the long item names at phone width. If shell were available I'd additionally run `npx vite build` as a cheap syntax smoke test and `open index.html` for the human to look at.

---

## What I'd report at the end

- The two stories are mocked, scenario by scenario, with the file that demonstrates each.
- The mock opened from the folder was broken since 2026-08-27 (module scripts don't run on `file://`) and is fixed; Vite still works for editors.
- The star-rating block: what I did with it under Stop 1's default, and that it's one paste from `prototype/README.md` to restore.
- The open question I defaulted on: renewing an overdue loan. Flagged on-screen for Rosa.
- Gaps I did not invent screens for, because no story covers them: hitting the 8-item cap on borrow, and what a volunteer sees at the desk when a held item's 7 days expire.
- The FLOW-002 scenario-number correction in the spec.
- Three side-fixes I made while aligning: the catalogue search now actually filters (FLOW-001 was listed as agreed but did nothing), the catalogue shows all twelve seed items with correct availability, and SCR-003 now shows real overdue data instead of one placeholder row.