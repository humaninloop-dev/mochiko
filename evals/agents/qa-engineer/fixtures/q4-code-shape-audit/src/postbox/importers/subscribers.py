from pathlib import Path

from sqlalchemy.orm import Session

from postbox.events import bus
from postbox.importers.base import Importer, ImportResult
from postbox.importers.csv_reader import read_rows
from postbox.importers.pipeline import (
    Context, DedupeStage, NormalizeStage, PersistStage, Pipeline, Row, ValidateStage,
)


class CsvSubscriberImporter(Importer):
    name = "subscribers"

    def run(self, session: Session, path: Path) -> ImportResult:
        rows: list[Row] = []
        for line, fields in read_rows(path):
            if line == 1 and fields and fields[0].lower() == "email":
                continue
            email = fields[0] if fields else ""
            name = fields[1] if len(fields) > 1 and fields[1] else None
            tags = fields[2] if len(fields) > 2 else ""
            rows.append(Row(line=line, email=email, name=name, tags=tags))
        ctx = Context(session=session, result=ImportResult(), seen=set())
        result = Pipeline([NormalizeStage(), ValidateStage(), DedupeStage(), PersistStage()]).run(rows, ctx)
        bus.publish("import.finished", {"result": result})
        return result
