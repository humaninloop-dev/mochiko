import type { NextFunction, Request, Response } from "express";

import { db } from "@tapestry/db";

export type StaffRole = "owner" | "front-desk";

// Cookie session for staff users. There is exactly one auth scheme in the app: a staff
// user signed in on a staff device. Every request carries a venueId from the session.
export async function staffSession(req: Request, res: Response, next: NextFunction) {
  const token = req.cookies?.session;
  if (!token) return res.sendStatus(401);
  const session = await db.staffSession.findUnique({ where: { token }, include: { user: true } });
  if (!session || session.expiresAt < new Date()) return res.sendStatus(401);
  req.staff = { id: session.user.id, role: session.user.role as StaffRole, venueId: session.user.venueId };
  next();
}

export function requireRole(role: StaffRole) {
  return (req: Request, res: Response, next: NextFunction) =>
    req.staff?.role === role || req.staff?.role === "owner" ? next() : res.sendStatus(403);
}
