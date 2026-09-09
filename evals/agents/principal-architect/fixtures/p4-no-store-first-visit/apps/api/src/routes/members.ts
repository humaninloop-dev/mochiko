import { Router } from "express";

import { db, isActive } from "@tapestry/db";
import { requireRole } from "../auth/session";

export const membersRouter = Router();

// Front desk looks a member up by name and sees whether they can come in today.
membersRouter.get("/", async (req, res) => {
  const q = String(req.query.q ?? "");
  const members = await db.member.findMany({
    where: { venueId: req.staff.venueId, name: { contains: q, mode: "insensitive" } },
    include: { membership: true },
    take: 20,
  });
  res.json(members.map((m) => ({ id: m.id, name: m.name, active: isActive(m.membership, new Date()) })));
});

membersRouter.post("/", requireRole("front-desk"), async (req, res) => {
  const member = await db.member.create({ data: { ...req.body, venueId: req.staff.venueId } });
  res.status(201).json(member);
});
