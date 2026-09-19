# From the harbour master, Port Ellery — 2026-09-14

Three things from this week, in the order they hurt.

1. **Electricity charged twice across a month end.** When a boat's stay runs over the end of
   the month, the electricity for the last day before the month end shows up on both the
   month-end invoice and the departure invoice. *Marisol* (berth C14) was charged 14 kWh on
   INV-2026-0831 and the same 14 kWh again on INV-2026-0903. Two other skippers have written
   since. The readings themselves look right in the stay view; it is the invoices that
   disagree.

2. **Skippers want to see the electricity per day.** Half the questions at the office window
   are "why is my electricity that much". Show the metered electricity for each day of the
   stay on the invoice — kWh and money per day — and the same on the skipper's copy. The
   accountant has already agreed the export can carry the per-day lines under the electricity
   line, so the invoice's own line shape changes too (`.mochiko/product/contracts/api.yaml`
   is what she signs off against).

3. **The arrivals board still shows yesterday's departures.** At 08:00 the board on the office
   tablet lists the boats that left yesterday until someone refreshes it by hand around 09:00.
   The engineers say the board work is still in progress; fine, but the office wants it on the
   list.

Can the first two go into the next release? The third I leave with you.
