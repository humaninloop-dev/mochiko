FILES-READ: `README.md`, `seed/README.md`, `seed/items.csv`, `seed/loans.csv`, `.mochiko/specs/lending/spec.md`, `.mochiko/specs/lending/prototype/README.md`, `.mochiko/specs/lending/prototype/package.json`, `.mochiko/specs/lending/prototype/nav.js`, `.mochiko/specs/lending/prototype/index.html`, `.mochiko/specs/lending/prototype/screens/scr-001-catalogue.html`, `.mochiko/specs/lending/prototype/screens/scr-002-item-detail.html`, `.mochiko/specs/lending/prototype/screens/scr-003-my-loans.html`, `.mochiko/specs/lending/prototype/screens/scr-004-reservations.html`

# Action plan — extend the lending mock for US-004 (reserve) and US-005 (renew)

## What I found that shapes the work

Four things in the current state change what "extend the mock" means, and I'd resolve each before writing screens:

1. **The mock no longer opens from the folder.** Every screen loads the nav with `<script type="module" src="nav.js">`. Browsers block ES modules over `file://` for cross-origin reasons, so a volunteer double-clicking `index.html` gets a page with no navigation at all — which is exactly the review path the project README commits to. Kai's Vite switch on 2026-08-27 traded that away, and there's no `node_modules/` here, so `npm install` needs network before the mock runs at all.
2. **SCR-002 has no "no copies available" state.** The spec's own screens table already says SCR-002 shows a "due-back date if none available", and US-004 Scenario 1 starts from exactly that state. So the reserve flow has no entry point today.
3. **SCR-003 is a single row (`The Hobbit / Tomorrow / No`)** and cannot show either US-005 scenario. `seed/loans.csv` gives a much better demo member: Marguerite Okonjo-Baptiste holds 8 loans — the FR-001 ceiling — including two overdue, one already renewed, five on items with a reservation queue, and two that are cleanly renewable.
4. **The seed contradicts itself on the Vango tent (`it_0311`).** `items.csv` says `on_loan 0` (available); `loans.csv` says Aisha Begum has it until 2026-09-18. The tent is the mock's current borrow example, so this has to be decided, not skirted.

## Phase 1 — Decide the data story and log the questions (no files written yet)

I'd settle these before touching screens; each is a point where I'd normally get a ruling from Rosa. I state my default and would proceed on it, marking each one visibly in the mock and in my final report rather than burying it.

- **1a. Tent conflict.** *Would confirm:* which source wins. **Default: `loans.csv` wins** — a loan record with a borrower and dates is harder evidence than a denormalised counter. Consequence: the tent becomes "out, due back 18 Sep" on SCR-001, and the borrow example on SCR-002 moves to **Raclette grill for 8 with 8 pans** (`it_0433`, 1 copy, free, no queue, already on the catalogue as available). *If Rosa rules the other way:* the tent stays available and borrowable, SCR-002 is untouched, and the queued-reservation example on SCR-004 becomes the Bosch drill instead — I'd note that Marguerite already holds a copy of the drill, which is odd but tolerable at this fidelity.
- **1b. Reservations vs the 8-item cap.** Marguerite is at the FR-001 limit, so a hold going ready raises a real question the spec doesn't answer: does a reservation count toward the eight, and may a member at the cap collect a ready hold? **Default: don't model a rule.** The mock shows the ready hold and carries a short "open question for Rosa" callout beside it. I would not invent a refusal the spec doesn't have.
- **1c. "Earliest date I might get it."** US-004 Scenario 1 requires a date; no rule exists. **Default:** earliest ≈ item's due-back date + 7 days per person ahead in the queue (FR-002's hold window), shown with the words "estimate" and annotated as an assumption on the screen.
- **1d. Which member the reservations belong to.** Marguerite holds 8 of the 12 seed items, so the only out-items she does *not* hold are the Kärcher and (under 1a) the tent. **Default:** she's the single demo member throughout; her reservations are Kärcher K4 (ready to collect) and the tent (queued). Her loan rows stay byte-faithful to `loans.csv`; only the reservations are mock-authored, and the README will say so.
- **1e. The star-rating block on SCR-002** (added 2026-08-26, no story behind it). "Bring the mock into line with the stories" arguably means deleting it. *Would confirm with Kai/Rosa.* **Default: keep it, don't delete someone's work on my own initiative** — move it below the borrow/reserve action so it doesn't compete with the story being reviewed, and add an HTML comment marking it as not backed by a story. *If they say cut it:* one deletion, no other change.
- **1f. Editing Rosa's spec.** **Default: I edit only the Screens & Flows section** (the part that documents the mock) plus the `FLOW-002` scenario reference, which points at "US-002 Scenario 2" when US-002 has only Scenario 1. I would not touch the stories, the FRs, or the status line beyond noting the mock is now current.

