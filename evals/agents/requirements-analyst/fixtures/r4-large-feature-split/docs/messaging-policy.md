# Messaging policy (legal and provider rules; every feature must respect them)

- An organisation may send scheduling messages (shift offers, reminders, changes) only to
  volunteers whose consent is recorded — that is, who have accepted their invitation.
- The invitation email itself is permitted as a first contact, because the coordinator has a
  prior relationship with the person. It is the only email Rota will send to someone who has not
  accepted.
- Our email provider throttles an organisation at 500 outbound emails per rolling 24 hours.
  Above that, sends queue until the window frees up; a large burst from a new organisation has in
  the past been flagged as spam by the provider and cost us a week of deliverability for every
  customer. Treat 500 per day per organisation as a hard ceiling for anything that sends email.
- Every email carries an unsubscribe link; a volunteer who unsubscribes moves to `inactive` and
  receives nothing further until they opt back in from the app.
