use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::data::errors::DataError;

pub struct CreatePrintOrderCommand {
    pub user_id: i32,
    pub file_name: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub material: String,
    pub color: Option<String>,
    pub quality: String,
    pub infill_percentage: i32,
    pub model_volume_cm3: Option<Decimal>,
    pub estimated_print_time_hours: Option<Decimal>,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total_price: Decimal,
    pub delivery_name: String,
    pub delivery_phone: String,
    pub delivery_address: String,
    pub delivery_notes: Option<String>,
}

pub async fn create_print_order(
    db: &PgPool,
    cmd: CreatePrintOrderCommand,
) -> Result<i32, DataError> {
    let result = sqlx::query!(
        r#"
        INSERT INTO print_orders (
            user_id, file_name, file_path, file_size_bytes,
            material, color, quality, infill_percentage,
            model_volume_cm3, estimated_print_time_hours,
            quantity, unit_price, total_price,
            delivery_name, delivery_phone, delivery_address, delivery_notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        RETURNING order_id
        "#,
        cmd.user_id,
        cmd.file_name,
        cmd.file_path,
        cmd.file_size_bytes,
        cmd.material,
        cmd.color,
        cmd.quality,
        cmd.infill_percentage,
        cmd.model_volume_cm3,
        cmd.estimated_print_time_hours,
        cmd.quantity,
        cmd.unit_price,
        cmd.total_price,
        cmd.delivery_name,
        cmd.delivery_phone,
        cmd.delivery_address,
        cmd.delivery_notes
    )
    .fetch_one(db)
    .await?;

    Ok(result.order_id)
}

pub async fn update_payment_status(
    db: &PgPool,
    order_id: i32,
    payment_status: &str,
    payment_method: Option<&str>,
    payment_transaction_id: Option<&str>,
) -> Result<(), DataError> {
    // Update payment status and set paid_at if status is 'paid'
    if payment_status == "paid" {
        sqlx::query!(
            r#"
            UPDATE print_orders
            SET payment_status = $1,
                payment_method = $2,
                payment_transaction_id = $3,
                paid_at = now()
            WHERE order_id = $4
            "#,
            payment_status,
            payment_method,
            payment_transaction_id,
            order_id
        )
        .execute(db)
        .await?;
    } else {
        sqlx::query!(
            r#"
            UPDATE print_orders
            SET payment_status = $1,
                payment_method = $2,
                payment_transaction_id = $3
            WHERE order_id = $4
            "#,
            payment_status,
            payment_method,
            payment_transaction_id,
            order_id
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

pub async fn update_order_status(
    db: &PgPool,
    order_id: i32,
    status: &str,
) -> Result<(), DataError> {
    sqlx::query!(
        r#"
        UPDATE print_orders
        SET status = $1
        WHERE order_id = $2
        "#,
        status,
        order_id
    )
    .execute(db)
    .await?;

    Ok(())
}
