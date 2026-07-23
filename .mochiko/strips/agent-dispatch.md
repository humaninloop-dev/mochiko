# Strip notes — `templates/agent-dispatch.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D6c; ratified 2026-07-23).
v4 also fixed a staleness: the footer paragraph claimed plan/tasks/implement were still
one-shot; all seven commands have been team-form since v0.17.0 (BACKLOG conversion rows).

## [v0.22.0] HTML comment header relocated (runtime-loaded rationale)
- **Disposition:** relocated → here (D6c). The live kernel stayed in the visible body: "a caller-side checklist, not a file you commit — fold the fields into each dispatch prompt", the degrades-gracefully point, the workflow-knowledge-stays-caller-side rule. The header's independence bullet was redundant with the body's "What the caller MUST get right" section and simply died.
- **Tier failed:** pure waste (map §5): Read-tool template loads do not strip HTML comments — the 1,783 B header (27% of the file) cost context on every team-form run.
- **Content (the relocated rationale, faithfully compressed):**
  - **Why this exists:** a mochiko agent is a self-contained professional that degrades gracefully — a sparse brief produces a worse result, not a broken agent; the checklist raises loop quality, it does not make the agent functional. The agent owns none of the workflow knowledge (which workflow, which siblings, what "done" means) — all of that lives caller-side; pushing it into a persona is the coupling this template is the antidote to.
  - **Companion note:** call-time companion to `workflow-contract.md` — the contract proves the LOOP is sound; agent-dispatch is how the caller briefs each CALL inside it well (redundant with the body footer's Pairs-with line).
  - **Header independence bullet:** independence is structural — different agent, different skill, never grade your own output (restated verbatim in the body's hard-line section; deleted as duplication).
