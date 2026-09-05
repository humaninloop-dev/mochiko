# Strip notes — `skills/authoring-architecture-store`

Entry formats: `strips/README.md`. Skill born at v0.81.0 (the product-architecture-schema Stage-1
wave, D3/D4/D7 — it retires `authoring-architecture`, whose own strip history stays at
`strips/authoring-architecture.md`); this file opens with the first edit that superseded any of
its shipped text.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the authoring family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/authoring-architecture-store/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before any store touch: **Read `schema.yaml` (this skill's own
  directory) and `../../schemas/skill-authoring-common.yaml` raw, in full, in the same
  declared first action** — schema, then common. The schema is the source of truth for this
  skill's binding rules, nested in six sections, each addressable by its section ID:
  `authoring-architecture-store.sec.independence` · `authoring-architecture-store.sec.scope` ·
  `authoring-architecture-store.sec.inputs` ·
  `authoring-architecture-store.sec.artifact` · `authoring-architecture-store.sec.output` ·
  `authoring-architecture-store.sec.reserved`. Interpret it live: a rule's `kind:` names what
  it is, and an absent `kind:` reads `constraint`; a rule carrying `when:` binds only where
  its terms hold against the schema's declared `conditions:`, except that a `class: floor`
  rule is always read and always delivered — `when:` gates when its obligation applies, never
  whether it reaches you; a `pointer:` rule binds you to that file's or skill's procedure,
  referenced never restated; `${var}` substitutes from this schema's `vars:` at read time;
  labels come from `plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
  `extends: authoring-common.<slug>` inherits text/labels/pointer from
  `skill-authoring-common.yaml` only — `class` and every absence-meaningful field are local —
  and the stub's `authoring-architecture-store.*` ID stays the citable ID. The floor pin: the
  9 rules of `class: floor` are non-waivable. Before the first store-touching step, state the
  floor count back — a skipped or partial read leaves that count blank: halt and surface it,
  and halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The `extends:` stub resolution
  and the family common-file co-Read are discharged by the render, which resolves every stub
  before the model sees it.
- **Consumers assessed:** the family common file
  `plugins/mochiko/schemas/skill-authoring-common.yaml` is unchanged and still bound by every
  unconverted consumer; nothing shared leaves. The block was this skill's own text.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin: the
  9 rules of `class: floor` are non-waivable. Before the first store-touching step, state the
  floor count back — a skipped or partial read leaves that count blank: halt and surface it,
  and halt likewise if the schema's `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.101.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2A, authoring family)

Ruling for every entry below: skill-content-schema D3 (three-home boundary) / D8/C4
(protected transfers), `DECISIONS.md` 2026-09-01 rows (Skill-content schema ruled ·
Skill-schema wave-2 family doors ruled — the authoring-family door); census:
`.mochiko/brainstorms/skill-content-schema/census-authoring.md` §A (AAS) + §B (AAS).
Schema home: `plugins/mochiko/skills/authoring-architecture-store/schema.yaml`. Minted IDs
carry the `authoring-architecture-store.` prefix (omitted below). Map — census §B row →
minted ID:
1 `one-home-one-writer` · 2a `diagram-craft-routing` · 2b `stance-routing` ·
2c `mint-routing` · 3 `store-layout` · 4 `scope-line-durable-home` · 5a `element-grammar` ·
5b `two-arm-schema-binding` (C-A3 stub, `extends: authoring-common.two-arm-template`,
`${template}` = `architecture-store`, both arms inherited verbatim — GI-020) ·
6 `nfr-one-home` · 7 `work-pointers-only` · 8 `present-tense-no-history` ·
9 `lifecycle-statuses` · 10 `sign-off-is-write-gate` · 11 `index-regenerated-every-write` ·
12 `full-ax-table` · 13 `health-view-binding` · 14 `readability-bar` ·
15 `graduation-by-depth` · 16 `landing-diff-on-delta` (`when: {delta: present}`) ·
17a `diff-both-directions` · 17b `diff-user-dispositioned` · 18 `fold-duty`
(`when: {structure_built: present}`) · 19 `drift-probe-scoped` ·
20a `as-built-against-code` · 20b `drift-user-disposition` · 21 `orphan-rule` ·
22a `first-visit-bootstrap` (`when: {ruled_content: absent}`) ·
22b `bootstrap-confirmation-gate` · 23 `sound-loop-on-judgment-writes`.
**Floor-count correction (build annotation, wave-lead ruled):** the census §B tally line
reads "Floors 8 · musts 21", but the §B table's own class column marks **9** floor rows
(1 · 10 · 11 · 16 · 17a · 17b · 20a · 20b · 23); built 9 per the table (the wave-1
"VC mints 24" census-arithmetic idiom) — SKILL.md floor pin = 9. Lands in the census
build-corrections appendix at the landing.
**Deleted as dedup, no content loss:** the `## Quality checks` checklist (12 boxes — each a
mirror of a mapped rule above, per the census §B preamble's dedup-note posture) and the
`## When NOT to Use` section (rows 2a–2c, mapped above). The body keeps overview, layout
table, lifecycle diagram, landing-duties table, drift/orphan teaching, and the first-visit
walkthrough as prose; obligation sentences are excised where their rule now lives in the
schema.
**Fix-round folds (V1 minors, disclosed):** the index content-set's spine-thumbnail
element folded into `full-ax-table`'s text, and the health-view glosses for counts 1, 2,
and 5 (open rows = walked, no stance formed · stale `not-now` = revisit condition gone
unreviewed · drift register = rows whose `Drift:` field names a live divergence) folded
into `health-view-binding`'s text — no element of the index content set or the five-count
view is dropped.
Accounting: body 10,884 → 4,904 (−5,980, obligations out + the load-first Rules block in)
+ schema 13,972 = **payload 18,876** (census §F estimate was ~24,750; figure includes the
V1 fix-round folds above); the delta over the
pre-conversion body is structural overhead (IDs, keys, section scaffolding, reading
grammar) — no content growth claimed. AAS was unbudgeted at birth (hard-cap-only); the
conversion re-seed is its first budget, via the ledger's third seeding path, no headroom
(census J-7 — P5 executes the ledger row).

## [v0.101.0] Write gate — protection transfers (census §A row 1; product-architecture-schema D10, kept byte-for-byte at v0.91.0)
- **Disposition:** superseded — protection transfers to schema rule `authoring-architecture-store.sign-off-is-write-gate` (`class: floor`, `kind: gate`), per D8/C4; the provenance sidecar carries the protected status.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema rows).
- **Content:** "**Ruled truth is never edited in place by a delivery run.** A design-time write is legal only as an in-flight-class delta, and only after the user's sign-off on the rendered diagram plus the named row changes — the sign-off IS the write gate. No sign-off, no store write."
- **Consumers assessed:** `schemas/architecture-store.yaml` carries the parallel gate language (untouched); `mochiko:patterns-system-design` drafts the delta this gate governs (untouched).

