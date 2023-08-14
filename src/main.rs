use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let headers: Vec<String> = req
        .headers()
        .into_iter()
        .map(|h| format!("{}: {}", h.0, h.1.to_str().unwrap()))
        .collect();
    Ok(Response::new(headers.join("\n").into()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::option_env!("PORT").unwrap_or("8080");
    println!("using port: {port}");
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse()?));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
