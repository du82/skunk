use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper;
use std::convert::Infallible;
use tokio::fs;
use std::path::Path;
mod error_handler;

const IP: [u8; 4] = [127, 0, 0, 1];
const PORT: u16 = 3000;
const VERSION: &str = "0.1.0";

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = if req.uri().path() == "/" {
        "/index.html".to_string()
    } else {
        req.uri().path().to_string()
    };

    let path = Path::new("static").join(path.trim_start_matches('/'));

    match fs::read_to_string(&path).await {
        Ok(contents) => Ok(Response::new(Body::from(contents))),
        Err(err) => {
            let (code, message) = match err.kind() {
                std::io::ErrorKind::NotFound => (404, "File not found"),
                std::io::ErrorKind::PermissionDenied => (403, "Permission denied"),
                _ => (500, "Internal server error"),
            };
            error_handler::handle_error(code, message, VERSION).await
        },
    }
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle)) }
    });

    let addr = (IP, PORT).into();

    println!("Server running on http://{}", addr);

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}