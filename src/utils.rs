use actix_web::{HttpResponse};
use serde::Serialize;

pub fn response_utils_created<T>(response: &T) -> HttpResponse where T: Serialize {
    HttpResponse::Created()
    .content_type("application/json")
    .json(response)
}