# Spec — arrivals-board

Status: in-flight (opened 2026-08-25) · capability: FEAT-004 · owner: the office

## Stories

- US-001 As the office, I see today's expected arrivals (from bookings and passes) on the tablet.
- US-002 As the office, I log an arrival against its booking with one tap.
- US-003 As the office, I log a departure with one tap and the stay closes.
- US-004 As the office, the board is right when I open the office at 08:00 without me touching it.

## Functional requirements

- FR-001 The board lists today's expected arrivals with berth and ETA where known.
- FR-002 An arrival or departure tap records the time and the user.
- FR-003 The board refreshes itself every 60 seconds and rolls to the new day at midnight harbour
  local time; yesterday's departures never show on today's board.
- FR-004 A departure closes the stay and starts the departure invoice (FEAT-003).

## Success criteria

- SC-001 At 08:00 the board shows only today's movements with no refresh.
- SC-002 A logged departure produces an issued invoice within one minute.
