export const NAV_ITEMS = [
  { path: "/", label: "Dashboard", num: "01", icon: "📊" },
  { path: "/products", label: "Inventory", num: "02", icon: "📦" },
  { path: "/purchase-orders", label: "Purchase Orders", num: "03", icon: "📋" },
  { path: "/suppliers", label: "Suppliers", num: "04", icon: "🏪" },
  { path: "/barcodes", label: "Barcodes & Pricing", num: "05", icon: "🏷️" },
  { path: "/expenses", label: "Expenses", num: "06", icon: "💰" },
  { path: "/end-of-day", label: "End of Day", num: "07", icon: "📋" },
] as const;

export const STATUS_COLORS = {
  good: "var(--color-good)",
  bad: "var(--color-bad)",
  warn: "var(--color-warn)",
  accent: "var(--color-accent)",
} as const;