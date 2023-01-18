-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tweets
(
    id UUID primary key not null,
    create_at timestamp default now() not null,
    message text not null
);