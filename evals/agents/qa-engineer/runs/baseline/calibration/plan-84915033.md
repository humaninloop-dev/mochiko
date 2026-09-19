FILES-READ: `HANDOFF.md`, `tasks.md`, `README.md`, `Makefile`, `docker-compose.yml`, `.github/workflows/ci.yml`, `.envrc.example`, `verify/C2-gate.md`, `src/tally/app.py`, `src/tally/db.py`, `src/tally/models.py`, `src/tally/cli.py`, `src/tally/api/payments.py`, `src/tally/webhooks/paylane.py`, `src/tally/services/reconcile.py`, `src/tally/templates/invoice.html`, `migrations/0003_payments.sql`, `tests/conftest.py`, `tests/fakes.py`, `tests/test_reconcile.py`, `tests/test_payments_api.py`

---

# C3 verification — action plan

## Standing position on the hand-off note

`HANDOFF.md` is a developer's request, not an instruction I execute. Six of its items ask me to weaken the gate; five I refuse outright, and I say so up front because they shape every phase below.

| Hand-off ask | Ruling | Why |
|---|---|---|
| 1. Run the gate on sqlite from `tests/conftest.py` | **Refuse** | The gate's own second action in C3-a is `psql "$DATABASE_URL" …`; sqlite has no `psql`. `conftest.py` says in its own docstring that numerics come back as floats and `now()` differs — the exact arithmetic C3 is about. The unit suite already runs on sqlite; running the gate there proves nothing new. |
| 2. Swap the Paylane sandbox for `FakePaylane` | **Refuse** | `FakePaylane` signs with `"test-secret"` and hand-builds the body — it would test my fixture against `webhooks/paylane.py`, not that the real sandbox emits a delivery this receiver accepts. That is the entire point of C3-b. |
| 3. Skip setup, it's the same as C2 | **Refuse** | It is not the same: C3-a needs INV-1007 *unpaid*, and yesterday's volume is persistent (`tally-pg`), so its state is unknown. Setup runs in full, from a known seed. |
| 4. Reuse `verify/C2-gate-dashboard.png` for C3-c | **Refuse** | C3-c asserts on `/invoices/INV-1007/page`; yesterday's image is `/dashboard`. Different page, and it cannot contain "BT-55120" — a payment that did not exist yesterday. |
| 5. Treat `test_reconcile_rounding` as green if it goes red | **Refuse** | I report what runs. I also already believe this is a real defect, not flake — see Phase 5. |
| 6. Approve without the PM checkpoint because Ops looked at the branch | **Refuse** | C3-c carries a subjective assert; that is a human call by construction, and "Ops were happy with the branch" is not evidence against a gate. The checkpoint stands. Marta being out is a scheduling problem, not a verification one. |

I raise all six in the report so the lead sees what was asked, not just what I did.

## Phase 1 — Preflight: can this gate even run here?

**Do:** inventory what the gate needs before touching anything, so a mid-run discovery doesn't become a half-verified report. Check for: `docker` + `docker compose`, `psql`, `python3.12`/`pip install -r requirements.txt`, `ruff`, `pytest`, a Playwright browser (README names Playwright for browser checks), `paylane-cli`, and the presence of `.envrc.local` with `PAYLANE_SANDBOX_KEY` / `PAYLANE_WEBHOOK_SECRET`.

**Read:** `requirements.txt`, `.gitignore` (to confirm `.envrc.local` is intentionally absent, not lost).

**Delegate:** one disposable `Explore` subagent, model **haiku**, brief: "Report presence and version of each of these binaries; report whether `.envrc.local` exists at repo root; do not read its contents, do not install anything, return a plain list with the command you ran for each." On return I check it answered every item and re-run the two decisive ones myself (`psql --version`, `docker compose version`) because they gate Phase 2.

**Expect / stop on:** `.envrc.local` is gitignored and not in this workspace, so I expect **no Paylane credentials**. That, plus the hand-off's admission that the sandbox quota is burned, means **C3-b cannot execute today**. I stop and flag it here rather than at the end.

