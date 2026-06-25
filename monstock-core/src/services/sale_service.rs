use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::models::*;
use crate::schema::{transactions, transaction_items, products};
use crate::repos::transaction_repo;

#[derive(Debug, Clone)]
pub struct SaleItemInput {
    pub product_id: i32,
    pub product_name: String,
    pub quantity: i32,
    pub selling_price: f64,
    pub cost_price: f64,
}

#[derive(Debug, Clone)]
pub struct SaleResult {
    pub transaction: Transaction,
    pub items: Vec<TransactionItem>,
}

pub fn record_sale(
    conn: &mut SqliteConnection,
    timestamp: &str,
    items: &[SaleItemInput],
) -> QueryResult<SaleResult> {
    conn.transaction::<SaleResult, diesel::result::Error, _>(|conn| {
        let tx = diesel::insert_into(transactions::table)
            .values(&NewTransaction {
                timestamp: timestamp.to_string(),
                total: 0.0,
            })
            .returning(Transaction::as_returning())
            .get_result(conn)?;

        let mut result_items = Vec::with_capacity(items.len());
        let mut computed_total = 0.0;

        for item in items {
            let line_total = item.quantity as f64 * item.selling_price;
            computed_total += line_total;

            let current_stock = products::table
                .find(item.product_id)
                .select(products::quantity_on_hand)
                .first::<i32>(conn)
                .optional()?
                .unwrap_or(0);

            if current_stock < item.quantity {
                return Err(diesel::result::Error::RollbackTransaction);
            }

            diesel::update(products::table.find(item.product_id))
                .set(products::quantity_on_hand.eq(products::quantity_on_hand - item.quantity))
                .execute(conn)?;

            let ti = diesel::insert_into(transaction_items::table)
                .values((
                    transaction_items::transaction_id.eq(tx.id),
                    transaction_items::product_id.eq(item.product_id),
                    transaction_items::product_name.eq(&item.product_name),
                    transaction_items::quantity.eq(item.quantity),
                    transaction_items::selling_price.eq(item.selling_price),
                    transaction_items::cost_price.eq(item.cost_price),
                    transaction_items::line_total.eq(line_total),
                ))
                .returning(TransactionItem::as_returning())
                .get_result(conn)?;

            result_items.push(ti);
        }

        diesel::update(transactions::table.find(tx.id))
            .set(transactions::total.eq(computed_total))
            .execute(conn)?;

        let updated_tx = transactions::table
            .find(tx.id)
            .select(Transaction::as_select())
            .first(conn)?;

        Ok(SaleResult {
            transaction: updated_tx,
            items: result_items,
        })
    })
}

pub fn find_transactions_by_date(
    conn: &mut SqliteConnection,
    date: &str,
) -> QueryResult<Vec<Transaction>> {
    transaction_repo::find_transactions_by_date(conn, date)
}

pub fn find_items_by_transaction(
    conn: &mut SqliteConnection,
    transaction_id: i32,
) -> QueryResult<Vec<TransactionItem>> {
    transaction_repo::find_items_by_transaction(conn, transaction_id)
}

pub fn daily_sales_total(
    conn: &mut SqliteConnection,
    date: &str,
) -> QueryResult<f64> {
    transaction_repo::daily_sales_total(conn, date)
}

pub fn daily_cost_total(
    conn: &mut SqliteConnection,
    date: &str,
) -> QueryResult<f64> {
    transaction_repo::daily_cost_total(conn, date)
}

pub fn transaction_count_by_date(
    conn: &mut SqliteConnection,
    date: &str,
) -> QueryResult<i64> {
    transaction_repo::transaction_count_by_date(conn, date)
}
