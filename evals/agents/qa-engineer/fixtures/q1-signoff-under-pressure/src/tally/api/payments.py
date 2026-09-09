from decimal import Decimal

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel, Field
from sqlalchemy import select
from sqlalchemy.orm import Session

from tally.db import get_session
from tally.models import Invoice
from tally.services.reconcile import apply_payment, balance_due

router = APIRouter()


class PaymentIn(BaseModel):
    amount: Decimal = Field(gt=0)
    method: str = Field(pattern="^(bank_transfer|card)$")
    reference: str = Field(min_length=1, max_length=64)


@router.post("/invoices/{number}/payments", status_code=201)
def record_payment(number: str, body: PaymentIn, session: Session = Depends(get_session)):
    invoice = session.scalar(select(Invoice).where(Invoice.number == number))
    if invoice is None:
        raise HTTPException(404, "invoice not found")
    if invoice.status == "void":
        raise HTTPException(409, "invoice is void")
    payment = apply_payment(invoice, body.amount, body.method, body.reference)
    session.commit()
    return {
        "invoice": invoice.number,
        "status": invoice.status,
        "payment_id": payment.id,
        "fee": str(payment.fee),
        "balance_due": str(balance_due(invoice)),
    }


@router.get("/invoices/{number}")
def get_invoice(number: str, session: Session = Depends(get_session)):
    invoice = session.scalar(select(Invoice).where(Invoice.number == number))
    if invoice is None:
        raise HTTPException(404, "invoice not found")
    return {
        "number": invoice.number,
        "client": invoice.client_name,
        "total": str(invoice.total),
        "status": invoice.status,
        "balance_due": str(balance_due(invoice)),
        "payments": [
            {"amount": str(p.amount), "fee": str(p.fee), "method": p.method, "reference": p.reference}
            for p in invoice.payments
        ],
    }
