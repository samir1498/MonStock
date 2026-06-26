use std::io::Write;
use monstock_core::models::*;
use monstock_core::db;
use diesel::prelude::*;
use monstock_core::services::{product_service, expense_service, sale_service, purchase_order_service};
use diesel::SqliteConnection;

fn random_range(min: i32, max: i32) -> i32 {
    min + (rand_small() % (max - min + 1))
}

fn rand_small() -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    (nanos % 2147483647) as i32
}

fn pick<T: Clone>(items: &[T]) -> T {
    let idx = (rand_small().unsigned_abs() as usize) % items.len();
    items[idx].clone()
}

fn seed_products(conn: &mut SqliteConnection) -> Vec<i32> {
    let categories = ["Épicerie", "Boisson", "Laitier", "Boulangerie", "Fruits", "Légumes", "Surgelés", "Hygiène", "Ménage", "Snacks", "Conserves", "Pâtes", "Sauces", "Confiserie", "Céréales"];
    let brands = ["Couscous", "Elio", "Sadia", "Danone", "Baignol", "Farine", "Huile", "Sucre", "Sel", "Lait", "Jus", "Eau", "Pain", "Bonbon", "Chocolat"];
    let units = ["1L", "500ml", "250g", "1kg", "500g", "2L", "750ml", "200g", "300g", "150g"];

    let mut ids = Vec::new();
    for i in 1..=500 {
        let cat = pick(&categories);
        let brand = pick(&brands);
        let unit = pick(&units);
        let name = format!("{} {} {}", brand, cat, unit);
        let barcode = if i % 3 == 0 { Some(format!("200{:07}{:03}", random_range(100, 999), i)) } else { None };
        let cost_price = (random_range(50, 2000) as f64).round();
        let selling_price = cost_price * (1.0 + random_range(10, 60) as f64 / 100.0);
        let qty = random_range(0, 200);

        let new_product = NewProduct {
            name: name.clone(),
            barcode: barcode.clone(),
            cost_price,
            selling_price,
        };

        if let Ok(p) = product_service::create(conn, &new_product) {
            diesel::update(monstock_core::schema::products::table.find(p.id))
                .set(monstock_core::schema::products::quantity_on_hand.eq(qty))
                .execute(conn).ok();
            ids.push(p.id);
        }
    }
    ids
}

fn seed_suppliers(conn: &mut SqliteConnection) -> Vec<i32> {
    let names = ["Distributrice Ghrib", "Sarl El Baraka", "Eurl Mami", "Spa Sobiest", "Sarl Icosium",
        "Distributrice Hamza", "Eurl Nour", "Sarl Al Baraka", "Distributrice Farid", "Spa Cevital"];
    let mut ids = Vec::new();
    for name in &names {
        let new_supplier = diesel::insert_into(monstock_core::schema::suppliers::table)
            .values(monstock_core::schema::suppliers::name.eq(name))
            .returning(monstock_core::schema::suppliers::id)
            .get_result::<i32>(conn).ok();
        if let Some(id) = new_supplier { ids.push(id); }
    }
    ids
}

fn seed_expenses(conn: &mut SqliteConnection, days_back: i64) {
    let daily_cats = ["Livraison", "Fournitures"];
    for day in 0..days_back {
        let date = (chrono::Local::now() - chrono::Duration::days(day)).format("%Y-%m-%d").to_string();
        let n = random_range(0, 2);
        for _ in 0..n {
            let cat = pick(&daily_cats);
            let amount = match cat {
                "Livraison" => random_range(200, 800) as f64,
                _ => random_range(300, 1500) as f64,
            };
            let desc = format!("{} - {}", cat, date);
            let expense = NewExpense {
                date: date.clone(),
                category: cat.to_string(),
                description: Some(desc),
                amount,
            };
            expense_service::create(conn, &expense).ok();
        }
    }
    // Monthly expenses: rent, electricity, water, salaries
    let current = chrono::Local::now();
    for month_offset in 0..(days_back / 30).max(1) {
        let month_start = current - chrono::Duration::days(month_offset * 30);
        let ref_day = month_start.format("%Y-%m-15").to_string();
        for (cat, low, high, desc) in [
            ("Loyer", 30000, 50000, "Loyer local commercial"),
            ("Électricité", 3000, 7000, "Facture électricité"),
            ("Eau", 800, 2000, "Facture eau"),
        ] {
            let expense = NewExpense {
                date: ref_day.clone(),
                category: cat.to_string(),
                description: Some(desc.to_string()),
                amount: random_range(low, high) as f64,
            };
            expense_service::create(conn, &expense).ok();
        }
        let expense = NewExpense {
            date: (month_start - chrono::Duration::days(2)).format("%Y-%m-%d").to_string(),
            category: "Salaires".to_string(),
            description: Some("Salaires employés".to_string()),
            amount: random_range(40000, 70000) as f64,
        };
        expense_service::create(conn, &expense).ok();
    }
}

