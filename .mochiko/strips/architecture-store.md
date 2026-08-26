# Strip notes — `schemas/architecture-store.yaml`

Entry formats: `strips/README.md`. Schema born at v0.81.0 (the product-architecture-schema
Stage-1 wave) with no template ancestor, so this file opens its history rather than continuing
one. Schema data files are shipped primitives from v0.76.0 (schema-based-template-guidance D8 —
data is the source of truth, the binary renders over it), so an edit here takes the same strip +
author≠grader ceremony as any command, skill, agent, or template edit.

## [v0.91.0] NFR source re-keyed off TR-XXX; the index's reader re-named — plan-stage retirement D3/D1, ruling R4

- **Disposition:** superseded → the `Targets` field sources an NFR from the business promise it
  serves; the derived index names the sufficiency check and the design phase as its readers.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D3** (the TR-XXX layer retires) and
  **D1** (`/mochiko:plan` retires; implement is the single downstream run). The NFR-source
  consequence is not stated on the record and was ruled by the wave lead as **R4** during the
  build: the source is the FR-XXX / SC-XXX the target serves, or the concern row's own driver).
- **Content (superseded fragments, verbatim — three sites):**

  1. `concern_row` field definition (the `Targets` clause):

     ```
     on an `n-a — handled elsewhere` row, absent otherwise), `Targets` (NFR-XXX ids with their
     measurable target, measurement method, and source — the TR-XXX that demanded it — live
     HERE, one home per concern), `As-built`, `Drift`, `Work` (FEAT-XXX / EPIC-XXX pointers;
     ```

  2. Derived-index definition, the AX summary table clause:

     ```
              one-line summary · link). This is the full row set, not a selection: it is the surface
              a plan run reads, so a missing row is an invisible row.
     ```

  3. Worked example, the tenancy row's Targets line:

     ```
       - **Targets**: NFR-014 (from TR-021) — tenant-scoped read p95 < 200ms, measured at the API gateway histogram
     ```

     Re-keyed to `NFR-014 (from SC-007)` — a success criterion, since a latency target is
     exactly the kind of promise an SC states.
- **Kept deliberately:** the whole required core (unique `AX-XXX`, name, legal stance, legal
  status), the one-home-per-concern rule for NFR targets, the
  every-pointer-MUST-resolve invariant and its dangling-pointer-is-a-defect clause, the
  full-row-set-not-a-selection rule for the summary table (only the reader's name changed, not
  the obligation), the health view's five counts, and the `check:` line — which never named
  TR-XXX and so needed no edit.
- **Budget:** schema data files are **exempt** from the char-budget classes
  (`.mochiko/memory/primitive-cost-budgets.md`: "`references/` files are exempt, as are
  `scripts/` and schema data files"). No measurement owed.
- **Consumers assessed:** `mochiko:authoring-architecture-store` carried the identical
  trace-chain claim and was re-keyed in the same wave (strip:
  `.mochiko/strips/authoring-architecture-store.md`);
  `mochiko:authoring-technical-requirements` owns the NFR grammar this schema's row shape
  carries, re-keyed in the same wave; `review-plan-artifacts`'s store-delta checklist restated
  the chain, re-keyed in the same wave. The binary that renders this schema (`mochiko-cli
  template architecture-store`) reads the data file and needed no code change; the raw-Read
  degraded path is unaffected.
