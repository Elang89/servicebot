use crate::actor::telegram::{PushEventMsg, TbActor};
use crate::model::PushEvent;

use actix::Addr;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::Future;

pub fn register_push_event(
    push_event: web::Json<PushEvent>,
    tb: web::Data<Addr<TbActor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("Push Event: {:?}", push_event);
    let msg = PushEventMsg {
        reference: push_event.reference.clone(),
    };

    tb.send(msg)
        .then(|_| Ok(HttpResponse::Ok().json("Message Sent")))
}
