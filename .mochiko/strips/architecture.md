# Strip notes — `commands/architecture.md`

<!-- Wave context: the schema-header runtime-kernel wave (v0.100.0) — shipped schema
top-of-file header comments trimmed to runtime-essential content. Ruling for every
[v0.100.0] entry below: `.mochiko/decisions/2026-08-28-schema-header-runtime-kernel.md`
(a recorded supersession-by-amendment of command-content-schema D14) + `DECISIONS.md`
2026-08-28 row. Pre-edit verbatim text: `git show e44b33d:plugins/mochiko/schemas/<file>`. -->

## [v0.100.0] `schemas/architecture.yaml` header — full-grammar comment superseded by the runtime kernel
- **Disposition:** superseded → the runtime-kernel header; the grammar is single-sourced in
  `plugins/mochiko/commands/architecture.md` "Rules — load the schema first" (runtime) and
  `.claude/rules/mochiko/primitive-edits.md` criterion 11 + the `command-content-schema` /
  `command-schema-ontology` records (edit-time)
- **Tier failed:** n/a — supersession by ruling (`2026-08-28-schema-header-runtime-kernel.md` R1)
- **Content:** the ~68-line canonical D14 header, faithfully compressed: provenance paragraph
  (D1/D7 citations, GI-020 note, narrative-stays-in-.md note) · full grammar block (section id ·
  id minting/freeze/tombstones D11 · labels D8 · class incl. M3 audit note · nine-kind vocabulary ·
  when semantics · enforces · extends resolution · text/${var} D5 · provenance sidecar D16 ·
  pointer) · conditions/moments block explanations · referential-closure paragraph (D15, deixis,
  legal self-reference variance) · rule-grain line (D12) · advisory-checker line (D13). Verbatim:
  `git show e44b33d:plugins/mochiko/schemas/architecture.yaml` (lines 1–68).
- **Kept deliberately:** the runtime kernel — identity + read-at-fire/interpret-live note pointing
  at the `.md` Rules block, `class:` value semantics, and the `conditions:` resolution-point
  vocabulary (the two semantics no `.md` restates). The `advisory` gloss is restated on the
  runtime axis ("guidance, not binding"), not carried verbatim — the ceremony-axis meaning
  ("may change without supersession ceremony") stays homed at
  `.claude/rules/mochiko/primitive-edits.md` criterion 5 and content-schema D6.
- **Consumers assessed:** the command's own `.md` (self-carries the reading grammar; unchanged) ·
  `.claude/skills/converting-command-to-schema/SKILL.md` (updated this wave to mint the kernel) ·
  `scripts/check-command-schema.py` (does not read header comments; post-edit `--all` PASS,
  0 findings).


Entry formats: `strips/README.md`. Command born at v0.81.0 (the product-architecture-schema
Stage-1 wave, D1 — the standing architecture desk); this file opens with the first edit that
superseded any of its shipped text.

