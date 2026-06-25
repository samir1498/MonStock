use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::models::*;
use crate::schema::purchase_orders;
use crate::repos::purchase_order_repo;

#[derive(Debug, Clone)]
pub struct PurchaseOrderInput {
    pub purchase_order_number: String,
    pub supplier_id: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PurchaseOrderItemInput {
    pub product_name: String,
    pub quantity: i32,
    pub unit_cost: f64,
}

#[derive(Debug, Clone)]
pub struct PurchaseOrderResult {
    pub order: PurchaseOrder,
    pub items: Vec<PurchaseOrderItem>,
}

pub fn create_purchase_order(
    conn: &mut SqliteConnection,
    po: &PurchaseOrderInput,
    items: &[PurchaseOrderItemInput],
) -> QueryResult<PurchaseOrderResult> {
    conn.transaction::<PurchaseOrderResult, diesel::result::Error, _>(|conn| {
        let order = diesel::insert_into(purchase_orders::table)
            .values(&NewPurchaseOrder {
                purchase_order_number: po.purchase_order_number.clone(),
                supplier_id: po.supplier_id,
                status: "Draft".to_string(),
                notes: po.notes.clone(),
                total: 0.0,
            })
            .returning(PurchaseOrder::as_returning())
            .get_result(conn)?;

        let new_items: Vec<NewPurchaseOrderItem> = items
            .iter()
            .map(|item| NewPurchaseOrderItem {
                purchase_order_id: order.id,
                product_name: item.product_name.clone(),
                quantity: item.quantity,
                unit_cost: item.unit_cost,
                line_total: 0.0,
            })
            .collect();

        let result_items = purchase_order_repo::insert_purchase_order_items(conn, &new_items)?;

        let computed_total: f64 = result_items.iter().map(|i| i.line_total).sum();

        diesel::update(purchase_orders::table.find(order.id))
            .set(purchase_orders::total.eq(computed_total))
            .execute(conn)?;

        let updated_order = purchase_orders::table
            .find(order.id)
            .select(PurchaseOrder::as_select())
            .first(conn)?;

        Ok(PurchaseOrderResult {
            order: updated_order,
            items: result_items,
        })
    })
}

pub fn receive_purchase_order(
    conn: &mut SqliteConnection,
    purchase_order_id: i32,
) -> QueryResult<()> {
    purchase_order_repo::receive_purchase_order(conn, purchase_order_id)
}

pub fn find_all(
    conn: &mut SqliteConnection,
) -> QueryResult<Vec<PurchaseOrder>> {
    purchase_order_repo::find_all_purchase_orders(conn)
}

pub fn find_by_id(
    conn: &mut SqliteConnection,
    purchase_order_id: i32,
) -> QueryResult<Option<PurchaseOrder>> {
    purchase_order_repo::find_purchase_order_by_id(conn, purchase_order_id)
}

pub fn find_items(
    conn: &mut SqliteConnection,
    purchase_order_id: i32,
) -> QueryResult<Vec<PurchaseOrderItem>> {
    purchase_order_repo::find_items_by_purchase_order(conn, purchase_order_id)
}

pub fn delete(
    conn: &mut SqliteConnection,
    purchase_order_id: i32,
) -> QueryResult<bool> {
    purchase_order_repo::delete_purchase_order(conn, purchase_order_id)
}
