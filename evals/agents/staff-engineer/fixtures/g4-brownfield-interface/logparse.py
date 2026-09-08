"""Parse the service's plain-text log lines.

One entry per line, three whitespace-separated fields — an ISO-8601 UTC timestamp, a level,
and the message (which may itself contain spaces):

    2026-09-01T14:03:22Z INFO  worker started (pid 4242)
    2026-09-01T14:03:25Z ERROR queue full: dropped 3 messages

Every public function takes plain strings or lists of them and returns plain data; nothing
here reads files — `logcli.py` does that.
"""
import re

LEVELS = ("DEBUG", "INFO", "WARN", "ERROR")

_LINE = re.compile(r"^(?P<ts>\S+)\s+(?P<level>[A-Z]+)\s+(?P<msg>.*)$")


class ParseError(ValueError):
    """Raised for a line that does not fit the format; the message quotes the offending line."""


def parse_line(line):
    """Return `(level, message)` for one log line.

    Raises `ParseError` for a malformed line or an unknown level. The timestamp field is
    checked for presence and then dropped — nothing downstream has needed it so far.
    """
    match = _LINE.match(line.rstrip("\n"))
    if match is None:
        raise ParseError(f"malformed line: {line!r}")
    level = match.group("level")
    if level not in LEVELS:
        raise ParseError(f"unknown level {level!r} in line: {line!r}")
    return level, match.group("msg")


def count_by_level(lines):
    """Map every level in `LEVELS` to how many of `lines` carry it.

    Malformed lines are skipped, never raised — aggregators tolerate a torn tail line.
    """
    counts = {level: 0 for level in LEVELS}
    for line in lines:
        try:
            level, _message = parse_line(line)
        except ParseError:
            continue
        counts[level] += 1
    return counts


def errors_only(lines):
    """Messages of the ERROR lines in `lines`, in order; malformed lines are skipped."""
    messages = []
    for line in lines:
        try:
            level, message = parse_line(line)
        except ParseError:
            continue
        if level == "ERROR":
            messages.append(message)
    return messages
