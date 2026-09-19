"""Render a notebook to PDF: an optional cover page, then one section per note."""

from pathlib import Path

import markdown
from sqlalchemy import select
from weasyprint import HTML

from quill.db import session
from quill.models import Notebook

PAGE_CSS = "@page { size: A4; margin: 18mm } pre { white-space: pre-wrap }"


def render_html(notebook: Notebook, cover: bool = True) -> str:
    parts: list[str] = []
    if cover:
        parts.append(f"<section class='cover'><h1>{notebook.title}</h1></section>")
    for note in notebook.notes:
        body = markdown.markdown(note.body_md, extensions=["fenced_code", "tables"])
        parts.append(f"<section><h2>{note.title}</h2>{body}</section>")
    return "<html><body>" + "".join(parts) + "</body></html>"


def export_pdf(slug: str, out: Path, cover: bool = True) -> int:
    with session() as s:
        notebook = s.scalar(select(Notebook).where(Notebook.slug == slug))
        if notebook is None:
            raise SystemExit(f"no notebook '{slug}'")
        html = render_html(notebook, cover=cover)
        pages = len(notebook.notes) + (1 if cover else 0)
    out.parent.mkdir(parents=True, exist_ok=True)
    HTML(string=html).write_pdf(str(out), stylesheets=[PAGE_CSS])
    print(f"exported {pages} page(s) to {out}")
    return pages
