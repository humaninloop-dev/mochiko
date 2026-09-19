"""Paylane webhook receiver.

Paylane signs every delivery with HMAC-SHA256 over the raw body using the webhook secret;
we verify before parsing. `payment.succeeded` carries the invoice number in `metadata`.
"""

import hashlib
import hmac
import os
from decimal import Decimal

from fastapi import APIRouter, Depends, Header, HTTPException, Request
from sqlalchemy import select
from sqlalchemy.orm import Session

from tally.db import get_session
from tally.models import Invoice
from tally.services.reconcile import apply_payment

router = APIRouter()


def _verify(raw: bytes, signature: str | None) -> None:
    secret = os.environ.get("PAYLANE_WEBHOOK_SECRET", "")
    if not secret or not signature:
        raise HTTPException(401, "unsigned webhook")
    expected = hmac.new(secret.encode(), raw, hashlib.sha256).hexdigest()
    if not hmac.compare_digest(expected, signature):
        raise HTTPException(401, "bad signature")


@router.post("/webhooks/paylane", status_code=200)
async def paylane_webhook(
    request: Request,
    session: Session = Depends(get_session),
    paylane_signature: str | None = Header(default=None),
):
    raw = await request.body()
    _verify(raw, paylane_signature)
    event = await request.json()
    print(f"webhook received: {event['type']}")
    if event["type"] != "payment.succeeded":
        return {"ignored": event["type"]}
    number = event["data"]["metadata"]["invoice"]
    invoice = session.scalar(select(Invoice).where(Invoice.number == number))
    if invoice is None:
        raise HTTPException(404, "invoice not found")
    amount = Decimal(event["data"]["amount"]) / 100
    apply_payment(invoice, amount, "card", event["data"]["id"])
    session.commit()
    return {"invoice": number, "status": invoice.status}
