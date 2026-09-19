# Grading bar — checklist report `checklist-report.md`

Grade `checklist-report.md` against every item below. Each item is PASS or FAIL with one line
of evidence from the file. The verdict is PASS only when every item passes.

1. **Header.** The report names the artifact graded and the checklist run.
2. **Evidence read.** An `Evidence read:` line lists at least one file, and every file it lists
   exists in this workspace.
3. **Per-item evidence.** Every checklist item carries PASS or FAIL and a one-line evidence
   pointer (a file with a line reference, or a quoted span).
4. **Binary verdict.** The verdict line is exactly PASS or FAIL — no qualifier.
5. **Fix list.** On FAIL, every failing item appears in the fix list with a concrete fix; on
   PASS, the fix list is empty or reads "none".
6. **No placeholders.** The report contains none of the tokens `TBD`, `TODO`, `FIXME`, `???`,
   or `[NEEDS CLARIFICATION]`.

---

*Dispatch note.* If an item above does not fit a checklist report, grade that item on your
usual standards instead of skipping it.
