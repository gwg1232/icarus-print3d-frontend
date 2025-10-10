use sqlx::PgPool;

use crate::{data::errors::DataError, handlers::dtos::print_order::PrintOrderDto};

pub async fn find_by_user_id(db: &PgPool, user_id: i32) -> Result<Vec<PrintOrderDto>, DataError> {
    let orders = sqlx::query_as!(
        PrintOrderDto,
        r#"
        SELECT
            order_id, user_id, file_name, material, color, quality,
            infill_percentage, quantity,
            unit_price as "unit_price: rust_decimal::Decimal",
            total_price as "total_price: rust_decimal::Decimal",
            delivery_name, delivery_phone, delivery_address, delivery_notes,
            status, payment_status,
            created_at as "created_at: chrono::DateTime<chrono::Utc>"
        FROM print_orders
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(orders)
}

pub async fn find_by_id(
    db: &PgPool,
    order_id: i32,
    user_id: i32,
) -> Result<Option<PrintOrderDto>, DataError> {
    let order = sqlx::query_as!(
        PrintOrderDto,
        r#"
        SELECT
            order_id, user_id, file_name, material, color, quality,
            infill_percentage, quantity,
            unit_price as "unit_price: rust_decimal::Decimal",
            total_price as "total_price: rust_decimal::Decimal",
            delivery_name, delivery_phone, delivery_address, delivery_notes,
            status, payment_status,
            created_at as "created_at: chrono::DateTime<chrono::Utc>"
        FROM print_orders
        WHERE order_id = $1 AND user_id = $2
        "#,
        order_id,
        user_id
    )
    .fetch_optional(db)
    .await?;

    Ok(order)
}
