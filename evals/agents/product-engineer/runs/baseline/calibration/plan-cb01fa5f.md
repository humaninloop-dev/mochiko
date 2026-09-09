FILES-READ: `.mochiko/specs/recurring-invoices/spec.md`, `README.md`, `packages/compass/README.md`, `packages/compass/tokens.css`, `packages/compass/COMPONENTS.md`, `packages/compass/src/StatusPill.tsx`, `apps/web/src/pages/InvoicesPage.tsx`, `apps/api/fixtures/dev-seed.json`

---

# Action plan — clickable mock for Recurring invoices

Target: a static, no-install click-through walkable at Monday's review (2026-09-14), living at `.mochiko/specs/recurring-invoices/prototype/` as the repo's working agreement requires, plus the manifest that fills the spec's empty **Screens & Flows** section, plus a findings list Tom needs before the stories are frozen.

---

## Phase 1 — Load the authoring procedure and fix the manifest format

**Do:** Invoke the `mochiko:authoring-prototype` skill and follow its structure and manifest format exactly; it, not this plan, decides the manifest's file name, location, and field set.

**Read:** the skill.

**Default I proceed under if the skill leaves a choice:** manifest file inside `prototype/`, and the spec's `## Screens & Flows` section replaced with the screen/flow inventory (the spec literally says "_To be filled by the prototype._", so leaving it untouched is not an option).

**Refuse:** starting to write HTML before the manifest shape is known — the screen list and the manifest are the same artifact and I don't want to retrofit IDs.

---

## Phase 2 — Reconcile the stories against Compass, and freeze the decision list

No files written. This is the phase that produces most of the value for Monday, because Compass blocks two things the stories assume.

**What I already know from the catalogue and cannot design around:**

1. **There is no "Recurring" nav section.** `SideNav` is a fixed list — Dashboard · Invoices · Clients · Expenses · Reports · Settings — and "adding a section is a guild decision." US-002 says "when I open Recurring." The feature has no front door.
2. **There is no schedule status pill.** `StatusPill` variants are a closed set (`draft`/`sent`/`paid`/`overdue`/`void`, confirmed by the union type in `StatusPill.tsx`). Schedules are active / paused / ended / failed — four states, none of which exist. Adding a variant is a guild decision.
3. **There is no "repeat every…" picker** — the catalogue says so explicitly. Cadence and the end condition must be composed from `Select`, `RadioGroup`, and `Field`. That's composition of existing parts, so I treat it as allowed, not as inventing a pattern — but I'll say so out loud at the review rather than let it pass unnoticed.
4. **There is no timeline/history list.** The fixture carries `sent_count` and `failure.at`, which invite a send-history view. **No story asks for one, so I will not draw one.** It goes in findings.

**Stops for a human ruling (I would raise all four in one message to Tom, not four times):**

| # | What I'd confirm | Default I plan under | Branch if ruled otherwise |
|---|---|---|---|
| A | US-004 "given a way to fix it" — what *is* the fix? Unarchive the client, reassign the schedule to another client, or just end it? | `Banner` (error) with the reason and one action link, **Restore client**, going to Clients | *Reassign* → the fix needs a client `Select` in a `Drawer`, one more screen and a new flow step. *End it* → no new screen, the fix collapses into the existing end-confirm flow. Either way the manifest and the flow keying change, so I'd rather ask than guess twice. |
| B | Entry point for "make recurring": a row `Menu` on the Invoices table, or an invoice **detail** page? This workspace has only `InvoicesPage.tsx` and no detail page or router, so I can't tell whether a detail page exists and was trimmed. | Row `Menu` on the Invoices list (`DataTable` supports an optional row `Menu`), opening a `Drawer` | Detail page exists → the entry becomes a `PageHeader` action on that page and I add an invoice-detail screen to the mock. |
| C | Nav section + status pill: are these going to Thursday's guild? Thursday **is today**, and the review is Monday — so a guild ruling could land before the walk-through. | Render both as visibly *proposed* (see Phase 4) and put them on today's guild agenda | If the guild rules before Monday, I re-render to the ruling and the proposal markers come out. |
| D | US-003 Scenario 1 says "until I **resume** it" but no scenario covers resuming. Is resume in scope? | Yes — render `Resume` in a paused row's `Menu`, and record "resume has no scenario" as a finding | Out of scope → the Menu item comes out and pausing becomes one-way, which I'd argue against on the spot. |

I do **not** wait on these; I build under the defaults and mark each affected element so the review can overturn it cheaply.

---

## Phase 3 — Placeholder data with honest shape

**Read (already done):** `apps/api/fixtures/dev-seed.json`.

**Do:** use the seven seeded schedules verbatim rather than inventing tidy ones. They already carry the awkward shapes I need on screen:

