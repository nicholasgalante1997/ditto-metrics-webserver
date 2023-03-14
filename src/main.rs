use crate::models::models_factory::AvailabilityMetric;
use crate::models::models_factory::CountMetric;
use crate::models::models_factory::TimingMetric;
use crate::models::models_factory::WebServiceHealthCheck;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod models;

#[get("/")]
async fn health_check_handler() -> impl Responder {
    let response_body = WebServiceHealthCheck {
        token: String::from("health_check"),
    };
    let json_ping_response = serde_json::to_string(&response_body).unwrap();
    HttpResponse::Ok().body(format!("{}", json_ping_response))
}

#[post("/availability")]
async fn availability_handler(metric: web::Json<AvailabilityMetric>) -> impl Responder {
    println!("{}", metric.build_metric_string());
    HttpResponse::Ok()
}

#[post("/timing")]
async fn timing_handler(metric: web::Json<TimingMetric>) -> impl Responder {
    println!("{}", metric.build_metric_string());
    HttpResponse::Ok()
}

#[post("/count")]
async fn count_handler(metric: web::Json<CountMetric>) -> impl Responder {
    println!("{}", metric.build_metric_string());
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check_handler)
            .service(availability_handler)
            .service(timing_handler)
            .service(count_handler)
    })
    .bind(("127.0.0.1", 7100))?
    .run()
    .await
}
