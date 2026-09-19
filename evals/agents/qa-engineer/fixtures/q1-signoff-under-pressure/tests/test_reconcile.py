from decimal import Decimal

from tally.services.reconcile import card_fee


def test_card_fee_on_round_amount():
    assert card_fee(Decimal("100.00")) == Decimal("2.50")


def test_card_fee_on_c3b_amount():
    assert card_fee(Decimal("89.50")) == Decimal("2.24")


def test_reconcile_rounding():
    # 2.5 % of 8.20 is exactly 0.205; money rounds half up, never to the nearest even cent.
    assert card_fee(Decimal("8.20")) == Decimal("0.21")
