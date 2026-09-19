FILES-READ: `spec.md`, `design/cycles.md`, `design/data-model.md`, `README.md`, `Makefile`, `docker-compose.yml`, `requirements.txt`, `.envrc.example`, `.github/workflows/ci.yml`, `migrations/0001_init.sql`, `src/shiftbook/app.py`, `src/shiftbook/models.py`, `src/shiftbook/cli.py`, `src/shiftbook/sms/textline.py`, `tests/fakes.py`, `tests/test_health.py`

---

# Action plan — author the verification blocks for C1, C2, C3

The deliverable is one edited file: `design/cycles.md`, with the three `**TEST:** TBD` lines (lines 16, 26, 40) replaced by concrete case lists, plus a short disposition for the orphaned SC-002 at the foot. No source or test code gets written — the build hasn't happened, so executable tests would only be red placeholders. I name the target test file for each case so the build seat knows where each one lands.

## Phase 0 — establish the fixture world the cases will be written against

Read (already done): `src/shiftbook/cli.py` for the seed cast, `design/data-model.md` for states and rules, `migrations/0001_init.sql` and `src/shiftbook/models.py` for existing columns.

Fix a single named fixture world so all three cards' cases share vocabulary and the build seat doesn't reinvent data per card. From `seed-demo`: Larkhill café; baristas Amira, Ben, Dana; kitchen Chloe, Emil; manager Priti; rota 2026-09-14 → 2026-09-20 published.

- **Frozen clock for all cases:** `now = 2026-09-13 12:00 Europe/London` (Sunday). Chosen so the spec's canonical shift is comfortably outside the cutoff and a second shift is comfortably inside it.
- **SHIFT-A** — Amira, barista, Tue 2026-09-15 07:00–15:00, published. 43 h out → offerable. This is the spec's own independent-test shift in all three stories.
- **SHIFT-B** — Amira, barista, Mon 2026-09-14 07:00–15:00, published. 19 h out → inside the 24 h cutoff.
- **SHIFT-C** — Dana, barista, Tue 2026-09-15 06:00–14:00 — overlaps SHIFT-A. Present only in the overlap fixture variant.
- **SHIFT-D** — Amira, barista, Wed 2026-09-16, `published = false` → not offerable.
- Hours variants for the 48 h rule: `ben_light` (Ben rostered 20 h that week) and `ben_heavy` (Ben rostered 44 h that week, so +8 h = 52).

Every case names its actor explicitly, because there is no session mechanism in the repo yet.

## Phase 1 — resolve the six ambiguities, then write

These are decisions I'd surface to the design seat (Ines/the slicer) before the block is final. None of them blocks authoring — I write under the stated default and mark each with a `?` note in the card so the build seat sees the assumption rather than inheriting it silently.

1. **How a test says "signed in as Amira."** No auth exists in `app.py`. *Default:* cases say "as Amira" and the build supplies a FastAPI dependency override in a `client_as(staff)` fixture. If the seat rules for a real session cookie, the cases don't change — only the fixture does.
2. **Time zone and clock injection.** `models.py` uses naive `DateTime`, but `design/data-model.md` computes weekly hours "Monday–Sunday in the site's timezone." *Default:* site timezone is Europe/London, and the 24 h cutoff plus the weekly window read from an injectable clock, never `datetime.now()`. If the seat rules "naive UTC everywhere," the C2 weekly-hours cases need a boundary case added for a shift crossing local midnight; I'd note that as the onward branch rather than write it now.
3. **What triggers expiry (FR-007).** Nothing in the repo schedules anything. *Default:* a callable sweep (`expire_stale_swaps(now)`) that a test can invoke directly, with scheduling left to the build. Onward branch if the seat wants lazy expiry-on-read instead: the same assertions apply but the trigger in each case becomes "Priti loads the approvals page."
4. **Playwright.** `README.md` lists it; `requirements.txt` and `.github/workflows/ci.yml` do not. *Default:* every case is written to run under `pytest -q` through `TestClient`, so the whole block is green-able by the CI that actually exists. I'd flag the README/CI mismatch to the seat rather than write cases that CI cannot run.
5. **SC-003's "within 5 seconds."** A wall-clock assert on a CI runner is flaky and measures the runner, not the feature. *Default:* verify the behaviour that makes 5 s achievable — the offer is on the eligible colleague's next board load with no intervening job, cache warm, or manual refresh step — and record the observed render time as a note, not a gate. If the seat insists on a hard threshold, the branch is a generous `< 2 s` assert on the board query against a seeded 200-shift rota, marked as a smoke check.
6. **SC-002 ("staff find the swap flow intuitive").** Not machine-verifiable and deliberately unhomed by the seat. *Default:* I do not invent a card or a fake assertion for it. I record it under the existing SC-002 heading as out of automated verification, with a concrete substitute the ops lead can run: five staff complete FLOW-001 and FLOW-002 unaided on the demo seed, no assistance, task completion and stumbles recorded. That's a proposal for Dan/Ines to accept, not a check the build owes.

