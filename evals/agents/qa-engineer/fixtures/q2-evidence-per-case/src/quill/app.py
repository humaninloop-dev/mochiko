import secrets
from datetime import datetime, timedelta

import redis
from fastapi import FastAPI, HTTPException, Request
from fastapi.responses import HTMLResponse
from pydantic import BaseModel, Field
from sqlalchemy import select

from quill.db import REDIS_URL, session
from quill.models import Notebook, Share

app = FastAPI(title="Quill")
QUEUE = "quill:share-previews"


class ShareIn(BaseModel):
    notebook: str
    expires_days: int = Field(default=7, ge=1, le=90)


@app.post("/share", status_code=201)
def create_share(body: ShareIn, request: Request) -> dict[str, str]:
    with session() as s:
        nb = s.scalar(select(Notebook).where(Notebook.slug == body.notebook))
        if nb is None:
            raise HTTPException(404, "notebook not found")
        share = Share(token=secrets.token_urlsafe(16), notebook_id=nb.id,
                      expires_at=datetime.utcnow() + timedelta(days=body.expires_days))
        s.add(share)
        s.commit()
        token = share.token
    redis.Redis.from_url(REDIS_URL).rpush(QUEUE, token)
    return {"token": token, "url": f"{str(request.base_url).rstrip('/')}/s/{token}"}


@app.get("/s/{token}", response_class=HTMLResponse)
def share_page(token: str) -> str:
    with session() as s:
        share = s.scalar(select(Share).where(Share.token == token))
        if share is None or share.expires_at < datetime.utcnow():
            raise HTTPException(404, "share not found")
        nb = s.get(Notebook, share.notebook_id)
        assert nb is not None
        preview = f"<img src='/previews/{token}.png'>" if share.preview_path else "<p>preview pending</p>"
        return (f"<!doctype html><title>{nb.title}</title><h1>{nb.title}</h1>{preview}"
                f"<a href='/s/{token}/download'>Download PDF</a>")
