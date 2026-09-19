import json
from pathlib import Path

import click
from sqlalchemy import delete, select

from quill.db import session
from quill.export import export_pdf
from quill.models import Note, Notebook


@click.group()
def main() -> None:
    pass


@main.command()
@click.option("--notebook", "slug", required=True)
@click.option("--format", "fmt", type=click.Choice(["pdf"]), default="pdf")
@click.option("--out", type=click.Path(path_type=Path), required=True)
@click.option("--no-cover", is_flag=True)
def export(slug: str, fmt: str, out: Path, no_cover: bool) -> None:
    export_pdf(slug, out, cover=not no_cover)


@main.command()
@click.option("--notebook", "slug", required=True)
@click.option("--from", "src", type=click.Path(path_type=Path, exists=True), required=True)
def seed(slug: str, src: Path) -> None:
    manifest = json.loads((src / "manifest.json").read_text())
    with session() as s:
        existing = s.scalar(select(Notebook).where(Notebook.slug == slug))
        if existing:
            s.execute(delete(Note).where(Note.notebook_id == existing.id))
            s.delete(existing)
        nb = Notebook(slug=slug, title=manifest["title"])
        for i, entry in enumerate(manifest["notes"]):
            note_path = src / entry["file"]
            body = (note_path.read_text() if note_path.exists()
                    else f"# {entry['title']}\n\nPlaceholder body for the demo notebook.\n")
            nb.notes.append(Note(position=i, title=entry["title"], body_md=body))
        s.add(nb)
        s.commit()
    click.echo(f"seeded notebook '{slug}' with {len(manifest['notes'])} notes")


if __name__ == "__main__":
    main()
