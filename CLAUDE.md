# Claude Code 규칙

## 프로젝트 규칙

### 1. 정적 홈페이지 원칙
- 모든 페이지는 정적 HTML 파일로 작성
- 서버 사이드 렌더링이나 빌드 프로세스 없이 직접 브라우저에서 실행 가능
- 외부 API 호출은 최소화하고 필요시 HTMX로 처리

### 2. 기술 스택 제한
- **필수 사용**: HTMX, Tailwind CSS
- **사용 금지**: React, Vue, Angular 등 프레임워크
- **최소화**: 순수 JavaScript, 커스텀 CSS

### 3. 파일 구조
```
/
├── index.html          # 메인 페이지
├── *.html             # 기타 정적 페이지
└── assets/            # 이미지, 아이콘 등 정적 자원
```

### 4. HTML 작성 규칙
- 모든 스타일링은 Tailwind 유틸리티 클래스 사용
- 동적 기능은 HTMX 속성으로 구현
- `<script>` 태그 사용 최소화
- `<style>` 태그 사용 금지 (Tailwind로 대체)

### 5. HTMX 사용 지침
- 폼 제출: `hx-post`, `hx-get` 사용
- 동적 콘텐츠 로딩: `hx-trigger`, `hx-swap` 활용
- 페이지 간 네비게이션: `hx-boost` 적용
- 서버 응답은 HTML 조각으로 처리

### 6. Tailwind CSS 사용 지침
- CDN 버전 사용 (빌드 프로세스 없음)
- 인라인 클래스만 사용
- 커스텀 CSS 클래스 생성 금지
- 반응형 디자인: Tailwind 반응형 유틸리티 활용

### 7. JavaScript 사용 제한
- HTMX로 불가능한 기능만 최소한으로 사용
- 인라인 스크립트 금지
- 필요시 단일 `<script>` 태그에 통합
- jQuery, 기타 라이브러리 사용 금지

### 8. 성능 최적화
- 이미지 최적화 및 lazy loading 적용
- CDN 리소스 활용
- 인라인 critical CSS 없음 (Tailwind CDN 사용)

### 9. 코드 작성 예시
```html
<!-- 좋은 예 -->
<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        hx-get="/api/data" 
        hx-target="#result">
    클릭
</button>

<!-- 나쁜 예 -->
<button style="background: blue;" onclick="fetchData()">클릭</button>
```

### 10. 검증 사항
- HTML 유효성 검사 통과
- HTMX 속성 올바른 사용
- Tailwind 클래스 유효성
- JavaScript 코드 최소화 확인