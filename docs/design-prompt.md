# MonStock — UI Design Prompt for Kimi LLM

## Project Overview

**MonStock** is a desktop inventory & sales management system built with Rust + egui, targeting small Algerian shops. It's a single-user, offline, dark-themed desktop app. No auth, no cloud, no customers (cash-only economy). All amounts in Algerian Dinar (DA).

**Target user:** Small shop owner in Algeria who needs a simple, fast tool to track stock, purchases, sales, and daily expenses. They want something that feels professional but doesn't overcomplicate things.

**GUI framework:** egui (immediate mode GUI for Rust). The design will be implemented with egui's widgets, so the mockup should be feasible in egui (no complex CSS animations, no z-index layering issues).

---

## Brand Identity

- **Name:** MonStock (Mon = Mon/My in French, Stock = Stock)
- **Tagline:** "Gérez votre stock simplement" (Manage your stock simply)
- **Tone:** Professional but not corporate. Friendly but not childish. Think "tool for a serious shopkeeper, not a toy."
- **Colour personality:** We need a primary brand colour that works for Algeria. Suggestions below.

---

## Design Inspirations

### Primary: pc-ctx-web (Claude Design style)
A dark dashboard UI with these characteristics:
- **Colour scheme:** Deep dark backgrounds (#0b0b12 page, #111118 surface, #181820 elevated), indigo accent (#6366f1), muted lavender secondary (#a1a1b5), subtle borders (#222230)
- **Typography:** System font stack (SF Pro Display / Inter, sans-serif). Good hierarchy: 26px titles, 15px card headers, 13px body, 11px labels
- **Layout:** Fixed sidebar (224px) with icon + text nav, scrollable content area, top bar with search + actions
- **Cards:** Dark surface cards with border, subtle hover lift, coloured top accent lines for stats
- **Badges:** Rounded pills with muted background + coloured text (green/amber/blue/red at 12% opacity)
- **Charts:** Pure SVG donut charts, animated bar charts with gradient fills
- **Animations:** Subtle staggered fade-up on page load, smooth hover transitions, count-up animations for stats
- **Overall feel:** Clean, premium, dark, data-dense but readable

### Secondary: StockFlow HTML mockup (existing `inventory_desktop.html`)
The current mockup has good layout bones (sidebar, stats row, chart + activity columns, product table, modal form) but needs a branding identity makeover.

---

## Colour Palette Suggestions

Pick one direction OR propose something better:

### Option A: Indigo / Deep Blue (matches pc-ctx-web)
- Primary: #6366f1 (indigo) — buttons, active states, links
- Background: #0b0b12 → #111118 → #181820 layered depth
- Success: #22c55e (green)
- Warning: #f59e0b (amber)
- Danger: #ef4444 (red)
- Info: #3b82f6 (blue)
- Text: #ededef primary, #a1a1b5 secondary, #5a6078 muted
- **Vibe:** Modern, SaaS-like, professional

### Option B: Algerian-inspired (green + gold + sand)
- Primary: #10b981 (emerald green) or #059669
- Accent: #d97706 (amber/gold — Sahara sand inspiration)
- Background warm dark: #0f0d0a → #1a1610
- Success: same green family
- Warning: gold/amber
- Danger: #dc2626 (red)
- **Vibe:** Warm, local, grounded in Algerian identity (green of flag, gold of Sahara)
- **Note:** Might be harder to make egui look great with warm tones in dark mode

### Option C: Teal / Cyan (fresh, neutral)
- Primary: #06b6d4 (cyan) or #0891b2
- Accent: #6366f1 (indigo for secondary)
- Background: standard dark
- **Vibe:** Clean, medical-inventory-like, neutral

### Option D: Propose your own
If you have a better direction, go for it. Must work well in a dark theme and look good with egui's rendering.

---

## Screen Designs Needed

For each screen, describe the layout, key widgets, interactions, and flow.

### 1. Dashboard (Day Overview)

This is the main screen the shopkeeper sees all day.

**Data to display:**
- Total sales today (DA)
- Total expenses today (DA)
- Profit today = Sales - COGS - Expenses (DA)
- Margin %
- Transaction count today
- Top-selling product today
- Sales per hour chart (bar chart, 24h)
- Recent activity feed (last 10 actions: items sold, stock received, expenses recorded)
- Low stock alerts strip (products with quantity_on_hand < threshold)

**Layout:**
- Top: Alert strip (red/orange gradient) for low-stock items, clickable to jump to inventory filtered by low stock
- Stats row (4 cards): Sales Today, Expenses, Profit, Transactions
- Middle: 2-column grid — Sales chart (left, wider) + Activity feed (right)
- Optional bottom: Quick actions row (Add Sale, Record Expense, Receive PO)

### 2. Inventory / Products

Full list of all products with search, filter, sort.

**Columns:** Product name, Barcode, Cost Price, Selling Price, Stock qty, Margin %, Status badge (In Stock / Low / Out)

**Interactions:**
- Search bar (real-time filtering)
- Filter pills: All / In Stock / Low Stock / Out of Stock
- Click row → edit product modal
- "+ Add Product" button → modal form
- Pagination (10 per page)
- Stock bar visual (mini progress bar next to stock number)

**Status logic:**
- In Stock: qty > 10 (green)
- Low Stock: qty 1-10 (amber)
- Out of Stock: qty = 0 (red)

### 3. Purchase Orders (Bons d'Achat)

List of purchase orders with status workflow: Draft → Received.

**PO fields:** PO number (auto: PO-YYYYMMDD-NNN), Supplier (dropdown), Items list (product name, qty, unit cost, line total), Total, Status, Notes, Created date

**List view columns:** PO number, Supplier, Status badge, Total (DA), Items count, Created date

**Interactions:**
- Create PO: Modal with supplier select, add items rows, auto-calculates total
- Receive PO: Button to mark as Received, which creates/updates products and increments stock
- View PO: Detail view showing items table

### 4. Suppliers

Simple CRUD list.

**Columns:** Name, Phone, Notes, Purchase order count

**Interactions:** Add/Edit/Delete supplier (modal), click to see related POs

### 5. Barcodes & Pricing

Product pricing overview table.

**Columns:** Product name, Barcode (editable inline), Cost price, Selling price, Margin %, Scanner input field

**Interactions:**
- Manual barcode entry (text input)
- Scanner support: USB scanner acts as keyboard, types digits then Enter. Input field captures it.
- Selling price inline edit
- Auto-calculated margin % (green if >30%, amber if 10-30%, red if <10%)

### 6. Expenses (Les Charges)

Expense recording and viewing with date-range filtering.

**Columns:** Date, Category, Description, Amount (DA)

**Filters:**
- Date range: Day / Week / Month / Custom
- Category filter dropdown (from expense_categories table)
- Summary: Total by category for the period, Grand total

**Interactions:**
- Add expense: Modal with date picker, category dropdown, description, amount
- Categories are dynamic (stored in DB): Livraison, Électricité, Eau, Loyer, Autre

### 7. End-of-Day Summary

What the shopkeeper sees at closing time.

**Sections:**
- Daily recap card: Sales, COGS, Expenses, Net Profit, Margin %
- Transactions list for the day (time, items, total)
- Profit breakdown: Sales - Cost of Goods Sold - Expenses = Net Profit
- Date picker to view any past day
- Export to... (clipboard? screenshot? text summary?)

---

## Layout Structure

```
+----------------------------------------------------------+
| Sidebar (w-56)          | Main                            |
|                         |                                 |
| [MonStock logo/icon]    | Top Bar:                        |
|                         |   Breadcrumbs | Search | +Add   |
| Navigation:             |                                 |
| ─────────────────       | Content (scrollable):           |
| 📊 Dashboard     [dot]  |   ┌─ Page Header ───────────┐  |
| 📦 Inventory             |   │ Title   [Export] [Today]│  |
| 📋 Purchase Orders       |   └─────────────────────────┘  |
| 🏪 Suppliers             |   ┌─ Alert Strip ────────────┐  |
| 🏷️ Barcodes & Pricing   |   │ ⚠ 3 items low stock    │  |
| 💰 Expenses              |   └─────────────────────────┘  |
| 📋 End of Day            |   ┌ Stats Cards (4 cols) ────┐  |
|                         |   │ #1 │ #2 │ #3 │ #4 │       |
| ─────────────────       |   └─────────────────────────┘  |
| Settings                 |   ┌ Chart │ Activity ────────┐  |
|                         |   │       │                   │  |
|                         |   └─────────────────────────┘  |
|                         |   ┌ Product Table ───────────┐  |
|                         |   │ [filters] [pagination]   │  |
|                         |   └─────────────────────────┘  |
+----------------------------------------------------------+
```

---

## UX Principles for Algerian Small Shops

1. **Offline-first:** No internet dependency. Everything local.
2. **Fast interactions:** Keyboard navigation preferred. Tab through fields, Enter to submit. Shopkeepers are fast.
3. **Barcode scanner friendly:** USB scanner = keyboard emulation. Focus management is critical — barcode input field must auto-focus when scanner tab is open.
4. **Algerian Dinar (DA):** Format amounts as `1,500.00 DA`. No $, no €.
5. **French + Arabic:** Default interface in French. Support Arabic RTL in future. For now, French labels.
6. **No auth / single user:** The app is used by one shopkeeper on one machine. No login screen.
7. **Cash-only:** No customer tracking, no credit, no invoicing. Every transaction is cash.
8. **Big fonts & buttons:** egui renders at native resolution. Targets should be at least 32px touch-friendly.
9. **Modal forms > separate pages:** Use modal dialogs for add/edit to keep context. Avoid deep navigation.
10. **Dark theme only:** Saves battery on OLED, looks professional, egui dark themes look better than light.

---

## Technical Constraints (egui)

- egui uses an immediate mode GUI: widgets are laid out in a function called every frame
- No CSS, no HTML, no flexbox — layout is done via `ui.horizontal()`, `ui.vertical()`, `ui.columns()`, `egui::Grid`, `egui::ScrollArea`
- egui has built-in: buttons, labels, text edit, sliders, combo boxes, tables (via `egui_extras::TableBuilder`), plots (via `egui_plot`), modal windows (`egui::Window` with `modal` flag)
- Custom styling via `egui::Style` (visuals, spacing, fonts). The style can be customized per-widget.
- No fancy CSS animations — only simple frame-based animations (lerp between values)
- Icons: egui doesn't have built-in icon fonts. Use text (Unicode emoji or custom widget drawing) or egui's `egui_phosphor` crate for icon glyphs
- The mockup should use a subset of icons that can be represented with simple SVG paths or emoji

---

## Deliverable

Generate a **single HTML file** (dark theme, standalone, no external deps) that serves as the design reference/mockup for all 7 screens. The HTML should:

1. Look premium and polished (like pc-ctx-web quality)
2. Show all states: empty, populated, filtered, modal open
3. Use the proposed colour palette consistently
4. Be mobile-responsive (in case viewed on phone)
5. Include realistic sample data (Algerian Dinar amounts, French labels, Algerian-relevant products like "Couscous", "Huile d'olive", "Datte", "Lait", "Pain")
6. Use system fonts or Inter (Google Fonts)
7. Include hover states, transitions, subtle animations
8. Show at least one modal dialog (Add Product)
9. Show the sidebar with active state indicators
10. Use the brand name **MonStock** prominently

**Brand name suggestion for the sidebar:** Use an icon + "MonStock" text. The icon could be a simple box/cube or box with stock-up arrow. Consider local Algerian elements (palm tree? desert? no — keep it professional, just a clean abstract mark).

### Colour palette options to choose from:
Option A (Indigo): `#6366f1` primary, `#0b0b12` page bg, `#111118` surface, `#ededef` text
Option B (Algerian Green): `#10b981` primary, warm dark backgrounds, gold accents
Option C (Your suggestion): Propose something better


