FILES-READ: `spec.md`, `design/cycles.md`, `design/data-model.md`, `README.md`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `.envrc.example`, `requirements.txt`, `migrations/0001_init.sql`, `src/shiftbook/app.py`, `src/shiftbook/cli.py`, `src/shiftbook/models.py`, `src/shiftbook/sms/textline.py`, `tests/fakes.py`, `tests/test_health.py`

---

# Action plan — author the verification blocks for C1, C2, C3

## Phase 0 — Grounding facts I established from the read (these drive everything below)

Recorded before writing anything, because several of them change what a case can honestly assert:

- **Real infrastructure exists and is reachable**: PostgreSQL 16 on port 5435 (`docker-compose.yml`), applied by `make migrate` which globs `migrations/*.sql` in sorted order, app served by `uvicorn shiftbook.app:app --port 8500`. Cases will run against this, not against mocks.
- **CI runs exactly**: `ruff check src tests` → `make migrate` → `pytest -q`, with `DATABASE_URL` pointing at the service container. **CI sets no `TEXTLINE_*` variables.** Therefore any case that touches the real Textline client cannot run in CI as the pipeline stands. This is an observed fact, not an assumption — it decides the C3 shape.
- **`make seed-demo` is a stub.** `cli.py::seed_demo` only prints a sentence; it inserts nothing. Every case I write whose Setup says "Amira, Ben, Dana, Chloe, Emil, Priti, published rota 2026-09-14…20" is depending on a command that today does nothing. I will not write Setup legs that silently assume it works.
- **The "existing" shift page is not in this workspace.** `app.py` says pages are mounted from `shiftbook.pages (existing)`, but there is no `pages` module under `src/shiftbook/`. C1 calls the shift page `[EXTEND]`; there is nothing here to extend. Absence drives a decision, so I verified this myself rather than delegating it.
- **Playwright is claimed but absent.** README lists Playwright for browser checks; it is not in `requirements.txt` and not installed in CI. So screen-level cases cannot be machine-executed today.
- **Schema for the feature does not exist yet**: `0001_init.sql` has `staff` and `shifts` only. `swap_requests` and `swap_events` arrive with the build; the partial unique index in `design/data-model.md` is the mechanism behind FR-003.
- **Dates**: today is 2026-09-10; the seeded rota week 2026-09-14…20 is comfortably beyond 24 hours, so "eligible shift" and "inside 24 hours" fixtures both have to be pinned explicitly rather than left to relative time.
- **Spec wording I will hold the cases to**: FR-004 refuses a take that puts the taker *over* 48 hours — so exactly 48 must be **allowed**; that boundary gets its own assert. FR-008 notifies the offerer **and the manager** on *taken*, but only **both staff** on approve/decline/expire — the manager is not on those three. Cases will mirror this exactly and not "helpfully" add the manager.

## Phase 1 — Add a shared verification environment preamble

**Write:** `design/cycles.md`, a short `## Verification environment` section directly after the intro paragraph, so three cards' Setup legs can reference it instead of repeating it.

Contents: the bring-up sequence (`docker compose up -d postgres`; wait for `pg_isready`; `make migrate`; fixtures; `uvicorn shiftbook.app:app --port 8500`), the `DATABASE_URL`, the named cast (Amira/Ben/Dana barista, Chloe/Emil kitchen, Priti manager), and one explicit warning: **`make seed-demo` is currently a no-op, so every Setup leg below states its rows as concrete inserts rather than trusting the seed command.** Each case's Setup will therefore carry its own SQL, and I will note that if the build makes `seed-demo` real, the Setup legs can collapse onto it.

I would **flag** the missing `shiftbook.pages` module to the design/lead seat here rather than paper over it: C1 is scoped as `[EXTEND] the shift page`, and the page is not present. Branch: if the ruling is *the page exists in a branch/another repo*, C1's shift-page asserts stay as written against `GET /shifts/{id}`; if the ruling is *the page is also new*, C1's scope grows and that is the design seat's call, not mine — I would still author the same asserts, marked as covering a new surface. **Default while unruled:** author against `GET /shifts/{id}` returning the shift page, and record the dependency in the preamble.

## Phase 2 — Author C1's cases (US-001 all three scenarios; FR-001, FR-002, FR-003, FR-009; SC-003)

**Write:** replaces `**TEST:** TBD` at line 16 of `design/cycles.md`.

Shared Setup for the card: environment preamble; insert the six staff with the sandbox-registered numbers from `.envrc.example`; insert published barista shift **S1** = Amira, 2026-09-15 07:00–15:00, `published=true`; sign-in as the named actor per case.

