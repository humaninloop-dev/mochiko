from decimal import Decimal

from tally.models import Invoice
from tally.services.reconcile import apply_payment, balance_due


def test_bank_transfer_settles_invoice(session):
    inv = Invoice(number="INV-9001", client_name="Test", total=Decimal("240.00"))
    session.add(inv)
    session.commit()
    apply_payment(inv, Decimal("240.00"), "bank_transfer", "BT-1")
    session.commit()
    assert inv.status == "paid"
    assert balance_due(inv) == Decimal("0.00")


def test_partial_payment_keeps_invoice_open(session):
    inv = Invoice(number="INV-9002", client_name="Test", total=Decimal("240.00"))
    session.add(inv)
    session.commit()
    apply_payment(inv, Decimal("100.00"), "bank_transfer", "BT-2")
    assert inv.status == "open"
    assert balance_due(inv) == Decimal("140.00")
