use actix_web::{get, post, HttpResponse, Responder};

#[get("/display")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("GET /display")
}

#[post("/display")]
async fn post() -> impl Responder {
    HttpResponse::Ok().body("POST /display")
}
