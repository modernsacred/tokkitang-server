# modeler-server

## 배포 URL

1. Live: https://ksauqt5f5er2djql3atquzas4e0ofpla.lambda-url.ap-northeast-2.on.aws

## 구성요소

1. 인프라: AWS Lambda + DynamoDB
2. CI/CD: Github Action
3. 서버 환경: Rust, Axum
4. 인증 방식: JWT

## 브랜치 전략

1. master: 배포 브랜치. CI/CD 구성됨.
2. develop: 개발 브랜치

## 로컬 테스트

1. 루트 경로에 `.env` 파일을 작성합니다.
2. `cargo run`을 실행합니다. 8080 포트로 서버가 실행됩니다.