<!-- Wave context: the near-dup convergence wave (v0.99.0) — three architecture rules
converge to `extends: common.<slug>` stubs under strongest-wording-wins. Ruling for every
[v0.99.0] entry below: `.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1–R6 +
wave flags → `DECISIONS.md` 2026-08-28 row. Every ID survives as a stub (R3 — no
tombstones); `class`, `kind`, `when:`, and `enforces:` stay local per ontology C3. Floor
entries are supersessions per R4. `arch.model-tiering` deliberately KEEPS its local text
(wave flag B — its drift-probe routing clause is arch-specific, not illustrative); the
adjudication is recorded in scripts/similar-rules-allowlist.yaml. -->

## [v0.99.0] `arch.tools-referenced-never-restated` — text superseded by the fuller block wording

- **Disposition:** superseded → `extends: common.tools-referenced-never-restated`; resolved
  text UPGRADES to implement's wording — "its home" becomes "its home skill or template".
- **Tier failed:** n/a — supersession by ruling (near-dup ADR R1/R2).
- **Content:** verbatim —

  ```
          Every tool binding is referenced, never restated — its procedure lives in its
          home.
  ```

- **Kept deliberately:** the whole obligation survives; `class: must` local; labels
  inherited, same value.
- **Consumers assessed:** none — per-command rule.

## [v0.99.0] `arch.rulings-plain-text` — text superseded by the widened `common.acceptance-plain-text`

- **Disposition:** superseded → `extends: common.acceptance-plain-text` (the block widened
  at this wave to "User rulings and acceptance are plain blocking text, never a timed
  prompt", so the desk's "rulings" vocabulary is carried, not lost). The stub keeps its
  minted ID `arch.rulings-plain-text` (R3 — IDs never re-slug).
- **Tier failed:** n/a — supersession by ruling (near-dup ADR R1/R2; wave move 7).
- **Content:** verbatim —

  ```
          User rulings are plain blocking text, never a timed prompt.
  ```

- **Kept deliberately:** "rulings" survives in the widened block text; `class: must` local;
  labels inherited, same value.
- **Consumers assessed:** none — per-command rule.

## [v0.99.0] `arch.transport-floor` (floor) — own text superseded by the widened block it seeded

- **Disposition:** superseded → `extends: common.transport-floor`. Arch's enumeration
  ("Trigger test, floor legs, composition-safe shapes, and disclosure") was the strongest
  wording and moved INTO the block at this wave (flag A) — the desk's "The visit gains a
  floor" opener drops for the block's unit-neutral statement.
- **Tier failed:** n/a — supersession by ruling (near-dup ADR R1/R2/R4 — floor; flag A).
- **Content:** verbatim —

  ```
          The visit gains a floor on its composition and messaging: a split trigger —
          message legs on any multi-seat messaging, topology legs on shared writes —
          non-waivable once triggered. Trigger test, floor legs, composition-safe shapes,
          and disclosure: mochiko:patterns-transport-floor, referenced never restated.
  ```

- **Kept deliberately:** the split trigger, both legs, non-waivability, and the full
  enumeration all survive in the widened block; `when: {seats: multi}` and its C4 comment
  stay local on the stub.
- **Consumers assessed:** none — per-command rule.

<!-- Wave context: the command-schema ontology wave (v0.98.0) — the D1–D8 grammar amendment
over the `.md` + schema pair: a closed `kind:` set with `constraint` as the omitted default
(D1), a per-schema `conditions:` block plus a rule-level `when:` guard with the condition
single-homed out of `text` (D3), a per-schema `moments:` block (D4), `enforces:` on every
`kind: fail` node (D6), and `extends: common.<slug>` binding the shared blocks in
`plugins/mochiko/schemas/common.yaml` (D8 — a narrow supersession of command-content-schema
D3). Ruling for every [v0.98.0] entry below:
`.mochiko/brainstorms/command-schema-ontology/record.md` D1–D11 as amended → `DECISIONS.md`
2026-08-27 command-schema-ontology row. Clause inventory (the audit referent): that session's
`conversion-inventory.md`. Pure additions ride the decision row and are not entered here —
the `kind:` lines, the `conditions:` and `moments:` blocks, the C4 floor-semantics comments,
and the `.md` Rules-block grammar breath. -->

## [v0.98.0] The schema header's D6/D14 grammar comment — superseded by the canonical D1–D8 header (build item 1)

- **Disposition:** superseded → the canonical header comment block at the top of
  `plugins/mochiko/schemas/architecture.yaml`, per the inventory's section G, which states
  the same grammar plus the `kind:` / `when:` / `conditions:` / `moments:` / `enforces:` /
  `extends:` fields.
- **Tier failed:** n/a — supersession by ruling (record D1–D8, build-surface item 1;
  `DECISIONS.md` 2026-08-27).
- **Content:** the superseded lines as shipped at v0.97.0, verbatim —

  ```
  # Command content schema for /mochiko:architecture — SOURCE OF TRUTH for the desk's
  # rule-shaped content (command-content-schema D1/D7 + this command's D10 rollout ruling,
  # DECISIONS.md 2026-08-26). The command .md instructs a raw, full Read of this file at
  # command fire; the model interprets it live — no build step, no binary on the read path
  # (GI-020). Narrative (Identity & Mission, Adaptive Goal Protocol prose) stays in
  # plugins/mochiko/commands/architecture.md.
  # Grammar (D6, as amended by D14): sections: list, each {id, title, intent, rules};
  # rule blocks {id, labels, class, text, pointer?} nest under their section.
  #   section id  <cmd>.sec.<slug> — minted once like rule IDs (D11); title verbatim from
  #           the command group it carries; intent one line, navigation only — sections
  #           never grow a second prose surface (narrative stays in the .md).
  #           the losers under a top-level tombstones: key. First mint — no tombstones yet.
  #   class   floor = non-waivable, must-survive under the charter audit (M3) ·
  #   arch.sec.* · class: values · labels · pointer: skills · file paths).
  ```

- **Kept deliberately:** every surviving D6/D11/D12/D14/D15/D16 grammar line is carried into
  the new block verbatim or near-verbatim — the section-ID mint rule, the ID-frozen /
  reword-keeps-its-ID / split / merge rules, the `arch.fail.*` segment line, the label
  registry pointer, the three-value `class` vocabulary, the `${var}` substitution line, the
  provenance-sidecar line, the `pointer:` line, the D15 referential-closure paragraph with
  this desk's own legal self-reference set ("this schema", "the desk", "the visit"), the D12
  grain line, and the D13 advisory-checker line. Two changes inside the kept material are
  deliberate and recorded here rather than silently: `charter audit` → **`pair audit`**
  (inventory J-13; the corpus in fact held three stragglers, not the two J-13 named —
  `architecture.yaml`, `feature.yaml`, `implement.yaml`), and the dropped "First mint — no
  tombstones yet" clause, which the canonical header does not carry for any of the six. The
  section-ID line's placeholder is also resolved from `<cmd>.sec.<slug>` to the concrete
  `arch.sec.<slug>`, matching how the other five schemas already wrote it.
- **Consumers assessed:** `scripts/check-command-schema.py` parses the YAML, never this
  comment (it gains its own D1–D8 checks in the same wave); no other primitive reads it.

## [v0.98.0] `arch.dm-author-baseline`, `arch.dm-km-landing`, `arch.transport-floor` — activation guards single-homed into `when:` (D3)

- **Disposition:** superseded → the `when:` field on each rule, with the rule's `text`
  reworded to carry the obligation alone; for `arch.dm-author-baseline` the guard's
  two-form **definition** relocates to the `store_ruled_content` note in this schema's
  `conditions:` block, which is where single-homing puts it. All three keep their IDs (a
  reword, D11); all three are `class: floor`.
- **Tier failed:** n/a — supersession by ruling (record D3, single-homing, as amended at C4
  for floors; `DECISIONS.md` 2026-08-27. Clause inventory: `conversion-inventory.md`
  section B.4, the three architecture MOVEs).
- **Content:** the guard-carrying texts as shipped at v0.97.0, verbatim —

  ```
  arch.dm-author-baseline:
          Author the baseline wherever the store carries no ruled content — scaffold-only
          (a `spine.md` holding just its `Scope:` header) or absent: greenfield elicits
          it; brownfield reconstructs and confirms it per
          arch.tools-brownfield-reconstruction — nothing absorbed is ever silently
          discarded.

  arch.dm-km-landing:
          Execute the KM landing for desk-side writes where knowledge-management exists.

  arch.transport-floor:
          A visit that composes more than one seat gains a floor on its composition and
          messaging: a split trigger — message legs on any multi-seat messaging, topology
          legs on shared writes — non-waivable once triggered. Trigger test, floor legs,
          composition-safe shapes, and disclosure: mochiko:patterns-transport-floor,
          referenced never restated.
  ```

- **Kept deliberately:** every obligation survives whole, and all three keep `class: floor`
  with the C4 comment carried verbatim — a floor is always read and always delivered, its
  `when:` gating when the obligation applies, never whether it is delivered.
  `arch.dm-author-baseline` keeps both mode arms in prose (greenfield elicits, brownfield
  reconstructs-and-confirms), keeps its citation of `arch.tools-brownfield-reconstruction`,
  and keeps the nothing-absorbed-is-ever-silently-discarded clause; the scaffold-only /
  absent definition it used to carry inline survives verbatim in the
  `store_ruled_content` note. `arch.dm-km-landing` keeps its `kind: duty` and now names the
  knowledge-management file by path, closing a D15 gap in the shipped text, which said
  "knowledge-management" with no path. `arch.transport-floor` keeps both floor legs, the
  non-waivable-once-triggered clause, the four-part pointer tail, and its `pointer:`.
- **Consumers assessed:** the desk's `.md` half names none of the three by ID.
  `arch.tools-brownfield-reconstruction` is cited by `arch.dm-author-baseline` and survives
  unchanged in this schema, so the citation still resolves.
  `mochiko:patterns-transport-floor` holds the transport procedure and is untouched.

## [v0.98.0] `arch.register` and `arch.no-git-mutations` local texts — superseded by `extends: common.*` (D8)

- **Disposition:** superseded → `extends: common.register` and
  `extends: common.no-git-mutations`, which inherit the identical text from
  `plugins/mochiko/schemas/common.yaml`; both stubs keep their `arch.*` IDs as the citable
  IDs.
- **Tier failed:** n/a — supersession by ruling (record D8 as amended by C2 and C3 — a
  narrow supersession-by-ruling of command-content-schema D3, legal only for text that is an
  exact duplicate across three or more command schemas; `DECISIONS.md` 2026-08-27. Both
  blocks cleared the bar at five members each — inventory section F.0/F.1).
- **Content:** the local rule bodies as shipped at v0.97.0, verbatim —

  ```yaml
        - id: arch.register
          labels: [reporting, binding]
          class: must
          text: >-
            User-facing prose follows templates/output-style.md.

        - id: arch.no-git-mutations
          labels: [user-gate]
          class: must
          text: >-
            Suggest commits; never run git mutations, never push.
  ```

- **Kept deliberately:** the resolved text of both rules is byte-identical to what shipped —
  `extends:` was admitted only on exact duplicates, so nothing was reworded to make either
  bindable. Per the C3 precedence clause every absence-meaningful field stays local and is
  re-declared on the stub: `class: must` on both, `kind: binding` on `arch.register`.
  `arch.register`'s `labels: [reporting, binding]` are **re-declared locally** because the
  common block carries `[reporting]` alone, and inheriting would have silently narrowed a
  shipped rule's labels (inventory F.5). `arch.no-git-mutations` inherits `[user-gate]`
  unchanged, which is what it shipped with.
- **Consumers assessed:** `plugins/mochiko/schemas/common.yaml` is the new home and ships in
  the same wave under full ceremony; `plugins/mochiko/commands/architecture.md` gains the
  `extends:` reading instruction and the obligated raw Read of `common.yaml` in its first
  action; `scripts/check-command-schema.py` gains `extends:` target resolution and the C3
  local-`class:` assert in the same wave. This desk's two other candidate boilerplate rules
  stay per-command by measurement: `arch.rulings-plain-text` is differently named and
  differently worded from `common.acceptance-plain-text`, and `arch.transport-floor` is not
  an exact duplicate of `common.transport-floor` even after guard-stripping (inventory F.2).

## [v0.98.0] The `fail-condition` label as the Not-done selector — re-keyed to `kind: fail` (build item 4)

- **Disposition:** superseded → `kind: fail` on `arch.fail.no-verdict`, which is now the
  operative selector for the Not-done set across both halves of the pair: the label leaves
  the rule, the `arch.sec.fail-conditions` intent line names the new key, and the `.md`
  Not-done line counts `kind: fail`.
- **Tier failed:** n/a — supersession by ruling (record D1 and build-surface item 4;
  `DECISIONS.md` 2026-08-27. Line-by-line: inventory section H).
- **Content:** the three superseded sites as shipped at v0.97.0, verbatim —

  ```
  plugins/mochiko/schemas/architecture.yaml, arch.fail.no-verdict:
          labels: [fail-condition, reporting]

  plugins/mochiko/schemas/architecture.yaml, arch.sec.fail-conditions intent:
        The fail-condition set — any one standing fails the visit; the .md Not-done line hard-codes this set's count.

  plugins/mochiko/commands/architecture.md, Adaptive Goal Protocol step 3:
  3. **Not done — default FAIL:** the 1 rule labeled `fail-condition` in
     `plugins/mochiko/schemas/architecture.yaml` (section `arch.sec.fail-conditions`) — any one
     standing fails the visit. If the schema's `fail-condition` count is not 1, the pair is out of
     sync: halt and surface it before closing.
  ```

- **Kept deliberately:** the whole count-pin mechanism survives, re-keyed. The count stays
  **1**, the out-of-sync halt clause is carried verbatim, the section pointer and the
  any-one-standing sentence are unchanged, and `arch.fail.no-verdict` keeps its ID, its
  `class: floor`, its `reporting` label, and its text. The rule additionally gains
  `enforces: [arch.dm-converge-goal, arch.dm-close-verdict]` (D6) — a pure addition riding
  the decision row.
- **Consumers assessed:** `plugins/mochiko/schemas/command-labels.yaml` retires the label in
  the same wave (its own strip entry at `.mochiko/strips/command-labels.md`) ·
  `.claude/rules/mochiko/primitive-edits.md` criterion 3 re-keys to `kind: fail` in the same
  wave · `scripts/check-command-schema.py` re-keys its count assert and keeps the
  bidirectional `arch.fail.*`-segment ↔ `kind: fail` cross-check (I4) · the other five
  command pairs re-key on the same ruling.

<!-- Wave context: the command-`.md`-scaffold standardization wave (v0.97.0) — one canonical
`.md` scaffold for all six pair commands (D1/D2: Identity & Mission · Rules block · Adaptive
Goal Protocol with Entry / Goal / count-pinned Not-done last) and the schema six-set
unification (D3/D4/D5: `<prefix>.sec.roles` · `reserved` · `tools` · `ways-of-working` ·
`boundaries` · `fail-conditions`; rule IDs and texts carried unchanged — pure relocation, the
D14 precedent). Ruling for every [v0.97.0] entry below:
`.mochiko/brainstorms/command-md-scaffold-standardization/record.md` D1–D7 as review-amended
→ `DECISIONS.md` 2026-08-27 command-md-scaffold-standardization row. -->

## [v0.97.0] The Rules-block five-section enumeration — superseded by the unified six-set (D3/D4)

- **Disposition:** superseded → the six-section enumeration in
  `plugins/mochiko/commands/architecture.md`, `## Rules — load the schema first`; the
  reserved-rulings gloss re-homed onto the newly minted `arch.sec.reserved`.
