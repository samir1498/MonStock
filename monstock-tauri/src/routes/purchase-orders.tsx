import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/purchase-orders")({
  component: () => (
    <div className="animate-in">
      <h1 className="text-xl font-semibold tracking-tight mb-6" style={{ color: "var(--color-text)" }}>
        Purchase Orders
      </h1>
      <p className="text-text-sec">Coming soon — PO management</p>
    </div>
  ),
});