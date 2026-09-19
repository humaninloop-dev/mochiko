FILES-READ: `spec.md`, `design/cycles.md`, `design/data-model.md`, `README.md`, `Makefile`, `docker-compose.yml`, `.envrc.example`, `requirements.txt`, `.github/workflows/ci.yml`, `migrations/0001_init.sql`, `src/shiftbook/__init__.py`, `src/shiftbook/app.py`, `src/shiftbook/cli.py`, `src/shiftbook/models.py`, `src/shiftbook/sms/__init__.py`, `src/shiftbook/sms/textline.py`, `tests/fakes.py`, `tests/test_health.py`

---

# Plan — author the verification blocks for C1, C2, C3

The deliverable is text in one file: replacing `**TEST:** TBD` at `design/cycles.md:16`, `:26`, and `:40`. No source, test, or migration files get written — this card is the "what must this cycle demonstrate" pass that precedes the build.

## Phase 1 — Coverage matrix before writing any prose

**Read:** `spec.md` (FR/US/SC lists), `design/cycles.md` (what each card claims), `design/data-model.md` (states and the partial unique index).

Build a scratch table mapping every FR-001…FR-009, every Given/When/Then in US-001…US-003, and SC-001/SC-003 to the card that claims it, then to the case that will demonstrate it. Purpose is to catch two failure modes: a requirement claimed by a card but with no case behind it, and a scenario in the spec claimed by no card at all.

What this pass already surfaces from the read:

- **C1 claims "US-001 (all three scenarios)" but its card sentence only mentions offering, duplicate refusal, and the 24-hour bar.** The third scenario is *withdraw*. So C1 needs a withdraw case, and `withdrawn` is in the data model's state list. I write it into C1 rather than inventing a fourth card.
- **FR-002 has two clauses** — role match *and* no overlapping shift — and it is listed on C1 (board visibility) while US-002's overlap scenario is on C2 (the take control). Both are real and different: C1 checks the board filter hides it, C2 checks the take is refused. I split them that way and say so in the blocks so the build doesn't think one is a duplicate of the other.
- **FR-001's "their own published shifts"** is unclaimed by any card sentence. It belongs in C1 (it's an offer-time rule). Add a case.
- **SCR-001's "newest first"** is unclaimed. Add a small C1 case.
- **SC-002 is on no card** and, as written ("staff find the swap flow intuitive"), no automated case can demonstrate it. Handled in Phase 6.

## Phase 2 — Pin the ambiguities, state defaults, name the stop points

I resolve these before drafting so the cases read as decisions, not hedges. Each one I would raise in the hand-back note; none of them blocks the writing, so I proceed under the stated default rather than waiting.

1. **Who triggers expiry (FR-007).** There is no worker, cron, or scheduled job anywhere — not in `docker-compose.yml`, not in `.github/workflows/ci.yml`, not in `cli.py`. Default: write the case against an expiry routine the test calls directly with a controlled clock, and add a line saying the build still has to schedule it, because a routine nobody runs passes its test and fails in production. **Stop point:** if Dan/Ines say expiry should instead be lazy (evaluated whenever the request is read), the case changes to "reading a stale request returns it as expired and persists that", and the "second run is a no-op" case becomes "two reads don't double-record the event". I'd rewrite the one case; nothing else in C2 moves.
2. **The 24-hour boundary.** FR-001 says "more than 24 hours away", so a shift starting at exactly now+24h is *not* offerable. Default: write it that way and mark it in the block as the assumption, since it's the kind of thing that gets argued about at review.
3. **The 48-hour boundary.** FR-004 says "over 48 hours", so a take landing on exactly 48.0 is allowed. Same treatment. Week is Mon–Sun in site timezone per the data model, so a case pins that the prior Sunday's shift doesn't count.
4. **Test level.** `README.md` claims Playwright, but Playwright is not in `requirements.txt` and CI runs only `ruff check src tests` and `pytest -q`. Default: phrase every case as an observation available through the HTTP layer (`TestClient`, as `tests/test_health.py` does) plus the database, so every case is runnable by the CI that actually exists. I flag the README/requirements mismatch rather than writing cases CI cannot run. The user-visible wording (the "talk to the manager" line, the decline reason) is checked in the rendered response body, which is server-rendered Jinja, so that holds without a browser.
5. **Actor on an expired transition.** `SwapEvent.actor_id` has no obvious value when a timer expires a request. Default: a system actor, stated as an assumption.
6. **SMS failure handling.** The spec says a text MUST be sent but is silent on what happens when Textline fails. Default: the state change stands and the failure is surfaced, not swallowed; written as a case with the assumption marked. **Stop point:** if the ruling is "a failed send rolls the approval back", that case inverts and the C2 approve case gains a note; I'd change those two lines only.

