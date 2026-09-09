FILES-READ: `spec.md`, `design/cycles.md`, `design/data-model.md`, `README.md`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `.envrc.example`, `requirements.txt`, `.gitignore` (listing only), `src/shiftbook/__init__.py`, `src/shiftbook/app.py`, `src/shiftbook/models.py`, `src/shiftbook/cli.py`, `src/shiftbook/sms/textline.py`, `tests/fakes.py`, `tests/test_health.py`, `migrations/0001_init.sql`

---

# Action plan — author the verification cases for C1, C2, C3

## Phase 1 · Ground the cases in what actually exists

Already done above; recording what each read contributes, because every Setup leg I write has to be runnable against this repo, not an imagined one.

- `spec.md` — nine FRs, three stories with nine Given/When/Then scenarios, three success criteria, three screens, two flows. This is the source of expected behaviour; each case I write traces to at least one scenario or FR.
- `design/cycles.md` — the three cards, their claimed story/FR coverage, and the design seat's C3 note about the Textline quota.
- `design/data-model.md` — the state machine (`offered → awaiting_approval → completed | offered | expired`, plus `offered → withdrawn`), the partial unique index for one-open-request-per-shift, and "weekly hours computed from published shifts Mon–Sun in the site's timezone". My asserts name these states verbatim so the build and I cannot drift.
- `cli.py` — the demo seed is the exact cast the spec's independent tests name: Amira, Ben, Dana (barista); Chloe, Emil (kitchen); Priti (manager); published rota 2026-09-14…2026-09-20. My Setup legs use `make seed-demo` and these names.
- `Makefile`, `docker-compose.yml`, `ci.yml`, `.envrc.example` — the real substrate: Postgres 16 on 5435, `make migrate` applies `migrations/*.sql` in sorted order, CI runs `ruff check src tests` then `make migrate` then `pytest -q`. Three sandbox-registered handsets.
- `models.py`, `migrations/0001_init.sql` — only `staff` and `shifts` exist. `swap_requests` / `swap_events` need a `0002_*.sql`; my Setup legs say `make migrate` and my asserts read those tables directly.
- `textline.py`, `fakes.py` — the real adapter reads `TEXTLINE_API_KEY`/`TEXTLINE_FROM` from the environment at construction; `FakeTextline` records `(to, body)` pairs.

**What I would not do in this phase:** re-slice, move requirements between cards, or edit `spec.md`. The slicing is the design seat's; I write cases against the cards as sliced and flag seams rather than fixing them myself.

**Delegation:** none here, and I would say so rather than manufacture some. The whole workspace is ~17 small files and every read is one whose *absence* would change a decision (see the missing `shiftbook.pages` below) — that is exactly the reading I do myself. The place I would delegate is later, at execution time: handing a fully-specified Setup/Action leg to a disposable general-purpose worker on Sonnet to run and bring back raw output. That is not this card.

## Phase 2 · Build the coverage matrix and surface the seams

I would lay out requirement → card → case before writing a word of case text, so nothing falls between cards and nothing gets asserted twice with different expectations.

| Claim | Card | Where I put it |
|---|---|---|
| US-001 s1, FR-001 | C1 | C1-T1 |
| FR-002 (role leg) | C1 | C1-T2 |
| FR-003, US-001 s3 | C1 | C1-T3, C1-T5 |
| US-001 s2 (24h) | C1 | C1-T4 |
| FR-009 (offer/withdraw) | C1 | C1-T6 |
| SC-003 (5s) | C1 | timing assert inside C1-T1 |
| US-002 s1, FR-005 | C2 | C2-T1 |
| FR-004 | C2 | C2-T2 |
| US-002 s3, FR-002 (overlap leg) | C2 | C2-T3 |
| US-003 s1, FR-006 (approve) | C2 | C2-T4 |
| US-003 s2, FR-006 (decline) | C2 | C2-T5 |
| US-003 s3, FR-007 | C2 | C2-T6 |
| SC-001 | C2 | C2-T7, as measurability only |
| FR-008 | C3 | C3-T1…T5 |
| SC-002 | nowhere | Phase 7 |

