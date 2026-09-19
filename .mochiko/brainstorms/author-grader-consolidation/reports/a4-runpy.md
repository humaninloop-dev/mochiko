---
report: review
round: 1
---

VALIDATE: the fix-on-sight defect close in `evals/contract/run.py` — `case_reminder_spawn`'s evidence write referenced an unbound name.
Checklist run: items a–e as briefed, under `.claude/rules/mochiko/rust-cli.md` (`evals/contract/**` takes an independent non-author review); default FAIL; read-only git only.
Evidence read: `git diff HEAD -- evals/contract/run.py` (full and `-U0`), `evals/contract/run.py` lines 4837–4906 and 5078–5095, `grep -n golden` over the whole file, `git blame -L 4900,4906`, `python3 -m py_compile`, and the written verdict at `evals/.work/contract-reminder-spawn-17b34d3b/verdict.json`.

- (a) PASS — `golden` is bound nowhere: every one of the ten `golden` hits in the file sits inside a comment or an f-string, and the only real binding is the module-level constant `REMINDER_GOLDEN` (assigned at line 4124), a different name. No local, parameter, or comprehension target named `golden` exists in `case_reminder_spawn` or at module scope.
- (b) PASS — `marker` is bound at line 4862 as `REMINDER_GOLDEN.split(";")[0].strip()`, is the exact string both arms grep for in the sidechain transcripts, and is already the value passed to `report("the line both arms looked for", marker)` on line 4902. The evidence payload now records the same string the assertions used.
- (c) PASS — `python3 -m py_compile evals/contract/run.py` exits 0 with no output.
- (d) PASS — no other line in the file references the unbound name; the pre-existing early-return `write_verdict` on line 4856 omits `reminder_line` entirely and is unaffected.
- (e) PASS — a genuine integrity repair, not a behavior change. `git blame -L 4900,4906` shows every surrounding line at commit `5d8fc69c` (deepeshBodh, 2026-09-15), with only line 4904 uncommitted, so the `NameError` was committed before this wave. It fired in the evidence write after both `R-SPAWN-*` assertions had already been appended, so the case's findings were computed and then lost. The newest verdict JSON confirms the repaired path: both spawn checks `ok`, and `reminder_line` equal to the `report(...)` detail verbatim, the gate line naming `mochiko-cli home`.

Re-confirmation of the three pre-graded changes: exactly the `EXPECTED["setup"]` addition of `setup.gate-loop-bound` with its `0008-gate-form` comment, the `PROBE_ARGUMENTS` row for `validation-primitive-edit`, and the zero-baseline `against` guard on the delivery-case percentage (two hunks, the guard and its single call-site substitution). None of the three widened beyond its stated scope.

VERDICT: PASS
Issues requiring fix: none blocking. One non-blocking observation, recorded below.

## Notes of note

The diff carries five logical changes, not the four named in the brief. The fifth is at line 5087 in `main()`: a comment revising "a hundred and fifty-one metered sessions" to "a hundred and fifty-nine". It is prose inside a comment, has no executable effect, and the file byte-compiles clean, so it does not affect this unit's verdict.

It is flagged only because it was not in the briefed set and its figure is not derivable from this diff. `PROBE_ARGUMENTS` grew from 36 entries to 37, a delta of one, while the comment moved by eight. The suite carries no counter that computes the figure, so the number is hand-maintained prose and cannot be checked without spending the full metered run. Whoever owns the wave should confirm 159 against the actual sandbox session count, or leave the figure to the seat that can measure it.

audit: run.py fix-on-sight (reminder-spawn NameError) · validator · opus · 1 files · 1 rounds · 0 blocking
