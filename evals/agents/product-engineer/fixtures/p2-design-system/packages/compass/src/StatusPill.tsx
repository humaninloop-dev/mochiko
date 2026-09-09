import * as React from "react";

export type StatusPillVariant = "draft" | "sent" | "paid" | "overdue" | "void";

const tone: Record<StatusPillVariant, string> = {
  draft: "var(--cp-ink-muted)",
  sent: "var(--cp-info)",
  paid: "var(--cp-success)",
  overdue: "var(--cp-warning)",
  void: "var(--cp-border)",
};

export function StatusPill({ variant, children }: { variant: StatusPillVariant; children: React.ReactNode }) {
  return (
    <span
      className="cp-status-pill"
      style={{ borderColor: tone[variant], color: tone[variant], borderRadius: "var(--cp-radius-pill)" }}
    >
      {children}
    </span>
  );
}