## [v0.101.0] Full-AX-table + named readers — protection transfers (census §A row 2; D4 + v0.91.0)
- **Disposition:** superseded — protection transfers to `authoring-architecture-store.full-ax-table` (must), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "the **full** AX summary table (every row — the sufficiency check and the design phase read the trip check here, so a missing row is an invisible row)"
- **Consumers assessed:** `schemas/architecture-store.yaml`:130 carries the same sentence (untouched, re-keyed identically at v0.91.0).

## [v0.101.0] NFR one-home + business-source chain — protection transfers (census §A row 3; D12 + wave-lead R4)
- **Disposition:** superseded — protection transfers to `authoring-architecture-store.nfr-one-home` (must), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "`NFR-XXX` targets live **on the concern row they belong to** — one home per concern, stance and pattern and target and as-built together. The ids survive unchanged, and each target names its business source, so `FR-XXX / SC-XXX → NFR-XXX` trace chains keep resolving."
- **Consumers assessed:** `mochiko:authoring-technical-requirements` owns the NFR grammar (untouched this entry; its own pair is P3's this wave); `review-plan-artifacts` re-keyed at v0.91.0 (untouched here).

## [v0.101.0] Single-writer derived index — protection transfers (census §A row 4; D4)
- **Disposition:** superseded — protection transfers to `authoring-architecture-store.index-regenerated-every-write` (floor) + `authoring-architecture-store.one-home-one-writer` (floor), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "The root `ARCHITECTURE.md` is a **projection of the store, not a second store**. Regenerate it on **every** store write — this skill is its single writer. It is never hand-maintained, and index-vs-store disagreement is a defect fixed by regenerating, never by editing the index." · "**One home. One writer.**"
- **Consumers assessed:** the router's row and `/mochiko:architecture` name the single-writer index generically — still literally true.

## [v0.101.0] Landing diff / fold trigger split — protection transfers (census §A row 5; D10 six-step lifecycle)
- **Disposition:** superseded — protection transfers to `authoring-architecture-store.landing-diff-on-delta` (floor, `when: {delta: present}` — criterion-11 MOVE of the fires-when guard), `.diff-both-directions` (floor), `.diff-user-dispositioned` (floor, reservation), and `.fold-duty` (must, `when: {structure_built: present}`), per D8/C4. The two-trigger table stays in the body as teaching.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "**Never gate the diff on whether structure was built.**" · "The diff runs **both directions** … Grading built topology against the signed delta is a capability **this duty owns**, taking that delta as its input" · "The diff **reports**: each difference is named, approved-versus-built, and dispositioned by the user. It never silently reconciles." · "The fold itself: flip in-flight-class elements to `built`, clear their FEAT-XXX keys, update `As-built:` and `Drift:` on every touched row, regenerate the index."
- **Consumers assessed:** `implement.md`'s landing steps reference the duties by skill, never restated (untouched).

## [v0.101.0] Orphan rule — protection transfers (census §A row 6; D10, supersedes AT-D6-C)
- **Disposition:** superseded — protection transfers to `authoring-architecture-store.orphan-rule` (must), per D8/C4; the replacement-for-a-pointer-list rationale stays in the body as teaching.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "**Every in-flight-class element keys an open feature.** An element whose `FEAT-XXX` names a closed, retired, or nonexistent feature is an orphan: the health view flags it, and desk visits clean it."
- **Consumers assessed:** the health view's count 4 names the orphan rule — carried by `health-view-binding` in the same schema.

## [v0.101.0] First-visit reconstruct-and-confirm + archive — protection transfers (census §A row 7; D16 migration)
- **Disposition:** superseded — protection transfers to `authoring-architecture-store.first-visit-bootstrap` (must, `when: {ruled_content: absent}`) + `.bootstrap-confirmation-gate` (must, reservation), per D8/C4; the numbered walkthrough stays in the body as procedure.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "Never fails, never silently invents." · "marking every reconstructed element as such — derived, not ruled" · "**Confirm** with the user before the store becomes truth … the confirmation is a real gate, not a formality" · "**Archive** the absorbed sources to `.mochiko/archive/product-baselines/<date>/`. Nothing is silently discarded." · "A pipeline run that meets a store with no ruled content — scaffold-only or absent — **offers the bootstrap** rather than failing."
- **Consumers assessed:** `/mochiko:architecture`'s first-visit path references the bootstrap by skill (untouched).

## [v0.101.0] Sound-loop review leg — protection transfers (census §A row 8; patterns-sound-loop floor pointer)
- **Disposition:** superseded — protection transfers to `authoring-architecture-store.sound-loop-on-judgment-writes` (floor, `kind: binding`, pointer `mochiko:patterns-sound-loop`), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "Judgment writes — baseline authoring, shelf-walk stance batches, amendments, and every `As-built:`/`Drift:` write — take the independent review leg before the user's ruling, per `mochiko:patterns-sound-loop`. Status flips and orphan cleanup are transcription and ride the landing audit. Do not relabel judgment as mechanical to skip the leg."
- **Consumers assessed:** `mochiko:patterns-sound-loop` is the floor's single source — pointed at, never restated.

## [v0.101.0] Two-arm schema citation — protection transfers to C-A3 stub (census §A row 9; template-schema D8, GI-020)
- **Disposition:** superseded — protection transfers to `authoring-architecture-store.two-arm-schema-binding` (`extends: authoring-common.two-arm-template`, must, `kind: binding`, `${template}` = `architecture-store`), per D8/C4; both arms preserved verbatim through the inherited block text (GI-020 — raw Read the first-class degraded path).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows).
- **Content:** "The shape is schema-backed: invoke `mochiko-cli template architecture-store`, or Read `plugins/mochiko/schemas/architecture-store.yaml` raw when the binary is absent."
- **Consumers assessed:** `plugins/mochiko/schemas/skill-authoring-common.yaml` (P1's, this wave) carries the block; `schemas/architecture-store.yaml` untouched.

## [v0.101.0] Remaining body obligations relocated (census §B rows 2a–2c · 3 · 4 · 5a · 7 · 8 · 9 · 13 · 14 · 15 · 19 · 20a · 20b)
- **Disposition:** relocated → `plugins/mochiko/skills/authoring-architecture-store/schema.yaml`, per the map entry above (D3).
- **Tier failed:** n/a — supersession by ruling (D3; `DECISIONS.md` 2026-09-01 rows).
- **Content (decisive line per row):** 2a "This skill says where a delta lands and what status it carries, never how to draw it" · 2b "This skill owns the field; that skill owns the judgment that fills it" · 2c "The architecture lens proposes; the map machinery disposes" · 3 the store-layout table's path set + "The root index stays at the repo root, never inside the store directory" · 4 "This is shelf scope's durable home: the shelf walk reads it rather than re-asking" · 5a "the schema constrains the skeleton, never the voice" · 7 "`Work:` holds pointers only" · 8 "**Present tense, no history.** … link it, never restate it" · 9 "Each MUST name the feature that owns the change" + "stance is a separate axis and never implies one" · 13 "The health view is a section of this index and **no separate artifact exists**. Five counts, each naming its rows" · 14 "a reader new to the repo can place any file or component in the system from the index alone" · 19 "The probe is **scoped** … Never all rows every visit" · 20a "`As-built:` claims are checked against **actual code**, never against recollection" · 20b "Findings … take a **user disposition** at the desk … The probe reports; it never silently reconciles."
- **Consumers assessed:** none restate these rules; the router row describes the skill generically and stays true.

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
