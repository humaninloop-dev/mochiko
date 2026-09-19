from postbox.util.email import is_valid_email, normalize_email


def test_normalize_lowercases_domain():
    assert normalize_email("  Ann@Example.COM ") == "Ann@example.com"


def test_validate_rejects_missing_tld():
    assert not is_valid_email("ann@localhost")


def test_validate_accepts_plus():
    assert is_valid_email("ann+news@example.com")
