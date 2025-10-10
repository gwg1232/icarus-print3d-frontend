use axum::{
    Extension,
    extract::{Multipart, State},
    response::{IntoResponse, Redirect, Response},
};
use rust_decimal::Decimal;
use sqlx::PgPool;
use tokio::{fs, io::AsyncWriteExt};
use tower_sessions::Session;

use crate::{
    auth::CurrentUser,
    data::commands,
    flash::FlashMessage,
    handlers::errors::HandlerError,
    paths,
    pricing,
};

pub async fn post_forms_print_order(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    mut multipart: Multipart,
) -> Result<Response, HandlerError> {
    let user_id = current_user.require_authenticated();

    // 폼 데이터를 저장할 변수들
    let mut file_name = String::new();
    let mut file_data: Option<Vec<u8>> = None;
    let mut material = String::from("pla");
    let mut color: Option<String> = None;
    let mut quality = String::from("normal");
    let mut infill_percentage: i32 = 20;
    let mut quantity: i32 = 1;
    let mut model_volume_cm3: f64 = 100.0; // 기본값, 실제로는 CAD 파일 분석 필요
    let mut delivery_name = String::new();
    let mut delivery_phone = String::new();
    let mut delivery_address = String::new();
    let mut delivery_notes: Option<String> = None;

    // 멀티파트 폼 데이터 파싱
    while let Some(field) = multipart.next_field().await.map_err(|_| {
        HandlerError::Data(crate::data::errors::DataError::Internal("Failed to read form field"))
    })? {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "cad_file" => {
                file_name = field
                    .file_name()
                    .unwrap_or("unnamed.stl")
                    .to_string();
                file_data = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| {
                            HandlerError::Data(crate::data::errors::DataError::Internal(
                                "Failed to read file",
                            ))
                        })?
                        .to_vec(),
                );
            }
            "material" => {
                material = field.text().await.unwrap_or_else(|_| "pla".to_string());
            }
            "color" => {
                let c = field.text().await.unwrap_or_default();
                if !c.is_empty() {
                    color = Some(c);
                }
            }
            "quality" => {
                quality = field.text().await.unwrap_or_else(|_| "normal".to_string());
            }
            "infill_percentage" => {
                infill_percentage = field
                    .text()
                    .await
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()
                    .unwrap_or(20);
            }
            "quantity" => {
                quantity = field
                    .text()
                    .await
                    .unwrap_or_else(|_| "1".to_string())
                    .parse()
                    .unwrap_or(1);
            }
            "model_volume_cm3" => {
                model_volume_cm3 = field
                    .text()
                    .await
                    .unwrap_or_else(|_| "100.0".to_string())
                    .parse()
                    .unwrap_or(100.0);
            }
            "delivery_name" => {
                delivery_name = field.text().await.unwrap_or_default();
            }
            "delivery_phone" => {
                delivery_phone = field.text().await.unwrap_or_default();
            }
            "delivery_address" => {
                delivery_address = field.text().await.unwrap_or_default();
            }
            "delivery_notes" => {
                let notes = field.text().await.unwrap_or_default();
                if !notes.is_empty() {
                    delivery_notes = Some(notes);
                }
            }
            _ => {}
        }
    }

    // 파일 검증
    let file_bytes = file_data.ok_or_else(|| {
        HandlerError::Data(crate::data::errors::DataError::Internal("CAD 파일이 필요합니다"))
    })?;

    // 배송 정보 검증
    if delivery_name.is_empty() || delivery_phone.is_empty() || delivery_address.is_empty() {
        FlashMessage::error("배송 정보를 모두 입력해주세요")
            .set(&session)
            .await?;
        return Ok(Redirect::to(paths::print_orders::new_order::PATH).into_response());
    }

    // 파일 저장
    let upload_dir = "uploads/cad_files";
    fs::create_dir_all(upload_dir)
        .await
        .map_err(|_| {
            HandlerError::Data(crate::data::errors::DataError::Internal(
                "Failed to create upload directory",
            ))
        })?;

    let file_path = format!("{}/{}-{}", upload_dir, user_id, file_name);
    let mut file = fs::File::create(&file_path).await.map_err(|_| {
        HandlerError::Data(crate::data::errors::DataError::Internal("Failed to save file"))
    })?;

    file.write_all(&file_bytes).await.map_err(|_| {
        HandlerError::Data(crate::data::errors::DataError::Internal("Failed to write file"))
    })?;

    // 가격 계산
    let (unit_price, total_price, estimated_time) = pricing::calculate_price(
        model_volume_cm3,
        &material,
        &quality,
        infill_percentage,
        quantity,
    );

    // 주문 생성
    let _order_id = commands::print_order::create_print_order(
        &db,
        commands::print_order::CreatePrintOrderCommand {
            user_id,
            file_name,
            file_path,
            file_size_bytes: file_bytes.len() as i64,
            material,
            color,
            quality,
            infill_percentage,
            model_volume_cm3: Decimal::from_f64_retain(model_volume_cm3),
            estimated_print_time_hours: Decimal::from_f64_retain(estimated_time),
            quantity,
            unit_price,
            total_price,
            delivery_name,
            delivery_phone,
            delivery_address,
            delivery_notes,
        },
    )
    .await?;

    FlashMessage::success("주문이 성공적으로 생성되었습니다")
        .set(&session)
        .await?;

    Ok(Redirect::to(paths::print_orders::BASE).into_response())
}
