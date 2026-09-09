create table if not exists notebooks (
  id serial primary key,
  slug varchar(64) unique not null,
  title varchar(200) not null
);
create table if not exists notes (
  id serial primary key,
  notebook_id integer not null references notebooks(id),
  position integer not null,
  title varchar(200) not null,
  body_md text not null
);
