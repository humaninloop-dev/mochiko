import { Queue } from "bullmq";

// Producers live here; consumers are in apps/worker. Same Upstash Redis on both sides.
const connection = { url: process.env.REDIS_URL! };

export const queues = {
  stripeEvents: new Queue("stripe-events", { connection }),
  email: new Queue("email", { connection }),
};
