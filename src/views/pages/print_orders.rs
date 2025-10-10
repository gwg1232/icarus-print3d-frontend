use maud::{html, Markup};

use crate::{
    auth::CurrentUser, flash::FlashMessage, handlers::dtos::print_order::PrintOrderDto,
    paths, views,
};

pub fn orders_page(
    current_user: &CurrentUser,
    flash: &Option<FlashMessage>,
    orders: Vec<PrintOrderDto>,
) -> Markup {
    views::layout::base::base_layout(
        current_user,
        flash,
        "내 주문 목록",
        "3D 프린터 출력 주문 관리",
        orders_list(orders),
    )
}

pub fn new_order_page(current_user: &CurrentUser, flash: &Option<FlashMessage>) -> Markup {
    views::layout::base::base_layout(
        current_user,
        flash,
        "새 출력 주문",
        "3D 프린터 출력 주문 생성",
        new_order_form(),
    )
}

fn orders_list(orders: Vec<PrintOrderDto>) -> Markup {
    html! {
        div class="container mx-auto px-4 py-8" {
            div class="flex justify-between items-center mb-6" {
                h1 class="text-3xl font-bold" { "내 주문 목록" }
                a href=(paths::print_orders::new_order::PATH)
                  class="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded-lg" {
                    "새 주문 만들기"
                }
            }

            @if orders.is_empty() {
                div class="bg-gray-50 rounded-lg p-8 text-center" {
                    p class="text-gray-600" { "아직 주문이 없습니다." }
                    p class="text-gray-500 text-sm mt-2" {
                        "3D 모델 파일을 업로드하고 첫 주문을 시작해보세요!"
                    }
                }
            } @else {
                div class="grid gap-4" {
                    @for order in orders {
                        div class="bg-white rounded-lg shadow p-6 hover:shadow-lg transition" {
                            div class="flex justify-between items-start" {
                                div class="flex-1" {
                                    h3 class="font-semibold text-lg mb-2" {
                                        (order.file_name)
                                    }
                                    div class="grid grid-cols-2 gap-4 text-sm" {
                                        div {
                                            span class="text-gray-600" { "재질: " }
                                            span class="font-medium" { (order.material.to_uppercase()) }
                                        }
                                        div {
                                            span class="text-gray-600" { "품질: " }
                                            span class="font-medium" { (order.quality) }
                                        }
                                        div {
                                            span class="text-gray-600" { "수량: " }
                                            span class="font-medium" { (order.quantity) "개" }
                                        }
                                        div {
                                            span class="text-gray-600" { "가격: " }
                                            span class="font-bold text-blue-600" {
                                                "₩" (order.total_price)
                                            }
                                        }
                                    }
                                }
                                div class="ml-4" {
                                    (status_badge(&order.status, &order.payment_status))
                                }
                            }
                            div class="mt-4 pt-4 border-t flex justify-between items-center" {
                                span class="text-sm text-gray-500" {
                                    (order.created_at.format("%Y-%m-%d %H:%M").to_string())
                                }
                                a href=(format!("{}/{}", paths::print_orders::BASE, order.order_id))
                                  class="text-blue-600 hover:text-blue-800 text-sm font-medium" {
                                    "상세 보기 →"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn new_order_form() -> Markup {
    html! {
        div class="container mx-auto px-4 py-8 max-w-3xl" {
            h1 class="text-3xl font-bold mb-8" { "새 출력 주문" }

            form action=(paths::forms::print_order::PATH)
                 method="post"
                 enctype="multipart/form-data"
                 class="bg-white rounded-lg shadow-lg p-8" {

                // 파일 업로드 섹션
                div class="mb-8" {
                    h2 class="text-xl font-semibold mb-4" { "1. CAD 파일 업로드" }
                    div class="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center hover:border-blue-400 transition" {
                        input type="file"
                               name="cad_file"
                               accept=".stl,.obj,.3mf"
                               required
                               class="block w-full text-sm text-gray-500
                                      file:mr-4 file:py-2 file:px-4
                                      file:rounded-full file:border-0
                                      file:text-sm file:font-semibold
                                      file:bg-blue-50 file:text-blue-700
                                      hover:file:bg-blue-100" {}
                        p class="text-sm text-gray-500 mt-2" {
                            "STL, OBJ, 3MF 파일을 지원합니다"
                        }
                    }
                }

                // 모델 정보
                div class="mb-8" {
                    h2 class="text-xl font-semibold mb-4" { "2. 모델 정보" }
                    div class="mb-4" {
                        label class="block text-sm font-medium text-gray-700 mb-2" {
                            "모델 부피 (cm³)"
                        }
                        input type="number"
                               name="model_volume_cm3"
                               step="0.01"
                               value="100"
                               required
                               class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent" {}
                        p class="text-xs text-gray-500 mt-1" {
                            "파일 업로드 후 자동 계산됩니다"
                        }
                    }
                }

                // 출력 옵션
                div class="mb-8" {
                    h2 class="text-xl font-semibold mb-4" { "3. 출력 옵션" }

                    div class="grid grid-cols-1 md:grid-cols-2 gap-4" {
                        div {
                            label class="block text-sm font-medium text-gray-700 mb-2" {
                                "재질"
                            }
                            select name="material"
                                   required
                                   class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500" {
                                option value="pla" selected { "PLA (일반용)" }
                                option value="abs" { "ABS (내구성)" }
                                option value="petg" { "PETG (강도)" }
                                option value="resin" { "레진 (정밀)" }
                            }
                        }

                        div {
                            label class="block text-sm font-medium text-gray-700 mb-2" {
                                "색상"
                            }
                            input type="text"
                                   name="color"
                                   placeholder="예: 검정, 흰색, 빨강"
                                   class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500" {}
                        }

                        div {
                            label class="block text-sm font-medium text-gray-700 mb-2" {
                                "출력 품질"
                            }
                            select name="quality"
                                   required
                                   class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500" {
                                option value="draft" { "빠름 (Draft)" }
                                option value="normal" selected { "표준 (Normal)" }
                                option value="high" { "고품질 (High)" }
                                option value="ultra" { "최고품질 (Ultra)" }
                            }
                        }

                        div {
                            label class="block text-sm font-medium text-gray-700 mb-2" {
                                "내부 채움률 (%)"
                            }
                            input type="number"
                                   name="infill_percentage"
                                   min="0"
                                   max="100"
                                   value="20"
                                   required
                                   class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500" {}
                        }

                        div {
                            label class="block text-sm font-medium text-gray-700 mb-2" {
                                "수량"
                            }
                            input type="number"
                                   name="quantity"
                                   min="1"
                                   value="1"
                                   required
                                   class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500" {}
                            p class="text-xs text-gray-500 mt-1" {
                                "5개 이상 5% 할인, 10개 이상 10% 할인"
                            }
                        }
                    }
                }

                // 배송 정보
                div class="mb-8" {
                    h2 class="text-xl font-semibold mb-4" { "4. 배송 정보" }

                    div class="space-y-4" {
                        div {
                            label class="block text-sm font-medium text-gray-700 mb-2" {
                                "수령인 이름"
                            }
                            input type="text"
                                   name="delivery_name"
                                   required
                                   class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500" {}
                        }

                        div {
                            label class="block text-sm font-medium text-gray-700 mb-2" {
                                "연락처"
                            }
                            input type="tel"
                                   name="delivery_phone"
                                   placeholder="010-0000-0000"
                                   required
                                   class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500" {}
                        }

                        div {
                            label class="block text-sm font-medium text-gray-700 mb-2" {
                                "배송 주소"
                            }
                            textarea name="delivery_address"
                                     rows="3"
                                     required
                                     class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500" {}
                        }

                        div {
                            label class="block text-sm font-medium text-gray-700 mb-2" {
                                "배송 메모 (선택)"
                            }
                            textarea name="delivery_notes"
                                     rows="2"
                                     placeholder="배송 시 요청사항이 있으시면 입력해주세요"
                                     class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500" {}
                        }
                    }
                }

                // 제출 버튼
                div class="flex justify-end space-x-4" {
                    a href=(paths::print_orders::BASE)
                      class="px-6 py-3 border border-gray-300 rounded-lg hover:bg-gray-50" {
                        "취소"
                    }
                    button type="submit"
                           class="px-8 py-3 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-lg transition" {
                        "주문하기"
                    }
                }
            }

            // 가격 안내
            div class="mt-8 bg-blue-50 rounded-lg p-6" {
                h3 class="font-semibold mb-2" { "💡 가격 안내" }
                ul class="text-sm text-gray-700 space-y-1" {
                    li { "• 재질에 따라 가격이 달라집니다 (PLA < ABS < PETG < 레진)" }
                    li { "• 품질이 높을수록 출력 시간이 길고 가격이 올라갑니다" }
                    li { "• 내부 채움률이 높을수록 재료비가 증가합니다" }
                    li { "• 최소 주문 금액: ₩5,000" }
                }
            }
        }
    }
}

fn status_badge(status: &str, payment_status: &str) -> Markup {
    let (badge_color, badge_text) = match (status, payment_status) {
        (_, "unpaid") => ("bg-yellow-100 text-yellow-800", "결제 대기"),
        ("pending", "paid") => ("bg-blue-100 text-blue-800", "주문 확인 중"),
        ("payment_confirmed", _) => ("bg-purple-100 text-purple-800", "출력 대기"),
        ("printing", _) => ("bg-orange-100 text-orange-800", "출력 중"),
        ("completed", _) => ("bg-green-100 text-green-800", "완료"),
        ("cancelled", _) => ("bg-red-100 text-red-800", "취소됨"),
        _ => ("bg-gray-100 text-gray-800", status),
    };

    html! {
        span class=(format!("px-3 py-1 rounded-full text-sm font-medium {}", badge_color)) {
            (badge_text)
        }
    }
}
