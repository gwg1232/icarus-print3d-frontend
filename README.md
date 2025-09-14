# ICARUS 3D 프린팅 서비스 프론트엔드

초고강도 3D 프린팅 서비스를 위한 정적 웹사이트입니다.

## 기술 스택
- HTMX - 동적 상호작용
- Tailwind CSS (CDN) - 스타일링
- 순수 HTML - 마크업

## 프로젝트 구조
```
/
├── index.html          # 메인 랜딩 페이지
├── CLAUDE.md          # Claude Code 개발 규칙
└── README.md          # 프로젝트 문서
```

## 백엔드 API 요구사항

### 1. 파일 검증 및 분석 API (즉시 견적용)
**Endpoint:** `POST /api/file/validate`

**Request:** multipart/form-data
- `file`: STL/OBJ/STEP/3MF 파일

**Response (HTMX 부분 HTML):**
```html
<div id="file-info" class="mt-4 bg-white rounded-xl p-6 shadow-lg">
    <h3 class="font-bold text-lg mb-4 text-[#092C5E]">파일 정보</h3>
    <div class="space-y-2 text-sm">
        <div class="flex justify-between">
            <span class="text-gray-600">파일명:</span>
            <span class="font-medium">model.stl</span>
        </div>
        <div class="flex justify-between">
            <span class="text-gray-600">크기:</span>
            <span class="font-medium">2.4MB</span>
        </div>
        <div class="flex justify-between">
            <span class="text-gray-600">치수:</span>
            <span class="font-medium">120 x 80 x 45 mm</span>
        </div>
        <div class="flex justify-between">
            <span class="text-gray-600">부피:</span>
            <span class="font-medium">125.3 cm³</span>
        </div>
    </div>
    <input type="hidden" id="model-volume" value="125300">
</div>
```

### 2. 실시간 가격 계산 API (HTMX)
**Endpoint:** `POST /api/price/calculate`

**Request:** Form data
- `file`: 파일 참조
- `material`: onyx | fiberglass | carbon
- `quantity`: number
- `layer-height`: 0.1 | 0.15 | 0.2
- `infill`: 20-100

**Response (HTMX 부분 HTML):**
```html
<div id="price-result" class="bg-gradient-to-br from-[#092C5E] to-[#4980C1] rounded-xl p-6 text-white">
    <h3 class="font-bold text-xl mb-4">예상 견적</h3>
    <div class="space-y-3">
        <div class="flex justify-between">
            <span>재료비</span>
            <span>₩45,000</span>
        </div>
        <div class="flex justify-between">
            <span>출력 시간</span>
            <span>8시간</span>
        </div>
        <div class="flex justify-between">
            <span>인건비</span>
            <span>₩25,000</span>
        </div>
        <div class="border-t border-white/30 pt-3 mt-3">
            <div class="flex justify-between text-xl font-bold">
                <span>총 견적</span>
                <span>₩70,000</span>
            </div>
        </div>
    </div>
    <button class="w-full bg-[#F28347] text-white py-3 rounded-lg mt-6 font-bold hover:bg-[#D94925] transition"
            hx-post="/api/quote"
            hx-include="#file-upload, input[name='material'], #quantity, #layer-height, #infill">
        정식 견적 요청
    </button>
</div>
```

### 3. 견적 요청 API
**Endpoint:** `POST /api/quote`

**Request Body:**
```json
{
  "name": "string",
  "email": "string",
  "phone": "string",
  "company": "string (optional)",
  "material": "onyx | fiberglass | carbon_fiber",
  "quantity": "number",
  "file": "file (STL format)",
  "message": "string (optional)"
}
```

**Response:**
```json
{
  "success": true,
  "quote_id": "string",
  "estimated_price": {
    "min": "number",
    "max": "number",
    "currency": "KRW"
  },
  "estimated_time": {
    "days": "number"
  },
  "message": "견적이 성공적으로 접수되었습니다."
}
```

### 4. 견적 폼 모달 API (HTMX)
**Endpoint:** `GET /api/quote-form`

**Response (HTMX HTML 모달):**
Returns a modal dialog HTML for quote request form

### 5. 상담 폼 모달 API (HTMX)
**Endpoint:** `GET /api/consultation-form`

**Response (HTMX HTML 모달):**
Returns a modal dialog HTML for consultation request form

### 6. 샘플 폼 모달 API (HTMX)
**Endpoint:** `GET /api/sample-form`

**Response (HTMX HTML 모달):**
Returns a modal dialog HTML for sample request form

### 7. 기술 상담 신청 API
**Endpoint:** `POST /api/consultation`

