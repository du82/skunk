use hyper::{Body, Response, StatusCode};
use std::convert::Infallible;

pub fn format_error_as_html(code: u16, message: &str, version: &str) -> String {
    format!("
    <html>
      <body>
        <h1>Error {}</h1>
        <p>{}</p>
        <hr>
        <p>DrunkSkunkWebServer {} - Proudly written in Rust lang</p>
      </body>
    </html>", code, message, version)
}

pub async fn handle_error(code: u16, message: &str, version: &str) -> Result<Response<Body>, Infallible> {
    let error_content = format_error_as_html(code, message, version);
    let mut response = Response::new(Body::from(error_content));
    *response.status_mut() = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    Ok(response)
}