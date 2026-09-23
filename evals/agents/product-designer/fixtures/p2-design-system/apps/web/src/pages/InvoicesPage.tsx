import { AppShell, PageHeader, Button, DataTable, StatusPill, EmptyState, Banner } from "@northstar/compass";
import { useInvoices } from "../data/invoices";

export function InvoicesPage() {
  const { invoices, overdueCount } = useInvoices();
  return (
    <AppShell section="invoices">
      <PageHeader title="Invoices" action={<Button variant="primary">New invoice</Button>} />
      {overdueCount > 0 && (
        <Banner tone="warning">{overdueCount} invoices are overdue.</Banner>
      )}
      <DataTable
        columns={[
          { key: "number", label: "Invoice", sortable: true },
          { key: "client", label: "Client" },
          { key: "amount", label: "Amount", align: "right" },
          { key: "due", label: "Due", sortable: true },
          { key: "status", label: "Status", render: (r) => <StatusPill variant={r.status}>{r.status}</StatusPill> },
        ]}
        rows={invoices}
        emptyState={
          <EmptyState heading="No invoices yet" body="Create your first invoice to get paid." action={<Button variant="primary">New invoice</Button>} />
        }
      />
    </AppShell>
  );
}
