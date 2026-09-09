FILES-READ: `spec.md`, `design/cycles.md`, `design/data-model.md`, `README.md`, `Makefile`, `.github/workflows/ci.yml`, `.envrc.example`, `requirements.txt`, `migrations/0001_init.sql`, `src/shiftbook/app.py`, `src/shiftbook/models.py`, `src/shiftbook/cli.py`, `src/shiftbook/sms/textline.py`, `tests/fakes.py`, `tests/test_health.py`

# Plan — author the verification blocks for C1, C2, C3

**Deliverable:** one file changed — `design/cycles.md`. The three `**TEST:** TBD` lines (lines 16, 26, 40) become concrete case lists, plus a short amendment to the trailing `## SC-002` section. No source or test files are written; the build seat writes the code and the tests from these cases. No agents, subagents, or workers are dispatched at any point — this is a single-file authoring task and delegation would cost more than it saves.

---

## Phase 1 — Reconcile spec ↔ cards ↔ code, and build the coverage matrix

Already read: everything listed above. Nothing further to read; the workspace is small and I have the whole surface.

I would write out (as working notes, not committed) a matrix of every testable obligation against the card that must demonstrate it:

| Obligation | Source | Card |
|---|---|---|
| Offer own published shift >24h out | FR-001, US-001 #1 | C1 |
| *Offer swap* absent inside 24h, message shown | FR-001, US-001 #2 | C1 |
| Board shows offer to same-role staff only | FR-002 (role leg) | C1 |
| Board hides offer from staff with an overlapping shift | FR-002 (overlap leg) | C1 or C2 — see stop 3.1 |
| One open request per shift; withdraw available | FR-003, US-001 #3 | C1 |
| Take → `awaiting_approval`, offer leaves the board | FR-005, US-002 #1 | C2 |
| Take refused over 48h/week, reason shown | FR-004, US-002 #2 | C2 |
| *Take this shift* absent when it overlaps | US-002 #3 | C2 |
| Approve → rota shows taker, state `completed` | FR-006, US-003 #1 | C2 |
| Decline with reason → back to `offered` | FR-006, US-003 #2 | C2 |
| Expire once shift <24h away | FR-007, US-003 #3 | C2 |
| SwapEvent per transition, actor + time | FR-009 | C1 (offer/withdraw), C2 (rest) |
| SMS on taken / approved / declined / expired | FR-008 | C3 |
| Offer visible within 5s | SC-003 | C1 |
| 90% resolved in 12h over month 1 | SC-001 | C2 — measurability only, see stop 3.2 |
| "Intuitive" | SC-002 | unplaceable, see Phase 6 |

Three things fall out of this reading that shape the cases, and I would carry them into the cards rather than silently resolving them:

- **Time is the dominant variable.** Every card has at least one case that turns on "more/less than 24 hours away", and C2 has expiry. Tests cannot sleep and cannot depend on the wall clock relative to the seeded rota (`2026-09-14…2026-09-20`, already in the past relative to any future run). Every case I write will be phrased against an injectable clock ("with now set to …"), which is a constraint on the build, not just the tests. I would state that once in a preamble line above C1 so it isn't repeated fifteen times.
- **CI runs `ruff check src tests`, `make migrate`, `pytest -q` — and nothing else.** The README claims Playwright for browser checks but `requirements.txt` has no Playwright and the workflow never invokes it. So no case may be written as a browser case, or it would be unrunnable on merge. All cases are HTTP-level via `TestClient` plus direct assertions on `swap_requests` / `swap_events`.
- **The board's role filter and the take-time overlap rule are the same FR (FR-002) split across two cards.** C1 claims FR-002 whole; C2 owns the overlap-at-take scenario from US-002. Left as is, either the overlap leg gets tested twice or falls between the cards.

## Phase 2 — Fix the shape of a case before writing any

Decide the format once and apply it uniformly, so the build seat reads three consistent blocks:

```
**TEST:**

1. `name-in-kebab-case` — Given …, when …, then <observable> and <observable>. [FR-00x, US-00y #z]
```

Rules I hold myself to while drafting:

