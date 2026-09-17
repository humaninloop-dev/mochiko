# baseline-delta — FEAT-004 arrivals-board (appliable before/after)

Written 2026-09-04 by the arrivals-board design phase; accepted at the run's landing
2026-09-05.

## contracts/api.yaml

### Before

(no path `/movements/today`)

### After

```yaml
  /movements/today:
    get:
      summary: Today's expected arrivals and logged departures for the office board
      responses:
        "200":
          description: movements
```
