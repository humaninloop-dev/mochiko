FILES-READ: `README.md`, `seed/README.md`, `seed/items.csv`, `seed/loans.csv`, `.mochiko/specs/lending/spec.md`, `.mochiko/specs/lending/prototype/README.md`, `.mochiko/specs/lending/prototype/package.json`, `.mochiko/specs/lending/prototype/nav.js`, `.mochiko/specs/lending/prototype/index.html`, `.mochiko/specs/lending/prototype/screens/scr-001-catalogue.html`, `.mochiko/specs/lending/prototype/screens/scr-002-item-detail.html`, `.mochiko/specs/lending/prototype/screens/scr-003-my-loans.html`, `.mochiko/specs/lending/prototype/screens/scr-004-reservations.html`

---

# Action plan — extend the lending mock for US-004 (reserve) and US-005 (renew)

## What the read told me (drives everything below)

- The mock is **not openable from the folder** as the project agreement requires. Two independent causes: `package.json` puts it behind `npm install && npm run dev` (Vite, Kai 2026-08-27), and every screen loads the nav with `<script type="module" src="../nav.js">`, which browsers block over `file://`. Opened straight from the folder today, **every screen renders with no navigation at all** — the volunteers who review this cannot walk it.
- **FLOW-001 is claimed but does not exist**: SCR-001's search box is an inert `<form>` with a Search button and no filtering. The manifest promises "type a word → results filter".
- **SCR-002 renders unrequested scope**: a star rating, "4.2 from 17 members", and a "write a review" link, added 2026-08-26. No story in the spec asks for ratings or reviews.
- **SCR-003's data shape lies**: one row, "The Hobbit / Tomorrow / No". Seed says the limit is 8 items, the median member holds 3, ~18% of loans are overdue and some by months, and item names run to a full line of manufacturer-model-contents. A one-row table with a two-word title hides every layout problem the real screen has.
- **SCR-004 is a stub** captioned "Coming soon — no story yet". US-004 now exists, so that caption is stale.
- **Manifest defects**: FLOW-002 cites "US-002 Scenario 2" but US-002 has only Scenario 1; SCR-004's row says "no story"; SCR-002's data-shown column does not mention ratings; nothing covers US-004 or US-005.
- The seed data is a gift and I will use it verbatim rather than invent: Kärcher K4 (1 copy, out, queue **6**) and Bosch drill (2 copies, both out, queue 4) give me real reserve targets; Marguerite Okonjo-Baptiste holds **8** loans including two overdue (one since 2026-07-26, ~6 weeks) and one already renewed (`ln_9067`, Janome sewing machine); her KitchenAid loan `ln_9044` sits on an item with a queue of 2 — that is US-005 Scenario 2 handed to me for free; her Hobbit loan `ln_9051` sits on an item with no queue and no prior renewal — that is US-005 Scenario 1.
- There is **no design system** anywhere in the workspace, so I keep the existing house look: system-ui, dashed/solid grey boxes, `#f4f4f4` fills. Rough on purpose, unchanged in character.

---

## Phase 1 — Confirm the four judgement calls before I touch files

I would stop here and put four questions to Rosa (spec author) and Kai (who made the last two mock changes), in one message, then proceed under the stated defaults if no ruling comes back.

1. **Remove the ratings/reviews block from SCR-002?** It answers no story. *If they say it is real scope:* it needs a story from Rosa first; I still take it out of the mock now and it returns when the story lands. *If they say drop it:* removed. **Default: remove, and report it as a finding.** Either ruling produces the same file today, so this does not block me.
2. **Remove Vite and the npm dependency?** Kai added it for hot reload; the project agreement says the mock is opened straight from the folder by non-developers. *If Kai wants hot reload:* he can point any static server at the folder — that does not require the mock to *depend* on one. **Default: delete `package.json`, make the nav a plain script, report it.** This one I would not reverse even on pushback, because a reviewer who cannot open it cannot review it.
3. **Can an overdue loan be renewed?** US-005 says "renew a loan once **before it is due**", which reads as no, but no scenario states it and FR-003 only mentions the queue rule. *If yes:* Renew appears on Marguerite's two overdue rows. *If no:* it is suppressed there with a reason. **Default: suppress on overdue rows, label the reason plainly, and flag the gap** — I will not present a guess as settled behaviour.
4. **Does a reservation count against the eight-item hold limit (FR-001)?** Unspecified. It matters because my mock member sits at the limit. **Default: it does not; reserving stays available at the limit. Flagged.**

