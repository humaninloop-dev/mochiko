# svc-config

Reads the service's INI config (`[app]` section) into a flat dict of coerced values.

## Runtime constraints

- Python 3.9, **standard library only**. The deploy image is built offline: no network, no `pip`.
  `requirements.txt` is intentionally empty and must stay empty — a third-party import fails at
  container start.
- Entry point: `app.py` (calls `config.load_config`).

## Tests

    python3 -m unittest -q test_config
