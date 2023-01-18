
#[macro_use]
extern crate diesel;
use std::env;

use dotenvy::dotenv;
use actix_web::{middleware, web, App, HttpServer};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
mod utils;
mod tweets;
mod schema;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let database_url = env::var("DATABASE_URL").expect("it can't connect to database");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("it can't build pool");
    HttpServer::new( move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(middleware::Logger::default())
        .service(tweets::get_tweet_by_id)
        .service(tweets::create_tweet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}