- **C1-1 · offer lands and is visible to the right people** *(CLI/HTTP)*
  Action: as Amira, POST the offer on S1; then GET the swap board as Ben, as Dana, as Chloe, and as a fourth barista **Farid** seeded with an overlapping shift 2026-09-15 06:00–14:00.
  Assert: offer request is 2xx; exactly one `swap_requests` row with `shift_id=S1`, `offered_by=Amira`, `state='offered'`; board HTML for Ben and for Dana contains S1 with a *Take this shift* control; board HTML for Chloe (kitchen) does **not** contain S1; board HTML for Farid (overlapping) does **not** contain S1; S1's shift page as Amira shows *Withdraw*, not *Offer swap*.
  Evidence: HTTP status lines, the four board response bodies, the `swap_requests` row dumped from psql.

- **C1-2 · inside 24 hours cannot be offered** *(CLI/HTTP)*
  Setup adds published barista shift **S2** = Amira, 2026-09-10 20:00–23:00 (under 24 h from the pinned now).
  Action: GET S2's shift page as Amira; POST the offer on S2 anyway.
  Assert: page body contains no *Offer swap* control and does contain the "talk to the manager" wording; the POST is refused with a 4xx; `select count(*) from swap_requests where shift_id=S2` is 0. The direct POST matters — a hidden button is not an enforced rule.

- **C1-3 · one open request per shift, and withdraw** *(CLI/HTTP)*
  Action: with C1-1's offer live, POST a second offer on S1; then POST withdraw as Amira; then POST a fresh offer on S1.
  Assert: second offer refused 4xx with a reason in the body, and still exactly one non-terminal row for S1; after withdraw the row is `state='withdrawn'` and S1 is absent from Ben's board; the fresh offer succeeds, proving the partial unique index constrains open requests only and does not permanently burn the shift.

- **C1-4 · every state change is recorded (FR-009)** *(CLI/SQL)*
  Assert: `swap_events` rows for create→`offered` and `offered`→`withdrawn` and `withdrawn`→… as applicable, each with non-null `actor_id` equal to Amira's id and an `at` timestamp inside the run window; no state in `swap_requests` reachable without a matching event row.

- **C1-5 · SC-003, visible within 5 seconds** *(CLI/HTTP, timed)*
  Action: capture `t0` at the offer POST response; immediately GET the board as Ben; capture `t1`.
  Assert: S1 present in that first board response and `t1 - t0 < 5.0`; the measured value is recorded in the evidence whether it passes or fails, so a 4.9 s pass is visible as a near-miss rather than a green tick.

- **C1-6 · FLOW-001 in a browser** *(GUI → human checkpoint)*
  Action: at `http://localhost:8500`, Amira's shift page → *Offer swap* → confirm → swap board.
  Assert: a human confirms the offer renders on the board as *offered by me* and that the confirm step is present. Classified GUI and routed to a checkpoint rather than auto-approved, because Playwright is named in the README but is not in `requirements.txt` or CI, so there is nothing to execute this with. I would **flag** that gap alongside the card: either Playwright gets added to the dependency set and CI, or every screen-level case in this feature is permanently a human checkpoint. Branch: if it is added, C1-6/C2-8 get rewritten as executable browser cases; **default while unruled:** they stay as checkpoints.

## Phase 3 — Author C2's cases (US-002, US-003; FR-004, FR-005, FR-006, FR-007, FR-009; SC-001)

**Write:** replaces `**TEST:** TBD` at line 26.

- **C2-1 · take moves to awaiting approval and hides the offer (FR-005)** *(CLI/HTTP)*
  Action: as Ben, take S1. Assert: `state='awaiting_approval'`, `taken_by=Ben`; S1 gone from Dana's board and from Ben's board; S1's shift page shows *awaiting approval*; event row `offered`→`awaiting_approval` with `actor_id=Ben`.

- **C2-2 · the 48-hour rule, both sides of the line (FR-004)** *(CLI/HTTP)*
  Setup A: Ben seeded with 44 published hours in 2026-09-14…20; S1 is 8 h → 52.
  Assert A: take refused 4xx, response body states the reason and names the 48-hour limit, request still `state='offered'`, S1 still on Dana's board, `taken_by` still null.
  Setup B: Ben re-seeded to 40 published hours → exactly 48.
  Assert B: take **succeeds**. Spec says *over* 48; the boundary is written as a case so the build cannot quietly implement `>=`.

- **C2-3 · overlap blocks the take** *(CLI/HTTP)*
  Action: as Farid (06:00–14:00 that Tuesday), open S1's board entry and POST the take.
  Assert: no *Take this shift* control on the page, POST refused 4xx, request unchanged.

