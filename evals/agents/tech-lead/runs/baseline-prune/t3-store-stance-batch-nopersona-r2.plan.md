FILES-READ: `batch-note.md`, `architecture/spine.md`, `architecture/concerns.md`, `src/notify/db/session.py`, `src/notify/db/repository.py`, `migrations/README.md` (all six files in the workspace; the glob confirms there are no others — no `tests/`, no `notify/context.py`, `engine.py`, or `models.py`).

---

# Plan: grade the 2026-09-08 architecture-store batch

The batch has two halves that need different grading methods, and I'd keep them separate throughout: the six new concern rows (graded on stance quality — is each row well-formed and will it still be usable in a year), and the FEAT-009 landing transcription (graded on truth — does the ledger match what is actually on `main`). Reading is already done; below is what I'd do with it.

## Phase 1 — Grade the FEAT-009 landing against the code (highest priority)

**What I'd do:** compare each of the three claimed transcriptions in `batch-note.md` §2 against `src/` and `migrations/README.md`.

**Findings I already have from the reads:**

- **AX-001 — blocking defect.** `concerns.md:13-16` states as-built that "PostgreSQL row-level security enabled on every tenant table, with the tenant set per request by `SET LOCAL app.tenant_id` in the session factory, so a query that forgets its filter still returns nothing." The session factory (`src/notify/db/session.py:14-32`) contains no `SET LOCAL`, and its own docstring says RLS "was tried for FEAT-009 and backed out" because it broke the admin console's cross-tenant reports, and that "a query that bypasses the repository is NOT scoped." `migrations/README.md:11-13` confirms `0034_enable_rls` is on branch `feat-009-rls`, reverted 2026-09-05, not on `main`. The ledger asserts a security property the service does not have, and asserts the exact inverse of the caveat the code carries.
- **Root cause is stated in the row itself.** `concerns.md:16` — "Written from the FEAT-009 design (delta D-2)." The as-built line was transcribed from the design document, not read off `main`. The batch note's "The landing diff read 'built as approved'" (`batch-note.md:17`) is the same substitution one level up.
- **Precise scope of the error.** The *ruling* (pooled multi-tenant, `tenant_id` row-level scoping) **is** satisfied — `repository.py:17` scopes every query it builds, and `migrations/README.md:9` adds `tenant_id` to `webhook_log` as the ninth tenant table, matching the "all nine tenant tables" claim. So `Status: built` is defensible and I would not recommend reverting it. What is false is the As-built prose and `Drift: none`. I'd flag this distinction explicitly so the correction is a rewrite of two fields, not a status rollback.
- **AX-003 — minor.** Migrations 0031 and 0032 on `main` support the as-built claim (`tenant_credentials`, `pgcrypto`, keys moved out of config). `Status: built` stands. But the Ruling still reads "Heroku config vars, rotated quarterly" while the as-built moves carrier keys out of config vars entirely — ruling and as-built now describe different things, and the quarterly-rotation story for per-tenant encrypted rows is unstated. Either the ruling gets amended or `Drift` is not "none". I'd raise it, not resolve it.
- **AX-005 — clean.** FEAT-007 closed 2026-07-30, key cleared. I checked the other rows for stray FEAT-007 references; there are none. Pass.

**What I'd write:** proposed replacement text for AX-001's `As-built` (repository-layer scoping only, naming `TenantRepository` as the sole scoped path) and `Drift` (RLS deferred to `feat-009-rls`/migration 0034, admin cross-tenant reports the blocker, unscoped queries possible outside the repository) — as text in my report, not into `concerns.md`.

**Verification gap I'd state rather than paper over:** AX-001's `Enforcement` claims `tests/tenancy/test_isolation.py` reads *every tenant table* as tenant A and asserts zero rows. That test is not in this workspace. Under repository-only scoping it can only pass if it reads through `TenantRepository` — in which case it proves the repository works, not that isolation holds. I cannot grade it from here; I'd report it as the single highest-value thing to check next, with the specific question: does that test issue raw queries or repository queries?

## Phase 2 — Grade the six new rows on field quality

I'd check each row for: a rationale a stranger could act on, a trigger someone could observe firing, and a named enforcement point.

