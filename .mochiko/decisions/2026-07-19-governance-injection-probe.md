# Governance-injection probe suite

**Status:** ruled · **Date:** 2026-07-19 (v0.12.0)
**Context:** the kinako dogfood's hand-built smoke test found a validator-PASSed surface set whose rules never injected on new-file Writes and two rules missing a layer — static trace closure cannot catch delivery failures. Full detail: `.mochiko/archive/ROADMAP.md` (Key Decisions, "Governance-injection probe suite").

**Decision (user-ruled):** productize as `testing-governance-injection` (stubs + disposable introspection subagents + trigger matrix + unconditional cleanup); `/mochiko:setup` G5 offers it token-priced; findings route to an amend run, never auto-fixed. Companion authoring/validation guidance fixes for the two known failure modes.

**Rationale:** guidance edits prevent only known failure modes; the probe catches the next unknown-unknown in rules delivery.

**Alternatives:** guidance-only (rejected — the probe caught what trace closure structurally cannot).
