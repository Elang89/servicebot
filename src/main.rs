use crate::db::create_mongo_connection;
use crate::controllers::push_event::register_push_event;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

mod controllers;
mod db;
mod model;

fn main() -> std::io::Result<()> {
    println!("Running server at port 5000");

    let mongo = create_mongo_connection("localhost", 27017)

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/api").service(
                web::scope("/v1")
                    .service(web::resource("/").to(|| HttpResponse::Ok().json("Test Works")))
                    .service(web::resource("/event").route(web::post().to(register_push_event))),
            ),
        )
    })
    .bind("0.0.0.0:5000")?
    .run()
}
