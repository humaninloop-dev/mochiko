# Compass — component catalogue

Names are canonical: designs, mocks, and code all use these.

## Layout

- **AppShell** — `SideNav` on the left (240 px), `PageHeader` at the top of the content column,
  content constrained to `--cp-content-max`.
- **SideNav** — product sections: Dashboard · Invoices · Clients · Expenses · Reports · Settings.
  Adding a section is a guild decision.
- **PageHeader** — title, optional description, optional primary `Button` on the right.

## Actions

- **Button** — variants `primary` · `secondary` · `danger` · `ghost`; sizes `md` · `sm`;
  states default · hover · disabled · loading (spinner replaces label).
- **Menu** — overflow ("…") menu of actions on a row or a header.

## Data

- **DataTable** — column headers (sortable columns show a caret), zebra rows on
  `--cp-surface-sunken`, row hover, optional row `Menu`. Paginates at **25 rows**; below 25 no
  pager is rendered. Takes an `emptyState` slot rendered in place of the body when there are no
  rows. Long cell text truncates with an ellipsis and a title tooltip.
- **StatusPill** — small pill; variants are a closed set: `draft` (grey) · `sent` (info) ·
  `paid` (success) · `overdue` (warning) · `void` (muted). Adding a variant is a guild decision.
- **EmptyState** — icon, one-line heading, one-line body, optional primary `Button`. Used for
  first-run and zero-result views.
- **KeyValue** — label/value pairs in a two-column list (detail panels).

## Feedback

- **Banner** — page-level message: `info` · `warning` · `error`; optional action link;
  dismissible when `info`. Sits under the `PageHeader`, above content.
- **Toast** — transient confirmation, bottom-left, 4 s.
- **ConfirmModal** — title, body, `danger` or `primary` confirm `Button`, `ghost` cancel.

## Forms

- **Field** — label, input, optional help text, error text (danger). Inputs: text · number ·
  money (currency prefix from the account) · date (native picker).
- **Select** — single choice, native on mobile.
- **RadioGroup** — 2–5 mutually exclusive options with an optional description each.
- **Toggle** — on/off with a label.
- **Drawer** — right-hand panel (480 px) for create/edit forms that do not need a full page.

## Not in Compass (yet)

- A date-range or "repeat every…" picker — nothing beyond `Select` + `Field`.
- A timeline / history list.
- Inline row editing.
