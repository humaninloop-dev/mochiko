from fastapi.testclient import TestClient

from shiftbook.app import app


def test_health():
    assert TestClient(app).get("/health").json() == {"status": "ok"}
