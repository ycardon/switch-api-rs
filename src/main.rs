#[macro_use]
extern crate lazy_static;

use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder, Result};
use std::{env, io};

mod display;
mod power;

/// a REST API that allows get information on MacBook CPU, power and display status
#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_power)
            .service(get_cpu)
            .service(get_display)
            .service(post_display)
    })
    .bind("0.0.0.0:3030")?
    .run()
    .await
}

#[get("/power")]
async fn get_power() -> Result<impl Responder> {
    Ok(web::Json(power::get()))
}

#[get("/cpu")]
async fn get_cpu() -> impl Responder {
    HttpResponse::Ok().body("GET /cpu")
}

#[get("/display")]
async fn get_display() -> impl Responder {
    HttpResponse::Ok().body(if display::is_on() { "ON" } else { "OFF" })
}

#[post("/display")]
async fn post_display(req_body: String) -> impl Responder {
    if "ON".eq(&req_body) {
        display::on()
    } else {
        display::off()
    }
    HttpResponse::Ok()
}
