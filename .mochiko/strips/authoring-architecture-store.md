# Strip notes — `skills/authoring-architecture-store`

Entry formats: `strips/README.md`. Skill born at v0.81.0 (the product-architecture-schema Stage-1
wave, D3/D4/D7 — it retires `authoring-architecture`, whose own strip history stays at
`strips/authoring-architecture.md`); this file opens with the first edit that superseded any of
its shipped text.

## [v0.91.0] Three plan-run references re-keyed — the write-gate rule, the index reader, and its checklist mirror — plan-stage retirement D1

- **Disposition:** superseded → "delivery run" and "design-time write" for the write-gate rule;
  "the sufficiency check and the design phase" for the index's named reader.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D1**: `/mochiko:plan` retires;
  `/mochiko:implement` is the single downstream run, and its design phase is where store deltas
  are now authored).
- **Found by sweep, not by brief:** the wave lead's extension named line 61 of this file. These
  three sites surfaced on the final residue sweep, in the same file, from the same ruling. The
  file was already allocated to this seat, so they were fixed here rather than left to contradict
  the line-61 edit.
- **Content (superseded fragments, verbatim — three sites):**

  1. Element-lifecycle section, the write-gate rule:

     ```
     **Ruled truth is never edited in place by a plan run.** A plan-time write is legal only as an
     in-flight-class delta, and only after the user's sign-off on the rendered diagram plus the named
     row changes — the sign-off IS the write gate. No sign-off, no store write.
     ```
  2. Derived-index section:

     ```
     It carries the spine thumbnail, the **full** AX summary table (every row — plan runs read the
     trip check here, so a missing row is an invisible row), and **Health**.
     ```
  3. Quality Checklist mirror of site 1:

     ```
     - [ ] No store write without its user sign-off; ruled truth never edited in place by a plan run
     ```

- **Kept deliberately:** the write gate itself in full — a store write is legal only as an
  in-flight-class delta, only after the user's sign-off on the rendered diagram plus the named
  row changes, and **the sign-off IS the write gate / no sign-off, no store write** is byte-for-byte
  intact. Likewise the full-row-set rule and its missing-row-is-an-invisible-row clause: only
  the name of the reader changed. Site 2 was re-keyed to match `schemas/architecture-store.yaml`
  :130, which carries the same sentence and was re-keyed identically in this wave.
- **Budget:** unbudgeted (hard-cap-only). Body 10,841 → **10,884** across these three sites
  (+43); description unchanged at 492.
- **Consumers assessed:** `schemas/architecture-store.yaml` (the parallel index sentence, re-keyed
  same wave, strip `.mochiko/strips/architecture-store.md`); `mochiko:patterns-system-design`
  drafts the delta this write gate governs — **it still says "recorded in the plan package" at
  its line 96 and is NOT allocated to this seat**, reported to the wave lead.

## [v0.91.0] NFR trace-chain claim re-keyed — `TR-XXX → NFR-XXX` becomes `FR-XXX / SC-XXX → NFR-XXX` — plan-stage retirement D3, ruling R4

- **Disposition:** superseded → the same sentence asserting the chain resolves to the business
  source.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D3**, which retires the TR-XXX
  layer; the NFR-source consequence is **not** stated on the record and was ruled by the wave
  lead as **R4** during the build: an NFR's source is the FR-XXX / SC-XXX it serves, or the
  concern row's own driver).
- **Why this file was touched at all:** D3 kills the upper link of a chain this skill asserts
  still resolves. Left alone, the store's own owner would claim a traceability chain whose
  top-level id class no longer exists — the kind of dead claim the record layer's
  no-silent-corruption principle exists to prevent.
- **Content (superseded text, verbatim):**

  ```
  `NFR-XXX` targets live **on the concern row they belong to** — one home per concern, stance and
  pattern and target and as-built together. The ids survive unchanged; only the path moved, so
  `TR-XXX → NFR-XXX` trace chains keep resolving.
  ```

  Replaced by the same sentence ending "The ids survive unchanged, and each target names its
  business source, so `FR-XXX / SC-XXX → NFR-XXX` trace chains keep resolving."
- **Kept deliberately:** the one-home-per-concern rule and the whole stance/pattern/target/
  as-built co-location it exists to state, the ids-survive-unchanged clause, and the
  chains-keep-resolving promise itself — what changed is which id sits at the top of the chain,
  not that the chain must resolve. The v0.81.0 D12 ruling that moved NFR rows onto concern rows
  is untouched.
- **Budget:** the skill is unbudgeted at birth (hard-cap-only). Body measured 10,810 at the
  v0.81.0 release-gate sweep and **10,841 after this edit (+31)**. Description untouched at 492.
  (An earlier draft of this entry estimated +9 from the edit text rather than measuring;
  corrected to the canonical-snippet count before the audit.) The file took a second [v0.91.0]
  edit after this one — see the entry above; its landed figure is **10,884**.
- **Consumers assessed:** `plugins/mochiko/schemas/architecture-store.yaml` carried the same
  claim in its `Targets` field definition and in a worked example — both re-keyed in the same
  wave (strip: `.mochiko/strips/architecture-store.md`);
  `mochiko:authoring-technical-requirements` owns the NFR grammar and was re-keyed to the
  business source in the same wave; `review-plan-artifacts`'s store-delta checklist restated the
  chain and was re-keyed too.
