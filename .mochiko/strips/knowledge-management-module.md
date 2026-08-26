# Strip notes — `templates/constitution-modules/knowledge-management.md`

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.91.0] Landing-ritual command list drops `plan` — the command no longer exists

- **Disposition:** superseded → the two surviving pipeline landings. The landing ritual's
  where-it-fires list named "specify/plan/implement landings where those commands run"; `/mochiko:plan`
  was retired this wave, so the list named a command that cannot run. The ritual itself is
  unchanged — the same three-part move fires at the same moments, one of which no longer exists.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D1 — `/mochiko:plan` retires and
  `/mochiko:implement` becomes the single downstream run; `DECISIONS.md` 2026-08-26 row.)
- **Content (superseded, verbatim, one site):** "at the command landing step (brainstorm close ·
  setup/amend · specify/plan/implement landings where those commands run)". Now:
  "… specify/implement landings where those commands run".
- **Kept deliberately:** the whole ritual — the one-move framing, all three numbered parts, the
  invariants, the `GLOSSARY.md` term format above it, and the "where those commands run"
  qualifier that keeps the list honest for projects not running every command. The brainstorm-close
  and setup/amend landing sites are untouched. The plan run's landing obligations did not vanish
  with the command: they fire at implement's landing, which is now the single downstream run.
- **Consumers assessed:** this is a **project-pinned constitution module** — it ships as the
  template every consuming project's knowledge-management module is written from, so a stale
  command name here propagates outward into projects that adopt it, which is why it was worth
  fixing rather than leaving as cosmetic drift. Mochiko's own pinned copy at
  `.mochiko/memory/knowledge-management.md` is a separate file and another seat's surface — it is
  NOT edited here; flagged to the wave lead for routing. `CLAUDE.md`'s landing-ritual paragraph
  cites that pinned copy, not this template.

## [v0.81.0] `ARCHITECTURE.md` doc-role re-worded to the derived index; In-flight agreement → orphan rule — product-architecture-schema D4/D10

- **Disposition:** superseded → the derived-index role and the orphan rule. `ARCHITECTURE.md`
  stays a repo-root core doc with the same read-job, but it is now a **rendered projection** of
  the architecture store (`.mochiko/product/architecture/`) that its owning skill regenerates on
  every store write — so its writer moment changes from a landing-time hand fold to a
  single-writer regeneration, and the In-flight-pointer invariant it carried is replaced by the
  store's own orphan rule.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D4 (derived index, single writer,
  index-vs-ledger disagreement is a defect) · D10 (the six-step delta lifecycle whose orphan rule
  explicitly supersedes the pinned AT-D6-C invariant); `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — three fragments):**

  ```
  | `ARCHITECTURE.md` (repo root) | the living system view — components, boundaries, data flow; decisions record *changes*, this records the *resulting system* | plan/implement landings on structural change · `mochiko:authoring-architecture` |
  ```

  ```
  - **In-flight agreement:** every `ARCHITECTURE.md` In-flight pointer targets an open
    feature and resolves; a closed feature still pointed at, or a pointer to a missing
    file, is a defect.
  ```

  ```
  - [ ] Invariants stated mechanically (… · in-flight agreement (`ARCHITECTURE.md` pointers target open features and resolve) · presence) with the vacuous-at-zero note
  ```
  *(validator-fragment line, elided at the unchanged middle; also re-worded: the `FEATURES.md`
  paragraph's "`ARCHITECTURE.md`'s capability peer" → "the architecture store's capability peer")*
- **Kept deliberately:** `ARCHITECTURE.md` **stays at the repo root** as a core KM artifact with
  its read-job intact — D4 is explicit that the top-level operating-doc reservation and the KM
  home are untouched. The admission rule (read-job · writer moment · carrier) still holds for the
  row, which is why the writer moment and carrier were re-stated rather than dropped. Every other
  invariant, the landing ritual, the never-overwrite floor, and the electives are untouched.
- **Addition riding the decision row (no strip):** the **index agreement** invariant (the derived
  index matches the store; a disagreement is fixed by re-rendering, never by editing the index) —
  D4's review fold S12 makes this the mechanism that closes the stale-index-misses-a-trip failure
  mode, and an invariant list that dropped the old pointer check without adding it would have
  left the new surface unguarded.
- **Open seam flagged to the wave landing (not ruled here):** this template ships to adopting
  projects, and the row now names a store that a KM-adopting, pipeline-less project will not
  have. The coupling is pre-existing in kind (the row already named a mochiko skill as its
  carrier) and was re-pointed, not widened; whether the KM module should carry an
  architecture-store-free degrade path is an open thread for the wave landing.
- **Consumers assessed:** the **project-pinned copy** at `.mochiko/memory/knowledge-management.md`
  carries the same invariant and is amended in the same edit set by this seat, direct by ruling
  (not as an amend offer) — the two must agree or the runtime source contradicts its template.
  `.claude/rules/mochiko/operating-docs.md` gains the derived/never-hand-edit line (this seat).
  `mochiko:grooming-operating-docs` resolves invariants from the project copy, never this
  template — unaffected by construction.

## [v0.44.0] Redesign record citation
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
`.mochiko/brainstorms/operating-docs-maintenance/record.md`
D1–D12 + R1–R7;
```
- **Kept deliberately:** the ruling date (2026-07-25) and the substantive clause it introduced — the redesign deliberately supersedes the prior four-part "no inner menu" bundle rule.

## [v0.29.0] Module rewritten whole per the operating-docs-maintenance redesign
- **Disposition:** superseded-by-redesign (ruled, not a minimalism strip) — old content
  retrievable at git history `7920ccb` and `.mochiko/archive/`
- **Content that left:** the four-part bundle (incl. the `DECISIONS.md` artifact + its
  evolution-roadmap disambiguation form) · the "no inner menu" whole-bundle rule (deliberately
  superseded by core-whole + electives, R6) · the three-carrier chain incl. the stub-backed
  CLAUDE.md-sync rows carrier (replaced by scaffolded pointers, D7) · the content-quality
  exemption (replaced by the enforced floor: boundary invariants + subtractive landing, D6/R1)
- **Provenance:** DECISIONS.md OD-D1–D12;
  `.mochiko/brainstorms/operating-docs-maintenance/record.md`
- **Consumers assessed:** `setup.md` (G5 — updated) · `brainstorm.md` (invariant source —
  retargeted to the project-pinned copy) · `specify.md` (Finalize KM landing added —
  GLOSSARY minting; format home fixed at audit: Document contracts pinned with the copy) ·
  `plan.md` (Finalize KM landing + ARCHITECTURE dispatch) · `implement.md` (Finalize KM
  landing + ARCHITECTURE dispatch) · `INTERROGATION-AGENDA.md` dimension 7 (updated) ·
  `validation-constitution` module fragment (the template's own embedded fragment rewritten
  with it) · `governance-intent-template.md` module-ruling rows (unchanged — module name and
  recorded-decline semantics survive)