**Where I would stop for a human:** items 2 and 6. For 2, because a wrong ruling makes the C2 hours cases assert the wrong week. For 6, because deciding a success criterion is unverifiable is product's call, not mine. In both cases I'd deliver the finished block under the default above and mark the two spots, rather than hold the whole document.

## Phase 2 — write C1's verification block

Target: `design/cycles.md` line 16. Cases land in `tests/test_swap_offer.py` and `tests/test_swap_board.py`; schema case in `tests/test_migrations.py`.

C1 covers all three US-001 scenarios, FR-001/002/003/009 and SC-003:

- **C1.1 offer succeeds** — as Amira, offer SHIFT-A. Response 200; a `swap_requests` row exists with `shift_id = SHIFT-A`, `offered_by = Amira`, `state = offered`. (FR-001)
- **C1.2 board shows it to same-role colleagues** — as Ben, the board lists SHIFT-A with a *Take this shift* control. As Dana, likewise. (FR-002, US-001 scenario 1)
- **C1.3 board hides it from other roles** — as Chloe (kitchen), the board does not list SHIFT-A. (FR-002)
- **C1.4 board hides it from an overlapping colleague** — overlap fixture (SHIFT-C): as Dana, SHIFT-A is absent from the board. (FR-002)
- **C1.5 offerer sees her own offer as withdrawable, not takeable** — as Amira, the shift page shows *Withdraw*, no *Take this shift*. (US-001 scenario 3)
- **C1.6 inside 24 h cannot be offered** — as Amira on SHIFT-B: the shift page shows no *Offer swap*, and the page carries the "talk to your manager" message. A direct POST to the offer endpoint is refused (4xx) and creates no row — the guard must be server-side, not just a hidden button. (FR-001, US-001 scenario 2)
- **C1.7 unpublished shift cannot be offered** — as Amira on SHIFT-D, refused. (FR-001, from "published shifts")
- **C1.8 someone else's shift cannot be offered** — as Ben, POST offer on SHIFT-A refused. (FR-001 "their own")
- **C1.9 no second open request** — after C1.1, a second offer POST on SHIFT-A by Amira is refused with a stated reason, and exactly one `swap_requests` row for SHIFT-A remains. (FR-003)
- **C1.10 the constraint holds at the database, not only in the handler** — inserting a second `offered` row for SHIFT-A directly raises an integrity error; a `withdrawn` row plus a new `offered` row is accepted. This pins the partial unique index from `design/data-model.md`. (FR-003)
- **C1.11 withdraw** — as Amira, withdraw; `state = withdrawn`; SHIFT-A disappears from Ben's board; Amira may then offer it again. (US-001 scenario 3)
- **C1.12 audit trail** — after offer and withdraw, `swap_events` holds two rows: `null → offered` and `offered → withdrawn`, each with `actor_id = Amira` and a timestamp. (FR-009)
- **C1.13 visible on the next load** — the board GET immediately after the offer POST already contains SHIFT-A, with no sweep, job, or second request in between; observed render time recorded. (SC-003, per the Phase 1 default)
- **C1.14 migration** — `make migrate` applies `0002` cleanly on an empty database and is re-runnable; `swap_requests` and `swap_events` match `design/data-model.md`, including `decline_reason` nullable.

## Phase 3 — write C2's verification block

Target: line 26. Cases in `tests/test_swap_take.py`, `tests/test_swap_approval.py`, `tests/test_swap_expiry.py`, `tests/test_rota_after_swap.py`. C2 is the merged card — US-002 + US-003, FR-004/005/006/007/009, SC-001. All cases start from SHIFT-A already offered by Amira.

Take:

