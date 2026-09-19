FILES-READ:
- `README.md`
- `.mochiko/specs/recurring-invoices/spec.md`
- `packages/compass/README.md`
- `packages/compass/COMPONENTS.md`
- `packages/compass/tokens.css`
- `packages/compass/src/StatusPill.tsx`
- `apps/web/src/pages/InvoicesPage.tsx`
- `apps/api/fixtures/dev-seed.json`

---

# Plan — clickable mock for recurring invoices (Monday design review)

## What I found that shapes the plan

Two things change the obvious approach, so I'm stating them up front:

**The workspace has no toolchain.** There is no `package.json`, no lockfile, no Vite config anywhere. `@northstar/compass` exports `AppShell`, `PageHeader`, `DataTable`, `EmptyState`, `Banner`, `Button` (per `InvoicesPage.tsx:1`) but `packages/compass/src/` contains only `StatusPill.tsx`. So `npm run dev` from `apps/web` cannot work here, and a prototype that imports the real Compass package cannot be run or verified. I will build a zero-install mock instead (below), not a Vite app.

**The spec needs four Compass things that don't exist,** and adding any of them is a Thursday guild decision — which falls *after* Monday. I will not invent them silently. The prototype's job becomes partly to be the artifact that goes to the guild.

---

## Phase 1 — Fix the deliverable's shape and location

**Do:** Settle placement and format before writing anything.

- Location: `.mochiko/specs/recurring-invoices/prototype/`, beside the spec — that's the stated working agreement in `README.md`, and it keeps a throwaway review artifact out of `apps/web`. I would **not** add routes or pages to `apps/web/src/`.
- Format: a self-contained, zero-install mock — `index.html` opened directly in a browser, plain JS, no build step. It `<link>`s the real `packages/compass/tokens.css` so every colour, space, radius and type size comes from the actual tokens rather than being eyeballed.
- Compass components that aren't in `src/` get prototype-local stand-ins, named exactly as the catalogue names them (`AppShell`, `PageHeader`, `DataTable`, `StatusPill`, `EmptyState`, `KeyValue`, `Banner`, `Toast`, `ConfirmModal`, `Field`, `Select`, `RadioGroup`, `Toggle`, `Drawer`, `Button`, `Menu`), built only from tokens, and behaving per the catalogue's stated rules (DataTable pages at 25 and renders no pager below that; long cells truncate with a `title` tooltip; Toast is bottom-left for 4 s; Banner sits under the PageHeader).

**Trade-off I'd state, not bury:** plain JS means the mock is not the production code. I'm accepting that because it's clickable Monday with zero setup on anyone's laptop, and because a React port is mechanical once component boundaries and names match 1:1.

**Stop / confirm:** if the team has a working Compass build and app scaffold outside this workspace, this choice is wrong.
- *Ruling "there is a real toolchain":* rebuild the same screens as a small Vite + React app under the same `prototype/` directory, importing `@northstar/compass` for real, keeping the fixture module and all copy unchanged. Roughly half a day's rework, no design rework.
- *Ruling "no, this is what we have":* proceed as above.
- **Default while unanswered:** proceed with the zero-install mock.

**Write:** nothing yet.

---

## Phase 2 — Write down the Compass gaps before designing around them

**Do:** Produce the gap list first so the design decisions are traceable, and so Thursday's guild has a written ask.

**Write:** `.mochiko/specs/recurring-invoices/prototype/COMPASS-GAPS.md`

Contents, each with the screen that forces it and the stopgap used in the mock:

1. **StatusPill has no variant for schedule status.** The catalogue's set is closed: `draft · sent · paid · overdue · void`, matching `StatusPillVariant` in `packages/compass/src/StatusPill.tsx:3`. Schedules are `active · paused · failed · ended` (all four appear in `dev-seed.json`). There is no honest mapping — reusing `paid` for `active` would be a lie in the design system's vocabulary. **Ask the guild for four new variants.** Mock stopgap: pills rendered from tokens (`--cp-success` active, `--cp-ink-muted` paused, `--cp-danger` failed, `--cp-border` ended) and visibly annotated as provisional.
2. **SideNav has no Recurring section**, and adding one is a guild decision. US-002 says "when I open Recurring", which implies a destination. Mock stopgap: show *both* entry points so the room can choose Monday — a provisional `Recurring` SideNav item, and a `Make recurring` / `Recurring schedules` route reached from the Invoices page. A tab strip under Invoices is *not* an option; Compass has no tabs and I won't invent one.
3. **No timeline/history list** (explicitly listed as absent). So the schedule detail shows sends via `KeyValue` (`Sends so far`, `Last sent`, `Next send`) rather than a send log. Worth naming as a likely follow-up ask, but the four stories don't require it.
4. **The money input takes its currency prefix from the account**, but `rs_04` is EUR on a GBP account (`dev-seed.json:2,15`). Mock renders each schedule's own currency and flags the conflict.

