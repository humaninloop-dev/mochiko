"""Test doubles for third parties."""

import hashlib
import hmac
import json


class FakePaylane:
    """Emits a signed `payment.succeeded` delivery the way the sandbox would."""

    def __init__(self, secret: str = "test-secret"):
        self.secret = secret
        self.sent: list[dict] = []

    def payment_succeeded(self, invoice: str, amount_pence: int, payment_id: str = "pay_fake_1"):
        body = json.dumps(
            {"type": "payment.succeeded",
             "data": {"id": payment_id, "amount": amount_pence, "currency": "GBP",
                      "metadata": {"invoice": invoice}}}
        ).encode()
        sig = hmac.new(self.secret.encode(), body, hashlib.sha256).hexdigest()
        self.sent.append({"body": body, "signature": sig})
        return body, sig
