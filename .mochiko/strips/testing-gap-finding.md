# Strip notes — `skills/testing-gap-finding`

Entry formats: `strips/README.md`. Skill born at v0.79.0 (the QA gap-finding build); this file
opens with the first edit that superseded any of its shipped text.

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
