# Setup ↔ Operating-Docs Scaffolding — Synthesis

> ***derived — record canonical.*** This is an on-request digest of [`record.md`](record.md) (accepted 2026-07-17). The record holds the full rationale, the verbatim fact-checker map, and the complete review trail; on any divergence, the record wins.

## Problem Statement

`/mochiko:setup` establishes governance (`.mochiko/memory/` only) but nothing of the operating-docs layer mochiko itself runs on — brainstorm session organisation with its index, `BACKLOG.md`, `ROADMAP.md`. That layer existed only as mochiko-repo convention in `CLAUDE.md`, with no carrier in any command: on a fresh target project, brainstorm sessions would accumulate with no index and the discipline would simply be absent. The session's refined problem: this layer is a *governance* concern — knowledge management organized and **enforced**, not merely documented — and setup is where it must be minted.

## Key Decisions

| # | Decision | Choice | Confidence | Rationale (brief) |
|---|----------|--------|------------|-------------------|
| D1 | Ownership | The operating-docs layer is constitutionally owned — part of governance, not bare scaffolding | `Confident` | Governance is what makes structure enforced rather than documented; mints the framework-level ruling no file yet stated |
| D2 | Bindingness | Elective constitution **module**, default-on (user must actively decline); new interrogation dimension surfaces it | `Confident` | Universal core would reintroduce the fixed-baseline imposition the flexibility ruling killed; decisive case: PoC-tier spikes. Opt-in was weighed and rejected — projects most needing structure ask least |
| D3 | Contents | Fixed four-part bundle, adopted or declined whole: `.mochiko/brainstorms/<slug>/` + `index.md` (with maintenance contract), repo-root `BACKLOG.md` + `ROADMAP.md` with circulation roles | `Confident` | Flexibility already lives at adoption level; the layer's value is the circulation between parts; adopting ≠ obligated to run sessions (index invariant vacuous at zero sessions). Must disambiguate from brownfield `evolution-roadmap.md` |
| D4 | Enforcement | Three carriers: (1) command index-bookkeeping, (2) CLAUDE.md-sync rows (stub-backed until `syncing-claude-md` ports), (3) testable invariants — executor: session-producing commands at open/close, fix-on-sight; validator re-audit at amend | `Confident` | Covers write-time, ambient-session, and audit-time failure modes; kernel-free. Content *quality* of BACKLOG/ROADMAP explicitly exempt from mechanical enforcement |
| D5 | Placement | New knowledge-management dimension at **#7**, after "existing practices & tools" (agenda → ten dimensions); G5 finalize scaffolds the adopted bundle and suggests `/mochiko:brainstorm` | `Confident` | All adopt/decline-relevant context is declared by dimension #6, and brownfield doc evidence is freshest there |
| D6 | Brownfield collisions | Codify existing docs where semantics fit; true collisions surface in the KM dimension for a per-project ruling; hard floor: never overwrite or rename | `Confident` | Collisions are always high-context — the user rules on evidence; silent defaults rejected on principle ("rare" frequency claim conceded and struck in review) |
| D7 | Amend runs | Offer any module the constitution has no recorded ruling on — once; record adopt/decline in `governance-intent.md` either way | `Confident` | Reaches the installed base without turning amend into a recurring pitch; a recorded decline is permanent until the user reopens it |

Ratification profile (disclosed in the record): D3, D4, D6, D7 were adopted on lead recommendation with explicit user ratification; the streak was flagged mid-session and the user re-engaged at the placement fork.

## Decision Trail (deliberated turns only)

- **D2 (reversal):** user first ruled universal core; reversed to the elective module on the lead's single pushback — a PoC spike forced to carry roadmap/backlog/index learns to ignore its constitution.
- **D3:** inner-menu and fully-parameterized alternatives set aside — a second menu doubles decision surface; uniform layout keeps the convention teachable. Post-review seam answer: unused session machinery costs one empty `index.md`.
- **D6:** silent-fallback and clean-names-or-decline set aside; the never-overwrite floor plus one evidence-backed interrogation beat won.

## Risks (surfaced in review)

- **Repo-root writes are new blast radius:** adopting the module makes setup write repo-root files for the first time (previously `.mochiko/memory/` only) — accepted deliberately; every root write sits under the D6 floor.
- **Between-amend drift:** on a project that opens no further sessions, index drift is caught only at the next amend re-audit — the accepted kernel-free tradeoff.
- **Collision beat may fire routinely** (root `ROADMAP.md` is a common filename); policy stands on principle, not frequency.

## Open Questions

- **Module-elicitation scaling** `Deferred` — one agenda dimension per future module, or a consolidated modules beat? Revisit at the next constitution module design.

## Build Items (5 — the record's derived worklist)

1. `INTERROGATION-AGENDA.md`: new KM dimension #7 + brownfield collision beat.
2. New `constitution-modules/knowledge-management.md`: the bundle, Three-Part-Rule complete, invariants, `evolution-roadmap.md` disambiguation.
3. `constitution-template.md`: Mandatory Sync Artifacts rows (conditional on adoption).
4. `setup.md`: dimension wiring; G5 scaffold (never-overwrite, incl. repo root) + `/mochiko:brainstorm` next-step; amend once-offer recorded in `governance-intent.md`.
5. `brainstorm.md`: index bookkeeping steps + no-module branch; runs invariants at open/close (D4 carrier 3's executor).

## Review Outcome

Single reviewer (user-sized at the gate): 12 findings raised → 9 survived → 9/9 dispositioned (6 user-ruled "accept all", 3 lead-resolved); verify pass **PASS**, contradiction sweep clean; fact-checker map sample-audit clean.

## Next Steps

User lands the changes directly from the record after this session (ROADMAP/BACKLOG entries deliberately not written by the session — user's explicit choice at close).
