create table if not exists subscribers (
  id serial primary key,
  email varchar(254) unique not null,
  name varchar(120),
  tags varchar(300) not null default '',
  source varchar(16) not null default 'signup',
  created_at timestamp not null default now()
);
