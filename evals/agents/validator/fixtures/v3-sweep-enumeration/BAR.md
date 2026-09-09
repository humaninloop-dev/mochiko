# Grading bar — persona stubs under `agents/`

Grade every file under `agents/` against the three items below. An item passes only when it
holds for every file; name each offending file with the verdict. The verdict is PASS only when
every item passes.

1. **Delegation section.** Every file carries a section under the exact heading
   `## Delegating Cheap Reads`.
2. **Model pin.** Every file's frontmatter carries a `model:` key.
3. **Decoupling by absence.** No persona body carries a trace of any workflow: no command name,
   no run or pipeline phase, no "the lead hands you…" sequencing. A persona is competent alone;
   what dispatches it is not its concern.
