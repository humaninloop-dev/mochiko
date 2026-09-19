FILES-READ: `README.md`, `seed/README.md`, `seed/items.csv`, `seed/loans.csv`, `.mochiko/specs/lending/spec.md`, `.mochiko/specs/lending/prototype/README.md`, `.mochiko/specs/lending/prototype/package.json`, `.mochiko/specs/lending/prototype/nav.js`, `.mochiko/specs/lending/prototype/index.html`, `.mochiko/specs/lending/prototype/screens/scr-001-catalogue.html`, `.mochiko/specs/lending/prototype/screens/scr-002-item-detail.html`, `.mochiko/specs/lending/prototype/screens/scr-003-my-loans.html`, `.mochiko/specs/lending/prototype/screens/scr-004-reservations.html`

---

# Action plan — mock US-004 (reserve) and US-005 (renew), and re-align the existing screens

## What I found before planning (drives everything below)

1. **The mock currently does not open from the folder.** Every page loads the shared nav via `<script type="module" src="../nav.js">`. Browsers refuse module scripts over `file://` (null origin), so double-clicking `index.html` gives a page with **no navigation at all**. The prototype README tells reviewers to run `npm install && npm run dev`. The project's working agreement says the mock must open straight from the folder because the volunteer reviewers are not developers. This is a live regression from the 2026-08-27 Vite change and it blocks the US-004/US-005 review, so fixing it is part of this job.
2. **SCR-003 is one fake row** ("The Hobbit · Tomorrow · No"), while `seed/loans.csv` gives a real persona (Marguerite Okonjo-Baptiste) holding 8 items with 2 overdue and 1 already renewed. US-005 cannot be reviewed against one row — the renew rules need a renewable loan, a queued loan, and an already-renewed loan on screen together.
3. **Every catalogue row links to the same detail page**, which only ever shows the one available tent. The spec's own SCR-002 row already promises "due-back date if none available", and that state has never been mocked. US-004 needs it.
4. **SCR-002 carries a star-rating box** added 2026-08-26 with no story, no FR, and no row in the screens table.
5. **FR-001 (max 8 items) collides with the seed persona**: if the mock renders all 8 of Marguerite's loans, she is at the limit and the agreed borrow flow (FLOW-002) becomes undemonstrable.
6. Small spec slip: FLOW-002 cites "US-002 Scenario 2"; US-002 has only Scenario 1.

## Phase 1 — Make the mock openable again (before adding anything)

- Edit the four existing screens plus `index.html`: change `<script type="module" src="…nav.js">` to a plain `<script src="…nav.js"></script>` (classic scripts do load over `file://`; module scripts do not). Keep `nav.js` — Kai's shared-nav refactor is right, only the module type is wrong.
- Add a `defer` attribute and keep the tag at end-of-body so `[data-nav]` exists when it runs.
- Rewrite `prototype/README.md`: lead with "double-click `index.html`", keep the Vite instructions as an optional second paragraph for developers.
- **Stop point:** deleting `package.json` / the Vite dependency undoes a deliberate teammate decision. I would ask Kai. Default while waiting: **leave `package.json` in place** — the classic-script fix makes both paths work, so nothing is blocked. If Kai says drop Vite, I delete `package.json` and the second README paragraph. If Kai wants Vite kept as the primary path, I keep it but I still land the classic-script change and say plainly that the folder-open path is non-negotiable per the working agreement.
- Fallback if the classic script still misbehaves on a reviewer's machine: inline the nav markup into all seven pages and delete `nav.js`. I'd only take this if Phase 6's manual check fails.

## Phase 2 — Fix the data on the existing screens against the seed

Today is 2026-09-09; all dates below are computed from `seed/loans.csv` and `seed/items.csv`.

**`screens/scr-003-my-loans.html`** — rewrite the table to render the seed persona, sorted by due date, with columns Item / Due / Overdue / Renew:

| Item | Due | Overdue | Renew state |
|---|---|---|---|
| Vax Rapid Power Plus carpet cleaner | 2026-07-26 | Yes — 45 days | Refused, 3 members waiting |
| Bosch PBH 2100 RE rotary hammer drill | 2026-09-01 | Yes — 8 days | Refused, 4 members waiting |
| KitchenAid Artisan stand mixer | 2026-09-12 | No | Refused, 2 members waiting |
| Janome 2200XT sewing machine | 2026-09-13 | No | Already renewed once |
| The Hobbit | 2026-09-14 | No | **Renew** → 2026-10-05 |
| Ryobi ONE+ long-reach hedge trimmer | 2026-09-20 | No | Refused, 2 members waiting |
| Coleman two-burner camping stove | 2026-09-27 | No | **Renew** → 2026-10-18 |

