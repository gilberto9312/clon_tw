use actix_web::{get, post, web::{ Data, Path }, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::{ r2d2::{ Pool, ConnectionManager }, PgConnection, QueryDsl };
use uuid::Uuid;
use super::schema::tweets;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Serialize, Deserialize};
use crate::utils::{ response_utils_created,response_utils_ok };


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
pub async fn get_tweet_by_id(path: Path<String>, pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    use crate::schema::tweets::dsl::*;
    let mut conn = pool.get().expect("it can't connect database");
    let t_id = &path.into_inner();
    let t_id_uuid: Uuid;
    match Uuid::parse_str(t_id) {
        Ok(t_id_uuid_result) => {
            // Do something with the Uuid
            t_id_uuid = t_id_uuid_result;
            println!("{:?}",t_id_uuid_result);
        }
        Err(e) => {
            println!("tweet id invalid, error: {:?}", e);
            return HttpResponse::NotFound().await.unwrap();
        }
    }
    
    let result = tweets.find(t_id_uuid).load::<Tweet>(&mut conn);
    match result {
        Ok(rows) => match rows.first() {
            Some(tweet) => {
                response_utils_ok(tweet)
            },
            _ => {
              HttpResponse::NotFound().await.unwrap()
            },
        },
        Err(_) => HttpResponse::NotFound().await.unwrap(),
    }

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
