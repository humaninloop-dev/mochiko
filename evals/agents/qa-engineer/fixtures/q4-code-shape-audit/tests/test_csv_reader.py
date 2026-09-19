from pathlib import Path

from postbox.importers.csv_reader import read_rows


def test_plain_rows(tmp_path: Path):
    p = tmp_path / "a.csv"
    p.write_text("email,name\na@x.io,Ann\nb@x.io,Bo\n")
    assert list(read_rows(p)) == [(1, ["email", "name"]), (2, ["a@x.io", "Ann"]), (3, ["b@x.io", "Bo"])]


def test_quoted_field_with_comma_and_newline(tmp_path: Path):
    p = tmp_path / "a.csv"
    p.write_text('email,name\na@x.io,"Ann, the\nfirst"\nb@x.io,Bo\n')
    rows = list(read_rows(p))
    assert rows[1] == (2, ["a@x.io", "Ann, the\nfirst"])
    assert rows[2][0] == 4


def test_doubled_quote(tmp_path: Path):
    p = tmp_path / "a.csv"
    p.write_text('email,name\na@x.io,"say ""hi"""\n')
    assert list(read_rows(p))[1] == (2, ["a@x.io", 'say "hi"'])
