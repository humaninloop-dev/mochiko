from pathlib import Path

import click

from postbox.db import SessionLocal
from postbox.events import bus
from postbox.importers import registry


@click.group()
def main() -> None:
    pass


@main.group("import")
def import_group() -> None:
    pass


def _print_summary(payload: dict) -> None:
    click.echo(payload["result"].summary())


bus.subscribe("import.finished", _print_summary)


@import_group.command("subscribers")
@click.argument("path", type=click.Path(path_type=Path, exists=True))
def import_subscribers(path: Path) -> None:
    with SessionLocal() as session:
        registry.get("subscribers").run(session, path)


if __name__ == "__main__":
    main()
