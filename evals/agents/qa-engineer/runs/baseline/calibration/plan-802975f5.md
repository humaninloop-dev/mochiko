FILES-READ: `README.md`, `spec.md`, `design/cycles.md`, `design/data-model.md`, `docker-compose.yml`, `Makefile`, `.github/workflows/ci.yml`, `.envrc.example`, `migrations/0001_init.sql`, `src/shiftbook/app.py`, `src/shiftbook/models.py`, `src/shiftbook/cli.py`, `src/shiftbook/sms/textline.py`, `tests/fakes.py`, `tests/test_health.py`

---

# Action plan — authoring the verification cases for C1–C3

## Phase 0 · Ground the cases in what actually exists (reading, done above)

What I confirmed and will hold the cases to:

- **Real infrastructure is available and named**: PostgreSQL 16 on port 5435 via `docker compose up -d postgres`, schema by `make migrate`, app by `uvicorn shiftbook.app:app --port 8500`. CI runs `ruff check src tests` then `make migrate` then `pytest -q`. Every case I write will run against that Postgres, not an in-memory stand-in — the one deliberate exception is the SMS transport in C3, discussed in Phase 2.
- **Named actors already exist in the seed**: `make seed-demo` gives Larkhill with Amira, Ben, Dana (barista), Chloe, Emil (kitchen), Priti (manager) and a published rota 2026-09-14…2026-09-20. These are exactly the names the spec's three "Independent test" lines use, so I will anchor each card's cases on those lines rather than invent a cast.
- **Only `staff` and `shifts` exist in the schema**; `swap_requests` and `swap_events` are not in `migrations/0001_init.sql`. C1's cases therefore have to assert the new tables' behaviour, including the one-open-request index, not assume it.
- **`FakeTextline` in `tests/fakes.py`** records `(to, body)` pairs and returns a fake id — enough to assert recipient sets and message text, and nothing more.
- **The real `Textline` adapter** reads `TEXTLINE_API_KEY` / `TEXTLINE_FROM` from the environment and has a `status()` call; `.envrc.example` registers exactly three sandbox handsets for six seeded staff.

**Things I found that I will flag before writing a line of case text** (details and branches in Phase 2): `src/shiftbook/pages.py` does not exist even though `app.py` says the shift page is mounted from it and C1 marks that page as the extend target; `Shift.starts_at`/`ends_at` are timezone-naive columns while the weekly-hours rule is specified in the site's timezone; the seeded rota is at fixed dates that will rot; there is no callable entry point for expiry; and two rule boundaries are stated ambiguously.

**Delegation decision for this phase:** none. The workspace is fourteen files and I have read all of them. The one read I considered handing off — confirming no route module or existing swap code hides anywhere — is a question where *absence* is what drives my case design, so I did the enumeration myself with a full glob rather than trust a summarised answer.

---

## Phase 1 · Build the coverage matrix before authoring

I would first write out, as a working note (not a deliverable), a grid of every obligation in `spec.md` against the card that has to demonstrate it, so that no case is invented for something already covered and nothing is silently orphaned:

| Obligation | Card that cites it | My case allocation |
|---|---|---|
| US-001 all 3 scenarios, FR-001, FR-003 | C1 | C1 |
| FR-002 role leg | C1 | C1 (board visibility) |
| FR-002 overlap leg | C1 *and* C2's card text | **split** — see Phase 2, stop 4 |
| FR-009 | C1 and C2 | C1 (create/withdraw), C2 (take/approve/decline/expire) |
| SC-003 (5 s visibility) | C1 | C1, as a measured elapsed time |
| US-002, FR-004, FR-005 | C2 | C2 |
| US-003, FR-006, FR-007 | C2 | C2 |
| SC-001 (90 % in 12 h, first month) | C2 | **not a cycle assert** — see Phase 2, stop 5 |
| FR-008 | C3 | C3 |
| SC-002 (intuitive) | unplaced | stays unplaced — Phase 7 |

The matrix is the thing I re-check in Phase 8. Its purpose is that when the build asks "what does this cycle have to demonstrate", the answer is a closed set, and when final validation asks "what did we never check", the answer is a short, named list rather than a shrug.

---

## Phase 2 · Resolve the authoring hazards — five stops, with branches

These are decisions I will not paper over by writing a plausible-looking case. Each is a point where I would put the question to the design seat and the lead, state what I would confirm, and proceed on the stated default if no ruling comes back.

