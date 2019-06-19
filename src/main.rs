use crate::actor::{telegram_receiver::TbReceiverActor, telegram_sender::TbSenderActor};
use crate::clients::create_telegram_client;
use crate::controllers::push_event::register_push_event;

use actix::prelude::*;
use actix::Addr;
use actix_rt::System;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;

mod actor;
mod clients;
mod controllers;
mod errors;
mod model;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    let sys = System::new("Service Bot");
    let env_port: u16 = env::var("ENV_PORT")
        .expect("ENV_PORT, not set")
        .parse()
        .expect("Invalid conversion from String to u16");

    let telegram_client = create_telegram_client();

    let tb_sender = TbSenderActor(telegram_client.clone());
    let tb_sender_addr: Addr<TbSenderActor> = tb_sender.start();
    let tb_receiver = TbReceiverActor(telegram_client.clone());
    let tb_receiver_addr: Addr<TbReceiverActor> = tb_receiver.start();

    println!("Running server on port {port}", port = env_port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(tb_sender_addr.clone())
            .data(tb_receiver_addr.clone())
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
