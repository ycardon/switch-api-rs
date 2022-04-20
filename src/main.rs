use actix_web::{get, post, web, middleware, App, HttpServer, HttpResponse, Responder, Result};
use serde::Serialize;
use std::{env, io};

// --- HTTP SERVER ---

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_cpu)
            .service(get_power)
            .service(get_display)
            .service(post_display)
    })
    .bind("0.0.0.0:3030")?
    .run()
    .await
}

// --- CPU ---

#[derive(Serialize)]
#[allow(non_snake_case)]
struct CpuResponse {
    isOnAC: bool,
    isOnBattery: bool,
    isCharged: bool,
    chargingStatus: String,
    chargePercent: u16,
    remainingChargeTime: String,
    message: String,
}

#[get("/cpu")]
async fn get_cpu() -> Result<impl Responder> {
    let cpu = CpuResponse {
        isOnAC: true,
        isOnBattery: false,
        isCharged: false,
        chargingStatus: String::from("test"),
        chargePercent: 50,
        remainingChargeTime: String::from("test"),
        message: String::from("test"),
    };
    Ok(web::Json(cpu))
}

// --- POWER ---

#[get("/power")]
async fn get_power() -> impl Responder {
    HttpResponse::Ok().body("GET /power")
}

// --- DISPLAY ---

#[get("/display")]
async fn get_display() -> impl Responder {
    HttpResponse::Ok().body("GET /display")
}

#[post("/display")]
async fn post_display() -> impl Responder {
    HttpResponse::Ok().body("POST /display")
}
