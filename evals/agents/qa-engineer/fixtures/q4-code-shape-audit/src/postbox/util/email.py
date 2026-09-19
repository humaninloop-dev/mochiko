"""Email helpers for the importer (T2.2)."""

import re

_EMAIL_RE = re.compile(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$")


def normalize_email(raw: str) -> str:
    value = raw.strip()
    if "@" not in value:
        return value
    local, _, domain = value.rpartition("@")
    return f"{local}@{domain.lower()}"


def is_valid_email(value: str) -> bool:
    return bool(_EMAIL_RE.match(value))
