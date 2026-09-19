create table if not exists invoices (
  id serial primary key,
  number varchar(16) unique not null,
  client_name varchar(120) not null,
  total numeric(12,2) not null,
  status varchar(12) not null default 'open',
  created_at timestamp not null default now()
);
