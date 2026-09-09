FILES-READ: `README.md`, `.mochiko/specs/recurring-invoices/spec.md`, `packages/compass/README.md`, `packages/compass/COMPONENTS.md`, `packages/compass/tokens.css`, `packages/compass/src/StatusPill.tsx`, `apps/web/src/pages/InvoicesPage.tsx`, `apps/api/fixtures/dev-seed.json`

---

# Action plan — recurring-invoices clickable mock

## What I already know from the read (drives everything below)

- The house rule is that a feature's mock lives at `.mochiko/specs/<feature>/prototype/` and is walked Monday **before the stories are frozen**. So the mock's job is to break the stories, not to flatter them.
- Compass is React + TypeScript behind an `@northstar/compass` import. I cannot render the real components without a build step, and a mock that needs `npm install` is a mock nobody opens. So I build in plain HTML/CSS/vanilla JS and reproduce Compass **vocabulary and tokens**, not Compass code.
- Three things the feature needs do not exist in Compass, and each is explicitly a guild decision (guild meets Thursday — *after* Monday). These are the spine of my findings, not blockers.
- `dev-seed.json` is a gift: it has real shape (a 71-character client name, a EUR schedule inside a GBP account, a null next-send, a never-sent schedule, an archived-client failure). I will use it verbatim rather than invent tidy data.

---

## Phase 1 — Load the authoring procedure, fix the output contract

**Do:** Load `mochiko:authoring-prototype`. It is the source of truth for the prototype's structure, the manifest's format and filename, and the invariants I must hold. Everything below about *where files go* is my working default and I reconcile it to the skill on load — the skill wins.

**Working defaults if the skill is silent:**
- Prototype root: `.mochiko/specs/recurring-invoices/prototype/`
- Manifest: `.mochiko/specs/recurring-invoices/prototype/screens-and-flows.md`
- The spec's `## Screens & Flows` section (currently `_To be filled by the prototype._`) gets replaced with a short screen/flow index pointing at the manifest — not a duplicate of it, so the two cannot drift.

**Refuse:** I will not edit any other part of `spec.md`. The stories are Tom's and they are under review Monday; my job is to surface problems with them in findings, not to quietly patch them.

---

## Phase 2 — One delegated sweep, then stop delegating

The workspace is eight files and I have read all of them, so manufacturing delegation would cost more than it saves. There is exactly one gap worth a cheap read, because I am about to claim in a finding that a nav section set is closed:

**Delegate:** one disposable `Explore` subagent, `model: haiku`.
**Brief:** "In `apps/` and `packages/`, list every distinct string passed as the `section` prop to `AppShell`, and every distinct string passed as `variant` to `StatusPill`. Report file path and line for each. Do not summarise or interpret; if there are none beyond the two files named, say so."
**On return I check:** that it names `apps/web/src/pages/InvoicesPage.tsx:7` (`section="invoices"`) and `packages/compass/src/StatusPill.tsx:3` (the five-variant union), and reports nothing else. If it returns a section or variant I have not seen, my findings F1/F2 below are wrong as stated and I re-read before writing them.

Everything else — reading the stories for implied screens, judging what the fixture shape means for layout — is interpretive and completeness-sensitive. I do that myself.

---

## Phase 3 — Skeleton before screens

**Write:**
- `prototype/index.html` — the walkthrough entry: a numbered list of the flows, each labelled with the story and scenario it renders, so a reviewer can start anywhere. Also carries the **fidelity legend** (below).
- `prototype/assets/shell.css` — the AppShell frame only: 240px SideNav, PageHeader band, content column capped at 1120px, page background on the sunken surface.
- `prototype/assets/compass-lofi.css` — rough stand-ins for the catalogue components I need, with class names taken verbatim from `COMPONENTS.md` (`.cp-datatable`, `.cp-status-pill`, `.cp-empty-state`, `.cp-banner`, `.cp-toast`, `.cp-confirm-modal`, `.cp-field`, `.cp-select`, `.cp-radio-group`, `.cp-drawer`, `.cp-menu`, `.cp-button`, `.cp-key-value`). Names match so Monday's conversation uses one vocabulary.
- `prototype/assets/proto.js` — small vanilla script for menu open/close, drawer open/close, modal confirm. No framework, no bundler, no network.
- `prototype/data/schedules.js` — `dev-seed.json` transcribed as a `const`, unchanged, with a header noting it is a verbatim copy of `apps/api/fixtures/dev-seed.json` as of 2026-09-10.