Seams I would raise (all as flags on the card, none as unilateral edits):

1. **FR-002 is split across two cards.** C1 claims FR-002, but FR-002 has two halves — "holds the shift's role" *and* "does not already work an overlapping shift" — and the overlap half is C2's material (C2's card names the overlap rule). I would write C1 to cover only the role half and C2 to cover the overlap half, and say so explicitly in both cards' case text so neither seat assumes the other covered it.
2. **FR-007's trigger is ambiguous.** The spec scenario says "awaiting approval for **12 hours**, when the shift is now less than 24 hours away"; FR-007 and the data model both name only the <24h condition. Two readings: 24h-only, or 24h AND a 12h dwell. These produce different pass/fail on the same run.
3. **No expiry mechanism is specified anywhere** — scheduled job, lazy-on-read, or a CLI sweep. The Action leg of the expiry case cannot be written without knowing which.
4. **FR-009 says "who made it" but expiry has no human actor.** `SwapEvent.actor_id` needs a defined value for system transitions.
5. **FR-004's boundary.** "over 48 hours" — is landing at exactly 48.0 allowed? I will assert strict (48.0 allowed, 48.1 refused) and flag it.
6. **FR-008 recipient set on approval.** The story says "both staff receive a text"; it does not say the manager does. I will assert exactly the two the spec names and record, not fail, a third message if the build sends one.

### Stop 1 — a spec-clarification checkpoint (items 2–6)

I would take these to the product/design seats in one batch before finalising C2's expiry and 48h cases. What would be confirmed: the expiry trigger condition, the expiry mechanism and its entry point, the system actor value, the 48h boundary, and the approval recipient set.

- If 24h-only → C2-T6 as drafted below.
- If 24h + 12h dwell → C2-T6 grows a second Setup leg (a request taken 13h ago on a shift 23h out) and a negative twin (taken 2h ago, shift 23h out → *not* expired), and I would push back that FR-007 should be amended to say so.
- If a scheduled job → Action becomes "invoke the job function directly with the pinned clock"; if lazy-on-read → Action becomes "GET the approvals page and the board" and the assert is that the read itself performs the transition and writes the event.
- If the boundary is inclusive ("48 or more refused") → I flip the two boundary asserts.

**Default I continue under:** 24h-away is the sole trigger; the build must expose a directly-callable sweep so the case is deterministic rather than dependent on a cron; system transitions record `actor_id` NULL plus `to_state = expired`; 48.0 is allowed; approval texts go to exactly the two staff.

## Phase 3 · Decide the assertion plane, and flag three infrastructure gaps

Before writing Setup legs I have to know what can genuinely be executed. Three problems, all concrete to this checkout:

