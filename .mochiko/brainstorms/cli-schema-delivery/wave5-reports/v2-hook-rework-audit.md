# V2 audit — wave 5 delta: the dependency-halt hook rework

**2026-09-04. Verdict: PASS.** Graded from the files and by running the matrix myself, not from
P2's report. Pre-edit reference: `7d098b9`. Binary built from this worktree, 0.1.0, grammar 1;
migration log at `plugins/mochiko/migrations/`, `migrate status` exit 0.

## 1. Diff scope

`git diff 7d098b9 -- plugins/mochiko/hooks/` touches exactly one file,
`plugins/mochiko/hooks/scripts/dependency-halt.sh`, at **+8/−2** by `--numstat`. No other hook
script changed and `hooks.json` is byte-unchanged. The change is the header comment amended to
"own commands and skills" plus the five-line resolution branch in the `UserPromptExpansion` arm,
and nothing else.

## 2. D7 conformance

**The gate is still absence or grammar skew only.** The two `block` calls are unchanged and
unreached by the new branch: one on `command -v mochiko-cli` failing, one on `migrate status`
exit 3. Every other non-zero status still falls through to `exit 0` on the same line as before, so
the hook still declines to vouch for a delivery it cannot verify. No judgment, no behavior gate.

**The converted check still gates unconverted skills silently.** The new branch only resolves a
path; the `grep -q -F '!\`mochiko-cli rules' "$primitive"` guard sits after it and is untouched. I
confirmed this is what silences an unconverted skill rather than a missing file:
`plugins/mochiko/skills/grooming-operating-docs/SKILL.md` exists and contains **zero** `!` lines,
so the fix does resolve its path and the converted check is what drops it. That is the meaningful
distinction, and it holds.

**An unknown name resolves to a non-existent path and stays silent.** For
`mochiko:no-such-thing`, neither `commands/no-such-thing.md` nor `skills/no-such-thing/SKILL.md`
exists; the grep fails and the hook exits 0. Verified by running the row and by checking both
paths are absent.

**POSIX sh, syntax, executable bit.** The new code uses only POSIX constructs — `${name#mochiko:}`
parameter expansion, `[ ! -f ... ]`, `case`. `sh -n` passes and so does `dash -n`, a stricter
check than P2 ran. I also re-ran both changed rows under `dash` directly and got byte-identical
output to `/bin/sh`, so the branch is not relying on a bash-ism. The file keeps mode `755`.
`shellcheck` is not installed on this host, matching P2's disclosure.

## 3. The matrix, run here

Run against the committed captures at `evals/contract/fixture/hook-input/` with `command_name`
(and `tool_input.skill` on the tool path) rewritten per row, `CLAUDE_PLUGIN_ROOT` at
`plugins/mochiko`, and "absent" removing `target/release` from `PATH`. Each row was run against
**both** the current script and `7d098b9`'s, so "unchanged" is measured, not asserted.

| row | state | result | vs pre-edit |
|---|---|---|---|
| `mochiko:review-brainstorm` | present | exit 0, context line, noun `skill` | **changed** (was silent) |
| `mochiko:review-brainstorm` | absent | **exit 2**, install line naming `/mochiko:review-brainstorm` | **changed** (was exit 0, silent) |
| `mochiko:specify` | present | exit 0, context line, noun `command` | unchanged |
| `mochiko:specify` | absent | exit 2, install line naming `/mochiko:specify` | unchanged |
| `other:thing` | both | exit 0, silent | unchanged |
| Skill tool `mochiko:review-brainstorm` | present | exit 0, context line, noun `skill` | unchanged |
| Skill tool `mochiko:review-brainstorm` | absent | exit 0, deny with install line | unchanged |
| Skill tool `upe:probe-skill` | both | exit 0, silent | unchanged |
| `mochiko:grooming-operating-docs` | present | exit 0, silent | unchanged |
| `mochiko:grooming-operating-docs` | absent | exit 0, silent | unchanged |
| `mochiko:no-such-thing` | both | exit 0, silent | unchanged |

Fourteen cells in total, of which **exactly two changed**, both the intended `review-brainstorm`
expansion rows. Every other cell is byte-identical in exit code, stdout, and stderr between the
two script versions. The two changed cells in full:

```
present: exit=0
{"hookSpecificOutput":{"hookEventName":"UserPromptExpansion","additionalContext":"mochiko-cli present · rules delivered by the skill's own render"}}

absent:  exit=2
mochiko-cli is not installed — /mochiko:review-brainstorm cannot run without it. Install: cargo install mochiko-cli
```

The `noun` substitution works: the same skill renders "the skill's own render" on the expansion
path and the same phrase on the tool path, while `mochiko:specify` still renders "the command's
own render".

## 4. Commands-first precedence

Scripted, not assumed: six command files and thirty-eight skill directories carrying a `SKILL.md`,
with **zero shared names**. Commands resolve first because the skill branch runs only when the
command file is absent, so the ordering is unambiguous today and would still favour a command if a
collision ever appeared. P2's claim reproduces.

## 5. Report honesty

P2's report survives checking. The described defect is real and its characterisation is right —
before the fix a converted skill exited 0 on the expansion path in **both** binary states, so the
absence case produced no install line at all, which is worse than merely unconfirmed. The
`hooks.json` reasoning is correct: the `"^mochiko:"` matcher is anchored on the namespace, not on
a command list, so the script was already firing for skill names and only the file resolution
inside it was wrong. Every matrix row P2 reported reproduces, including the three safety rows. The
ceremony call is right too: this is a pure addition, so it rides the decision row with no strip
note, and `plugins/mochiko/hooks/**` is inside the primitive-edit path scope, which is why this
audit is owed before the bump.

## Non-blocking observations

- **The context line on the expansion path is now emitted for converted skills as well as
  commands.** That is the intended consequence, but it means a `/mochiko:<skill>` prompt line now
  carries one extra line of additional context that it did not before. It is one short sentence and
  not a budgeted class, so nothing is over; noting it because the wave measured payloads carefully
  and this line is deliberately excluded from those figures as the harness's output.
- **`name` is still interpolated into a path without sanitising.** A name like
  `mochiko:../../something` would resolve to a path outside the plugin root, where the script would
  then run a literal-string `grep`. This is unchanged from the pre-edit script, which interpolated
  the same value the same way, so the rework introduces no new exposure and I am not failing it.
  The blast radius is a read of an attacker-named path by a hook whose input comes from the local
  session, which is why it has not mattered. Worth a backlog line rather than a wave-5 fix.

**Fix list: none.** Nothing blocks this delta.
