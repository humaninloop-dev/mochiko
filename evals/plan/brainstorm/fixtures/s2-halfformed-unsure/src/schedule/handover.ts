import type { Db } from "../db/client";

/**
 * Hand one course day from its current instructor to another. Added 2026-04-28 for the
 * office's course page ("Reassign day"). The session row is what the course page reads,
 * so the page shows the new instructor immediately.
 */
export async function handOverDay(db: Db, sessionId: string, toInstructorId: string, actorId: string): Promise<void> {
  await db.tx(async (t) => {
    const before = await t.one<{ instructor_id: string | null }>(
      `select instructor_id from sessions where id = $1 for update`,
      [sessionId],
    );
    await t.query(`update sessions set instructor_id = $2 where id = $1`, [sessionId, toInstructorId]);
    await t.query(
      `insert into audit_log (entity, entity_id, action, actor_id, detail)
       values ('session', $1, 'handover', $2, $3)`,
      [sessionId, actorId, { from: before.instructor_id, to: toInstructorId }],
    );
  });
}
