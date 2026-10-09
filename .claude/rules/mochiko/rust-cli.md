---
paths:
  - "crates/mochiko-cli/**"
  - "plugins/mochiko/migrations/**"
  - "plugins/mochiko/hooks/**"
  - "evals/contract/**"
  - ".github/workflows/**"
  - "plugins/mochiko/.claude-plugin/plugin.json"
  - ".claude-plugin/marketplace.json"
  - "CHANGELOG.md"
---

# Rust CLI — kernel-class delivery under the bright line <!-- GI-019 · GI-020 · GI-012 -->

`crates/mochiko-cli/` is mochiko's admitted kernel-class tool: it serves every command's and
skill's rules from the migration log — carried in the plugin at `plugins/mochiko/migrations/`
since wave 3 — replayed in memory at fire, and it validates the log's own data. Admitted by four
recorded rulings: `schema-based-template-guidance` D11-kernel-position-softened (template delivery, 2026-08-16),
`cli-schema-delivery` D11-widened-kernel-admission (the widened role, 2026-09-03), `hook-enforced-artifact-schema`
D1-conformance-deny-channels/D7-governance-routing-supersession (mechanical conformance gates on artifact writes, 2026-09-13), and
`human-readable-ids` D7-scoped-rename-command (the `ids` ID-rewrite tool, 2026-10-08). The standing bright line
binds it.

- **Bright line (GI-019-kernel-tooling-admission).** The tool renders, replays, and validates its own data — including
  an artifact's **mechanical conformance** to the shape the log declares for its home (path,
  closed under `.mochiko/` · file set · headings · frontmatter · placeholders · per-section,
  per-entry and whole-file size; `mochiko-cli check`, AM-3-conformance-gate-admission), decided against the log's data and
  the repository's own layout — yes/no facts such as whether `.gitignore` carries
  `.mochiko/runs/` or a path sits under a git worktree — with no judgment (AM-5-field-review-fold). It also rewrites ID tokens in
  the repository's text (`mochiko-cli ids rename|rekey|literal`, AM-6-id-tool-admission): preview by default, every slug
  supplied by the seat, any write blocked by a changed span that is not an ID token. It MUST NOT grade an artifact's
  meaning or quality, MUST NOT dispatch or sequence agents, and MUST NOT hold judgment that
  skills own. Its hooks block on exactly two grounds:
  the binary's absence or a log outside its grammar range (the dependency halt), and a
  conformance deny from `check` (exit 4) — never on behavior or judgment; every other outcome
  is an explicit `allow`. Home: CLAUDE.md `## Non-negotiable constraints`; detail: ledger GI-019-kernel-tooling-admission
  (clause iv).
- **Dependency, not fallback (GI-020-plugin-install-model as amended v3.0.0).** The plugin depends on this binary;
  absence or skew halts loudly and never degrades. No code path may read a schema file as a
  fallback, and no schema file ships in the plugin (the transition clause expired at v3.0.3 with
  the wave-6 end state). Detail: ledger GI-020-plugin-install-model.
- **The log is truth (record `cli-schema-delivery` D1-migration-log-truth/D2-full-schema-scope/D6-migration-integrity-constraints).** A migration file under the log directory is the only
  editing surface for schema content; the derived views under `--out` are generated and never
  hand-edited. The log's own constraints — required fields, and the ruling anchor a supersession
  or tombstone of protected content must carry — are enforced by the binary at apply; the rules
  themselves live in the schema, not here.
- **Quality gate (GI-012-release-gates-module).** `cargo test --all`, `cargo fmt --all --check`, `cargo clippy
  --all-targets -- -D warnings`, and `cargo audit --deny warnings` under CI; the full similarity
  sweep is opt-in (`MOCHIKO_FULL_SIMILAR=1`, its own CI step); the plugin contract suite
  (`python3 evals/contract/run.py`, Docker sandbox) is the maintainer-side gate at every
  `plugin.json` bump, and a SKIPPED suite is not green. Every crate unit lands on a
  lead-approved plan with an independent non-author code review — author≠grader extends to code.
- **Release (record D4).** Distributed as a developer tool — crates.io plus the Homebrew tap via
  `.github/workflows/release.yml` on `mochiko-cli-v*` tags; the plugin ships no binary. A tag
  MUST NOT land without the four crate layers green, the contract suite green against the tagged
  binary, and an unchanged render output shape — or a coordinated `plugin.json` bump when that
  shape changes. Detail: ledger GI-012-release-gates-module.
- **Maintainer break-glass.** Working from a source tree ahead of the published release:
  `cargo install --path crates/mochiko-cli`. This is the maintainer's path only — it is never a
  user-facing install route, and it does not soften the dependency above.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md` —
GI-019-kernel-tooling-admission, GI-020-plugin-install-model, GI-012-release-gates-module.
