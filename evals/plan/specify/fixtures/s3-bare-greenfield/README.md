# Nook

Room booking for a single coworking space. A member picks a room and a time slot, the
booking costs credits from their monthly allowance, and the host at the front desk can see
who is in which room right now.

## Status

Nothing is built yet. This repository was created this week and holds these notes, the
interview write-ups in `docs/`, and nothing else — no source, no dependencies, no
configuration.

## Who it is for

One coworking space with about a hundred and twenty members, six bookable rooms (four
meeting rooms, two phone booths), and one host on the desk during opening hours.

## The words we use

- **Member** — someone with a plan; every plan comes with monthly **credits**.
- **Room** — a bookable space with a capacity and a credit cost per half hour.
- **Booking** — one member, one room, one time slot; it spends credits when it is made
  and refunds them if cancelled early enough.
- **Host** — the person on the desk; sees today's bookings and can free a room that was
  never claimed.

## Intended shape

A web app members open on their phones on the way in. Two engineers, part time. Pilot
with the space's own members; if it holds, other spaces may follow.