- **Tier failed:** n/a — supersession by ruling (`command-md-scaffold-standardization` D3 +
  D4; `DECISIONS.md` 2026-08-27 row).
- **Content:** verbatim — "desk's binding rules, nested in five sections, each addressable by
  its section ID: `arch.sec.roles` (the Delivery Manager's always-happens floor, seat wiring,
  and the user's reserved rulings) · `arch.sec.tools` (tool bindings) ·
  `arch.sec.ways-of-working` · `arch.sec.boundaries` (the non-waivable floor) ·
  `arch.sec.fail-conditions` (the Not-done set)."
- **Kept deliberately:** the whole boilerplate around the enumeration — anchor phrase
  ("before health is surfaced, before any seat is spawned"), the raw-Read first-class clause,
  the `${var}` / `pointer:` / labels interpretation clause, the not-open-until-read close —
  verbatim; every surviving gloss word carried across, the roles gloss split across `roles`
  and `reserved` without rewording.
- **Consumers assessed:** `plugins/mochiko/schemas/architecture.yaml` unified the same wave;
  `scripts/check-command-schema.py` D14 section-count guard re-keyed the same wave; no other
  primitive enumerates this command's sections.

## [v0.97.0] The desk protocol's own three step labels — superseded by the canonical Entry / Goal / Not-done steps (D1/D2)

- **Disposition:** superseded → `## Adaptive Goal Protocol` steps 1–3 in
  `plugins/mochiko/commands/architecture.md`; every sentence relocated, the step labels
  folded to inline bold inside the canonical steps.
- **Tier failed:** n/a — supersession by ruling (`command-md-scaffold-standardization` D1 as
  narrowed at review C2, D2; `DECISIONS.md` 2026-08-27 row).
- **Content:** verbatim, the labels and the free-standing `$ARGUMENTS` line that left their
  positions — "1. **Health first, then the ask.**" · "2. **Converge to a goal and its done
  condition.**" · "3. **Run to the done condition.**" · "`$ARGUMENTS` = the incoming
  architecture demand or store query; empty → surface health, then ask what the visit is
  for."
- **Kept deliberately:** the per-visit done-condition contract, explicitly preserved by D1's
  C2 narrowing — step 2 reads "**Goal — the done condition, converged per visit**" and
  carries the micro-brainstorm / one-line-visit-goal / crisp-ask / baseline-walk prose and
  the run-to-the-done-condition close verbatim; the preamble "Every visit has a goal; a visit
  is never goal-less."; the whole health-view enumeration relocated intact under Entry (its
  own text gates *before* taking the request); `## Identity & Mission` untouched.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` criteria re-keyed the
  same wave (D6-R2); `README.md:5` + `ARCHITECTURE.md` charter-form sites ripple the same
  wave (build item 6).

## [v0.97.0] "the 1 rules labeled" — the Not-done line's number-agreement defect (D6-R4)

- **Disposition:** superseded → "the 1 rule labeled `fail-condition`" in the canonical
  Not-done line.
- **Tier failed:** n/a — supersession by ruling (`command-md-scaffold-standardization` D6-R4
  hygiene rider; `DECISIONS.md` 2026-08-27 row).
- **Content:** verbatim — "**Not done — default FAIL:** the 1 rules labeled `fail-condition`
  in".
- **Kept deliberately:** the count-pin (1) and the out-of-sync halt clause verbatim; only the
  agreement defect died.
- **Consumers assessed:** `scripts/check-command-schema.py` `NOT_DONE_RE` hard-coded plural
  `rules` and no longer matched — re-keyed to `rules?` with both-way negative tests in the
  same wave's checker rework.

## [v0.97.0] `arch.sec.roles` intent line and the reserved-rulings home — split into a first-class `arch.sec.reserved` (D4)

- **Disposition:** superseded → `plugins/mochiko/schemas/architecture.yaml` gains a minted
  section node `arch.sec.reserved` (title "Reserved to the user — never the desk's"; intent
  "Decisions held by the user: gates, rulings, and escalations no seat may take."), carrying
  `arch.user-reserved-rulings` relocated whole — ID, text, labels, class byte-identical. The
  `arch.sec.roles` intent line is reworded to drop the clause the extraction made false.
- **Tier failed:** n/a — supersession by ruling (record D4; `DECISIONS.md` 2026-08-27 row).
- **Content:** verbatim, the `arch.sec.roles` intent line that left — "The Delivery Manager's
  always-happens floor, the other seats' wiring, and the rulings reserved to the user."
- **Kept deliberately:** every rule text, ID, label, class and pointer in the schema untouched
  — `arch.user-reserved-rulings` moves node-to-node with nothing rewritten. The roles intent's
  surviving clauses (the DM floor, the other seats' wiring) carry over verbatim into the
  reworded line; only the reserved clause leaves, and it leaves to the new section's own
  `intent:`, not to nothing. `arch.sec.roles` itself is NOT tombstoned — it survives with 13
  of its 14 rules.
- **Consumers assessed:** `.mochiko/provenance.yaml` is keyed by rule ID and carries no
  `sec.` keys (verified — 107 anchors, zero section keys), so no anchor re-keys; the
  `arch.user-reserved-rulings` anchor follows its rule by ID. The pair-form audit keys on the
  `fail-condition` label set (1, unchanged) and `arch.*` ID continuity (no ID vanished — a
  mint, not a retirement). The checker's D14 section-count guard FAILed this pair mid-wave —
  a transient state after the same wave's `.md` rewrite landed "nested in six sections" and
  before this schema edit (at HEAD the `.md` said five); the mint takes the schema to 6 and
  clears that finding. `architecture.md`'s section enumeration is re-keyed by
  the same wave's `.md` rewrite.

<!-- Wave context: the D16 provenance-sidecar amendment (v0.96.0) — schemas carry runtime
content only; decision anchors move to `.mochiko/provenance.yaml`, keyed by rule ID. Ruling:
record D16 (post-rollout amendment, user-ruled 2026-08-26, incl. the repo-side-home
refinement) → `DECISIONS.md` 2026-08-26 command-content-schema row. -->

## [v0.96.0] `ruling:` fields and the grammar-header ruling description — extracted to the provenance sidecar (D16)

- **Disposition:** relocated → `.mochiko/provenance.yaml` — every `ruling:` field in
  `plugins/mochiko/schemas/architecture.yaml` carried verbatim as an `anchors:` entry keyed by its
  rule's mint-once ID (checker-verified: the pair's run reports `anchors 13`, each entry
  format-checked, non-dangling, and resolved against a live `DECISIONS.md` row); the
  grammar-header ruling description superseded by the provenance note.
- **Tier failed:** n/a — supersession by ruling (record D16; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim, the header lines that left —
  '#   ruling  "YYYY-MM-DD <session-slug> [D#]" — protected content, anchored to a live' /
  '#           DECISIONS.md row. Leaves only by recorded supersession-by-ruling.' — plus
  `ruling?` in the grammar tuple and " · ruling: anchors" in the D15 namespace list. The
  per-rule anchor VALUES left the file unchanged (relocation, not deletion — the sidecar is
  the verbatim home).
- **Kept deliberately:** every rule's text, class, labels, pointer untouched; protection
  semantics unchanged — an anchored rule still leaves only by recorded
  supersession-by-ruling, the anchor now joined by ID from the sidecar.
- **Consumers assessed:** checker reworked the same wave (inline `ruling:` now a finding,
  dangling sidecar keys a finding, both negative-tested; sidecar absent degrades to a
  warning for plugin-standalone checkouts); the conversion skill's step 7 re-pointed; the
  pair-form audit criteria in `.claude/rules/mochiko/primitive-edits.md` gained the sidecar
  home. The sidecar is repo-side and never shipped — plugin install unchanged (GI-020).

<!-- Wave context: the command-content-schema D10 rollout wave (v0.95.0) —
`commands/architecture.md` splits into a narrative `.md` (Identity & Mission · the obligated
schema read · Adaptive Goal Protocol) and `plugins/mochiko/schemas/architecture.yaml`
(mint-once rules at D12 grain, prefix `arch`; sections mirror the command's own groups per
D14; referential closure per D15; labels from `command-labels.yaml`). Source text = the
shipped v0.94.0 command, frozen as the step-0 referent
`.mochiko/brainstorms/command-content-schema/referents/architecture-shipped-v0.94.0.md` —
structure-only extraction, no simplification pass. Ruling for every [v0.95.0] entry below:
`.mochiko/brainstorms/command-content-schema/record.md` (D2 · D6 · D7 · D12 · D14 · D15 +
the Session-trail "D10 rollout ruling 2026-08-26" naming `architecture.md`, prefix `arch`) →
`DECISIONS.md` 2026-08-26 command-content-schema row. Every Content field quotes the SHIPPED
v0.94.0 text — what actually left the file (the GI-006 referent); the schema's rewording
lives at the named new homes, never restated here. -->

## [v0.95.0] `## Roles & Responsibilities` — the whole section moves to the schema (D2/D7/D10 rollout)