- `cl_01` — *"Bramblewood Landscape Architecture & Garden Design Partnership LLP"* — exercises the catalogue's truncate-with-ellipsis behaviour in the Client column.
- `Tŷ Newydd Holiday Cottages` — non-ASCII, keeps the type honest.
- `rs_04` is **EUR** while the account is **GBP** — mixed currency in one table. This is real: FR-001 says the schedule copies the *invoice's* currency, but `Field`'s money input takes "currency prefix from the **account**." That's a contradiction the create form has to face. Finding.
- `rs_06` is ended with `next_send: null` — forces me to decide and show what an empty Next send cell reads as (I'll render "—").
- Amounts spanning 75.00 → 6,200.00, and I'll respect the noted 15.00–18,500.00 range in the invoices table so column widths are tested at the extremes.
- Seven rows is above the noted median of 5 and, like the p95 of 23, still **under `DataTable`'s 25-row pager threshold** — so the Recurring list renders with no pager, and I'll note that the pager is essentially dead code for this surface.

**The invoices list is different:** the account has **214** invoices, so that table *does* page. I'll render 25 rows plus the pager, which surfaces a finding on its own — **no story says how a freelancer finds the right invoice among 214 to make it recurring.** There is no search or filter in any story or in the catalogue's `DataTable`.

The seed has no `invoices` array, so invoice rows are mine to invent: `INV-####` numbering consistent with the schedules' `source_invoice` values, statuses drawn only from the closed pill set, and the row I recur in the demo is a **paid** one so the flow matches US-001's independent test.

**Pretend-today = 2026-09-14** (review day), stated on the cover page. I checked the seed against it: every active schedule's `next_send` (09-15, 09-30, 10-01) still reads as future on Monday, and `rs_05`'s failed 09-01 reads as past. Nothing needs redating.

---

## Phase 4 — Build the skeleton

**Write:**
- `.mochiko/specs/recurring-invoices/prototype/index.html` — cover: what this is, that it's rough on purpose, pretend-today, the list of walkable flows keyed to scenario IDs, and the open decisions A–D.
- `.mochiko/specs/recurring-invoices/prototype/prototype.css` — one stylesheet, `@import`ing the real `../../../../packages/compass/tokens.css` so the mock cannot drift from the system and every colour, space, radius, and type size is a token *by name*. **Tradeoff I'd state:** this makes the folder non-portable outside the repo; since the walk-through happens from the repo, I take the no-drift side over the emailable side. If someone needs to send it standalone, I vendor a dated copy instead.

The stylesheet gives me rough stand-ins named after the catalogue — `AppShell`, `SideNav`, `PageHeader`, `DataTable`, `StatusPill`, `EmptyState`, `Banner`, `Toast`, `ConfirmModal`, `Field`, `Select`, `RadioGroup`, `Drawer`, `KeyValue`, `Button`, `Menu` — as plain CSS classes. Structure (sidenav at `--cp-sidenav-width`, content at `--cp-content-max`, 4-point spacing, 25-row pager rule, zebra rows, truncation) is rendered **precisely**, because it's what the build must honour. Everything advisory stays deliberately unfinished: no shadows, no icons beyond glyph placeholders, no illustration, no hover choreography, no real logo.

**Two elements get an explicit "proposed" marker** — a dashed outline and a footnote link on the cover, so nobody can sign these off by accident:
- the **Recurring** item in `SideNav`,
- the schedule status indicator, which I render as a rough `ScheduleStatus (proposed)` element rather than minting fake `StatusPill` variants. **I refuse to map schedule states onto existing pill variants** (active→sent, ended→void); it would look correct and be a lie about what Compass supports.

Interactions are plain `<a href>` between static files plus a few `<details>`-style toggles for the `Menu`. **No framework, no build, no npm.** `npm run dev` is not part of this deliverable.

---

## Phase 5 — Screens

Every screen below traces to a story; nothing else gets drawn.

| File (in `prototype/`) | Screen | Earns its place from |
|---|---|---|
| `invoices.html` | Invoices list, 25 rows + pager, row `Menu` open on a paid invoice showing **Make recurring** | US-001 S1 (entry), decision B |
| `make-recurring.html` | `Drawer` over Invoices: read-only copied summary (client, line items, currency — FR-001), cadence `Select` monthly/quarterly (FR-002), first-send date `Field`, end-condition `RadioGroup` (on a date / after N sends) with the conditional `Field` | US-001 S1 |
| `make-recurring-error.html` | Same `Drawer`, first send = 2026-09-01, `Field` error text "Date must be today or later", nothing created | US-001 S2 |
| `recurring.html` | Recurring list — exactly the five columns the story names (client, amount, cadence, next send, status), plus a row `Menu`; all seven seeded rows, no pager | US-002 S1 |
| `recurring-created.html` | Same list with the new 6-send monthly schedule at next month's date + `Toast` | US-001 S1 / its independent test |
| `recurring-empty.html` | `EmptyState`: "how to create one from an invoice" — its action goes to **Invoices**, not a "New schedule" button, because FR-001 says a schedule can only start from an invoice | US-002 S2 |
| `recurring-paused.html` | The paused row after Pause, `Toast`, `Menu` now offering **Resume** | US-003 S1 |
| `recurring-end-confirm.html` | `ConfirmModal`, `danger` confirm, body stating it cannot be resumed | US-003 S2 |
| `recurring-ended.html` | Ended row, `Next send` = "—", `Menu` with **no** Resume | US-003 S2 / FR-004 |
| `recurring-failed.html` | `Banner` (`error`) under the `PageHeader`: "Sunrise Yoga Collective CIC has been archived — the 1 Sep invoice was not sent," with the fix action link; the row reads failed | US-004 S1 |

**Deliberately not built** (each becomes a finding, not a screen): a single-schedule detail page (no story describes one — pause/end live on the row `Menu` and the failure reason lives in a `Banner`, so I don't need to invent it); a send-history timeline; editing a schedule's amount or cadence; unarchiving flow internals; anything about the fixture's "manual-send on / generated invoice still in draft" note.

---

## Phase 6 — Manifest and spec update

**Write:** the manifest per the skill's format, and replace `## Screens & Flows` in `.mochiko/specs/recurring-invoices/spec.md` with the inventory: every screen above with its ID, file, and the story it comes from; and seven flows, each keyed to one scenario —

F-1 → US-001 S1 · F-2 → US-001 S2 · F-3 → US-002 S1 · F-4 → US-002 S2 · F-5 → US-003 S1 · F-6 → US-003 S2 · F-7 → US-004 S1.

Every scenario in the spec is covered by exactly one flow, and no flow exists without a scenario. That's the property I check by hand before I hand it over.

---

## Phase 7 — Checks

No unit tests — this is static HTML and a test suite here would be theatre. What I run instead:

**Delegated, three separate disposable `Explore` subagents, each with an explicit `model: haiku` override** (a bare spawn would inherit the expensive tier), one gap each, terse findings with file+line:

1. *Link integrity* — "List every `href`/`src` in `.mochiko/specs/recurring-invoices/prototype/*.html` whose target file does not exist in that folder or at the given relative path." **On return:** I open any file it names; a broken `@import` to `tokens.css` would mean the whole mock renders unstyled at the review, so this one is load-bearing.
2. *Token discipline* — "List every hex colour, rgb(), or hard-coded px/rem in `prototype.css` and the HTML `style` attributes." **On return:** each hit must either become a token by name or be an intentional rough-sketch value I can justify aloud; I decide, not the agent.
3. *Vocabulary check* — "List every CSS class name in the prototype that starts with a component-like prefix, deduplicated." **On return:** I diff that list against `COMPONENTS.md` myself — completeness matters here, so the judgement is mine and only the enumeration is delegated.

**By hand, because absence would change a decision:** walk all seven flows in a browser start-to-finish, clicking only what a reviewer would click; confirm each dead end is a deliberate one; confirm the two "proposed" markers are impossible to miss; confirm the long client name actually truncates rather than blowing the column; confirm no pager renders on the Recurring list and one does on Invoices.

**I do not grade the prototype.** Whether it's good enough for Monday is the review's call and the reviewer's, not mine.

---

## Phase 8 — Report

I hand Tom the prototype link, the manifest, and the findings — the findings being the point, since the stories freeze after Monday:

**Blocking, needs today's guild:**
1. The feature has no front door — `SideNav` has no Recurring section and adding one is a guild decision. US-002 is unbuildable as written until this lands.
2. Schedule status has no representation — four states, and `StatusPill` is a closed set of five unrelated ones. Rendered as proposed in the mock; not a commitment.

**Story gaps the screens exposed:**
3. US-004's "a way to fix it" is undefined — I built *Restore client* as a default; three plausible fixes give three different screen counts (decision A).
4. Currency contradiction: FR-001 copies the *invoice's* currency, but the money `Field` prefixes the *account's*. The seed already has a EUR schedule on a GBP account, so this isn't hypothetical.
5. Nothing says how you find the invoice to recur among 214. No search, no filter, no story.
6. Resume is named inside US-003 S1 but has no scenario of its own — no acceptance criteria for what resuming does to the next send date.
7. Validation is specified only for a past first-send date. Nothing covers end-date-before-first-send, zero or negative send counts, or a quarterly schedule starting on the 31st.
8. Nobody is told a send failed except by opening Recurring — no story mentions email or any notification, yet US-004's whole premise is "before the client notices."
9. The fixture's manual-send accounts (generated invoice stays in draft) have no story at all.
10. Editing a schedule after creation has no story — the only paths out are pause and end.

**Structural notes for engineering:** the Recurring list will never page (p95 of 23 sits under the 25-row threshold); cadence + end condition are composed from `Select`/`RadioGroup`/`Field` because the catalogue has no repeat picker, and that composition wants a guild look; the mock is disposable and deliberately not a starting point for `apps/web`.