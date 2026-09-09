import express from "express";

import { staffSession } from "./auth/session";
import { bookingsRouter } from "./routes/bookings";
import { membersRouter } from "./routes/members";
import { stripeWebhook } from "./webhooks/stripe";

const app = express();

// Stripe needs the raw body for signature verification, and no session — mounted first.
app.post("/webhooks/stripe", express.raw({ type: "application/json" }), stripeWebhook);

app.use(express.json());
app.use(staffSession); // every route below this line requires a staff session (owner | front-desk)
app.use("/members", membersRouter);
app.use("/bookings", bookingsRouter);

app.listen(3000);
