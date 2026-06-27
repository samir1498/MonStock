import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/expenses")({
  component: () => (
    <div className="animate-in">
      <h1 className="text-xl font-semibold tracking-tight mb-6" style={{ color: "var(--color-text)" }}>
        Expenses
      </h1>
      <p className="text-text-sec">Coming soon — Expense tracking</p>
    </div>
  ),
});