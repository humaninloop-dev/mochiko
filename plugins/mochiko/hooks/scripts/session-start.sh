#!/bin/sh
# SessionStart — report whether the plugin's binary dependency is present, which version it is,
# and whether the migration log this plugin carries is inside that binary's grammar range.
#
# This hook is loud and never blocking (record D7a): it always exits 0, whatever it finds. The
# halt that matters lives at fire time, in the command's own delivery slot and in
# dependency-halt.sh. Reporting a missing dependency before the first fire is the whole job.
#
# POSIX sh only. Depends on grep, sed, tr, and command -v; deliberately not on jq, which is not
# present on every machine a consuming project runs on.

set -u

# The plugin root. Claude Code substitutes ${CLAUDE_PLUGIN_ROOT} into the hook's command string,
# so $0 is already absolute; deriving the root from it keeps the script working even where the
# variable is not also exported into the environment.
ROOT=${CLAUDE_PLUGIN_ROOT:-$(cd "$(dirname "$0")/../.." 2>/dev/null && pwd)}

input=$(cat)

# Pull one string field out of JSON on stdin. The first match wins, so a value echoed back inside
# a later field (a prompt quoting a key name, say) cannot shadow the real one.
extract() {
	tr -d '\n' |
		grep -o "\"$1\"[[:space:]]*:[[:space:]]*\"[^\"]*\"" |
		head -n 1 |
		sed 's/.*"\([^"]*\)"$/\1/'
}

field() { printf '%s' "$input" | extract "$1"; }

if ! command -v mochiko-cli >/dev/null 2>&1; then
	echo "mochiko-cli is not installed — the mochiko plugin depends on it; every converted command halts until it is. Install: cargo install mochiko-cli"
else
	# Already formatted as "mochiko-cli <version> · grammar <low>..<high>".
	version_line=$(mochiko-cli --version 2>/dev/null)
	status=$(mochiko-cli migrate status --plugin-root "$ROOT" 2>&1)
	status_code=$?

	if [ "$status_code" -eq 3 ]; then
		# The version contract. The binary's own message is the only accurate thing to say.
		printf '%s\n' "$status"
	elif [ "$status_code" -eq 0 ]; then
		log_grammar=$(printf '%s' "$status" | sed -n '1s/.*· grammar \([0-9][0-9]*\) ·.*/\1/p')
		plugin_version=$(extract version <"$ROOT/.claude-plugin/plugin.json" 2>/dev/null)
		[ -n "$plugin_version" ] || plugin_version="unknown"
		echo "$version_line · plugin $plugin_version · log grammar $log_grammar · in range"
	else
		# The log is absent, empty, or unsound. Nothing is blocked here, but staying silent
		# would hide a real breakage until the first fire, so report both lines.
		printf '%s\n%s\n' "$version_line" "$status"
	fi
fi

# Best effort, and deliberately quiet on failure: a false negative costs nothing, and this hook
# must never fail because a settings file was unreadable or shaped unexpectedly.
cwd=$(field cwd)
for settings in "$HOME/.claude/settings.json" "${cwd:-.}/.claude/settings.json"; do
	if grep -q 'disableSkillShellExecution' "$settings" 2>/dev/null; then
		echo "This environment disables skill shell execution, which the mochiko plugin declares unsupported: rules are delivered through inline command execution and cannot be delivered without it."
		break
	fi
done

exit 0
