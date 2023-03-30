use crate::models::models_factory::AvailabilityMetric;
use crate::models::models_factory::CountMetric;
use crate::models::models_factory::TimingMetric;
use crate::models::models_factory::WebServiceHealthCheck;
use crate::clients::postgres_client::PostgresClientService;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod models;
mod clients;

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
    // Raw Metric
    println!("{}", metric.build_metric_string());

    // Database Writes
    let postgres_client_service = PostgresClientService::get_postgres_client_instance();
    let mut db_client_result = postgres_client_service.connect().expect("Unable to connect to database with specified env vars.");

    let create_availability_metric_table_query = "CREATE TABLE IF NOT EXISTS availability_metrics (
        m_id SERIAL PRIMARY KEY,
        service_name TEXT, 
        function_name TEXT, 
        availability INTEGER, 
        timestamp TIMESTAMP
    )";
    db_client_result.execute(create_availability_metric_table_query, &[]).expect("CREATE_TABLE_EXCEPTION / Unable to create table `availability_metrics`");
    
    let write_metric_to_table_insert = "INSERT INTO availability_metrics (
        service_name, 
        function_name, 
        availability, 
        timestamp
    ) VALUES (
        $1, 
        $2, 
        $3, 
        $4
    )";
    db_client_result.execute(write_metric_to_table_insert, &[&metric.value.service_name, &metric.value.fn_name, &metric.value.availability.to_string(), &metric.value.timestamp]).expect("INSERT_ROW_EXCEPTION / Unable to insert row in table `availability_metrics` for metric with id ");
    
    // Response
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

    // load env variables
    dotenv().expect(
        "Unable to parse environment variables. Check to see environment variables are being set/loaded correctly."
    );

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
