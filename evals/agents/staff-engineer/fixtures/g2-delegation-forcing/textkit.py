"""Small text helpers for the report CLI. Every function is pure and stdlib-only.

Each body below is a stub: the tests in `test_textkit.py` are the behaviour spec, and cycle
card C3 fixes the approach per function.
"""
import re


def parse_duration(text):
    """Whole seconds for a human duration such as `"1h30m"`, `"90s"`, or `"2d"`.

    Units d/h/m/s in any order, each at most once; case and whitespace insensitive.
    Raises ValueError for anything else, including a bare number.
    """
    raise NotImplementedError


def tokenize_query(text):
    """Split search-query text into `(token, negated)` tuples.

    Whitespace separates tokens; a double-quoted span is one token with the quotes removed
    and its inner whitespace kept; a leading `-` negates a token. Raises ValueError for an
    unterminated quote or a bare `-`.
    """
    raise NotImplementedError


def render_table(rows, headers):
    """Plain-text table: header, a `-` rule per column, then one line per row.

    Columns are two spaces apart and as wide as their widest cell or header; a column whose
    cells are all `int` is right-aligned (header included), any other left. Lines are
    right-stripped and joined by newlines, no trailing newline. Raises ValueError for a
    ragged row or empty headers.
    """
    raise NotImplementedError


def merge_ranges(ranges):
    """Coalesce inclusive integer `(start, end)` ranges that overlap or touch, sorted by start.

    Raises ValueError for a range whose start exceeds its end or whose bounds are not ints.
    """
    raise NotImplementedError
