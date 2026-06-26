<h1 align="center">MonStock</h1>

<p align="center">
  <strong>Inventory &amp; Sales Management for Small Shops</strong>
  <br>
  <sub>Lightweight · Offline-first · Cross-platform · Open source</sub>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#install">Install</a> •
  <a href="#build">Build</a> •
  <a href="#tech">Tech</a>
</p>

---

## Why MonStock?

Running a small shop means tracking stock, purchases, sales, and daily expenses — usually on paper or spreadsheets. MonStock replaces that with a fast, offline desktop app. No internet required. No subscriptions. Your data stays on your machine.

---

## Features

| | |
|---|---|
| **📦 Stock Management** | Track inventory levels, low-stock alerts, barcode support |
| **🛒 Purchase Orders** | Create POs, auto-update inventory on receive |
| **💰 Sales Recording** | Record daily sales with cost tracking |
| **📊 Dashboard** | Daily profit/loss, sales chart, recent activity |
| **📄 Expenses** | Category-based expense tracking |
| **🌐 i18n** | French / English toggle |
| **🌙 Dark / Light** | Theme toggle |
| **💾 Backups** | Automatic daily + manual backup |
| **📷 Barcode Scanner** | USB scanner support (keyboard emulation) |
| **📋 Pagination** | All list screens paginated |

---

## Install

Download the latest binary from the [Releases page](https://github.com/samir1498/MonStock/releases).

| Platform | File |
|---|---|
| Linux | `monstock-x86_64-unknown-linux-gnu` |
| Windows | `monstock-x86_64-pc-windows-msvc.exe` |
| macOS | `monstock-x86_64-apple-darwin` |

**Linux:** requires libxcb and libgtk-3 (`sudo apt install libxcb1-dev libgtk-3-dev`).

### From Source

```bash
git clone https://github.com/samir1498/MonStock
cd MonStock
cargo run -p monstock-desktop --release
```

Database and migrations are created automatically on first launch. Optionally seed with demo data:

```bash
cargo run --bin seed
```

---

## Architecture

```
MonStock/
├── Cargo.toml                   # workspace root
│
├── monstock-core/               # lib crate (GUI-agnostic)
│   ├── migrations/              # Diesel SQL migrations
│   └── src/
│       ├── models/              # Product, Supplier, Expense, etc.
│       ├── repos/               # Typed CRUD
│       └── services/            # Business logic
│
├── monstock-desktop/            # binary crate (egui)
│   └── src/
│       ├── main.rs              # eframe entry point
│       ├── screens/             # One module per screen
│       ├── components.rs        # Reusable UI helpers
│       └── style.rs             # Colors, fonts, layout
│
└── docs/                        # Design references
```

Core business logic has zero GUI dependencies — enabling CLI, Tauri, or web frontends without rewriting business rules.

---

## Tech

| Layer | |
|---|---|
| Language | Rust |
| GUI | egui / eframe |
| Database | SQLite via Diesel ORM |
| Fonts | OpenSans + JetBrains Mono |

---

## License

MIT