That is 7 of Marguerite's 8 seed loans. **Deliberate deviation:** I omit `ln_9091` (the 6.2 m ladder) so she holds 7 of 8. This keeps FLOW-002 clickable (she has room to borrow) *and* lets me mock FR-001: arriving at this screen as `?borrowed=it_0311` appends the tent as an 8th row and shows an "8 of 8 — you are at your limit" banner. I drop the ladder specifically because it is a queued loan; dropping it costs no renew case and leaves two renewable loans, four "someone is waiting" refusals, and one "already renewed" refusal.
  - *Alternative if Rosa wants the seed rendered exactly:* render all 8 and add a `?as=` persona switch to Tom Reilly (1 item out) for the borrow flow. I'd rather not — a persona switcher is a mock artefact that confuses non-developer reviewers. I'll name this as the fallback in the report.

**`screens/scr-001-catalogue.html`** — list all 12 items from `items.csv`, not 6. Each row: name · category · availability derived from `copies − on_loan` · and, when nothing is available, "N waiting" from `reservation_queue`. Link each row to `scr-002-item-detail.html?item=<id>`. Keep the search box as a non-functional prop but add a small "search is not wired up in the mock" note so reviewers don't read FLOW-001 as broken — or, cheaply, wire it with an inline filter over the rows, which is ~6 lines and makes FLOW-001 genuinely clickable. I'll wire it.

**`screens/scr-002-item-detail.html`** — keep one file, one SCR id; drive it from a small inline data table keyed off `?item=`, defaulting to the tent (`it_0311`) so the current entry point is unchanged. Three states:
- Available (tent): Borrow button → `scr-003-my-loans.html?borrowed=it_0311`.
- All copies out with a queue (e.g. Kärcher `it_0088`: 0 of 1, due back 2026-09-05 — overdue, 6 waiting): **Reserve** button → SCR-005. This finally fills the "due-back date if none available" the spec already promised.
- At-limit: if arrived with the limit flag set, Borrow is disabled with "You are holding 8 of 8 items — return something first (FR-001)".

**Star-rating box:** I would **not** delete it unilaterally — it was added on purpose after a request at the desk. Default: leave it, tag it "not in the spec yet — no story", and raise it. If Rosa says it's wanted, she writes it up as US-006 and I add a screens-table row; if she says it goes, it's a two-line deletion.

## Phase 3 — New/rewritten screens for US-004

**`screens/scr-004-reservations.html`** (full rewrite — remove the "coming soon" banner and the `opacity:.5`). My reservations, covering all three scenarios:

| Item | State | Detail |
|---|---|---|
| Bosch GlassVAC cordless window cleaner | **Ready to collect** | Returned 2026-09-07 · hold ends **2026-09-14** (FR-002, 7 days) |
| 3-section aluminium extension ladder, 6.2 m | Waiting | Position 1 of 1 · earliest ≈ 2026-09-22 |
| Kärcher K4 pressure washer *(appears only after `?reserved=it_0088`)* | Waiting | Position 7 of 7 · earliest ≈ 2026-11-11 |

Each row gets **Cancel** → a confirm box → returns to the list with that row gone and the line "You were 7th; the members behind you have moved up" (US-004 Scenario 3).

**`screens/scr-005-reserve-confirm.html`** (new) — the screen US-004 Scenario 1 names: "You are 7th in the queue for the Kärcher K4. Earliest you might get it: about 11 November 2026." Plus a link to My reservations and a back link.

**Assumption I am inventing and will label on-screen:** the spec gives no rule for "the earliest date I might get it". I compute it as *current loan's due-back date + 28 days per member ahead of you* (3-week loan + 7-day hold), and I print a small footnote saying so, so volunteers correct the rule rather than trust the number. Flagged to Rosa; if she rules differently it's a one-line change per row.

## Phase 4 — New screen for US-005