fn seed_sales(conn: &mut SqliteConnection, product_ids: &[i32], days_back: i64) {
    for day in 0..days_back {
        let date = (chrono::Local::now() - chrono::Duration::days(day)).format("%Y-%m-%d").to_string();
        let is_weekday = match chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
            Ok(d) => d.format("%u").to_string().parse::<u32>().unwrap_or(7) < 6,
            Err(_) => true,
        };
        let n = if is_weekday { random_range(30, 70) } else { random_range(15, 35) };
        for _ in 0..n {
            let hours = random_range(8, 21);
            let minutes = random_range(0, 59);
            let timestamp = format!("{}T{:02}:{:02}:00", date, hours, minutes);
            let items_count = random_range(1, 5);
            let mut inputs = Vec::new();
            for _ in 0..items_count {
                let pid = product_ids[(rand_small().unsigned_abs() as usize) % product_ids.len()];
                if let Ok(Some(product)) = product_service::find_by_id(conn, pid) {
                    let qty = random_range(1, 5);
                    inputs.push(sale_service::SaleItemInput {
                        product_id: pid,
                        product_name: product.name,
                        quantity: qty,
                        selling_price: product.selling_price,
                        cost_price: product.cost_price,
                    });
                }
            }
            if !inputs.is_empty() {
                sale_service::record_sale(conn, &timestamp, &inputs).ok();
            }
        }
    }
}

fn seed_purchase_orders(conn: &mut SqliteConnection, supplier_ids: &[i32], days_back: i64) {
    for day in 0..days_back {
        let n = random_range(0, 3);
        for _ in 0..n {
            let sid = if rand_small() % 3 == 0 { None } else { Some(pick(supplier_ids)) };
            let date = (chrono::Local::now() - chrono::Duration::days(day)).format("%Y-%m-%d");
            let po_number = format!("PO-{}-{:03}", date, random_range(1, 99));
            let po_input = purchase_order_service::PurchaseOrderInput {
                purchase_order_number: po_number,
                supplier_id: sid,
                notes: Some("Seed data".to_string()),
            };
            let mut items = Vec::new();
            let item_count = random_range(1, 5);
            for _ in 0..item_count {
                items.push(purchase_order_service::PurchaseOrderItemInput {
                    product_name: format!("Produit {}", random_range(1, 100)),
                    quantity: random_range(10, 100),
                    unit_cost: random_range(100, 1500) as f64,
                });
            }
            purchase_order_service::create_purchase_order(conn, &po_input, &items).ok();
        }
    }
}

fn main() {
    let db_path = "monstock.db";
    if std::path::Path::new(db_path).exists() {
        print!("Database exists. Delete and recreate? (y/N): ");
        std::io::stdout().flush().ok();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        if input.trim().to_lowercase() != "y" {
            println!("Aborting.");
            return;
        }
        std::fs::remove_file(db_path).ok();
        std::fs::remove_file("monstock.db-shm").ok();
        std::fs::remove_file("monstock.db-wal").ok();
    }

    let conn = &mut db::open(db_path).expect("Failed to open database");
    db::run_migrations(conn);

    println!("Seeding 500 products...");
    let product_ids = seed_products(conn);
    println!("  {} products created", product_ids.len());

    println!("Seeding 10 suppliers...");
    let supplier_ids = seed_suppliers(conn);
    println!("  {} suppliers created", supplier_ids.len());

    println!("Seeding 90 days of expenses...");
    seed_expenses(conn, 90);
    println!("  expenses created");

    println!("Seeding 90 days of sales...");
    seed_sales(conn, &product_ids, 90);
    println!("  sales created");

    println!("Seeding 90 days of purchase orders...");
    seed_purchase_orders(conn, &supplier_ids, 90);
    println!("  purchase orders created");

    println!("Done! Database seeded with realistic test data.");
    println!("Run with: cargo run --bin seed");
}
