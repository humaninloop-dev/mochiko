"""Developer CLI: demo seed and Paylane sandbox helpers."""

import os
from decimal import Decimal

import click
import httpx
from sqlalchemy import delete

from tally.db import SessionLocal
from tally.models import Invoice, Payment

PAYLANE_SANDBOX = "https://sandbox.paylane.example/v1"


@click.group()
def main():
    pass


@main.command("seed-demo")
def seed_demo():
    with SessionLocal() as session:
        session.execute(delete(Payment))
        session.execute(delete(Invoice))
        for n, (client, total) in enumerate(
            [("Northlight Studio", "1200.00"), ("Kestrel & Co", "480.00"), ("Alder Press", "95.00"),
             ("Northlight Studio", "300.00"), ("Fennel Films", "2150.00"), ("Alder Press", "60.00"),
             ("Kestrel & Co", "240.00"), ("Fennel Films", "89.50")],
            start=1001,
        ):
            session.add(Invoice(number=f"INV-{n}", client_name=client, total=Decimal(total)))
        session.commit()
    click.echo("seeded INV-1001 … INV-1008")


@main.group()
def paylane():
    pass


@paylane.command("simulate-payment")
@click.option("--invoice", required=True)
@click.option("--amount", required=True)
def simulate_payment(invoice: str, amount: str):
    """Ask the Paylane sandbox to emit a payment.succeeded event for an invoice."""
    key = os.environ["PAYLANE_SANDBOX_KEY"]
    r = httpx.post(
        f"{PAYLANE_SANDBOX}/test/payments",
        headers={"authorization": f"Bearer {key}"},
        json={"amount": int(Decimal(amount) * 100), "currency": "GBP", "metadata": {"invoice": invoice}},
        timeout=20,
    )
    r.raise_for_status()
    click.echo(f"sandbox payment {r.json()['id']} created for {invoice}")


if __name__ == "__main__":
    main()
