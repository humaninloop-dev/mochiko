import type { Db } from "../db/client";

export type WeekDay = {
  day: string; // ISO date
  courseTitle: string;
  studentCount: number;
};

/**
 * The instructor's own week: one row per course day they teach, with the number of
 * booked students. Built 2026-03-14 as a single query off bookings.instructor_id so the
 * dashboard's week tab renders in one round trip.
 */
export async function instructorWeek(db: Db, instructorId: string, weekStart: string): Promise<WeekDay[]> {
  const rows = await db.query<{ day: string; title: string; n: number }>(
    `select s.day, c.title, count(b.id)::int as n
       from bookings b
       join courses c on c.id = b.course_id
       join sessions s on s.course_id = c.id
      where b.instructor_id = $1
        and b.status = 'booked'
        and s.day >= $2::date
        and s.day <  ($2::date + interval '7 days')
      group by s.day, c.title
      order by s.day`,
    [instructorId, weekStart],
  );
  return rows.map((r) => ({ day: r.day, courseTitle: r.title, studentCount: r.n }));
}
