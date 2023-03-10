use axum::{http::Request, middleware::Next, response::Response};

pub async fn response_header_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(req).await;

    let headers = response.headers_mut();

    headers.insert(
        "Access-Control-Allow-Origin",
        "https://tokkitang.com".parse().unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Origin",
        "http://localhost:5173".parse().unwrap(),
    );

    response
}
