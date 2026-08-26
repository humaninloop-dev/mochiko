# Strip notes — `skills/testing-gap-finding`

Entry formats: `strips/README.md`. Skill born at v0.79.0 (the QA gap-finding build); this file
opens with the first edit that superseded any of its shipped text.

## [v0.91.0] Fix round 2 — the durable gate set's mint moment re-keyed off plan time (V1 multiline sweep)

- **Disposition:** superseded → "minted at first fold (or at **card authoring inside the
  implement run**)".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1: cards are authored inside the
  implement run — after the design phase, or directly on a zero-gap sufficiency verdict — never
  in a separate plan run). Caught by V1's **multiline-aware** sweep: the phrase wrapped across
  two lines as "at plan\ntime", so every line-scoped grep in this wave — including the
  twenty-term sweeps this seat ran — missed it.
- **Content (superseded text, verbatim, with its line break):**

  ```
  **The artifact:** `.mochiko/features/FEAT-XXX/gates.md` — minted at first fold (or at plan
  time, when the cards are authored) and **surviving graduation**:
  ```

- **Kept deliberately:** the whole artifact contract — `gates.md` mints at first fold, **survives
  graduation** (work rows vanish, the gate set persists), and is the named read source of the
  accumulated territory `**TEST:**` gates, read at every later final validation as the union of
  the territory features' `gates.md` plus their cards' cases. The parenthetical's *point* is
  unchanged: the file can also come into being when the cards are authored, ahead of any fold —
  only the name of that moment moved.
