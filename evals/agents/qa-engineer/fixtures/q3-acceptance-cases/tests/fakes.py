"""Test doubles."""


class FakeTextline:
    """Records what would have been sent; never talks to Textline."""

    def __init__(self) -> None:
        self.sent: list[tuple[str, str]] = []

    def send(self, to: str, body: str) -> str:
        self.sent.append((to, body))
        return f"fake-{len(self.sent)}"

    def status(self, message_id: str) -> str:
        return "delivered"