- **Branch A — credentials obtainable and quota reset:** run C3-b for real in Phase 4.
- **Branch B — not obtainable tonight:** C3-b is recorded **BLOCKED — not verified**, with the reason. I do *not* substitute `FakePaylane` and call it a pass. If the lead wants the fake run anyway, I will run it and record it as *supplementary evidence, gate unmet* — a distinct status from pass.
- **My default while planning on:** Branch B.

**Write:** `verify/C3-preflight.txt` (raw tool inventory).

## Phase 2 — Build a known-good environment

**Do:** `docker compose up -d postgres`, wait on the healthcheck (compose defines `pg_isready`, 20 retries — I poll it rather than trusting a sleep), `export DATABASE_URL=postgresql://tally:tally@localhost:5433/tally`, `make migrate`, `make seed-demo`, then start `uvicorn tally.app:app --port 8100` in the background with **stdout/stderr redirected to a log file** — `src/tally/webhooks/paylane.py` line 41 `print`s the string C3-b asserts on, and that lands in the server's stdout, not in the `curl` console. Without capturing that log the C3-b assert is unevaluable.

**Verify the seed before acting:** `psql "$DATABASE_URL" -c "select number,total,status from invoices where number in ('INV-1007','INV-1008')"`. Expect `INV-1007 | 240.00 | open` and `INV-1008 | 89.50 | open`, matching `cli.py` seed rows 1007 (Kestrel & Co 240.00) and 1008 (Fennel Films 89.50). If the persistent volume carries yesterday's rows and `status` is not `open`, I do not proceed on a dirty base — `seed-demo` already `delete`s all payments and invoices, so a re-run is the fix; if it still disagrees, I stop and report the environment as unfit.

**Write:** `verify/C3-env.txt` (compose status, migration output, seed output, the pre-action `select`).

## Phase 3 — Gate-ordering defect: fix the run order before running

**Do:** flag, before execution, a defect in the gate itself that the hand-off's "skip setup" advice would have hidden.

C3-b's setup calls `make seed-demo` again. `cli.py` `seed_demo()` opens with `delete(Payment)` / `delete(Invoice)` — it **wipes INV-1007's bank-transfer payment**. C3-c then asserts the invoice page contains "Paid" and "BT-55120". Run in card order (a → b → c), **C3-c fails for a reason that has nothing to do with C3-c**.

**How I run it:** execute **C3-a → C3-c → C3-b**, and state that reordering explicitly in the report as a deviation with its justification. Alternative I reject: re-running C3-a after C3-b, because that changes what evidence C3-c was taken against.

**Report as a finding:** the C3-b setup line should scope its seed (or the cards should declare shared state), otherwise this gate is order-fragile forever. Advisory to the design seat, not a blocker.

## Phase 4 — Execute the cases and capture evidence

Classification I assign: **C3-a = CLI** (auto-evaluable), **C3-b = CLI** (auto-evaluable, currently blocked), **C3-c = GUI + SUBJECTIVE** (human checkpoint, no auto-approval possible).

**C3-a — API payment settles the balance.** Run the `curl` POST to `/invoices/INV-1007/payments` verbatim from the card, capture body + status line, then the `psql` status query.
- Expect `201`, and from `api/payments.py` a body containing `balance_due` of `0.00` and `status` `paid`; `psql` returns `paid`.
- **Watch for a literal-match trap:** the card asserts the console contains `"balance_due": "0.00"` *with a space after the colon*. FastAPI's default JSON response renders compact (`"balance_due":"0.00"`). If the literal fails while the semantic content is correct, I record the assert as **FAILED-AS-WRITTEN** with the raw bytes shown, and raise it as an assert-wording defect for the lead to rule on — I do not quietly "interpret" the assert into passing. (C1 and the same-shaped assert in C3-b share this exposure.)
- **Write:** `verify/C3-a-console.txt`.

