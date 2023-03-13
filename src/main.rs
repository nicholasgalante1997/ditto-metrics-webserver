use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use crate::models::models_mod::JSONPingHealthCheck;

mod models;

#[get("/")]
async fn health_check() -> impl Responder {
    let response_body = JSONPingHealthCheck { token: String::from("health_check") };
    let json_ping_response = serde_json::to_string(&response_body).unwrap();
    HttpResponse::Ok().body(format!("{}", json_ping_response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
    })
    .bind(("127.0.0.1", 7100))?
    .run()
    .await
}