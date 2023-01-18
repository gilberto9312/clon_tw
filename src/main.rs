use std::env;
use dotenvy::dotenv;
use actix_web::{App, HttpServer};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
mod utils;
mod tweets;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("it can't connect to database");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("it can't build pool");
    HttpServer::new( move || {
        App::new()
        .app_data(pool.clone())
        .service(tweets::get_tweet_by_id)
        .service(tweets::create_tweet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}