**Request Body:**
```json
{
  "name": "string",
  "email": "string",
  "phone": "string",
  "company": "string (optional)",
  "project_type": "prototype | production | replacement",
  "current_method": "cnc | injection | traditional_3d | other",
  "requirements": "string",
  "preferred_date": "string (ISO 8601)"
}
```

**Response:**
```json
{
  "success": true,
  "consultation_id": "string",
  "message": "상담 신청이 접수되었습니다. 24시간 내 연락드리겠습니다."
}
```

### 8. 샘플 요청 API
**Endpoint:** `POST /api/sample-request`

**Request Body:**
```json
{
  "name": "string",
  "email": "string",
  "phone": "string",
  "company": "string",
  "address": {
    "street": "string",
    "city": "string",
    "postal_code": "string"
  },
  "material": "onyx | fiberglass | carbon_fiber",
  "application": "string",
  "message": "string (optional)"
}
```

**Response:**
```json
{
  "success": true,
  "request_id": "string",
  "estimated_delivery": "string (ISO 8601)",
  "message": "샘플 요청이 접수되었습니다."
}
```

### 9. 기술 자료 다운로드 API
**Endpoint:** `GET /api/download/technical-docs`

**Query Parameters:**
- `type`: `material_guide | technical_specs | case_studies`

**Response:**
```json
{
  "success": true,
  "download_url": "string",
  "file_name": "string",
  "file_size": "number (bytes)",
  "expires_at": "string (ISO 8601)"
}
```

### 10. 뉴스레터 구독 API
**Endpoint:** `POST /api/newsletter/subscribe`

**Request Body:**
```json
{
  "email": "string",
  "interests": ["technology", "case_studies", "materials", "pricing"]
}
```

**Response:**
```json
{
  "success": true,
  "message": "뉴스레터 구독이 완료되었습니다."
}
```

### 11. 파일 업로드 (STL 검증) API - JSON 버전
**Endpoint:** `POST /api/file/validate`

**Request Body (multipart/form-data):**
- `file`: STL 파일

**Response:**
```json
{
  "success": true,
  "file_id": "string",
  "analysis": {
    "volume": "number (mm³)",
    "dimensions": {
      "x": "number (mm)",
      "y": "number (mm)",
      "z": "number (mm)"
    },
    "printable": true,
    "estimated_material": "number (g)",
    "warnings": ["string"]
  }
}
```

### 12. 실시간 가격 계산 API - JSON 버전
**Endpoint:** `POST /api/price/calculate`

**Request Body:**
```json
{
  "material": "onyx | fiberglass | carbon_fiber",
  "volume": "number (mm³)",
  "quantity": "number",
  "infill_percentage": "number (0-100)",
  "layer_height": "0.1 | 0.15 | 0.2"
}
```

**Response:**
```json
{
  "success": true,
  "pricing": {
    "material_cost": "number",
    "printing_time": "number (hours)",
    "labor_cost": "number",
    "total": "number",
    "currency": "KRW"
  },
  "breakdown": {
    "material_weight": "number (g)",
    "support_material": "number (g)",
    "machine_time": "number (hours)"
  }
}
```

## CORS 설정 요구사항
백엔드는 다음 CORS 헤더를 지원해야 합니다:
- `Access-Control-Allow-Origin: *` (프로덕션에서는 특정 도메인으로 제한)
- `Access-Control-Allow-Methods: GET, POST, OPTIONS`
- `Access-Control-Allow-Headers: Content-Type`

## 에러 응답 형식
모든 API는 에러 발생 시 다음 형식으로 응답해야 합니다:
```json
{
  "success": false,
  "error": {
    "code": "string",
    "message": "string",
    "details": {}
  }
}
```

## 인증
현재 단계에서는 인증이 필요하지 않으나, 향후 다음 기능 구현 시 JWT 기반 인증 추가 예정:
- 마이페이지
- 주문 추적
- 견적 히스토리

## 환경 변수
백엔드에서 관리해야 할 환경 변수:
- `API_BASE_URL` - API 서버 주소
- `FILE_UPLOAD_MAX_SIZE` - 최대 업로드 파일 크기
- `SMTP_CONFIG` - 이메일 발송 설정
- `STORAGE_PATH` - 업로드 파일 저장 경로

## 배포 시 고려사항
- 정적 파일은 CDN을 통해 서빙
- API 서버는 별도 도메인 또는 서브도메인에 배포
- HTTPS 필수 적용
- Rate limiting 적용 (분당 60회 요청 제한 권장)