- **Budget:** unbudgeted (hard-cap-only). Body 11,052 → **11,053**; description unchanged at 709,
  inside the 1,536 cap. (Canonical-snippet count taken after the edit, not estimated.)
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` owns card authoring and was re-keyed to
  the same moment in the main pass ("inside the `/mochiko:implement` run — after the design
  phase, or directly on a zero-gap sufficiency verdict"); the router's `testing-gap-finding` row
  names the fold-back target `gates.md` but not its mint moment — no re-key owed there.

**Sweep lesson, recorded for the next wave:** two of this wave's misses (this one and
`authoring-epic`'s Why-together line) were invisible to line-scoped greps — one because the
phrase wrapped mid-sentence, one because it named a mechanism rather than a stage. A
vocabulary-retirement sweep should run multiline-aware (`grep -Pzo` or equivalent) and include
the *mechanism* names a retirement kills, not only the stage names.

## [v0.91.0] Blindness-fence inclusion list: the `requirements.md` member re-keyed to the sufficiency report + design-phase deltas — plan-stage retirement D3, Addendum A6

- **Disposition:** superseded → two named members in its place — the run's **sufficiency report**
  and the feature's **design-phase deltas**.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D3 fence consequence: "`requirements.md`
  is a named member of `mochiko:testing-gap-finding`'s explicit inclusion list; its slot re-keys
  to the sufficiency report + the design-phase deltas (spec-layer artifacts, never code); the
  narrowing is recorded, its adequacy watched (Open questions)"; plus Addendum **A6**: "map
  entries live at `.mochiko/features/FEAT-XXX-<slug>.md`; the `FEAT-XXX/` directory is run
  output, out of fence").
- **Content (superseded text, verbatim):**

  ```
  not a layer label: `spec.md` (FR-XXX, SC-XXX, stories, declared edge cases) · the feature's
  `requirements.md` · Screens & Flows (SCR-XXX, FLOW-XXX) · `data-model.md` (entities, state
  ```

  Replaced by the same list with the `requirements.md` member swapped for "the run's
  **sufficiency report** and the feature's **design-phase deltas** — those two artifacts only,
  never the `FEAT-XXX/` run-output directory at large".
- **A6 application, recorded because it is a judgment call an auditor will check:** the fence
  names no map-entry path today, so A6's map-entry half is a no-op here. Its operative half is
  the run-output distinction, and it bites on the *new* members: both replacements live under
  `.mochiko/features/FEAT-XXX/`, which A6 declares out of fence as a directory. The fence
  therefore admits the **two named artifacts only** and says so explicitly, rather than
  admitting the directory that contains them. Without that guard the re-key would have widened
  the fence to the cards and cycle reports the pass is structurally blind to.
- **Budget:** the skill is unbudgeted (hard-cap-only). Body 10,929 → 11,052 chars; description
  unchanged at 709. No budget obligation; the ≤1,536 description cap holds.
- **Kept deliberately:** every other inclusion-list member (`spec.md`, Screens & Flows,
  `data-model.md`, `contracts/`, the store's concern rows for their `NFR-XXX` targets), the
  spine-stays-outside-the-fence guard, the whole structural exclusion list (code, cards,
  `**TEST:**` cases, cycle reports, the builder's tests), the delegated-reads-inherit-the-list
  rule, and two-message dispatch. The narrowing is one member wide.
- **Open watch (from the ruling, not discharged here):** whether the sufficiency report plus the
  design-phase deltas carry what `requirements.md` carried for the blind explorer's expectation
  derivation. The record parks this in Open questions, to be watched at the first gap-finding
  pass under the new shape.
- **Consumers assessed:** the router's `testing-gap-finding` row restates the fence list and was
  re-keyed in the same wave; `mochiko:patterns-vertical-tdd` owns the `**TEST:**` grammar this
  skill consumes (untouched); `implement.md` (P1's rewrite dispatches the pass at final
  validation).

## [v0.81.0] Runtime-NFR references re-pointed from `nfrs.md` to the store's concern rows — product-architecture-schema D12

- **Disposition:** superseded → the architecture store's concern rows
  (`.mochiko/product/architecture/concerns.md` plus any graduated
  `concerns/AX-XXX-<slug>.md`), which now home the `NFR-XXX` targets. `nfrs.md` dies as a file;
  the ids and the targets survive, so all three references move rather than drop.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12, `Contested` — the absorb
  names `testing-gap-finding`/gates runtime-NFR re-points among its added consumer rewires;
  `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — three fragments):**

  ```
  `contracts/` (`api.yaml` and any sibling contract documents) ·
  `nfrs.md`. All define externally-observable promised behavior, so the pass stays black-box.
  ```

  ```
  4. **Runtime NFR** — each `nfrs.md` numeric target (p95, availability, limits) as a measurable
     expectation against the built system.
  ```

  ```
    broken, with **evidence captured** and the **spec clause cited**. A broken `nfrs.md` numeric
    target qualifies.
  ```
- **Kept deliberately:** the blindness fence's shape and every other admissible input verbatim
  (`spec.md` · the feature's `requirements.md` · Screens & Flows · `data-model.md` ·
  `contracts/`); the explicit-inclusion-list-not-a-layer-label rule; the structural exclusions
  (code · cards · `**TEST:**` cases · cycle reports · the builder's tests); two-message dispatch;
  the finding-kind split, which still makes a broken numeric target a **spec-violation** — the
  clause it cites moved home, its blocking force did not.
- **Fence guard (addition riding the decision row, no strip):** the inclusion admits the store's
  **concern rows only**. The **spine deep view is excluded** — it is design structure, not
  externally-observable promised behavior, and the per-feature `architecture.md` it replaces was
  never an admissible input either. Admitting the whole store would have silently widened the
  fence that D12's re-point had no mandate to widen; the narrow read keeps the pass black-box.
- **Consumers assessed:** the router's `testing-gap-finding` row restates the fence inclusion
  list verbatim and is re-keyed in the same edit set by this seat (P4). `implement.md` (P2)
  dispatches the pass and names the fence by reference, not by list. `mochiko:qa-engineer` and
  `mochiko:devils-advocate` mount the skill without restating the list (grep clean). The skill's
  own `description:` names the ownership set, never the inclusion list — unchanged, 709 chars.