**Stop 1 — the extend target for C1 does not exist in this workspace.**
`app.py` comments that the rota, shift page and manager pages are "mounted from `shiftbook.pages` (existing)", and C1 marks `[EXTEND] the shift page`. There is no `pages.py`, no route definitions anywhere but `/health`. *What I would confirm:* whether this workspace is a trimmed slice with the page module withheld, or the extend target genuinely is not there. *If it's a trimmed slice:* I write the case actions against the real route paths once I can read them. *If it genuinely isn't there:* C1 is not an extension of an existing page, it is greenfield, and the design seat may want to revisit the brownfield label — theirs to rule, not mine. *Default I proceed under:* I author every case action against user-visible behaviour and HTTP status/content rather than hardcoded URL literals I cannot verify, and I leave one explicit placeholder note in the C1 block naming the shift-page route as unverified at authoring time. This costs the build nothing and stops me from shipping cases that reference paths I invented.

**Stop 2 — there is nothing to call to make a request expire.**
US-003's third scenario and FR-007 need a request that has sat for 12 hours to expire once the shift falls inside 24 hours. A case cannot wait 12 hours, and it cannot assert a sweep that has no trigger. *What I would confirm:* that C2's build will expose an invokable expiry entry point — a management command or an internal function the tests can call — rather than only a background timer. *If yes:* my C2 expiry case calls it directly after setting the shift start to just under 24 hours out. *If the build wants a timer only:* the case becomes non-executable and I would say so plainly and ask for the entry point as a build requirement, because otherwise FR-007 ships unverified. *Default I proceed under:* I write the expiry case assuming a callable trigger and state that dependency inside the case's Setup, so the build reads it before starting rather than after.

**Stop 3 — C3's instruction to test against the fake, versus what FR-008 actually promises.**
The design seat's note is well-founded: ten sandbox messages a day, and the team is hitting it. Asserting message bodies and recipient sets against `FakeTextline` is the right shape for the bulk of C3, and I will write it that way. But I will be honest about what it proves: it proves the trigger points and the template text. It proves nothing about the real adapter — that `Textline.send` builds the request Textline accepts, that credentials load, that a delivery id comes back. If C3 is entirely fake-backed, FR-008 closes on inference. *What I would confirm:* whether C3 may spend **one** of its ten daily messages on a single real-adapter smoke case, sending to one number from `TEXTLINE_TEST_NUMBERS`, asserting a non-empty message id and a `status()` in queued/sent/delivered. *If approved:* it goes in as a separately-labelled case so it can be skipped when quota is exhausted without invalidating the other four. *If refused:* the four fake-backed cases stand and I record in the card that FR-008's real transport is unverified at cycle close and must be picked up at final validation. *Default I proceed under:* include the single real smoke case, clearly marked as skippable, and note the quota cost of one message.

**Stop 4 — FR-002's overlap clause is cited on C1 but its user-visible scenario lives in US-002 on C2.**
FR-002 covers both role scoping and overlap exclusion. C1 cites FR-002 and carries US-001, which has no overlap scenario; C2's prose says "the 48-hour and overlap rules enforced". *What I would confirm:* which card owns the overlap leg. *My default:* split it by observable surface — C1 asserts an offered shift is **not listed on the board** for a colleague who already works an overlapping shift; C2 asserts **Take this shift is unavailable** on the shift page in the same situation. Both are real, both are cheap, and a duplicated rule checked from two surfaces is not waste. Whether the slicing should have kept them together is the design seat's call and I will say so rather than quietly rewrite the split.

**Stop 5 — SC-001 is a field metric, not a cycle assert.**
"90 % of requests reach completed or declined within 12 hours, measured over the first month" cannot be demonstrated by a cycle that has existed for a day. *Default:* I do not fabricate a percentage assert. Instead C2 gets a case asserting the *measurability*: that the event trail records the take timestamp and the completed/declined timestamp with actors, so the metric is computable from the request history exactly as SC-001 says it will be measured. I state in the card that the 90 % threshold itself is a post-launch measurement, not a cycle gate.

**Two ambiguities I would raise with product (Dan) alongside the stops**, because they change what a case asserts and I refuse to guess:

- FR-004 says refuse a take that puts the taker **over** 48 hours. Landing on exactly 48.0 should therefore be allowed. I will write both a refuse case at 50 and a boundary case at exactly 48 that expects success, and flag that the boundary reading is mine unless product says otherwise.
- FR-001 says **more than** 24 hours away, so exactly 24 hours is not offerable. Same treatment: a boundary case, with the reading flagged.

**One implementation risk I would raise with the build, not resolve myself:** `Shift.starts_at`/`ends_at` are naive `DateTime` columns, while the data model says weekly hours are computed Monday–Sunday **in the site's timezone**. In September, Larkhill is on British Summer Time, an hour off UTC. A naive-UTC implementation will misplace shifts near the week boundary and near the 24-hour cutoff. I will pin the site timezone explicitly in the setup of the week-boundary and cutoff cases so that this failure surfaces as a failing assert rather than a rounding mystery in production.

