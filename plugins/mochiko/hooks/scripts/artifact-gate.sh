#!/bin/sh
# PreToolUse — the write-time artifact gate (record D1, D3, D7c; ledger GI-019 clause iv).
#
# Registered on Write|Edit un-narrowed, and on Bash|PowerShell narrowed to commands whose text
# names `.mochiko`. Every declared artifact home sits under `.mochiko/`, so that narrowing keeps
# the shell arm off the run's ordinary command volume; the Write|Edit arm runs un-narrowed because
# the out-of-home report sniff (record D9) exists precisely to catch a report written outside every
# home, and any `if` that could express "under a home" would defeat it.
#
# The script holds no rule and parses nothing. It hands its own stdin to the binary untouched and
# prints what comes back. That is deliberate: a wave-0 probe measured the shipped grep-and-sed
# field reader truncating `printf 'retry probe' > "<abs>/probe-home/c.md"` at the first escaped
# quote and allowing a write it should have denied (record I4). The payload is parsed in Rust.
#
# Only exit 4 — a conformance denial — becomes a deny. Exit 1 (unsound log), 2 (unreadable payload
# or usage error) and 3 (grammar skew) are pass-throughs: a binary that cannot read its own log
# must never deny a consumer's write, and absence and skew already halt loudly elsewhere
# (session-start.sh, dependency-halt.sh).
#
# POSIX sh only. Depends on command -v and printf; deliberately not on jq.

set -u

# Claude Code substitutes ${CLAUDE_PLUGIN_ROOT} into the hook's command string, so $0 is already
# absolute; deriving the root from it keeps the script working where the variable is not exported.
ROOT=${CLAUDE_PLUGIN_ROOT:-$(cd "$(dirname "$0")/../.." 2>/dev/null && pwd)}

# The explicit allow. Empty stdout is never a verdict here: the platform denies a background
# subagent's call when no hook returns a decision (wave 0, leg 1 / the guide's Limitations), so a
# silent pass would read as a deny on exactly the transport every producing seat rides.
allow() {
	printf '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}\n'
	exit 0
}

# Fail-open on absence (record D7c). The dependency halt is the loud surface for a missing binary;
# a gate that denied every write without it would wedge a consumer's repository.
command -v mochiko-cli >/dev/null 2>&1 || allow

# stdin passes through untouched. `--plugin-root` names the log this plugin carries.
out=$(mochiko-cli check --hook-json - --plugin-root "$ROOT" 2>/dev/null)
code=$?

# The only deny. Its text is the binary's own, including the advisory halt sentence (record D9).
# The emptiness guard is the same one the exit-0 branch carries: a deny with nothing to say cannot
# be rendered as a decision, and printing nothing would read as a deny on a background subagent
# while telling the seat nothing. It falls to the explicit allow below — fail-open, D7c floor.
if [ "$code" -eq 4 ] && [ -n "$out" ]; then
	printf '%s\n' "$out"
	exit 0
fi

# Exit 0 already carries an explicit allow, and carries `additionalContext` when a first-touch
# amnesty let a standing violation through, so it is echoed rather than replaced.
if [ "$code" -eq 0 ] && [ -n "$out" ]; then
	printf '%s\n' "$out"
	exit 0
fi

# 1, 2, 3, and anything unexpected. The wrapper's own exit status is always 0: a non-zero wrapper
# is a platform-level failure, never a verdict. A platform-level timeout is outside this script's
# control — it returns no decision at all, which is why `timeout: 5` is generous against a check
# measured in tens of milliseconds.
allow
