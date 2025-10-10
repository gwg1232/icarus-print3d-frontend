-- 3D 프린터 출력 주문 테이블
CREATE TABLE print_orders (
    order_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    -- 파일 정보
    file_name VARCHAR(255) NOT NULL,
    file_path VARCHAR(512) NOT NULL,
    file_size_bytes BIGINT NOT NULL,

    -- 출력 옵션
    material VARCHAR(50) NOT NULL, -- PLA, ABS, PETG, Resin 등
    color VARCHAR(50),
    quality VARCHAR(20) NOT NULL, -- draft, normal, high, ultra
    infill_percentage INTEGER NOT NULL DEFAULT 20 CHECK (infill_percentage >= 0 AND infill_percentage <= 100),

    -- 치수 정보 (mm 단위)
    model_volume_cm3 DECIMAL(10, 2),
    estimated_print_time_hours DECIMAL(10, 2),

    -- 수량 및 가격
    quantity INTEGER NOT NULL DEFAULT 1 CHECK (quantity > 0),
    unit_price DECIMAL(10, 2) NOT NULL,
    total_price DECIMAL(10, 2) NOT NULL,

    -- 배송 정보
    delivery_name VARCHAR(100) NOT NULL,
    delivery_phone VARCHAR(20) NOT NULL,
    delivery_address TEXT NOT NULL,
    delivery_notes TEXT,

    -- 주문 상태
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    -- pending, payment_confirmed, printing, completed, cancelled

    -- 결제 정보
    payment_method VARCHAR(20), -- card, transfer, etc
    payment_status VARCHAR(20) NOT NULL DEFAULT 'unpaid',
    -- unpaid, paid, refunded
    payment_transaction_id VARCHAR(100),
    paid_at TIMESTAMPTZ,

    -- 타임스탬프
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 인덱스 생성
CREATE INDEX idx_print_orders_user_id ON print_orders(user_id);
CREATE INDEX idx_print_orders_status ON print_orders(status);
CREATE INDEX idx_print_orders_payment_status ON print_orders(payment_status);
CREATE INDEX idx_print_orders_created_at ON print_orders(created_at DESC);

-- 업데이트 트리거
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_print_orders_updated_at
    BEFORE UPDATE ON print_orders
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
