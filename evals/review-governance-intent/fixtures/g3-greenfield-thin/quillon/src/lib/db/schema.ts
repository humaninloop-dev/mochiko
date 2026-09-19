// src/lib/db/schema.ts — stub
import { pgTable, serial, text, timestamp, integer } from "drizzle-orm/pg-core";

export const users = pgTable("users", {
  id: serial("id").primaryKey(),
  email: text("email").notNull().unique(),
  passwordHash: text("password_hash").notNull(),
  createdAt: timestamp("created_at").defaultNow(),
});

export const clients = pgTable("clients", {
  id: serial("id").primaryKey(),
  userId: integer("user_id").notNull(),
  company: text("company"),
  contactName: text("contact_name").notNull(),
  contactEmail: text("contact_email").notNull(),
  billingAddress: text("billing_address"),
  vatId: text("vat_id"),
});

export const invoices = pgTable("invoices", {
  id: serial("id").primaryKey(),
  clientId: integer("client_id").notNull(),
  number: text("number").notNull(),
  issuedAt: timestamp("issued_at"),
  status: text("status").notNull(), // draft | issued | paid | cancelled
});
