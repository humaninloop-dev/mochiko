FILES-READ: `.mochiko/specs/recurring-invoices/spec.md`, `README.md`, `packages/compass/README.md`, `packages/compass/COMPONENTS.md`, `packages/compass/tokens.css`, `packages/compass/src/StatusPill.tsx`, `apps/web/src/pages/InvoicesPage.tsx`, `apps/api/fixtures/dev-seed.json`

---

# Plan — Recurring-invoices clickable mock for Monday 2026-09-14

## What I read and what it changes about the job

Two findings reshape the work before any code:

**The workspace has almost no app in it.** `apps/web` contains exactly one file, `apps/web/src/pages/InvoicesPage.tsx:1`, and it imports `@northstar/compass` and `../data/invoices` — neither exists here. There is no `package.json`, no Vite config, no `AppShell`/`DataTable`/`Button` source. The only real Compass component on disk is `packages/compass/src/StatusPill.tsx`. So `npm run dev` from `apps/web` (README line 21) will not run, and I cannot build the mock "into the app". The mock has to be self-contained. That matches the README anyway: the clickable mock belongs at `.mochiko/specs/recurring-invoices/prototype/`, beside the spec.

**Three of the four stories need something Compass does not have.** These are guild decisions (Thursday), not mine, and I will not make them unilaterally — but the prototype is precisely the artifact that argues them, so I build proposals, mark them as proposals, and take them to the guild with the mock as evidence.

---

## Phase 1 — Fix the facts the mock must be honest about (no writes)

Re-read `apps/api/fixtures/dev-seed.json` line by line and turn it into the mock's data, unaltered. The fixture is doing real work here and I will not replace it with tidy invented rows:

- `cl_01` is *"Bramblewood Landscape Architecture & Garden Design Partnership LLP"* — 66 characters. The catalogue says `DataTable` truncates with ellipsis and a title tooltip. That row must be on screen at the review, so the team sees the real truncation in the Client column and in the drawer, not a comfortable fake.
- `rs_04` is **EUR 6,200 in a GBP account** (`account.currency: "GBP"`). FR-001 says a schedule copies the invoice's currency, but the catalogue's money input takes "currency prefix from the account". That is a genuine conflict — flagged in Phase 6, and the list must render per-schedule currency, not account currency.
- `cl_03` is *"Tŷ Newydd Holiday Cottages"* — keep the diacritic, it exercises the font stack.
- `rs_05` is the US-004 case, already failed: client `cl_05` archived 2026-08-30, failure at 2026-09-01T06:00:04Z, reason `client_archived`. The reason copy can be exact rather than lorem: "Sunrise Yoga Collective CIC was archived on 30 August 2026."
- `rs_06` is ended with `next_send: null` — the Next send cell needs a defined empty treatment, and per FR-004 the row must not offer Resume.
- `rs_07` has `sent_count: 0` and `next_send == first_send == 2026-09-30` — a schedule that has never sent. Different from "active and running".
- `rs_03` is **paused but still carries `next_send: 2026-10-01`**. FR-003 says paused must not generate. So what does that date mean on screen? Open question for Tom (Phase 6). Default: render it muted and labelled "if resumed", so resuming is predictable.
- Volume note: median 5 schedules, p95 23. `DataTable` paginates at 25 and renders no pager below that. So virtually no account ever paginates this table. I will not mock a pager screen; I will build a 23-row density state to check the table holds up at p95.

**Date handling.** Today is 2026-09-09; the review is Monday 2026-09-14. `rs_02` sends 2026-09-15 — "tomorrow" at the review. I'll make the mock's notion of today a single constant defaulting to **2026-09-14**, so relative dates and the US-001 past-date validation read correctly on the day, and note the constant in the prototype README.

## Phase 2 — Scaffold a self-contained prototype

Write, under `.mochiko/specs/recurring-invoices/prototype/`:

- `package.json`, `vite.config.ts`, `tsconfig.json`, `index.html`, `src/main.tsx`
- `src/App.tsx` — hash-based routing so every screen is a linkable URL; a "scenario" switcher in the corner to jump between populated / empty / p95 / failed states during the walkthrough.
- `src/fixtures.ts` — imports `../../../../apps/api/fixtures/dev-seed.json` directly (single source of truth), plus derived empty-account and 23-schedule variants.
- `src/state.ts` — in-memory store; pause/resume/end/create mutate it so the click-through actually changes state within a session.
- `src/schedule.ts` — next-send arithmetic, end-condition evaluation, first-send validation.

**Compass stand-ins.** `src/compass/` gets prototype-only implementations of just what the screens need: `AppShell`, `SideNav`, `PageHeader`, `Button`, `Menu`, `DataTable`, `EmptyState`, `KeyValue`, `Banner`, `Toast`, `ConfirmModal`, `Field`, `RadioGroup`, `Drawer`. Each is built to the catalogue's documented props, variants and states — `DataTable` gets the 25-row pager threshold, zebra on `--cp-surface-sunken`, row hover, the `emptyState` slot, and ellipsis-plus-title truncation, because a stand-in that behaves differently from the real thing would make the review judge a fiction. They import `packages/compass/tokens.css` and use tokens by name only, never raw hex. The real `packages/compass/src/StatusPill.tsx` is imported as-is for *invoice* status.

I will **not** add these to `packages/compass/src/`, and will not touch `COMPONENTS.md` or `tokens.css`. The prototype README will say plainly that these are throwaway stand-ins and the real components are the source of truth.

## Phase 3 — The three things Compass does not have

Each gets a proposal in the mock, visibly marked, and an entry in `PROPOSALS.md`:

1. **Schedule status has no pill.** `StatusPill` variants are a closed set — `draft · sent · paid · overdue · void` — and the fixture has `active · paused · ended · failed`. I will not map schedule states onto invoice variants (active→paid, failed→overdue); it is semantically wrong and would quietly teach the review the wrong model. Instead: `src/compass/SchedulePill.tsx`, four variants using existing tokens (`--cp-success` / `--cp-ink-muted` / `--cp-border` / `--cp-danger`), rendered identically to `StatusPill` and labelled "proposed — not in Compass" in the prototype's notes panel.
2. **"Recurring" is not a nav section.** US-002 says "when I open Recurring". `SideNav` has six sections and adding one is a guild call. Default: show it as a proposed seventh item, visually marked. I will also sketch the fallback in `PROPOSALS.md` — Recurring as a tab on Invoices — so the guild has both options.
3. **No "repeat every…" picker.** Listed under "Not in Compass (yet)". I compose it from catalogued parts only: `RadioGroup` for monthly/quarterly, date `Field` for first send, `RadioGroup` (end date / after N sends / no end) with a nested date or number `Field`. Composition of existing components is not a new pattern, so this is not a guild ask — but I'll show it Monday to confirm.

There is also no timeline component, so US-004's history and `sent_count` go into `KeyValue` pairs in the detail drawer rather than an invented history list.

## Phase 4 — Build the screens

`src/screens/`, one file each, covering every scenario in the spec:

| # | Screen | Story |
|---|---|---|
| 1 | `RecurringListScreen` — client, amount (own currency), cadence, next send, status; row `Menu` | US-002 S1 |
| 2 | Empty state — `EmptyState` telling them to start from an invoice, with a button to Invoices | US-002 S2 |
| 3 | `InvoiceDetailScreen` — "Make recurring" entry point, reusing the real `StatusPill` | US-001 entry |
| 4 | `NewScheduleDrawer` — cadence, first send, end condition; prefilled from the invoice per FR-001 | US-001 S1 |
| 5 | Same drawer, first send in the past → error text on the `Field`, nothing created | US-001 S2 |
| 6 | Creation success → `Toast` + new row showing next send date | US-001 S1 |
| 7 | `ScheduleDetailDrawer` — `KeyValue` list; actions vary by status | US-002/003 |
| 8 | Pause → row and drawer show paused | US-003 S1 |
| 9 | End → `ConfirmModal` with `danger` confirm → ended, **no Resume anywhere** | US-003 S2, FR-004 |
| 10 | Failed schedule — page-level `error` Banner, failed row, drawer showing the archived-client reason and fix actions ("Restore client" primary, "End schedule" secondary) | US-004 S1 |
| 11 | 23-row density check (not a review screen; a build check) | p95 sanity |

