FILES-READ: `.mochiko/specs/recurring-invoices/spec.md`, `README.md`, `packages/compass/README.md`, `packages/compass/tokens.css`, `packages/compass/COMPONENTS.md`, `packages/compass/src/StatusPill.tsx`, `apps/web/src/pages/InvoicesPage.tsx`, `apps/api/fixtures/dev-seed.json`

---

# Action plan — clickable mock for Recurring invoices

## Phase 0 — Load the authoring procedure

Invoke `mochiko:authoring-prototype` before writing anything; it is the source of truth for the manifest's shape and the prototype's structural invariants. Everything below about *what* goes in the mock stands; the exact manifest layout defers to it. (Not loaded in this plan-only run, so I describe manifest content by intent rather than by its field names.)

No delegation here — the whole workspace is eight files and I have read all of them. There is no locate, sweep, or enumeration left worth spending a subagent on at this stage. Delegation appears in Phase 6, where the work is deterministic checking rather than reading.

## Phase 1 — Pin the constraints, and name the two that block

Already read: the Compass catalogue, tokens, the one shipped component source, the Invoices page as the reference for how a real Northstar screen is assembled, and the dev fixtures.

Three hard facts shape everything:

1. **`StatusPill` variants are a closed set** — `draft · sent · paid · overdue · void`, enforced in the type in `packages/compass/src/StatusPill.tsx`. A schedule's states are `active · paused · ended · failed`. Not one of them maps. The catalogue says adding a variant is a guild decision.
2. **`SideNav` sections are a closed set** — Dashboard · Invoices · Clients · Expenses · Reports · Settings. US-002 says "when I open **Recurring**". There is no Recurring section. Adding one is a guild decision.
3. **Compass has no "repeat every…" picker and no timeline/history list.** The cadence-and-end form must be assembled from `Select`, `RadioGroup`, and `Field` alone.

**Stop #1 — the guild items.** Both (1) and (2) are decisions the working agreements reserve for the design guild, which meets Thursday. The design review is Monday. I cannot get a ruling before the artefact is needed, and I will not quietly invent either pattern.

- What would be confirmed: does Recurring become a seventh nav section, or a tab inside Invoices? And does the schedule-status pill become four new `StatusPill` variants, a second component, or reuse of existing ones?
- Branch — *new nav section*: single top-level `Recurring` entry; the Invoices page keeps only the row action.
- Branch — *tab under Invoices*: Recurring lives as a second tab on the Invoices page; the mock's nav frame is unchanged but Compass gains a tab pattern it also does not have — which is itself a guild item, so this branch does not avoid the guild, it relocates it.
- Branch — *reuse existing pill variants*: rejected on sight if proposed. Painting `active` as `paid`-green teaches the review a false vocabulary.
- **Default I proceed under:** render Recurring as a top-level nav item and render schedule status in a pill-shaped element that is visually a pill but is *not* labelled or described as `StatusPill` anywhere. Both carry a visible "not in Compass — guild decision" marker in the mock chrome and an entry in the findings. The mock's job is to make the guild question concrete for Thursday, not to pre-empt it.

**Refusals fixed here, before any file is written:**
- I will not add a variant to `packages/compass/src/StatusPill.tsx`, or a section to `SideNav`. That is production design-system code and a guild call.
- I will not build this into `apps/web` as React/Vite. A reviewer would have to install and run a dev server against a fixture API to look at a mock. The mock ships as static files that open from disk.
- I will not import from `@northstar/compass`. The mock *speaks* Compass's names; it does not depend on Compass.

## Phase 2 — Fix the screen and flow inventory from the stories, and nothing else

Seven scenarios across four stories. Every flow keys to exactly one of them; every screen has to be earned by one.

| Flow | Scenario | Screens walked |
|---|---|---|
| F1 Make a schedule | US-001 S1 | Invoices → row menu → create drawer → Recurring list with the new row + confirmation |
| F2 Past first-send date | US-001 S2 | Create drawer with error, nothing created |
| F3 See my schedules | US-002 S1 | Recurring list, populated |
| F4 First run | US-002 S2 | Recurring list, empty, with guidance back to invoices |
| F5 Pause | US-003 S1 | Recurring list → row menu → paused state |
| F6 End | US-003 S2 | Recurring list → row menu → confirm modal → ended, resume unavailable |
| F7 Failed send | US-004 S1 | Recurring list with error banner → failed schedule's reason and the fix |

