use crate::actor::telegram::TbActor;
use crate::clients::create_telegram_client;
use crate::controllers::push_event::register_push_event;
use crate::db::create_mongo_connection;

use actix::prelude::*;
use actix::Addr;
use actix_rt::System;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use mongodb::db::ThreadedDatabase;
use std::env;

mod actor;
mod clients;
mod controllers;
mod db;
mod errors;
mod model;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    let sys = System::new("Service Bot");
    let env_port: u16 = env::var("ENV_PORT")
        .expect("ENV_PORT, not set")
        .parse()
        .expect("Invalid conversion from String to u16");
    let mongo_instance = create_mongo_connection();
    let telegram_client = create_telegram_client();
    let tb_actor = TbActor::new(telegram_client);
    let tb_addr: Addr<TbActor> = tb_actor.start();

    println!("Running server on port {port}", port = env_port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(tb_addr.clone())
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .service(web::resource("/").to(|| HttpResponse::Ok().json("Test Works")))
                        .service(
                            web::resource("/event")
                                .route(web::post().to_async(register_push_event)),
                        ),
                ),
            )
    })
    .bind("0.0.0.0:5000")?
    .start();

    sys.run()
}
