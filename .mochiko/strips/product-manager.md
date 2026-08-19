# Strip notes — `agents/product-manager.md`

Entry formats: `strips/README.md`.

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
