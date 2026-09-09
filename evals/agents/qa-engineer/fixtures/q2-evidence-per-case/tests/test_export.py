from quill.export import render_html
from quill.models import Note, Notebook


def test_render_html_has_cover_and_sections():
    nb = Notebook(slug="t", title="T")
    nb.notes = [Note(position=0, title="A", body_md="hello"), Note(position=1, title="B", body_md="```py\nx=1\n```")]
    html = render_html(nb)
    assert "class='cover'" in html
    assert html.count("<section>") == 2
    assert "<code" in html


def test_render_html_without_cover():
    nb = Notebook(slug="t", title="T")
    nb.notes = [Note(position=0, title="A", body_md="hello")]
    assert "cover" not in render_html(nb, cover=False)
