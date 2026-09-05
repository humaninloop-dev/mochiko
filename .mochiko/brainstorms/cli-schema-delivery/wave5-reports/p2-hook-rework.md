# Wave 5 — P2 hook rework: the expansion path resolves skills

**2026-09-04.** P3 measured that a `/mochiko:<skill>` prompt line takes the `UserPromptExpansion`
path. On that path `dependency-halt.sh` resolved `commands/<name>.md` only, found nothing for a
skill, and exited 0 — so a converted skill was ungated by the hook there. This is the approved
five-line fix plus the header-comment amendment. Nothing else changed.

## The defect, measured before the fix

Worse than merely unconfirmed. On the expansion path a converted skill exited 0 silently with the
binary **present and absent alike**, so the absence case produced no install line at all. The
`PreToolUse`/`Skill` limb was correct throughout and served as the regression control, and the
harness still failed closed independently.

## The diff

```diff
--- a/plugins/mochiko/hooks/scripts/dependency-halt.sh
+++ b/plugins/mochiko/hooks/scripts/dependency-halt.sh
@@ -1,6 +1,6 @@
 #!/bin/sh
 # The dependency halt (record D7b). Registered twice: on UserPromptExpansion for this plugin's
-# own commands, and on PreToolUse/Skill for this plugin's own skills.
+# own commands and skills, and on PreToolUse/Skill for this plugin's own skills.
 #
@@ -36,8 +36,14 @@ event=$(field hook_event_name)
 case "$event" in
 UserPromptExpansion)
 	name=$(field command_name)
-	primitive="$ROOT/commands/${name#mochiko:}.md"
+	bare=${name#mochiko:}
+	primitive="$ROOT/commands/$bare.md"
 	noun=command
+	# A `/mochiko:<skill>` prompt line takes this path too, and resolves to no command file.
+	if [ ! -f "$primitive" ]; then
+		primitive="$ROOT/skills/$bare/SKILL.md"
+		noun=skill
+	fi
 	;;
```

Commands resolve first, so a command always wins a name contest. There are zero name collisions
between the six commands and the thirty skills today, checked rather than assumed, so the ordering
is unambiguous in the current tree and correct if that ever changes. `sh -n` passes; shellcheck is
not installed on this host, so it was not run.

## `hooks.json` — no change required

The `UserPromptExpansion` registration matches `"^mochiko:"`, anchored on the namespace rather than
on any list of command names, so a skill invocation already matched and the script already fired
for it. The before-run proves that positively: the script ran on a skill name and exited 0, which
it could only do having been invoked. The registration was never at fault — only the file
resolution inside the script was. Nothing in `hooks.json` keys on whether a primitive is a command
or a skill.

## Test matrix — before and after

Run on the host against the committed captures at `evals/contract/fixture/hook-input/`, with
`command_name` (and for the tool path, `tool_input.skill`) rewritten per row and
`CLAUDE_PLUGIN_ROOT` pointed at the real plugin. "Absent" removes `target/release` from `PATH`.

| row | before | after |
|---|---|---|
| `mochiko:review-brainstorm` · present | exit 0, no output | exit 0, context line naming the **skill** |
| `mochiko:review-brainstorm` · absent | exit 0, no output | **exit 2**, install line |
| `mochiko:specify` · present | exit 0, context line naming the command | unchanged |
| `mochiko:specify` · absent | exit 2, install line | unchanged |
| `other:thing` · present | exit 0, silent | unchanged |
| Skill tool, `mochiko:review-brainstorm` · present | context line naming the skill | unchanged |
| Skill tool, `mochiko:review-brainstorm` · absent | deny with install line | unchanged |
| Skill tool, `upe:probe-skill` · present | exit 0, silent | unchanged |

A diff of the full before and after transcripts shows exactly two changed rows, both intended;
every other row is byte-identical.

The two changed rows in full, after the fix:

```
--- review-brainstorm · PRESENT
exit=0
stdout: {"hookSpecificOutput":{"hookEventName":"UserPromptExpansion","additionalContext":"mochiko-cli present · rules delivered by the skill's own render"}}

--- review-brainstorm · ABSENT
exit=2
stderr: mochiko-cli is not installed — /mochiko:review-brainstorm cannot run without it. Install: cargo install mochiko-cli
```

## Safety rows — the converted check still gates

Added beyond the eight, to confirm the fix opens no escape hatch. All silent, exit 0:

| row | result |
|---|---|
| `mochiko:grooming-operating-docs` (unconverted skill) · present | exit 0, silent |
| `mochiko:grooming-operating-docs` · absent | exit 0, silent |
| `mochiko:no-such-thing` (neither) · absent | exit 0, silent |

An unconverted skill carries no `!` line, so the converted check at line 62 exits 0 — it is covered
by the GI-020 transition clause and correctly never gated. A name that is neither a command nor a
skill resolves to a path that does not exist, the check fails, and the hook stays silent.

## Ceremony

Additive change — a new resolution branch and a corrected comment, with nothing removed or
superseded — so it rides the decision row with no strip note, per the standing rule that pure
additions need none. `plugins/mochiko/hooks/**` is inside the primitive-edit path scope, so it
still owes the independent author≠grader audit before the `plugin.json` bump; the lead has queued
that as a V2 delta. Not committed.
