create table if not exists staff (
  id serial primary key,
  name varchar(80) not null,
  role varchar(16) not null,
  phone varchar(20) not null,
  is_manager boolean not null default false
);
create table if not exists shifts (
  id serial primary key,
  rota_id integer not null,
  staff_id integer not null references staff(id),
  role varchar(16) not null,
  starts_at timestamp not null,
  ends_at timestamp not null,
  published boolean not null default false
);