I would also note a fifth item that needs no ruling from me but does need Rosa's attention: **FR-001 has no scenario**. Nothing in US-002 describes what a member sees at eight items. I will render a plain "7 of 8 items out" counter on SCR-003 traceable to FR-001, and report the missing scenario rather than invent a refusal screen.

## Phase 2 — Make the mock openable, before adding anything to it

No new screen is worth building on a frame that does not load.

- **Delete** `.mochiko/specs/lending/prototype/package.json`.
- **Rewrite** `.mochiko/specs/lending/prototype/nav.js` as a classic script (no `import`/`export`), with the "Reservations (coming soon)" label changed to "Reservations".
- **Edit** all five HTML files to load it as `<script src="../nav.js"></script>` / `<script src="nav.js"></script>` — dropping `type="module"` is the whole fix for `file://`.
- **Rewrite** `.mochiko/specs/lending/prototype/README.md`: open `index.html` by double-clicking it; no install, no build; note that Kai's hot-reload workflow still works via any static server without the mock depending on one.

**Delegation:** one throwaway `Explore` subagent, **model: haiku**, single brief — "list every occurrence of `type=\"module\"`, every `src=`/`href=` value, and every mention of vite/npm/package.json across `.mochiko/specs/lending/prototype/**` and the repo root README, with file and line number; do not edit." On return I check that the file list matches the eight prototype files I already know about and that nothing outside the prototype references `package.json` (if the root README or spec does, I fix that reference too). This is a mechanical sweep, so its absence-of-findings would not by itself let me skip my own check on the files I am editing.

**Check:** a link-resolution pass — every `href` and `src` in the prototype must name a file that exists after my edits. Expected result: zero dangling links, zero `type="module"`, zero external URLs or npm imports remaining. Plus a manual walk of every flow from `file://` — I would state explicitly in my report that this smoke walk is the verification, since there is no test runner here and I will not add one to a throwaway mock.

## Phase 3 — Give the existing screens honest data and working behaviour (US-001, US-002, US-003)

The new stories are only walkable if the screens they hang off are real.

**`screens/scr-001-catalogue.html`** — extend to all twelve seed items with their full names, categories, and availability derived from `copies` vs `on_loan` (Bosch drill: all 2 out; Kärcher: out; tent: available; KitchenAid: out; Hobbit: 2 of 3; Janome: out; GlassVAC: available; Coleman stove: 1 of 2; Vax carpet cleaner: out; Ryobi trimmer: out; raclette: available; ladder: 1 of 2). Out-of-stock rows also show queue depth ("6 waiting") because US-004 makes that the decision a member is making. Add a **working** filter: a few lines of inline vanilla JS that hides non-matching rows as you type, so FLOW-001 stops being a promise. Link the three items I actually detail to their detail pages; the rest link to the available-state page — and I will say so in the report rather than pretend twelve detail pages exist.

**`screens/scr-002-item-detail.html`** — keep as the tent, available, Borrow. **Remove** the ratings/reviews block per Phase 1.

