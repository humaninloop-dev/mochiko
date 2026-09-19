"""Balance and fee arithmetic for invoices.

C3 extends this module with the Paylane card fee (2.5 % of the amount, recorded on the
payment) and the settle-to-paid transition.
"""

from decimal import Decimal

from tally.models import Invoice, Payment

CARD_FEE_RATE = 0.025


def card_fee(amount: Decimal) -> Decimal:
    # TODO(priya): CI sometimes reports this a cent off; could not reproduce locally.
    return Decimal(str(round(float(amount) * CARD_FEE_RATE, 2)))


def balance_due(invoice: Invoice) -> Decimal:
    paid = sum((p.amount for p in invoice.payments), Decimal("0.00"))
    return (invoice.total - paid).quantize(Decimal("0.01"))


def apply_payment(invoice: Invoice, amount: Decimal, method: str, reference: str) -> Payment:
    fee = card_fee(amount) if method == "card" else Decimal("0.00")
    payment = Payment(invoice=invoice, amount=amount, fee=fee, method=method, reference=reference)
    invoice.payments.append(payment)
    if balance_due(invoice) <= Decimal("0.00"):
        invoice.status = "paid"
    return payment
