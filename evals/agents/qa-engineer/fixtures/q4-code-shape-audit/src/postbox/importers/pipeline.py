"""Stage pipeline for importers (T2.3)."""

from abc import ABC, abstractmethod
from dataclasses import dataclass

from sqlalchemy import select
from sqlalchemy.orm import Session

from postbox.importers.base import ImportResult
from postbox.models import Subscriber
from postbox.util.email import is_valid_email, normalize_email


@dataclass
class Row:
    line: int
    email: str
    name: str | None = None
    tags: str = ""


class Stage(ABC):
    @abstractmethod
    def process(self, row: Row, ctx: "Context") -> Row | None: ...


@dataclass
class Context:
    session: Session
    result: ImportResult
    seen: set[str]


class NormalizeStage(Stage):
    def process(self, row: Row, ctx: Context) -> Row | None:
        row.email = normalize_email(row.email)
        return row


class ValidateStage(Stage):
    def process(self, row: Row, ctx: Context) -> Row | None:
        if not is_valid_email(row.email):
            ctx.result.invalid_lines.append(row.line)
            return None
        return row


class DedupeStage(Stage):
    def process(self, row: Row, ctx: Context) -> Row | None:
        if row.email in ctx.seen or ctx.session.scalar(
            select(Subscriber.id).where(Subscriber.email == row.email)
        ):
            ctx.result.duplicates += 1
            return None
        ctx.seen.add(row.email)
        return row


class PersistStage(Stage):
    def process(self, row: Row, ctx: Context) -> Row | None:
        ctx.session.add(Subscriber(email=row.email, name=row.name, tags=row.tags, source="import"))
        ctx.result.imported += 1
        return row


class Pipeline:
    def __init__(self, stages: list[Stage]):
        self.stages = stages

    def run(self, rows: list[Row], ctx: Context) -> ImportResult:
        for row in rows:
            current: Row | None = row
            for stage in self.stages:
                if current is None:
                    break
                current = stage.process(current, ctx)
        ctx.session.commit()
        return ctx.result
