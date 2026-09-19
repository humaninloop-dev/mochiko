FILES-READ: `.mochiko/specs/recurring-invoices/spec.md`, `README.md`, `packages/compass/README.md`, `packages/compass/COMPONENTS.md`, `packages/compass/tokens.css`, `packages/compass/src/StatusPill.tsx`, `apps/web/src/pages/InvoicesPage.tsx`, `apps/api/fixtures/dev-seed.json`

---

# Action plan — recurring-invoices clickable mock

## Framing (what I concluded from the workspace before planning any work)

Three facts shape everything below.

**The repo is a sketch, not a runnable app.** The whole workspace is eight files. There is no `package.json`, no lockfile, no Vite config, no `@northstar/compass` package manifest, and `apps/web/src/pages/InvoicesPage.tsx:2` imports `../data/invoices`, which does not exist. So `npm run dev` from `apps/web` (README line 21) cannot host this mock. I would not spend the pre-Monday time bootstrapping a build; I would ship a zero-install prototype that opens in a browser.

**Three things the feature needs are explicitly guild decisions, and the guild meets Thursday — after Monday.** I would not make any of them locally:
- US-002 says "open Recurring", but `SideNav`'s sections are Dashboard · Invoices · Clients · Expenses · Reports · Settings, and COMPONENTS.md line 9 says adding one is a guild decision.
- Schedule statuses are active / paused / ended / failed. `StatusPill`'s variants are a closed set of draft · sent · paid · overdue · void, and there is no danger-toned variant at all for "failed".
- US-001's cadence + end-condition control is close to the "repeat every…" picker listed under "Not in Compass (yet)".

The prototype is the correct vehicle for *asking* these, not for answering them — README line 20 says the mock is walked Monday *before the stories are frozen*. So they get mocked visibly inside the prototype and written up as the guild ask.

**"Screens & Flows: _To be filled by the prototype._"** (spec.md line 74) is part of the deliverable. Tom's spec invites that edit; I'd make it, and touch nothing else in his file.

---

## Phase 1 — Decide the host, and stop to confirm one thing

**What I'd do.** Settle the prototype as a self-contained static page under `.mochiko/specs/recurring-invoices/prototype/`, per the convention in README line 19–20. Plain `<script>` tags, no ES modules, no `fetch`, no build step — so double-clicking `index.html` works, and no one is debugging a toolchain at 9am Monday. It links the *real* `packages/compass/tokens.css` by relative path (`../../../../packages/compass/tokens.css`) rather than copying values, so the colours in the room are the actual product colours.

