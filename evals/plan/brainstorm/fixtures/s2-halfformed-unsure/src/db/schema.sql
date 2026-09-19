-- Current schema (plain dump kept for reading; Prisma migrations are the source of truth).
-- Dumped 2026-05-10.

create table schools (
  id            uuid primary key,
  name          text not null,
  created_at    timestamptz not null default now()
);

create table instructors (
  id            uuid primary key,
  school_id     uuid not null references schools(id),
  name          text not null,
  email         text not null
);

create table boats (
  id            uuid primary key,
  school_id     uuid not null references schools(id),
  name          text not null,
  berths        integer not null            -- capacity; a course borrows this number
);

create table courses (
  id            uuid primary key,
  school_id     uuid not null references schools(id),
  title         text not null,
  starts_on     date not null,
  ends_on       date not null,
  boat_id       uuid not null references boats(id),
  capacity      integer not null            -- may be lowered below boats.berths, never raised
);

-- One row per course day. The instructor for a day lives here; a handover updates it.
create table sessions (
  id            uuid primary key,
  course_id     uuid not null references courses(id),
  day           date not null,
  instructor_id uuid references instructors(id),
  unique (course_id, day)
);

create table bookings (
  id            uuid primary key,
  course_id     uuid not null references courses(id),
  student_id    uuid not null,
  status        text not null,              -- booked | cancelled | waitlisted
  -- Copied from the course's first session at booking time (added 2026-03-14 so the
  -- instructor week view could be one query). Not updated afterwards.
  instructor_id uuid references instructors(id),
  created_at    timestamptz not null default now()
);

create table audit_log (
  id            bigserial primary key,
  entity        text not null,
  entity_id     uuid not null,
  action        text not null,
  actor_id      uuid not null,
  at            timestamptz not null default now(),
  detail        jsonb
);
