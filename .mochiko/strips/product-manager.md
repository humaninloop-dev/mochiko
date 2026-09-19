# Strip notes — `agents/product-manager.md`

Entry formats: `strips/README.md`.

## [v0.110.0] Frontmatter `model: opus` superseded — the seat default key sends this seat to `sonnet`

- **Disposition:** superseded → `model: sonnet` on the same frontmatter key. The seat default key
  (`orchestrator-model-selection` D1) assigns this persona the `down` class on one criterion —
  does a structurally independent seat stand between this seat's output and the run's verdict —
  and D2 pins the class default in the persona file as a tier alias, never `inherit` and never an
  absent `model:`. D3's ground for this row: "feature-map producer; selection is the user's
  ruling"; D3's G7 fold states the standard that pass rests on — the independent gap-finding pass
  plus the lead's gate plus the user's acceptance.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/orchestrator-model-selection/record.md` D3, the ten-row seat class table,
  with D1/D2 as its mechanism and D8 as its alias rule; `DECISIONS.md` 2026-09-19 row). The same
  ruling supersedes `model-tiered-seats` D5 and its fold F6 — the deferral this table discharges.
- **Content:** verbatim — `model: opus` (frontmatter, line 10).
- **Kept deliberately:** every other byte of the file — the frontmatter `description:` value
  (v0.63.0 protected prose framing), `name:`, `color:`, the `skills: authoring-feature-map` mount,
  and the whole body including `## Delegating Cheap Reads`. That section's `model: haiku` override
  sentence is the reads rung, a different key from the seat default; D7 item 2 states the persona
  sections are unchanged by this ruling. D8 also keeps the new value a family alias, never a full
  id such as `claude-sonnet-5`.
- **Consumers assessed:** `plugins/mochiko/.claude-plugin/plugin.json` agents list — path-only
  entries with no model axis, unchanged · the router's agents table
  (`plugins/mochiko/skills/mochiko/SKILL.md`, the `product-manager` product-layer PRODUCER row) —
  no persona row names a model; the row is role text and stays as it is · `ARCHITECTURE.md` agents
  row (line 61 after this wave), which read "Personas (all `model: opus`)" — corrected in this same
  wave ·
  `plugins/mochiko/skills/patterns-model-tiering/` and its floor set — superseded by migration
  `0007-seat-default-key`, which records the schema content by construction and takes no entry
  here · the persona eval kit `evals/agents/product-manager/` and the runner constant `ARM_MODEL`
  in `evals/commands/agents.py` — both live on the `primitive-evals-v2` branch, not on `main`;
  noted, not edited (D7 item 4, deferred to the branch merge) ·
  `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-manager.md` carries a frozen
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

## [v0.68.0] "Feature derivations" produce-line reworded → frame-first "Capability framings and derivations"
- **Disposition:** superseded → the frame-first produce line in the persona body ("What You
  Produce" item 1). Under the new PM seat position (record D5), the PM frames the capabilities an
  intent will touch as a hypothesis *before* stories are drafted, then confirms or corrects that
  frame against them after — so the produce line no longer reads as derivation *from* a drafted
  story set.
- **Tier failed:** n/a — supersession by ruling (record D5 — PM repositioned to specify's front,
  capability-frame-as-hypothesis at intent, stories win conflicts; `DECISIONS.md` 2026-08-13 "PM
  role & feature derivation" row → `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`).
- **Content:** the reworded produce line, before → after (verbatim):
  - **Before:** "1. **Feature derivations** — the capabilities a set of stories implies, checked
    against the existing map: extensions of what exists before duplicates of it"
  - **After:** "1. **Capability framings and derivations** — the capabilities an intent implies,
    framed as a hypothesis before stories are drafted and confirmed or corrected against them
    after, checked against the existing map: extensions of what exists before duplicates of it"
- **Kept deliberately:** the persona is capability-first already, so nothing else in the body was
  reworded. Two **pure additions** landed in the same v0.68.0 edit and ride the decision row (no
  strip owed per `strips/README.md` — additions are not supersessions), recorded here only for
  GI-006 reconstructability: (a) a Core Identity bullet carrying the framing-first posture
  (frame capabilities as a nouns-and-verbs hypothesis before any story is drafted; stories win a
  genuine conflict — record D5); (b) a "What You Embrace" bullet carrying the pending-rows
  completeness view (an entry's honest state includes the cut-but-undelivered work pending on it —
  record D2). The frontmatter `description:` was left **byte-for-byte untouched** (438 parsed
  chars, budget 548) — the description asserts no story-first sequencing, frame-first is a
  body-level posture, and the value was deliberately benchmark-slimmed at v0.63.0 (entry below);
  reopening it for no routing gain was declined at build (lead-ruled).
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for
  `product-manager`: `commands/specify.md` (staffing reference — the PM's front-of-specify
  touchpoints are that command's own D5 ripple, owned by the specify seat) and the `mochiko`
  router skill. The agent name and the routing prose framing of the `description:` are unchanged;
  the staffing contract is intact. Body-only edit.

## [v0.63.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-manager.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Context: user stories are drafted and the built capabilities behind them need naming — commentary claimed the example demonstrated that deriving durable capabilities from stories, against the real map and never blind to it, is the product-manager's core producer work.
  2. Context: a drafted story doesn't clearly belong to any capability the product needs — commentary claimed it demonstrated that saying no to a story with stated reasoning, as a recommendation rather than a silent drop, is the product-manager's discipline.
  3. Context: more features are on the table than the team should build now — commentary claimed it demonstrated that portfolio advice with the trade-offs shown, while the selection ruling stays with the user, is the product-manager's judgment.

  Description parsed-value char delta: **2,138 → 437** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/product-manager.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-manager.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Senior Product Manager who owns the product's capability layer … Recommends and gives reasons; selection is always the user's ruling. Authors the feature map; does not grade its own output.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `product-manager`: `commands/specify.md` (staffing reference) and `skills/*/SKILL.md`. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** no prior strip entries exist for this primitive; no `KEPT:` / protected / `DECISIONS.md`-traceable line touches the `description:` value or its `<example>` blocks. No overlap.