**Stop #2 — one screen the stories do not quite ask for.** US-004 requires the user to see the *reason* and be *given a way to fix it*. A table row cannot carry a reason sentence plus a remedial action without becoming a screen of its own. No story asks for a schedule detail view.

- What would be confirmed: does US-004 imply a schedule detail surface, or should reason-and-fix live in a banner over the list?
- Branch — *detail exists*: it needs its own story, because it will also want to show cadence, end condition, source invoice, and sends-so-far, and none of that is currently specified.
- Branch — *banner only*: US-004 is satisfiable but thin, and the "way to fix it" has to be a single link.
- **Default:** render a minimal read-only detail panel — a `Drawer` of `KeyValue` rows plus an error `Banner` — opened from the failed row, containing only what US-004 needs. It is marked in the manifest as inferred rather than specified, and it appears in the findings as a screen looking for a story. I will not enrich it beyond the failure case.

**Scope I will not render**, each recorded as a finding instead: editing a schedule (amount, cadence, dates), deleting one, any list of past sends, a per-schedule notification setting, and any email or out-of-app alert for failures. Real products need all of these; no story asks for them; the mock stays silent so the silence is visible on Monday.

## Phase 3 — Build the frame, then fill it

Written to `.mochiko/specs/recurring-invoices/prototype/`, the location the working agreements name.

**Written first — the skeleton:**
- `prototype/tokens.css` — a verbatim copy of `packages/compass/tokens.css`, with a header naming the original as the source of truth and the copy as disposable. Copied rather than linked relatively, so the folder survives being zipped, moved, or opened from anywhere. Every value in the mock comes from these names; no raw hex, no raw pixel spacing.
- `prototype/prototype.css` — the AppShell frame: 240 px side nav from `--cp-sidenav-width`, content capped at `--cp-content-max`, page on `--cp-surface-sunken`, plus deliberately rough renderings of DataTable, Drawer, Banner, EmptyState, Menu, ConfirmModal, KeyValue, Toast, Field, Select, RadioGroup, Button. Rough on purpose: flat borders, no shadows, no easing, no icon set — grey blocks where icons go. The structure is the commitment; the surface must not read as one.
- `prototype/_shell.html` — the frame as one canonical block, hand-copied into each screen so the nav, header slot, and banner slot sit identically everywhere.

**Then the screens**, one file each, all linked with plain `<a href>`. Zero JavaScript: menus and the modal are `<details>` and separate pages, which keeps the mock openable from a `file://` URL with nothing installed.

- `invoices.html` — the Invoices list, rebuilt rough from `InvoicesPage.tsx` so the entry point is familiar. Row overflow menu offers **Make recurring** on `sent` and `paid` rows only.
- `invoices-menu-open.html` — the menu open on a paid row (F1 step 2).
- `new-schedule.html` — the create `Drawer`: read-only summary of the source invoice's client, line items, amount and currency (FR-001), a cadence `Select` of monthly/quarterly (FR-002), a first-send date `Field`, and an end-condition `RadioGroup`.
- `new-schedule-error.html` — same drawer, first-send date in the past, error text under the `Field`, nothing created (F2).
- `recurring.html` — the schedules list (F3), and the landing point of F5/F6/F7.
- `recurring-empty.html` — `EmptyState` pointing back to invoices (F4).
- `recurring-created.html` — the list after F1, new row present with its next send date, `Toast` visible.
- `schedule-paused.html`, `end-confirm.html`, `schedule-ended.html` — F5 and F6; the ended row's menu shows Resume present but disabled, which is how FR-004 becomes visible rather than asserted.
- `schedule-failed.html` — F7's reason-and-fix drawer.
- `index.html` — cover page: what this is, what it is not, the seven flows as links, and the guild-decision markers called out up front.

## Phase 4 — Placeholder data with honest shape

I use `apps/api/fixtures/dev-seed.json` as-is rather than inventing tidy rows. It is already doing the work a good fixture does, and copying it keeps the mock's shape honest:

- Seven schedules, against a median of 5 and p95 of 23. `DataTable` pages at 25, so a realistic account never sees a pager — I render none, and note in the findings that the p95 account sits one row under the threshold.
- `Bramblewood Landscape Architecture & Garden Design Partnership LLP` exercises the catalogue's truncate-with-tooltip rule in a real column, at a real width.
- Amounts spanning 75.00 to 6,200.00 against a stated live range up to 18,500.00 — I add one row near the top of the range to size the column honestly.
- All four statuses present, including `ended` with a **null** next-send date and `failed` with a next-send date **in the past**. Both are rendered, because both are cells someone has to decide the content of.
- `rs_04` is **EUR** in a **GBP** account. Rendered as-is, mixed, in one table.

