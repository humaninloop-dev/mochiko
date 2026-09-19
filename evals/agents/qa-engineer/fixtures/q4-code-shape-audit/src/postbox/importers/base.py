"""Importer framework (T2.3): an abstract Importer, a registry, and the result shape."""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from pathlib import Path

from sqlalchemy.orm import Session


@dataclass
class ImportResult:
    imported: int = 0
    duplicates: int = 0
    invalid_lines: list[int] = field(default_factory=list)

    def summary(self) -> str:
        lines = ", ".join(str(n) for n in self.invalid_lines)
        return (f"Imported {self.imported} subscribers ({self.duplicates} duplicates skipped, "
                f"{len(self.invalid_lines)} invalid rows skipped (lines {lines}))")


class Importer(ABC):
    name: str

    @abstractmethod
    def run(self, session: Session, path: Path) -> ImportResult: ...


class ImporterRegistry:
    def __init__(self) -> None:
        self._importers: dict[str, type[Importer]] = {}

    def register(self, name: str, cls: type[Importer]) -> None:
        self._importers[name] = cls

    def get(self, name: str) -> Importer:
        try:
            return self._importers[name]()
        except KeyError:
            raise KeyError(f"no importer named {name!r}") from None

    def names(self) -> list[str]:
        return sorted(self._importers)


registry = ImporterRegistry()
