use actix_web::{get, post, web::{self, Data}, Responder, HttpResponse};

use chrono::{NaiveDateTime, Utc};
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use uuid::Uuid;
use super::schema::tweets;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Serialize, Deserialize};

use crate::utils::response_utils_created;


#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
struct Tweet {
    id: Uuid,
    create_at: NaiveDateTime,
    message: String
}

impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            create_at: Utc::now().naive_utc(),
            message
        }
    }
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(id: web::Path<String>) -> impl Responder {
    format!("Hello {id}!")
}

#[post("/tweets")]
pub async fn create_tweet(req_body: String, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {

    let new_tweet = Tweet::new(req_body);
    let mut conn = pool.get().expect("it can't connect database");
    diesel::insert_into(tweets::table)
    .values(&new_tweet)
    .execute(&mut conn).expect("fail to insert");


    response_utils_created(&new_tweet)
    
}