## Phase 3 — Write the C1 block

**Write:** `design/cycles.md:16`, replacing `**TEST:** TBD`.

Cases, all on the seed data `cli.py` describes (Larkhill; Amira/Ben/Dana barista, Chloe/Emil kitchen, Priti manager; rota 2026-09-14…09-20 published), with the clock fixed so Tuesday 2026-09-15 07:00–15:00 sits well past 24h out:

- **C1-V1 — offer reaches the right board, and only it.** Amira offers her Tuesday 07:00–15:00 barista shift. Her shift page then shows it as offered by her. The board as Ben (barista, free that morning) lists it with a *Take this shift* control. The board as Chloe (kitchen) does not list it. The board as Dana, rostered Tue 06:00–14:00, does not list it — the overlap half of FR-002.
- **C1-V2 — board order.** With two offers made minutes apart, SCR-001 lists the newer first.
- **C1-V3 — the 24-hour bar, at the edge.** A shift starting now+23h: the shift page has no *Offer swap* control and does show the line telling the person to talk to the manager; posting the offer directly is refused and creates no row. At exactly now+24h it is still refused; at now+24h+1min it succeeds. (Boundary reading noted as an assumption.)
- **C1-V4 — one open request per shift.** Amira offering the same shift again is refused with a message, and there is still exactly one request row. Separately, inserting a second `offered` row for that shift straight into the database fails on the partial unique index, so the rule survives a code path that forgets to check.
- **C1-V5 — withdraw.** While offered, Amira's shift page offers *Withdraw* and not *Offer swap*. Withdrawing sets the request to `withdrawn` and drops it off Ben's board. Amira can then offer the shift again, producing a new request.
- **C1-V6 — the audit trail (FR-009).** After offer-then-withdraw, `swap_events` holds two rows: to `offered` and `offered → withdrawn`, both with Amira as actor and both timestamped.
- **C1-V7 — SC-003 freshness.** The board request issued immediately after the offer response already contains the shift: no queue, cache, or background job between the two. The test also bounds elapsed wall time from offer response to board render at under 5 seconds — a loose bound whose job is to fail if someone later moves offers onto an async worker.
- **C1-V8 — offering what isn't yours to offer.** A staff member cannot offer another person's shift, and cannot offer an unpublished one; both refused, no request created (FR-001).

The block closes with: run `pytest -q` and `ruff check src tests` (the CI commands), all green, C1 done when V1–V8 pass.

## Phase 4 — Write the C2 block

**Write:** `design/cycles.md:26`.

- **C2-V1 — take moves and hides.** Ben takes Amira's offer: the request is `awaiting_approval` with `taken_by` = Ben; the board as Dana no longer shows it; the manager's approvals page lists it for Priti.
- **C2-V2 — the 48-hour rule, at the edge.** Dana holds 44 published hours Mon–Sun; the offered shift is 8h, so her take is refused and the response states the weekly-hours reason. The request stays `offered` and records no state change. With 40h held — landing exactly on 48 — the take succeeds. A shift on the preceding Sunday does not count toward the week.
- **C2-V3 — overlap at take time.** For Dana (rostered Tue 06:00–14:00) the shift page shows no *Take this shift*; posting it directly is refused and nothing changes state.
- **C2-V4 — approval updates the rota.** Priti approves: the request is `completed`, and the rota for that week — read through the rota view, not just the shift row, since this card is the one that touches the published-rota query — shows Ben on Tuesday 07:00 and no longer Amira.
- **C2-V5 — decline returns the offer.** Priti declines with a reason. The request is back to `offered`, `taken_by` is cleared, the reason is stored and shown to both staff, the offer is visible again to Ben and Dana, and it can be taken again.
- **C2-V6 — expiry.** A request awaiting approval, clock advanced so the shift is 23h out, expiry routine runs: request `expired`, the rota still shows Amira on Tuesday, the offer is not on the board. Running the routine a second time changes nothing. In the same run, a request whose shift is 30h out is untouched.
- **C2-V7 — only the manager decides, and only on live requests.** A non-manager approving or declining is refused. Approving a request that is `offered`, `withdrawn`, or `expired` is refused.
- **C2-V8 — the audit trail across all four endings.** Offer → taken → approved leaves three events with the right from/to states and actors Amira, Ben, Priti. The decline and expiry paths each record their own transition, expiry under a system actor.
- **C2-V9 — SC-001 is measurable.** The history query behind SC-001 returns hours-from-taken-to-decided for a seeded set of finished requests. This demonstrates the data exists to measure the 90 % target; it does not demonstrate the target, which needs a month of real use.

