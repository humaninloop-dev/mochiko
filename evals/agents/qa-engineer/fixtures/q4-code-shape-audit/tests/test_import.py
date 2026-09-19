from pathlib import Path

from sqlalchemy import select

from postbox.importers.subscribers import CsvSubscriberImporter
from postbox.models import Subscriber


def test_import_counts_and_persists(session, tmp_path: Path):
    p = tmp_path / "s.csv"
    p.write_text("email,name,tags\na@x.io,Ann,vip\nA@X.IO,Ann again,\nnot-an-email,,\nb@x.io,Bo,\n")
    result = CsvSubscriberImporter().run(session, p)
    assert (result.imported, result.duplicates, result.invalid_lines) == (2, 1, [4])
    assert {s.email for s in session.scalars(select(Subscriber))} == {"a@x.io", "b@x.io"}


def test_import_skips_existing(session, tmp_path: Path):
    session.add(Subscriber(email="a@x.io"))
    session.commit()
    p = tmp_path / "s.csv"
    p.write_text("email\na@x.io\n")
    result = CsvSubscriberImporter().run(session, p)
    assert (result.imported, result.duplicates) == (0, 1)


def test_summary_line(session, tmp_path: Path):
    p = tmp_path / "s.csv"
    p.write_text("email\na@x.io\nbad\n")
    result = CsvSubscriberImporter().run(session, p)
    assert result.summary() == "Imported 1 subscribers (0 duplicates skipped, 1 invalid rows skipped (lines 3))"
