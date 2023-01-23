use actix_web::{HttpResponse};
use serde::Serialize;

pub fn response_utils_created<T>(response: &T) -> HttpResponse where T: Serialize {
    HttpResponse::Created()
    .content_type("application/json")
    .json(response)
}

pub fn response_utils_ok<T>(response: &T)->HttpResponse where T: Serialize {
    HttpResponse::Ok()
              .content_type("application/json")
              .json(response)
}