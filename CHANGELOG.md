# Changelog

## 0.1.0 — unreleased

### Features
- Purchase Orders (bons d'achat) with Draft/Received status
- Stock tracking — auto-increment on PO receive, decrement on sale
- Product management with barcodes, cost/selling price, margin
- Expenses with categories and date range filtering
- Daily dashboard with stats cards, sales chart, activity feed
- Dashboard low-stock alert with paginated card grid
- Sales recording with transaction history
- Pagination across all list screens
- i18n (English / French) with global toggle
- Dark/light theme toggle
- Backup auto-creation + manual backup with backup history
- Barcode scanner support (USB keyboard-emulation scanners)
- Bar chart (sales by hour) via custom egui drawing

### Refactors
- Architecture audit: service layer depth, DRY, N+1 queries
- UI component extraction: data_table, modal_window, modal_actions, delete_btn
- Consistent table styling across all screens
- Centered stat cards with fixed heights (no layout shift)
- Settings screen with backup actions (no command-line hints)

### Bugfixes
- update_product now correctly persists cost_price
- 11 Clippy warnings fixed (redundant imports, needless lifetimes, etc.)
- Navigation ID collision fix

### Tech
- Rust + egui 0.34 + Diesel + SQLite
- Workspace split: monstock-core (lib) / monstock-desktop (binary)
- 27 integration tests
