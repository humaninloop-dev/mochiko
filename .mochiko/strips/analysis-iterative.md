# Strip notes — `skills/analysis-iterative/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 1 (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, batch-ratified 2026-07-25): body
228 → 192 lines, 36 cut = 16% — **under the 30–70 never-stripped band**; per R3 the under-band
second pass generates the survivor-provenance (KEPT) entries below rather than forcing cuts.

<!-- Wave context: wave 1 of the brainstorm target-state build (v0.117.0). Ruling: `.mochiko/brainstorms/brainstorm-target-state/record.md`
(`DECISIONS.md` 2026-10-06 row) — D21 for the two repairs; D19 as changed at review (S5) and at
verify (V4) for the scoped supersession. The skill's behaviour for setup and specify is unchanged.
Pre-edit verbatim text: `git show 2e4c57c:plugins/mochiko/skills/analysis-iterative/SKILL.md`. -->

<!-- Wave context: wave 1 of the combined engine/desk build (v0.120.0) — the skill converts to a
schema-bearing pair and becomes the shared thinking stage. Ruling for every [v0.120.0] entry
below: `.mochiko/brainstorms/implement-brainstorm-led/record.md` D1-engine-asking-contract,
D2-iterative-becomes-engine, D3-four-move-now and D4-shaped-for-six as review-amended, with the
defaults *What analysis-iterative keeps*, *Precedence* and *Supersessions owed at the build*
(`DECISIONS.md` 2026-10-11 "Shared thinking engine ruled" row); wave plan
`.mochiko/brainstorms/implement-brainstorm-led/wave1-engine.md` §3.8. D2 reverses the 2026-09-01
"stay prose" ruling for this skill (`DECISIONS.md` 2026-09-01 skill-schema wave-2 row). The rules
land by `0058-thinking-engine.yaml` on the review six-set; the `## Rules — delivered by
mochiko-cli` block and the `allowed-tools` grant are pure additions riding D2. Pre-edit verbatim
text: `git show 5162f1a:plugins/mochiko/skills/analysis-iterative/<file>`. -->

## [v0.120.0] The enrichment variant — `SPECIFICATION-INPUT.md`, `ENRICHMENT.md` and `### Specification-input enrichment` — stripped

- **Disposition:** superseded → none in this skill: the variant had no caller (record F27). A
  sparse idea is thought through before a spec exists in `/mochiko:specify`'s intent and story
  stages, which run on this skill (`review-specifications.not-for`, reworded by
  `0062-spec-review-boundary.yaml`).
