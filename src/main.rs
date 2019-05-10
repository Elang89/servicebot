use crate::controllers::push_event::register_push_event;
use crate::db::create_mongo_connection;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson};
use std::env;

mod controllers;
mod db;
mod model;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    let env_port: u16 = env::var("ENV_PORT")
        .expect("ENV_PORT, not set")
        .parse()
        .expect("Invalid conversion from String to u16");

    let mongo_instance = create_mongo_connection();
    let coll = mongo_instance.collection("events");

    let doc = doc! {
        "title": "Jaws",
        "array": [ 1, 2, 3 ],
    };

    match coll.insert_one(doc.clone(), None) {
        Ok(_) => (),
        Err(err) => panic!("Failed to insert to database, {:?}", err),
    };

    println!("Running server on port {port}", port = env_port);

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
