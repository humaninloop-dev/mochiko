"""Share-preview worker: pops share tokens off the Redis list and renders a first-page PNG."""

import sys
import time
from pathlib import Path

import redis
from sqlalchemy import select

from quill.db import REDIS_URL, session
from quill.models import Notebook, Share

QUEUE = "quill:share-previews"
PREVIEWS = Path("/tmp/quill-previews")


def render_preview(token: str) -> Path:
    with session() as s:
        share = s.scalar(select(Share).where(Share.token == token))
        if share is None:
            raise LookupError(token)
        nb = s.get(Notebook, share.notebook_id)
        assert nb is not None
        out = PREVIEWS / f"{token}.png"
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_bytes(b"\x89PNG\r\n\x1a\n")   # placeholder until the renderer lands
        share.preview_path = str(out)
        s.commit()
    return out


def main() -> None:
    r = redis.Redis.from_url(REDIS_URL)
    print("worker listening on", QUEUE, flush=True)
    while True:
        item = r.blpop([QUEUE], timeout=5)
        if item is None:
            continue
        token = item[1].decode()
        started = time.monotonic()
        try:
            path = render_preview(token)
        except LookupError:
            print(f"unknown share {token}; dropped", file=sys.stderr, flush=True)
            continue
        print(f"rendered share preview {token} -> {path} ({time.monotonic() - started:.2f}s)", flush=True)


if __name__ == "__main__":
    main()
