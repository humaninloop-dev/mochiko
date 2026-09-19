// Mirror of crewboard/domain/swaps.py::can_swap — keeps the "Propose swap" button disabled
// before the round trip. KEEP IN SYNC BY HAND (drifted once: INC-27 follow-up, 2026-06).
export function canSwap(give: Shift, take: Shift, me: Staff, hoursThisWeek: number): [boolean, string] {
  if (give.venueId !== take.venueId) return [false, "different venue"];
  if (give.week !== take.week) return [false, "different rota week"];
  if (!me.roles.includes(take.role)) return [false, `you do not hold role ${take.role}`];
  if (hoursThisWeek - give.hours + take.hours > me.contractedHours) return [false, "over contracted hours"];
  return [true, ""];
}