- **Disposition:** superseded → `plugins/mochiko/schemas/architecture.yaml` section
  `arch.sec.roles`: the per-visit latitude as `arch.staffing-latitude`; the Delivery
  Manager's always-happens floor as the nine `class: floor` rules `arch.dm-health-first` ·
  `arch.dm-converge-goal` · `arch.dm-author-baseline` · `arch.dm-shelf-walk` ·
  `arch.dm-drift-dispatch` · `arch.dm-route-triggers` · `arch.dm-store-integrity-close` ·
  `arch.dm-km-landing` · `arch.dm-close-verdict`; the other seats as
  `arch.seat-architect-producer` · `arch.seat-tech-lead-grader` ·
  `arch.seat-drift-probe-empirical`; the user's reserved set as
  `arch.user-reserved-rulings` — text per the referent at D12 grain.
- **Tier failed:** n/a — supersession by ruling (record D2 — rules move to the schema,
  narrative stays; D7 — R&R seat wiring + reserved-to-user items are rule-shaped scope; the
  D10 rollout ruling for this command; `DECISIONS.md` 2026-08-26 row).
- **Content:** the whole shipped section, verbatim:

```
## Roles & Responsibilities

There is **no Bindings section**. The bare minimum that must always happen is carried here as
the Delivery Manager's owned responsibilities; everything beyond it is your per-visit judgment —
below the sound-loop floor (`mochiko:patterns-sound-loop`, Boundaries), how you staff, sequence,
and run the visit is yours to shape.

**You, the Delivery Manager — the always-happens floor:**

- Surface health before the ask.
- Converge every visit to a one-line goal and its done condition.
- Author the baseline wherever the store carries no ruled content — **scaffold-only** (a
  `spine.md` holding just its `Scope:` header) or absent: greenfield **elicits** it; brownfield
  **reconstructs and confirms** it (Tools) — nothing absorbed is ever silently discarded.
- Walk shelf rows under the breadth invariant, ordered by retrofit cost, dealt
  recommend-then-arbitrate (Boundaries).
- Dispatch the scoped drift probe and take each finding to a user disposition.
- Route every fired upgrade trigger to `/mochiko:feature`'s growth door — flagged in the health
  view until it is routed or the user rules it closed.
- Keep store integrity intact at close — the derived index regenerated, orphans flagged,
  statuses agreeing across index and ledger.
- Execute the KM landing for desk-side writes where knowledge-management exists.
- Close the visit with a verdict against its done condition.

**Other seats:**

- **Principal-architect** — the producing seat: baseline authoring, shelf-walk stance batches,
  amendments, and delta authoring. Recommends with reasons; never rules. Plans first and works
  only on a plan you approved.
- **Tech-lead** — the independent grader of the architect's **judgment** writes, before the
  user's ratification. Status flips and orphan cleanup are transcription and ride the landing
  audit instead.
- **Drift-probe seat** — an empirical read of the codebase grading the store's `As-built:`
  claims. Never the seat that wrote the claim; evidence, never memory.
- **The user** — every row stance, the baseline confirmation, every amendment, each drift
  finding's disposition, the shelf-scope override, the D13 mint at the feature desk, and the
  governance-ledger waiver a true floor drop needs.
```

