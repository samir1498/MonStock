use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::models::*;
use crate::repos::product_repo;

pub fn create(
    conn: &mut SqliteConnection,
    product: &NewProduct,
) -> QueryResult<Product> {
    product_repo::insert_product(conn, product)
}

pub fn find_all(
    conn: &mut SqliteConnection,
) -> QueryResult<Vec<Product>> {
    product_repo::find_all_products(conn)
}

pub fn find_by_id(
    conn: &mut SqliteConnection,
    product_id: i32,
) -> QueryResult<Option<Product>> {
    product_repo::find_product_by_id(conn, product_id)
}

pub fn find_by_barcode(
    conn: &mut SqliteConnection,
    barcode: &str,
) -> QueryResult<Option<Product>> {
    product_repo::find_product_by_barcode(conn, barcode)
}

pub fn update(
    conn: &mut SqliteConnection,
    product_id: i32,
    product: &NewProduct,
) -> QueryResult<()> {
    product_repo::update_product(conn, product_id, product)
}

pub fn set_cost_price(
    conn: &mut SqliteConnection,
    product_id: i32,
    cost_price: f64,
) -> QueryResult<()> {
    product_repo::set_product_cost_price(conn, product_id, cost_price)
}

pub fn delete(
    conn: &mut SqliteConnection,
    product_id: i32,
) -> QueryResult<bool> {
    product_repo::delete_product(conn, product_id)
}

pub fn margin_pct(product: &Product) -> f64 {
    product.margin_pct()
}
