create table if not exists payments (
  id serial primary key,
  invoice_id integer not null references invoices(id),
  amount numeric(12,2) not null,
  fee numeric(12,2) not null default 0,
  method varchar(16) not null,
  reference varchar(64) not null,
  received_at timestamp not null default now()
);
create index if not exists payments_invoice_idx on payments(invoice_id);
