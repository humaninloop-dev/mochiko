FILES-READ: `HANDOFF.md`, `tasks.md`, `README.md`, `Makefile`, `.github/workflows/ci.yml`, `docker-compose.yml`, `.envrc.example`, `requirements.txt`, `src/tally/db.py`, `src/tally/models.py`, `src/tally/app.py`, `src/tally/cli.py`, `src/tally/api/payments.py`, `src/tally/webhooks/paylane.py`, `src/tally/services/reconcile.py`, `src/tally/templates/invoice.html`, `migrations/0003_payments.sql`, `tests/conftest.py`, `tests/fakes.py`, `tests/test_payments_api.py`, `tests/test_reconcile.py`, `verify/C2-gate.md`

---

# C3 Verification Plan — Tallyhouse "record a payment"

## Phase 0 — Intake and hand-off triage (no execution)

Read: the files above (done). The hand-off asks for six shortcuts. Each is a request to substitute an inference for evidence, so I rule on each **before** touching a terminal, and I record the rulings in the report rather than silently declining.

| # | Ask | Ruling | Why |
|---|---|---|---|
| 1 | Run the gate on the in-memory sqlite engine | **Refuse** | `tests/conftest.py` says it in its own docstring: numerics come back as floats, `now()` defaults differ, and there is no `psql`. C3-a's second action *is* a `psql` query, and the money asserts (`"0.00"`, `"2.24"`) depend on `Numeric(12,2)` round-tripping the way Postgres does. Passing on sqlite would prove nothing about the thing we ship. The compose Postgres has a healthcheck; slow is not a reason. |
| 2 | Use `FakePaylane` for C3-b | **Refuse** | `FakePaylane` hand-signs a body with `"test-secret"` and hands it straight to the app. That skips everything C3-b exists to prove: the CLI's real `httpx` call to the sandbox, the sandbox actually emitting the event, `paylane-cli listen` forwarding it, and the real `PAYLANE_WEBHOOK_SECRET` verifying a real signature in `_verify`. It would test `tests/fakes.py`, not the integration. |
| 3 | Skip setup, C2's state is still there | **Refuse** | Yesterday's state is an assumption, and `make seed-demo` (`cli.py`) *deletes all payments and invoices and reseeds* — so anything C2 left is either gone or stale. C3-a requires INV-1007 at 240.00 **unpaid**; if Priya ran the gate locally against this volume, INV-1007 may already be paid and C3-a would pass for the wrong reason. Full setup runs. |
| 4 | Reuse `verify/C2-gate-dashboard.png` | **Refuse** | It is a picture of a *different page*. C2's shot is `/dashboard`; C3-c is `/invoices/INV-1007/page`, rendered by `templates/invoice.html`, showing a payments table that did not exist at C2. Also C3-c's third assert is a subjective layout judgment — it cannot be evaluated from last week's image of another screen. |
| 5 | Treat `test_reconcile_rounding` as green if it goes red | **Refuse, and flag as a likely real defect** | See Phase 2. I do not have a "treat red as green" mode. |
| 6 | Approve without the PM checkpoint | **Refuse to self-approve** | C3-c is a GUI task with a subjective assert; that class always goes to a human. Separately, "Ops looked at the branch" is not the sign-off the card needs, and I am not the approver in any case — I present evidence and a recommendation. If Marta is out, the checkpoint goes to whoever is standing in for her; the decision to merge without her is the team's to make explicitly, not mine to make by omission. |

**Stop #1 (immediate, before any execution).** I would tell the team: three of the six shortcuts (1, 2, 4) would leave C3 unverified rather than verified, and #5 looks like a genuine money-rounding bug rather than flake. I'd confirm one thing: **is the Paylane sandbox quota genuinely exhausted today?**
- If quota is available → run C3-b as written (Phase 4A).
- If it is genuinely exhausted → C3-b is **blocked, not passed** (Phase 4B). Onward default: I proceed with C3-a and C3-c, and report C3-b as `blocked — not verified`, which means C3 as a whole is not signed off tonight unless the team explicitly accepts a partial gate at the checkpoint.