- Every case states an **observable outcome** a reader could check by looking at a response, a page, or a table row — never "the logic is correct".
- Every case carries its requirement ids in brackets, so the matrix in Phase 1 can be re-derived from the card alone.
- Named people from `cli.py seed-demo` (Amira, Ben, Dana — barista; Chloe, Emil — kitchen; Priti — manager) so cases read like the spec's own independent tests.
- Each block closes with a **Not covered here** line pointing at the card that does cover it, so a gap is visible on the card rather than only in my head.
- No case names a source file or function that doesn't exist yet as though it does; the build is free to structure the code.

Suggested test file per card, offered as a suggestion in the card text rather than a mandate: `tests/test_swap_offer.py`, `tests/test_swap_take_approve.py`, `tests/test_swap_notifications.py`, with the clock and seed fixtures in `tests/conftest.py`.

## Phase 3 — Stops for a human ruling

I would raise these three together, in one pass, before writing — they are cheap to answer and each changes card text. None of them blocks the work: I state a default for each and write the cards that way, marking the assumption inline so a reversal is a one-line edit.

**3.1 — Where does the overlap filter live?** FR-002 says an offered shift is shown only to staff who hold the role *and* don't already work an overlapping shift; C1 claims FR-002, C2 owns "Take this shift is not available when it overlaps". Confirming with the design seat: does C1's board filter exclude overlapping staff, or does C1 filter by role only and C2 add overlap on both board and button?
- *Ruling "C1 does both"*: C1 gains a board case with Dana rostered against Amira's slot; C2's overlap case narrows to the button/POST rejection only.
- *Ruling "C2 adds overlap"*: C1's cases assert role filtering only and I annotate C1 as covering FR-002 partly; C2 gains a board-visibility case alongside its button case.
- **Default I write to:** the second. C1 is the walking skeleton and a colleague's competing roster is a second concept; overlap belongs with the other take-time rules. I annotate C1's FR-002 claim as "role leg only" and note the split in both blocks.

**3.2 — Is SC-001 in-cycle at all?** "90% within 12 hours, measured over the first month" cannot be demonstrated by a cycle; the month hasn't happened. C2 lists it.
- *Ruling "keep it"*: C2 gets a measurability case only — the history carries the timestamps the metric needs.
- *Ruling "move it"*: it goes to a post-launch measurement note beside SC-002.
- **Default:** keep it, as a measurability case, and say plainly in the card that the target itself is measured post-launch and is not a build gate.

**3.3 — Does an untaken `offered` request expire inside 24h?** FR-007 and the state model only cover `awaiting_approval`. An offer made 30 hours out that nobody takes will sit on the board while the shift becomes un-offerable — visible to a colleague who then cannot act on it.
- *Ruling "yes, offers expire too"*: C2 gains a fourth expiry case and the state model needs `offered → expired`.
- *Ruling "no / later"*: recorded as an open question on the card, no case.
- **Default:** no case, but I add one line under C2 flagging it as unspecified so it is decided rather than discovered in build. I would not invent the behaviour in a verification block — that is writing spec through the back door.

**3.4 — Any live Textline call?** I would confirm, but my default needs no ruling: zero. All C3 cases run against `FakeTextline`. If someone wants a one-off sandbox smoke, it is a manual pre-release step outside CI with a stated budget of two messages against the numbers in `.envrc.example`, and I would say so on the card rather than let a CI job silently consume a ten-message daily quota shared by the team.

## Phase 4 — Write C1's block

Replacing line 16 of `design/cycles.md`. Cases I would author:

1. `offer-appears-on-board` — Amira offers her Tue 07:00–15:00 barista shift (published, 40h out with the clock pinned); the request is `offered`, and Ben's board shows the shift with a *Take this shift* control. [FR-001, FR-002, US-001 #1]
2. `board-excludes-other-roles` — the same offer is absent from Chloe's board (kitchen). [FR-002, US-001 #1]
3. `board-excludes-the-offerer` — Amira's own board does not list her own offer. [FR-002] *(not in the spec's words; the spec's "colleagues eligible" implies it. Marked in the card as an inference so it can be struck if the design seat disagrees.)*
4. `offer-refused-inside-24h` — with now set 6 hours before start, the shift page shows no *Offer swap* and shows the talk-to-your-manager text; a direct POST is refused and no `swap_requests` row is created. [FR-001, US-001 #2]
5. `unpublished-shift-cannot-be-offered` — an unpublished shift offers no *Offer swap*. [FR-001]
6. `cannot-offer-someone-elses-shift` — Ben cannot offer Amira's shift. [FR-001]
7. `second-open-request-refused` — with Amira's request `offered`, a second offer on that shift is refused and exactly one row remains; the partial unique index rejects it at the database level too. [FR-003, US-001 #3]
8. `withdraw-then-reoffer` — Amira withdraws (state `withdrawn`, offer gone from Ben's board), and a fresh offer on the same shift then succeeds. [FR-003, US-001 #3] *(the re-offer half is an assumption — `withdrawn` is not an open state, so the index permits it; noted inline.)*
9. `offer-and-withdraw-are-recorded` — a `swap_events` row per transition with `from_state`, `to_state`, `actor_id` = Amira, and `at`. [FR-009]
10. `offer-visible-on-next-board-load` — the board request issued immediately after the offer returns it; the board handler's own timing is asserted well inside the 5-second budget. [SC-003] *(phrased as end-to-end freshness, not a benchmark: the point is no queue or cache delays visibility. Noted as such.)*

Closing line: **Not covered here** — taking, approval, expiry (C2); any SMS (C3); the overlap leg of FR-002 (C2, per 3.1).

## Phase 5 — Write C2's block

Replacing line 26.

1. `take-moves-to-awaiting-approval` — Ben takes Amira's offer; state `awaiting_approval`, `taken_by` = Ben, and the offer is gone from Dana's board. [FR-005, US-002 #1]
2. `taken-offer-cannot-be-taken-again` — Dana's take on the same request is refused; `taken_by` is still Ben. [FR-005]
3. `take-refused-over-48-hours` — Dana already holds 44 published hours that week; the 8-hour shift would make 52; the take is refused, the response names the 48-hour limit, and the request stays `offered` and on the board. [FR-004, US-002 #2]
4. `take-allowed-at-exactly-48-hours` — 40h + 8h = 48 is permitted. [FR-004] *(boundary; "over 48" reads as exclusive. Assumption noted.)*
5. `weekly-hours-counted-monday-to-sunday` — hours on the Sunday before and the Monday after the shift's week do not count toward the 48. [FR-004, per the data-model rule]
6. `overlapping-shift-blocks-take` — Dana is rostered 06:00–14:00 that Tuesday; her shift page offers no *Take this shift*, a direct POST is refused, and the offer is absent from her board. [FR-002 overlap leg, US-002 #3]
7. `approval-updates-the-rota` — Priti approves; the rota query for that week returns Ben on Tue 07:00 and no longer Amira, `shifts.staff_id` is Ben, state `completed`. [FR-006, US-003 #1] *(this is the `[MODIFY]` the card flags — the case must read the rota through the same query the rota page uses, so a stale-assignment bug fails here rather than in production.)*
8. `decline-returns-the-offer` — Priti declines with "Ben is not till-trained"; state back to `offered`, `taken_by` cleared, `decline_reason` stored, the shift still Amira's, and the offer visible to Dana again and takeable. [FR-006, US-003 #2]
9. `stale-request-expires` — a request `awaiting_approval` with the clock moved to 12 hours before start: state `expired`, `shifts.staff_id` still Amira, and neither *Approve* nor *Decline* acts on it afterwards. [FR-007, US-003 #3]
10. `expired-request-cannot-be-approved` — a manager POST to approve an `expired` request is refused and leaves the rota unchanged. [FR-007]
11. `only-a-manager-can-decide` — Ben's and Amira's approve/decline attempts are refused. [FR-006] *(inference from "As a manager"; noted.)*
12. `every-transition-is-recorded` — taken, approved, declined, expired each leave a `swap_events` row with the acting person (and for expiry, the system as actor, since no human acts — flagged as needing a convention). [FR-009]
13. `history-supports-the-resolution-metric` — for a taken-then-approved and a taken-then-declined request, the recorded events give the taken timestamp and the resolution timestamp, so time-to-resolution is computable from the history alone. [SC-001, measurability only — the 90%/12h target is measured post-launch and is not a build gate.]

Closing lines: **Not covered here** — SMS (C3). **Open question** — whether an untaken `offered` request expires inside 24h (per 3.3).

## Phase 6 — Write C3's block, and deal with SC-002

Replacing line 40. Every case runs against `FakeTextline` from `tests/fakes.py`, asserting on `.sent` — recipients and body — with the real `Textline` client never constructed.

1. `taken-texts-offerer-and-manager` — Ben takes; exactly two messages, to Amira's and Priti's numbers, each naming the shift date and time; nothing to Ben. [FR-008, US-002 #1]
2. `approval-texts-both-staff` — exactly two messages, to Amira and Ben, both saying the swap was approved; **none to Priti**, who took the action. [FR-008, US-003 #1] *(FR-008 says "both staff members" for approval/decline/expiry and adds the manager only for taken; asserting the absence is the point of the case.)*
3. `decline-texts-both-staff-with-the-reason` — the decline reason text appears verbatim in both bodies. [FR-008, US-003 #2]
4. `expiry-texts-both-staff` — on expiry, Amira and Ben are told, and the message says the shift stays with Amira. [FR-008, US-003 #3]
5. `no-text-on-offer-or-withdraw` — offering and withdrawing send nothing. [FR-008 by omission]
6. `send-failure-does-not-undo-the-decision` — with the fake raising on send, an approved swap stays `completed` and the rota stays updated. [FR-008 vs FR-006] *(marked as an inference: the spec does not say what happens when Textline is down, and "the rota silently reverts because a text failed" is the worse of the two readings. Flagged for the design seat.)*
7. `tests-never-call-textline` — no case constructs the real client; the adapter is injected. [quota guard]

Plus a card note: the sandbox quota means CI must never send. If a live sandbox check is wanted, it is a manual pre-release step, budget two messages, not a CI job.

**SC-002 ("staff find the swap flow intuitive").** I would not invent a verification case for it. It states no observable outcome, no threshold, and no instrument; anything I wrote would be a test that passes by construction and gives false coverage. I would amend the trailing `## SC-002` section to say so explicitly — that it is not verifiable by any of C1–C3 as written, and that making it verifiable needs a criterion product can name (for example, a stated proportion of staff completing an offer without help in a walkthrough, or a support-contact rate over month 1). That is a product decision, not mine to make, so I leave it as a named gap on the page rather than closing it quietly. Whichever the design seat picks, SC-003's 5-second case in C1 and SC-001's measurability case in C2 stand as they are.

## Phase 7 — Self-check before reporting

- Re-derive the Phase 1 matrix from the edited `design/cycles.md` alone: every FR-001…FR-009 and every one of the nine Given/When/Then scenarios appears in at least one card, and each appears in exactly one unless deliberately split (FR-002, FR-009). Any orphan means a missing case, not a matrix footnote.
- Confirm no case needs a browser, a real SMS, a sleep, or a tool absent from `requirements.txt` — all of them run under `pytest -q` with the Postgres service the workflow already starts.
- Confirm every case names something a person could look at and disagree about.
- Confirm the rest of each card (the checkbox line, the story/FR list, brownfield notes, the design seat's Textline note) is untouched — I am filling three placeholders and appending one paragraph, not rewriting the slicing.
- No verification run is possible or attempted here: there is no code behind these cases yet, and CI is not mine to trigger. I would say that plainly rather than imply the cases have been exercised.

I would not write anything to memory for this: the quota constraint, the fake adapter, and the card structure are all already recorded in `README.md` and `design/cycles.md`.

## Phase 8 — What I report back

- One file changed: `design/cycles.md` — three verification blocks written (C1: 10 cases, C2: 13, C3: 7) and the SC-002 section amended.
- The coverage matrix, showing FR-001…FR-009 and all nine scenarios placed, and naming the two deliberate splits.
- The rulings I took defaults on and would like confirmed: the overlap leg placed in C2, SC-001 kept as measurability only, no case for expiry of untaken offers, no live Textline in CI.
- The assumptions written into cases where the spec is silent, each flagged inline on the card too: 48 hours inclusive; re-offer permitted after withdrawal; offerer excluded from their own board; managers only for approve/decline; SMS failure does not roll back an approval; an actor convention needed for system-driven expiry events.
- The two gaps I am handing back rather than closing: SC-002 has no measurable criterion, and the README's claim of Playwright browser checks is not backed by `requirements.txt` or `.github/workflows/ci.yml` — so no card depends on a browser, and adding that capability is a separate decision.
- Explicitly not done: no test code, no source changes, and nothing run.