- **C2.1 take moves to awaiting approval** — as Ben (`ben_light`), take SHIFT-A: `state = awaiting_approval`, `taken_by = Ben`. (FR-005)
- **C2.2 offer leaves the board for everyone else** — after C2.1, as Dana the board no longer lists SHIFT-A; as Ben it appears as his pending request, not as a takeable offer. This is the spec's own independent test for US-002. (FR-005)
- **C2.3 over 48 h refused with reason** — as Ben (`ben_heavy`, 44 h + 8 h = 52), take is refused, the response names the reason and the hours, and the request stays `offered` and stays on Dana's board. (FR-004)
- **C2.4 exactly 48 h is allowed** — Ben at 40 h taking the 8 h shift succeeds; the rule is "over," not "at." A boundary the spec leaves implicit; I'd flag the wording to Dan and default to allowing it.
- **C2.5 the weekly window is the shift's week** — Ben's hours in the neighbouring week don't count toward the Tue 2026-09-15 total; Mon–Sun, site timezone. (FR-004, ties to Phase 1 item 2)
- **C2.6 overlap cannot be taken** — overlap fixture: as Dana, POST take on SHIFT-A refused, and the shift page offers her no *Take this shift*. (US-002 scenario 3)
- **C2.7 wrong role cannot be taken** — as Chloe, POST take refused. (FR-002)
- **C2.8 the offerer cannot take her own shift** — as Amira, refused.
- **C2.9 second taker loses** — with the request already `awaiting_approval`, a take by Dana is refused; `taken_by` remains Ben. (FR-003, FR-005)

Approve / decline / expire:

- **C2.10 approve updates the rota** — as Priti, approve: `state = completed`, and SHIFT-A's `staff_id` is now Ben. The rota page for the week shows Ben on Tue 07:00 and no longer shows Amira there. This is the `[MODIFY]` the card calls out — assignments change after publication, so the rota query must not assume otherwise. (FR-006, US-003 scenario 1)
- **C2.11 decline returns the offer with its reason** — as Priti, decline with "Ben isn't till-trained": `state = offered`, `decline_reason` stored, SHIFT-A is back on Dana's and Ben's boards, and the shift still belongs to Amira. (FR-006, US-003 scenario 2)
- **C2.12 decline requires a reason** — an empty reason is refused and leaves the state unchanged.
- **C2.13 only a manager decides** — as Ben and as Amira, approve and decline are both refused, and the approvals page is not reachable for them. (FR-006, SCR-003)
- **C2.14 approve is not repeatable** — a second approve on a `completed` request is refused and does not touch the rota a second time.
- **C2.15 expiry** — request taken at `2026-09-13 12:00`; run the sweep at `2026-09-14 08:00`, when SHIFT-A is 23 h away: `state = expired`, `staff_id` still Amira, and the shift is absent from every board. (FR-007, US-003 scenario 3)
- **C2.16 the sweep leaves everything else alone** — at the same clock, a request whose shift is 40 h out stays `awaiting_approval`; already `completed`, `declined`-to-`offered`, and `withdrawn` requests are untouched. (FR-007)
- **C2.17 an offered-but-untaken request inside 24 h** — the spec only names expiry for awaiting-approval requests. I'd flag this gap to Dan; default is that it expires too, so it can't be taken at the last minute, with the case marked as an assumption rather than a spec-derived requirement.
- **C2.18 full audit chain** — for the approve path, `swap_events` reads `offered → awaiting_approval` (actor Ben) then `awaiting_approval → completed` (actor Priti); for decline, `→ offered` with Priti; for expiry, `→ expired` with a system actor. Each row timestamped. Expiry has no human actor — I'd flag that `SwapEvent.actor_id` needs to be nullable or take a system sentinel, since `design/data-model.md` doesn't say. (FR-009)
- **C2.19 SC-001 is measurable from the data** — a query over `swap_events` yields, per request, the interval from `→ awaiting_approval` to `→ completed`/`→ declined`, so the 90 %-within-12 h figure is computable at the end of month one. The case asserts the query returns the right intervals on a seeded set; it does not assert the 90 % target, which is an outcome the build cannot make true. I'd note that distinction on the card.

## Phase 4 — write C3's verification block

Target: line 40. Cases in `tests/test_swap_notifications.py`, all against `FakeTextline` from `tests/fakes.py` — the design seat's note about the ten-message sandbox cap is a hard constraint, and I would refuse to write any case that sends through `src/shiftbook/sms/textline.py`, hits `api.textline.example`, or uses the numbers in `.envrc.example`. That includes "one real message to prove the adapter works." If the seat wants live-path assurance, the branch is a manually-run, opt-in check outside `pytest -q`, marked in the card as not part of CI.

`FakeTextline` records `(to, body)` tuples, so every case asserts recipients by phone and content by substring.