Note also what is *not* a gap: the "repeat every…" picker the catalogue calls out as missing isn't needed — FR-002 fixes cadence to monthly or quarterly, which is a two-option `RadioGroup`.

---

## Phase 3 — Build the fixture module

**Read:** `apps/api/fixtures/dev-seed.json` (already read).

**Write:** `prototype/fixtures.js`

- All seven schedules and six clients carried over verbatim, including the deliberately awkward ones: the 76-character client name on `cl_01`, the EUR schedule, the `failed` schedule on archived client `cl_05`, and `rs_06` whose `next_send` is `null`.
- **Invented data, marked as such:** `dev-seed.json` has no invoices array, only `source_invoice` numbers. I'd synthesise ~10 invoices consistent with those numbers (INV-0044, 0071, 0099, 0152, 0187, 0203, 0210 plus a few drafts and a void) so US-001 has a real starting point, with a header comment saying these are invented and the account really has 214.
- **A pinned "today."** The fixtures are dated relative to now, and stale dates read badly in a demo. I'd pin the mock's today to the review date, **2026-09-14**, in one constant. Effect: `rs_02` next sends "tomorrow", `rs_05` failed on 1 Sep and is visibly overdue, `rs_07` sends 30 Sep. One line to change if the review moves.
- Deliberately **no pagination scenario**: the notes say median 5 schedules, p95 23, and DataTable pages at 25 — so a real user essentially never sees a pager. The mock shows 7 and no pager, which is correct behaviour, and I'd say so rather than let anyone assume pagination was forgotten.

---

## Phase 4 — Screen 1: the Recurring list (US-002, and the US-004 entry)

**Write:** `prototype/index.html`, `prototype/prototype.css`, `prototype/app.js` (screens split into clearly-commented sections).

- `AppShell` + `PageHeader` titled **Recurring**, with a description line. **No primary button in the header** — you cannot create a schedule here; FR-001 says it must come from an invoice. That constraint is worth showing rather than papering over with a dead "New schedule" button.
- `DataTable` with the columns US-002 Scenario 1 names: Client · Amount · Cadence · Next send · Status. `Next send` renders an em dash for `rs_06`'s null. Client column carries the long `cl_01` name so truncation + tooltip is visible in the room.
- Row `Menu` per catalogue, contents varying by status: active → Pause, End, View; paused → Resume, End, View; failed → Fix, End, View; ended → View only. **Resume is absent, not disabled**, on ended rows — the catalogue doesn't describe disabled menu items, and absence enforces FR-004 without inventing a state.
- `Banner` tone `error` under the header when any schedule is failed: "1 schedule could not send." with an action link to the failed row.
- Empty variant (US-002 Scenario 2): `EmptyState` — heading "No recurring schedules", body explaining they start from a sent or paid invoice, and the primary Button goes to **Invoices**, not to a create form. Same reason as above.

---

## Phase 5 — Screen 2: make an invoice recurring (US-001)

- An **Invoices** screen mirroring `apps/web/src/pages/InvoicesPage.tsx` exactly — same five columns, same `StatusPill` usage — so the mock reads as the existing product. Add a row `Menu` with **Make recurring**.
- Eligibility: FR-001 / US-001 say "a sent or paid invoice". I'd treat `overdue` as eligible too (it has been sent) and omit the item for `draft` and `void`. **Flag for Tom** — this is a real ruling, not a detail. *If overdue is not eligible:* one line in the eligibility predicate. *If it is:* as built.
- `Drawer` (480 px) titled "Make recurring", using only catalogue form parts:
  - `KeyValue` showing what's copied from the invoice — client, currency, line-item total (FR-001), read-only.
  - Cadence: `RadioGroup`, Monthly / Quarterly (FR-002).
  - First send: `Field` type date.
  - Ends: `RadioGroup` — "On a date" (reveals a date `Field`) / "After a number of sends" (reveals a number `Field`).
  - Primary `Button` "Create schedule", ghost "Cancel".
- **Scenario 2 (past date):** submitting a first-send before the pinned today shows `Field` error text "The first send date must be today or later.", the drawer stays open, and nothing is added to the list. The native date picker can't prevent a typed past date, so the error state is the real behaviour and the mock shows it.
- **Success:** drawer closes, `Toast` "Schedule created", the new row appears in Recurring with its computed next send date — which is exactly US-001's independent test (paid invoice → monthly → ends after 6 sends → listed with next month's date).

**Stop / confirm — spec vs. live data:** US-001 offers only "an end date or a number of sends", but three of seven live schedules (`rs_03`, `rs_05`, `rs_06`) have `ends.type: "none"` — schedules that never end. The spec cannot create what the beta already contains.
- *Ruling "never-ending is real":* add a third "Never" option to the RadioGroup, and US-001 Scenario 1 needs a wording fix.
- *Ruling "the fixture is stale":* drop it, and the fixture needs correcting.
- **Default while unanswered:** show the two spec'd options live and a third "Never" option rendered in a marked-provisional state, so the question gets answered in the room on Monday rather than after.

