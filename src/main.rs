use actix_web::{App, HttpServer};

mod utils;
mod tweets;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(tweets::get_tweet_by_id)
        .service(tweets::create_tweet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}