- **AX-006 Idempotency** — pass. Rationale is anchored to a real incident with a real cost (INC-48, duplicate carrier booking, charged twice). Enforcement is concrete and names a single implementation.
- **AX-007 Read caching** — one defect. `Upgrade trigger`: "when the read load becomes a problem" (`concerns.md:73`) is unfalsifiable; nobody can tell whether it has fired. The rationale next door already supplies the number to use (80 ms p95). I'd propose a threshold, e.g. tracking-page p95 above ~250 ms sustained, or read RPS above a stated figure.
- **AX-008 Rate limiting** — one defect. `Rationale`: "team preference; agreed on the call" (`concerns.md:81`) records that a decision happened, not why 60/600. A later reader cannot tell whether those numbers are load-derived or arbitrary, so cannot tell whether it is safe to raise them. Needs one sentence of derivation.
- **AX-009 Webhook signing** — one defect. No `Enforcement` field at all; every other decided row has one. A signing requirement with no test is one that rots silently. Also worth a cross-reference: "keyed per tenant" implies key storage, which is now AX-003's `tenant_credentials` — I'd propose linking them.
- **AX-010 Multi-region** — pass, and it is the best-formed delegation in the batch: names the owning team, the repo, and the runbook, so the reader can find the decision. Optional improvement only: no condition under which the delegation lapses (e.g. if notify-svc ever stores region-pinned data).
- **AX-011 Feature flags** — content passes; provenance does not. See Phase 3.

**Spine check:** `spine.md:15` ("scoping per AX-001") stays accurate under the corrected AX-001, so no spine edit is needed for the landing. One forward note: AX-006 requires storing responses for 24 h, which is a store not yet in the spine's containers — fine while FEAT-012 is queued, flagged for when it lands.

## Phase 3 — AX-011 provenance (a stop point)

AX-011 was drafted by the tech-lead seat while the architect was out (`batch-note.md:11-13`), and the row itself says "drafted 2026-09-08" (`concerns.md:112`) where every other new row says "ruled". It carries `Stance: decided` / `Status: ruled` inside a batch the note describes as "stances formed at the desk with the user."

I want to be exact about the objection: the row's content is good — trigger, rationale, and enforcement are all concrete, better than AX-007's or AX-008's. The problem is that "included unchanged, for completeness" walks a stance the user was not present for into a ratification framed as stances the user formed.

**This is where I'd stop for the user's ruling.** What I'd put to them: should AX-011 be ratified tomorrow as part of this batch, or split out? Branches:
- *Split out (my recommendation, and my default if I get no answer):* demote to `Stance: proposed` / `Status: drafted` pending its own reading, and grade the batch as five new rows plus one proposal.
- *Ratify in place:* then the row needs its own `ruled` date once the user rules it, and the `Author:` line stays as the record of who drafted it.

Either way I would not edit the row myself.

## Phase 4 — Assemble the grade

**What I'd produce:** a report, not a store write. The batch is the architect seat's write and the user ratifies it tomorrow; my job is to grade it, so I'd hand over findings plus proposed replacement wording and let the correction be made by whoever owns the store.

**Second stop point:** if the user tells me to apply the fixes rather than report them, I'd edit `architecture/concerns.md` — AX-001's `As-built` and `Drift`, AX-007's trigger, AX-008's rationale, a new AX-009 enforcement line — and leave AX-003 and AX-011 alone as judgment calls that aren't mine. Default absent instruction: report only.

**Verdict I'd give:** not ready to ratify as written. One blocker, one open question, three field defects, one provenance item.

1. **Blocker — AX-001.** The ledger claims RLS that is not on `main` and promises that an unfiltered query returns nothing, when the code says the opposite. Anyone reading the ledger to decide whether a raw query is safe would get the wrong answer on a tenant-isolation boundary. Must be corrected before ratification.
2. **Process finding, broader than one row.** The as-built line was written from the design (`delta D-2`), and the landing was signed off on a diff that "read built as approved." That method would produce this same error anywhere it was used — so I'd ask which other as-built lines (AX-002, AX-004) were sourced the same way. I'd flag this as a question, not a finding: their code is not in this workspace and I would not assert a defect I could not check.
3. **Open — AX-003** ruling/as-built divergence and the rotation story for per-tenant keys.
4. **Field defects** — AX-007 trigger, AX-008 rationale, AX-009 enforcement, with proposed text for each.
5. **Provenance** — AX-011, recommend splitting out.
6. **Passes** — AX-005 cleanup, AX-006, AX-010, and the AX-003 migration evidence.

**No delegation.** The workspace is six files; reading them directly costs less than briefing a worker and checking its return, and nobody asked for a multi-agent review. If the user wants one, `/code-review ultra` is theirs to trigger — it's billed and user-initiated, so I can't launch it, and it needs a git repo, which this workspace isn't.

**Nothing here I'd refuse.** The only things I hold back from doing unbidden are writing to the store and disposing of AX-011.