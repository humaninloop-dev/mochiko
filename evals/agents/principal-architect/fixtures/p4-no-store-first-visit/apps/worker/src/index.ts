import { Worker } from "bullmq";

import { connection } from "./redis";
import { processStripeEvent } from "./jobs/stripe-events";
import { sendEmail } from "./jobs/email";

new Worker("stripe-events", processStripeEvent, { connection, concurrency: 4 });
new Worker("email", sendEmail, { connection, concurrency: 8 });
