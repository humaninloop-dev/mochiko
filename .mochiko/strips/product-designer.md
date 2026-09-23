# Strip notes — `agents/product-designer.md`

Entry formats: `strips/README.md`. Renamed from `agents/product-engineer.md` at v0.114.0 (entry
below); the entries after it were written against the old name and are kept as written.

## [v0.114.0] `product-engineer` renamed `product-designer`; design-director judgment added

- **Disposition:** superseded → `plugins/mochiko/agents/product-designer.md` (`git mv` of the
  same file, then reworded). The persona now carries the design-director judgment the
  Impeccable port puts on it — the brief wins · refinement preserves, redesign replaces ·
  visual authority is evidence · mode chosen per surface — as identity and taste, with no
  workflow trace. Procedure sits in the new `mochiko:patterns-design-direction` skill, now
  mounted beside `authoring-prototype`.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/impeccable-design-integration/record.md` D3 — the judgment's home on
  this persona, the `ui-designer` Design track superseded; D4 — the rename, this wave, MINOR,
  no deprecation alias, revert only as a second supersession by ruling; `DECISIONS.md`
  2026-09-19 row). Wave: `wave1-design-integration.md`, seat S1.
- **Content:** verbatim, each line that left or changed —
  - `name: product-engineer`
  - `description:` "Staff-level Product Engineer who makes intended user experiences tangible
    before they are built — authoring clickable low-fidelity prototypes that render user
    stories as screens and walkable flows, honoring an existing design system, and keeping
    fidelity honest: structure and flows precise, pixels deliberately rough. Produces the
    prototype and its manifest; does not grade its own output." (v0.63.0 protected prose
    framing; re-cut for the new judgment, 392 → 449 chars by the canonical snippet, under the
    490 budget)
  - `skills: authoring-prototype` (now `authoring-prototype, patterns-design-direction`)
  - "You are the **Product Engineer** — a staff-level engineer who makes intended user
    experiences tangible before they are built."
  - "You think like an engineer who has:" (now "a design director")
  - "Shipped products with design systems and watched mocks that ignored them create surprise
    twice — so where a system exists, you build from its tokens and components, at rough
    fidelity" (re-homed: the baseline-first design-system read now lives in
    `authoring-prototype.design-system-honored`, reworded by migration
    `0011-design-direction-craft-floor.yaml`; the persona keeps the taste as "visual authority
    is evidence" and "the product's own design language where one exists")
  - What You Produce items 1–3, renumbered 2, 3, 5 around the two new items (a direction per
    surface · the product's design record, as shipped)
  - "The concrete procedure lives in your skill, which is the single source of truth:" (now
    "skills, which are")
  - "Grading your own prototype" (now "Grading your own work" — the persona now produces a
    direction and design-record entries besides the prototype)
  - "A design system's language where one exists — at low fidelity, not reproduction" (now
    "The product's own design language where one exists — …")
- **Kept deliberately:** `model: sonnet` (the seat default key, `orchestrator-model-selection`
  D3 — S1 ran deviated at `opus` for this wave only, disclosed in the roster line), `color:
  green`, the Skills Available framing, the five remaining Core Identity bullets (build-time
  discovery, fidelity honesty, honest data shape, scope invention, disposable code), the
  Quality Standards and Reject/Embrace lines not listed above, and the Delegating Cheap Reads
  section verbatim. No accessibility content was added to the persona (D11 — the standard of
  record is routed, never restated, and design-time taste carries none of it).
- **Consumers assessed:** the address `mochiko:product-engineer` — router
  `skills/mochiko/SKILL.md:155`, `plugin.json` agents list, `README.md:132`,
  `.mochiko/memory/primitive-cost-budgets.md` persona row, `evals/agents/product-engineer/`,
  the contract-suite pre-registration, the live `patterns-model-tiering.seat-default-key`
  floor, `ARCHITECTURE.md:195,203,212` (store re-render) — all re-keyed by seat S5 in
  `0015-product-designer-rekey.yaml` and its ripple, not here. Historical migrations
  `0007`/`0010`, `.mochiko/benchmarks/**`, the trail, session records, and DECISIONS rows are
  frozen and not rewritten (D4). The persona-edit advisory grid is S5's to attempt.
- **Wave deviation (disclosed, lead-ruled):** the `patterns-design-direction` skill this persona now
  mounts routes the `adaptive` platform value to BOTH `references/ios.md` and
  `references/android.md` (rules `patterns-design-direction.platform-routing` and
  `patterns-craft-floor.platform-native`, migration `0011-design-direction-craft-floor.yaml`);
  the approved S1 plan text routed `adaptive` to none. Accepted by the wave lead as faithful to
  D6 ("loaded … on the platform value") and to upstream's ported behavior.
- **Persona-edit advisory grid — skipped (S5, 2026-09-23; advisory only, never a gate):** the kit
  was re-keyed to `evals/agents/product-designer/` (stamped in its `preregistration.md`) and the
  pre-flight `uv run evals/run.py agent check product-designer` was run before any session. It
  halted with `error: git show b9efb59:plugins/mochiko/agents/product-designer.md failed: fatal:
  path 'plugins/mochiko/agents/product-designer.md' exists on disk, but not in 'b9efb59'`.
  Reason: the runner keys the `pre` arm's persona text by the persona's current name at the old
  ref (`evals/plan/agents.py:89-99`, `persona_text`), and before this rename the file was
  `product-engineer.md`, so no `pre` arm can be read under the new name; the rubric in
  `rules.json` is also stale against the D3 body rewrite. The grid (`agent grid product-designer
  --arms pre,post`) was therefore not run and no metered session was spent. Owed: a runner
  change that lets the `pre` arm name a pre-rename path, then a re-mint (`agent mint
  product-designer --old-ref <pre-rename ref>`) and the pilot-form grid with a positive control.

## [v0.110.0] Frontmatter `model: opus` superseded — the seat default key sends this seat to `sonnet`

- **Disposition:** superseded → `model: sonnet` on the same frontmatter key. The seat default key
  (`orchestrator-model-selection` D1) assigns this persona the `down` class on one criterion —
  does a structurally independent seat stand between this seat's output and the run's verdict —
  and D2 pins the class default in the persona file as a tier alias, never `inherit` and never an
  absent `model:`. D3's ground for this row: "prototype producer, graded"; D3's G7 fold states the
  standard that pass rests on — the independent gap-finding pass plus the lead's gate plus the
  user's acceptance.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/orchestrator-model-selection/record.md` D3, the ten-row seat class table,
  with D1/D2 as its mechanism and D8 as its alias rule; `DECISIONS.md` 2026-09-19 row). The same
  ruling supersedes `model-tiered-seats` D5 and its fold F6 — the deferral this table discharges.
- **Content:** verbatim — `model: opus` (frontmatter, line 9).
- **Kept deliberately:** every other byte of the file — the frontmatter `description:` value
  (v0.63.0 protected prose framing), `name:`, `color:`, the `skills: authoring-prototype` mount,
  and the whole body including `## Delegating Cheap Reads`. That section's `model: haiku` override
  sentence is the reads rung, a different key from the seat default; D7 item 2 states the persona
  sections are unchanged by this ruling. D8 also keeps the new value a family alias, never a full
  id such as `claude-sonnet-5`.
- **Consumers assessed:** `plugins/mochiko/.claude-plugin/plugin.json` agents list — path-only
  entries with no model axis, unchanged · the router's agents table
  (`plugins/mochiko/skills/mochiko/SKILL.md`, the `product-engineer` specify-cluster PRODUCER row)
  — no persona row names a model; the row is role text and stays as it is · `ARCHITECTURE.md`
  agents row (line 61 after this wave), which read "Personas (all `model: opus`)" — corrected in
  this same wave ·
  `plugins/mochiko/skills/patterns-model-tiering/` and its floor set — superseded by migration
  `0007-seat-default-key`, which records the schema content by construction and takes no entry
  here · the persona eval kit `evals/agents/product-engineer/` and the runner constant `ARM_MODEL`
  in `evals/commands/agents.py` — both live on the `primitive-evals-v2` branch, not on `main`;
  noted, not edited (D7 item 4, deferred to the branch merge) ·
  `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-engineer.md` carries a frozen
  `model: opus` copy — a benchmark archive, not a shipped primitive, deliberately untouched.

## [v0.78.0] Delegating Cheap Reads retargeted — `mochiko:explorer` dispatch superseded by native `Explore` + `model: haiku` override

- **Disposition:** superseded → the reworded `## Delegating Cheap Reads` sentence: "spawn a
  disposable native `Explore` subagent with an explicit `model: haiku` override (the
  override makes the read cheap; a bare spawn inherits the session tier)".
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents, so the
  `mochiko:explorer` dispatch this section prescribed failed on exactly the transport the
  section was built for.
- **Content:** verbatim superseded span (identical across all ten personas): "spawn a
  disposable `mochiko:explorer` subagent (its `model: haiku` frontmatter makes the read
  cheap)".
- **Kept deliberately:** the rest of the `## Delegating Cheap Reads` section byte-for-byte —
  the class-key summary (locate/enumerate/targeted-read cheap; interpretive, absence-driven,
  completeness-sensitive kept), one-gap-per-spawn, the bulk-read-stays-out rule, and the
  closing pointer to `mochiko:patterns-model-tiering`.
- **Consumers assessed:** the section wording is shared across the ten personas; all ten
  edited in the same v0.78.0 wave (this entry mirrored in each persona's strip file). No
  command or skill names the section.

## [v0.63.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-engineer.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Context: a feature's user stories are being drafted and the experience needs to be visible before design starts — commentary claimed the example demonstrated that making a drafted experience clickable at low fidelity, story by story, is the product-engineer's core producer work.
  2. Context: the project has an existing design system and the mock should read as part of the product — commentary claimed it demonstrated that honoring an existing design system at low fidelity, without over-polishing into false precision, is the product-engineer's judgment.
  3. Context: a reviewer found flows in the mock that no story scenario covers — commentary claimed it demonstrated that keeping the prototype an honest rendering of the stories, surfacing gaps instead of inventing scope, is the product-engineer's discipline.

  Description parsed-value char delta: **2,208 → 391** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/product-engineer.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-engineer.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Staff-level Product Engineer who makes intended user experiences tangible before they are built … Produces the prototype and its manifest; does not grade its own output.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `product-engineer`: `skills/*/SKILL.md` reference(s) only; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** no prior strip entries exist for this primitive; no `KEPT:` / protected / `DECISIONS.md`-traceable line touches the `description:` value or its `<example>` blocks. No overlap.