- **Tier failed:** n/a — supersession by ruling (`implement-brainstorm-led`
  D2-iterative-becomes-engine: "the Who / Problem / Value enrichment variant
  (`SPECIFICATION-INPUT.md`, `ENRICHMENT.md`, and the *Two output shapes* section) has no caller
  (F27) and is stripped, recorded")
- **Content (faithfully compressed; verbatim at `git show 5162f1a:` each path):**
  - `SPECIFICATION-INPUT.md` (4,763 characters) — "A focused variant of the iterative-analysis
    engine for conditioning a sparse feature description into a specification-ready brief."
    *When this variant fits* (the caller decides sparseness and names the missing triad
    elements; the hand-off is caller-side) · *Question Agenda* — Part 1, triad gaps, conditional:
    the WHO, PROBLEM and VALUE question templates, three options and "My Recommendation" each;
    Part 2, always asked: the SCOPE (out of scope for v1) and SUCCESS templates · *Output* (the
    `ENRICHMENT.md` template) · *Completion* ("do NOT ask any follow-up questions"; the enriched
    description is the deliverable) · *Key Differences from General Analysis* (questions · depth
    2–5 · output · purpose).
  - `ENRICHMENT.md` (2,403 characters) — the *Enriched Feature Description* template (Actor ·
    Problem · Value · Out of Scope · Success Criteria · Summary · Original Input, verbatim), the
    "Add dark mode" worked example, seven field guidelines.
  - `SKILL.md` `### Specification-input enrichment`, verbatim: "When the work is conditioning a
    sparse feature description before a specification is authored — the description is thin on
    the **Who / Problem / Value** triad — run the focused variant in
    [SPECIFICATION-INPUT.md](SPECIFICATION-INPUT.md). It fills the missing triad elements plus
    scope and success, and concludes with the [ENRICHMENT.md](ENRICHMENT.md) artifact, handed to
    whoever authors the specification next (the requirements producer). ¶ The enriched
    description is plain output: whoever requested the enrichment reads the artifact directly.
    How a caller asks for this shape — and tells you which triad elements it already knows are
    missing — is a caller-side dispatch concern, carried in the caller's own brief, not
    something this skill parses or owns."
- **Kept deliberately:** `SYNTHESIS.md` (the deliverable for a caller that names none,
  `analysis-iterative.deliverable`) and `references/ADAPTIVE-EXAMPLES.md`, both unchanged.
- **Consumers assessed:** `grep -rn -i 'triad\|Who / Problem\|SPECIFICATION-INPUT\|ENRICHMENT.md\|enrich'`
  over `plugins/mochiko` (this skill's directory and `migrations/` excluded) — before: only
  `review-specifications/SKILL.md:3` and `:11` (re-keyed this wave, its [v0.120.0] entry) and the
  `not-for` rule (0062); after: none. `crates/`, `evals/contract/`, `scripts/`, the hooks: no
  reference to either file. `brainstorm`, `specify`, `setup`, the router and the agenda name no
  part of the variant.

## [v0.120.0] `## Two output shapes, one engine` — the dispatch seam superseded by the shared stage

- **Disposition:** superseded → one asking for every caller, the rendered rules
  (`analysis-iterative.sec.*`, `0058-thinking-engine.yaml`); the concluding artifact is the
  caller's (`analysis-iterative.deliverable` — the calling command's deliverable, else
  `SYNTHESIS.md`), so there is no shape left to dispatch between.
- **Tier failed:** n/a — supersession by ruling (`implement-brainstorm-led`
  D2-iterative-becomes-engine — the *Two output shapes* section stripped; *Supersessions owed at
  the build*: "the protected `[v0.25.0] KEPT:` entry and v0.63.0 keep-set naming `## Two output
  shapes, one engine` the caller-facing dispatch seam")
- **Content:** verbatim, with the horizontal rule above it (the enrichment subsection is quoted
  in the entry above) —

  ```
  ---

  ## Two output shapes, one engine

  The adaptive questioning the Overview describes is the single questioning engine. It carries two output shapes; the engine is identical and only the agenda and the concluding artifact differ. Pick the shape that matches the work in front of you.

  ### General analysis (default)

  Open exploration — brainstorming, trade-off analysis, thinking a problem through. Converges when the decisions are settled and concludes with a synthesis document. Use [SYNTHESIS.md](SYNTHESIS.md) for the output.

  ### Specification-input enrichment
  [the entry above]
  ```
- **Kept deliberately:** the "single questioning engine" claim, now the skill's whole job (the
  Overview names the shared thinking stage); General analysis's "converges when the decisions are
  settled and concludes with a synthesis document" — carried by `analysis-iterative.stop-rule`
  and `analysis-iterative.deliverable`.
- **Protected-content reconciliation (MANDATORY):**
  - **`[v0.25.0] KEPT: confidence-signals table, asymmetry principle, two-output-shapes dispatch`**
    — its last surviving limb, the two-output-shapes dispatch ("survives intact" at [v0.63.0]),
    is **superseded by this ruling**; the entry's KEPT status ends whole. Its signals table and
    asymmetry principle left at [v0.63.0], verbatim in that entry's three homes.
  - **`[v0.25.0] KEPT: the four question-format templates + ratification-streak doctrine`** —
    superseded at [v0.63.0]; its intent rode the Common Mistakes rows since. This wave carries
    the streak as a rule by ruling — `analysis-iterative.ratified-streak` (D1's "the ratified-yes
    streak flag") — and the recommend-then-arbitrate shape through `analysis-iterative.options-rule`
    and `question-form` (a pick with the case against). Not a RETURNED re-add: D1 states the
    obligation fresh; the four format templates stay out.
  - **[v0.63.0] guardrails keep-set**, member by member: the goal/output contract (Overview +
    Output's synthesis-doc contract) → the Overview rewritten (its entry below) and the contract
    relocated into `analysis-iterative.deliverable` (the `## Output` entry); the non-waivable
    floor → `analysis-iterative.open-unknowns`, `class: floor`; the anti-patterns (`## Common
    Mistakes`, `## When NOT to Use`) → kept, rows reworded (their entries); the caller-facing
    dispatch seam → superseded here; the `references/ADAPTIVE-EXAMPLES.md` pointer → kept (the
    Reference entry).
- **Consumers assessed (shared skill):** `brainstorm` (`brainstorm.thinking-stage`,
  `brainstorm.lead-inline-questioning`), `specify` (`spec.thinking-stage`, `spec.story-stage`,
  `spec.intent-stage-first`), `setup` (`setup.interrogation-inline`,
  `setup.coverage-survivor-routing`), `review-specifications` (`not-for`, `description:`, opening
  line), the router (`SKILL.md:66`, `:124`), `INTERROGATION-AGENDA.md` (`:5`, `:166`). None
  dispatches to a shape by name: grep "output shape|general analysis|specification-input|two
  shapes" over `plugins/mochiko`, `.mochiko/schema-views`, `README.md`, `ARCHITECTURE.md` outside
  this skill finds no consumer.

## [v0.120.0] `## Output` — the deliverable and floor lines relocated into the rules; the table kept under `## Confidence marks`

- **Disposition:** superseded → schema content, `0058-thinking-engine.yaml`: the deliverable
  line → `analysis-iterative.deliverable` (`kind: binding`, `pointer: SYNTHESIS.md`); the floor
  line → `analysis-iterative.open-unknowns` (`class: floor`). The confidence-indicators table stays
  in the body under `## Confidence marks`.
- **Tier failed:** n/a — supersession by ruling (`implement-brainstorm-led`
  D2-iterative-becomes-engine — the skill's rules rendered by `mochiko-cli`; a supersession-transfer
  of protected lines into schema content, skill-content-schema D8-skill-governance-envelope/C4)
- **Content:** verbatim —

  ```
  ## Output

  Conclude with the deliverable your calling command names; when it names none, generate the synthesis document using [SYNTHESIS.md](SYNTHESIS.md).

  **Floor (non-waivable)** — surface every elicited unknown as an open question in the produced artifacts; a vague zone the principal could not resolve is never silently omitted.

  **Confidence indicators** — assign based on conversation observation:
  ```
- **Kept deliberately:** the table, byte for byte, under the new heading, its lead-in shortened
  to "Assign based on conversation observation:". The floor line's meaning stands verbatim in the
  rule (its "**Floor (non-waivable)** —" label is now `class: floor`); the deliverable line's
  `[SYNTHESIS.md](SYNTHESIS.md)` link is now the rule's `pointer:`. Both lines are protected — the
  [v0.63.0] floor-line addition and the keep-set's synthesis-doc contract as narrowed at
  [v0.117.0] — and keep that status through their rule anchors (`open-unknowns`:
  `2026-08-10 validator-scope-and-verbosity`; `deliverable`: the `brainstorm-target-state`
  D21-questioning-skill-repairs anchor, 2026-10-06), leaving only by recorded supersession-by-ruling (skill-pair criterion 12).
- **Consumers assessed:** `setup.synthesis-artifact`, `brainstorm.deliverable-record` and
  specify's Goal (the Intent section) each name their deliverable, as at [v0.117.0]; a direct
  think-through keeps `SYNTHESIS.md`.

## [v0.120.0] Title and `## Overview` — rewritten around the shared thinking stage and its callers

- **Disposition:** superseded → `# Iterative Analysis — the Shared Thinking Stage` and the
  two-paragraph Overview: "The shared thinking stage: a question thought through with the user and
  turned into rulings — the asking, and, where a caller switches it on, the ruled-set contract
  that keeps the next writer inside what the user ruled. Each question builds on the previous
  answer; style, depth and output adapt to the complexity that emerges, and each question's
  format to the user's state — confident, uncertain, or somewhere in between. ¶ Its callers
  extend it with their domain part in their own rules: `/mochiko:brainstorm`'s deciding,
  `/mochiko:specify`'s story stage, the `/mochiko:architecture` desk's focus stage and
  `/mochiko:implement`'s design stage; `/mochiko:setup`'s interrogation and specify's intent stage
  reach it through their own pointers, the contract off."
- **Tier failed:** n/a — supersession by ruling (`implement-brainstorm-led`
  D2-iterative-becomes-engine — "the existing questioning skill is the one that becomes the
  shared stage"; D1-engine-asking-contract — each command "extends the engine with its own domain
  expertise"; D3-four-move-now and D4-shaped-for-six as changed at review, S4)
- **Content:** verbatim — "# Iterative Analysis" and "Guide collaborative thinking by adapting
  questioning style, depth, and output to the complexity that emerges during conversation. Each
  question builds on the previous answer, and the format of each question adapts to the user's
  current state — confident, uncertain, or somewhere in between. Conclude with a structured
  synthesis document whose depth matches the conversation's depth."
- **Kept deliberately:** "each question builds on the previous answer", the adaptation of style,
  depth and output to the emerging complexity, and of each question's format to the user's state
  (obligation map row P1, `kept`). The last sentence moves to `analysis-iterative.deliverable`
  and the "Padding the synthesis" row ("Output depth matches conversation depth"). Protected —
  the [v0.63.0] keep-set's goal/output contract — reconciled in the `## Two output shapes` entry.
- **Consumers assessed:** none quotes the Overview (grep "Guide collaborative" over
  `plugins/mochiko`: this file only).

## [v0.120.0] `## When NOT to Use` — the implementation-details row and the spec-review parenthetical reworded

- **Disposition:** superseded → (a) "**Build-time implementation details** — decomposing and
  coding a card is the builder's (`mochiko:executing-tdd-cycle`); a design choice the user rules,
  as in `/mochiko:implement`'s design stage, runs here"; (b) the Specification review row's
  parenthetical → "(this skill rules what gets written, never grades what was)".
- **Tier failed:** n/a — supersession by ruling (`implement-brainstorm-led` *Touch set for the
  build*, added at verify, V5: "the *When NOT to Use* line '**Implementation details** — use
  planning skills instead' (F21) rewritten, since implement's design phase now runs the skill";
  D3-four-move-now; for (b) D2-iterative-becomes-engine — the parenthetical named the stripped
  enrichment shape, and the desk's and implement's stages run after a spec exists)
- **Content:** verbatim — "- **Implementation details** — use planning skills instead" and
  "(this skill conditions raw input *before* a spec exists)".
- **Kept deliberately:** the other three rows byte for byte (quick clarifications · clear
  direction · time-sensitive) and the Specification review row's route to
  `mochiko:review-specifications` — the [v0.63.0] keep-set's anti-patterns (obligation map P2,
  `reworded`).
- **Consumers assessed:** no primitive quotes a row (grep "use planning skills instead",
  "conditions raw input" over `plugins/mochiko`: this file only).

## [v0.120.0] `## Common Mistakes` — two rows reworded to agree with the engine; the [v0.117.0] brainstorm-only supersession ends

- **Disposition:** superseded → "| Options before an unsure user can choose | Options force
  premature decisions on an unsure user | Open probes help them discover what they think; the
  fork, with its options, follows once they can choose |" and "| Multiple forks per turn |
  Fractured attention, shallow answers across all of them | One fork per turn; the defaults
  batch, confirmed on one screen, is the one stated exception |".
- **Tier failed:** n/a — supersession by ruling (`implement-brainstorm-led` default *What
  analysis-iterative keeps*, added at review, S3: "'one question per turn — always' reads one fork
  per turn, with the defaults batch confirmed on one screen as the one stated exception; 'always
  using structured options' keeps an open probe allowed for an unsure user beside the engine's
  options rule"; D1-engine-asking-contract)
- **Content:** verbatim —

  ```
  | Always using structured options | Options force premature decisions on an unsure user | Open probes help them discover what they think |
  | Multiple questions per turn | Fractured attention, shallow answers across all of them | One question per turn — always |
  ```
- **Kept deliberately:** the other five rows byte for byte (obligation map P8, `kept`); row 1's
  "Why it fails" and its fix's first clause verbatim — the open probe stays allowed beside
  `analysis-iterative.question-form` ("An open probe to an unsure user stays allowed"). The
  [v0.117.0] entry's brainstorm-only supersession ends: `brainstorm.own-rules-win` is superseded by
  `0060-brainstorm-on-engine.yaml` and `spec.own-rules-win` by `0061-specify-on-engine.yaml`
  (default *Precedence*), so the rows and the engine now hold for every caller and the "two texts
  that disagree" that entry accepted no longer exist.
- **Consumers assessed:** `setup` (`setup.interrogation-inline`), specify's intent and story
  stages, `brainstorm` — each now reads the reworded rows beside the engine's rules. No primitive
  quotes a row (grep "One question per turn — always", "Always using structured options": this
  file only). `INTERROGATION-AGENDA.md:5` and `:27` state "one question per turn" in their own
  words — handed to the lead (the engine's `engine-precedence` holds meanwhile).

## [v0.120.0] `## Reference` — the pointer kept, a precedence clause added

- **Disposition:** superseded → "See [ADAPTIVE-EXAMPLES.md](references/ADAPTIVE-EXAMPLES.md) for
  annotated conversations showing these principles in action; they predate the delivered rules,
  which hold where an example differs."
- **Tier failed:** n/a — supersession by ruling (`implement-brainstorm-led` default *Precedence* —
  "the engine's asking rules hold in every user command" — with *What analysis-iterative keeps*,
  which keeps "the adaptive examples")
- **Content:** verbatim — "See [ADAPTIVE-EXAMPLES.md](references/ADAPTIVE-EXAMPLES.md) for
  annotated conversations showing these principles in action."
- **Kept deliberately:** the pointer (the [v0.63.0] keep-set) and `references/ADAPTIVE-EXAMPLES.md`
  byte for byte. Where the examples differ, recorded for a later ruling: Example 1 Turn 3 reads
  three confident yeses to the lead's pick as "a convergence signal — time to nudge toward
  wrap-up", where `analysis-iterative.ratified-streak` flags the streak and changes the next
  fork's form; no example carries the status line (`status-line`) or the case against the pick
  (`question-form`).
- **Consumers assessed:** no primitive outside this skill names the reference file (grep
  "ADAPTIVE-EXAMPLES" over `plugins/mochiko`).

## [v0.120.0] `description:` — rewritten around the shared thinking stage

- **Disposition:** superseded → "This skill MUST be invoked when running the shared thinking
  stage — a question worked through with the user one fork per turn, facts before forks, every
  ruling the user's, the next writer held inside the ruled set when the caller asks — in
  brainstorm's deciding, specify's intent and story stages, the architecture desk's focus stage,
  implement's design stage and setup's interrogation. SHOULD also invoke on 'think this through',
  'thinking stage', 'ruled set', or a multi-decision trade-off. Distinct from
  review-specifications, which grades a drafted spec; this skill rules what gets written." (594
  characters, the canonical snippet; budget 595; cap 1,536.)
- **Tier failed:** n/a — supersession by ruling (`implement-brainstorm-led`
  D2-iterative-becomes-engine, *Accepted risk*: "The skill's `description:` and the router's lines
  … are rewritten around the stage"; *Touch set for the build*: "`description:` rewritten"). This
  ruling supersedes, for this conversion only, skill-pair criterion 7 of
  `.claude/rules/mochiko/primitive-edits.md` ("The frontmatter `description:` value is
  byte-identical across the conversion"); the criterion's ≤ 1,536 cap holds.
- **Content:** verbatim (476 characters, the [v0.63.0] slim form) — "This skill MUST be invoked
  when enriching a sparse feature description into the Who / Problem / Value triad before a
  specification is authored, or when running a collaborative think-through / brainstorm through
  adaptive, one-question-at-a-time discovery. SHOULD also invoke when working through a complex,
  multi-decision trade-off space. Distinct from review-specifications, which reviews an
  already-drafted spec for gaps: this skill conditions raw input BEFORE a spec exists."
- **Kept deliberately:** the MUST/SHOULD grading; brainstorm as a caller; the multi-decision
  trade-off trigger; the `review-specifications` sibling distinction, re-keyed from "conditions
  raw input BEFORE a spec exists" to "rules what gets written". The [v0.63.0] slim discipline
  holds — inside its 595 budget. Leaving: the Who / Problem / Value trigger (the stripped
  variant) and "one-question-at-a-time discovery" (one fork per turn, `one-fork-per-turn`).
- **Consumers assessed:** the router's row (re-keyed, `.mochiko/strips/mochiko.md` [v0.120.0]);
  `review-specifications`' description (re-keyed, its [v0.120.0] entry); the ledger's description
  row, 476 → 594.

## [v0.117.0] `## Output` — the synthesis line re-keyed to defer to the calling command's deliverable

- **Disposition:** superseded → "Conclude with the deliverable your calling command names; when it
  names none, generate the synthesis document using [SYNTHESIS.md](SYNTHESIS.md)."
- **Tier failed:** n/a — supersession by ruling (`brainstorm-target-state` D21-questioning-skill-repairs)
- **Content:** verbatim — "Generate the synthesis document using [SYNTHESIS.md](SYNTHESIS.md)."
- **Kept deliberately:** the [v0.63.0] entry's kept-deliberately line protects "Output's
  synthesis-doc contract" (the goal/output contract, with the Overview). By this ruling it is
  narrowed, not dropped: the contract stands for any caller that names no deliverable, so a direct
  think-through keeps today's synthesis, and setup's named deliverable is itself a synthesis
  (`governance-intent.md`). "Synthesis" is not swapped for "record" (D21's rationale). The Overview
  is untouched. The rest of `## Output` stays byte for byte: the floor line and the
  confidence-indicators table.
- **Consumers assessed (shared skill):** `setup` — `setup.interrogation-inline` points here, and
  `setup.synthesis-artifact` names the deliverable (`governance-intent.md` from the
  `governance-intent` template), so setup concludes with the file its own rule names, as before ·
  `specify` — `spec.intent-stage-first` points here; its Goal names the confirmed Intent section ·
  `brainstorm` — `brainstorm.deliverable-record` names `record.md` and
  `brainstorm.synthesis-on-request` keeps a synthesis on request: the fault D21 repairs · the
  router (`SKILL.md:66`, `:124`), `review-specifications` (`SKILL.md:3`), `INTERROGATION-AGENDA.md:5`,
  `:166` and `ARCHITECTURE.md:367` name the skill and quote neither sentence (grep "synthesis
  document" over `plugins/mochiko`, `README.md`, `ARCHITECTURE.md`, `.mochiko/schema-views`).

## [v0.117.0] `## Two output shapes, one engine` — the dangling "adaptive flow above" pointer re-keyed

- **Disposition:** superseded → "The adaptive questioning the Overview describes is the single
  questioning engine."
- **Tier failed:** n/a — supersession by ruling (`brainstorm-target-state` D21-questioning-skill-repairs)
- **Content:** verbatim — "The adaptive flow above is the single questioning engine." It pointed
  at `## Adaptive Flow`, removed whole at [v0.63.0].
- **Kept deliberately:** the rest of the section byte for byte — one engine, two output shapes,
  both subsections. This is the caller-facing dispatch seam the [v0.63.0] entry's
  kept-deliberately line protects, and only its dangling pointer is re-keyed.
- **Consumers assessed:** no consumer quotes the sentence (grep "adaptive flow" over the same
  scope: this line only).

## [v0.117.0] `## Common Mistakes` — the rows kept at v0.63.0 superseded for `/mochiko:brainstorm` only

- **Disposition:** superseded for `/mochiko:brainstorm` only → `brainstorm.own-rules-win`
  (`0044-brainstorm-target-state.yaml`, `brainstorm.sec.roles`): where this skill and brainstorm's
  own rules differ, brainstorm's rules win. No text leaves this file.
- **Tier failed:** n/a — supersession by ruling (`brainstorm-target-state` D19-brainstorm-only-reach as changed at review,
  S5, and at verify, V4; `DECISIONS.md` 2026-10-06 row)
- **Content (the kept table, verbatim):**

  ```
  | Always using structured options | Options force premature decisions on an unsure user | Open probes help them discover what they think |
  | Ignoring "unsure" signals | Uncertainty read as indifference → recommendations passively accepted, never owned | Probe deeper from a different angle |
  | Multiple questions per turn | Fractured attention, shallow answers across all of them | One question per turn — always |
  | Questions that don't connect | Topic-jumping breaks collaborative momentum | Show how the previous answer shapes the direction |
  | Premature synthesis | Forced wrap-up leaves core trade-offs unaddressed | Nudge, never force; the user has final say |
  | Rigid opening script | A recited introduction ignores the topic's tone and energy | Adapt the opening to both |
  | Padding the synthesis | Filler sections dilute a lean conversation's record | Output depth matches conversation depth |
  ```

  The differences the record names (D19, S5): "One question per turn — always", where D6 and D17
  batch; and always-structured options listed as a mistake, where D13 fixes the form when options
  are put (an open question to an unsure user stays allowed).
- **Kept deliberately:** every row byte for byte, for setup and every other caller — the [v0.63.0]
  entry's kept-deliberately anti-patterns stand for them. The v0.63.0 cut is not reversed (D19),
  and its finding that detail buys weight, not honesty, binds the form of brainstorm's new rules.
  Accepted by the ruling: two texts that disagree stay in the plugin.
- **Consumers assessed:** `setup` (`setup.interrogation-inline`) and `specify`
  (`spec.intent-stage-first`) read the rows as before; `brainstorm` reads them under
  `brainstorm.own-rules-win`. No other primitive quotes a row.

## [v0.63.0] Guardrails cut — procedure walkthroughs removed, guardrails kept; slim description

- **Disposition:** superseded → the benchmark-ruled guardrails body + slim description (`.mochiko/benchmarks/guardrails-vs-detail/variants/body/analysis-iterative/`, `.../descriptions/analysis-iterative/`). Shipped as one merged edit: slim frontmatter + guardrails body.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict; `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`)
- **Content (faithfully compressed — section-level inventory of the body cut, 11,438 → 3,942 chars, −66%):**
  - **Removed whole:** `## When to Use` (trigger list — carried by the description) · `## Adaptive Flow` (the Opening → Discovery → Adaptive-Questioning → Conclusion phase walkthrough) · `## Discovery` (discovery-phase procedure) · `## Question Format Adaptation` and its four format subsections `### Structured Options + Recommendation`, `### Open-Ended Probes`, `### Confirmations`, `### Recommend-then-Arbitrate` (the four question-format code-block templates) · `## Reading Confidence Signals` (the per-turn signal-recalibration table plus the asymmetry principle) · `## Smart Wrap-up` (the ratification-streak doctrine and wrap-up procedure).
  - **Shortened:** `## Output` (787 → 525 chars — the synthesis-doc pointer and the confidence-indicators table kept; extra prose trimmed).
  - **Kept intact:** `# Iterative Analysis`, `## Overview`, `## When NOT to Use`, `## Common Mistakes`, `## Two output shapes, one engine` with `### General analysis` and `### Specification-input enrichment`, `## Reference`.
  - Old description (new slim form is 476 chars; **old verbatim, 808 chars**): "This skill MUST be invoked when enriching a sparse feature description into the Who / Problem / Value triad (plus explicit out-of-scope and success criteria) before a specification is authored, or when running a collaborative think-through / brainstorm that explores a problem through adaptive, one-question-at-a-time discovery and ends in a structured synthesis. SHOULD also invoke when a feature description lacks Who/Problem/Value clarity and needs conditioning before it reaches a requirements producer, or when deliberately working through a complex, multi-decision trade-off space. A general / shared skill — useful across clusters, not tied to a single workflow. Distinct from review-specifications, which reviews an already-drafted spec for gaps: this skill conditions raw input BEFORE a spec exists."
  - Verbatim removed text survives in three homes: git history of `plugins/mochiko/skills/analysis-iterative/SKILL.md`; the before/after pair in `variants/descriptions/analysis-iterative/` (before-state) and `variants/body/analysis-iterative/` (after-state); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** the goal/output contract (Overview + Output's synthesis-doc contract), the non-waivable floor (the floor line added below), the anti-patterns (`## Common Mistakes`, `## When NOT to Use`), the caller-facing dispatch seam (`## Two output shapes, one engine`), and the `references/ADAPTIVE-EXAMPLES.md` pointer. *(Touched by ruling at [v0.117.0], `brainstorm-target-state` — see those entries: Output's synthesis-doc contract now defers to the calling command's deliverable, the synthesis kept for a caller that names none (D21); the Common Mistakes rows are superseded for `/mochiko:brainstorm` only and stand for every other caller (D19); the dispatch seam's dangling "adaptive flow above" pointer is re-keyed (D21).)*
- **Floor line added (pure addition, rides this ruling):** in `## Output` — "**Floor (non-waivable)** — surface every elicited unknown as an open question in the produced artifacts; a vague zone the principal could not resolve is never silently omitted." (guardrails-vs-detail cross-cutting finding 1; build-plan §4.4). Final shipped body is 4,120 chars, within the 4,928 budget.
- **Protected-content reconciliation (MANDATORY):** the two prior `[v0.25.0] KEPT:` survivor entries reconciled against this cut —
  - **`[v0.25.0] KEPT: the four question-format templates + ratification-streak doctrine`** — **superseded by this ruling.** The four format code blocks (`### Structured Options + Recommendation`, `### Open-Ended Probes`, `### Confirmations`, `### Recommend-then-Arbitrate`) and the ratification-streak doctrine (in the removed `## Smart Wrap-up`) are removed by the guardrails cut. Their dogfood-backed intent (adapt question format to the user's state; flag a yes-streak) is now carried as guardrails only — the "always structured options" and "ignoring unsure signals" rows of `## Common Mistakes`. The verbatim templates survive in the three homes above.
  - **`[v0.25.0] KEPT: confidence-signals table, asymmetry principle, two-output-shapes dispatch`** — **partially superseded.** The per-turn confidence-signals recalibration table and the asymmetry principle (both in the removed `## Reading Confidence Signals`) are superseded by this ruling; verbatim in the three homes. The **two-output-shapes dispatch survives intact** (`## Two output shapes, one engine`) — the caller-facing enrichment-vs-general-analysis seam is kept. (Note: the Output section's *synthesis-doc* confidence-indicators table — Confident/Assumed/Contested/Unsure/Deferred — is a different table and is kept.)
- **Consumers assessed (shared skill — 3 consuming commands):** `commands/setup.md`, `commands/specify.md`, `commands/brainstorm.md`. Each dispatches the skill by name and relies on its behavioral output (adaptive questioning → structured synthesis or enrichment artifact); none quotes a removed section anchor. The kept Overview, Output contract, and two-output-shapes dispatch preserve the invocation contract each relies on. Contracts intact.


## [v0.25.0] Adaptive Flow ASCII diagram (9 lines) → one-line flow
- **Disposition:** deleted (the one-line "Opening → Discovery → Adaptive Questioning → Conclusion" retained; the four labeled phase paragraphs directly below carry every behavior the box encoded)
- **Tier failed:** 2 (no behavior named beyond the surviving paragraphs)
- **Content:** the four-column box diagram with per-phase annotations
- **Consumers assessed:** 6 consumer files checked, none reference the diagram

## [v0.25.0] Three inline example blocks (15 lines)
- **Disposition:** relocated → `references/ADAPTIVE-EXAMPLES.md` (the skill's declared example home; the ops-team crisp-answer and API-versioning format-comparison conversations exist there in expanded, annotated form — verified before landing)
- **Tier failed:** 1 (compressed copies of the reference's annotated conversations)
- **Content:** crisp-answer branch, unsure-answer branch, same-topic-three-formats examples
- **Consumers assessed:** none reference the inline examples

## [v0.25.0] Common Mistakes densified: 7 subsections → 7-row table (net −12 lines)
- **Disposition:** compressed in place (densification, zero deletions — every mistake/failure/fix survives as a row)
- **Tier failed:** n/a — no content left the skill; form only
- **Content:** the seven mistake subsections (structured-options overuse, ignored unsure signals, multiple questions, disconnected questions, premature synthesis, rigid opening, padded synthesis)
- **Consumers assessed:** none reference the subsection headings

## [v0.25.0] KEPT: the four question-format templates + ratification-streak doctrine
- **Tier-2 evidence:** contested at the under-band pass and kept — the format code blocks ARE the
  skill's operative procedure (structured options / open probe / confirmation /
  recommend-then-arbitrate), and the streak doctrine + recommend-then-arbitrate format carry
  dogfood evidence: validated across four design sessions including the skill-succinctness
  session itself (streak flagged after 4× "yes a"; user re-engaged with genuine reads). Session
  ruling: batch-1 ratification 2026-07-25.

## [v0.25.0] KEPT: confidence-signals table, asymmetry principle, two-output-shapes dispatch
- **Tier-2 evidence:** contested at the under-band pass and kept — the signals table is the
  per-turn recalibration procedure; the asymmetry principle names the failure direction
  (early wrap-up costs rework, an extra question costs one turn); the two-shapes section is the
  live dispatch seam callers rely on (enrichment vs general analysis). Session ruling: batch-1
  ratification 2026-07-25.
