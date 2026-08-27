# Strip notes — `commands/brainstorm.md`

Entry formats: `strips/README.md`. Wave context: the D4 codification pre-shrink — the
shared team-form prose relocated into `templates/command-shape.md` (design:
`.mochiko/brainstorms/pattern-codification-and-minimalism/record.md`, D3/D4/D9). **Stale as a standing claim:**
the shape is now **v5** (2026-07-30) — see the v0.35.0 section below.

<!-- Wave context: the near-dup convergence wave (v0.99.0) — three brainstorm rules
converge to `extends: common.<slug>` stubs under strongest-wording-wins. Ruling for every
[v0.99.0] entry below: `.mochiko/decisions/2026-08-28-near-dup-convergence.md` R1–R6 +
wave flags → `DECISIONS.md` 2026-08-28 row. Every ID survives as a stub (R3 — no
tombstones); `class`, `kind`, `when:`, and `enforces:` stay local per ontology C3. Floor
entries are supersessions per R4. -->

## [v0.99.0] `brainstorm.model-tiering` — text single-homed into `common.model-tiering`

- **Disposition:** superseded → `extends: common.model-tiering`; brainstorm's wording is
  the block text except one formatting mote — its `Explore` carried backticks, the block's
  does not (matching arch/feat/impl/setup/spec, all bare).
- **Tier failed:** n/a — supersession by ruling (near-dup ADR R1/R2).
- **Content:** verbatim —

  ```
          Exploration and fact-finding dispatches ride mochiko:patterns-model-tiering's
          class key — locate/enumerate reads to a native `Explore` subagent spawned
          model: ${explore_model}, interpretive or absence-driven reads on the session
          tier — and every seat brief carries the routing rule; referenced, never restated.
  ```

- **Kept deliberately:** `class`/`kind: routing` local; labels + pointer inherited, same
  values.
- **Consumers assessed:** none — per-command rule.

## [v0.99.0] `brainstorm.plan-approval-producers` — text single-homed into `common.plan-approval-producers`

- **Disposition:** superseded → `extends: common.plan-approval-producers`; brainstorm's
  wording IS the block text (exact triple with setup/specify — the D8 exact bar already
  licensed it), resolved text unchanged.
- **Tier failed:** n/a — supersession by ruling (near-dup ADR R1; D8 exact bar).
- **Content:** verbatim —

  ```
          Any seat that writes artifacts plans first and works only on a plan you approved;
          grading and fact-finding seats are exempt.
  ```

- **Kept deliberately:** `class: must` local; labels inherited, same values.
- **Consumers assessed:** none — per-command rule.

## [v0.99.0] `brainstorm.author-grader-default-fail` (floor) — short text superseded by the stronger block wording

- **Disposition:** superseded → `extends: common.author-grader-default-fail`; resolved text
  UPGRADES to setup's strongest wording, gaining "grading reads the authored surfaces
  themselves — never the author's report".
- **Tier failed:** n/a — supersession by ruling (near-dup ADR R1/R2/R4 — floor).
- **Content:** verbatim —

  ```
          No output is cleared by its author — default FAIL.
  ```

- **Kept deliberately:** the whole obligation survives inside the stronger block text;
  `class: floor` local on the stub (C3).
- **Consumers assessed:** none — per-command rule.

<!-- Wave context: the command-schema ontology wave (v0.98.0) — the rule grammar gains
`kind:`, a declared `conditions:` block with a rule-level `when:`, a declared `moments:`
block, `enforces:` on fail nodes, and `extends: common.<slug>` (D1–D8). Ruling for every
[v0.98.0] entry below: `.mochiko/brainstorms/command-schema-ontology/record.md` D1–D11 as
amended → `DECISIONS.md` 2026-08-27 row. Clause inventory and audit referent:
`.mochiko/brainstorms/command-schema-ontology/conversion-inventory.md` — sections A.6
(kinds), B.6 (the `when:` extraction), C.6 / D.6 (the declared blocks), E.6 (`enforces:`),
F (bindings), G (the canonical header), H (the Not-done re-key).

Riding the decision row, no strip owed: every `kind:` line (a pure addition, `constraint`
staying the omitted default) · the `conditions:` and `moments:` blocks · `enforces:` on the
four fail nodes · the C4 semantics comment on the one floor that gained a `when:` · the
`.md` Rules-block sentences naming the new grammar and instructing the `common.yaml` Read.
Brainstorm has no DECLARE case — both of its shape-gated rules took the MOVE disposition. -->

## [v0.98.0] Two rule-level activation guards single-homed out of `text:` into `when:` (D3)

- **Disposition:** superseded → the `when:` field on each rule, resolving against the new
  top-level `conditions:` block; both IDs kept, both rewords tabled in the inventory's
  section B.6.
- **Tier failed:** n/a — supersession by ruling (`command-schema-ontology` D3, single-homing:
  "rule-level activation conditions live in `when:` alone and leave the `text`";
  `DECISIONS.md` 2026-08-27.)
- **Content:** the two texts as they stood at v0.97.0, verbatim (block scalars quoted in their
  folded, single-line form — the text a run sees), each with the guard that left:

  1. `brainstorm.km-close-ritual` — guard "Where `${km_path}` exists" →
     `when: {km_file: present}`:
     "Where `${km_path}` exists, run its close ritual."
  2. `brainstorm.transport-floor` (`class: floor`) — guard "When the run composes more than
     one seat" → `when: {seats: multi}`:
     "When the run composes more than one seat, mochiko:patterns-transport-floor governs its
     composition and messaging under a split trigger — message legs on any multi-seat
     messaging, topology legs on shared writes — non-waivable once triggered; referenced,
     never restated."

- **Kept deliberately:** both obligations survive whole — only the activation guard moved, and
  both IDs are untouched. `brainstorm.km-close-ritual` keeps the `${km_path}` reference in its
  reworded text ("Run the close ritual of `${km_path}`"), so the run still learns *which*
  file's ritual it owes; the `when:` term carries only whether the rule binds.
  `brainstorm.transport-floor` stays `class: floor` and is still always read and always
  delivered — `when:` gates when the obligation applies, never whether it reaches the run
  (C4), stated on the block as a comment; the guard-stripped remainder is what
  `common.transport-floor` now carries (next entry but one). Guards that are NOT rule-level
  activation stayed in prose untouched, per the inventory's stays-prose table:
  `brainstorm.pair-maps-independent`'s "In a review pair" (a genuine composition branch with
  no declared dimension — the inventory's J-2 recommends prose this wave),
  `brainstorm.reopen-born-verify`'s subject-carried trigger,
  `brainstorm.index-bookkeeping`'s "where the outcome landed",
  `brainstorm.next-step-offer`, `brainstorm.user-review-waiver`, and
  `brainstorm.synthesis-on-request`'s mid-sentence carves.
- **Consumers assessed:** `plugins/mochiko/commands/brainstorm.md` (the Rules block gains the
  `when:` interpretation sentence the same wave) · `scripts/check-command-schema.py` (the
  `when:`-resolution and per-dimension coverage checks land the same wave) ·
  `.claude/rules/mochiko/primitive-edits.md` criterion 11 and
  `.claude/skills/converting-command-to-schema/SKILL.md` (both amended the same wave, with
  their own entries) · `.mochiko/provenance.yaml` (anchors are keyed by rule ID and both IDs
  are kept, so no anchor moves).

## [v0.98.0] The `fail-condition` label removed from all four fail nodes — `kind: fail` is the selector (D1)

- **Disposition:** superseded → `kind: fail` on each of the four nodes in
  `plugins/mochiko/schemas/brainstorm.yaml`, section `brainstorm.sec.fail-conditions`.
- **Tier failed:** n/a — supersession by ruling (`command-schema-ontology` D1 and
  build-surface item 4, "`kind: fail` replaces the `fail-condition` label as the operative
  selector"; `DECISIONS.md` 2026-08-27. The label is retired from the registry the same wave —
  entry in `.mochiko/strips/command-labels.md`.)
- **Content:** the `fail-condition` label value, verbatim, as the first entry of the
  `labels:` list on each of: `brainstorm.fail.record-unaccepted` ·
  `brainstorm.fail.unreviewed-no-waiver` · `brainstorm.fail.survivor-undispositioned` ·
  `brainstorm.fail.index-mismatch`.

  The section's `intent:` line named the retired key too, and is re-keyed with them —
  verbatim as it stood at v0.97.0:

  > The fail-condition set — any one standing fails the run; the .md Not-done line hard-codes this set's count.

- **Kept deliberately:** every node's second label (`user-gate` · `independence` ·
  `user-gate` · `landing`) survives, so the topic clusters are unchanged; the `intent:` line
  keeps both of its clauses — the any-one-standing semantics and the count-pin note — with
  only "The fail-condition set" becoming "The kind: fail set"; the
  `brainstorm.sec.fail-conditions` section ID and the `brainstorm.fail.*` ID segment are
  untouched — both are ID grammar, not the retired label; `class: floor` on all four is
  untouched; the count stays 4, so the `.md` count-pin does not move.
- **Consumers assessed:** `plugins/mochiko/schemas/command-labels.yaml` (registry line removed
  the same wave) · the `.md` Not-done line (re-keyed the same wave — next entry) ·
  `.claude/rules/mochiko/primitive-edits.md` criterion 3 (re-keyed the same wave) ·
  `scripts/check-command-schema.py` (the count check and the bidirectional
  `.fail.`-segment ↔ `kind: fail` cross-check re-key the same wave).

## [v0.98.0] The `.md` Not-done line's `fail-condition` key superseded by `kind: fail`

- **Disposition:** superseded → the re-keyed line in
  `plugins/mochiko/commands/brainstorm.md`, Adaptive Goal Protocol step 3.
