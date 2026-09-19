import type { Job } from "bullmq";

import { db } from "@tapestry/db";
import { queues } from "../queues";

// Membership status is derived from Stripe invoice events; the rule for "active" itself
// lives in packages/db (isActive).
export async function processStripeEvent(job: Job<{ eventId: string }>) {
  const ev = await db.stripeEvent.findUniqueOrThrow({ where: { id: job.data.eventId } });
  const payload = ev.payload as { data: { object: { customer: string; period_end?: number } } };
  const membership = await db.membership.findFirst({ where: { stripeCustomerId: payload.data.object.customer } });
  if (!membership) return;
  if (ev.type === "invoice.paid") {
    await db.membership.update({ where: { id: membership.id }, data: { paidThrough: new Date(payload.data.object.period_end! * 1000), status: "active" } });
  } else if (ev.type === "invoice.payment_failed") {
    await db.membership.update({ where: { id: membership.id }, data: { status: "past_due" } });
    await queues.email.add("payment-failed", { membershipId: membership.id });
  }
}
