FILES-READ: `README.md`, `seed/README.md`, `seed/items.csv`, `seed/loans.csv`, `.mochiko/specs/lending/spec.md`, `.mochiko/specs/lending/prototype/README.md`, `.mochiko/specs/lending/prototype/package.json`, `.mochiko/specs/lending/prototype/nav.js`, `.mochiko/specs/lending/prototype/index.html`, `.mochiko/specs/lending/prototype/screens/scr-001-catalogue.html`, `.mochiko/specs/lending/prototype/screens/scr-002-item-detail.html`, `.mochiko/specs/lending/prototype/screens/scr-003-my-loans.html`, `.mochiko/specs/lending/prototype/screens/scr-004-reservations.html`

---

# Action plan — extend the lending mock for US-004 (reserve) and US-005 (renew)

## Phase 0 — What the reads already told me (drives everything below)

Findings from the workspace as it stands, before I touch anything:

1. **The mock does not open the way its readers open it.** The repo's working agreement says the mock is opened straight from the folder in a browser by volunteers who are not developers. Kai's 2026-08-27 change made it a Vite app (`package.json`, `npm run dev`) and turned the shared nav into `<script type="module" src="../nav.js">`. Module scripts are blocked by browsers on a `file://` URL — so anyone who double-clicks `index.html` today gets a page with **no navigation at all** and no error they'd recognise. This is not a style preference; the mock is currently broken for its actual audience.
2. **SCR-002 renders a feature no story asks for** — a star-rating block ("4.2 from 17 members", "write a review"), added 2026-08-26 as a "quick win". No story, no functional requirement, no row in the Screens table.
3. **The placeholder data lies about shape.** My loans shows one row, "The Hobbit / Tomorrow / No". The seed says the median member holds 3, the cap is 8, ~18% are overdue (some by months), one loan in three is renewed, and item names are long enough to be the main layout risk. `loans.csv` has a member (Marguerite Okonjo-Baptiste) holding exactly 8 — the cap — with 2 overdue and 1 already renewed. The current single tidy row hides every layout and edge case the real screen will hit.
4. **FLOW-001 claims something that doesn't happen.** The catalogue search box is an inert `<form>`; typing filters nothing. The flow table asserts "type a word → results filter".
5. **Availability labels are inconsistent** across catalogue rows — "all copies out", "out", "available", "2 of 3 available" — and the catalogue shows 6 of the 12 seed items.
6. **FLOW-002 cites "US-002 Scenario 2"**; US-002 has only a Scenario 1.
7. **SCR-004 is a `coming soon` stub** whose row says "no story yet" — US-004 now gives it a reason to exist.

## Phase 1 — Decisions I would confirm before building

I'd take these to Rosa (spec author) and Kai (who made the toolchain and ratings changes) in one short message, and proceed under the stated default rather than idling:

| # | What I'd confirm | Default I proceed under | Branch if ruled otherwise |
|---|---|---|---|
| A | Remove Vite and `package.json`, return the mock to plain files openable by double-click | **Remove.** The repo's own working agreement is explicit that volunteers open it from the folder; a broken nav for every reviewer outweighs Kai's hot reload | If Kai wants hot reload kept, I keep `package.json` but still make every page work correctly over `file://` unaided, so the dev server is a convenience and never a requirement |
| B | Delete the star-rating / review block on SCR-002 | **Delete**, and record it as a finding. The mock should not show volunteers a capability nobody has agreed to build | If Rosa says ratings are genuinely wanted, I ask her to write the story first (US-006), then build it in a following pass — I don't re-add it on a verbal nod |
| C | How "the earliest date I might get it" (US-004 S1) is derived when a queue is 6 deep | **Show** the earliest copy's due-back date plus three weeks for each member ahead of me, labelled visibly as an estimate | If Rosa wants a different rule (e.g. no estimate at all past position 2), I change the one line of arithmetic and the label |
| D | Do reservations count toward the eight-item cap in FR-001? | **No** — FR-001 says "hold", and I render no cap messaging on the reserve path | If yes, SCR-002's Reserve button needs a blocked state and FR-001's wording needs to say so |
| E | Can an overdue loan be renewed? US-005 says "renew a loan **once before it is due**" but has no scenario for it, and two of the eight seeded loans are overdue | **No** — I render Renew as unavailable on overdue rows with a reason drawn straight from the story's own wording, and flag that US-005 is missing this scenario | If overdue loans may be renewed, I drop that state and Marguerite's two overdue rows become renewable |

I would not stop the build waiting on any of these; each one is a small, localised change if the ruling goes the other way.

## Phase 2 — Restore a mock that actually opens (skeleton before screens)

Nothing new gets built on a frame that doesn't load.

