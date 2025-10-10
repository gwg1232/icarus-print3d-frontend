use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// 3D 프린팅 가격 계산 모듈

#[derive(Debug, Clone)]
pub struct PricingConfig {
    pub material_cost_per_gram: Decimal,
    pub labor_cost_per_hour: Decimal,
    pub machine_cost_per_hour: Decimal,
    pub quality_multiplier: Decimal,
}

impl PricingConfig {
    pub fn for_material(material: &str) -> Self {
        match material.to_lowercase().as_str() {
            "pla" => Self {
                material_cost_per_gram: dec!(0.03),  // 그램당 30원
                labor_cost_per_hour: dec!(10.0),
                machine_cost_per_hour: dec!(5.0),
                quality_multiplier: dec!(1.0),
            },
            "abs" => Self {
                material_cost_per_gram: dec!(0.04),  // 그램당 40원
                labor_cost_per_hour: dec!(10.0),
                machine_cost_per_hour: dec!(5.0),
                quality_multiplier: dec!(1.0),
            },
            "petg" => Self {
                material_cost_per_gram: dec!(0.045), // 그램당 45원
                labor_cost_per_hour: dec!(10.0),
                machine_cost_per_hour: dec!(5.0),
                quality_multiplier: dec!(1.0),
            },
            "resin" => Self {
                material_cost_per_gram: dec!(0.15),  // 그램당 150원 (레진은 비쌈)
                labor_cost_per_hour: dec!(15.0),
                machine_cost_per_hour: dec!(10.0),
                quality_multiplier: dec!(1.5),
            },
            _ => Self {
                material_cost_per_gram: dec!(0.05),  // 기본값
                labor_cost_per_hour: dec!(10.0),
                machine_cost_per_hour: dec!(5.0),
                quality_multiplier: dec!(1.0),
            },
        }
    }
}

pub fn get_quality_multiplier(quality: &str) -> Decimal {
    match quality.to_lowercase().as_str() {
        "draft" => dec!(0.8),    // 빠른 출력, 품질 낮음
        "normal" => dec!(1.0),   // 표준 품질
        "high" => dec!(1.3),     // 높은 품질, 시간 많이 걸림
        "ultra" => dec!(1.6),    // 최고 품질, 매우 느림
        _ => dec!(1.0),
    }
}

pub fn calculate_material_weight(
    volume_cm3: f64,
    material: &str,
    infill_percentage: i32,
) -> f64 {
    // 재질별 밀도 (g/cm³)
    let density = match material.to_lowercase().as_str() {
        "pla" => 1.24,
        "abs" => 1.04,
        "petg" => 1.27,
        "resin" => 1.2,
        _ => 1.2,
    };

    // 내부 채움률을 고려한 무게 계산
    // 기본적으로 벽면(shell)은 20% 정도 차지한다고 가정
    let shell_ratio = 0.2;
    let infill_ratio = (infill_percentage as f64 / 100.0) * (1.0 - shell_ratio);
    let effective_density_ratio = shell_ratio + infill_ratio;

    volume_cm3 * density * effective_density_ratio
}

pub fn estimate_print_time(
    volume_cm3: f64,
    quality: &str,
    infill_percentage: i32,
) -> f64 {
    // 기본 출력 속도: 약 10cm³/hour (normal 품질 기준)
    let base_speed = 10.0;

    let quality_time_multiplier = match quality.to_lowercase().as_str() {
        "draft" => 0.7,
        "normal" => 1.0,
        "high" => 1.5,
        "ultra" => 2.0,
        _ => 1.0,
    };

    let infill_time_factor = 0.5 + (infill_percentage as f64 / 200.0);

    (volume_cm3 / base_speed) * quality_time_multiplier * infill_time_factor
}

pub fn calculate_price(
    volume_cm3: f64,
    material: &str,
    quality: &str,
    infill_percentage: i32,
    quantity: i32,
) -> (Decimal, Decimal, f64) {
    let config = PricingConfig::for_material(material);
    let quality_mult = get_quality_multiplier(quality);

    // 재료비 계산
    let weight_grams = calculate_material_weight(volume_cm3, material, infill_percentage);
    let material_cost = Decimal::from_f64_retain(weight_grams)
        .unwrap_or(dec!(0))
        * config.material_cost_per_gram;

    // 시간 계산
    let print_time_hours = estimate_print_time(volume_cm3, quality, infill_percentage);

    // 인건비 + 기계 사용료
    let time_cost = (config.labor_cost_per_hour + config.machine_cost_per_hour)
        * Decimal::from_f64_retain(print_time_hours).unwrap_or(dec!(0));

    // 최소 가격 설정 (5000원)
    let min_price = dec!(5000);

    // 단가 = (재료비 + 시간비용) * 품질배수 * 재질배수
    let mut unit_price = (material_cost + time_cost) * quality_mult * config.quality_multiplier;

    // 최소 가격 적용
    if unit_price < min_price {
        unit_price = min_price;
    }

    // 수량 할인 (5개 이상: 5%, 10개 이상: 10%)
    let quantity_discount = if quantity >= 10 {
        dec!(0.9)
    } else if quantity >= 5 {
        dec!(0.95)
    } else {
        dec!(1.0)
    };

    unit_price = unit_price * quantity_discount;

    // 100원 단위로 반올림
    unit_price = (unit_price / dec!(100)).round() * dec!(100);

    let total_price = unit_price * Decimal::from(quantity);

    (unit_price, total_price, print_time_hours)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_calculation() {
        let (unit_price, total_price, time) = calculate_price(
            100.0,   // 100 cm³
            "pla",
            "normal",
            20,
            1
        );

        assert!(unit_price > dec!(0));
        assert_eq!(total_price, unit_price);
        assert!(time > 0.0);
    }

    #[test]
    fn test_quantity_discount() {
        let (unit_1, total_1, _) = calculate_price(100.0, "pla", "normal", 20, 1);
        let (unit_10, total_10, _) = calculate_price(100.0, "pla", "normal", 20, 10);

        // 10개 구매시 단가가 더 저렴해야 함
        assert!(unit_10 < unit_1);
    }
}