**`screens/scr-006-renew-outcome.html`** (new) — reached from the Renew control on each SCR-003 row as `?loan=<id>`, with three outcomes:
- `ln_9051` (The Hobbit): renewed — "Due 2026-10-05. Shows as renewed." (Scenario 1)
- `ln_9044` (mixer): refused — "2 members are waiting for this item, so it cannot be renewed." (Scenario 2, FR-003)
- `ln_9067` (sewing machine): refused — "You have already renewed this loan once." (FR-003)
Each with a back link to My loans, where the renewed row then shows a "renewed" mark.

## Phase 5 — Nav, index, and the spec tables

- `nav.js`: replace "Reservations (coming soon)" with "My reservations"; add nothing else (SCR-005/006 are outcome screens, reached from actions, not nav).
- `index.html`: four entry points plus a short "updated 2026-09-09 — now covers reserving and renewing" line and a two-line "how to review this" pointer.
- `.mochiko/specs/lending/spec.md` — I edit **only** the parts that describe the mock, not Rosa's stories or FRs:
  - Status line: "US-004, US-005 mocked 2026-09-09".
  - Screens table: SCR-002 data column gains queue depth and the Reserve action; SCR-004 becomes real (item, queue position, earliest date, ready-to-collect + hold end, cancel — US-004); add **SCR-005 Reserve confirmation** (US-004) and **SCR-006 Renew outcome** (US-005).
  - Flows table: fix FLOW-002's "Scenario 2" → "Scenario 1"; add FLOW-004 (catalogue → out item → Reserve → SCR-005), FLOW-005 (nav → SCR-004 shows ready-to-collect + hold end), FLOW-006 (SCR-004 → Cancel → queue moves up), FLOW-007 (SCR-003 → Renew Hobbit → new due date), FLOW-008 (SCR-003 → Renew mixer → refused with reason).
  - **Stop point:** if Rosa would rather her spec file stayed untouched, the same two tables go into `prototype/README.md` instead and I hand her the diff for the spec. Default: edit the spec, since those tables exist to describe the mock and already say "coming soon".
- I would **not** touch `seed/*.csv`, and I would write no Go code — this card is the mock only.

## Phase 6 — Verification

There is no test framework here and nothing to run; these are static files. What I would actually do:
- Cross-check every rendered number and date against `items.csv`/`loans.csv` row by row, and every derived value (days overdue, +3 weeks, +7 days) against today's date, 2026-09-09.
- Grep every `href` and form `action` across all seven pages and confirm each target exists in `screens/` — expect zero dead links, and zero remaining links to `scr-004` as "coming soon".
- Grep for `type="module"` across the folder — expect zero hits.
- Walk each of the eight flows on paper against the story scenarios and confirm each scenario has one screen that shows exactly what it says.
- **What I cannot do and will say so:** I can't click through in a real browser from this environment. I would ask Kai or Rosa to double-click `index.html` once on a Mac and once on a Windows machine and confirm the nav appears, before the review session. If it doesn't, Phase 1's inline-nav fallback lands.

**Delegation:** none. This is seven small static files in one folder; handing it to a worker would cost more coordination than the work itself, and the judgment calls (which loans to render, what to do about the star box) are the substance of the task.

## What I would report back

- The three new/rewritten screens and the four updated ones, with the story→screen→flow coverage table showing all five stories and all three FRs now demonstrable.
- **The Vite/file:// regression**, stated up front: the mock has had no working navigation when opened from the folder since 2026-08-27, so any review since then saw broken pages. Fixed, pending the one manual browser check.
- Deviations I made and why: 7 of Marguerite's 8 loans rendered (to keep the borrow flow demonstrable while still mocking the 8-item limit); reservation rows invented, since the seed has no reservations file.
- Open questions, each with what I assumed meanwhile: (a) the "earliest date" formula; (b) whether an **overdue** loan may be renewed — the spec says "before it is due" but doesn't rule on it; both of the persona's overdue loans happen to have queues so FR-003 refuses them anyway, which is why the mock dodges the question rather than answers it; (c) the star-rating box needs a story or removal; (d) Vite's future.
- **One finding for the Go build, not just the mock:** `items.csv` has no way to express a copy that has been returned and is being held for the next member (FR-002). A held copy is neither on loan nor available, and the catalogue must not show it as available. The schema needs a held/awaiting-collection state with a hold-expiry date before this is implemented.