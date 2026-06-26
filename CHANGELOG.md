# Changelog

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
