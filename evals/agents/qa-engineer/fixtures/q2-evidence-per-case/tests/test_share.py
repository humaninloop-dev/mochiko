from datetime import datetime, timedelta

from quill.models import Notebook, Share


def test_share_row_round_trips(db):
    nb = Notebook(slug="demo", title="Demo")
    db.add(nb)
    db.commit()
    share = Share(token="abc", notebook_id=nb.id, expires_at=datetime.utcnow() + timedelta(days=7))
    db.add(share)
    db.commit()
    assert db.get(Share, share.id).token == "abc"
