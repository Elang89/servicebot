use crate::model::PushEvent;

use actix_web::{web, HttpResponse};


pub fn register_push_event(push_event: web::Json<PushEvent>) -> HttpResponse {
    HttpResponse::Ok().json(push_event.0)
}