**Stop / confirm.** One question I'd put to the team before building, because the two answers produce materially different work: *should the mock be a throwaway static page beside the spec, or a real route inside `apps/web` that we keep?*
- If they say **static page beside the spec** (my default, and what README's convention describes): proceed as below.
- If they say **route in `apps/web`**: the job changes to bootstrapping a workspace — package manifests for `apps/web` and `packages/compass`, a Vite config, and building the ~14 catalogue components that COMPONENTS.md describes but that don't exist as code (only `StatusPill.tsx` does). That is not a Monday-sized task, and I'd say so and propose the static page for Monday with the route as follow-up.

I proceed under the default. No blocking wait.

**What I would refuse to do here:** modify `packages/compass/src/StatusPill.tsx`, `packages/compass/COMPONENTS.md`, `packages/compass/tokens.css`, `apps/web/src/pages/InvoicesPage.tsx`, or `apps/api/fixtures/dev-seed.json`. A design mock has no business changing the design system or shipping an unreviewed pattern into the product before the stories are frozen. The invoice entry-point screen gets its own copy inside the prototype instead.

---

## Phase 2 — Prototype fixtures

**Write:** `.mochiko/specs/recurring-invoices/prototype/fixtures.js`

A copy (not a reference) of the seed data, so the click-through is deterministic and unaffected by anyone editing dev fixtures. From `apps/api/fixtures/dev-seed.json` I'd carry over all six clients and all seven schedules, because each one earns its place:

| Row | What it demonstrates on screen |
|---|---|
| `rs_01` | Long client name → `DataTable` ellipsis + title tooltip (COMPONENTS.md line 24) |
| `rs_02` | Monthly, ends after 12 sends, 6 done — the "6 of 12" progress question |
| `rs_03` | Paused, but `next_send` is `2026-10-01` — do we show a next date for a paused schedule? |
| `rs_04` | EUR on a GBP account — collides with `Field`'s money input taking "currency prefix from the account" |
| `rs_05` | Failed, client `cl_05` archived 2026-08-30 — this is US-004's whole scenario, already in the seed |
| `rs_06` | Ended, `next_send: null` — the resume-is-impossible state (FR-004) |
| `rs_07` | `sent_count: 0`, next send = first send `2026-09-30` — a freshly created schedule |

I'd add: two invoices to act as US-001 entry points (one paid, one draft, to show the ineligible case), and a second "fresh account" dataset with zero schedules for US-002 Scenario 2. Today is pinned to **2026-09-09** so relative language ("in 6 days") is stable regardless of when it's opened.

I would *not* invent a p95 account with 23 schedules to test pagination — `DataTable` paginates at 25, so even the p95 account renders no pager. Worth one line in the writeup so no one designs a pager we'll never show.

---

## Phase 3 — The Compass mock layer

**Write:** `prototype/compass-mock.css`, `prototype/compass-mock.js`

Small render helpers named exactly after the catalogue — `AppShell`, `SideNav`, `PageHeader`, `Button`, `Menu`, `DataTable`, `StatusPill`, `EmptyState`, `KeyValue`, `Banner`, `Toast`, `ConfirmModal`, `Field`, `Select`, `RadioGroup`, `Toggle`, `Drawer` — so the room, the spec, and future engineers all say the same words (Compass README lines 10–12).

Every colour, space, radius, and type value comes from `var(--cp-*)`. I would keep a running list of any value I'm forced to use that has no token, and put it in the writeup rather than quietly hardcoding it. From reading `tokens.css` I already expect that list to be: **1px borders** (no border-width token), **the 480px `Drawer` width** and **the 25-row `DataTable` page size** (both stated in COMPONENTS.md prose but absent from tokens), and **any elevation/shadow** for `Drawer`, `Menu`, and `ConfirmModal` (no shadow token exists). I'd render those overlays with a `--cp-border` outline instead of inventing a shadow.

`StatusPill` in the mock layer is the sensitive one. It renders the five real variants faithfully, and for schedule statuses it renders the nearest existing tone with the schedule's own label, plus a small visible "proposed" marker in the prototype (not in any product surface):

- active → `sent` tone (info)
- paused → `draft` tone (grey)
- ended → `void` tone (muted)
- failed → `overdue` tone (warning) — **the weakest of the four.** "Failed" wants `--cp-danger`, and the closed set has no danger pill. This is the sharpest of the guild asks.

---

## Phase 4 — The screens

**Write:** `prototype/index.html`, `prototype/app.js`

Nine states, each traceable to a story. Every one is built only from catalogue components.

1. **Invoice detail (paid, INV-0187)** — entry point for US-001. `PageHeader` + row `Menu` containing "Make recurring".
2. **Invoice detail (draft)** — same screen, action disabled with help text. The spec says a schedule starts from "a sent or paid invoice" (line 18) but never states the negative; this screen makes the inferred rule visible so Tom can confirm or correct it Monday.
3. **Make recurring — `Drawer`** (US-001 S1). `RadioGroup` for Monthly · Quarterly. `Field` (date) for first send. `RadioGroup` for ends: on a date (`Field` date) · after N sends (`Field` number) · never. A read-only `KeyValue` block showing the client, line items, and currency carried over from the invoice — that block *is* FR-001 made visible. A live "Next send: 1 October 2026" line. Footer: `primary` Create, `ghost` Cancel. Built from `Select`/`Field`/`RadioGroup` only — no invented repeat-picker.
4. **`Drawer` with a past date** (US-001 S2). `Field` error text "Must be today or later", Create disabled, nothing created. Reachable by actually typing a past date, not a static screenshot.
5. **Recurring — list** (US-002 S1). `DataTable`: Client · Amount · Cadence · Next send · Status, with a row `Menu` (View · Pause/Resume · End). A `warning` `Banner` above it when any schedule has failed, with an action link jumping to `rs_05`.
6. **Recurring — empty** (US-002 S2). `EmptyState`: heading, one-line body explaining schedules start from an invoice, `primary` Button through to Invoices.
7. **Schedule detail** — `KeyValue` panel (client, source invoice, cadence, first send, ends, sent so far, next send) plus status pill and actions.
8. **Pause / resume / end** (US-003). Pause is reversible → immediate action + `Toast`, no modal. End is not (FR-004) → `ConfirmModal` with a `danger` confirm. After ending, resume is *absent*, not merely disabled.
9. **Failed schedule** (US-004) — `rs_05`. `error` `Banner`: "Could not send on 1 September — Sunrise Yoga Collective CIC was archived on 30 August", with an action link as the "way to fix it".

I would **not** build a send-history timeline. COMPONENTS.md line 51 lists a timeline as not-in-Compass, and no story asks for one.

Clickable paths that actually work end-to-end: invoice → drawer → create → list → detail → pause → resume → end; list → failed banner → failed detail → fix; empty account → invoices.

---

## Phase 5 — Verification

No test framework exists here and I would not add one for a throwaway mock. What I would actually check, and report honestly:

- Walk all nine screens and every link in the flow map above; every control either does something or is visibly disabled — no dead buttons in front of the room.
- Search my own prototype CSS/JS for hex codes, `rgb(`, and `px` outside the token-gap list from Phase 3. Expected result: zero hex, zero rgb, and only the four documented px exceptions.
- Cross-check every component name I used against COMPONENTS.md. Expected: all present, none invented.
- Confirm `packages/compass/**`, `apps/web/**`, and `apps/api/**` are byte-for-byte untouched.
- Open `index.html` directly from the filesystem. If a browser blocks the cross-directory `tokens.css` link, the documented fallback is `npx serve .` from the repo root — I'd put that line in the prototype README rather than duplicating the token values.

I cannot verify browser rendering in this environment beyond reading my own output, and I'd say that plainly rather than claim it's been seen working.

---

## Phase 6 — The writeup

**Write:**
- `prototype/README.md` — how to open it, what's real (tokens, component names, seed data) and what's mocked (the components themselves, all persistence).
- `prototype/WALKTHROUGH.md` — the Monday script: story → screen → what to look at, in the order I'd drive it.
- `prototype/OPEN-QUESTIONS.md` — the decisions the mock surfaces but does not make.

**Edit:** `.mochiko/specs/recurring-invoices/spec.md` — fill only the "Screens & Flows" section (line 74) with the screen list and flow map. No other line of Tom's spec changes.

The open-questions list, which is the real output of a design review:

*For the guild (Thursday, after Monday):*
1. A `SideNav` "Recurring" section, or a tab inside Invoices? The mock shows the nav section; that is a proposal, not a decision.
2. Four new `StatusPill` variants (active/paused/ended/failed), or reuse? "Failed" has no honest home in the current set — there is no danger pill.
3. Are the four un-tokened values I needed (1px border, 480px drawer, 25-row page, overlay elevation) genuine token gaps?

*For Tom:*
4. `rs_04` is EUR on a GBP account, but the money `Field` prefixes the account currency. FR-001 says the schedule copies the invoice's currency. Which wins in the form?
5. Does a paused schedule show a next send date? `rs_03` has one in the seed, but FR-003 says paused schedules generate nothing.
6. Eligibility: only sent and paid invoices — what about partially paid, void, or draft?
7. "A way to fix it" for an archived client (US-004): unarchive, reassign the schedule to another client, or just end it?
8. Month-end arithmetic. `rs_07` is quarterly from 30 September — is the next send 30 or 31 December? The spec is silent, and this will bite in build.
9. The seed's own note mentions manual-send accounts where the generated invoice stays in draft. Does that count toward `sent_count` and an "after N sends" end condition?

**Delegation:** none. This is a single-surface mock in an eight-file workspace; splitting it across workers would cost more coordination than it saves, and nothing here asked for it.

**What I'd report back:** the prototype path and how to open it; the nine screens mapped to US-001 through US-004; that `packages/compass` and the product code are untouched and why; the three guild questions with my recommendation on each; the six spec questions for Tom; and the plain statement that `apps/web` cannot currently be run in this workspace (no package manifest, and `InvoicesPage` imports a module that doesn't exist), so the mock deliberately does not depend on it.