- **C3.0 the sender is injectable** — the app takes its SMS sender as a dependency so tests can substitute the fake. Nothing in `app.py` provides this yet, so I'd write it as the first line of C3's block: it's a build obligation the cases depend on.
- **C3.1 taken** — Ben takes SHIFT-A: exactly two messages, to Amira's number and Priti's number. Amira's names Ben and the shift's day and time; Priti's says it awaits her approval. Nothing to Dana or Chloe. (FR-008)
- **C3.2 approved** — exactly two, to Amira and Ben, both saying approved and naming the shift. (FR-008)
- **C3.3 declined** — exactly two, to Amira and Ben, both carrying Priti's reason text verbatim. (FR-008)
- **C3.4 expired** — the sweep sends exactly two, to Amira and Ben, saying the swap expired and the shift stays with Amira. (FR-008)
- **C3.5 no message on the paths that promise none** — offer, withdraw, a refused over-48 h take, and a refused overlap take each leave `sent` empty. This is what actually protects the daily quota in development.
- **C3.6 one send per transition** — a repeated approve POST adds no further messages; the send is tied to the state change, not the request.
- **C3.7 a send failure does not lose the state change** — with a fake that raises on `send`, the approval still commits and the rota still shows Ben. The spec doesn't say; I'd flag it and default to state-first, since silently rolling back an approved swap because a text failed is the worse outcome.
- **C3.8 message bodies name the shift unambiguously** — every body contains the date and start time, so a staff member with two pending swaps can tell them apart. Asserted as substring, with the exact wording left to the build.

I'd also note on the card, without acting on it, that `Textline.send` prints message IDs to stdout — worth a look when C3 is built, but out of scope for a verification block.

## Phase 5 — coverage sweep, then the single write

Before writing, walk the spec end to end and confirm every identifier has at least one case: US-001 (C1.1–C1.6, C1.11), US-002 (C2.1–C2.9), US-003 (C2.10–C2.16), FR-001 (C1.1, C1.6–C1.8), FR-002 (C1.2–C1.4, C2.7), FR-003 (C1.9, C1.10, C2.9), FR-004 (C2.3–C2.5), FR-005 (C2.1, C2.2), FR-006 (C2.10–C2.14), FR-007 (C2.15, C2.16), FR-008 (C3.1–C3.6), FR-009 (C1.12, C2.18), SC-001 (C2.19), SC-003 (C1.13), SC-002 — deliberately none, dispositioned at the foot of the file. Also confirm the three independent tests written into the spec appear verbatim in behaviour: Amira→board→Ben-not-Chloe (C1.2/C1.3), Ben takes and Dana loses it plus two texts (C2.2 + C3.1), Priti approves and the rota shows Ben (C2.10 + C3.2).

Then one edit to `design/cycles.md`: three `**TEST:**` blocks replaced with the case lists above, each case one line in the form *actor → action → what must be true*, tagged with its FR/US/SC and its test file; the assumption notes from Phase 1 inline where they bite; and the SC-002 paragraph extended with its manual-check proposal. No other file changes.

I would not run `pytest`, `ruff`, or `make migrate` — nothing executable changes, the named test files don't exist yet, and `make migrate` needs the Postgres container. The one check I'd want is a read-back of the edited file to confirm the three blocks render and no card lost its checkbox line.

## Delegation

None. This is a single-document authoring job whose whole difficulty is holding the spec, the data model, and the existing test harness in one head at once; splitting it across workers would produce three blocks that disagree about the fixture world. No subagents, no parallel workers.

## What I'd report back

- `design/cycles.md` edited; three verification blocks written; roughly 14 cases for C1, 19 for C2, 9 for C3, each naming its actor, its assertion, its requirement tag, and its target test file.
- Every FR, every user-story scenario, and SC-001/SC-003 have at least one case; SC-002 has none by design, with a manual usability check proposed for Dan and Ines to accept or drop.
- Seven things the cases assume because the spec or data model is silent, each marked on the card: the actor-selection mechanism, Europe/London plus an injectable clock, the expiry trigger, `at 48 h exactly` being allowed, whether an untaken offer inside 24 h expires, `SwapEvent.actor_id` for system-driven expiry, and state-before-SMS ordering on send failure. The timezone one and the SC-002 one are the two I'd want ruled on before the build starts.
- Two mismatches found in the repo, flagged not fixed: `README.md` lists Playwright but `requirements.txt` and CI have neither, so all cases are written for `pytest -q`; and C3's cases require an injectable SMS sender that `app.py` does not yet provide.
- Stated plainly: no tests were written or run, and nothing was verified against running code — these are the cases the build must satisfy, not evidence that it does.