#!/bin/sh
# The dependency halt (record D7b). Registered twice: on UserPromptExpansion for this plugin's
# own commands and skills, and on PreToolUse/Skill for this plugin's own skills.
#
# The gate is on dependency absence only, never on behavior or judgment (GI-019). It fires for
# mochiko's own primitives, only where that primitive actually takes its rules from the binary,
# and only when the binary is missing or the migration log sits outside the binary's grammar
# range. Everything else proceeds untouched. It renders nothing: the measured platform behavior
# is that UserPromptExpansion sees the raw user line and runs before expansion, so this hook
# cannot tell whether the command's own delivery slot already fired. Injecting rules here would
# therefore double-deliver on every fire, so it confirms presence in one line and stops.
#
# POSIX sh only. Depends on grep, sed, tr, and command -v; deliberately not on jq.

set -u

ROOT=${CLAUDE_PLUGIN_ROOT:-$(cd "$(dirname "$0")/../.." 2>/dev/null && pwd)}

input=$(cat)

# Pull one string field out of the hook's JSON stdin. The first match wins, so a value echoed
# back inside a later field cannot shadow the real one.
field() {
	printf '%s' "$input" |
		tr -d '\n' |
		grep -o "\"$1\"[[:space:]]*:[[:space:]]*\"[^\"]*\"" |
		head -n 1 |
		sed 's/.*"\([^"]*\)"$/\1/'
}

# Escape a string for use as a JSON string value.
escape() { sed -e 's/\\/\\\\/g' -e 's/"/\\"/g' | tr '\n' ' '; }

event=$(field hook_event_name)

case "$event" in
UserPromptExpansion)
	name=$(field command_name)
	bare=${name#mochiko:}
	primitive="$ROOT/commands/$bare.md"
	noun=command
	# A `/mochiko:<skill>` prompt line takes this path too, and resolves to no command file.
	if [ ! -f "$primitive" ]; then
		primitive="$ROOT/skills/$bare/SKILL.md"
		noun=skill
	fi
	;;
PreToolUse)
	[ "$(field tool_name)" = "Skill" ] || exit 0
	name=$(field skill)
	primitive="$ROOT/skills/${name#mochiko:}/SKILL.md"
	noun=skill
	;;
*)
	exit 0
	;;
esac

# Someone else's command or skill. Not our dependency, not our business.
case "$name" in
mochiko:*) ;;
*) exit 0 ;;
esac

# The converted check. A primitive that still reads a shipped schema file is covered by the
# transition clause and is never gated; one whose rules come from the binary is. The primitive's
# own file is the truth here, so there is no list to keep in sync with the conversion waves.
grep -q -F '!`mochiko-cli rules' "$primitive" 2>/dev/null || exit 0

block() {
	if [ "$event" = "UserPromptExpansion" ]; then
		printf '%s\n' "$1" >&2
		exit 2
	fi
	printf '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"%s"}}\n' \
		"$(printf '%s' "$1" | escape)"
	exit 0
}

context() {
	printf '{"hookSpecificOutput":{"hookEventName":"%s","additionalContext":"%s"}}\n' \
		"$event" "$(printf '%s' "$1" | escape)"
	exit 0
}

if ! command -v mochiko-cli >/dev/null 2>&1; then
	block "mochiko-cli is not installed — /$name cannot run without it. Install: cargo install mochiko-cli"
fi

status=$(mochiko-cli migrate status --plugin-root "$ROOT" 2>&1)
status_code=$?

# Exit 3 is the version contract, and its message is the binary's own.
[ "$status_code" -eq 3 ] && block "$status"

# Every other non-zero exit is left alone on purpose: this hook gates absence and grammar skew,
# and nothing else. It stays silent rather than confirming a delivery it cannot vouch for — the
# command's own render will fail on the same broken log, and its halt clause catches that.
[ "$status_code" -eq 0 ] || exit 0

context "mochiko-cli present · rules delivered by the $noun's own render"