---

## Phase 3 · Fix the shared harness contract that every case inherits

Before writing individual cases I would settle the common Setup preamble, so the three blocks stay short and every case is reproducible from a cold machine:

- `docker compose up -d postgres`, wait for the healthcheck, `make migrate`, `make seed-demo`, app on 8500.
- Every case begins from a freshly migrated and seeded database. No case depends on residue from a previous case.
- **Times are constructed relative to the moment the case runs, never taken from the seed's fixed 2026-09-14 dates.** The seed rota is dated next week as of today, 2026-09-10, and will silently stop being "more than 24 hours away" the moment someone runs these in October. Each Setup states the shift's start as an offset from now — Amira's anchor shift at now + 5 days, 07:00–15:00, barista, published — and the sub-24-hour cases construct a shift at now + 20 hours explicitly.
- Assertions land on three surfaces and each case says which it uses: the HTTP response of the server-rendered page, the database rows, and — where the requirement is about what a person can see and click — a Playwright check in a real browser.
- Each case names its actor by seeded person, since the spec's independent tests already do.

---

## Phase 4 · Author C1's cases

I would replace `**TEST:** TBD` under C1 in `design/cycles.md` with a Setup/Action/Assert block containing these cases. Sketched content, not final wording:

1. **Offer an eligible shift.** Setup: Amira's barista shift at now + 5 days, published. Action: as Amira, open the shift page, choose *Offer swap*, confirm. Assert: a `swap_requests` row exists for that shift with `offered_by` = Amira and state `offered`; the swap board shows the shift labelled as offered by her. *Expected to show:* the request row and the board entry.
2. **Role scoping on the board.** Same offer. Assert: Ben (barista) and Dana (barista) see the entry on the board; Chloe (kitchen) does not — asserted as absence from Chloe's board, not merely a hidden button. This is the spec's own independent test for US-001.
3. **Overlap exclusion from the board** (per stop 4). Setup: additionally give Dana a published shift overlapping Amira's. Assert: Ben still sees the offer, Dana does not.
4. **One open request per shift.** Action: attempt a second offer on the same shift — once through the interface, once by direct insert. Assert: the interface refuses and the database rejects the second row on the partial unique index. *Expected to show:* a constraint violation on the direct insert. If the build implemented the rule only in application code, this case fails, which is the point.
5. **Inside 24 hours.** Setup: Amira's shift at now + 20 hours. Assert: *Offer swap* is absent on the shift page and the page carries the "talk to the manager" guidance. Plus the boundary variant at exactly 24 hours, expecting the offer to be unavailable, carrying my flagged reading.
6. **Withdraw, and no second request.** As Amira with an open request: *Withdraw* is offered, *Offer swap* is not; after withdrawing, the request is `withdrawn` and the board no longer lists it, and a fresh offer is then permitted.
7. **The audit trail.** After creating and withdrawing: `swap_events` rows exist for both transitions with from-state, to-state, Amira as actor, and timestamps.
8. **Visibility within five seconds (SC-003).** Action: record the wall-clock moment the offer confirms, then poll a colleague's board. Assert: the entry is present within 5 seconds, with the measured elapsed time captured as evidence rather than a bare pass. I would note in the case that this is measured on a local stack and is a floor, not a production latency claim.

---

## Phase 5 · Author C2's cases

Replacing C2's `**TEST:** TBD`:

1. **Take an offered shift.** Setup: C1's offer standing, Ben under 40 hours that week. Action: as Ben, *Take this shift*. Assert: state is `awaiting_approval`, `taken_by` = Ben; the offer is gone from Dana's board; it appears on Priti's approvals screen. This is the spec's independent test for US-002, minus the texts, which C3 owns.
2. **Weekly hours refusal.** Setup: Ben rostered to 42 published hours in the Monday–Sunday week containing the shift, with the site timezone pinned. Action: *Take this shift* on an 8-hour shift. Assert: refused, the request stays `offered`, and the reason shown names the 48-hour limit. *Expected to show:* refusal with a visible reason, not a silent no-op.
3. **The 48-hour boundary.** Ben at exactly 40 hours taking the 8-hour shift lands on 48.0. Assert: allowed. Flagged as my reading of "over 48".
4. **Overlap makes Take unavailable.** Dana, holding an overlapping shift, opens the shift page: *Take this shift* is not offered.
5. **Approve.** Action: as Priti, approve. Assert: state `completed`; the rota query for that week returns Ben on that Tuesday 07:00 shift and no longer returns Amira on it; and — this is the modify-the-rota-query exposure the card names — a second week of unrelated rota rows is byte-identical before and after, so the change to accommodate post-publication reassignment has not disturbed existing rota output.
6. **Decline with a reason.** Assert: the offer returns to the board for eligible colleagues, `decline_reason` is stored and displayed, `taken_by` is cleared, state is `offered` again.
7. **Expiry.** Setup: a request in `awaiting_approval` whose shift start is moved to now + 23 hours. Action: invoke the expiry trigger (dependent on stop 2). Assert: state `expired`, the shift's `staff_id` is still Amira, and the offer is not on anyone's board.
8. **Complete event trail and SC-001 measurability.** Assert: for a request that went offered → awaiting_approval → completed, every transition has a row with the correct actor (Ben for the take, Priti for the approval) and a timestamp, and the take-to-outcome interval is computable from those rows. Expiry rows record the system as actor.

