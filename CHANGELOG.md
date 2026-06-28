# Changelog

## 0.2.0 — Tauri v2 frontend (all screens, tests, error handling)

### Features
- All 7 Tauri screens implemented: Dashboard, Products, Purchase Orders, Suppliers, Barcodes, Expenses, End of Day
- CRUD modals for expenses, suppliers, purchase orders (with line items)
- Dashboard with stats cards, low-stock alerts, sales chart, recent transactions
- Products page with DataTable, StockBar, margin tags, status indicators
- Purchase order receive workflow — marks as Received, auto-updates stock
- Barcode screen for inline barcode editing
- End of Day screen with daily stats + transaction list
- ErrorBoundary component catching render errors with fallback UI
- Toast notification system (success/error, auto-dismiss, click-to-dismiss)
- All mutations now show success/error toasts and handle errors gracefully
- FR/EN translations for all screens and toast messages

### Tests
- Vitest + React Testing Library setup with 67 tests
- Component tests: Card, Button, Tag, Modal, Pagination, DataTable, FormField
- Error handling tests: ErrorBoundary render/catch/fallback, Toast show/dismiss/auto-dismiss
- Edge case tests: DataTable loading/null/large, StockBar 0/negative/clamp, Pagination partial pages

### Tech
- Tauri v2 + Vite 8 + React 19 + TanStack Router v1 + Tailwind v4 + TypeScript 6
- `tauri-plugin-updater` for auto-update support
- Separate versioning: Tauri at `0.2.0` (tagged `tauri-v*`), egui at `0.3.0` (tagged `desktop-v*`)
- CI runs Rust checks + frontend build + tests on all 3 OS
- Release workflows: `release-tauri.yml` (deb/AppImage/rpm/msi/dmg) and `release-desktop.yml` (binary)

## 0.3.0 — egui_extras::Table migration (full-width DataTable)

### Features
- Replaced `egui::Grid` DataTable with `egui_extras::Table` — columns now fill available width
- `ColumnDef` + `ColumnSizing` system: `Auto`, `Exact(f32)`, `Remainder` (flex)
- Remaining-space columns stretch to fill window on resize
- Fixed header row stays visible while body scrolls
- Column resizing ready (`.resizable(true)` on any `ColumnDef`)
- Sortable headers preserved (clickable ▲/▼) with migrated sort state
- `monstock-tauri` workspace member added (Tauri v2 scaffolding)

### Tech
- New `components/data_table.rs` module — `DataTable` builder struct
- Removed old `components::data_table()` function
- Removed dead `sortable_header()` from `style.rs`
- Added `serde`/`serde_json` workspace deps for Tauri IPC

## 0.2.1 — fix DataTable column alignment

### Bugfixes
- DataTable headers and data rows now share the same column layout (removed spacer between headers that threw off alignment)

## 0.2.0 — architecture cleanup

### Features
- Clickable sort headers (▲ ▼) on all 4 data screens
- Barcode scanner wired to sales screen (scan → auto-add product)
- Import products from backup files via Settings
- Filtering & sorting state per screen

### Refactors
- Service layer: created `supplier_service`, `expense_category_service` — zero repo bypasses from desktop
- Fixed 3 services using inline diesel instead of repos (`purchase_order_service`, `sale_service`, `dashboard_service`)
- Removed 5 unused functions
- `ModalScreen` trait eliminates 4 identical save/cancel closures
- Extracted `monstock-backup`, `monstock-import`, `monstock-cli` crates
- `quantity_on_hand` field added to `NewProduct` (removes seed's raw diesel hack)
- DB path moved to `dirs::data_dir()` (no more CWD-relative)
- Validation errors now i18n-aware (`lang` param in all save methods)
- Horizontal scroll on DataTable + full bidirectional scroll on main panel

### Bugfixes
- Purged `inventory_desktop.html` from git history
- DB path fix: `~/.local/share/monstock/monstock.db` (Linux)

### Tech
- 5 workspace crates: `monstock-core`, `monstock-desktop`, `monstock-backup`, `monstock-import`, `monstock-cli`
- 26 integration tests
