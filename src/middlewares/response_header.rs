use axum::{http::Request, middleware::Next, response::Response};

pub async fn response_header_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let origin = req.headers().get("Origin").map(|o| o.to_owned());

    let mut response = next.run(req).await;

    if let Some(origin) = origin {
        let response_headers = response.headers_mut();

        if !response_headers.contains_key("Access-Control-Allow-Origin") {
            response_headers.insert("Access-Control-Allow-Origin", origin);
        }
    }

    response
}
