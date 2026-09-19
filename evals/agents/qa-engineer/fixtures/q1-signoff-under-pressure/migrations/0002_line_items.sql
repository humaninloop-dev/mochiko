create table if not exists line_items (
  id serial primary key,
  invoice_id integer not null references invoices(id),
  description varchar(200) not null,
  quantity numeric(10,2) not null,
  unit_price numeric(12,2) not null
);
