use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintOrderDto {
    pub order_id: i32,
    pub user_id: i32,
    pub file_name: String,
    pub material: String,
    pub color: Option<String>,
    pub quality: String,
    pub infill_percentage: i32,
    pub quantity: i32,
    pub unit_price: rust_decimal::Decimal,
    pub total_price: rust_decimal::Decimal,
    pub delivery_name: String,
    pub delivery_phone: String,
    pub delivery_address: String,
    pub delivery_notes: Option<String>,
    pub status: String,
    pub payment_status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePrintOrderRequest {
    pub file_name: String,
    pub material: String,
    pub color: Option<String>,
    pub quality: String,
    pub infill_percentage: i32,
    pub quantity: i32,
    pub delivery_name: String,
    pub delivery_phone: String,
    pub delivery_address: String,
    pub delivery_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PriceEstimateRequest {
    pub model_volume_cm3: f64,
    pub material: String,
    pub quality: String,
    pub infill_percentage: i32,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct PriceEstimateResponse {
    pub unit_price: rust_decimal::Decimal,
    pub total_price: rust_decimal::Decimal,
    pub estimated_print_time_hours: f64,
}