On screen 10 I will *not* offer "point the schedule at a different client" as a fix, because FR-001 says the schedule copies the invoice's client — I'll raise it as a question instead of designing around the requirement.

## Phase 5 — Verify

`src/schedule.test.ts` (vitest) — small and targeted, because a mock that shows wrong dates on the projector misleads the review:

- quarterly from `rs_07`'s 2026-09-30 → 2026-12-30
- monthly from `rs_02`'s 2026-09-15 → 2026-10-15
- monthly month-end rollover: 2026-01-31 → 2026-02-28
- `rs_02` stops at `sent_count` 12 (count end); `rs_01` stops at 2027-05-31 (date end)
- first send 2026-09-13 rejected against today 2026-09-14; 2026-09-14 accepted
- paused schedule generates nothing (FR-003); ended has no resume path (FR-004)

Then `npx tsc --noEmit` and `npm run build`, and a manual click-through of screens 1–11 in the browser. Expected result: all listed dates on screen match the fixture's `next_send` values, and no schedule row shows a Resume action once ended.

If the arithmetic tests fail I fix the arithmetic; if the build fails on the stand-ins I fix the stand-ins. I would report actual output either way rather than asserting it passed.

## Phase 6 — Write up, and the two stops

**Fill in `## Screens & Flows` in `spec.md`** — that section literally reads "_To be filled by the prototype._", so completing it is sanctioned. I will change nothing else in Tom's document.

`prototype/README.md`: how to run it, the fixed "today" constant, what's a real Compass component vs a stand-in, and a walkthrough script ordered by story for Monday.

`prototype/OPEN-QUESTIONS.md` — for Tom, not blockers:
1. Does "a sent or paid invoice" (US-001) include **overdue**? An overdue invoice has been sent. *Default I build under: yes, overdue is eligible.*
2. What does Next send mean on a **paused** schedule? `rs_03` carries a date while FR-003 forbids sending. *Default: shown muted, labelled "if resumed".*
3. **Mixed currency** — `rs_04` is EUR in a GBP account, but the money `Field` prefixes account currency. FR-001 says copy the invoice's currency. The input pattern can't express this today.
4. The fixture note mentions **manual-send accounts** where the generated invoice stays a draft. No story covers it. I will not invent that flow; it needs a story or an explicit out-of-scope.
5. US-004 says "given a way to fix it" — is restoring the client the only fix, given FR-001 pins the client?

**Stop 1 — design guild, Thursday.** I'd bring the three proposals (schedule pill, Recurring nav section, cadence composition) with the running mock as evidence. If the guild approves the pill variants, the proposal becomes the spec for a real `StatusPill` extension or a sibling component and I'd file that as follow-up work — I still would not write it into `packages/compass` off the back of a mock. If the guild rejects the nav section, I switch the prototype to the Invoices-tab fallback, which is a routing change in `App.tsx` and one screen's chrome. If the guild rejects a distinct pill entirely and wants existing variants reused, I apply their mapping and note in the walkthrough that the colours don't carry their usual meaning.

**Stop 2 — Monday's review itself** is the point of the artifact, so the mock ships before it either way; the guild outcome only changes how the proposals are labelled on screen.

**No delegation.** Nothing here asks for it, and the work is one small interdependent prototype — splitting it across workers would cost more in reconciliation than it saves.

## What I'd report at the end

The prototype path and how to run it; the eleven screens mapped to US-001…US-004 and FR-001…FR-004 so the review can check coverage; the three guild asks and their status; the five open questions for Tom; the fact that stand-in components were necessary because `apps/web` has one file and no build, and that nothing under `packages/compass` or `apps/api` was modified; and the actual output of the tests and build rather than a claim about them.