- **C2-4 · manager approval updates the rota (FR-006, `[MODIFY]` rota query)** *(CLI/HTTP + SQL)*
  Action: as Priti, approve the Ben take from C2-1.
  Assert: request `state='completed'`; the rota view for week 2026-09-14…20 lists **Ben** on Tuesday 07:00–15:00 and no longer lists Amira on it; the underlying assignment reflects the change (this is the point of the brownfield `[MODIFY]` — a rota query that assumed assignments are frozen at publication would still show Amira, and this assert catches exactly that); event row `awaiting_approval`→`completed` with `actor_id=Priti`.

- **C2-5 · decline returns the offer with its reason (FR-006)** *(CLI/HTTP)*
  Action: fresh offer+take, then Priti declines with reason "Ben is not till-trained".
  Assert: `state='offered'`, `decline_reason` stored verbatim; S1 reappears on Ben's and Dana's boards; the reason text is visible to the two staff; event row with `actor_id=Priti`.

- **C2-6 · a stale request expires (FR-007)** *(CLI/SQL)*
  Setup: shift **S3** starting 2026-09-11 18:00 (under 24 h from pinned now), with a request in `awaiting_approval` created 12 h ago.
  Action: invoke the expiry sweep.
  Assert: `state='expired'`; `shifts.staff_id` for S3 is still Amira; event row `awaiting_approval`→`expired` recorded with a system actor.
  **Flag:** neither the spec nor `design/data-model.md` names *how* expiry is triggered — no scheduler, no command, no endpoint exists. My case has to invoke something, so it will specify an explicitly invocable sweep (`python -m shiftbook.cli expire-swaps`) and the card will carry a note that this case is imposing a new surface the design seat has not ruled on. Branch: if the design seat picks a different trigger (background task, request-time lazy check), I rewrite the Action leg to match and keep every assert identical, since the observable outcome is what matters. **Default while unruled:** the CLI sweep, because it is the only form that is deterministically executable inside a verification run.

- **C2-7 · SC-001 is measurable** *(CLI/SQL)*
  Assert: a SQL query over `swap_events` returns, for the C2-4 request, the elapsed time between the `awaiting_approval` transition and the terminal transition; the same query over the C2-5 declined request returns a value; the query returns rows for all terminal requests in the window.
  **Flag on the card:** SC-001 itself — 90 % within 12 hours across the first month — is a live post-release metric and **cannot** be verified by any cycle. What a cycle can prove is that the history carries the data the metric is computed from, and that is what this case asserts. I will state that plainly on the card rather than let a green C2 imply SC-001 is met.

- **C2-8 · FLOW-002 in a browser** *(GUI → human checkpoint)*: board → *Take this shift* → manager approvals screen → *Approve* → rota shows Ben. Same Playwright reasoning as C1-6.

## Phase 4 — Author C3's cases, and stop on the quota instruction (FR-008)

**Write:** replaces `**TEST:** TBD` at line 40.

The design seat's note tells me to write C3 entirely against `FakeTextline`. I will not write it *entirely* that way, and I will say so on the card rather than quietly comply or quietly defy.

My reasoning, stated as evidence: `FakeTextline.send` appends to a list and returns `fake-N`. Cases built only on it prove the message **text and recipient set** are right. They cannot fail if the real `Textline` client is never wired into the approval path, if `TEXTLINE_FROM` is unset, if `send` is called with the wrong argument order, or if `raise_for_status` blows up on the real endpoint. C3 is the *only* card carrying FR-008. If C3 is fake-only, nothing in this feature ever demonstrates that a text leaves the building.

So the block gets two layers:

- **C3-1…C3-4 · the message matrix, on `FakeTextline`** *(CLI, runs in CI, zero quota)*
  Injected via dependency override in the app; asserts read `fake.sent`.
  - *taken*: exactly two entries — Amira's phone and Priti's (manager) phone; each body names the shift date/time and Ben as taker.
  - *approved*: exactly two — Amira and Ben; body states the outcome. Priti is **not** messaged (spec says both staff).
  - *declined*: exactly two — Amira and Ben; each body contains the decline reason verbatim, "Ben is not till-trained".
  - *expired*: exactly two — Amira and Ben.
  Each case also asserts the total count, so a duplicate send or a stray extra recipient fails rather than passing on a substring match.

- **C3-5 · one real delivery through the real adapter** *(CLI, local only, 2 of the 10 daily messages)*
  Setup: `.envrc.local` loaded from the team vault; both staff phones set to numbers in `TEXTLINE_TEST_NUMBERS`.
  Action: run the approve path with the real `Textline` client.
  Assert: `send` returns two non-empty message ids; `status(id)` for each returns one of queued/sent/delivered (not `failed`); the adapter's `textline: sent <id> to <number>` lines appear in captured console output.
  Marked explicitly: **not runnable in CI** — the workflow sets only `DATABASE_URL`, so this case is a local, credentialed, once-per-day run recorded in the cycle's evidence; budget noted as 2 messages, leaving 8 for reruns and manual poking.

