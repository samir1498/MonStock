# MonStock

Inventory & sales management for small shops. Rust (egui + Tauri) + SQLite. Offline, cross-platform, open source.

## Screenshots

| Dark mode | Light mode |
|---|---|
| ![Dashboard dark](screenshots/dashboard-dark.png) | ![Dashboard light](screenshots/dashboard-light.png) |

## Features

- Stock tracking with low-stock alerts and barcode support
- Purchase orders (create, receive, auto-update stock)
- Sales recording with cost/profit tracking
- Daily dashboard: profit/loss, sales chart, stats cards
- Expenses with categories and date filtering
- French / English interface
- Dark / light theme
- Daily automatic backups + manual backup
- Pagination on all list screens

## Download

Grab the latest binary from [Releases](https://github.com/samir1498/MonStock/releases).

| Platform | egui Desktop | Tauri |
|---|---|---|
| Linux | `monstock-x86_64-unknown-linux-gnu` | `.deb` / `.AppImage` / `.rpm` |
| Windows | `monstock-x86_64-pc-windows-msvc.exe` | `.msi` |
| macOS | `monstock-x86_64-apple-darwin` | `.dmg` |

egui: needs `libxcb1-dev` and `libgtk-3-dev`.  
Tauri (Linux): needs `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`.

## Build from source

```bash
git clone https://github.com/samir1498/MonStock
cd MonStock

# egui desktop
cargo run -p monstock-desktop --release

# Tauri desktop (pnpm required)
cd monstock-tauri
pnpm install
pnpm run dev        # dev server + Tauri window
pnpm run build      # tsc + vite build
pnpm run test       # vitest (67 tests)
cargo tauri build   # production bundles (.deb/.AppImage/.msi/.dmg)
```

Seed demo data: `cargo run --bin seed`

## Project structure

```
MonStock/
├── Cargo.toml                  # workspace root
├── monstock-core/              # lib — models, repos, services (no GUI)
├── monstock-desktop/           # binary — egui frontend
├── monstock-tauri/             # Tauri v2 app (React frontend + Rust backend)
├── monstock-backup/            # backup/restore logic
├── monstock-import/            # import from external formats
├── monstock-cli/               # CLI utilities
└── docs/                       # design references
```

Business logic is GUI-agnostic — could be reused for CLI, Tauri, or web frontends.

### Environment

Tauri dev on Wayland requires `WEBKIT_DISABLE_DMABUF_RENDERER=1`.
egui desktop uses the `egui` feature flag — Linux needs `libxcb1-dev`, `libgtk-3-dev`.

## Versioning

- egui desktop tracks workspace version (current `0.3.0`), tagged `desktop-v*`
- Tauri versions are independent (current `0.2.0`), tagged `tauri-v*`

Both releases are built via GitHub Actions and published to [Releases](https://github.com/samir1498/MonStock/releases). Tauri includes auto-update via `tauri-plugin-updater`.

## Built with

| | |
|---|---|---|
| Language | Rust, TypeScript |
| GUI | egui / eframe, Tauri v2 + React 19 |
| Database | SQLite + Diesel ORM |
| Fonts | OpenSans + JetBrains Mono (egui), Inter + JetBrains Mono (Tauri) |

## License

MIT