Plus a standing note on the block: every C2 case binds `FakeTextline`, so the CI run cannot touch the sandbox. C2 asserts state and rota only; message content is C3's.

## Phase 5 — Write the C3 block

**Write:** `design/cycles.md:40`. Every case runs against `tests/fakes.py:FakeTextline`, per the design seat's note.

- **C3-V1 — taken.** Ben takes: exactly two recorded messages, to Amira's and Priti's numbers from `staff.phone`. None to Ben, none to Dana. Bodies identify the shift's day and time and name who took it.
- **C3-V2 — approved.** Exactly two, to Amira and Ben, stating the outcome and the shift. Not to the manager.
- **C3-V3 — declined.** Exactly two, to Amira and Ben, each carrying the manager's reason as typed.
- **C3-V4 — expired.** Exactly two, to Amira and Ben, saying the request expired and the shift stays with Amira.
- **C3-V5 — silence where FR-008 promises none.** Offering and withdrawing record nothing on the fake.
- **C3-V6 — a failed send does not undo the swap.** The adapter raises for one recipient; the approval is still `completed`, the other recipient's message is still recorded, and the failure is surfaced rather than swallowed. (Marked as the assumption from Phase 2, item 6.)
- **C3-V7 — the quota stays intact.** The whole C3 suite runs with the fake bound by a fixture and the real `Textline` class never constructed; the suite passes with no `TEXTLINE_API_KEY` in the environment, which is the state CI is in.
- **C3-V8 — one SMS segment.** Each body fits a single message (≤160 GSM-7 characters) with a long café/staff name substituted. Marked as an assumption — the spec sets no length rule, and paying for two segments per notification is a product call.

Also on the block, and **a stop point I would raise before anyone acts on it:** one live end-to-end send to a sandbox-registered handset, done once by a person before release, spending 2 of the 10 daily messages, recorded on the card. This is not a CI case and I would not schedule or run it. If the team declines the spend, C3 ships on the fake alone and I'd note on the card that the real adapter's wire format is unverified by this feature's cycles.

## Phase 6 — The SC-002 stub

**Write:** the `## SC-002` section at `design/cycles.md:42–44` gains a line, not a `TEST:` block — it is not a card and I would not invent one.

The line records that SC-002 as written cannot be demonstrated by any cycle: "intuitive" names no observation. Default action is to leave it visible and unverified, with a suggested measurable replacement for the product seat to accept or reject — for example, a set number of staff completing an offer-and-take unaided in a moderated session, with a task-success threshold. **Stop point:** this is Dan and Ines's call, not mine. If they accept a measurable version, it becomes a criterion in `spec.md` and then either a fourth card or an explicit manual check; if they drop it, the line changes to say so. Either ruling touches only this section.

## Phase 7 — Consistency pass

Re-read the three blocks together against the Phase 1 matrix and check: every FR named on a card has at least one case; no case appears on two cards; every case names an observation someone could disagree about (a state, a row, a rendered string, a count of recorded messages) rather than "works correctly"; the cases only reference things that exist — `FakeTextline`, `staff.phone`, `swap_requests`/`swap_events` as the data model defines them, the CI commands as `ci.yml` actually lists them.

I would not run `pytest` or `ruff`: the change is markdown, the cases describe tests that do not exist yet, and `make migrate` needs the Postgres container. Saying the suite is green here would mean nothing about this work.

## Delegation

None. This is one file, roughly a hundred lines of prose, and its whole value is that one person held the spec, the data model, and the existing code in view at once while writing it. Splitting it across workers would produce exactly the overlap and gap problems Phase 1 exists to catch.

## What I would report at the end

- The three blocks are written; `design/cycles.md` is the only file changed.
- The coverage map: which case demonstrates each of FR-001…FR-009, each US scenario, SC-001 and SC-003 — and that SC-002 is demonstrated by nothing and why.
- The gaps found while writing, each of which is a build or product item, not a verification item: no scheduler exists for FR-007 expiry; the README promises Playwright that `requirements.txt` and CI don't provide; `tests/conftest.py` doesn't exist so the fake-adapter fixture C1–C3 all depend on has to be built with C1; C1 needs a migration adding `swap_requests`, `swap_events`, and the partial unique index on top of `0001_init.sql`.
- The four assumptions I wrote in and would want ruled on: exactly-24h is not offerable, exactly-48h is allowed, a failed SMS doesn't undo the state change, notifications should fit one segment.
- The two things I stopped short of: spending sandbox quota on a live send, and rewriting SC-002 in the spec.