**(a) There is no shift page to extend.** C1's card tags `[EXTEND]` the shift page, and `app.py` comments that "Rota, shift page, and manager pages are mounted from `shiftbook.pages` (existing)". No `pages.py` or `pages/` exists — the package is `__init__.py`, `app.py`, `cli.py`, `models.py`, `sms/`. Either this checkout is missing a module or the brownfield tag is wrong. **I would flag this and not paper over it**: C1's Setup leg cannot say "open the existing shift page" if that route does not answer. I would write C1-T4 (the 24-hour case, which asserts on the shift page's rendered text) so its Setup leg begins with an explicit precondition check — `GET /shifts/{id}` returns 200 before the case proceeds — and note that if it does not, the case is blocked on the missing module, not failed.

**(b) Playwright is in the README but not in `requirements.txt` and not in CI.** README claims Playwright for browser checks; `requirements.txt` has fastapi/uvicorn/sqlalchemy/psycopg/jinja2/httpx/click/pytest/ruff and nothing else, and `ci.yml` runs only ruff, migrate, pytest. So no browser-level assert can be run automatically today. Since the app is server-rendered FastAPI + HTMX, the honest and CI-runnable plane is **HTTP request + assertion on returned HTML and on database rows**, and that is where I write the executable asserts. The genuine click-through of SCR-001/002/003 (FLOW-001, FLOW-002) I classify as **GUI** and write as a human checkpoint with screenshot evidence — not auto-approved, because I cannot execute a browser here and I will not claim an outcome I did not observe.

**(c) The seed fixture will rot, and it controls every 24h assert.** `seed-demo` publishes a rota for the fixed window 2026-09-14…2026-09-20. Today is 2026-09-10, so "next Tuesday 07:00" is 2026-09-15 — comfortably >24h away, and C1 passes. Run the same case on 2026-09-15 and the identical shift is *inside* 24 hours and C1-T1 fails while C1-T4 passes for the wrong reason. Every 24-hour and weekly-hours assert on these cards is date-sensitive. **This is the single biggest threat to these cases being reproducible**, so I would write every Setup leg against a pinned clock and relative seeding rather than the wall clock:

- a `tests/conftest.py` fixture pinning "now" to a fixed instant, injected wherever the app reads the current time (which means the build must not call `datetime.now()` inline — a real build constraint I would state on the cards);
- shifts seeded as `now + 5 days` / `now + 6 hours` / `now + 23 hours`, not as literal 2026-09 dates.

I would put this as a stated precondition at the head of the verification block for all three cards rather than repeating it in nine Setup legs.

Also noted for the build: `models.py` uses naive `DateTime`, while the data model says weekly hours are computed "in the site's timezone". Naive timestamps plus a Mon–Sun boundary is a latent off-by-one-day. I flag it; I do not fix it. My weekly-hours case seeds shifts well clear of the week boundary so it tests the 48h rule and not the timezone bug, and I add a separate flagged case for a shift straddling Sunday midnight only if the lead wants it in scope.

**(d) `FakeTextline` has no seam to enter through.** `Textline.__init__` reads `os.environ["TEXTLINE_API_KEY"]` eagerly and nothing in `app.py` constructs or injects it. For C3's cases to be runnable at all, the build must make the SMS adapter injectable (constructor argument or FastAPI dependency). I state that as a precondition on C3 rather than writing cases that cannot be set up.

## Phase 4 · Author C1's cases

I would replace `**TEST:** TBD` at line 16 of `design/cycles.md` with the block below. Format: Setup / Action / Assert per case, each named, each tagged CLI or GUI so the execution seat knows what auto-runs and what needs a human.

> **Precondition (all cases):** `docker compose up -d postgres`; `make migrate` (includes the new `swap_requests` / `swap_events` migration); `make seed-demo`. Clock pinned via the test fixture; shifts seeded relative to the pinned now, never as literal dates. Asserts read the database at `DATABASE_URL` directly as well as the HTTP response.

- **C1-T1 · an offered shift reaches an eligible colleague's board (CLI)** — US-001 s1, FR-001, SC-003.
  Setup: seeded café; Amira (barista) owns shift **S**, published, `now + 5 days`, 07:00–15:00.
  Action: as Amira, POST the offer confirmation for S; record the response timestamp; then as Ben (barista) GET the swap board; record that response timestamp.
  Assert: (1) the offer request succeeds and lands back on S's shift page; (2) `swap_requests` holds exactly one row for S with `state = 'offered'`, `offered_by = Amira`, `taken_by` NULL; (3) Ben's board HTML contains S with its date and 07:00–15:00 and a *Take this shift* control; (4) elapsed time between the two recorded timestamps is under 5 seconds. Evidence: both response bodies, the `swap_requests` row dump, the two timestamps and the delta.

- **C1-T2 · the board shows the offer to the right role and only that role (CLI)** — FR-002 (role leg only; the overlap leg is verified in C2-T3).
  Setup: state after C1-T1.
  Action: GET the board as Chloe (kitchen) and as Dana (barista).
  Assert: Chloe's board does not contain S and renders its empty state; Dana's board does contain S with a *Take this shift* control. (Dana is in the case deliberately — one colleague seeing it would not distinguish "role-filtered" from "shown to one person".)

- **C1-T3 · a shift cannot carry two open requests (CLI)** — FR-003.
  Action: as Amira, attempt a second offer on S both through the page and by POSTing the offer endpoint directly, bypassing any hidden control.
  Assert: both attempts are refused with a stated reason returned to the caller; `swap_requests` still has exactly one open row for S; the partial unique index exists on the table (`\d swap_requests` output captured); no `swap_events` row was written for either refused attempt.

- **C1-T4 · a shift inside 24 hours cannot be offered (CLI)** — US-001 s2, FR-001.
  Setup: shift **S2**, Amira's, published, starting `now + 6 hours`. Precondition check: `GET` S2's shift page returns 200 (see the missing-`pages`-module flag above; if it does not, this case is blocked, not failed).
  Action: GET S2's shift page as Amira; then POST the offer endpoint for S2 directly.
  Assert: the page renders no *Offer swap* control and does render the "talk to the manager" text; the direct POST is refused with a stated reason; `swap_requests` has no row for S2.

- **C1-T5 · the offerer can withdraw, and may then offer again (CLI)** — US-001 s3.
  Action: as Amira, withdraw the request on S; GET the board as Ben; then as Amira offer S again.
  Assert: the request row reaches `state = 'withdrawn'`; S is absent from Ben's board; Amira's shift page shows *Offer swap* again; the second offer succeeds and creates a new `offered` row. (This last leg is what proves FR-003 means "one *open* request", not "one ever" — the boundary C1-T3 alone would leave ambiguous.)

- **C1-T6 · every state change is recorded with actor and time (CLI)** — FR-009.
  Action: none; asserts over the rows produced by T1, T3 and T5.
  Assert: `swap_events` contains, in order, a row to `offered` with `actor_id = Amira`, a row `offered → withdrawn` with `actor_id = Amira`, and a row to `offered` with `actor_id = Amira` for the re-offer; every row has a non-NULL `at`; the refused attempts from T3 produced no rows. Evidence: full table dump for the request ids.

- **C1-T7 · FLOW-001 walked in a browser (GUI — checkpoint, never auto-approved)**.
  Setup: app on port 8500 against the seeded database.
  Action: a human, signed in as Amira, opens SCR-002 for S, clicks *Offer swap*, confirms; then signs in as Ben and opens SCR-001.
  Assert: the confirm step is reachable and legible; the board shows the offer with a working *Take this shift* control. Evidence: screenshots of both screens. I present this as a checkpoint with the screenshots and a recommendation; the human rules.

## Phase 5 · Author C2's cases

Replaces `**TEST:** TBD` at line 26.

- **C2-T1 · taking moves the request to awaiting approval and clears the board (CLI)** — US-002 s1, FR-005, FR-009.
  Setup: C1-T1 state; Ben's other published shifts that week total well under 48h; Ben has no shift overlapping S.
  Action: as Ben, POST *Take this shift*; then GET the board as Dana and as Ben; GET the approvals screen as Priti.
  Assert: `state = 'awaiting_approval'`, `taken_by = Ben`; S absent from both Dana's and Ben's boards; Priti's approvals list contains the request with *Approve* and *Decline*; a `swap_events` row `offered → awaiting_approval`, `actor_id = Ben`. *SMS on this transition is asserted in C3-T1, not here* — cross-referenced so neither card assumes the other proved it.

- **C2-T2 · a take that breaches 48 rostered hours is refused with the reason (CLI)** — FR-004.
  Setup: seed Ben with published shifts totalling **44 h** Mon–Sun of S's week (well clear of the week boundary, to keep this a 48h test and not a timezone test); S is 8 h, so taking it would reach 52 h.
  Action: as Ben, POST *Take this shift*.
  Assert: refused; the reason rendered to Ben names the 48-hour limit and his resulting total; the request stays `offered` with `taken_by` NULL; S is still on Dana's board; no `swap_events` row.
  Boundary leg: re-seed Ben at **40 h** so taking S lands at exactly 48.0 → the take **succeeds** (reading "over 48" as strict; flagged in Stop 1 and flips if product rules otherwise).

- **C2-T3 · an overlapping shift blocks the take (CLI)** — US-002 s3, FR-002 (overlap leg; the role leg is in C1-T2).
  Setup: Dana has her own published shift `S_dana` 06:00–14:00 on S's day, overlapping S's 07:00–15:00.
  Action: GET the board as Dana; GET S's shift page as Dana; POST the take endpoint for S as Dana directly.
  Assert: S carries no *Take this shift* control for Dana on the shift page (US-002 s3's wording) and is suppressed from her board (FR-002's wording) — both asserted, because the two documents word it differently; the direct POST is refused with a stated reason; Ben, with no overlap, is unaffected and still sees the control.

- **C2-T4 · approval moves the person on the rota (CLI)** — US-003 s1, FR-006, FR-009.
  Setup: state after C2-T1 (request awaiting approval).
  Action: as Priti, POST *Approve*.
  Assert: `state = 'completed'`; the published rota view for that week — the `[MODIFY]`-tagged query — shows **Ben** on S's slot and no longer Amira; the underlying shift row's `staff_id` is Ben's; a `swap_events` row `awaiting_approval → completed`, `actor_id = Priti`; the request is gone from Priti's approvals list. Evidence: rota page HTML before and after, plus the shift row before and after.

- **C2-T5 · declining returns the offer to the board with the reason (CLI)** — US-003 s2, FR-006, FR-009.
  Setup: a fresh offer on S taken by Ben and awaiting approval.
  Action: as Priti, POST *Decline* with reason "Ben needs till training".
  Assert: `state` is back to `offered`, `taken_by` NULL, `decline_reason` stored verbatim; the reason is rendered where staff can see it; S reappears on Dana's and Ben's boards; the rota is **unchanged** — Amira still on S, shift row `staff_id` unchanged; `swap_events` row `awaiting_approval → offered`, `actor_id = Priti`.

- **C2-T6 · a request awaiting approval expires once the shift is inside 24 hours (CLI)** — US-003 s3, FR-007, FR-009. *(Shape depends on Stop 1; drafted under my stated default.)*
  Setup: shift **S3**, Amira's, published, starting `now + 23 hours`, with a request in `awaiting_approval` taken by Ben.
  Action: invoke the expiry sweep directly against the pinned clock.
  Assert: `state = 'expired'`; S3's shift row still has `staff_id = Amira`; the request is absent from Priti's approvals list; S3 is **not** back on any board; a `swap_events` row `awaiting_approval → expired` with `actor_id` NULL. Negative leg: a second request on a shift at `now + 30 hours` is untouched by the same sweep — proving the sweep is conditional and not a blanket expiry.

- **C2-T7 · the request history can answer SC-001 (CLI, measurability only)** — SC-001.
  Action: query `swap_events` for every request produced by T1, T4, T5, T6.
  Assert: for each, a taken-at timestamp and a terminal-state timestamp (`completed` / `offered`-after-decline / `expired`) are both retrievable, so the "within 12 hours" duration is computable per request.
  Flagged explicitly on the card: **SC-001's 90 %-over-a-month threshold is post-launch telemetry and cannot be a cycle gate.** This case proves the data exists to measure it; nothing on C2 can prove the criterion itself. I would say this on the card rather than let a green tick imply SC-001 is met.

- **C2-T8 · FLOW-002 walked in a browser (GUI — checkpoint)**: Ben takes from SCR-001, Priti approves from SCR-003, the rota shows Ben. Screenshots of all three screens; presented for human ruling.

## Phase 6 · Author C3's cases

Replaces `**TEST:** TBD` at line 40.

> **Precondition:** the SMS adapter must be injectable. `Textline.__init__` currently reads `TEXTLINE_API_KEY` from the environment eagerly and nothing constructs it in `app.py`; C3's build must add a seam (constructor argument or FastAPI dependency) or none of these cases can be set up. `FakeTextline` is installed in that seam for T1–T5, per the design seat's quota note. Phone numbers asserted against the seeded staff records.

- **C3-T1 · a taken shift texts the offerer and the manager (CLI)** — FR-008, US-002.
  Setup: offer on S by Amira; fake adapter installed with an empty `sent` list.
  Action: as Ben, take S.
  Assert: `sent` has exactly **2** entries; recipients are exactly Amira's and Priti's seeded phone numbers; each body names Ben as the taker and S's date and 07:00–15:00; **no** message to Ben, Dana, Chloe or Emil. Evidence: the full `sent` list verbatim.

- **C3-T2 · approval texts both staff (CLI)** — FR-008, US-003 s1.
  Action: Priti approves.
  Assert: exactly the messages the spec names — one to Amira, one to Ben — each stating the swap was approved and naming the shift. If the build also texts Priti, I **record it as an observed extra and raise it**, rather than pass or fail it, since the spec says "both staff" and is silent on the manager (Stop 1, item 6).

- **C3-T3 · a decline texts both staff with the manager's reason (CLI)** — FR-008, US-003 s2.
  Action: Priti declines with "Ben needs till training".
  Assert: two messages, to Amira and Ben; the manager's reason appears **verbatim** in both bodies (not paraphrased, not truncated).

- **C3-T4 · an expiry texts both staff (CLI)** — FR-008, US-003 s3.
  Action: run the expiry sweep on the C2-T6 fixture.
  Assert: two messages, to Amira and Ben; bodies state the request expired and that the shift stays with Amira.

- **C3-T5 · refusals are silent (CLI)** — FR-008 negative.
  Action: attempt a second offer on an already-offered shift (C1-T3) and a 48h-breaching take (C2-T2), with a fresh fake adapter.
  Assert: `sent` is empty after both. A messaging layer that fires on rejected actions is a real and easy defect; nothing else on these cards would catch it.

- **C3-T6 · one live send proves the delivery path (CLI, real Textline, quota cost 1 — proposed, see Stop 2)**.
  Setup: real `Textline` adapter, sandbox credentials from `.envrc.local`, recipient = the first number in `TEXTLINE_TEST_NUMBERS`. Behind an environment marker so `pytest -q` in CI never triggers it.
  Action: send one approval-template message; poll `status()` on the returned id.
  Assert: `send` returns a message id and `status()` reports `delivered`. Evidence: the id, the status response, and the timestamp so quota use is auditable.

### Stop 2 — the fake-versus-real checkpoint

I would honour the design seat's instruction to write T1–T5 against `FakeTextline`; the reason given is real and specific and the fake genuinely proves what those cases assert — trigger, recipient set, and body text. But I would state plainly on the card what it does **not** prove: `FakeTextline.send` appends a tuple and returns a string; it never exercises credentials, the request shape, the endpoint, or delivery. Five green cases against it and FR-008 could still fail for every real staff member. What would be confirmed at the stop: whether C3-T6's single live send is authorised.

- Authorised → T6 runs once per cycle, by me, manually, one message of ten.
- Declined → I strike T6 and **carry an explicit unverified-delivery note into C3's verification report**, so the cycle closes with the gap on the record rather than hidden behind five passes.
- Countered with "test delivery in staging instead" → T6 moves to a release gate outside these cards and I note where it now lives.

**Default I continue under:** T6 stays on the card, marked as requiring authorisation before each run.

## Phase 7 · Dispose of SC-002

SC-002 — "Staff find the swap flow intuitive" — sits at the foot of `cycles.md` with no card. I would not invent a card for it; which bundles exist is the design seat's call, not mine. What I would do is write it as a **subjective checkpoint case** and hand it back to that seat with a note that it needs a home:

- **SC-002 · staff comprehension (SUBJECTIVE — checkpoint, never automated)**: with C1–C3 built and the demo café seeded, two staff who have not seen the feature attempt FLOW-001 and FLOW-002 unaided; the observer records where they hesitate or ask. There is no pass threshold to assert — the output is Ines's ruling as ops lead. Suggested placement: after C3, as a feature-level gate rather than a cycle gate.

I would flag that as written SC-002 has no measurable criterion at all, and recommend product either give it one (task completion without help, say) or accept it as an explicitly non-gating observation.

## Phase 8 · Audit the cases before handing them over

I would re-read the whole block I wrote against a short checklist, because a case that cannot be executed is worse than a TBD — it looks like coverage:

- Every case names a real seeded actor (Amira, Ben, Dana, Chloe, Priti) and real infrastructure (Postgres on 5435, `make migrate`, `make seed-demo`), never a mock — the one exception being C3's fake, which is flagged as a deviation with its reason and its limit stated.
- Every Assert is something observable and captured: an HTTP response, rendered HTML, a database row, a recorded timestamp, or the fake's `sent` list. No assert says "works correctly".
- No case depends on the wall clock; all are relative to the pinned now.
- No case asserts an implementation detail I invented — endpoint paths are described by their user-facing action ("the offer confirmation", "*Take this shift*") so the build picks the routes, while state names and table names come verbatim from `design/data-model.md`.
- Each case traces to a scenario or FR, and the matrix in Phase 2 has no empty cell except SC-002.
- Ordering dependencies are explicit (C2-T1 depends on C1-T1's state) so the execution seat cannot run them out of order and get a phantom failure.
- The cases the build will implement land under `tests/` where `ruff check src tests` will lint them — suggested files `tests/test_swap_offer.py`, `tests/test_swap_take_approve.py`, `tests/test_swap_notifications.py`, plus a `tests/conftest.py` for the pinned clock and relative seeding.

I would also note the boundary of my own seat here: I am authoring what each cycle must demonstrate. Executing these cases, and the code-shape audit of what gets built, come later against the built thing — nothing on this card lets me claim any of these pass.

## Phase 9 · What I would write and what I would report

**Files written:** `design/cycles.md` only — three edits, replacing the `TBD` at lines 16, 26 and 40 with the blocks from Phases 4, 5 and 6, plus the shared precondition paragraph and the SC-002 disposition appended to the SC-002 section at line 42. No other file touched; in particular I would not edit `spec.md`, add the missing migration, or add the SMS injection seam — those are build work, and I would leave them as stated preconditions on the cards.

**Report I would hand back:**

1. **Delivered** — 7 cases on C1, 8 on C2, 6 on C3, plus one subjective checkpoint for SC-002; every FR and every story scenario mapped, with the matrix included.
2. **Blocking on a human ruling** — Stop 1 (expiry trigger, expiry mechanism, system actor, 48h boundary, approval recipients) and Stop 2 (live-send authorisation). C2-T6 and C3-T2/T6 are provisional until these land; I state which way each drafted assert would flip.
3. **Preconditions the build must satisfy or these cases cannot run** — a `0002` migration for `swap_requests`/`swap_events` with the partial unique index; an injectable time source (no inline `datetime.now()`); an injectable SMS adapter; a directly-callable expiry entry point; a seed that computes dates relative to now.
4. **Findings raised, not fixed** — `shiftbook.pages` is referenced by `app.py` and tagged `[EXTEND]` on C1 but does not exist in this checkout; Playwright is in the README but absent from `requirements.txt` and CI, so all browser-level verification is a human checkpoint until that changes; `models.py` uses naive timestamps against a data model that specifies a site timezone for the Mon–Sun weekly-hours window; FR-002's two halves land on two different cards.
5. **Stated limits of the coverage** — SC-001 is post-launch telemetry and C2 only proves it is measurable; SC-002 is unmeasurable as written and has no card; and if Stop 2 goes against the live send, FR-008's actual delivery is unverified by anything on C3 regardless of how green it looks.