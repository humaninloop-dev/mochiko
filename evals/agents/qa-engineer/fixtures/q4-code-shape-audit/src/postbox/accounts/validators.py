"""Address handling shared by sign-up (C1). Normalise first, then validate."""

from email_validator import EmailNotValidError, validate_email


def normalize_email(raw: str) -> str:
    """Lower-case the domain, strip whitespace, keep the local part as typed."""
    value = raw.strip()
    if "@" not in value:
        return value
    local, _, domain = value.rpartition("@")
    return f"{local}@{domain.lower()}"


def is_valid_email(value: str) -> bool:
    try:
        validate_email(value, check_deliverability=False)
    except EmailNotValidError:
        return False
    return True