---

## Phase 6 · Author C3's cases

Replacing C3's `**TEST:** TBD`. Setup for all of these: C1 and C2 shipped, the notifier wired to `FakeTextline`, its recorded list cleared before each action. Each case asserts the recipient set **exactly** — over-asserting recipients would encode a promise FR-008 does not make.

1. **On take:** exactly two messages — to Amira's phone and to Priti's. Dana receives nothing. Both bodies name the shift date and time and identify Ben as taker.
2. **On approval:** exactly two messages, to Amira and Ben. Priti is not promised one by FR-008; I assert her absence and flag it, because an ops lead may well want it and that is a spec question, not something a case should quietly assume.
3. **On decline:** two messages, to Amira and Ben, each containing the manager's stated reason verbatim.
4. **On expiry:** two messages, to Amira and Ben, stating the shift stays with Amira.
5. **Real transport smoke** (conditional on stop 3, marked skippable, budget one message of ten): send one notification through the real `Textline` adapter to a single number from `TEXTLINE_TEST_NUMBERS`. Assert a non-empty message id is returned and `status()` reports queued, sent or delivered. This is the only case in the feature that touches the vendor, and the card will say so.

I would also record in the card that with cases 1–4 fake-backed, C3 demonstrates *what would be sent and when*, not *that Textline delivered it* — so nobody reads a green C3 as proof the SMS path works end to end.

---

## Phase 7 · SC-002's disposition

"Staff find the swap flow intuitive" has no observable pass or fail and I will not invent one. I would leave the design seat's note in place and add, under it, that this is a judgement a person has to make — a walkthrough of the offer → take → approve flow with an actual staff member or with Ines, brought to a human at final validation as a decision rather than a test result. If I were pushed to make it automatable I would refuse; a case that asserts "the page loads" and calls it intuitiveness is worse than an honest gap, because it converts an open question into a false green.

---

## Phase 8 · Self-audit, classification, write, report

- **Re-walk the matrix from Phase 1** against the authored blocks: every user story scenario, every FR-001 through FR-009, and SC-003 traces to at least one named case; SC-001 and SC-002 are explicitly recorded as not cycle-demonstrable with the reason.
- **Classify each case** as automatable-and-auto-approvable (the HTTP, database and Playwright asserts — all of C1 and C2, and C3's fake-backed cases), or needing a human at the point of running (the real-transport smoke, because it spends vendor quota and touches a live handset; and SC-002's walkthrough). The classification goes in the card so the build and the later verification run inherit it rather than re-deriving it.
- **Check every case is self-sufficient**: cold-start setup, an action a person or a script can actually perform, and an assert that fails loudly if the behaviour is absent. Any case whose assert I cannot picture producing evidence for gets rewritten or cut.
- **File written:** `design/cycles.md` only — the three `**TEST:** TBD` lines replaced with full blocks, plus the note under SC-002. No other file is touched; in particular I write no test code, since authoring cases is not building them, and I add nothing to `migrations/` or `tests/`.

**What I would report at the end:**

1. The three authored blocks, with case counts and the requirement each case closes.
2. The coverage matrix, including the two obligations deliberately left unclosed (SC-001 as a post-launch metric, SC-002 as a human judgement) so neither can be mistaken for an oversight later.
3. The five stops and the two product ambiguities, each with the default I proceeded under and what changes if the ruling differs — the missing page module, the absent expiry trigger, the fake-versus-real SMS trade and its one-message cost, the split of the overlap rule across C1 and C2, the 48-hour and 24-hour boundary readings, and the naive-timestamp-versus-site-timezone risk.
4. The build-facing dependency this authoring created: C2 must expose a callable expiry trigger, or FR-007 cannot be demonstrated.
5. That no case was executed. Everything here is the specification of what the build must demonstrate; the evidence comes later, from running it against that Postgres container, and until then no card is green.