use reqwest::header;

pub fn default_header() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();

    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static(r#"application/json"#),
    );
    headers.insert(
        "Accept",
        header::HeaderValue::from_static(r#"application/json"#),
    );

    headers
}