- **Kept deliberately:** nothing of the section remains in the `.md` — the schema carries it
  whole; the `.md`'s Rules section names `arch.sec.roles` as "the Delivery Manager's
  always-happens floor, seat wiring, and the user's reserved rulings", and the obligated raw
  Read delivers it. Four D15-required rewordings inside the move, IDs unaffected (first
  mint): the document-shape remark "There is **no Bindings section**." dies without
  relocation — the schema's existence states it, and D15 classes such remarks as deixis (the
  checker's own marker list names the phrase); the `(Tools)` / `(Boundaries)` cross-section
  deixis becomes addressable rule-ID references (`arch.tools-brownfield-reconstruction` ·
  `arch.breadth-invariant` · `arch.recommend-then-arbitrate` · `arch.sound-loop-floor` — the
  latter resolving the intro's "(`mochiko:patterns-sound-loop`, Boundaries)" pointer inside
  `arch.staffing-latitude`, whose bound also renders widened, "below the sound-loop floor" →
  "Beyond this schema's `class: floor` rules", the sound-loop floor named as one of them);
  "the D13 mint at the feature
  desk" — a bare decision-number reference — becomes "the capability mint at
  `/mochiko:feature`'s growth door (`arch.dispatch-feature-growth-door`)", the D13 provenance
  carried by the rule's `ruling:` anchor (`2026-08-19 product-architecture-schema D13`). The
  always-happens list keeps its floor force as `class: floor` on all nine DM rules.
