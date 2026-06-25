# MonStock

Inventory and sales management system for small shops.
Built with Rust + egui + SQLite (Diesel ORM).

## Features

### 1. Purchase Orders (Bon d'Achat)
- Create purchase orders when stock arrives
- Track supplier info, items, quantities, and cost prices
- Auto-generates purchase order numbers (e.g. PO-20260625-001)
- Status tracking: Draft → Received (auto-creates/updates products, increments stock on receive)

### 2. Stock & Inventory
- Products track `quantity_on_hand` — incremented on purchase order receipt, decremented on sale
- View current stock levels and low-stock alerts

### 3. Product Pricing & Barcodes
- Assign barcodes to products (manual or scanner input)
- Set selling price per product
- Auto-calculates margin percentage
- Overview table of all priced products

### 4. Expenses (Les Charges)
- Record daily operating costs
- Predefined categories: Livraison, Électricité, Eau, Tabac, Chema, Loyer, Autre
- View totals by day, week, month
- Filter by category

### 5. Dashboard (End of Day)
- Daily sales summary
- Profit calculation: Sales – Cost of Goods – Expenses
- Margin percentage
- Transactions count, top-selling product
- Sales per hour chart
- Recent transactions list
- Date picker for historical view

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust |
| GUI | egui + eframe (desktop) |
| ORM | Diesel with SQLite |
| Migrations | diesel_migrations (embedded, auto-run) |
| Charts | egui_plot |
| Barcodes | USB scanner (keyboard emulation, no library needed) |

## Architecture: Core / GUI Split

Business logic lives in `monstock-core` — a pure Rust library with **no GUI dependencies**. The desktop UI (`monstock-desktop`) depends on the core crate. This means:

- **Switch to Tauri?** Just create `monstock-tauri` and depend on `monstock-core`.
- **Add a CLI?** Depend on `monstock-core`, reuse all models + repos + db.
- **Unit test business logic?** Test `monstock-core` directly.

```
MonStock/
├── Cargo.toml                              # workspace root
│
├── monstock-core/                          # lib crate (GUI-agnostic)
│   ├── Cargo.toml                          # diesel, chrono, serde, thiserror
│   ├── .env                                # DATABASE_URL=monstock.db
│   ├── migrations/                         # Diesel SQL migrations (auto-run)
│   └── src/
│       ├── lib.rs
│       ├── schema.rs                       # Diesel schema (table definitions)
│       ├── db.rs                           # open() connection + migration runner
│       ├── models/                         # Product, Supplier, PurchaseOrder+Items,
│       │                                     Expense, Transaction+Items
│       └── repos/                          # Typed CRUD for all entities
│
├── monstock-desktop/                       # binary crate (egui)
│   ├── Cargo.toml                          # depends on monstock-core, eframe, egui
│   └── src/main.rs                         # eframe entry point
│
├── inventory_desktop.html                  # Design reference (StockFlow mockup)
└── data/
    └── monstock.db                         # SQLite database (auto-created on first run)
```

## Database Schema

Diesel manages schema via `diesel migration run`. Current tables:

### products
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | |
| name | TEXT NOT NULL | |
| barcode | TEXT UNIQUE | |
| cost_price | REAL | Latest cost from PO |
| selling_price | REAL | |
| created_at | TEXT | ISO 8601 |
| quantity_on_hand | INTEGER | Current stock count, updated on PO receive and sale |

### suppliers
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | |
| name | TEXT NOT NULL | |
| phone | TEXT | |
| notes | TEXT | |

### purchase_orders
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | |
| purchase_order_number | TEXT UNIQUE | e.g. PO-20260625-001 |
| supplier_id | INTEGER FK → suppliers | |
| status | TEXT | Draft / Received |
| notes | TEXT | |
| total | REAL | |
| created_at | TEXT | |

### purchase_order_items
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | |
| purchase_order_id | INTEGER FK → purchase_orders (CASCADE) | |
| product_name | TEXT | |
| quantity | INTEGER | |
| unit_cost | REAL | |
| line_total | REAL | |

### expense_categories

| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | |
| name | TEXT UNIQUE | Livraison, Électricité, Eau, Loyer, Autre |
| created_at | TEXT | |

### expenses
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | |
| date | TEXT | |
| category | TEXT | Livraison, Électricité, etc. |
| description | TEXT | |
| amount | REAL | |
| created_at | TEXT | |

### transactions
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | |
| timestamp | TEXT | |
| total | REAL | |

### transaction_items
| Column | Type | Notes |
|--------|------|-------|
| id | INTEGER PK | |
| transaction_id | INTEGER FK → transactions (CASCADE) | |
| product_id | INTEGER FK → products | |
| product_name | TEXT | |
| quantity | INTEGER | |
| selling_price | REAL | Price at time of sale |
| cost_price | REAL | Cost at time of sale |
| line_total | REAL | |

## Build & Run

```bash
# Prerequisites
cargo install diesel_cli --no-default-features --features sqlite

# Run Diesel migrations (one-time setup)
cd monstock-core
diesel migration run
cd ..

# Build & run desktop app
cargo run -p monstock-desktop --release

# The database is auto-created and migrated on first launch
```

## Adding new migrations

```bash
cd monstock-core
diesel migration generate <migration_name>
# Edit up.sql and down.sql
diesel migration run              # apply
diesel migration redo             # rollback + re-apply
```

## Key Dependencies

```toml
# monstock-core
diesel = { version = "2.2", features = ["sqlite", "returning_clauses_for_sqlite_3_35"] }
diesel_migrations = "2.2"
chrono = "0.4"
serde = "1"

# monstock-desktop
monstock-core = { path = "../monstock-core" }
eframe = "0.34"
egui = "0.34"
egui_plot = "0.34"      # planned
```

## Design Reference

The file `inventory_desktop.html` contains a polished dark-theme mockup (StockFlow) that serves as the UI target. It includes:
- Sidebar navigation with icons + badges
- Stats cards with trend indicators
- Bar chart for stock movement
- Activity feed (recent actions)
- Product table with stock bars, status badges, pagination
- Modal forms for add/edit
- Search bar with real-time filtering

## Barcode Scanner Support

USB barcode scanners act as keyboard input. No special library needed.
Just ensure the barcode field has focus — the scanner types the digits
and sends an Enter key.

## Currency

Algerian Dinar (DA). Amounts stored as REAL in database.

## License

MIT