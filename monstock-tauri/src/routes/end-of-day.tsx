import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/end-of-day")({
  component: () => (
    <div className="animate-in">
      <h1 className="text-xl font-semibold tracking-tight mb-6" style={{ color: "var(--color-text)" }}>
        End of Day
      </h1>
      <p className="text-text-sec">Coming soon — Daily closing summary</p>
    </div>
  ),
});