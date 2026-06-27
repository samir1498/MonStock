import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { getDashboardData, getLowStockProducts } from "../lib/api";
import { fmtDA, today } from "../lib/utils";
import { fmtDate } from "../lib/utils";

export const Route = createFileRoute("/")({
  component: DashboardPage,
});

function DashboardPage() {
  const date = today();

  const { data: dash, isLoading: dashLoading } = useQuery({
    queryKey: ["dashboard", date],
    queryFn: () => getDashboardData(date),
  });

  const { data: lowStock } = useQuery({
    queryKey: ["low-stock"],
    queryFn: () => getLowStockProducts(),
  });

  if (dashLoading) {
    return (
      <div className="animate-pulse flex items-center gap-3 mt-8 justify-center" style={{ color: "var(--color-text-sec)" }}>
        <div className="w-4 h-4 rounded-full border-2 border-border-strong border-t-text-sec animate-spin" />
        Loading...
      </div>
    );
  }

  return (
    <div className="animate-in">
      {/* Page header */}
      <div className="flex items-end justify-between mb-6 pb-4" style={{ borderBottom: "1px solid var(--color-border)" }}>
        <div>
          <h1 className="text-xl font-semibold tracking-tight" style={{ color: "var(--color-text)" }}>
            Dashboard
          </h1>
          <p className="text-xs mt-1" style={{ color: "var(--color-text-dim)" }}>
            {new Date().toLocaleDateString("fr-DZ", { weekday: "long", year: "numeric", month: "long", day: "numeric" })}
          </p>
        </div>
      </div>

      {/* Low stock alert */}
      {lowStock && lowStock.length > 0 && (
        <div
          className="flex items-center gap-2.5 px-4 py-3 rounded-lg mb-5 cursor-pointer transition-all hover:border-strong"
          style={{
            background: "var(--color-raised)",
            border: "1px solid var(--color-border)",
          }}
        >
          <div
            className="w-6 h-6 rounded-sm flex items-center justify-center text-[10px] font-mono"
            style={{
              background: "var(--color-surface)",
              border: "1px solid var(--color-border)",
              color: "var(--color-warn)",
            }}
          >
            !
          </div>
          <span className="text-[12.5px] font-medium" style={{ color: "var(--color-text)" }}>
            {lowStock.length} {lowStock.length === 1 ? "product" : "products"} running low on stock
          </span>
          <span className="ml-auto text-[11.5px]" style={{ color: "var(--color-text-dim)" }}>
            View inventory →
          </span>
        </div>
      )}

      {/* Stats */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-3 mb-5">
        <StatCard label="Sales Today" value={dash ? fmtDA(dash.sales_total) : "—"} />
        <StatCard label="Expenses Today" value={dash ? fmtDA(dash.expenses_total) : "—"} />
        <StatCard
          label="Net Profit"
          value={dash ? fmtDA(dash.profit) : "—"}
          color={dash && dash.profit >= 0 ? "var(--color-good)" : "var(--color-bad)"}
        />
        <StatCard label="Transactions" value={dash ? String(dash.transaction_count) : "—"} />
      </div>
    </div>
  );
}

function StatCard({ label, value, color }: { label: string; value: string; color?: string }) {
  return (
    <div
      className="p-[18px] rounded-lg transition-all duration-200 hover:border-strong"
      style={{
        background: "var(--color-surface)",
        border: "1px solid var(--color-border)",
      }}
    >
      <div
        className="text-[11px] uppercase tracking-wider font-medium mb-2"
        style={{ color: "var(--color-text-dim)" }}
      >
        {label}
      </div>
      <div
        className="text-[22px] font-semibold tracking-tight"
        style={{ color: color || "var(--color-text)" }}
      >
        {value}
      </div>
    </div>
  );
}