- **Delete** `.mochiko/specs/lending/prototype/package.json`.
- **Rewrite** `prototype/nav.js` — plain script, no module syntax (module scripts are exactly what breaks under `file://`); nav entries become Home · Catalogue · My loans · Reservations, with the "(coming soon)" label dropped.
- **Add** `prototype/style.css` — one shared stylesheet holding the crude house look already in use (system font, dashed boxes, grey fills, no polish), lifted out of the four near-identical `<style>` blocks. Adds a narrow max-width so the long real item names wrap the way they will on a member's phone. Deliberately rough: this is a mock, and it must keep reading as one.
- **Add** `prototype/data.js` — plain script holding fixtures taken from `seed/items.csv` (all 12 items, with copies / on-loan / queue depth) and `seed/loans.csv` (Marguerite's 8 loans, real dates, real overdue spans, the one already-renewed loan), plus a reservation set for the current member. One source of truth so catalogue, item detail, loans and reservations can't disagree with each other.
- **Edit** all four screens to drop their inline `<style>` and their module script tags in favour of the shared CSS and plain scripts.
- **No build step, no install, no dependency.** Every page must work by double-clicking it.

Refusal here: I will not introduce a framework, a bundler, or a package manager to make this easier to write. The reader cost has to stay zero.

## Phase 3 — Bring the three existing screens into line with the stories as they now stand

**`screens/scr-001-catalogue.html` (SCR-001, US-001)**
- Render all 12 seed items from `data.js`, long names intact.
- One consistent availability rule across every row: *Available now (n of m)* / *All copies out — due back <date>, N waiting*. The queue depth has to be visible here, because that is what tells a member reserving is even a thing.
- Make the search box actually filter, in a few lines of inline script, so FLOW-001 stops being a claim and becomes a click.
- Each row links to the item detail screen carrying the item id.

**`screens/scr-002-item-detail.html` (SCR-002, US-002 + US-004 S1)**
- Drive the screen from an item id in the URL, so one file covers both the available and the all-copies-out states rather than spawning a second screen id for a state. Links from the catalogue carry the id; opening the file bare falls back to a sensible default item.
- Available state (e.g. the Vango tent): Borrow → lands on My loans with the new loan visibly marked as just-added, due three weeks out.
- All-copies-out state (e.g. the Kärcher K4, queue 6, or the Bosch drill, 2 copies both out, queue 4): due-back date, number already waiting, **Reserve** → lands on Reservations showing my new place in the queue and the estimated earliest date. That is US-004 Scenario 1, end to end.
- **Remove the star-rating and review block** (decision B).

**`screens/scr-003-my-loans.html` (SCR-003, US-003 + US-005 both scenarios)**
- Replace the single tidy row with Marguerite's full eight loans — the cap, which is the layout's hard case — with real dates, not "Tomorrow": one 9 days overdue, one overdue since July (the "overdue by months" case the seed warns about), one already renewed, and the rest on loan.
- Add a **Renew** action per row with the outcomes the story actually specifies, each shown inline rather than on a new screen:
  - *Renewable* — the Hobbit and the Coleman stove, both with no queue: renewing moves the due date three weeks later and the row shows as renewed (US-005 S1).
  - *Refused — someone is waiting* — the drill, mixer, carpet cleaner, hedge trimmer and ladder all carry queues in the seed: the control is disabled with the reason stated plainly (US-005 S2, FR-003).
  - *Refused — already renewed once* — the Janome sewing machine loan (FR-003).
  - *Refused — already overdue* — the two overdue loans, under decision E, flagged as a gap in US-005 rather than presented as settled.
- Nice property worth noting in the report: the seed data happens to exercise every one of these branches without me inventing anything.

## Phase 4 — Build SCR-004 Reservations for real (US-004, all three scenarios)

**`screens/scr-004-reservations.html`** — replace the greyed-out stub entirely:
- Several reservations, not one, at honest depths taken from the seed (queues run up to 6):
  - **Ready to collect** — a returned copy where I'm next, showing the date the hold ends, 7 days out, per FR-002 (US-004 S2).
  - **In the queue** — e.g. position 3 of 6 on the Kärcher, with the estimated earliest date labelled as an estimate (US-004 S1, under decision C).
  - **Deep in the queue** — a position far enough back that the estimate is uncomfortable. If that reads badly to Rosa, that is the mock doing its job.
- **Cancel** on each row → the reservation disappears and the remaining positions move up by one, visibly (US-004 S3).
- Cross-screen state is passed by a simple marker in the link (just-reserved, just-cancelled, just-renewed) rather than any stored session state — deterministic, works from a plain file, and nothing clever for anyone to mistake for real logic.

**`prototype/index.html`** — refresh the landing box to name the four screens and the walkable routes, replacing the two-link stub.

## Phase 5 — Update the manifest in the spec

Edit **only** the `Screens & Flows` section and the `Status` line of `.mochiko/specs/lending/spec.md`. I do not touch Rosa's stories or functional requirements.

- Screens table: SCR-002's data column gains queue depth and due-back; SCR-003 gains renewal state; **SCR-004 stops saying "coming soon"** and becomes Reservations — queue position, estimated availability, ready-to-collect with hold-end date — keyed to US-004.
- Flows table: correct FLOW-002's dangling "US-002 Scenario 2" to Scenario 1, and add:
  - FLOW-004 — catalogue → out-of-stock item → Reserve → reservations shows my place and estimate (US-004 S1)
  - FLOW-005 — nav Reservations → ready-to-collect with hold-end date (US-004 S2)
  - FLOW-006 — reservations → Cancel → row gone, queue moves up (US-004 S3)
  - FLOW-007 — My loans → Renew an unqueued loan → due date +3 weeks, marked renewed (US-005 S1)
  - FLOW-008 — My loans → Renew a queued loan → refused with the reason (US-005 S2)
- Status line: US-004 and US-005 mocked, dated, with my name on it.

Every flow keys to a scenario and every screen traces to a story — SCR-004 is the one that was breaking that rule, and this closes it.

## Phase 6 — Rewrite the prototype README

**`prototype/README.md`**:
- Replace the `npm install && npm run dev` instructions with: open `index.html` in a browser. State plainly that there is no build step and nothing to install, and note why the Vite setup was removed so Kai isn't surprised.
- A short click-through checklist a volunteer can follow unaided, one line per flow above.
- A **"What the mock exposed"** section carrying the open questions from Phase 1 (estimate derivation, reservations vs the 8-item cap, overdue renewals) so they live somewhere durable rather than only in a chat message. These go in my artifact, not in Rosa's story text.

## Phase 7 — Verification

No test framework exists here and I would not add one — installing a runner to check a static mock defeats the point of the mock. Verification is a walkthrough plus two mechanical sweeps I'd hand off rather than spend my own attention on:

- **Delegate** to a disposable `Explore` subagent, `model: haiku`: *"List every `href` and `src` value in every file under `.mochiko/specs/lending/prototype/`, and for each, report whether the target file exists. Report facts with file and line."* On return I check: no link points at a missing file, and nothing still points at the deleted `package.json`.
- **Delegate**, second spawn, `model: haiku`: *"Search the whole repository for `type=\"module\"`, `vite`, `npm run`, and `package.json` outside `node_modules`. Report every hit with file and line."* On return I check nothing module-scripted survives (the exact thing that silently killed the nav) and no other document still tells a reader to run a dev server.
- I do these two as separate single-question spawns rather than one broad one, and I read the returned lines rather than trusting a summary.
- **Myself**, not delegated: opening each screen and walking all eight flows, and judging whether the honest data made anything look wrong. That's interpretive — it's the entire reason the mock exists — and it doesn't leave my hands.
- I would **not** grade my own prototype. The walkthrough goes to Rosa and a reviewer; my report says what I built and what it exposed, not that it's good.

## Phase 8 — What I'd report

1. **Built:** US-004 and US-005 are clickable end to end — five new flows, SCR-004 turned from a stub into a real screen, renewal added to SCR-003 with all four outcomes reachable.
2. **Repaired, beyond the card:** the mock did not open for its intended readers at all — module scripts meant the nav silently vanished for anyone double-clicking from the folder. Vite and `package.json` removed; it now opens by double-click, as the repo's working agreement requires. Flagged to Kai with the reason.
3. **Removed:** the star-rating and review block on SCR-002 — no story, no requirement. If it's wanted, it needs a story first; I've not deleted the idea, only the unsanctioned rendering of it.
4. **Corrected:** the catalogue search now actually filters (FLOW-001 was asserting behaviour that didn't exist); availability wording made consistent; FLOW-002's reference to a non-existent US-002 Scenario 2 fixed.
5. **Data made honest:** My loans went from one tidy row to a member at the eight-item cap with real dates, two overdue (one by two months), one already renewed, and full-length item names — because that's the shape the seed says is real, and it's where the layout will hurt.
6. **Findings needing Rosa's ruling:** how the "earliest date I might get it" estimate is derived at queue depth 6 and whether it should be shown that far back at all; whether reservations count against FR-001's eight-item cap; and that US-005 has no scenario for renewing an already-overdue loan even though roughly a fifth of live loans are overdue.
7. **Reminder:** this mock is disposable. It exists to settle US-004 and US-005 while they're still cheap to change; none of it should be promoted into the Go service.