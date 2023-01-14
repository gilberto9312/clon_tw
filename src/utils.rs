
use actix_web::{HttpResponse};

pub fn response_utils_created(response: String) -> HttpResponse{
    HttpResponse::Created()
    .content_type("application/json")
    .json(response)
}