**Tokens:** every colour, space, radius and type value in my CSS is `var(--cp-*)`. I link the real file rather than copying it, so the mock cannot drift from the system:
`<link rel="stylesheet" href="../../../../packages/compass/tokens.css">`.
If that relative depth turns out wrong when I place the files, I fall back to a verbatim copy at `prototype/assets/tokens.css` with a dated provenance header — never to hand-typed hex.

**Fidelity legend** (rendered on `index.html` and repeated as a footer strip on every screen): grey dashed outline = *rough, advisory, do not read as a visual commitment*; solid = *structurally binding*. Anything that departs from the Compass catalogue also carries a small orange dot linked to its finding number. This is how I keep the mock from being signed off for its looks.

**Test:** open `index.html` from `file://` with no server and click every link. Expect: every screen loads, tokens resolve (a text colour of `#17202A`, not browser-default black), zero console errors, zero network requests.

---

## Phase 4 — The screens (only what a story asks for)

Ten states across four surfaces. Every one is keyed to a scenario in the manifest.

| File | Renders | Notes |
|---|---|---|
| `invoices.html` | US-001 entry point | The existing Invoices list, redrawn at low fidelity straight from `InvoicesPage.tsx` — same five columns, same warning Banner for overdue count. Adds a row `Menu` (DataTable supports one) with **Make recurring**, enabled on `sent`/`paid` rows and disabled elsewhere per US-001. This screen exists *only* to host the entry point; I say so in the manifest so nobody thinks Invoices is in scope. |
| `make-recurring.html` | US-001 S1 | Drawer (480px) over the invoices list. Read-only `KeyValue` block showing the client, line items and currency carried across, which is FR-001 made visible. Then: cadence `Select` (monthly / quarterly — the closed set from FR-002), first-send `Field` (date), and a `RadioGroup` for the ending — *on a date* / *after N sends* — each revealing one `Field`. Composed from primitives because the catalogue states outright there is no "repeat every…" picker. |
| `make-recurring-error.html` | US-001 S2 | Same drawer, first send set to 2026-09-03. `Field` error text in danger: the date must be today or later. Drawer stays open, list behind is unchanged — nothing created. |
| `recurring.html` | US-002 S1 | The list. Exactly the five columns the story names — client, amount, cadence, next send, status — plus a row `Menu`. All seven fixture rows. |
| `recurring--density.html` | US-002 S1 at p95 | Same screen, 23 rows (the fixture's stated p95). Proves the layout holds and that **no pager renders**, since DataTable only paginates at 25. Not a new screen and not new scope — the same screen at honest cardinality. |
| `recurring--empty.html` | US-002 S2 | `EmptyState`: heading, one-line body explaining schedules start from an invoice, primary Button to Invoices. |
| `recurring--created.html` | US-001 S1 + its independent test | Post-create: `Toast`, new row for Okafor Dental, monthly, ends after 6 sends, next send **2026-10-10**. Matches the story's own acceptance test from today's date. |
| `recurring--paused.html` | US-003 S1 | rs_01 now paused; its row Menu now offers Resume. |
| `end-confirm.html` | US-003 S2 | `ConfirmModal`, danger confirm, ghost cancel, body naming the client and the remaining sends. |
| `recurring--ended.html` | US-003 S2 + FR-004 | rs_01 ended. Resume is **absent from the menu**, not disabled — an ended schedule is not resumable, and a greyed control implies a state you could get back to. |
| `recurring--failed.html` | US-004 S1 | rs_05 failed. Error `Banner` under the PageHeader naming the archived client and the reason in plain words. The "way to fix it" is the open question — see Phase 5. |

**Data discipline — the four fixture facts I will not sand off:**
- `cl_01`'s 71-character name goes in the client column, so the truncation-plus-tooltip behaviour is visible rather than assumed.
- `rs_04` is EUR inside a GBP account. The list shows mixed currencies; the money `Field` in the drawer shows the account prefix. That mismatch is deliberate and becomes a finding.
- `rs_06` has `next_send: null`. The cell renders as an em-dash, not blank, not "N/A" — and I flag that nothing specifies it.
- `rs_07` has `sent_count: 0` and next-send equal to first-send: a schedule that has never fired.

**Scope I refuse to render:** no schedule detail page, no edit-a-schedule flow, no history/timeline, no "skip this send", no notification or email preview. No story asks for any of them. Several are tempting — the failure case in particular feels like it wants a detail view — and each one goes in findings instead of into the mock.

---

## Phase 5 — Stops: three guild items and one story hole

These are where I would stop for a human. I do not block on any of them; I render under a stated default, mark it in the mock with the orange dot, and put it on Monday's agenda.

**Stop 1 — "Recurring" is not a nav section.** US-002 says "when I open Recurring", but SideNav's sections are a closed set (Dashboard · Invoices · Clients · Expenses · Reports · Settings) and adding one is a guild decision. The guild meets Thursday, after Monday.
*Confirm with Tom and the guild:* is Recurring a seventh section, or a tab inside Invoices, or a filtered view of Invoices?
*Branches:* seventh section → my mock is already right, guild ratifies. Tab under Invoices → the shell changes on every screen and US-002's wording needs rewording. Filtered view → the list collapses into Invoices and roughly half the mock is redrawn.
*My default:* render it as a seventh SideNav item, dotted and dotted-marked, because it is the reading the story's own words force.

**Stop 2 — StatusPill has no variant for a schedule.** Its variants are a closed set of five invoice states (`draft` `sent` `paid` `overdue` `void`), and a schedule is `active` / `paused` / `ended` / `failed`. Four new variants, and adding one is a guild decision.
*Confirm:* new StatusPill variants, or a separate component, or reuse of existing tones?
*Branches:* new variants → tone mapping needs a guild ruling (my read: active→success, paused→muted, ended→border/void tone, failed→danger — and note that **Compass has no danger-toned pill today**, its warning tone is the closest). Separate component → its own guild item. Reuse → `paused` and `ended` become visually identical to `draft` and `void`, which I think misleads.
*My default:* four new variants with the mapping above, rendered dotted.

**Stop 3 — the cadence/ending control has no Compass pattern.** The catalogue names this gap explicitly. I compose it from `Select` + `RadioGroup` + `Field`. If the guild wants a real recurrence control, the drawer's whole middle section changes.
*Default:* composed from primitives, as the catalogue's own fallback implies.

**Stop 4 — US-004 says "I am given a way to fix it" and never says what the fix is.** This one is a genuine hole in the story, not a system gap. The reason is `client_archived`; the fix could be unarchive the client, reassign the schedule to another client, end the schedule, or retry. Each implies a different screen and at least one implies touching the Clients feature.
*Confirm with Tom before Monday if possible, at Monday otherwise.*
*Branches:* unarchive → a cross-feature confirm, and someone must decide what happens to the missed send. Reassign → a client picker that contradicts FR-001's "copies the client". End it → cheapest, but it is not really a fix.
*My default for the mock:* the error Banner carries an action link labelled **Fix this schedule** that opens a stub panel listing the candidate fixes as an unresolved question, visibly marked as an open decision rather than a design. I will not invent a resolution and let it get walked past as if it were settled.

---

## Phase 6 — Manifest and findings

**Write** `prototype/screens-and-flows.md` (format per the skill): every screen with its file, its purpose, the Compass components it uses, and the story-and-scenario it comes from; every flow as an ordered click path keyed to one scenario — US-001 S1, US-001 S2, US-002 S1, US-002 S2, US-003 S1, US-003 S2, US-004 S1 — so a reviewer can walk each acceptance test literally. Each screen also states which parts are binding and which are rough.

**Write** the findings, numbered, with the four stops above plus:

- **F5 — no entry point is specified.** US-001 says "given a sent or paid invoice, when I choose to make it recurring" but never says from where. There is no invoice detail page in `apps/web/src/pages/`. I default to a DataTable row Menu; a detail page would be a different design.
- **F6 — `overdue` and `void` invoices are unaddressed.** The story says sent or paid. An overdue invoice is a sent invoice. My mock disables the action on overdue and void; that is a guess.
- **F7 — a failed schedule's available actions are undefined.** US-003 S2 covers "active or paused". Can a failed schedule be ended? Paused? It has to be endable or it is unclearable.
- **F8 — resume has no scenario.** US-003 S1 depends on resuming ("until I resume it") but no scenario or requirement covers it, and no FR mirrors FR-003/FR-004 for it.
- **F9 — mixed currency.** `rs_04` is EUR in a GBP account, but the money Field takes "currency prefix from the account". FR-001 says currency is copied from the invoice. The list therefore mixes currencies with no stated totalling or formatting rule.
- **F10 — manual-send accounts.** The fixture notes two beta accounts whose generated invoice stays in draft. No story covers a schedule that fired successfully but produced a draft. That is arguably a third success state.
- **F11 — the source invoice is not frozen.** FR-001 copies line items, client and currency at creation. Nothing says what happens when that invoice is later edited or voided.
- **F12 — no functional requirements cover US-002 or US-004.** FR-001 to FR-004 source only US-001 and US-003. The list view and the failure notification have acceptance criteria but no requirements.
- **F13 — an ended schedule has no next send.** `rs_06`'s `next_send` is null and US-002 S1 lists next send as a column. I render an em-dash; unspecified.
- **F14 — "ends after N sends" progress is invisible.** The fixture tracks `sent_count` against a count-type ending, but US-002 S1's column set omits it, so a user cannot see that a 12-send schedule is 6 in. I did not add the column — adding it would be scope no story asked for — but the omission looks unintentional.

---

## Phase 7 — Verify, then hand off

**Checks I run myself** (no test framework — this is a static mock, and adding a runner would be exactly the toolchain I refuse):
1. Open `index.html` from the filesystem with no server; walk all seven flows. Expect every path reachable and every path returning to the list.
2. Search my own CSS for raw hex, px and rem values outside the token file. Expect zero hits — every value should be a `var(--cp-*)`. This is the one check that most reliably catches me drifting off the design system.
3. Confirm every class name I invented appears in `COMPONENTS.md`. Any that does not is either a naming mistake or an undeclared new pattern.
4. Confirm the density screen renders 23 rows and no pager, matching DataTable's stated 25-row threshold.
5. Confirm the `recurring--ended` screen has no Resume control anywhere in the DOM, not merely a disabled one — FR-004 made structural.
6. Grep the whole prototype for `http`, `import`, `require`, `script src=` pointing off-disk. Expect nothing but the tokens stylesheet link.

**Refuse:** I do not grade this prototype. I built it; I am the wrong reader for whether it walks well. It goes to Tom and to Monday's review to be judged.

**What I report at the end:**
- The prototype path, and that it opens by double-clicking `index.html` — no install, no `npm run dev`, no API.
- The manifest path and the seven flows, each named by the story scenario it walks, so the review can be run as a checklist against the acceptance criteria.
- The four stops, top of the report, each with its default and its branches — Recurring's place in the nav, the four missing StatusPill variants, the missing recurrence pattern, and the undefined fix for a failed send. The first three need Thursday's guild; the review should decide whether it can freeze the stories Monday without them.
- The fourteen findings, marked by which ones change the stories versus which ones only change the design.
- The plain statement that this mock is disposable and is not the starting point for the build — the structure it settles is the output, and the code is scaffolding.