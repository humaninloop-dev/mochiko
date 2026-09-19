#!/bin/sh
# SubagentStart — one line of context for every spawned seat (record D1 as amended 2026-09-13,
# user ruling R5).
#
# Wave 0 measured that a SubagentStart hook's hookSpecificOutput.additionalContext lands in the
# subagent's own transcript as a hook_additional_context attachment before its first turn, and that
# `agent_type` is `general-purpose` for named and unnamed spawns alike — so the registration carries
# no matcher and every seat gets the same line.
#
# The line below is frozen: the plugin contract suite reads it as a golden, so rewording it is a
# row change, not an edit. It points at the tool rather than restating any rule — the homes, the
# file sets and the budgets all live in the migration log, and `mochiko-cli home <path>` is how a
# seat reads them.
#
# This hook never blocks and never emits a permissionDecision key: SubagentStart is not a
# permission event, and a reminder that could deny would be a second gate nobody ruled on.
#
# POSIX sh only. Depends on grep, tr, command -v and printf; deliberately not on jq.

set -u

input=$(cat)

# A presence test over the whole payload, never field extraction: wave 0's false allow came from a
# grep-and-sed field reader truncating at an escaped quote, and this script must not repeat it.
printf '%s' "$input" | tr -d '\n' |
	grep -q '"hook_event_name"[[:space:]]*:[[:space:]]*"SubagentStart"' || exit 0

# Fail-open on absence (record D7c). The line tells a seat to run the binary, so it is worse than
# useless where the binary is not installed; session-start.sh is the loud surface for that.
command -v mochiko-cli >/dev/null 2>&1 || exit 0

printf '%s\n' '{"hookSpecificOutput":{"hookEventName":"SubagentStart","additionalContext":"mochiko gate: artifacts under declared homes take their shape from `mochiko-cli home <path>`; existing files are not templates."}}'
exit 0