**Stop / checkpoint here.** What I would put to the human: *the design seat scoped C3 as fake-only for quota reasons; I am adding one real-adapter delivery case at a cost of 2 of 10 daily sandbox messages, because otherwise FR-008's wiring is never demonstrated anywhere in the feature. Approve, or rule it out?*
- If ruled **keep**: block ships as above, and I additionally raise that CI needs `TEXTLINE_*` secrets or C3-5 stays a manual step forever.
- If ruled **drop C3-5**: I remove it and write an explicit line on the card recording that FR-008 is verified for content only and that no cycle demonstrates real delivery — an accepted, named gap, not an invisible one. I would also recommend it be carried to final validation as a manual smoke.
- **Default while unruled:** C3-5 stays in, marked local-only.

## Phase 5 — SC-002, and the traceability self-audit

- **SC-002 ("staff find the swap flow intuitive")**: I will not manufacture an executable case for it — there is no assertion that can pass or fail on evidence. Under the existing `## SC-002` heading I would add a short QA note: this is a subjective criterion, verifiable only by a moderated walkthrough with two or three staff after C1–C3 land, producing an observation record rather than a pass/fail. Placing it on a card is the design seat's call, so I recommend and flag; I do not slice.
- **Self-audit pass** over what I just wrote: build a small traceability table (temporary, in my head / in the report, not shipped into the card) mapping every story scenario, FR-001…FR-009, SC-001, SC-003 to at least one named case, and confirming no case asserts something outside its card's scope — e.g. C1 must not assert on notifications (C3 owns FR-008), and C2 must not re-litigate board visibility (C1 owns FR-002). Anything unmapped comes back as a gap report to the design seat.

## Phase 6 — Delegation

- **Reading**: I did it myself. The workspace is fourteen small files, several of the decisive findings are *absences* (no `pages` module, no Playwright, no `TEXTLINE_*` in CI, a `seed-demo` that only prints), and absence is exactly what a cheap read is worst at reporting reliably. No Explore spawn.
- **Authoring**: not delegable. These cases are the judgment.
- **One delegation I would make, after the blocks are drafted** (and would note as unavailable in this plan-only run): a disposable general-purpose worker on **sonnet**, briefed to execute *only* the environment preamble — `docker compose up -d postgres`, wait for health, `make migrate`, `make seed-demo`, `psql "$DATABASE_URL" -c "select count(*) from staff; select count(*) from shifts;"`, `ruff check src tests`, `pytest -q` — and return verbatim stdout/stderr, exit codes and timings. Fenced: run those commands and nothing else, no fixes, no interpretation, no assert evaluation.
  On return I would read the counts against my prediction: **I expect both counts to be 0**, confirming `seed-demo` inserts nothing and that my per-case SQL Setup legs are necessary rather than belt-and-braces. If the counts are non-zero, my Phase 1 warning is wrong and I rewrite the preamble to lean on the seed command. Either way I would re-run the `select count(*)` myself before believing it, and disclose the delegation in the report.

## Phase 7 — What I would report

A short verification-authoring report to the lead:

1. **Delivered**: `design/cycles.md` updated — a `## Verification environment` preamble plus three filled `**TEST:**` blocks (C1: six cases, C2: eight, C3: five), each in Setup/Action/Assert form, each classified CLI or GUI, each naming the evidence to capture.
2. **Coverage**: every US-001/002/003 scenario and FR-001…FR-009 mapped to a named case; SC-003 has a timed case; SC-001 has a measurability case only.
3. **Flags raised, none of them mine to settle**:
   - `shiftbook.pages` — the shift page C1 is meant to `[EXTEND]` is not in this workspace.
   - `make seed-demo` is a stub; Setup legs carry their own inserts until it is real.
   - FR-007 has no expiry trigger anywhere in the design; my case imposes a CLI sweep pending a ruling.
   - Playwright is documented but not installed or in CI; all screen-level cases are human checkpoints until that changes.
   - CI sets no Textline credentials, so no real-delivery case can run there.
   - SC-001 is a post-release metric, not cycle-verifiable; SC-002 is subjective and needs a moderated walkthrough, not a case.
4. **The one open decision**: the C3 real-adapter case versus the design seat's fake-only instruction, with both branches written out and my default stated.
5. **Note for later**: once these cycles pass, the cases are the natural seed for the feature's `gates.md` regression set — particularly the 48-hour boundary, the recipient-count asserts, and the rota-query assert in C2-4, which are the three most likely to rot silently.