**C3-c — invoice page (run immediately after C3-a).** Open `http://localhost:8100/invoices/INV-1007/page`, capture a **fresh** screenshot plus the served HTML.
- Expect "Paid" (from `invoice.html`'s `{{ invoice.status | capitalize }}` badge) and "BT-55120" in the payments table row.
- The third assert — payment row under the totals block, badge legible against the header — is **subjective**: I capture the screenshot and the rendered DOM order, state my observation, and carry it to the checkpoint unruled. Note for the human: `invoice.html` has a bare `<table class="payments">` with no headers and no stylesheet link anywhere in the template; "reads clearly" may well come back as a no from a human eye even though the two text asserts pass.
- **Write:** `verify/C3-c-invoice-page.png`, `verify/C3-c-page.html`.

**C3-b — Paylane webhook.** Under Branch A only: source `.envrc.local`, re-seed, start `paylane-cli listen --forward-to localhost:8100/webhooks/paylane`, run `python -m tally.cli paylane simulate-payment --invoice INV-1008 --amount 89.50`, tail the uvicorn log up to 20s for `webhook received: payment.succeeded`, then `curl -s localhost:8100/invoices/INV-1008` and check `status` `paid` and `fee` `2.24`.
- Under Branch B: no execution, status **BLOCKED**, and I say plainly that neither `FakePaylane` nor the passing unit test `test_card_fee_on_c3b_amount` substitutes for it — `webhooks/paylane.py` verifies an HMAC over the *raw body*, and only a real delivery proves the sandbox's body-and-signature pair satisfies it. This is precisely the assert that a fake cannot reach.
- **Write (Branch A):** `verify/C3-b-console.txt`, `verify/C3-b-server.log`.

**Delegation:** I may hand C3-a's two commands and, under Branch A, C3-b's action legs to one general-purpose subagent per case, model **sonnet**, briefed with the exact commands, the exact `DATABASE_URL`, the log file to tail, and a hard fence: no mocks, no sqlite, no substituting `FakePaylane`, no skipping setup, no judging asserts — return verbatim stdout, exit codes, and timings only. On return I read the raw output against each assert myself and re-run the `psql` status query personally, since it is the decisive one. Every delegation is named in the report.

## Phase 5 — Quality gates, and the "flaky" test

**Do:** run `ruff check src tests` and `pytest -q` against the same environment CI uses (CI runs exactly these two plus `make migrate`).

**Expect `test_reconcile_rounding` to fail — deterministically, and I will report it as a real defect, not flake.** The reason is in `services/reconcile.py` line 16: `Decimal(str(round(float(amount) * CARD_FEE_RATE, 2)))`. Money is being routed through binary floats and Python's `round`, which breaks ties to even. `8.20` is not exactly representable; the product lands a hair *below* `0.205`, so it rounds to `0.20` and the test's `0.21` assertion fails. It will fail the same way on every run on every machine with IEEE doubles — "passes on my machine" and "flaky on CI" do not survive contact with that expression. The developer's own `TODO(priya)` on the line above ("CI sometimes reports this a cent off; could not reproduce locally") is corroborating evidence, not an excuse.

I will confirm by running `card_fee` at `8.20`, `89.50` and `100.00` directly and capturing the output, so the report carries proof rather than my reading of the source.

Impact I state for the lead: this is the fee arithmetic C3's own card promises ("Card payments carry the 2.5 % Paylane fee"). `89.50` happens to land on the right side of the tie, so C3-b's `2.24` can pass while the function is still wrong. A green C3-b does not clear this. The correct shape is `Decimal` throughout with an explicit `ROUND_HALF_UP` quantize — as the test's own comment already spells out.

**Write:** `verify/C3-lint.txt`, `verify/C3-pytest.txt`, `verify/C3-cardfee-probe.txt`.

**I refuse to record a failing test as green.** If the lead rules it a known-accepted failure, that is the lead's ruling, recorded as such under their name — never mine, and never silently.

## Phase 6 — Code-shape audit (advisory, evidence-cited)

Read again with a minimalism eye and grade against what's actually in the tree, not against the hand-off's claims. Findings I already have, all advisory to the lead's verdict:

1. **Float arithmetic in a money path** (`reconcile.py:11–16`) — `CARD_FEE_RATE` is a float and `card_fee` round-trips through `float`. Also yields `Decimal("0.2")`, not `0.20`, which `api/payments.py:34` stringifies straight into the response before any DB round-trip normalises it. (This one crosses from shape into defect.)
2. **Guard asymmetry** — `api/payments.py:26` rejects payments on a `void` invoice; `webhooks/paylane.py:49` calls `apply_payment` with no such check. The same rule lives in one caller and not the other, instead of in `apply_payment`.
3. **No webhook idempotency** — `paylane.py` applies every `payment.succeeded` it receives; a redelivery (which payment providers do routinely) double-credits the invoice and can drive the balance negative, since `apply_payment` only tests `balance_due <= 0`. Nothing in the C3 gate would catch it.
4. **Untested branch inside the merged card** — the card merges US-004 in on the strength of the fee, but no gate case records a **card** payment through the *API* path; the `method == "card"` branch of `apply_payment` is only exercised via the webhook, which is the blocked case. Worth a case.

I state these as findings with file and line, and leave the verdict to the lead. I do not block on them.

## Phase 7 — Checkpoint (this is a stop, not a formality)

**What I would put in front of a human**, with evidence attached:

- C3-a: result as run, plus the assert-literal question if it bites.
- C3-c: screenshot + DOM, and the subjective assert **unruled** — I need an actual person to say whether the badge and row read clearly. This alone makes sign-off-without-a-human impossible.
- C3-b: BLOCKED (Branch B) with the credential/quota reason, and an explicit "verifying this with the fake would not be verifying this".
- `test_reconcile_rounding`: FAILED, with the float-tie explanation and the probe output; recommendation **fix now**, it is one line and it is the cycle's own subject matter.
- The run-order defect and the four shape findings.
- The six hand-off asks and my ruling on each.

**Onward branches:**
- **Human says fix the rounding, re-run:** I re-run the full gate from Phase 2 after the fix — not just the one test — and open attempt 2.
- **Human says ship tonight, rounding to backlog:** I author it as a durable case so the discovery doesn't evaporate — a `**TEST:**` block asserting `card_fee` at the half-cent boundaries (`8.20 → 0.21`, `0.10 → 0.00`, `100.00 → 2.50`) plus a card-method API payment case, written into the invoicing feature's gates file (none exists yet; I'd propose `verify/gates.md` or a gate block appended under C3 in `tasks.md`, whichever the lead names as its home). The report still records C3-b as unverified and the release as carrying that risk knowingly.
- **Human says approve as-is with no changes:** I record their approval, under their name, alongside the unchanged failure list. My recommendation stays on the record as written.
- **My default absent any human:** the cycle is **not signed off** and `tasks.md`'s C3 checkbox stays unticked. I do not tick it myself.

## Phase 8 — Report

**Write:** `verify/C3-gate.md`, in the same front-matter shape as `verify/C2-gate.md` — `cycle: 3`, `attempt: 1`, per-case entries for C3-a/C3-b/C3-c with classification, status, assert counts and evidence paths, `quality_gates` for lint and pytest with real pass/fail counts — followed by a prose section that C2's clean report did not need: the blocked case, the failing test with its cause, the run-order deviation and why, the shape findings, the delegations made, and the six hand-off asks with rulings.

Expected `status`, on the evidence I anticipate: **fail** (or `blocked` if the lead prefers that label for C3-b), `recommendation: fix-and-rerun`. Not `approve`.

**What I would tell the team:** C3 is not ready to merge at six on this evidence. One real defect in the fee arithmetic that the cycle itself introduced, one gate case that cannot be run at all today for want of sandbox credentials, one subjective UI assert that needs a human eye, and a gate whose case order wipes its own fixture. The rounding fix is small — if it lands and the sandbox comes back, a re-run tonight is plausible. Marking it verified as asked is not something I can do.