---
paths:
  - "crates/mochiko-cli/**"
---

# Rust CLI — kernel-class tooling under the bright line <!-- GI-019 · GI-020 · GI-012 -->

`crates/mochiko-cli/` is mochiko's first admitted kernel-class tool (template-schema delivery;
foundation seed for future native tooling). It is admitted **only** by the recorded D11 ruling,
and the standing bright line binds it — full rulings D1–D11:
`.mochiko/brainstorms/schema-based-template-guidance/record.md`.

- **Bright line (GI-019).** This tool MUST NOT gate pipeline progress, MUST NOT dispatch or
  sequence agents, and MUST NOT hold judgment that skills own — it renders and composes guidance
  views, nothing more. Home: CLAUDE.md `## Non-negotiable constraints`; detail: ledger GI-019.
- **Additive install (GI-020).** The plugin MUST install and function markdown-only — the binary
  is strictly additive. Schema data files ship in the plugin and are the source of truth; the
  binary only renders them. The raw-Read fallback (D8) MUST stay honest: any change that makes the
  data unreadable without the binary is a governance defect. Detail: ledger GI-020.
- **Quality gate (M6 → GI-012).** The crate MUST land with its own test suite and pass an
  independent non-author code review before landing (author≠grader extends to code). At crate
  landing, `cargo test` PASS joins the release gates — the GI-012 dormant clause activates.
- **Landing wave (D3/D10).** Conversion scope is the 8 pipeline artifact templates only. Their
  `.md` deletions and skill re-points are `plugins/mochiko/` primitives — each takes the strip +
  author≠grader ceremony (`.claude/rules/mochiko/primitive-edits.md`). This crate is not a plugin
  primitive, but its landing wave includes them.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md` — GI-019,
GI-020, GI-012.
