# 0004 — Maximum pause length

Decided 2026-06-10 · founder + head of operations · status: in force

## Decision

- A subscription may be paused for at most eight weeks at a time.
- At six weeks paused, the customer receives an email saying the subscription will cancel
  itself at eight weeks unless resumed.
- At eight weeks paused with no resume, the subscription is cancelled and the delivery slot
  released; the customer can re-subscribe but is not guaranteed the same slot.

## Why

Delivery slots on a van route are the scarce thing. A subscription paused indefinitely holds a
slot we cannot sell.
