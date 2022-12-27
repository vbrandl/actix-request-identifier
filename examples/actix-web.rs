use actix_request_identifier::{RequestId, RequestIdentifier};
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn show_request_id(id: RequestId) -> impl Responder {
    format!("{}", id.as_str())
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let http_server = HttpServer::new(|| {
        App::new()
            .service(show_request_id)
            .wrap(RequestIdentifier::with_uuid())
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    println!("example server listening on 127.0.0.1:8080");
    println!("try `curl -v http://127.0.0.1:8080/`");
    println!("try `curl -v -H 'x-request-id: 12345' http://127.0.0.1:8080/`");
    http_server.await
}
