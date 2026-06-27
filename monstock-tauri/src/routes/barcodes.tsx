import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/barcodes")({
  component: () => (
    <div className="animate-in">
      <h1 className="text-xl font-semibold tracking-tight mb-6" style={{ color: "var(--color-text)" }}>
        Barcodes & Pricing
      </h1>
      <p className="text-text-sec">Coming soon — Barcode scanning & price editing</p>
    </div>
  ),
});