- **Tier failed:** n/a — supersession by ruling (`command-schema-ontology` build-surface
  item 4; `DECISIONS.md` 2026-08-27. Reworded line tabled in the inventory's section H.)
- **Content:** step 3 as it stood at v0.97.0, verbatim —

  > 3. **Not done — default FAIL:** the 4 rules labeled `fail-condition` in
  >    `plugins/mochiko/schemas/brainstorm.yaml` (section `brainstorm.sec.fail-conditions`) — any one
  >    standing fails the run. If the schema's `fail-condition` count is not 4, the pair is out of
  >    sync: halt and surface it before closing.

- **Kept deliberately:** the whole guard the old key carried — the hard-coded count 4, the
  section citation, the any-one-standing clause, and the out-of-sync halt clause — survives
  verbatim on the new key; only the two `fail-condition` tokens become `kind: fail`. The count
  does not move: this wave adds and removes no fail node.
- **Consumers assessed:** `scripts/check-command-schema.py` greps this literal phrase for the
  count guard (re-keyed the same wave) · `.claude/rules/mochiko/primitive-edits.md`
  criterion 3 · `.claude/skills/converting-command-to-schema/SKILL.md` (both re-keyed the same
  wave, with their own entries).

## [v0.98.0] Four rules' `labels:`, `text:`, and `pointer:` extracted into `common.yaml` blocks (D8)

- **Disposition:** superseded → the shared blocks `common.register` ·
  `common.no-git-mutations` · `common.acceptance-plain-text` · `common.transport-floor` in
  `plugins/mochiko/schemas/common.yaml`, each bound by a stub carrying
  `extends: common.<slug>` plus its local `class:` (and, where they apply, local `kind:`,
  `when:`, and `labels:`).
- **Tier failed:** n/a — supersession by ruling (`command-schema-ontology` D8 as narrowed at
  C2 and given its precedence clause at C3 — itself a recorded supersession-by-ruling of
  command-content-schema D3, amended not reversed; `DECISIONS.md` 2026-08-27. The
  3-or-more-exact-duplicate bar and the bind/no-bind table: inventory F.0/F.1. Brainstorm is
  the only command that binds all four surviving blocks.)
- **Content:** the fields as they stood at v0.97.0, verbatim —

  ```yaml
        - id: brainstorm.register
          labels: [binding, reporting]
          text: >-
            User-facing prose follows templates/output-style.md.

        - id: brainstorm.no-git-mutations
          labels: [user-gate]
          text: >-
            Suggest commits; never run git mutations, never push.

        - id: brainstorm.acceptance-plain-text
          labels: [user-gate]
          text: >-
            User acceptance is plain blocking text, never a timed prompt.

        - id: brainstorm.transport-floor
          labels: [floor-pointer, seats]
          text: >-
            When the run composes more than one seat, mochiko:patterns-transport-floor
            governs its composition and messaging under a split trigger — message legs on any
            multi-seat messaging, topology legs on shared writes — non-waivable once
            triggered; referenced, never restated.
          pointer: "mochiko:patterns-transport-floor"
  ```

  (`brainstorm.transport-floor`'s guard clause is recorded twice by design: it leaves the text
  once, and the first entry above records the single-homing that put it in `when:` — this
  entry records the guard-stripped remainder moving to the shared block.)
- **Kept deliberately:** all four keep their IDs — a stub's `brainstorm.*` ID stays the
  citable ID — and all four declare `class:` **locally**, per C3's precedence clause:
  `class:` and every absence-meaningful field are never inherited, so a floor's bindingness
  stays readable from its own file. `brainstorm.register` re-declares
  `labels: [binding, reporting]` locally, because the block carries `[reporting]` alone and
  inheriting it would silently narrow a shipped rule's labels; the other three match their
  blocks and inherit. `brainstorm.transport-floor` keeps `class: floor` and its
  `when: {seats: multi}` locally, and the block carries no `class:` at all — a class there
  would be inherited but always overridden. Each resolved text is byte-identical to what was
  removed except `transport-floor`'s single-homed guard, so no rule's meaning changes.
  `brainstorm.model-tiering` was a prototype stub and does **not** bind: its block failed the
  3+ bar (two members) and, per the inventory's J-9, brainstorm's text differs from the block
  by two backticks around `Explore` — it stays a full per-command rule, restored
  byte-identical to v0.97.0, and the prototype was never committed so nothing shipped in stub
  form and no tombstone is due.
- **Consumers assessed:** `plugins/mochiko/commands/brainstorm.md` (its first action now Reads
  `common.yaml` raw beside the schema — added the same wave; brainstorm had no such
  instruction before, so this is the load-bearing half of the edit) ·
  `plugins/mochiko/schemas/common.yaml` (new shipped primitive, same wave) ·
  `scripts/check-command-schema.py` (`extends:` target resolution, orphan-block detection, and
  the C3 local-`class:` assert land the same wave) · `.mochiko/provenance.yaml` (keyed by rule
  ID; all four IDs are kept, so no anchor moves) · the other four commands binding the same
  blocks (specify 4 · setup 3 · feature 2 · architecture 2 · implement 1 — a text change to a
  block now reaches every binder, which is the drift-protection the extraction buys and the
  cost it carries).

## [v0.98.0] The schema header's D6 grammar block superseded by the canonical ontology header (G)

- **Disposition:** superseded → the canonical header comment at the top of
  `plugins/mochiko/schemas/brainstorm.yaml` (inventory section G, `<cmd>`/`<prefix>`
  substituted), which keeps every surviving D6/D11/D12/D14/D15/D16 line and adds the D1–D8
  grammar. `at:` is deliberately absent (D4 as amended at I5 — deferred to graduation).
- **Tier failed:** n/a — supersession by ruling (`command-schema-ontology` D1–D8 amend
  command-content-schema D6's rule-block grammar; `DECISIONS.md` 2026-08-27.)
- **Content:** the superseded header lines as they stood at v0.97.0, verbatim —

  ```
  # rule-shaped content (command-content-schema D1/D2, rolled out to this command by the D10
  # rollout ruling 2026-08-26 — record Session trail; DECISIONS.md 2026-08-26). The command
  # Grammar (D6, as amended by D14): sections: list, each {id, title, intent, rules};
  # rule blocks {id, labels, class, text, pointer?} nest under their section.
  #   section id  <cmd>.sec.<slug> — minted once like rule IDs (D11); title verbatim from
  #           the command group it carries; intent one line, navigation only — sections
  #           never grow a second prose surface (narrative stays in the .md).
  #   brainstorm.sec.* · class: values · labels · pointer: skills · file
  #   paths). Deixis ("these rules", "this section", "above"/"below", document-shape
  ```

- **Kept deliberately:** every grammar line the amendment does not touch survives verbatim —
  the ID mint-once/reword/split/merge rules, the `brainstorm.fail.*` segment line, the labels
  registry line, the `class:` value definitions (already reading "pair audit", so brainstorm
  was not one of the inventory's two J-13 stragglers), the `text:`/`${var}` line, the D16
  provenance sidecar line, the `pointer:` line, the deixis ban and its curated-marker note,
  the legal self-reference pair ("this schema" and "the run" — brainstorm is a run command, so
  the desks' third term is correctly absent), the D12 grain line, and the D13
  advisory-checker line. The command-content-schema D10 rollout citation is superseded by the
  canonical header's D1/D7 citation, which every schema now shares; the rollout fact survives
  in that record's Session trail, which the old line itself pointed at.
- **Consumers assessed:** the other five command schemas take the same canonical header the
  same wave · `.claude/skills/converting-command-to-schema/SKILL.md` restates this header for
  new conversions (amended the same wave, with its own entry) ·
  `scripts/check-command-schema.py` (grammar checks extended the same wave).

<!-- Wave context: the command-`.md`-scaffold standardization wave (v0.97.0) — one canonical
`.md` scaffold for all six pair commands (D1/D2: Identity & Mission · Rules block · Adaptive
Goal Protocol with Entry / Goal / count-pinned Not-done last) and the schema six-set
unification (D3/D4/D5: `<prefix>.sec.roles` · `reserved` · `tools` · `ways-of-working` ·
`boundaries` · `fail-conditions`; rule IDs and texts carried unchanged — pure relocation, the
D14 precedent). Ruling for every [v0.97.0] entry below:
`.mochiko/brainstorms/command-md-scaffold-standardization/record.md` D1–D7 as review-amended
→ `DECISIONS.md` 2026-08-27 command-md-scaffold-standardization row. -->

## [v0.97.0] The bold `**Goal:**` opener and the `## Goal` section — the goal-form scaffold superseded (D1/D2)

- **Disposition:** superseded → `plugins/mochiko/commands/brainstorm.md` `## Adaptive Goal
  Protocol`; the opener's mission clause and `$ARGUMENTS` handling into step 1 (Entry, N1),
  the `## Goal` body verbatim into step 2.
- **Tier failed:** n/a — supersession by ruling (`command-md-scaffold-standardization` D1 as
  narrowed at review C2, D2, N1; `DECISIONS.md` 2026-08-27 row) — the goal-form half of the
  F3 two-form split.
- **Content:** verbatim — "**Goal:** think `$ARGUMENTS` through with the user and leave one
  hardened decision record behind. Empty topic → ask what we are thinking through." and the
  heading "## Goal".
- **Kept deliberately:** the fixed done-condition contract untouched (D1's C2 narrowing
  supersedes layout and vocabulary only) — the whole `## Goal` body (record path, the five
  confidence marks, cold-review-or-waiver, index bookkeeping, user acceptance) relocated
  verbatim into step 2; the 4-count Not-done line verbatim. (No strip owed for the new
  `## Identity & Mission` — pure addition riding D2 step 2, knowingly strip-fodder if the D7
  absorption trigger later fires.)
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` dual criteria blocks
  collapsed the same wave (D6-R2); `.claude/skills/converting-command-to-schema/SKILL.md`
  re-keyed the same wave (build item 7); `README.md:5` ripple (build item 6).

## [v0.97.0] The Rules-block three-section enumeration (`brainstorm.sec.harness` · `brainstorm.sec.bindings`) — superseded by the unified six-set (D3/D4/D5)

- **Disposition:** superseded → the six-section enumeration in
  `plugins/mochiko/commands/brainstorm.md`; the harness gloss splits across
  `brainstorm.sec.roles` and `brainstorm.sec.reserved`, the bindings gloss carries whole onto
  `brainstorm.sec.tools`, and `ways-of-working` / `boundaries` are newly enumerated under the
  D5 breadth invariant.
- **Tier failed:** n/a — supersession by ruling (`command-md-scaffold-standardization` D3,
  D4, D5; `DECISIONS.md` 2026-08-27 row).
- **Content:** verbatim — "run's binding rules, nested in three sections, each addressable by
  its section ID: `brainstorm.sec.harness` (lead role, seat wiring, review independence, and
  the decisions reserved to the user) · `brainstorm.sec.bindings` (deliverable, index,
  synthesis, register, next step) · `brainstorm.sec.fail-conditions` (the Not-done set)."
- **Kept deliberately:** every gloss word — "lead role, seat wiring, review independence" →
  `roles`; "the decisions reserved to the user" → `reserved`; "deliverable, index, synthesis,
  register, next step" → `tools`; "the Not-done set" unchanged. Surrounding boilerplate
  verbatim (anchor phrase "before any questioning, before any seat is spawned", raw-Read
  clause, interpretation clause, not-open-until-read close). The new `ways-of-working` and
  `boundaries` glosses are wave-authored prose, retuned once in-wave to match the schema's
  actual contents (lead-approved scope extension).
- **Consumers assessed:** `plugins/mochiko/schemas/brainstorm.yaml` tombstones
  `brainstorm.sec.harness` / `.bindings` the same wave; the checker's section-count and
  token-resolution lints re-keyed the same wave; no `brainstorm.sec.*` token appears anywhere
  else in this `.md`.

## [v0.97.0] `brainstorm.sec.harness` and `brainstorm.sec.bindings` — tombstoned; their 25 rules redistributed across the six-set (D3/D4/D5)

- **Disposition:** superseded → the six minted section nodes in
  `plugins/mochiko/schemas/brainstorm.yaml` (`brainstorm.sec.roles` · `reserved` · `tools` ·
  `ways-of-working` · `boundaries`, plus the pre-existing `brainstorm.sec.fail-conditions`).
  Both retired IDs are recorded under the schema's top-level `tombstones:` key per D11.
  All 25 rules relocate with ID, text, labels, class and pointer byte-identical.
- **Tier failed:** n/a — supersession by ruling (record D3 · D4 · D5; `DECISIONS.md`
  2026-08-27 row).
- **Content:** verbatim, the two section nodes' `title:` and `intent:` lines that left —
  `brainstorm.sec.harness`, title "Harness", intent "How the session is led, staffed,
  reviewed, and bounded, and the decisions reserved to the user." ·
  `brainstorm.sec.bindings`, title "Bindings", intent "Deliverable, index, synthesis,
  register, and next-step bindings the session lands on." No rule content left the file.
  Plus, from the same schema's grammar header, the sentence the new `tombstones:` key made
  false — verbatim: "First mint — no tombstones yet." (the surrounding line, "a merge retires
  the losers under a top-level tombstones: key.", survives unchanged).
- **Kept deliberately:** every one of the 29 rules survives — 25 relocated, the 4
  `fail-condition` rules untouched in their existing node. Rule counts and the
  `fail-condition` count are identical pre/post (29 / 4). Both retired intents' substance
  survives distributed across the six new intents, which is why nothing relocates to a
  strip-only home. The grammar header's tombstone clause itself survives — only its
  no-tombstones-yet claim leaves, superseded by the two section tombstones this same edit
  mints.
- **Consumers assessed:** provenance is rule-ID-keyed (9 `brainstorm.*` anchors, no `sec.`
  keys) — no anchor re-keys. No surviving rule text in the schema references
  `brainstorm.sec.harness` or `brainstorm.sec.bindings` (grepped, clean — the I8 check).
  `brainstorm.md`'s Rules-block enumeration and its "nested in three sections" phrase are
  re-keyed to six by the same wave's `.md` rewrite; the checker's D14 section-count guard
  reads that phrase. The D11 both-live-and-tombstoned guard is satisfied — neither retired ID
  is re-minted.

## [v0.97.0] Schema grammar-header narrative description — re-worded to the post-D2 scaffold (D2)

- **Disposition:** superseded → the canonical post-D2 wording carried by all six schemas:
  "Narrative (Identity & Mission, Adaptive Goal Protocol prose) stays in
  plugins/mochiko/commands/brainstorm.md."
- **Tier failed:** n/a — supersession by ruling (record D2 — the goal-form `**Goal:**` opener
  and `## Goal` section are superseded by the canonical scaffold's Adaptive Goal Protocol;
  `DECISIONS.md` 2026-08-27 row).
- **Content:** verbatim, the sentence that left — "Narrative (the goal statement and the Goal
  section's done condition) stays in plugins/mochiko/commands/brainstorm.md."
- **Kept deliberately:** the clause's whole job survives — naming which surface holds the
  narrative and which holds the rules. Only its enumeration of the retired goal-form sections
  leaves, replaced by the sections the D2 scaffold actually ships. No rule content touched.
- **Consumers assessed:** header comment only — never parsed, never read by a run; the checker
  reads `kind`, `command`, `vars`, `sections`, `tombstones` and is unaffected (re-run: 0
  findings). All six schemas now carry this sentence byte-identical but for the filename.

<!-- Wave context: the D16 provenance-sidecar amendment (v0.96.0) — schemas carry runtime
content only; decision anchors move to `.mochiko/provenance.yaml`, keyed by rule ID. Ruling:
record D16 (post-rollout amendment, user-ruled 2026-08-26, incl. the repo-side-home
refinement) → `DECISIONS.md` 2026-08-26 command-content-schema row. -->

## [v0.96.0] `ruling:` fields and the grammar-header ruling description — extracted to the provenance sidecar (D16)

- **Disposition:** relocated → `.mochiko/provenance.yaml` — every `ruling:` field in
  `plugins/mochiko/schemas/brainstorm.yaml` carried verbatim as an `anchors:` entry keyed by its
  rule's mint-once ID (checker-verified: the pair's run reports `anchors 9`, each entry
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

## [v0.95.0] `## Harness` — the whole section moves to the schema (D2, exercised by the D10 rollout ruling)

- **Disposition:** superseded → `plugins/mochiko/schemas/brainstorm.yaml`, section
  `brainstorm.sec.harness` — the ten bullets at D12 grain as eighteen rules, in the shipped
  order: `brainstorm.lead-inline-questioning` · `brainstorm.staffing-latitude` ·
  `brainstorm.transport-floor` · `brainstorm.model-tiering` ·
  `brainstorm.plan-approval-producers` · `brainstorm.author-grader-default-fail` ·
  `brainstorm.record-review-independence` · `brainstorm.blind-map-dispatch` ·
  `brainstorm.pair-maps-independent` · `brainstorm.coverage-survivor-routing` ·
  `brainstorm.non-coverage-survivors` · `brainstorm.reopen-born-verify` ·
  `brainstorm.user-record-acceptance` · `brainstorm.user-survivor-challenge` ·
  `brainstorm.user-review-waiver` · `brainstorm.user-pen-boundary` ·
  `brainstorm.no-git-mutations` · `brainstorm.acceptance-plain-text`. Skill-owned floors ride
  as `pointer:` rules (`mochiko:analysis-iterative` · `mochiko:patterns-transport-floor` ·
  `mochiko:patterns-model-tiering`); the blind-map / coverage-routing / reopen-verify trio
  carries `ruling:` anchors to the 2026-08-10 cold-review-gap-challenge row, the model-tiering
  rule to the 2026-08-16 model-tiered-seats row.
- **Tier failed:** n/a — supersession by ruling (command-content-schema D2 — rules move to the
  schema, narrative stays — as exercised for this command by the D10 rollout ruling
  2026-08-26, record Session trail, structure-only extraction against the frozen referent
  `.mochiko/brainstorms/command-content-schema/referents/brainstorm-shipped-v0.94.0.md`;
  `DECISIONS.md` 2026-08-26 command-content-schema row).
- **Content:** the whole shipped section, verbatim:

```
## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; run the questioning
  yourself, inline, via `mochiko:analysis-iterative` — one question per turn, format adapted to
  the user's state. Teammates or subagents per seat is your call.
- **Transport floor.** When the run composes more than one seat,
  `mochiko:patterns-transport-floor` governs its composition and messaging under a split
  trigger — message legs on any multi-seat messaging, topology legs on shared writes —
  non-waivable once triggered; referenced, never restated.
- **Model tiering.** Exploration and fact-finding dispatches ride
  `mochiko:patterns-model-tiering`'s class key — locate/enumerate reads to a native
  `Explore` subagent spawned `model: haiku`, interpretive or absence-driven reads on the session
  tier — and every seat brief carries the routing rule; referenced, never restated.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author — the record is yours, so its review
  seat is always someone else, reading the frozen record cold from the file, default FAIL.
- **Blind-map dispatch:** a review seat is spawned in two messages — first the topic statement
  and goal line only, *never* the record path, so it builds its Phase 0 angle map with no
  sight of what the session decided; its map returns before you send the record path and the
  cold read begins. The anchoring fence is structural, not a trust ask. In a pair, both seats
  build their maps independently.
- **Coverage-survivor routing:** a surviving coverage finding is a candidate that questions the
  topic itself, not a fold — present each gap as a candidate topic; **the user** rules the path: **explore now**
  (re-enter `mochiko:analysis-iterative` on that angle; the resulting decision lands in the
  record's same `D…` namespace), **rule inline**, or **defer**. Non-coverage survivors keep the
  ordinary fold / repair / ruling path and may be dispositioned in batches.
- **Reopen-born verify:** a decision born from a coverage-survivor reopen gets one bounded verify
  round — internal consistency and record-fitness, no fresh cold read, no blind-map coverage
  hunt against it, and no second reopen off it.
- **Reserved to the user:** record acceptance · the disposition of any review survivor that
  challenges a user ruling · the waiver, if the review is to be skipped · any amendment to a
  user-ruled decision, and any new decision — their word, never yours.
- Suggest commits; never run git mutations, never push. User acceptance is plain blocking
  text, never a timed prompt.
```

- **Kept deliberately:** every obligation survives one-for-one as rule text — none dropped,
  none merged; "the record is yours … default FAIL" splits at D12 grain into the general floor
  (`brainstorm.author-grader-default-fail`) and the record-specific consequence
  (`brainstorm.record-review-independence`); the reserved-to-user list splits into its four
  items, "their word, never yours" carried on the acceptance and pen-boundary rules. Rewrite
  delta, recorded separately from the verbatim Content above (D15 referential closure only,
  no other rewording): "toward the Goal" → "toward the run's goal" · "the record is yours" →
  "The record at ${record_path} is yours" · "the record path" → "${record_path}" (twice, in
  the blind-map rule) · "In a pair, both seats build their maps independently" → "In a review
  pair, both seats build their Phase 0 angle maps independently" · "the record's same `D…`
  namespace" → "${record_path}'s same `D…` namespace" · `model: haiku` →
  "model: ${explore_model}" (var-injected, D5). Additionally: bullet-label fragments
  normalized to standalone sentences (subject inserted); inline emphasis/backticks dropped for
  block-scalar safety; no obligation altered.
- **Consumers assessed:** commands are entry points, nothing mounts them; the pair-form audit
  block in `.claude/rules/mochiko/primitive-edits.md` (lead-owned, same wave) grades the
  `.md` + schema pair from v0.95.0; the pointed-at skills are untouched.

## [v0.95.0] `## Bindings` — the whole section moves to the schema (D2, exercised by the D10 rollout ruling)

- **Disposition:** superseded → `plugins/mochiko/schemas/brainstorm.yaml`, section
  `brainstorm.sec.bindings` — the five bullets at D12 grain as seven rules, in the shipped
  order: `brainstorm.deliverable-record` · `brainstorm.record-as-you-go` ·
  `brainstorm.index-bookkeeping` · `brainstorm.km-close-ritual` ·
  `brainstorm.synthesis-on-request` · `brainstorm.register` · `brainstorm.next-step-offer`.
  The repeated paths declare once in `vars:` (`record_path` · `index_path` · `km_path`, D5).
- **Tier failed:** n/a — supersession by ruling (command-content-schema D2 as exercised by the
  D10 rollout ruling 2026-08-26, record Session trail, against the frozen referent
  `.mochiko/brainstorms/command-content-schema/referents/brainstorm-shipped-v0.94.0.md`;
  `DECISIONS.md` 2026-08-26 command-content-schema row).
- **Content:** the whole shipped section, verbatim:

```
## Bindings

- **Deliverable:** `.mochiko/brainstorms/<slug>/record.md` — kebab-case `<slug>` derived at the
  start, decisions in one `D1…` namespace, written as the session progresses, never
  reconstructed at the end.
- **Index:** `.mochiko/brainstorms/index.md` — read before opening; enter the session on open
  (status: open); update at acceptance or supersession with where the outcome landed. Where
  `.mochiko/memory/knowledge-management.md` exists, run its close ritual.
- **Synthesis:** on request only, after acceptance — beside the record, stamped
  *derived — record canonical*; under a review waiver, stamped *derived, unchecked*.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** pipeline entry (e.g. `/mochiko:specify` when the record is honestly a
  feature description) is an offer after acceptance, never a default.
```

- **Kept deliberately:** all five bindings survive whole — the Deliverable bullet splits at
  D12 grain into the artifact binding and the written-as-you-go obligation; the Index bullet
  into the index bookkeeping and the KM close ritual; both synthesis stamps (*derived — record
  canonical* and the waiver's *derived, unchecked*) survive verbatim in
  `brainstorm.synthesis-on-request`. Rewrite delta, recorded separately (D15/D5 only): the
  literal paths substitute as `${record_path}` / `${index_path}` / `${km_path}`; "beside the
  record" → "beside ${record_path}". Additionally: bullet-label fragments normalized to
  standalone sentences (subject inserted); inline emphasis/backticks dropped for block-scalar
  safety; no obligation altered.
- **Consumers assessed:** none — bindings local to this command; `templates/output-style.md`
  and the KM module are referenced, not edited.

## [v0.95.0] The `**Not done — default FAIL**` list — 4 clauses become the `fail-condition` rule set (D7 form, exercised by the D10 rollout ruling)

- **Disposition:** superseded → the 4 `brainstorm.fail.*` rules labeled `fail-condition` in
  `plugins/mochiko/schemas/brainstorm.yaml` (section `brainstorm.sec.fail-conditions`), in the
  shipped order: `brainstorm.fail.record-unaccepted` · `brainstorm.fail.unreviewed-no-waiver` ·
  `brainstorm.fail.survivor-undispositioned` · `brainstorm.fail.index-mismatch`; the `.md`'s
  Not-done line re-keys to the count pointer — "the 4 rules labeled `fail-condition` in
  `plugins/mochiko/schemas/brainstorm.yaml`" — N=4 pinned, the C2 guard, the count match
  checker-verified (D13).
- **Tier failed:** n/a — supersession by ruling (command-content-schema D7's fail-condition
  form as exercised for this command by the D10 rollout ruling 2026-08-26, record Session
  trail; `DECISIONS.md` 2026-08-26 command-content-schema row).
- **Content:** verbatim —

```
**Not done — default FAIL:** an unaccepted record · an unreviewed record with no recorded
waiver · an undispositioned review survivor · an index entry missing or contradicting the
record's status.
```

- **Kept deliberately:** all four clauses survive one-for-one as rules — none dropped, none
  merged; the default-FAIL posture survives on the `.md` line itself ("any one standing fails
  the run") plus its out-of-sync halt sentence. Rewrite delta, recorded separately (D15 only):
  "an index entry" → "An index entry in ${index_path}".
- **Consumers assessed:** the pair-form audit's FAIL-survival criterion keys to the
  `fail-condition` label set (`.claude/rules/mochiko/primitive-edits.md`, lead-owned, same
  wave); the D13 checker binds the `.md` count to the schema set deterministically.

---

## [v0.78.0] Model-tiering floor line retargeted — `mochiko:explorer` superseded by native `Explore` + `model: haiku` override

- **Disposition:** superseded → the reworded floor line: locate/enumerate reads go to "a
  native `Explore` subagent spawned `model: haiku`".
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents.
- **Content:** verbatim superseded phrase: "the cheap explorer seat (`mochiko:explorer`)"
  (line wrap varies per command; only this phrase changed).
- **Kept deliberately:** the rest of the floor line byte-for-byte — the class-key summary,
  session-tier carve-outs, the every-seat-brief obligation, and the closing
  `mochiko:patterns-model-tiering` referenced-never-restated pointer.
- **Consumers assessed:** the same phrase edited in all six commands in the same v0.78.0
  wave (entry mirrored per command strip file); the pointed-at skill reworded in the same
  wave (`strips/patterns-model-tiering.md`).

---

## [v0.48.0] Shape v8 goal+harness rewrite — choreography dies in place
- **Disposition:** superseded → the v8 goal+harness rewrite of this command (whole-file)
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/command-architecture-realignment/record.md` D1–D6; DECISIONS.md 2026-08-02 command-architecture row)
- **Content:** the entire v7-form file superseded — preamble dispatch-brief protocol · Seats & checks table + validation model · team-transport mandate + roster probe (D5: transport-neutral now) · seat lifecycle/recycling · every G-numbered gate, the run-start weight card, floor-gate set, counted bounds/caps/kill-switch, ordering invariants, ground-rules block · run-start declaration + departure trail + per-run contract file · KM-landing command steps · the Recovery section and resume table. Verbatim text below (pre-edit file at the v0.47.0 tree).
- **Kept deliberately:** the Goal's record+review-or-waiver+index+acceptance condition (re-worded, no gate names) · lead-inline questioning via analysis-iterative · lead-penned record always cold-graded or user-waived (Independence line) · survivor-challenging-user-ruling dispositions reserved to the user · the lead's-pen boundary (no amendment to a user-ruled decision, no new decision, without the user's word — restored at the audit fix round) · index/KM close ritual (Bindings) · synthesis on-request-only, stamped derived, with the waiver-path *derived, unchecked* stamp (restored at the audit fix round) · pipeline-entry-as-offer next step (restored at the audit fix round) · no-git-mutation + plain-blocking-acceptance lines · output-style register pointer. **Named drop within the synthesis rule:** the pre-ship fidelity sample-check by the still-seated verify reviewer dies with the review-sizing machinery (D2's drop-set) — the generic Independence line covers grading, and synthesis fidelity is the user's read at acceptance.
- **Consumers assessed:** none — commands are entry points, nothing mounts them.

<details><summary>Verbatim superseded file (v0.47.0)</summary>

````markdown
---
description: Think a problem through with the user and harden the record at the end — the session is just the lead and the user (plus a fact-checker teammate, seated from the start whenever the topic touches existing code, that maps the reality surface into the record and verifies claims against the files); at convergence the lead sizes the review at a named gate, under the weight card the user ruled at run start — a lens-split cold pair by default (independent reads, one four-message cross-examination, only survivors return for rulings), a single reviewer for lean records, or, on the user's recorded waiver alone, none. Deliverable is one decision record, plus a fidelity-checked synthesis on request after acceptance; pipeline entry is an offer, never a default. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Brainstorm — Think Together, Review Cold

**Goal:** think `$ARGUMENTS` through with the user and leave one hardened decision record
behind. Empty topic → ask what we are thinking through.

**You are the lead**: you compose the run and own its counters, every verdict, every
escalation, every human gate, and the user-facing conversation — agents produce and review,
you adjudicate. Every dispatch carries its own brief in the spawn or send prompt — the seat's
role and skill (named as a hint, the agent decides fit), the exact inputs to Read, where the
output lands (write vs return), the bar it must clear, its peer edges and holds, and the
independence reminder that matches the seat (author: never grade your own output; grader:
read the artifact itself, default FAIL, quote evidence) — the seat owns none of this context
and gets all of it from you. This file is self-contained: brainstorm's whole
contract lives here. You run the questioning inline via `mochiko:analysis-iterative` — one
question per turn, format adapted to the user's state. **First-spawn probe:** the
fact-checker at start where its seat fills, otherwise the reviewers at convergence.

## Goal

`.mochiko/brainstorms/<slug>/record.md` exists, each decision carrying statement + rationale +
confidence mark, and its Review section carrying the sizing ruling, **every survivor's
disposition**, and the verify outcome quoting the evidence the folds landed — or, in their
place, the recorded waiver; the tally is on the record even at zero survivors, which is
vacuously clean; the session's index entry names where the outcome landed; the KM close ritual
ran; and the user has accepted the record.

**Not done:** an unreviewed record with no recorded waiver · an undispositioned survivor · a
survivor dispositioned by a reviewer's status alone · folds with no verify outcome recorded · a
departure with no trail line · no user acceptance · a synthesis shipped without its fidelity
check.

## Seats & checks

| seat | agent × skill | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| fact-checker | a neutral empiricist, no skill mounted | produces the reality map, and settles the reviewers' fact disputes; reports what is, never argues what should be, volunteering file-grounded facts that cut either way; never grades the record | at start, conditional on the topic having a reality surface; **probe seat** when filled, its announcement naming that surface | you only, one send per fact; the reviewers' fact disputes reach it through you |
| reviewer(s) | `mochiko:devils-advocate` × `mochiko:review-brainstorm` in the **end-stage reviewer role**; a pair splits the hunt by lens — one **decision-quality**, one **record-integrity** | grades the frozen record; never authors it | cold at convergence only, never in the room before it; count per the sizing ruling | withheld from each other until findings are formed; one cross-exam |

**Validation model:** the sized end-stage review of `record.md`; there is no in-loop critique
seat. No seat ever grades its own output.

**Team transport:** check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` before anything else — unset
→ stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178); the first
spawn is the authoritative probe, and there is no teamless fallback. A seat is spawned with
**`name:`** — a nameless spawn is a one-shot subagent, the forbidden transport; every later
send is a `SendMessage` to that same named seat. Verify from the roster: the `members` array
in `~/.claude/teams/<team>/config.json` (`<team>` = `session-` + first eight chars of the
session ID) must carry the seat's `name` — absent ⇒ kill and respawn explicitly requesting an
agent team; failing again stops the run. Teammates don't load `skills:` frontmatter — every
spawn prompt names the skill and role itself. Tell the user up front they can watch or
message any teammate; announce each seat in one line when filled; never narrate or reply to
teammate housekeeping.

**Seat lifecycle:** the fact-checker meets the standing multi-unit criterion but counts no
loop unit — **cadence-exempt**, recycled only on the user's gate-time order. The reviewers
are cold end-stage seats, exempt by nature. A respawn is a reset: briefed from the on-disk
record alone, versioned successor name, never the dead seat's bare name. End-of-need
shutdown; no ritual sends.

## Constraints

- **Run-start weight card** — evidence: your stated read of the four rigor factors against this
  topic — **reversibility** (rework cost if the record is wrong) · **blast radius** (how much
  downstream work will read it as authoritative) · **precedent** (first-of-kind, or mirroring an
  audit-cleared pattern) · **input confidence** (scored on the artifact under review; a user
  ruling discounts ambiguity risk only, and one introducing new surface raises consistency
  risk) — plus the process you compose from it — the stated default below, or your departures
  from it · rules: the user · decides: the run's composed process. Rigor scales with
  cost-of-being-wrong, never task size.
- **Review sizing** *(at convergence)* — evidence: convergence signals — answers turning
  confirmatory, no new dimensions, the wrap confirmed with the user · rules: you, on your own
  weight statement (decision count · confidence-mark mix · reality-surface load), sizing under
  the user's weight card and never around it · decides: pair / single, a heavyweight record
  defaulting to the full pair and any size below that default costing one trail line. **None is
  not yours to take** — `record.md` is lead-penned, so shipping it uncold-read needs the user's
  recorded waiver at the weight card. A single reviewer gets the whole hunt surface and no
  cross-examination — its findings arrive undebated, the trade this gate priced.
- **Review protocol** — the record is **frozen** from reviewer spawn until every disposition
  lands (Review section excepted). Each reviewer reads it cold, forms findings independently,
  and reports findings-formed — count only — before its counterpart is introduced; a pair then
  runs the one-shot four-message cross-exam
  (`skills/review-brainstorm/references/CROSS-EXAM.md`, the pair protocol's single source —
  owner-withdrawal only, the counterpart persuades, never vetoes). Each reviewer returns its
  own survivors (severity, concrete failure scenario, resolution path, unresolved counterpart
  objections attached) and its own tally ("N raised, M survived"; fallen retrievable on ask)
  with a recommended status — **the cross-set merge and the combined tally are yours, never a
  reviewer's**. Fact disputes go to the fact route, never argument; a fact already routed is
  cited, never re-routed. An overruled survivor marks its element `Contested`; nobody
  re-raises it.
- **Survivor rulings** — evidence: a survivor in user territory — a challenge to a user
  ruling, or a user-declared fact offered as confirmation · rules: the user · decides: its
  disposition. Theirs to answer, not a tie-break.
- **Tie-break** — evidence: a lead↔reviewer argument unresolved at the two-exchange cap ·
  rules: the user, on both positions plus your recommendation · decides: the disposition, and
  whether the element marks `Contested`.
- **Acceptance** — evidence: every survivor dispositioned and the verify pass recorded, or the
  waiver · rules: the user · decides: done. Then offer, don't push: if the record is honestly
  the shape of a next stage (e.g. a feature description for `/mochiko:specify`), name it as an
  option and stop.
- **Floor gates:** the weight card · survivor rulings · tie-break · acceptance — each reading
  `rules: the user`, none of them yours to compose away. **Review sizing is the one lead-ruled
  gate here**, so it is deliberately out of the set: it sizes under the weight card the user
  already ruled, never around it. Survivor rulings and tie-break fire only when their evidence
  exists; that bounds when they open, never who rules them. **`record.md` is lead-penned, so it
  always takes the cold grade** — the sized review and the verify pass over your folds are
  non-discretionary wherever a review runs, and it ships with zero cold reads only on the user's
  recorded waiver at the weight card, never on your sizing.
- **Bounds:** per reviewer one cold read, plus (pair only) the one-shot four-message
  cross-exam, plus one verify pass; lead↔reviewer argument **max two exchanges per survivor**,
  you count them; one fact-checker dispatch per fact. No kill-switch and no no-progress exit —
  the human-attended session is the escalation surface, not a substitute for the caps. Any
  bound this run declares — including a declared cost range — has you as its named counter,
  **rises only at a user checkpoint**, and is re-declared only on the record; busting a bound
  escalates, never silently continues.
- **Invariants:** **no standing challenger** — beyond these two seats the conversation is you
  and the user: the v2 standing episodic advocate generated 3:1 machine-to-user traffic and
  folded amendments into user-ruled decisions without consent. A **reality surface** is existing
  code, docs, or a system under redesign; that call at the start fills or leaves empty the
  fact-checker seat. The checker's map lands **verbatim**, a checker-authored section you write
  around and never restate — the first completed run's headline finding was an over-claim
  living in the lead's paraphrase of the map, not in the map. Both reviewer briefs name that
  map as the fact substrate: reviewers do **not** re-read the reality surface it already covers
  (the first completed run read it three times over — the pair's dominant cost); the lens split
  lives in those briefs, never in the skill, which stays one document. The **synthesis is
  on request only, after acceptance** — never auto-generated; write it beside the record
  stamped ***derived — record canonical***, and before it ships the reviewer that ran the
  verify pass — still seated — sample-checks its fidelity (every ruling present, no confidence
  mark inflated, no rejected alternative resurrected). Under a waiver it is stamped
  **"derived, unchecked"** instead — the same recorded-absence discipline as the waiver.
  Governance context is native: the CLAUDE.md governance region loads with the session; read
  `.mochiko/memory/governance-ledger.md` only when a decision needs waiver or amendment
  detail — never a blocking gate. `KEPT:` the no-fallback transport bet stays `Contested`.
- **Ground rules:** kernel-free — no brain code, no capability catalogs, no DAG-mediated
  orchestration. Suggest commits; never run git mutations, never push. No internal machinery
  vocabulary in user-facing prose — the conversation is yours and the user's, in the mochiko
  register (`templates/output-style.md`). User acceptance is plain blocking text, never a
  timed prompt. The record is written as the session progresses, never reconstructed at the
  end; it reads standalone as the review surface — review findings and dispositions live in
  its closing Review section, never interleaved — and your pen covers your own formulation
  only: nothing amends a user-ruled decision, and no new decision exists, without the user's
  word. Every departure from the stated default is one trail line — by record, never by
  silence — and rulings batch into the fewest checkpoints that respect the floor gates.

## Bindings

- **Artifacts:** `.mochiko/brainstorms/<slug>/record.md` in one decision namespace (D1…) — the
  deliverable, kept in place at acceptance; a conditional `synthesis.md` beside it. Derive the
  kebab-case `<slug>` at the start.
- **Uncertainty carrier:** the lead-penned record.
- **Fact route:** the fact-checker seat; an `Explore` subagent when it is unfilled, or for a
  one-off fetch with no standing-perspective value.
- **Verify-pass owner:** the record-integrity reviewer, or the sole reviewer in single mode.
- **Run-start declaration:** one line on `record.md`'s `Status` line — the surface Recovery
  already keeps — for a default run; a departing run, or one declaring non-default bounds,
  writes a departure record at
  `.mochiko/brainstorms/<slug>/brainstorm-contract.md` beside the record instead: the
  done-condition and bounds as (re-)declared, departures taken, and the counter state Recovery
  reads on resume. Counted unit: the
  lead↔reviewer **exchange per survivor**, the bound you count; the cold reads, the cross-exam
  and the verify pass are one-shot.
- **Departure trail:** one line per departure under that same declaration as it is taken,
  part of what the user accepts — a review sized below the default included.
- **KM landing:** `.mochiko/brainstorms/index.md` is the session index — read it before
  opening, enter this session on open (status: open), and at acceptance or supersession update
  it with where the outcome landed (a `DECISIONS.md` row, or an explicit no-graduation). Run
  the open and close invariants — at close, the subtractive landing ritual — from the
  project-pinned `.mochiko/memory/knowledge-management.md` under fix-on-sight. No index and no
  module → skip; the layer was declined.

## Recovery

No resume table — the record is the whole state. Note resume state on its `Status` line, with
the run's counter state — exchanges consumed · bounds declared · departures taken; a departing
run's counter state lives in `brainstorm-contract.md` beside the record instead — read it
there on resume. Sessions
and teams do not survive `/resume`; resume from the workspace, never a context `phase` field:
re-read `record.md` and continue from the last decision or the survivor queue, respawning the
fact-checker mid-session or the reviewers per the sizing ruling (the frozen record makes a
cold re-read cheap, and a respawn is cold by design).
````

</details>

---
## [v0.46.0] Doctrine-purge rewrite — obligated reads out, shape mechanics inlined
- **Disposition:** superseded → the command's own text
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** the preamble's obligated shape/loop-discipline reads and "in the mochiko command shape" framing left.
- **Kept deliberately:** all gates/bounds/bindings/recovery (incl. the sizing-gate read and the `sized-end-stage-review.md` trail-line deferral) — plus inlined weight-card factors, bound-integrity rule, transport, lifecycle, ground rules, as-you-go lead-pen rules, counter-state recovery.
- **Consumers assessed:** none.

---
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

## [v0.44.0] v2-revision evidence citation (standing-advocate rule)
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(`.mochiko/brainstorms/brainstorm-v2-revision/record.md`)
```
- **Kept deliberately:** the dogfood finding it evidenced, stated in the rule itself — 3:1 machine-to-user traffic and amendments folded into user-ruled decisions without consent.

## [v0.44.0] v2-2-revision evidence citation (lens-split rule)
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
(Both rules'
  evidence: `.mochiko/brainstorms/brainstorm-v2-2-revision/record.md`.)
```
- **Kept deliberately:** both rules — the three-reads cost finding and the lens split living in the briefs, not the skill.

## [v0.44.0] KEPT survivor's evidence pointer (no-fallback transport bet)
- **Disposition:** superseded → the pointer lives here; the `KEPT:` marker and its claim stay in
  the command per the amended P9.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above), executed under the lead's stage-A ruling (option (a)).
- **Content (verbatim, the pointer only):**
```
, its
  provenance this command's own v2 design record
  (`.mochiko/brainstorms/brainstorm-command-rewrite/record.md`, D9).
```
- **Kept deliberately:** `KEPT:` plus the survivor's claim — the no-fallback transport bet stays
  `Contested`. The audit still sees a marked survivor; only its evidence moved.
- **Tier-2 evidence (the pointer, preserved for the verify-path):** this command's own v2 design
  record, `.mochiko/brainstorms/brainstorm-command-rewrite/record.md`, D9.

## [v0.43.0] The `<!-- shape-form: v7 -->` marker retired from the preamble
- **Disposition:** superseded → deleted. The marker was added by this same version's conversion
  entry below and retires in the same version, at the wave close.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-01 wave-close
  ratifications row, *shape-form marker retirement when the last command converts*; the trigger
  was written into the marker clause itself). **Ground and full record:**
  `.mochiko/strips/command-shape.md` [v0.43.0 wave close], entry 1 — *The form marker and its
  Conformance bullet retired* — not restated here.
- **Content (verbatim):** `<!-- shape-form: v7 -->`
- **Kept deliberately:** the entire preamble otherwise — goal line, obligated reads, probe seat —
  and every P18–P20 binding the marker used to gate. The slots bind unconditionally now; nothing
  the marker declared was lost, because the marker declared only which grading branch to take, and
  there is one branch.
- **Consumers assessed:** `validation-command-shape` check 20 was the sole grep consumer and its
  form branch retired in the same ceremony. All six commands swept together — a marker left in any
  one of them would be the only file in the library still declaring a form.
- **Measured:** `commands/brainstorm.md` **10,632 → 10,607 B** (−25). Derived figures in this note's
  conversion section re-measured accordingly, superseded values kept inline.

# v0.43.0 — the v6→v7 conversion

**Wave context:** shape **v7** landed at v0.40.0 (`lead-owned-process-flexibility`,
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`; `DECISIONS.md` 2026-08-01),
with **D4** ruling **convert-on-touch** and all six commands staying v6-form. `implement`
converted first in the dedicated v0.43.0 wave and cleared its independent audit; the user then
**widened that wave to all six** (2026-08-01), so brainstorm converts here against the cleared
precedent rather than on a touch of its own. BACKLOG: "convert-on-touch residuals". The two
check-6 v7 ceiling terms were calibrated and landed at the precedent conversion
(`.mochiko/strips/validation-command-shape.md` [v0.43.0]) — this conversion **fits inside both
and re-keys neither**.

**Post-conversion measurement, all blocks, body-only in words** (`##` heading lines excluded and
the `#` title line counted in the preamble, per check 6 as the precedent measured it — *this
note's own [v0.35.0] preamble figure of 107 was measured on the same title-included convention,
verified at commit `b32dd82`, where the block reads 107 title-included / 100 title-excluded;
107 − 5 for the parenthetical the [v0.37.0] entry removed = the 102 v6 baseline below*):
preamble **102/130** (was 102 — the conversion's +4 was the form marker, retired at the wave close) · Goal **123/150** (was 115) · Seats & checks **187/250**
(unchanged) · Constraints **714/750** (was 511) · Bindings **230/254** (was 139) · Recovery
**50/60** (unchanged). Term derivation as check 6 requires: **G = 4 → 5** — the four prior gate
lines (review sizing · survivor rulings · tie-break · acceptance) plus the run-start weight
card, all five carrying the complete three-part `evidence:`/`rules:`/`decides:` form — so
Constraints is 90·(5+2) = 630 **plus the +120 P18 term** = 750. **S = 2** and **R = 0**, both
unchanged. **A = 2**, unchanged (`record.md` · the conditional `synthesis.md`; the session index
is a KM-landing fold target, not an output), so Bindings is 90 + 12·2 + 30 (KM) **plus the +110
P19/P20 term** = 254. Bytes: **8,577 → 10,607** (+2,030 B, +23.7%). Constraints carries the
U4 sizing flip below; the P18 bullet measures **122 w** and the block's residual headroom is
**36 w (4.8%)** — under the ceiling, and stated here because it is the tightest block in the
file and the next editor should know it before adding to it.

> **A-term judgment, following the precedent's recorded one.** P19 names
> `brainstorm-contract.md` as a **departing** run's per-run carrier. It is **not counted in A** —
> neither a deliverable nor a round report, and it exists only on a departing run. Counting it
> (A = 3) would raise the Bindings ceiling to 266 and only loosen the check; the conservative
> reading is the one measured here (`.mochiko/strips/implement.md` [v0.43.0]).

> **The sizing-gate ownership question — raised by this seat, ruled by the wave lead, EXECUTED
> here.** `templates/sized-end-stage-review.md` **v2** moved sizing ownership user → lead by
> recorded supersession (U4), and its interim note says an unconverted command's user-ruled
> sizing "stands, as written, in those commands **until their conversion touch**". This is that
> touch. The author's first pass left the **Review sizing** line verbatim at `rules: the user`
> and escalated rather than deciding — flipping it removes a user gate from the floor set, the
> unsafe direction, on a `DECISIONS.md`-traceable line (Brainstorm v2.2 row). **The wave lead
> ruled: execute the flip at this touch** — an already-user-ratified supersession whose named
> landing site is exactly this conversion, and leaving it un-executed strands the template's
> interim note pointing at a touch that already happened. The flip is landed and logged as its
> own supersession entry below; brainstorm's floor set is consequently **four of five**, with
> the sizing line named in P18 as the one lead-ruled gate. The template's other conversion-touch
> obligation was discharged in the same pass: P20 names the home for a below-default sizing
> trail line. **This paragraph is the escalation's provenance — who raised it, who ruled it —
> kept because the first pass's reasoning is what the ruling answered.**

## [v0.43.0] The Goal's end state loses "the sized review ran" and the verify-pass clause

- **Disposition:** superseded → rewritten in place as artifact state. What the clauses proved
  **survives as what the record carries** — its Review section holding the sizing ruling, every
  survivor's disposition, and the verify outcome quoting the evidence the folds landed.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, ratified at **A4**, 2026-08-01:
  *"Goal blocks lose process residue. Done = artifact state + floor compliance + user
  acceptance"*; graded by `validation-command-shape` check 23, v7-form only).
- **Content (v6, verbatim — the clause that left):**
  ```
  the sized review ran per the user's ruling with **every survivor
  dispositioned** and the verify
  pass confirming the folds landed — or the ruling was **none** and the waiver is recorded
  ```
- **Protected traces checked at source before removing anything:** the v0.35.0 CS-D8 ledger
  homes the Brainstorm v2.2 sized-review row ("weight-statement inputs · heavyweight→pair
  default · none→waiver") at the **Review sizing gate line**, not at the Goal, so this entry
  supersedes only the Goal's echo of it and the row's home stands. *(That home was itself edited
  later in this same wave by the lead-ruled U4 flip — its own entry below. All three of the
  row's parts survive there: the weight-statement inputs, the heavyweight→pair default, and the
  none→waiver path. What changed is who rules the size, not the keying.)* The waiver's Goal consequence is a `Kept deliberately` survivor
  of the [v0.35.0] *waiver's sole-validator clause* entry; it is **reworded, not dropped**
  (below).
- **Kept deliberately:**
  - **Every survivor dispositioned** — in substance verbatim, re-read as the Review section's
    content rather than as the review's own event.
  - **The waiver's Goal consequence** — "or the ruling was **none** and the waiver is recorded"
    → "or, in their place, the recorded waiver". The `none`→waiver *trigger* survives on the
    Review sizing gate line, and the waiver is floor-anchored at P18 as the one zero-cold-read
    path — after the U4 flip below, ruled by the user at the weight card rather than at the
    sizing gate, which is what makes the anchoring load-bearing rather than decorative.
  - **The per-decision standard** (statement + rationale + confidence mark) — untouched, the
    [v0.35.0] uncertainty-carrier entry's assigned home.
  - **"the KM close ritual ran" and "the user has accepted the record"** — both are explicit
    end-state elements in the shape's own Goal spec, so neither reads as residue.
- **Consumers assessed:** not a shared primitive. Cross-file consumers checked: the grader's
  check 23 (this is the text class it was written for) and the four sibling conversions running
  in parallel this wave, each owning its own Goal block — no text is shared between them.

## [v0.43.0] The zero-survivor tally rule re-read from reporting to artifact state

- **Disposition:** superseded → rewritten in place. The same rule, named by what the record
  carries rather than by what the lead reports.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above).
- **Content (v6, verbatim → v7):** `Zero survivors is vacuously clean — the tally is still
  reported` → `the tally is on the record even at zero survivors, which is vacuously clean`
- **Protected content:** `DECISIONS.md`-traceable (Brainstorm v2.2 tally rule; the v0.35.0
  CS-D8 ledger row homes it at "**Goal**, final clause"). The home survives in the converted
  Goal, so the row is **preserved** and this entry records a rewording, not a supersession of
  the row.
- **Kept deliberately:** both halves — the vacuous-clean reading and the still-reported tally.
- **Consumers assessed:** as above.

## [v0.43.0] Two not-done states re-read from process to artifact state

- **Disposition:** superseded → rewritten in place. The same states, named by the artifact that
  is missing or wrong rather than by the step that did not run.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above).
- **Content (v6, verbatim → v7):**
  - `an unrun verify pass` → `folds with no verify outcome recorded`
  - `a reviewer's status taken as the disposition without your read` → `a survivor dispositioned
    by a reviewer's status alone`
- **Protected content:** neither is `DECISIONS.md`-traceable to a Goal home. The second is a
  **v0.35.0 addition** ("a **not-done state** for a reviewer's status taken as the disposition
  without the lead's read", logged in that wave's *Additions* list), so no protected row is
  touched; only its lead-read choreography — the residue class check 23 fails — left, and the
  guard it carried is preserved as artifact state. The first re-anchors on the verify pass,
  which **hardened to floor** at U1-B/D6(c) and is now stated as such at P18.
- **Kept deliberately:** the lead's ownership of the merge and the verdict, which the deleted
  half asserted — homed at `templates/sized-end-stage-review.md` ("**The cross-set merge and the
  combined tally are the lead's, never a reviewer's**", read this run to confirm) and in
  `review-brainstorm`'s own description ("the cross-set merge and clearing verdict are
  lead-owned"); the seat table's "grades the frozen record; never authors it" is untouched. The
  four remaining not-done states are unedited.
- **Consumers assessed:** as above.

## [v0.43.0] Review sizing passes user → lead (U4), and `none` passes to the user's waiver

- **Disposition:** superseded → the Review sizing gate line survives **as the stated default's
  carrier** with its `rules:` clause flipped to the lead, and the gate leaves the P18 floor set.
- **Tier failed:** n/a — supersession by ruling. Two citations, both required: **U4**
  (`.mochiko/brainstorms/lead-owned-process-flexibility/record.md` — "review sizing passes to
  the lead **by recorded supersession** of the brainstorm-v2-2 ruling; the supersession is logged
  at the landing under the primitive-edit ceremony", ratified with the set at **A4**, 2026-08-01)
  and **`templates/sized-end-stage-review.md` v2's interim note**, which defers the edit to each
  command's conversion touch — "that stands, as written, in those commands **until their
  conversion touch**". This is brainstorm's touch, so the deferral is spent here. Executed on the
  **wave lead's explicit ruling** after this seat escalated it rather than deciding alone (the
  provenance paragraph above).
- **Protected content, leaving by ruling and named as such:** the `rules: the user` clause is
  `DECISIONS.md`-traceable — the Brainstorm v2.2 row (the sized lens-split review), whose
  v0.35.0 CS-D8 ledger entry reads "Sized review: weight-statement inputs · heavyweight→pair
  default · none→waiver → **Review sizing** gate line, all three parts + the default keying P7
  requires". Only the *owner* of the sizing call leaves; all three parts stay.
- **Content (v6, verbatim — the clause that left):**
  ```
  rules: the user, on your
  weight statement (decision count · confidence-mark mix · reality-surface load) · decides:
  pair / single / none, a heavyweight record defaulting to the full pair; **none** records a
  waiver.
  ```
- **Kept deliberately:**
  - **The default keying** — "a heavyweight record defaulting to the full pair" — untouched, and
    still what P7 requires a review-sizing line to carry. The lead sizes *against* that stated
    default; it did not become discretionary by changing hands.
  - **The weight-statement inputs**, all three (decision count · confidence-mark mix ·
    reality-surface load) — now the lead's own statement rather than its submission to the user.
  - **The `none`→waiver path** — kept, and *hardened*: `none` is no longer available to whoever
    rules the gate. `record.md` is lead-penned, so per the template's own clause ("On a
    **lead-penned** artifact `none` is not the lead's to take") a zero-cold-read run needs the
    user's recorded waiver at the weight card. The flip therefore moves the sizing call to the
    lead **without** moving the waiver: U1-B's protection is strictly the same as before.
  - **The single-reviewer trade clause** — "its findings arrive undebated, the trade this gate
    priced" — untouched.
  - **A user gate, not lost but relocated:** the run-start weight card (P7, added this wave)
    carries the composed process including the review, so the user still rules the *shape* of
    the review at run start; what the lead gained is the size under it.
- **Ripple, executed in the same pass so the file does not self-contradict:** the frontmatter
  `description:` ("at convergence **the user sizes** the review at a named gate" → the lead
  sizes, under the user's weight card, with `none` on the user's recorded waiver alone) · **P18**
  drops review sizing, the floor set becoming **four of five** with the sizing line named as the
  one lead-ruled gate (check 21(1) requires every marked gate to read `rules: the user`, and the
  four that remain do) · the below-default sizing trail line binds at **P20**, which the
  template's interim note names as its home at the conversion touch.
- **Consumers assessed:** `sized-end-stage-review.md`'s interim note is the shared surface. Its
  deferral is now **spent for brainstorm** — the note's "unconverted command wins for its own
  run" tie-break no longer reaches this file. `setup` is the other command binding this branch
  and is executing the same flip in this same wave, in its own seat; the note itself is
  untouched here (a shared-template edit is not this seat's to make, and its retirement belongs
  to the wave close that retires the form marker). The other four commands do not bind P6's
  sized branch.

*Pure additions this wave, riding the decision row rather than these entries:*

- **The form marker** `<!-- shape-form: v7 -->` in the preamble — check 20's branch key.
- **The run-start weight-card gate line** (P7) — U1-A's standing user stop, in the three-part
  countable form, taking **G from 4 to 5**.
- **`**Floor gates:**`** (P18) — the floor set is **four of five**: the weight card · survivor
  rulings · tie-break · acceptance, each reading `rules: the user`. **Review sizing is named as
  the one lead-ruled gate and deliberately excluded**, so the non-floor case is *stated* rather
  than inferred — the shape of the exclusion the precedent's fix round asked for, and the U4
  flip above is what created it (at v6 the fact map recorded brainstorm as 4 user-ruled / 0
  lead-ruled). Survivor rulings and tie-break are scoped as **conditional in firing, not in who
  rules** — the precedent's lesson that blocking and floor are independent axes. Plus the
  **always-cold-graded lead-penned surface** (U1-B, check 21(2)): `record.md` is lead-penned, so
  the sized review and the verify pass over the lead's folds are non-discretionary wherever a
  review runs, and zero cold reads ship only on the user's recorded waiver at the weight card —
  never on the lead's sizing, which is precisely why the flip costs the floor nothing.
- **`**Run-start declaration:**`** + **`**Departure trail:**`** (P19/P20) — the declaration on
  `record.md`'s `Status` line for a default run (the surface Recovery already keeps, so the
  counter state and the resume state are one surface, which is why Recovery needed no edit and
  stays at 50/60), an instantiated `templates/workflow-contract.md` as
  `.mochiko/brainstorms/<slug>/brainstorm-contract.md` for a departing one; the departure trail
  under that same declaration, part of what the user accepts. **Counted unit: the lead↔reviewer
  exchange per survivor** — the one bound the file already tells the lead to count ("max two
  exchanges per survivor, you count them"); the cold reads, the cross-exam and the verify pass
  are one-shot, which is why no seat accrues a recycle denominator and the P17 cadence-exempt
  line stays accurate as written.
- **A not-done state** `a departure with no trail line` — invariant 4's Goal carrier, per the
  precedent.

*Judgment considered and deliberately not written:* the review **freeze**
(`sized-end-stage-review.md`: the artifact is frozen from reviewer spawn until every disposition
lands, Review section excepted) versus a departure line landing on the `Status` line mid-review.
No carve-out clause was invented — the file already directs `Status`-line writes during a run
(Recovery's resume note), so the coexistence is the file's settled practice and not a
contradiction this conversion introduces. Flagged for the audit rather than resolved by
command-local doctrine.

---

## [v0.37.0] `@`-reference drop-bug attribution removed — the bug is resolved
- **Disposition:** superseded → user ruling (2026-08-01). Only the bug-cause parenthetical retires; the empty-topic ask is fully kept.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/decisions/2026-08-01-at-reference-recovery-superseded.md`; `DECISIONS.md` 2026-08-01).
- **Content (superseded, verbatim):** the parenthetical "(the known `@`-reference drop bug)" inside the Goal's "Empty topic (the known `@`-reference drop bug) → ask what we are thinking through."
- **Kept deliberately:** the empty-topic ask — "Empty topic → ask what we are thinking through." brainstorm has no feature/most-recent fallback, so an empty topic must ask regardless of the bug; only the attribution is gone.
- **Consumers assessed:** five-command recovery — see the shared consumer list in the `strips/plan.md` v0.37.0 entry.
- **Protected-set note:** as recorded in the plan entry — record §7's protection premise for this recovery is spent now the bug is resolved; deliberate supersession, not a check-14 re-drop.

# v0.35.0 — the goal-shape rebuild wave (CS-D10 step 4)

**Wave context:** command goal-shape rebuild, **step 4 of 4** — the five-command wave after the
audit-PASSed plan pilot (design: `.mochiko/brainstorms/command-succinctness-strip/record.md`,
rulings CS-D1–D10; shape home `templates/command-shape.md` v5). brainstorm is the anatomy's ruled
**floor case** — 0 numbered gates, 0 resume rows, ~77% slot-bound — and one of only **two**
commands that bind the sized end-stage review, so it is also the case that exercises the v5
conditional read and the conditional-block rule together.

**Baseline provenance — read this before auditing the ledger.** The working tree held a rewrite by
a since-stopped seat executing a superseded instruction; the wave lead ruled that draft **unowned**
and directed this authoring to base on `HEAD`. **This ledger is derived from `HEAD` (the
authoritative 47-line, 1,376-word baseline), not from that draft** — every one of HEAD's 47 lines
was walked clause by clause against the delivered file. Disclosed for the grader's benefit: that
draft *was* read once, before the collision was discovered, so it is not a blind-independent
authoring; no line was taken from it, and the two files diverge materially (it dropped the
`no standing challenger` phrasing, the `subtractive landing ritual` and `fix-on-sight` KM clauses,
and the no-fallback provenance — all four of which survive here).

**Mechanical backstop for the dropped-content class** (the pilot's named failure mode, and the
sibling wave's method): every backticked token in HEAD was diffed against the delivered file.
**22 in HEAD, 25 delivered, 3 absent — each an accounted relocation, none a loss:**
`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` (the hard-require prose relocated to shape Layer 2 at
v0.11.0; the env var itself survives in `description:`, where check 1's team-form grep finds it) ·
`FAIL` (shape v5's Goal block states "Initial state is **FAIL**" as doctrine — restating it would
be the altitude failure; the command binds the *not-done states*, which is P4) · bare
`analysis-iterative` (deduped against the namespaced `mochiko:analysis-iterative`, which the
preamble carries — the same namespace-prefix convention the pilot set). The 6 new tokens are the
two newly-bound evidence pointers, the conditional read, the `KEPT:` marker the strip README
mandates, and `Explore`/`Status` gaining code formatting.

## [v0.35.0] The flow body, the seat bullets, and the Contract section retired into the five-block anatomy

- **Disposition:** superseded → the file's own v5 blocks (Goal · Seats & checks · Constraints ·
  Bindings · Recovery)
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/command-succinctness-strip/record.md`, CS-D3 · CS-D4 · CS-D5; executed
  per CS-D10 step 4)
- **Content:** the v4 headings `## Team-form parameters (shape Layer 2)` · `## Session
  parameters` · `## The seats` · `## Convergence — this workflow's review bindings` ·
  `## Done-condition and acceptance` · `## Contract (authoring-time fill)`, and the narrative
  prose that carried them. The Contract appendix is dissolved rather than moved: its four
  bullets are now the document — done-condition → **Goal**, producer↔validator → the **Seats &
  checks** table, bounds → **Constraints** bounds, human gates → **Constraints** gate lines. Per
  shape v5 no per-run contract file is written; brainstorm's values are constant at authoring
  time.
- **Kept deliberately:** every routing decision and trigger the retired prose carried — see the
  survivor ledger below. brainstorm is the anatomy's ruled **floor case**: **0 numbered gates**
  (its four gate lines carry plain names; what makes a gate is the three parts, not a number)
  and **0 resume rows**, so Recovery is one-lined with its absence *stated* ("No resume table —
  the record is the whole state"), never left to inference.
- **Measured:** 1,376 w / 9,547 B → **1,222 w / 8,452 B** (−154 w, −11.2%; −1,095 B, −11.5%),
  `wc`-measured on the landed file after the final fix round, per the pilot's standing habit.
  Against the measured floor of **872 w / 5,994 B** (`.mochiko/strips/command-shape.md`, the
  per-command parameter-floor table, where brainstorm is one of the two *measured* rows):
  **+350 w (+40.1%), deliberately over.** That note's own ruling is that landing materially
  *under* a row is as much a finding as landing over — under means content was dropped — and
  brainstorm is the thinnest command, least able to amortize the anatomy's ≈440-word fixed cost
  (its finding 1). Per-block against the grader's check-6 ceilings at **G = 4 · S = 2 · A = 2 ·
  R = 0**: preamble 107/130 · Goal 115/150 · Seats 165/190 · Constraints 511/540 · Bindings
  139/144 · Recovery 50/60 — every block under, tightest Bindings at 3.5%, which is the headroom
  the Bindings `+30` KM term exists to provide.
- **Wrap:** unwrapped (203 chars/line average, longest prose line 1,076) → prose wrapped at
  ≤98, matching `commands/plan.md` (mode 95–98) and the templates (96–103); table rows and the
  frontmatter `description:` are exempt. The step-4 briefing's "~80" was read as the loose form
  of this house standard, with the audit-PASSed pilot as the binding precedent.

### Slot map — every v4 parameter to its v5 home

| v4 parameter | v5 home |
|---|---|
| goal line + empty-topic `@`-reference branch | preamble (P1) |
| hard-require env var · probe-seat keying · no-fallback `Contested` provenance | frontmatter `description:` (env var) · preamble (P2) · Constraints `KEPT:` survivor (P9) |
| `analysis-iterative` questioning method, one question per turn | preamble |
| record path · decision namespace · per-decision standard | Goal (P3) + Bindings (P10) |
| done-condition + the not-done states | Goal (P3/P4) |
| the two seat descriptions | Seats & checks rows (P5) |
| the sized-end-stage-review binding | validation-model line (P6) + the conditional read in the preamble |
| sizing · survivor rulings · tie-break · acceptance | Constraints gate lines (P7) |
| review / verify / argument / fact caps | Constraints bounds (P8) |
| verbatim map · reviewers-don't-re-read · synthesis · governance-native · reality surface · no standing challenger | Constraints invariants (P9) |
| uncertainty carrier | Bindings (P11) |
| fact route | Bindings (P12) |
| verify-pass owner | Bindings (P13) |
| index bookkeeping + KM invariants | Bindings KM landing (P10 + the `+30` ceiling term) |
| pause posture | Recovery (P15); **P16 vacuous — absence stated** |
| `## Contract` block | dissolved into the document, as above |

P14 (clearing unit + checkpoint keying) does not bind: brainstorm has no devolved branch — its
only teammate-to-teammate surface is the cold convergence review, and every verdict is judgment.

## [v0.35.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling) protects two sets: `KEPT:`/Tier-2-evidenced lines **and** every
line traceable to a `DECISIONS.md` row. brainstorm's prior strip notes carry **no live `KEPT:`
entry** — all seven v0.11.0 entries are relocations — so the protected set is derived from the v4
text against the `DECISIONS.md` rows that rule in this file (brainstorm v1 → v2 → v2.1 → v2.2,
fact-checker-seat, constitution-native-surfaces, OD-D6/D7/D9, model-tiered-seats,
command-altitude). **43 protected rules enumerated; all 43 resolved — 39 translated into the new
blocks, 4 relocated to a confirmed home. Zero dropped.**

Per the step-4 instruction the **compressed-evidence clauses were grepped, not trusted** — the
pilot lost content inside lines that still read as complete. brainstorm's four multi-clause
survivors were checked clause by clause and are intact: the **verbatim-map** rule keeps its
over-claim evidence · the **reviewers-don't-re-read** rule keeps its three-times-read cost
evidence · the **synthesis** rule keeps all three fidelity criteria *and* both stamps (`derived —
record canonical` and the waiver's `derived, unchecked`) · the **KM landing** keeps the ritual,
the invariants, the project-pinned copy, and the null exit.

**Two evidence pointers are newly bound that the v4 file never had** (P9 mandates the pointer,
and the same gap was found in the specify sibling): the verbatim-map and don't-re-read rules now
cite `.mochiko/brainstorms/brainstorm-v2-2-revision/record.md`, whose **F9** is the actual home
of the over-claim finding — "the record's fact-map section was *lead prose* summarizing the
checker's map; the over-claim lived in the paraphrase and survived until end-stage review caught
it." The v4 file asserted this evidence without citing it. Read this run to confirm the home
holds the content.

| protected line | source | resolved |
|---|---|---|
| No standing challenger — the v2 standing episodic advocate's 3:1 machine-to-user traffic and unconsented amendment folds | Brainstorm v2.1 row (2026-07-05); `brainstorm-v2-revision/record.md` F1 + D2 | **Constraints invariants**, evidence intact. Re-homed from the validation-model line where v4 kept it: P6 is "one line naming which validation branch", P9 is where an out-of-scope ruling with its evidence belongs |
| The checker's map lands **verbatim** — a checker-authored section the lead writes around, never restates | Brainstorm v2.2 row; `brainstorm-v2-2-revision/record.md` F9/M1 | **Constraints invariants** + the evidence pointer newly bound |
| Reviewers do **not** re-read the reality surface the map covers (the pair's dominant cost) | Brainstorm v2.2 row, same record | **Constraints invariants**, cost evidence intact |
| Sized review: weight-statement inputs · heavyweight→pair default · none→waiver | Brainstorm v2.2 row (the sized lens-split review) | **Review sizing** gate line, all three parts + the default keying P7 requires |
| A single reviewer gets the whole hunt surface, no cross-examination — findings undebated, the trade the gate priced | Brainstorm v2.2 row | **Review sizing** gate line, closing clause |
| Reviewers cold at convergence only, never in the room before it | Brainstorm v2.1 row; shape Layer 2 (cold = a property of the stage) | Seat table's **spawn** cell |
| Pair splits by lens — decision-quality / record-integrity; the skill stays one document | Brainstorm v2.2 row | Seat table's **agent × skill** cell + the invariant placing the split in the briefs |
| Survivors in user territory route to the user — theirs to answer, not a tie-break | Brainstorm v2.1 row's territory ruling ("nothing amends a user-ruled decision without the user's word") | **Survivor rulings** gate line |
| Tie-break at the two-exchange cap; an overruled element marks `Contested` | Brainstorm v2.1/v2.2 rows | **Tie-break** gate line |
| Zero survivors is vacuously clean — the tally is still reported | current body (the v2.2 tally rule) | **Goal**, final clause |
| Synthesis on request only, after acceptance; never auto-generated; `derived — record canonical`; fidelity sample-check by the still-seated verify-pass reviewer; waiver → `derived, unchecked` | Brainstorm v2.2 row | **Constraints invariants**, all five clauses — grepped, not assumed |
| Fact-checker: neutral empiricist, no skill mounted; reports what is, never argues what should be; volunteers facts that cut either way; settles reviewers' fact disputes | Fact-checker-seat row (2026-07-05) F1 | Seat table's **agent × skill** and **produces / grades** cells |
| The seat is **conditional** on a reality surface, and "conditional" governs whether it exists, never when it spawns — it spawns at start | Fact-checker-seat row **F2** (the v4 command's own wording primed the misread) | Seat table's **spawn** cell ("at start, conditional on…") + the reality-surface invariant |
| One-off fact-fetches with no standing-perspective value go to `Explore` | Fact-checker-seat row; model-tiered-seats row (the cheap-explorer avenue) | Bindings' **Fact route** |
| Hard-require agent teams, **no fallback transport** (`Contested` dogfood-pilot bet) | Brainstorm v2 row (2026-07-04), `brainstorm-command-rewrite/record.md` D9 | `description:` declaration + `command-shape.md` Layer 2 + a `KEPT:` provenance pointer — see the survivor-provenance entry below |
| Governance is native: the CLAUDE.md region loads with the session; the ledger is read only for waiver/amendment detail, never a blocking gate | constitution-native-surfaces row (D1–D8) | **Constraints invariants** |
| Index bookkeeping — read before opening, entry on open, update at acceptance or supersession naming where the outcome landed | OD-D6/D7 + the KM module's command carrier | Bindings' **KM landing** |
| Open/close invariants + the **subtractive landing ritual** from the **project copy** `.mochiko/memory/knowledge-management.md`, under fix-on-sight | OD-D6 (subtractive landing) + the CS step-1 adjudication making the project-copy reference mandatory in KM-carrying commands | Bindings' **KM landing**; check 1's KM member greps the project path, and it is the project path |
| No index and no module → skip without ceremony (the layer was declined) | OD-D9 (the module is elective) | Bindings' **KM landing**, closing clause |
| The `@`-reference recovery — empty `$ARGUMENTS` has a named cause and a prompt | command-altitude row (its retrofit-regression warning names this class); the class the pilot dropped and had restored under audit | **Preamble** goal line, both halves. Grepped, not assumed — the pilot's named failure mode |
| Uncertainty rides the **lead-penned record** (the shape's lead-penned branch, not producer-authored) | current body (P11) | Bindings' **Uncertainty carrier** |
| Verify-pass owner = the record-integrity reviewer, or the sole reviewer in single mode | Brainstorm v2.2 row | Bindings' **Verify-pass owner** (P13) |
| Bounds — one cold read per reviewer + the one-shot four-message cross-exam + one verify pass; max two exchanges per survivor, lead-counted; one dispatch per fact; the attended session is the escalation surface, not a substitute for the caps | Brainstorm v2.1/v2.2 rows | **Constraints** bounds, with the kill-switch and no-progress-exit absence now *stated* rather than merely absent |
| The record **stays in place** at acceptance; pipeline entry is an **offer, never a default** | Brainstorm v2.2 row + `description:` | **Acceptance** gate line |
| Recovery — the record is the whole state; resume from its `Status` line; the frozen record makes a cold re-read cheap | current body | **Recovery**, one-lined with the stated absence |

**Additions, logged rather than folded silently** (pure additions ride the decision row; these
are within-command precision, not doctrine): the conditional read of
`templates/sized-end-stage-review.md` **at the sizing gate** rather than up front (shape v5
pilot-checkpoint ruling B — brainstorm is one of only two commands that may load it, and pays
+2,992 B/run for the branch it binds, knowingly); a **not-done state** for a reviewer's status
taken as the disposition without the lead's read; and the two evidence pointers named above.

**Retained, not dropped:** the obligated `mochiko:loop-discipline` read, per shape v5's
transition note — its drop is deferred to a named live-run trigger, so omitting it here would be
non-conformant rather than early.

## [v0.35.0] KEPT: the no-fallback transport bet stays `Contested`, provenance in this command's own v2 design record

- **Tier-2 evidence:** the bet is brainstorm's own — `brainstorm-command-rewrite/record.md`
  **D9** ("Flag-off posture: hard require; no fallback transport — `Contested`"), where the
  rationale is recorded as the user's, overriding the recommendation: "a fallback means some runs
  silently test the wrong thing; hard-require guarantees every run exercises the actual
  hypothesis." The v0.11.0 strip entry that relocated the transport *mechanics* ruled explicitly
  that "the command keeps: the env var, its probe-seat parameter, and the `Contested` provenance
  pointer" — this entry is that ruling still standing at v5.
- **Contested keep — flagged for the grader.** The specify sibling resolved the *same*
  `DECISIONS.md` row this wave **without** a command-side pointer (`description:` declaration +
  `command-shape.md` Layer 2), and shape v5 Layer 2 now carries both the no-fallback rule and its
  `Contested` mark, which v0.11.0's shape did not. The divergence is deliberate and narrow: what
  the shape does **not** carry is *which record made the bet*, and brainstorm is the command that
  made it — specify inherits it and has no origin claim. Cost: ~20 words, ~4% of the Constraints
  ceiling. A grader ruling this redundant with the 2026-07-04 DECISIONS row would be making a
  reasonable call, and the honest fix is one deletion.

## [v0.35.0] Reviewer lens-brief detail returned to the reviewing skill

- **Disposition:** relocated → `skills/review-brainstorm/SKILL.md` (its line 12 carries the lens
  parentheticals; the five hunt classes are its §2, record fitness its
  `references/RECORD-FITNESS.md`, the cross-exam its `references/CROSS-EXAM.md`)
- **Tier failed:** 1 (altitude — the command transcribed the reviewer's own procedure)
- **Content:** "(scenario stress, hunt classes 1–4, rejected-road steelmans)" and
  "(inconsistencies, the fitness checklist, reality-grounding as a sample audit of the map)".
  Read the home this run to confirm: `SKILL.md:12` reads "*decision-quality* (scenario stress,
  hunt classes 1–4, rejected-road steelmans) or *record-integrity* (inconsistencies, record
  fitness, the map audit)" — the command was restating text the skill already owned.
- **Kept deliberately:** the genuinely caller-side half — that a pair splits by
  **decision-quality** / **record-integrity** lens, that the split is set **in the spawn briefs**
  and not in the skill, and that the skill stays **one document**. Those are the lead's dispatch
  parameters, not the reviewer's procedure.

## [v0.35.0] Uncertainty-carrier mechanics

- **Disposition:** relocated → `templates/command-shape.md` (the anatomy's **Bindings** block,
  P11, which states that a lead-penned record carries statement + rationale + a confidence mark
  per element "with user corrections and reversals logged where they happen")
- **Tier failed:** 1 (altitude — restated shape prose)
- **Content:** "user corrections and reversals logged where they happen" as a command-side
  clause. The command now binds only the *carrier* ("the lead-penned record") and leaves the
  carrier's contract to the shape.
- **Kept deliberately:** the per-decision standard (statement + rationale + confidence mark)
  survives in **Goal**, where it is the measurable end state check 13 grades rather than a
  restatement of the carrier's definition.

## [v0.35.0] The waiver's sole-validator clause

- **Disposition:** relocated → `templates/sized-end-stage-review.md` v1 ("**None** → a review
  waiver in the artifact's Review section (who waived, at which gate, why): the validator seat
  passes to the user alone, deliberately and auditably")
- **Tier failed:** 1 (altitude — relocated into the conditional read this command binds and loads
  at the sizing gate, so the clause reaches the lead exactly when it applies)
- **Content:** the v4 Contract bullet "Under a waiver the user is the sole validator — recorded,
  never silent."
- **Kept deliberately:** the waiver's *trigger* on the **Review sizing** gate line ("**none**
  records a waiver"), its consequence in the **Goal** ("or the ruling was **none** and the waiver
  is recorded"), and the matching recorded-absence discipline on the synthesis stamp.

## [v0.35.0] Fact-checker rename note ("né grounder")

- **Disposition:** relocated → the `DECISIONS.md` row 2026-07-05 ("Fact-checker seat kept +
  renamed (né grounder)") and its record `.mochiko/brainstorms/fact-checker-seat/record.md`
- **Tier failed:** 1 (altitude — completed-rename provenance carried in a live supervisor file)
- **Content:** "(The design records call this seat the *grounder* — renamed 2026-07-05 after the
  name misread as a findings-tester.)" No live surface still says *grounder*; the decisions index
  is the provenance carrier, and the note had become reader-facing history.
- **Kept deliberately:** the substance the rename protected — the seat's actual job, stated
  positively in the Seats row ("reports what is, never argues what should be"), which is what the
  misread got wrong. Per that record's **F2** the v4 command's *own wording* primed the misread,
  so the fix is precise phrasing in the seat cell, not a historical footnote.
- **Contested call — flagged for the grader:** this is the one `DECISIONS.md`-traceable line
  removed rather than translated into the new blocks.

## [v0.11.0] Hard-requirement transport mechanics
- **Disposition:** relocated → `templates/command-shape.md` (Layer 2, Hard requirement) + `templates/agent-dispatch.md` (Seat transport — already the mechanics' single source; the command's restatement removed)
- **Tier failed:** 1 (altitude — restated shape/transport prose)
- **Content:** the env-check-as-proxy prose, the no-fallback rationale body, the `name:`-discriminator sentence ("a spawn without a `name:` is a one-shot subagent, the forbidden form"), and the post-spawn addressability instruction. The command keeps: the env var, its probe-seat parameter, and the `Contested` provenance pointer.

## [v0.11.0] Never-narrate-machinery + housekeeping constraint
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, production surface; Layer 2, seat legibility)
- **Tier failed:** 1
- **Content:** "Never narrate machinery — no 'phase', 'round', or 'gate' talk; teammate housekeeping (idle notifications, acks) is never narrated and never replied to."

## [v0.11.0] Lead's-pen boundary
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, as-you-go artifact)
- **Tier failed:** 1
- **Content:** "Your pen covers only your own formulation. Nothing amends a user-ruled decision and no new decision is created without the user's word."

## [v0.11.0] Watch/message note + seat-announcement rule
- **Disposition:** relocated → `templates/command-shape.md` (Layer 2, seat legibility)
- **Tier failed:** 1
- **Content:** "Tell the user at the start that they can watch or message any teammate directly … announce it in one line … An unexplained teammate spawn reads as a malfunction (first fresh-run lesson, 2026-07-05)." The fact-checker-specific announcement content (reality surface + map-arriving) stays as a session parameter.

## [v0.11.0] Team preamble — teammates don't load `skills:` frontmatter
- **Disposition:** relocated → `templates/command-shape.md` (Layer 2, seats)
- **Tier failed:** 1
- **Content:** "Teammates do not load `skills:` frontmatter — every spawn prompt must name the skill and role itself, plus the topic and what to Read."

## [v0.11.0] Review machinery generics (sizing gate · freeze · withheld counterparts · cross-exam sequence · survivor routing · dispositions · bound)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, sized end-stage review); the four-message sequence transcription → already single-sourced at `review-brainstorm/references/CROSS-EXAM.md`, transcription removed
- **Tier failed:** 1
- **Content:** the sizing-gate mechanics (weight statement → recommend pair/single/none → user rules → waiver on none), the record-frozen-during-review rule, the withheld-counterpart / findings-formed-count protocol, the "one-shot — four messages, no more: a→b findings · b→a findings + attacks · …" transcription, owner-withdrawal-only, the three answer-owner routing bullets + `Contested`-on-overrule, the resolved/user-ruled/recorded-open vocabulary, and the review+verify bound with escalation. The command keeps its bindings: weight-statement inputs, heavyweight→pair keying, fact-dispute route (fact-checker), verify-pass owner (record-integrity / sole reviewer).

## [v0.11.0] Acceptance-gate phrasing
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, contract clause)
- **Tier failed:** 1
- **Content:** "— plain blocking text, never a timed prompt."

## [v0.11.0] Recovery preamble
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, recovery)
- **Tier failed:** 1
- **Content:** "Teams do not survive `/resume`, and a shared account limit can throttle the team and the main session together — escalation then has nowhere to go but pause."