## Phase 5 — Write the manifest and the findings

- `prototype/screens-and-flows.md` — the inventory: every screen with the story that earns it, every flow keyed to its scenario, the four Compass components used outside their catalogued range, and the two screens marked inferred rather than specified. Format per the skill.
- `prototype/FINDINGS.md` — the gaps the screens exposed. The ones I already hold:

  1. **The fixtures contain a schedule type the story forbids.** `rs_03` and `rs_06` have `ends: {type: "none"}` — no end date, no send count. US-001 S1 offers *either* an end date *or* a number of sends. Either the story is missing a third option or the fixtures are wrong. I build the two options the story gives, and render the two open-ended fixture rows in the list showing what the "Ends" column says for them — which is exactly the question. **This is the finding I would lead with on Monday.**
  2. **Resume has no scenario.** US-003 S1 says paused "until I resume it", but resuming has no given/when/then, and nothing says what happens to a next-send date that fell due while paused — does it skip, or fire immediately? The mock shows a Resume action and a paused row whose next-send date has already passed, and stops there.
  3. **"A way to fix it" is undefined.** US-004 gives no remedy. Unarchive the client, reassign the schedule, or end it? The mock renders one — a link to the archived client — labelled as an assumption, not a design.
  4. **Overdue invoices are ambiguous.** US-001 says "sent or paid". An overdue invoice was sent. Eligible or not? My default is yes, flagged.
  5. **Mixed currency is unhandled.** The money input takes its prefix from the account, but a schedule carries the source invoice's currency. `rs_04` proves it happens today.
  6. **Line items are copied but never shown.** FR-001 copies them; no scenario lets the user see or confirm what they signed up to send twelve times.
  7. **No story covers editing a schedule**, and an amount that never changes for a year is not how freelance work behaves.
  8. **Failure notification is in-app only.** "I want to be told" — the only surface any story implies is a banner the user must already be logged in to see.
  9. Plus the two guild items from Phase 1.

Then `spec.md`'s **Screens & Flows** section — it says "To be filled by the prototype", so filling it is invited. I confine the edit to that section, pointing at the manifest and listing the screens; I do not touch Tom's stories or requirements, and every disagreement I have with them goes in the findings for him to rule on, not into his document.

## Phase 6 — Deterministic checks

No test runner applies to a static mock, so the checks are mechanical and each has an expected result I state before running it.

**Delegation — three disposable `Explore` subagents, each with `model: haiku` set explicitly, one gap each:**

1. *Link integrity.* Brief: list every `href` in the prototype's HTML files and report any target file that does not exist. Expect: none missing. On return I confirm it covered all files I wrote — a short list is more likely a missed directory than a clean mock.
2. *Token compliance.* Brief: report every hex colour and every hard-coded pixel value in `prototype.css` and the HTML files, excluding `prototype/tokens.css`. Expect: zero. Any hit is mine to fix by name.
3. *Copied-fixture fidelity.* Brief: compare the schedule rows rendered in `recurring.html` against `apps/api/fixtures/dev-seed.json` and report mismatched client, amount, currency, cadence, next-send, or status. Expect: differences only on the one row I added at the top of the amount range. I check the returned diff line by line rather than trusting a "looks fine".

**By hand, because judgement is the point:** walk all seven flows myself, clicking, from `index.html`. Then re-read the four stories against the manifest, checking that every scenario has a flow (expect 7 of 7) and every screen traces to a story (expect zero orphans beyond the one marked inferred).

I do **not** grade the prototype. Whether it is good enough for Monday is the review's call and a reviewer's, not mine.

## Phase 7 — Report

I hand over: the walkable mock at `.mochiko/specs/recurring-invoices/prototype/`, its manifest, the findings, and the filled Screens & Flows section — and I say plainly that the mock is disposable and should be deleted once the stories are frozen.

The report leads with the three things that need a decision, not the file list: the **`ends: none` contradiction** between the fixtures and US-001, which the review can settle in a minute and which changes the create form; the **two guild items** — a Recurring nav section and four schedule-status pill variants — which need Thursday and which I have marked rather than invented; and **"a way to fix it"** in US-004, where I have rendered an assumption and need it either confirmed or replaced. Then the remaining findings, then the seven flows and how to click them.