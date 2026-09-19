# Changelog

All production promotions, newest first. The top entry's version matches the deployed task
definition's tag.

## 0.14.2 — 2026-09-05

- Checkout: retry the Adyen tokeniser call once on a network error before failing the order.
- Makers: payout statement now shows the settled amount beside the order total.

## 0.14.1 — 2026-08-29

- Fixed the VAT rate lookup for Northern Ireland addresses.
