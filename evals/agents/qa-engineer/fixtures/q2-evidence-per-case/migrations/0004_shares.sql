create table if not exists shares (
  id serial primary key,
  token varchar(32) unique not null,
  notebook_id integer not null references notebooks(id),
  expires_at timestamp not null,
  preview_path varchar(300),
  created_at timestamp not null default now()
);
