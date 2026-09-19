import type { Membership } from "@prisma/client";

// The rule for "may this member come in". Used by the staff app's member lookup (api)
// and by the worker when it applies invoice events.
export function isActive(m: Membership | null, now: Date): boolean {
  if (!m) return false;
  if (m.status === "cancelled") return false;
  const grace = new Date(m.paidThrough.getTime() + m.graceDays * 86_400_000);
  return m.status === "active" || (m.status === "past_due" && now <= grace);
}
