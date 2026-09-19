# Plans and billing (as built, September 2026)

## Plan types

| Plan | Billing | Notes |
|---|---|---|
| Monthly rolling | Card on file charged on the member's billing day each month | The default plan at most studios |
| Annual prepaid | Twelve months charged up front at a 15% discount | About 20% of members; popular at yoga studios |
| Family | One payer's card; up to four linked members | Linked members have their own portal logins and check in on their own; only the payer sees billing |

## Billing mechanics

- The billing day is the calendar day the member joined; a member who joined on the 31st is
  charged on the last day of shorter months.
- A failed card charge is retried three times over seven days; after the third failure the
  membership goes to `lapsed` and check-in is refused until a payment succeeds.
- Members can update their card in the portal. Staff can take a payment at the desk.
- Freeze (pause): staff can freeze a membership for one to three months from the staff app;
  billing stops for the freeze and resumes after. Members cannot freeze themselves today.

## Regional rules

- Studios in the United Kingdom use a membership contract with a 30-day notice period: a member
  gives notice, and the membership ends at the end of the next billing period after the 30 days
  have run. Studios configure this per studio; 61 of our 74 UK studios have it on.
- Studios elsewhere (Ireland, Netherlands, Australia) have no notice period configured; the
  contract says membership ends when the member cancels, and the studio decides what "ends"
  means. In practice most let the member keep coming until the paid period runs out.
- Annual prepaid refunds: there is no policy. When an annual member asks for a refund of unused
  months, the studio owner decides and issues it by hand through the payment provider.

## Cancellation today

- Only staff can cancel a membership, from the staff app. The member asks at the desk, by phone,
  or by email; staff find the member and press Cancel; the app asks for an end date and whether to
  refund anything, with no defaults.
- There is no confirmation to the member when staff cancel. Whether billing actually stops depends
  on staff picking an end date on or before the next billing day.
