# Trace summary — governance surface set v3.2.0 (2026-09-24, AM-5)

The producer's manifest over the ratified synthesis (`governance-intent.md`, GI-001–022; this
round's intent is the AM-5 entry). Grading surface for `validation-constitution`. Superseded
rounds: v2.0.0 and v3.0.0 carried their manifests inside their producer reports
(`.mochiko/brainstorms/cli-schema-delivery/reports/wave2-gov-producer.md` for v3.0.0), and the
v1.0.0 manifest this file held — including its ruled FP-1 — is in git history. The v3.1.0
manifest (AM-3, with its FP-1 to FP-5) is in git history at `794cea8`. v3.1.1 was a lead PATCH
(the `validator` retirement) that carried no manifest. The v3.1.2 manifest (AM-4, its ruled FP-1
and validator record) is in git history at `8f4e5ab`. The v3.1.3 number is retired unminted (its
owed PATCH folded into AM-5).

## Trace summary — GI element to homes

| GI-ID | Principle / element | Source | Primary home | Companions present |
|-------|--------------------|--------|--------------|--------------------|
| GI-001 | Fact profile — no compliance modules, negatives incl. no-UI; unchanged at AM-5 | session fact profile | region ratified stamp | ledger header ✓ · **stamp re-versioned v3.2.0 · 2026-09-24 (AM-5)** at region `:109` and ledger `:4` |
| GI-002 | Identity · risk surface · team reality | session dimensions 1–5 | region Technology stack line | ledger amendment-policy trigger ✓; no Three-Part entry (FP-3 at AM-2, accepted) — unchanged at AM-5; the first-public-release trigger stays open (AM-5 Q3), restated on the exception row `:88` |
| GI-003 | Secrets out of the repo | floor-asserted: FLOOR-SEC | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ — unchanged at AM-5 |
| GI-004 | Primitive audit ratchet | floor-asserted: FLOOR-TEST | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ · `rust-cli.md` quality-gate bullet ✓ — unchanged at AM-5 |
| GI-005 | Record-layer integrity | floor-asserted: FLOOR-ERR | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ · `rust-cli.md` log-is-truth bullet ✓ — unchanged at AM-5 |
| GI-006 | Traceability as observability | floor-asserted: FLOOR-OBS | region line (NON-NEGOTIABLE) | index = line ✓ · ledger ✓ — unchanged at AM-5; binds this round: the AM-3 `rust-cli.md` bullet preserved at ledger `:528-541`, the AM-3 GI-019 wording pointed at `8f4e5ab` from `:483-494` |
| GI-007 | Deliberate exclusions, narrowed to markdown primitives | session dimension 10 | ledger GI-004/005/006 trace lines | synthesis ✓ — unchanged at AM-5 |
| GI-008 | Waiver — FLOOR-TEST for the skill-shipped helpers | waiver | ledger waiver table | synthesis ✓ — unchanged at AM-5 |
| GI-009 | Knowledge-management core | module: knowledge-management-core | `.mochiko/memory/knowledge-management.md` | region Governance-operations line `:140` ✓ · ledger GI-009 ✓ · `operating-docs.md` ✓ — unchanged at AM-5 |
| GI-010 | CHANGELOG elective | module: km-elective-changelog | `CHANGELOG.md` | ledger ✓ · region release-gates line, gate 4 ✓ · `rust-cli.md` `paths` ✓ — unchanged at AM-5 (the exception row asks each bump's entry to cite it — a GI-012 obligation, not a GI-010 change) |
| GI-011 | RUNBOOK elective — declined durable | module decline | ledger declined-durable line | synthesis ✓ — unchanged at AM-5 |
| GI-012 | Release gates — **wave-4 hook-ship precondition breached at 0.109.0, excepted; held-bump rule added** | module: release-gates | ledger GI-012 (detail) | region release-gates line `:129` (clause verbatim + breach pointer) ✓ · region amend-triggers line `:135` (breach mark) ✓ · ledger amendment policy `:62-69` (void annotation + breach pointer) ✓ · **exception registry row `:88`** (incl. FP-1) ✓ · GI-012 held bumps `:281-282` · breach marker `:298-300` · void annotation `:310` · Testability `:333` · trace `:340` ✓ · amendment log `:709` ✓ · `rust-cli.md` ✓ (unchanged) |
| GI-013 | layer-rules — declined durable | module decline | ledger amendment-policy line | ✓ — unchanged at AM-5 |
| GI-014 | evolution-notes — declined durable | module decline | ledger amendment-policy line | ✓ — unchanged at AM-5 |
| GI-015 | Live-token confrontation | brownfield confrontation | ledger confrontation table | folded into GI-003 enforcement ✓ — unchanged at AM-5 |
| GI-016 | Marketplace-lag confrontation | brownfield confrontation | ledger confrontation table | folded into GI-012 gate 5 ✓ — unchanged at AM-5 |
| GI-017 | Pointer-only region | minted | region line | index = line ✓ · ledger ✓ — **binds this round:** region `:129`/`:135` gain pointers only, the exception's detail stays in the ledger; the stated-limits line `:420-422` points at the `hook-enforced-artifact-schema` record § Build trail |
| GI-018 | ARCHITECTURE.md lag accepted | brownfield confrontation | ledger confrontation table | ✓ — unchanged at AM-5 |
| GI-019 | Kernel-class tooling admission (bright line) — **check-source predicate widened to the repository's own layout (R1); gate paragraphs re-worded for the field review, stated as the gate with a build-state line (Q2 `Contested`); owed PATCH (a, c, d, e) folded** | minted AM-1 · widened AM-2 · conformance-gate admission AM-3 · predicate widened AM-5 | `CLAUDE.md` `## Non-negotiable constraints` prose (unchanged) | region index line ✓ (unchanged) · ledger admission `:400-419` · stated limits `:420-422` · clause (iv) `:423-432` · C1 note `:433` · amnesty `:444-459` · reach `:460-477` · reads `:478-482` · build-state line `:483-494` · supersession sub-block `:528-541` · Testability `:543-564` · trace `:572` ✓ · `rust-cli.md` bright-line bullet `:23-35` ✓ |
| GI-020 | Clone-only install, required `mochiko-cli` | minted AM-1, superseded-by-ruling AM-2 | `CLAUDE.md` `## Non-negotiable constraints` prose | region index line ✓ · ledger ✓ · `rust-cli.md` dependency bullet ✓ — unchanged at AM-5 |
| GI-021 | Depth level `high` | minted AM-1 | region ratified stamp | ledger header ✓ — unchanged at AM-5 |
| GI-022 | No repo-level feature map for this repo | minted AM-2 | region index line | ledger ✓ — unchanged at AM-5 |

## Trace summary, reverse — changed surface line to GI element

| Surface | Changed line | GI |
|---------|--------------|-----|
| region `:109` | ratified stamp v3.1.2 → v3.2.0 · 2026-09-24 (AM-5) | GI-001, GI-021 |
| region `:129` | release-gates line: precondition clause kept verbatim, "— breached at 0.109.0, excepted under the ledger's exception registry (AM-5)" appended inside its bold span | GI-012, GI-017 |
| region `:135` | amend-triggers line: "(breached at 0.109.0 — exception registry)" after "hard preconditions of the wave-4 hook ship" (lead-ruled inside Q3) | GI-012, GI-017 |
| ledger `:4` | version 3.1.2 → 3.2.0 | GI-001 |
| ledger `:62-69` | amendment policy first-publish bullet: AM-3 text verbatim; void annotation on its pre-authorized strike; AM-5 breach pointer to the exception | GI-012 |
| ledger `:88` | exception registry: first row (breach, how, exposure, CHANGELOG citation, expiry strike set, tripwire, 2026-12-31 backstop) — its amend-triggers strike-set item is FP-1, user-accepted at acceptance | GI-012, GI-002 |
| ledger `:281-282` | GI-012 Enforcement: held-bump rule (R3) | GI-012 |
| ledger `:298-300` | GI-012: breach-marker paragraph before the AM-3 precondition paragraph | GI-012 |
| ledger `:310` | GI-012 AM-3 paragraph: void annotation on its pre-authorized strike; paragraph otherwise verbatim | GI-012 |
| ledger `:333` | GI-012 Testability: Pass/Fail hook-ship limbs point at the exception; Fail gains the held-bump limb | GI-012 |
| ledger `:340` | GI-012 trace: AM-5 clause | GI-012 |
| ledger `:400-419` | GI-019 admission: check-kind list re-worded (closed world, run-key name, per-entry budgets, report sniff, write position, ignore guard, worktree test); predicate widened to the repository's own layout (R1) | GI-019 |
| ledger `:420-422` | GI-019: stated-limits pointer (d) | GI-019, GI-017 |
| ledger `:423-432` | GI-019 clause (iv) argument body: R1 rationale sentence | GI-019 |
| ledger `:433` | GI-019 C1 note: (e) carried into the 3.2.0 row, v3.1.3 retired; field result | GI-019, GI-012 |
| ledger `:444-459` | GI-019 amnesty: closed-world path clause, its outside-`.mochiko/` limb worded as Reach's (acceptance advisory 3); file-set context; known-gap corrected (c); "in a declared home" | GI-019 |
| ledger `:460-477` | GI-019 Reach re-worded for the closed world | GI-019 |
| ledger `:478-482` | GI-019 What the gate reads widened | GI-019 |
| ledger `:483-494` | GI-019 build-state line with strike trigger (Q2 `Contested`, R2); the strike also takes the `rust-cli.md` build-state pointer (validator delta) | GI-019, GI-006 |
| ledger `:528-541` | GI-019 recorded supersession: AM-3 `rust-cli.md` bullet preserved | GI-019, GI-006 |
| ledger `:543-564` | GI-019 Testability: tier 1 re-pointed to the conformance tier, both limbs noted "(re-pointed at AM-5 (a))" (acceptance advisory 2); conformance tier active (a), R1 limbs | GI-019 |
| ledger `:572` | GI-019 trace: AM-5 clause | GI-019 |
| ledger `:709` | amendment-log row 3.2.0 (MINOR; (e) flips, v3.1.3 retired, AM-3 strike void, FP-1 named) | all of the above |
| `rust-cli.md` `:23-35` | bright-line bullet: closed-world path, per-entry size, repository-layout facts, no judgment (R1); build-state pointer to ledger GI-019 (acceptance advisory 1) | GI-019, GI-017 |
| this file | regenerated whole at v3.2.0; superseded-rounds pointer extended to the v3.1.2 manifest | GI-006 |

## Floor coverage

All four Essential Floor categories accounted for, none dropped and none re-expressed at AM-5:
GI-003 (FLOOR-SEC) · GI-004 (FLOOR-TEST) · GI-005 (FLOOR-ERR) · GI-006 (FLOOR-OBS). Module set
unchanged: compliance none · knowledge-management (core + CHANGELOG) · release-gates. Depth
`high` unchanged.

## Flagged proposals — ruled by the user at acceptance (2026-09-24, "accept")

- **FP-1 — accepted.** The exception row's expiry strike set (ledger `:88`) includes the region
  amend-triggers line's (`CLAUDE.md:135`) "those two are hard preconditions of the wave-4 hook
  ship" clause and its breach mark. **Why:** without it the expiry PATCH would leave a clause on
  the always-on region that still reads as a live precondition. It goes beyond synthesis Q3/R6's
  enumerated strike set and was ruled in by the user at acceptance; the row's temporary FP-1
  wrapper is removed, and the `:709` log row reads "(FP-1, user-accepted at acceptance)". *(The
  `:135` breach mark itself is lead-ruled inside Q3 — not part of FP-1.)*
- **FP-2 — declined by the user (not authored).** A touch-time pointer in `rust-cli.md`'s
  Release bullet: while the GI-012 exception row is open, each bump's `CHANGELOG.md` entry cites
  it.

**Lead-ruled formulation (not proposals):** the "GI-012's two paragraphs" strike-set reading
(FC-3, refined by A1 — the release-train paragraph's closing first-publish sentence only, its
tag-train rule stays, plus the AM-3 precondition paragraph, with the AM-5 breach marker joining);
the `:135` breach mark.

**Waivers:** GI-008 (sole) — row and notes unchanged at AM-5.
**Exceptions:** the AM-5 GI-012 row (`:88`), expiring at the first `mochiko-cli-v*` publish with
all four controls; backstop 2026-12-31.
**Narrowings:** GI-003 secret-scanning, recorded in its ledger entry and un-narrowed at v0.76.0.
**Owed at the next PATCH (carried from v3.1.2, still outside scope):** the ledger's semver MAJOR
limb lacks the template's `depth-level flip (low→high)` clause.

## Validator record (evidence floor)

PASS 60/60 (fresh `validation-constitution` seat, round 1, 2026-09-24); advisories 1–4 taken at
acceptance, 5 at the landing (BACKLOG), 6 this line, 7 disclosed (haiku Explore hand-back
failures, fact checks run by the grader).
