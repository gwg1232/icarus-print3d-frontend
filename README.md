# icarus-print3d

3D 프린팅 서비스 정적 웹사이트

## 기술 스택

- **HTMX** - 동적 인터랙션
- **Tailwind CSS** (CDN) - 스타일링
- **Lucide Icons** - 아이콘
- **Vanilla JavaScript** - 파일 업로드 및 계산기

## 주요 기능

- 📁 드래그 앤 드롭 파일 업로드
- 💰 실시간 견적 계산
- 📱 반응형 디자인
- 🚀 HTMX 기반 동적 콘텐츠
- 💳 간단 가격 계산기
- 📧 문의 폼

## 실행 방법

```bash
# Python 서버로 실행
python3 -m http.server 8000

# 또는 Node.js 서버
npx serve .
```

브라우저에서 `http://localhost:8000` 접속

## 파일 구조

```
├── index.html    # 메인 페이지
├── api.html      # HTMX 응답 템플릿
├── server.html   # Mock 서버 로직
└── README.md     # 문서
```

## 특징

- 별도의 백엔드 서버 없이 정적 파일만으로 구동
- HTMX를 활용한 SPA 같은 사용자 경험
- Tailwind CSS CDN으로 빠른 스타일링
- Mock 서버 로직으로 데모 기능 구현