- **Consumers assessed:** the pair-form audit criteria in
  `.claude/rules/mochiko/primitive-edits.md` were re-keyed lead-side in the same wave (D10
  five-command rollout block); no skill or template addresses this command's section
  headings; `/mochiko:feature`'s symmetric charter carries its own parallel text and is
  untouched by this move.

## [v0.95.0] `## Tools` — the whole section moves to the schema (D2/D7/D10 rollout)

- **Disposition:** superseded → the tool-binding rules in
  `plugins/mochiko/schemas/architecture.yaml` section `arch.sec.tools`
  (`arch.tools-referenced-never-restated` through `arch.register`), text per the referent at
  D12 grain; skill-owned procedures ride as `pointer:` rules
  (`mochiko:authoring-architecture-store` · `mochiko:patterns-architecture-shelves` ·
  `mochiko:patterns-system-design`). Two D12 grain splits inside the move: the
  shelves bullet yields `arch.tools-shelves-skill` + `arch.shelf-scope-source` (the
  shelf-scope override is independently citable — it sits in the user's reserved set); the
  dispatch-targets bullet yields `arch.dispatch-feature-growth-door` ·
  `arch.dispatch-setup-waiver` · `arch.dispatch-implement-delivery` (the third clause was
  already independently superseded once, this file's [v0.91.0] entry — proof of independent
  citability; its `ruling:` anchor is `2026-08-26 plan-stage-utility`).
- **Tier failed:** n/a — supersession by ruling (record D2/D7 — Tools bindings are rule-shaped
  scope; the D10 rollout ruling for this command; `DECISIONS.md` 2026-08-26 row).
- **Content:** the whole shipped section, verbatim:

```
## Tools

Each tool below is referenced, never restated — its procedure lives in its home.

- **The store** — `.mochiko/product/architecture/`: `spine.md` (the topology deep view —
  containers, boundaries, communication styles) and the `concerns.md` ledger of `AX-XXX` rows,
  a row graduating to its own file at `.mochiko/product/architecture/concerns/AX-XXX-<slug>.md`
  only when it carries real depth — extend beats mint. Repo-root `ARCHITECTURE.md`
  is the store's **derived index** — spine thumbnail, AX summary table, health view — and is a
  rendered projection, never a second store.
- **`mochiko:authoring-architecture-store`** — the store's owner: AX and spine grammar, the
  element lifecycle (`ruled` → `in-flight (FEAT-XXX)` / `modifying (FEAT-XXX)` /
  `removing (FEAT-XXX)` → `built`), the stance vocabulary (`decided` · `not-now` · `n-a` ·
  `open`), row graduation, the fold at landings, the health view, and the index regeneration.
- **`mochiko:patterns-architecture-shelves`** — the opinion carrier: the per-surface shelves as
  data, their suggested defaults and upgrade-trigger patterns, and the recommend-then-arbitrate
  deal. **Shelf scope is read from the `Scope:` line in `spine.md`'s header** — declared there
  by `/mochiko:setup` and **overridable here** by an ordinary store write to that line, the
  user's ruling like any other. The store carries the scope; a full-stack or monorepo product
  composes the shelves its scope names.
- **`mochiko:patterns-system-design`** — altitude and diagram craft for the spine and for the
  deltas features draft against it.
- **Store schema** — the shape is `plugins/mochiko/schemas/architecture-store.yaml`, the shelf
  data `plugins/mochiko/schemas/architecture-shelf-backend.yaml` (rendered by `mochiko-cli`, or
  Read raw when the binary is absent — the shipped schema is the first-class source of truth).
  A small required core; the schema constrains the skeleton, never the voice.
- **Brownfield reconstruction** — the first visit to an existing repo derives the store from
  what exists — repo `ARCHITECTURE.md` prose, any per-feature `architecture.md` files,
  `nfrs.md`, structural `D-XXX` rows — presents it for confirmation, then archives the absorbed
  sources to `.mochiko/archive/product-baselines/<date>/`. The derivation inherits the
  setup-bootstrap `Assumed` caveat and its partial-baseline poisoning risk: say so when you
  present it.
- **Scoped drift probe** — rows touched since the last desk visit plus a sample of the
  retrofit-expensive rows; never all rows every visit. Findings land in the touched row's
  `Drift:` field and take a user disposition here.
- **Dispatch targets** — `/mochiko:feature` for a fired upgrade trigger (it arrives at the
  growth door as a candidate under the capability-write test; extend-beats-mint applies and the
  user rules the mint — the architecture lens proposes, the map machinery disposes) ·
  `/mochiko:setup` for a governance-ledger waiver when a floor-asserted obligation must truly
  drop (Boundaries) · `/mochiko:implement` owns all delivery; the desk runs none of it.
- **Register** — user-facing prose per `templates/output-style.md`.
```

- **Kept deliberately:** nothing of the section remains in the `.md`. D15 rewordings inside
  the move, IDs unaffected (first mint): "Each tool below" loses its document-shape "below"
  (`arch.tools-referenced-never-restated` names every binding); "overridable here" and "a
  user disposition here" become "at the desk" (legal self-reference); the waiver target's
  "(Boundaries)" deixis becomes the addressable `arch.floor-precedence`. Paths `Scope:`,
  store, schema, and archive strings survive verbatim; the store home and schema paths ride
  the `vars:` block (`${store_dir}` · `${store_schema}` · `${shelf_schema}`) per D5.
- **Consumers assessed:** the [v0.91.0] entry below records the prior supersession of the
  dispatch-targets line this section carried; its trace now continues at
  `arch.dispatch-implement-delivery` (same `plan-stage-utility` ruling, machine-anchored).
  No other primitive addresses this section's headings.

## [v0.95.0] `## Ways of Working` — the whole section moves to the schema (D2/D7/D10 rollout)

- **Disposition:** superseded → the rules in `plugins/mochiko/schemas/architecture.yaml`
  section `arch.sec.ways-of-working` (`arch.proactive-report-first` ·
  `arch.reference-never-restate` · `arch.author-grader-separation` ·
  `arch.recommend-then-arbitrate` · `arch.model-tiering` · `arch.single-writer-store` ·
  `arch.no-git-mutations` · `arch.rulings-plain-text`), text per the referent at D12 grain;
  the "Commits and rulings" bullet splits into its two independently-citable obligations
  (`arch.no-git-mutations` + `arch.rulings-plain-text`).
- **Tier failed:** n/a — supersession by ruling (record D2/D7 — Ways of Working is rule-shaped
  scope; the D10 rollout ruling for this command; `DECISIONS.md` 2026-08-26 row).
- **Content:** the whole shipped section, verbatim:

```
## Ways of Working

- **Proactive report first** — health before the ask, every visit.
- **Reference, never restate** — the store grammar lives in its skill, the opinions in the
  shelf data, the delivery bounds in the pipeline commands; the desk points at them and adds
  nothing.
- **Author ≠ grader** — wherever a seat produces (baseline, stance batch, amendment, delta), no
  output is cleared by its author; a producing seat plans first and works only on a plan you
  approved.
- **Recommend, then arbitrate** — a shelf row is dealt with its suggested default and the
  reasoning behind it, and the user forms the stance. A default is never applied by silence.
- **Model tiering** — exploration and fact-finding dispatches ride the class-keyed tiering
  floor: locate/enumerate reads go to a native `Explore` subagent spawned `model: haiku`,
  interpretive or absence-driven reads stay session tier — the drift probe **grades claims
  against code and is interpretive**, so it stays session tier and sends only its
  locate/enumerate legs down. Every seat brief carries the routing rule. Class key, dispatch
  ladder, and brief obligation: `mochiko:patterns-model-tiering`, referenced never restated.
- **Single writer on the store** — one seat holds the pen per visit; the derived index is
  regenerated by the store skill on every store write, never hand-edited alongside it.
- **Commits and rulings** — suggest commits; never run git mutations, never push. User rulings
  are plain blocking text, never a timed prompt.
```

- **Kept deliberately:** nothing of the section remains in the `.md`. D15 rewordings inside
  the move, IDs unaffected (first mint): "its skill" and "the store skill" name their
  referent, `mochiko:authoring-architecture-store`. The author ≠ grader bullet carries
  `class: floor` in the schema — non-waivable house law (GI-004), matching the exemplar
  `implement.yaml`'s treatment of the same obligation; the Explore model rides
  `${explore_model}` per D5.
- **Consumers assessed:** the model-tiering bullet's protected provenance
  (`2026-08-16 model-tiered-seats`) is now a machine-checked `ruling:` anchor on
  `arch.model-tiering`; no other primitive addresses this section's headings.

## [v0.95.0] `## Boundaries — the non-waivable floor` — the whole section moves to the schema (D2/D7/D10 rollout)

- **Disposition:** superseded → the eleven `class: floor` rules in
  `plugins/mochiko/schemas/architecture.yaml` section `arch.sec.boundaries`
  (`arch.truth-user-ruling` · `arch.breadth-invariant` · `arch.floor-precedence` ·
  `arch.na-handled-elsewhere-pointer` · `arch.derived-index-never-hand-maintained` ·
  `arch.drift-empirical` · `arch.no-depth-dial-coupling` · `arch.no-delivery-harness` ·
  `arch.no-silent-store-mutations` · `arch.sound-loop-floor` · `arch.transport-floor`), text
  per the referent at D12 grain; skill-owned floors ride as `pointer:` rules
  (`mochiko:patterns-architecture-shelves` · `mochiko:authoring-architecture-store` ·
  `mochiko:patterns-sound-loop` · `mochiko:patterns-transport-floor`); protected content
  carries `ruling:` anchors (`2026-08-19 product-architecture-schema` D5/D9/D14 ·
  `2026-08-13 charter-ritual-balance`).
- **Tier failed:** n/a — supersession by ruling (record D2/D7 — the Boundaries floor is
  rule-shaped scope; the D10 rollout ruling for this command; `DECISIONS.md` 2026-08-26 row).
- **Content:** the whole shipped section, verbatim:

```
## Boundaries — the non-waivable floor

- **Architecture truth is the user's ruling.** The desk elicits, recommends, and records; a
  stance, a baseline, or an amendment lands only on the user's word. A row left unruled stays
  `open` and shows in the health view — it never acquires a stance by default, by silence, or
  by the desk's own judgment.
- **The breadth invariant.** Every shelf row in scope is walked. A row may close in two seconds
  — `n-a`, one line, done — but it is never silently skipped, and the walk order is by retrofit
  cost: tenancy, auth, and data partitioning before flags and experimentation. A shelf scope
  narrowed at the desk is the user's explicit override, recorded as one.
- **Floor precedence.** Where a governance floor card asserts the category, `n-a — genuinely
  never` is **unavailable** on that row: the legal moves are a stance within the obligation,
  `n-a — handled elsewhere` with its **required** pointer, or a narrowing. A true drop is a
  governance event and routes to a `governance-ledger.md` waiver through `/mochiko:setup` —
  never granted at the shelf. An arbitrated constitution card binds code-layer structure and a
  shelf row binds product topology; a genuine conflict between them is the user's.
- **`n-a — handled elsewhere` carries its pointer.** The reason axis is not decoration: a
  concern another repo or system owns names that owner. Only *genuinely never* may stand
  without one.
- **The derived index is never hand-maintained.** The store is the single source; the root
  `ARCHITECTURE.md` is regenerated from it by the store skill on every write. An
  index-vs-ledger disagreement is a defect fixed on sight, never reconciled by editing the
  index.
- **Drift is empirical.** An `As-built:` claim is graded against the code by a seat that reads
  it, never affirmed from memory or from the ruling that preceded it. An ungraded claim is
  reported as ungraded.
- **No depth-dial coupling.** The desk never reads the governance low/high depth level: shelf
  scope and per-row stances are its only adaptiveness, and `not-now` is the per-dimension depth
  valve. The governance dial governs a floor row's rigor; the desk governs its stance — two
  instruments, two axes.
- **No delivery harness at the desk.** The desk rules architecture and routes work; it plans
  and builds none of it. Delivery leaves for `/mochiko:feature` or the pipeline, and the
  boundary is audited from the store delta the work leaves behind.
- **No silent store mutations.** Every write is visible in the store and in the regenerated
  index; an integrity defect — an orphan element, a dead `Work:` pointer, a status
  disagreement — is fixed on sight and surfaced, never quietly corrected.
- **The sound-loop floor.** A judgment-authored write to a governing surface obliges the loop:
  a seat produces on a plan you approved, an independent non-author seat reviews before the
  user's gate — the user's ruling alone never substitutes for the review leg — and every
  baseline, stance batch, and amendment takes that review leg. Status flips and orphan cleanup
  are transcription and ride the landing audit. Trigger test, exemptions, seat wiring, and
  disclosure: `mochiko:patterns-sound-loop`, referenced never restated.
- **The transport floor.** A visit that composes more than one seat gains a floor on its
  composition and messaging: a split trigger — message legs on any multi-seat messaging,
  topology legs on shared writes — non-waivable once triggered. Trigger test, floor legs,
  composition-safe shapes, and disclosure: `mochiko:patterns-transport-floor`, referenced never
  restated.
```

- **Kept deliberately:** every floor survives whole as `class: floor` — must-survive under the
  pair audit (M3). One D15 rewording inside the move, ID unaffected (first mint): "the store
  skill" in the derived-index floor names its referent,
  `mochiko:authoring-architecture-store`. Every bullet's substantive wording otherwise
  survives in its rule text.
- **Consumers assessed:** the pair-form audit's `class: floor` = must-survive criterion
  (`.claude/rules/mochiko/primitive-edits.md`, re-keyed lead-side this wave) now grades
  exactly this set; `/mochiko:feature`'s symmetric Boundaries floor is its own text,
  untouched.

## [v0.95.0] Adaptive Goal Protocol defect sentence — re-keyed to the fail-condition set (D7/D10 rollout)

- **Disposition:** superseded → `arch.fail.no-verdict` in
  `plugins/mochiko/schemas/architecture.yaml` (section `arch.sec.fail-conditions`, labeled
  `fail-condition`, `class: floor`) + the `.md`'s new Not-done line hard-coding the
  `fail-condition` count (1) in the checker-grepped phrase form, with the out-of-sync halt
  clause.
- **Tier failed:** n/a — supersession by ruling (record D7 — the FAIL list moves, the `.md`
  Not-done line re-keys to the `fail-condition` label set with the C2 count guard; the D10
  rollout ruling for this command; `DECISIONS.md` 2026-08-26 row).
- **Content:** verbatim — "**A visit that ends with no stated done-condition verdict is a
  defect.**"
- **Kept deliberately:** the surrounding `$ARGUMENTS` sentence survives verbatim in the `.md`;
  the obligation itself survives strengthened — from "is a defect" prose to a
  `fail-condition` rule any one of which standing fails the visit, count-guarded by the D13
  checker.
- **Consumers assessed:** the pair-form audit's FAIL-survival criterion keys to the
  `fail-condition` label set; the checker's `NOT_DONE_RE` verified matching the new `.md`
  line (PASS run, 0 findings, cited in the audit brief).

## [v0.91.0] Charter dispatch-targets line: "plan and implement own all delivery" → implement alone — plan-stage retirement D1

- **Disposition:** superseded → the same Tools dispatch-targets clause naming `/mochiko:implement`
  as the sole owner of delivery.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1: `/mochiko:plan` retires as a
  command and `/mochiko:implement` becomes the single downstream run; the record's Build surface
  names this line explicitly as charter-protected content requiring a supersession by ruling).
- **Content (superseded text, verbatim):**

  ```
    drop (Boundaries) · `/mochiko:plan` and `/mochiko:implement` own all delivery; the desk runs
    none of it.
  ```

  Replaced by:

  ```
    drop (Boundaries) · `/mochiko:implement` owns all delivery; the desk runs none of it.
  ```

- **Kept deliberately:** the desk-runs-none-of-it clause itself — the protected half of this
  line is the boundary (the architecture desk never hosts delivery), and D1 does not touch it;
  only the enumeration of which pipeline commands own delivery changed. The sibling Boundaries
  clause "**No delivery harness at the desk**" is untouched and still carries the same rule
  independently, so the boundary survives in two places exactly as before.
- **Consumers assessed:** the full file was swept — this was the only `/mochiko:plan` site in
  `commands/architecture.md`. The remaining "plan" occurrences are sound-loop plan-approval
  language ("a producing seat plans first and works only on a plan you approved") and are
  unaffected by this ruling. `/mochiko:feature`'s parallel dispatch-targets block and the
  router's entry-point table were re-keyed in the same wave.
