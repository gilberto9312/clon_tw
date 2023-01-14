use actix_web::{get, post, web, Responder, HttpResponse};
use crate::utils::response_utils_created;

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(id: web::Path<String>) -> impl Responder {
    format!("Hello {id}!")
}

#[post("/tweets")]
pub async fn create_tweet() -> HttpResponse {

    let tweet = format!("nuevo tweet");
    response_utils_created(tweet)
    
}