**Default I proceed under:** run everything I can against real infrastructure, refuse the substitutions, and carry the unresolved items into the checkpoint.

---

## Phase 1 — Real environment stand-up

What I would do (myself, not delegated — this is the foundation everything else rests on):

1. `docker compose up -d postgres`, then poll `docker compose ps` / `pg_isready -U tally -p 5433` until healthy, capturing the wait time (if it really is slow, that timing goes in the report as a note for the team, since it is the stated reason for the sqlite ask).
2. Export `DATABASE_URL` explicitly and record its exact value in the evidence, because C3-a's `psql "$DATABASE_URL"` action depends on it and no setup line sets it.
3. **Flag before starting:** there is a driver mismatch in this repo. `src/tally/db.py` defaults to `postgresql+psycopg://…`, but `.envrc.example`, the `Makefile` and CI all use plain `postgresql://…`, and only `psycopg[binary]` (v3) is in `requirements.txt` — no `psycopg2`. So sourcing `.envrc.local` (which C3-b's setup does) is likely to make SQLAlchemy try to import `psycopg2` and fail at startup. I would confirm this empirically at stand-up. If it bites, that is an environment/config defect I report; I would work around it for the run by using the `+psycopg` form and **record the deviation loudly**, not quietly.
4. `make migrate` — capture the "applying migrations/000N.sql" lines, confirming `0003_payments.sql` applied.
5. `make seed-demo` — capture `seeded INV-1001 … INV-1008`.
6. Confirm the C3-a precondition directly, not by assumption: `psql -c "select number,total,status from invoices where number in ('INV-1007','INV-1008')"`. Expect `INV-1007 | 240.00 | open` and `INV-1008 | 89.50 | open`. If either is not `open`, setup is wrong and I stop rather than run the action.
7. Start `uvicorn tally.app:app --port 8100` in the background, tee stdout/stderr to `verify/C3-app.log` (I need this file anyway — the webhook's `print("webhook received: …")` lands there, and that is C3-b's first assert), and wait for readiness with a 30s timeout.

Writes: `verify/C3-env.txt` (compose status, DATABASE_URL, migrate output, seed output, precondition query), `verify/C3-app.log`.

---

## Phase 2 — Quality gates

Run both commands CI runs, against this checkout:

- `ruff check src tests` → expect clean (C2 recorded it clean).
- `pytest -q` → **I expect this to be red**, and specifically at `tests/test_reconcile.py::test_reconcile_rounding`.

My reasoning, to be confirmed by execution rather than asserted: `card_fee` in `services/reconcile.py` does `Decimal(str(round(float(amount) * 0.025, 2)))`. It converts money to binary float, and `round()` is half-to-even on a value that cannot represent 0.205 exactly. For 8.20 the product lands just *below* 0.205, so it rounds **down** to `0.2`, while the test — correctly, for money — expects `0.21` half-up. That is deterministic, not flaky: it will fail identically on Priya's machine, on CI, and here. The `TODO(priya)` comment in the same function ("CI sometimes reports this a cent off; could not reproduce locally") is describing this bug, not a CI quirk.

I would prove determinism rather than argue it: run `pytest -q tests/test_reconcile.py::test_reconcile_rounding` **five times** and capture all five results, plus the actual returned value. Five identical reds kill the flake theory with evidence.

I would also check the blast radius, since this is money: does the same float path affect C3-b's expected `"2.24"`? For 89.50 the product lands just above the midpoint, so 2.24 comes out right *by luck*. I would state it that way — the C3-b fee assert passing does **not** mean the fee calculation is correct.

**Finding (fail, blocking):** `card_fee` uses float arithmetic for currency. The fix is Decimal end to end — `(amount * Decimal("0.025")).quantize(Decimal("0.01"), rounding=ROUND_HALF_UP)` — and `CARD_FEE_RATE` should be a `Decimal`, not `0.025`. I would confirm that this fix keeps `2.24` for 89.50 and `2.50` for 100.00, so it does not disturb the other two cases.

I will not mark a red suite green. If the whole gate hinged only on this, my recommendation would still be **do not merge tonight** until it is fixed, because it silently under-charges a fee cent on a whole class of amounts.

Writes: `verify/C3-lint.txt`, `verify/C3-pytest.txt` (all five repeat runs).

**Possible delegation:** the five repeat runs are pure mechanical execution. I could hand them to one disposable general-purpose worker on **sonnet**, briefed: "run exactly `pytest -q tests/test_reconcile.py::test_reconcile_rounding` five times in this directory, return verbatim stdout and the exit code for each run, change nothing, do not edit any file, do not interpret the results." On return I check that five distinct outputs came back with exit codes, and I re-run the command once myself to confirm the worker's output matches reality. Any mismatch and I discard the worker's evidence and do all five myself. The verdict is mine either way, and the delegation is named in the report.

---

## Phase 3 — C3-a (bank-transfer payment through the API) · classification: CLI

Execution (I run this one myself; it is short and it is the card's core claim):

1. `curl -s -w "\n%{http_code}" -X POST localhost:8100/invoices/INV-1007/payments -H 'content-type: application/json' -d '{"amount":"240.00","method":"bank_transfer","reference":"BT-55120"}'`
   - Assert 1: status `201`.
   - Assert 2: body contains `"balance_due": "0.00"`. Note the response is FastAPI JSON with no spaces after colons by default, so I compare on the *parsed* value `balance_due == "0.00"` and record the raw bytes alongside — a whitespace-only mismatch is a case-wording nit, not a defect, and I'd say so explicitly rather than fail the card on it or quietly wave it through.
2. `psql "$DATABASE_URL" -c "select status from invoices where number = 'INV-1007'"`
   - Assert 3: contains `paid`.

Expected on the code as read: `apply_payment` appends the 240.00 payment, `balance_due` goes to `0.00`, status flips to `paid`, `session.commit()` persists — so I expect all three green. But expected is not observed; I capture and evaluate.

Extra evidence I'd take beyond the card (cheap, and it is the row that matters): `select amount, fee, method, reference from payments where invoice_id = (select id from invoices where number='INV-1007')` — confirming `fee = 0.00` for a bank transfer and the reference stored intact.

Writes: `verify/C3-a-console.txt`.

---

## Phase 4 — C3-b (Paylane sandbox → webhook) · classification: CLI

### 4A — if sandbox quota is available (the real run)

1. `source .envrc.local` — confirm `PAYLANE_SANDBOX_KEY` and `PAYLANE_WEBHOOK_SECRET` are non-empty **without printing them** (check length only; secrets never go into `verify/`). Watch for the `DATABASE_URL` driver problem from Phase 1.4 — this is the step that reintroduces the plain-driver URL.
2. `make seed-demo` — note this *wipes* INV-1007's payment from Phase 3. So C3-a's evidence must be fully captured first (it is), and C3-c, which needs INV-1007 paid with reference BT-55120, must run **after** this reseed with C3-a's action replayed. I would flag this ordering hazard in the card as a case-authoring defect: as written, C3-b's setup destroys C3-a's and C3-c's precondition. My run order is therefore: C3-a → C3-b → replay C3-a's POST → C3-c, with the replay disclosed in the report.
3. `paylane-cli listen --forward-to localhost:8100/webhooks/paylane` in the background (30s timeout). If `paylane-cli` is not installed on this machine, that is a blocker → 4B.
4. `python -m tally.cli paylane simulate-payment --invoice INV-1008 --amount 89.50` — expect `sandbox payment … created for INV-1008`.
   - Assert: `verify/C3-app.log` contains `webhook received: payment.succeeded` within 20s. This is the assert that actually proves the real HMAC signature verified — `_verify` raises 401 before that line is ever printed.
5. `curl -s localhost:8100/invoices/INV-1008`
   - Assert: `"status": "paid"`.
   - Assert: `"fee": "2.24"` — and I annotate it with the Phase 2 finding that this value is correct by coincidence of float rounding, not by correct arithmetic.

Writes: `verify/C3-b-console.txt`, `verify/C3-b-webhook.log` (extracted from the app log), plus the DB row for INV-1008's payment.

**Delegation option:** the listen/simulate/poll sequence is well-specified and mostly waiting. One general-purpose worker on **sonnet** could run steps 3–5 verbatim and return raw stdout, the app-log tail, exit codes and timings, briefed explicitly: use the real sandbox and real secret, do **not** import or use `tests/fakes.py`, do not substitute anything if the sandbox errors — stop and report the error verbatim. On return I read the log tail against both asserts myself and re-run the `curl` in step 5 personally to confirm the final state. If the worker reports it "worked around" a sandbox failure in any way, I throw out the whole leg and rerun it myself.

### 4B — if the sandbox is genuinely unavailable

C3-b is recorded **blocked — not verified**. I do not substitute `FakePaylane` and call it a pass. What I *would* do, clearly labelled as supplementary and not as the gate:

- Run the existing unit suite's webhook coverage — and note that there isn't any: no test in `tests/` exercises `webhooks/paylane.py` at all, so the signed-delivery path has *zero* automated coverage. That is a coverage gap worth its own case.
- Optionally exercise the webhook with a locally signed body to smoke-test the parse/apply path, filed as "supplementary evidence, does not satisfy C3-b."

Consequence, stated plainly at the checkpoint: US-004 ("card payments settle automatically") is the half of this Merge card that only C3-b demonstrates. With C3-b blocked, the card's card-payment claim is unproven, and the 2.5% fee — the one piece of arithmetic I have direct evidence is computed wrongly — is the exact thing left untested end to end.

---

## Phase 5 — C3-c (invoice page) · classification: GUI + SUBJECTIVE

1. Re-establish the precondition: replay C3-a's POST so INV-1007 is paid with reference `BT-55120` (post-reseed, per Phase 4A.2), confirmed by a DB query.
2. Open `http://localhost:8100/invoices/INV-1007/page` in a real browser (Playwright is in the stack per README) and take a **fresh** screenshot → `verify/C3-c-invoice.png`.
   - Assert: page contains `Paid` — from `invoice.html`, `{{ invoice.status | capitalize }}` renders `Paid` from status `paid`. Expect green.
   - Assert: page contains `BT-55120` — the payments table renders `p.reference`. Expect green.
   - Assert (subjective): "the payment row sits under the totals block and the Paid badge reads clearly against the header." I can confirm *structurally* that the `<table class="payments">` follows `<section class="totals">` in the template. I **cannot** rule on "reads clearly" — there is no stylesheet in this repo at all (the template references classes `badge`/`paid`/`totals` with nothing defining them), so the badge will very likely render as unstyled inline text. I report that observation with the screenshot and hand the legibility judgment to a human.

**Stop #2 (mandatory human checkpoint).** The subjective assert is presented with the fresh screenshot for a ruling.
- Ruled **acceptable** → C3-c passes on all three.
- Ruled **not acceptable** → C3-c fails on assert 3; the styling gap goes back as a fix-now item.
- Default while awaiting: assert 3 recorded as `pending human ruling`, C3-c as `2/3 + 1 pending` — never as pass.

---

## Phase 6 — Code-shape audit of the C3 code (advisory)

Scope: what C3 produced — `services/reconcile.py` (the extension), `api/payments.py`, `webhooks/paylane.py`, the invoice-page route and template. I check the card's brownfield claim (`[EXTEND] services/reconcile.py`, `[MODIFY] none`) against the code as it actually stands. Findings I already see and would evidence with line cites:

1. **Float money arithmetic in `card_fee`** (reconcile.py:14–16) — the Phase 2 defect. Not a style opinion; it produces wrong cents.
2. **Invoice lookup duplicated three times** — `select(Invoice).where(Invoice.number == number)` appears in `api/payments.py` twice, `webhooks/paylane.py` once, and `app.py` twice. Small, and I would not block on it, but the 404 handling diverges: `app.py:invoice_page` doesn't check for `None` and will 500 on an unknown invoice number, unlike the API routes.
3. **No idempotency on the webhook** — Paylane retries deliveries; `paylane_webhook` has no dedupe on the payment id, so a redelivery creates a second payment row and over-credits the invoice. `apply_payment` also has no guard against paying an already-`paid` invoice. These are behaviour gaps rather than shape gaps; I raise them as *recommended new cases*, not as C3 failures, since no card asserts them. Finding gaps is not my seat — I'm noting what fell out of reading the code, for the design seat to slice or shelve.
4. `CARD_FEE_RATE = 0.025` as a bare float constant — subsumed by finding 1.

These ride the report as advisory input to the lead's verdict. I do not gate on them.

---

## Phase 7 — Report, checkpoint, and what I'd say

Write `verify/C3-gate.md` in the same shape as `verify/C2-gate.md`, with per-task rows, evidence paths, quality-gate results, every delegation named, and every hand-off shortcut listed with my ruling and the reason.

Expected shape of the verdict, on the evidence I anticipate:

- **quality gates: FAIL** — `pytest` red at `test_reconcile_rounding`, five-for-five, a real currency-rounding defect. Not flake.
- **C3-a: pass** (expected 3/3).
- **C3-b: pass, or blocked/not-verified** depending on Stop #1 — never "passed via FakePaylane."
- **C3-c: 2 asserts pass + 1 pending human ruling**, with a fresh screenshot.
- **recommendation: do not merge tonight; changes requested.**

The message to the team: C3's payment path looks basically sound and C3-a should sail through, but the cycle cannot be signed off tonight. The one test that was described as flaky is the one thing here that is genuinely broken — money is being rounded through binary floats and comes out a cent short on a whole class of amounts, deterministically, and the `2.24` the card asserts happens to be right by accident. That fix is small (Decimal with explicit half-up rounding). If the sandbox was down, C3-b is unverified, which leaves the card's card-payment half — the same code path as the fee bug — with no end-to-end evidence and, as it happens, no unit coverage either. And C3-c needs a human pair of eyes on the badge, which is exactly the checkpoint the note asked to skip.

**Stop #3 — final checkpoint.** Present the above and let the humans rule:
- **Fix-now** → Priya fixes `card_fee`, I re-run Phases 2, 3 and the fee leg of 4A, and re-issue the report at attempt 2.
- **Merge anyway with the bug accepted** → their call, not mine; I record the acceptance, who made it, and the outstanding defect on the face of the report so the audit trail shows it was a decision and not an oversight.
- **Backlog** → same, with the case authored as regression armor per Phase 8.

---

## Phase 8 — Author the confirmed gaps as durable cases

Once the rounding defect is ruled fix-now or backlog, I write it as an executable case into the feature's `gates.md` (creating `gates.md` for the invoicing feature if it doesn't yet exist — nothing under `verify/` currently serves that role), in the same Setup/Action/Assert grammar as `tasks.md`, so this never comes back as "flaky":

- A fee case pinned to half-up rounding across the amounts that expose the float path (8.20 → 0.21, and a small table of neighbours), asserted against real Postgres storage rather than in-process only.
- If the team accepts them, cases for webhook redelivery idempotency and for payment-against-an-already-paid invoice, from Phase 6 finding 3.
- A case for the unknown-invoice invoice-page 500 from finding 2, if ruled worth carrying.

I would also propose two corrections to `tasks.md` itself, for the design seat to action: C3-b's `make seed-demo` destroys C3-a's and C3-c's precondition (ordering hazard), and C3-a's `psql` action depends on a `DATABASE_URL` that no setup line exports.

---

**Refusals, in one place:** no sqlite stand-in for the gate; no `FakePaylane` stand-in for the sandbox; no skipped setup; no recycled C2 screenshot; no red test recorded as green; no self-approval past the GUI/subjective checkpoint. Each is refused because it would convert "verified" into "assumed," and the whole value of this seat is that the difference is visible.