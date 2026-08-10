# Strip notes — `agents/system-architect`

Entry formats: `strips/README.md`.

## [v0.64.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (Wave 2 editorial extension of the agents-arm ruling); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept verbatim.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail agents-arm user ruling (b) 2026-08-10, extended to the untested agents by the Wave 2 user ruling; `DECISIONS.md` 2026-08-11 build row Wave 2 residual; `report/final-verdict.md` — 0 route misses over 20+ staffings).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Component shape worked out before detailed design — container-level topology as the shape detail conforms to.
  2. Async settlement path with retries and a provider webhook — sync-vs-async interaction style, ordering/failure captured in a sequence view.
  3. Feature lands in an existing system with no architecture doc — reconstruct the baseline from code, mark confidence, design the delta on it.

  Description parsed-value char delta: **2,236 → 481** (chars of the parsed block-scalar value; block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in git history of `plugins/mochiko/agents/system-architect.md` (pre-v0.64.0).
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Senior system architect whose craft is topology — deciding what the components are, where the boundaries cut, how the pieces talk (sync vs async, request/response vs event), and where each responsibility lives, then proving the shape can actually be built and operated under its real constraints. Reads the current system before proposing a change and designs the delta from it, making every structural change visible. Authors the architecture view; does not grade its own output.") — and the entire agent body, byte-for-byte untouched (verified against git HEAD).
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `system-architect`: referenced only by the router `plugins/mochiko/skills/mochiko/SKILL.md`; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed.
- **Standing watch:** an F-X1-class route miss on the untested agents re-opens ruling (b).
- **KEPT reconciliation:** first strip entry for this primitive — no prior KEPT/protected content or earlier strip touches this file. No overlap.
