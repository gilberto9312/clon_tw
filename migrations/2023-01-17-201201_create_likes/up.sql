-- Your SQL goes here
CREATE TABLE IF NOT EXISTS likes
(
    id UUID primary key not null,
    create_at timestamp default now() not null,
    tweet_id UUID not null REFERENCES tweets (id)
)