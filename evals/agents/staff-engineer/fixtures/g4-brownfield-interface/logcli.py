"""Tiny CLI over logparse: `python3 logcli.py <verb> <logfile>`, verbs: levels | errors | show."""
import sys

from logparse import ParseError, count_by_level, errors_only, parse_line


def _read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read().splitlines()


def main(argv):
    if len(argv) != 3:
        print(__doc__)
        return 2
    verb, path = argv[1], argv[2]
    lines = _read(path)
    if verb == "levels":
        for level, count in count_by_level(lines).items():
            print(f"{level:<5} {count}")
    elif verb == "errors":
        for message in errors_only(lines):
            print(message)
    elif verb == "show":
        for line in lines:
            try:
                level, message = parse_line(line)
            except ParseError as exc:
                print(f"?     {exc}")
                continue
            print(f"{level:<5} {message}")
    else:
        print(__doc__)
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
