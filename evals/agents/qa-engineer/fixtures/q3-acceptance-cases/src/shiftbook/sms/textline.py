"""Textline SMS adapter (real client). Sandbox accounts deliver only to registered handsets."""

import os

import httpx

TEXTLINE_API = "https://api.textline.example/v2"


class Textline:
    def __init__(self, api_key: str | None = None, sender: str | None = None):
        self.api_key = api_key or os.environ["TEXTLINE_API_KEY"]
        self.sender = sender or os.environ["TEXTLINE_FROM"]

    def send(self, to: str, body: str) -> str:
        r = httpx.post(
            f"{TEXTLINE_API}/messages",
            headers={"authorization": f"Bearer {self.api_key}"},
            json={"from": self.sender, "to": to, "body": body},
            timeout=15,
        )
        r.raise_for_status()
        message_id: str = r.json()["id"]
        print(f"textline: sent {message_id} to {to}")
        return message_id

    def status(self, message_id: str) -> str:
        r = httpx.get(f"{TEXTLINE_API}/messages/{message_id}",
                      headers={"authorization": f"Bearer {self.api_key}"}, timeout=15)
        r.raise_for_status()
        state: str = r.json()["status"]   # queued | sent | delivered | failed
        return state