---

## Phase 6 — Pause, end, resume (US-003)

- Schedule detail `Drawer` opened from any row: `KeyValue` block (client, source invoice, amount + currency, cadence, first send, ends, next send, sends so far, status) and the status-appropriate actions.
- **Pause:** row status flips to paused, next send cell changes to "Paused" rather than a date — the visible expression of FR-003 — plus a `Toast`. Resume restores the computed next send.
- **End:** `ConfirmModal`, danger confirm, body stating plainly that an ended schedule cannot be restarted. On confirm: status `ended`, next send em dash, and the row Menu no longer offers Resume (FR-004).
- Ending from a *paused* schedule works too, per US-003 Scenario 2's "active or paused".

---

## Phase 7 — Failed sends and the fix path (US-004)

- `rs_05` is pre-seeded failed with reason `client_archived` against `cl_05` (archived 2026-08-30) — the exact situation US-004 describes, already in the fixtures, so no invention needed.
- Failed row → detail drawer shows a `Banner` tone `error`: "Could not send on 1 September — Sunrise Yoga Collective CIC is archived." plus the fix: a primary Button "Restore client" and a secondary "End this schedule".
- Restoring unarchives the client in the mock's state, clears the failure, and recomputes the next send — with a `Toast`. I'd note in the prototype README that restoring a client is really a Clients-screen action and the mock stubs it.
- I'd also add a **"Run today's sends"** demo control (Phase 8) so the room can watch the failure *happen* rather than only see its aftermath — that's the story's independent test performed live.

---

## Phase 8 — Demo controls

A small, visually separate strip (clearly not part of the product chrome) with: **Reset**, **Empty account** (for US-002 Scenario 2), **Restore failure**, **Run today's sends**, and the pinned date. A design review goes badly when a walkthrough can only be done once; this makes every scenario re-runnable in front of the room.

---

## Phase 9 — Write-ups

**Write:** `prototype/README.md` — how to open it (double-click `index.html`; no install), what is real vs. faked (fake: all persistence, the invoice list, client restore; real: tokens, catalogue behaviours, all schedule data), the four open questions, and a **demo script** that walks each story's "Independent test" from the spec verbatim, in order, with the clicks.

**Write (edit):** `.mochiko/specs/recurring-invoices/spec.md` — fill only the `## Screens & Flows` section, which is explicitly marked "_To be filled by the prototype._" One short paragraph per screen, naming the Compass components used, plus a pointer to `prototype/` and to `COMPASS-GAPS.md`. I would not touch Tom's stories, requirements, or front matter — that's their document and Monday is where the stories get frozen.

**Write:** a "for the guild" section at the end of `COMPASS-GAPS.md` — the two asks (four StatusPill variants; a Recurring SideNav section) written as decisions with options and a recommendation, ready for Thursday.

---

## Phase 10 — Verify

There is no test runner, no linter and no build in this workspace, so **I would run nothing automated, and I'd say that plainly rather than imply the mock was tested.** Verification is a manual pass I'd do myself before reporting done:

- Walk all four independent tests from the spec exactly as written; each must complete without a dead end.
- Check every token used resolves against `tokens.css` and that no raw hex, px or font value appears in `prototype.css` — that's the one Compass rule I can actually enforce here, and I'd grep for `#`, `px` and raw colour names to confirm.
- Confirm the awkward data renders: long client name truncates with a tooltip, EUR amount shows as EUR, null next send shows an em dash, no pager at 7 rows.
- Confirm the negative paths hold: no Resume on an ended schedule, no invoice generated while paused, past first-send date creates nothing.

---

## Delegation

None. This is a few files in one workspace and the judgement calls (which Compass gaps to escalate, how to represent statuses that don't exist yet) are the substance of the task — splitting them across workers would cost more in briefing and review than it saves. No subagents, no parallel workers.

---

## What I'd report back

1. Where it is and how to open it — `.mochiko/specs/recurring-invoices/prototype/index.html`, double-click, no install.
2. That it's a plain-JS mock, not React, **because this workspace has no package.json and Compass ships only `StatusPill.tsx`** — so nothing importing `@northstar/compass` could have been run or verified. If a real toolchain exists elsewhere, that's a half-day port with no design rework.
3. The four screens, mapped to US-001…US-004, and confirmation that each story's own independent test was walked by hand.
4. **Two guild asks, unresolved and deliberately visible in the mock:** four new StatusPill variants, and where Recurring lives in the nav. Both are Thursday decisions; Monday is a chance to arrive at the guild with a preference.
5. **Two questions for Tom:** whether schedules can never end (three of seven live schedules do, but US-001 can't create one), and whether an overdue invoice can be made recurring.
6. **One thing I did not build:** the fixture notes mention beta accounts with manual-send, where a generated invoice lands in draft rather than going out. No story covers it, so it's absent from the mock — worth a decision before the stories are frozen.
7. That I filled the spec's `Screens & Flows` placeholder and changed nothing else in Tom's document.