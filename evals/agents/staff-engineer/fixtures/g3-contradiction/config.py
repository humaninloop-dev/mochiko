"""Read the service's INI-style config file into a flat dict of coerced values."""
import configparser
from pathlib import Path

SECTION = "app"


def _coerce(value):
    """Turn an INI string into bool, int, or str — tried in that order."""
    lowered = value.strip().lower()
    if lowered in ("true", "yes", "on"):
        return True
    if lowered in ("false", "no", "off"):
        return False
    try:
        return int(lowered)
    except ValueError:
        return value.strip()


def load_config(path):
    """Return the `[app]` section of the INI file at `path` as a dict of coerced values.

    Raises FileNotFoundError when the file cannot be read.
    """
    parser = configparser.ConfigParser()
    if not parser.read(Path(path)):
        raise FileNotFoundError(path)
    # TODO: reject unknown keys once the schema settles.
    return {key: _coerce(value) for key, value in parser[SECTION].items()}
