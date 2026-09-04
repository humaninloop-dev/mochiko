#!/bin/sh
# Maintainer-side only. This lives in the repository's own .claude/ directory and is never part
# of the shipped plugin: only plugins/mochiko/ ships, and the plugin's own hooks are separate.
#
# After an edit or write under the migration log, replay the log and print what the checks found.
# Purely advisory (record D7c) — it always exits 0, and it gates nothing. The point is to catch a
# malformed migration at the moment it is written rather than at the next fire.

set -u

ROOT=$(cd "$(dirname "$0")/../.." 2>/dev/null && pwd) || exit 0

file_path=$(
	tr -d '\n' |
		grep -o '"file_path"[[:space:]]*:[[:space:]]*"[^"]*"' |
		head -n 1 |
		sed 's/.*"\([^"]*\)"$/\1/'
)

case "$file_path" in
*plugins/mochiko/migrations/*) ;;
*) exit 0 ;;
esac

command -v mochiko-cli >/dev/null 2>&1 || exit 0

mochiko-cli migrate validate --report --plugin-root "$ROOT/plugins/mochiko" 2>&1

exit 0
