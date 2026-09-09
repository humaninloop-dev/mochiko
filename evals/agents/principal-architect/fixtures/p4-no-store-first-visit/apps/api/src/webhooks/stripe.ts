import type { Request, Response } from "express";
import Stripe from "stripe";

import { db } from "@tapestry/db";
import { queues } from "../queues";

const stripe = new Stripe(process.env.STRIPE_SECRET!);

// Verifies the signature, stores the event once, and hands processing to the worker.
// Moved off the request path in 2026-05 after invoice.paid handling timed out and Stripe
// retried into duplicates.
export async function stripeWebhook(req: Request, res: Response) {
  let event: Stripe.Event;
  try {
    event = stripe.webhooks.constructEvent(req.body, req.headers["stripe-signature"] as string,
      process.env.STRIPE_WEBHOOK_SECRET!);
  } catch {
    return res.sendStatus(400);
  }
  const stored = await db.stripeEvent.createMany({ data: [{ id: event.id, type: event.type, payload: event as object }], skipDuplicates: true });
  if (stored.count === 1) {
    await queues.stripeEvents.add("process", { eventId: event.id }, { jobId: event.id });
  }
  res.sendStatus(200);
}
