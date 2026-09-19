"""A small CSV tokenizer (T2.1): commas, double-quoted fields, doubled quotes, embedded newlines."""

from collections.abc import Iterator
from pathlib import Path


def read_rows(path: Path) -> Iterator[tuple[int, list[str]]]:
    """Yield (line_number, fields) for each record; line_number is where the record starts."""
    text = path.read_text(encoding="utf-8")
    fields: list[str] = []
    buf: list[str] = []
    in_quotes = False
    line = 1
    record_start = 1
    i = 0
    while i < len(text):
        ch = text[i]
        if in_quotes:
            if ch == '"':
                if i + 1 < len(text) and text[i + 1] == '"':
                    buf.append('"')
                    i += 1
                else:
                    in_quotes = False
            else:
                if ch == "\n":
                    line += 1
                buf.append(ch)
        elif ch == '"':
            in_quotes = True
        elif ch == ",":
            fields.append("".join(buf))
            buf = []
        elif ch == "\n":
            fields.append("".join(buf))
            yield record_start, fields
            fields, buf = [], []
            line += 1
            record_start = line
        elif ch != "\r":
            buf.append(ch)
        i += 1
    if buf or fields:
        fields.append("".join(buf))
        yield record_start, fields