*Reading for this phase:* already done — the two CSVs, both READMEs, the spec, all five prototype files.

## Phase 2 — Make the mock openable from disk again

- **Write `prototype/nav.js`:** convert to a classic script (no `import`/`export`), add `Reservations` as a live link, drop the "(coming soon)" label. Keep the same `data-nav` injection and `../` base-path logic.
- **Write all six HTML files' script tags:** `<script type="module" src="…/nav.js">` → `<script src="…/nav.js" defer>`. Classic scripts do load over `file://`; modules do not.
- `package.json` stays as-is — Vite still works for Kai's hot-reload loop; this only restores the double-click path the project README promises.
- **Flag:** I'd tell Kai directly that the module switch silently broke folder-opening review, rather than just fixing it quietly.

## Phase 3 — SCR-002 gains a "no copies available" state (US-004 entry point)

- **Write `screens/scr-002-item-detail.html`** (edit): item becomes the Raclette grill — category kitchen, 1 copy, Available now, `Borrow (due in 3 weeks)` → SCR-003, unchanged in structure. Star block demoted per 1e.
- **Write `screens/scr-002-item-detail-out.html`** (new — a *state* of SCR-002, not a new SCR id, matching the spec's existing table row): Vango Odyssey Air 500 tent, camping, 1 copy, **no copies available**, "due back 18 Sep 2026", "nobody waiting yet", and a `Reserve` button → SCR-004. A second worked example block on the same page for the Kärcher K4 (6 waiting, due back 5 Sep — already overdue) so volunteers see a deep queue and a genuinely uncertain estimate, since the seed says queues run up to 6 deep.

## Phase 4 — SCR-003 rebuilt for US-005 (renew)

- **Write `screens/scr-003-my-loans.html`:** table of all 8 of Marguerite's loans straight from `loans.csv`, columns Item / Due / Overdue? / Renewed? / Action. Concretely:
  - **Renewable (US-005 Sc.1):** The Hobbit, due 14 Sep, queue 0 → `Renew`; and Coleman camping stove, due 27 Sep, queue 0 → `Renew`.
  - **Refused, queue exists (US-005 Sc.2, FR-003):** Bosch drill (4 waiting, overdue since 1 Sep), KitchenAid mixer (2 waiting), Vax carpet cleaner (3 waiting, overdue since 26 Jul), hedge trimmer (2 waiting), ladder (1 waiting) → `Renew` disabled with the reason in words: "Can't renew — 4 members are waiting for this."
  - **Refused, already renewed (FR-003, "once"):** Janome sewing machine, `Renewed: yes` → "Already renewed once."
  - Overdue flags computed against today, 9 Sep 2026: the drill and the carpet cleaner.
- **Renew interaction:** a few lines of inline classic JS on the page — clicking an enabled `Renew` pushes the due date three weeks out (Hobbit 14 Sep → **5 Oct**; stove 27 Sep → **18 Oct**), sets Renewed to "yes", and disables the button. Disabled buttons show their refusal reason on click. No build step, works from `file://`, no extra state files. This keeps the existing convention of one file per screen while still being clickable.

## Phase 5 — SCR-004 becomes real (US-004)

- **Write `screens/scr-004-reservations.html`:** drop the greyed-out "coming soon" banner and the `opacity: .5` body rule.
  - **Row 1 — ready to collect (Sc.2):** Kärcher K4 Full Control pressure washer — "Ready to collect", "hold ends 15 Sep 2026" (returned 8 Sep + the 7 days FR-002 requires). Beside it, the 1b open-question callout.
  - **Row 2 — in the queue (Sc.1):** Vango tent — "You are 1st in the queue", "earliest you might get it: 18 Sep 2026 (estimate)", with the 1c estimation rule spelled out in a caption.
  - **Cancel (Sc.3):** a `Cancel` on each row; inline JS removes the row, shows "Reservation cancelled — the queue has moved up", and decrements the displayed positions of anything below it.
- **Write `screens/scr-001-catalogue.html`** (edit): tent flips to `out · due back 18 Sep`; add "N waiting" to the out items from `items.csv` (drill 4, Kärcher 6, mixer 2); out items link to the new out-state page, available items to the borrow page. Keep the long item names verbatim — the seed README is explicit that members search on them, so short names would misrepresent search.
- **Write `index.html`** (edit): add Reservations to the start links.

## Phase 6 — Documentation

- **Write `prototype/README.md`:** "open `index.html` straight from the folder" as the primary path, `npm run dev` as the optional developer path; a screen/state map including the new out-state file; and an **Assumptions** section listing 1b, 1c, 1d and the fact that the reservation rows are mock-authored rather than from `loans.csv`.
- **Write `.mochiko/specs/lending/spec.md`** — Screens & Flows only:
  - SCR-002 row: note the two states (available → Borrow; none available → Reserve).
  - SCR-004 row: Reservations · "queue position, earliest date, ready-to-collect + hold end date" · US-004.
  - SCR-003 row: add renew action, renewed flag.
  - New flows: FLOW-004 SCR-001 → out item → SCR-002 (out) → Reserve → SCR-004 shows position + earliest date (US-004 Sc.1); FLOW-005 nav Reservations → ready to collect + hold end (Sc.2); FLOW-006 SCR-004 → Cancel → row gone, queue moves up (Sc.3); FLOW-007 SCR-003 → Renew → due +3 weeks, shows renewed (US-005 Sc.1); FLOW-008 SCR-003 → Renew on a queued item → refusal with reason (Sc.2).
  - Fix `FLOW-002`'s "US-002 Scenario 2" → Scenario 1.
  - Status line: mock brought up to US-005, dated 2026-09-09.

## Phase 7 — Verification

There is no test framework here and nothing to unit-test; this is static HTML. My checks would be:

- Open each of the seven files via `file://` and confirm the nav renders on every one — this is the check that proves Phase 2 worked, and it's the exact thing that fails today.
- Walk each flow and confirm the observable result: search box present on SCR-001; tent shows out + due back; Reserve lands on SCR-004 with a position and a date; Kärcher shows ready + 15 Sep; Cancel removes a row and renumbers; Renew on The Hobbit shows 5 Oct and "Renewed: yes"; Renew on the drill refuses and says four people are waiting; the sewing machine refuses as already renewed.
- Cross-check every name, category, copy count, queue depth and date on screen against `items.csv` / `loans.csv`, and every rule against FR-001/002/003.
- I would **not** run `npm install` (needs network, and nothing in this task requires the Vite path); I'd say so rather than imply the dev server was exercised. If the file:// walkthrough surfaced anything I couldn't fix within this scope, I'd report it unfixed rather than trimming the story coverage.

## Delegation

None. This is a handful of small static files in one folder; splitting it across workers would cost more in coordination than it saves, and I can't spawn anything here anyway. If they want a second pair of eyes on the diff, `/code-review ultra` is theirs to trigger — it's billed and user-initiated, so I wouldn't launch it.

## What I'd report at the end

- The seven files written and the two documentation files updated.
- Story-by-story coverage: US-004 Sc.1/2/3 and US-005 Sc.1/2, each with the screen and click path that demonstrates it.
- The six decisions from Phase 1, each marked as *my default, awaiting Rosa's ruling* — especially the tent seed conflict (and that it moved the borrow example to the raclette grill), the reservations-vs-8-item-cap gap, and the invented earliest-date rule.
- That I changed two already-agreed screens (SCR-001 tent availability, SCR-002 borrow example) and exactly why, so that's a conscious call to accept or reverse.
- That the mock had stopped opening from the folder since 2026-08-27 and now does again, addressed to Kai.
- That the star-rating block is still there, still unbacked by any story, and awaits a call.
- That `npm install`/`npm run dev` were not exercised.