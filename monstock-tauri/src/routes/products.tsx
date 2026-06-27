import { createFileRoute } from "@tanstack/react-router";
import { useQuery } from "@tanstack/react-query";
import { listProducts } from "../lib/api";
import { fmtDA, marginPct } from "../lib/utils";

export const Route = createFileRoute("/products")({
  component: ProductsPage,
});

function ProductsPage() {
  const { data, isLoading } = useQuery({
    queryKey: ["products"],
    queryFn: () => listProducts(1, 50),
  });

  return (
    <div className="animate-in">
      <div className="flex items-end justify-between mb-6 pb-4" style={{ borderBottom: "1px solid var(--color-border)" }}>
        <div>
          <h1 className="text-xl font-semibold tracking-tight" style={{ color: "var(--color-text)" }}>
            Inventory
          </h1>
          <p className="text-xs mt-1" style={{ color: "var(--color-text-dim)" }}>
            {data?.total ?? 0} products registered
          </p>
        </div>
      </div>

      {isLoading ? (
        <div className="animate-pulse text-text-dim">Loading...</div>
      ) : (
        <div className="rounded-lg overflow-hidden" style={{ background: "var(--color-surface)", border: "1px solid var(--color-border)" }}>
          <div className="overflow-x-auto">
            <table className="w-full text-[12.5px]">
              <thead>
                <tr>
                  <Th>Product</Th>
                  <Th>Barcode</Th>
                  <Th>Cost</Th>
                  <Th>Price</Th>
                  <Th>Stock</Th>
                  <Th>Margin</Th>
                  <Th>Status</Th>
                </tr>
              </thead>
              <tbody>
                {data?.items.map((p) => {
                  const margin = marginPct(p.cost_price, p.selling_price);
                  const status = p.quantity_on_hand <= 0 ? "Out" : p.quantity_on_hand <= 10 ? "Low" : "In Stock";
                  const statusColor = status === "In Stock" ? "var(--color-good)" : status === "Low" ? "var(--color-warn)" : "var(--color-bad)";
                  return (
                    <tr key={p.id} className="hover:bg-raised transition-colors">
                      <td className="px-4 py-3">
                        <div className="flex items-center gap-2.5">
                          <div className="w-7 h-7 rounded-sm flex items-center justify-center text-[11px] flex-shrink-0"
                            style={{ background: "var(--color-raised)", border: "1px solid var(--color-border)" }}>
                            {p.name.slice(0, 2).toUpperCase()}
                          </div>
                          <div>
                            <div className="font-medium text-[12.5px] truncate max-w-[200px]" style={{ color: "var(--color-text)" }}>
                              {p.name}
                            </div>
                            <div className="text-[11px] font-mono" style={{ color: "var(--color-text-dim)" }}>
                              PRD-{String(p.id).padStart(3, "0")}
                            </div>
                          </div>
                        </div>
                      </td>
                      <td className="px-4 py-3 font-mono text-[11.5px]" style={{ color: "var(--color-text-sec)" }}>
                        {p.barcode ?? "—"}
                      </td>
                      <TdNum>{p.cost_price.toFixed(2)}</TdNum>
                      <TdNum>{p.selling_price.toFixed(2)}</TdNum>
                      <td className="px-4 py-3">
                        <div className="flex items-center gap-2">
                          <div className="w-10 h-[3px] rounded-full overflow-hidden" style={{ background: "var(--color-raised)" }}>
                            <div
                              className="h-full rounded-full transition-all"
                              style={{
                                width: `${Math.min(p.quantity_on_hand / 50 * 100, 100)}%`,
                                background: statusColor,
                              }}
                            />
                          </div>
                          <span className="font-mono text-[12px]" style={{ color: "var(--color-text)" }}>
                            {p.quantity_on_hand}
                          </span>
                        </div>
                      </td>
                      <TdNum color={margin > 50 ? "var(--color-good)" : margin > 30 ? "var(--color-warn)" : "var(--color-bad)"}>
                        {margin.toFixed(1)}%
                      </TdNum>
                      <td className="px-4 py-3">
                        <Tag color={statusColor}>{status}</Tag>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
}

function Th({ children }: { children: React.ReactNode }) {
  return (
    <th className="px-4 py-2.5 text-[10.5px] uppercase tracking-wider font-medium whitespace-nowrap text-left"
      style={{ color: "var(--color-text-dim)", borderBottom: "1px solid var(--color-border)" }}>
      {children}
    </th>
  );
}

function TdNum({ children, color }: { children: React.ReactNode; color?: string }) {
  return (
    <td className="px-4 py-3 font-mono text-[12px] text-right" style={{ color: color || "var(--color-text)", borderBottom: "1px solid var(--color-border)" }}>
      {children}
    </td>
  );
}

function Tag({ children, color }: { children: React.ReactNode; color: string }) {
  return (
    <span
      className="inline-flex px-2 py-[2px] rounded-[4px] text-[10.5px] font-medium"
      style={{
        border: `1px solid ${color}33`,
        background: `${color}0f`,
        color,
      }}
    >
      {children}
    </span>
  );
}