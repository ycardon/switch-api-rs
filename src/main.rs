use actix_web::{middleware, App, HttpServer};
use std::{env, io};

mod display;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(display::get)
            .service(display::post)
    })
    .bind("0.0.0.0:3030")?
    .run()
    .await
}

// https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/