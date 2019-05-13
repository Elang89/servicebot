use crate::actor::telegram::TbActor;
use crate::model::PushEvent;

use actix::Addr;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::future::Future;

pub fn register_push_event(
    push_event: web::Json<PushEvent>,
    tb: web::Data<Addr<TbActor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("Push Event: {:?}", push_event);
    let msg = PushEvent {
        reference: push_event.reference.clone(),
    };

    tb.send(msg)
        .from_err()
        .and_then(|response| match response {
           Ok() => Ok(HttpResponse::Ok().json(response)),
           Err() => Ok(HttpResponse::Ok().json(response))
        })
}