**New `screens/scr-002-item-detail-out.html`** — the *same* screen, no-copy-available state: Kärcher K4, 1 copy, all out, due back 2026-09-05 (already overdue — honest, and taken from Tom Reilly's loan), **6 members waiting**, and a **Reserve** action. Still labelled SCR-002 with the state named, so the manifest keeps one screen ID and two states.

**`screens/scr-003-my-loans.html`** — rebuild from `loans.csv` as Marguerite, showing **seven** loans, not eight. I deliberately drop `ln_9099` (Coleman stove) so the Borrow flow from SCR-002 can complete without the mock immediately violating FR-001's eight-item ceiling. Seven still stresses the layout well past the median of three. Columns: item (full long name), borrowed, due, overdue, renewed, and a Renew action. That includes the ~6-week overdue Vax carpet cleaner and the already-renewed Janome. A plain "7 of 8 items out" counter sits above the table.

## Phase 4 — US-004, reserve an item that is out

**Scenario 1** — from `scr-002-item-detail-out.html`, **Reserve** navigates to `screens/scr-004-reservations.html`, which shows the new Kärcher reservation with **place in queue (7 of 7)** and an **earliest date I might get it**, derived visibly from the due-back date plus the queue ahead rather than dropped in as a magic number.

**`screens/scr-004-reservations.html`** — replace the stub entirely (drop the `opacity: .5` and the "no story yet" banner). It carries both states US-004 needs:
- **Queued** rows: Kärcher (7 of 7), Bosch drill (3 of 4) — with place, earliest date, and **Cancel**.
- **Scenario 2, ready to collect**: the Vax carpet cleaner, returned, held for me, **hold ends 2026-09-17** — seven days out, which is FR-002 made visible.

**Scenario 3, cancel** — clicking Cancel on the Bosch drill row removes it and visibly moves the queue up (a member behind me goes 4-of-4 → 3-of-3, and my own remaining positions re-number). Inline vanilla JS on the page, no new file, so the before/after is one click apart for a reviewer.

## Phase 5 — US-005, renew a loan

All on SCR-003, driven by inline vanilla JS:

- **Scenario 1** — Renew on **The Hobbit** (no queue on it, not previously renewed): the due date moves from 2026-09-14 to **2026-10-05**, three weeks later, and the row flips to renewed with the Renew action gone.
- **Scenario 2** — Renew on the **KitchenAid stand mixer** (queue of 2): refused in place, with the reason stated in members' words — two people are waiting for it. This is FR-003's refusal rule shown, not just asserted.
- **Already renewed** — the Janome row shows renewed with no action, traceable to FR-003's "once", and I flag that no *scenario* covers this state even though the requirement does.
- **Overdue rows** — per my Phase 1 default, no Renew, reason stated, and flagged as unconfirmed.

## Phase 6 — Bring the manifest in the spec back into line

Edit the **Screens & Flows** section of `.mochiko/specs/lending/spec.md` only — the stories and requirements above it are Rosa's and I do not touch them.

- SCR-002: add the no-copy-available state and the Reserve action to Purpose/Data shown; story becomes US-002, US-004.
- SCR-003: add renewed flag and Renew action; story becomes US-003, US-005.
- SCR-004: replace "*coming soon* / — / —" with the real purpose (place in queue, earliest date, ready-to-collect with hold end, cancel) and story US-004.
- Fix **FLOW-002**'s citation from "US-002 Scenario 2" to Scenario 1.
- Add FLOW-004 (catalogue → out item → Reserve → place in queue and earliest date, US-004 S1), FLOW-005 (nav → Reservations → ready to collect with hold end, US-004 S2), FLOW-006 (Reservations → Cancel → removed, queue moves up, US-004 S3), FLOW-007 (My loans → Renew Hobbit → due +3 weeks, renewed, US-005 S1), FLOW-008 (My loans → Renew KitchenAid → refused with reason, US-005 S2).
- Every US-004 and US-005 scenario now keys to exactly one flow — that mapping is the thing I check before calling this done.
- Amend the Status line's "(no screens yet)" parenthetical, since it is now false. I would call that edit out explicitly to Rosa rather than change her status line silently.

**Delegation:** a second `Explore` subagent, **model: haiku**, brief — "in the finished prototype folder, list every `href`/`src` value with its file, and every filename present under `screens/`; do not edit." On return I diff the two lists myself to confirm no dangling link and no orphan file. Completeness matters here, so I verify the pairing rather than accept a "looks fine".

## What I would refuse or stop on

- **I will not add a reviews or ratings feature back** without a story, however small the ask.
- **I will not keep Vite or any install step.** A mock the reviewing volunteers cannot open has failed at its only job.
- **I will not invent screens FR-001 implies but no scenario describes** — no "you've hit your limit" refusal screen. It goes in the findings for Rosa to write a scenario, or not.
- **I will not invent the overdue-renewal rule**; I ship a labelled default and flag it.
- **I will not grade this myself.** The mock goes to Rosa and the volunteers to walk; my report says what I built and what I am unsure of, not that it is good.

## What I would report at the end

1. **Built** — US-004 and US-005 walkable end to end: five screen files touched, one added (`scr-002-item-detail-out.html`), each of the five new scenarios reachable by clicking from `index.html`.
2. **Fixed while I was in there** — the mock now opens from the folder (Vite and the ES-module nav both removed); the nav was silently absent on every screen before this; the catalogue search now actually filters, so FLOW-001 is real; SCR-003 carries seven real loans with long names, a six-week overdue item, and a renewed item instead of one tidy row.
3. **Findings for Rosa** — (a) ratings/reviews existed in the mock with no story behind it and has been removed; (b) FR-001's eight-item limit has no scenario, and my mock member deliberately sits at seven so the borrow flow can complete; (c) US-005 does not say whether an overdue loan can be renewed — mock assumes not; (d) FR-003 forbids a second renewal but no scenario covers what that looks like; (e) nothing says whether reservations count against the hold limit; (f) US-004 does not say what happens to a reservation whose seven-day hold expires; (g) FLOW-002 cited a scenario that does not exist.
4. **Honest limits** — three of twelve catalogue items have real detail pages; all state changes are hard-coded front-end fakes; dates are fixed relative to 2026-09-10; this code is disposable and nothing in it should be promoted into the Go app.
5. **Ask** — Rosa and two desk volunteers walk the five new flows on a phone before any of this is built.