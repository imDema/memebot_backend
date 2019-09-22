-- Your SQL goes here
CREATE TABLE users (
    userid SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    userupvote INTEGER,
    userdownvote INTEGER
);

CREATE TABLE memes (
    memeid SERIAL PRIMARY KEY,
    author INTEGER NOT NULL REFERENCES users(userid),
    image VARCHAR NOT NULL,
    image_data BYTEA NOT NULL,
    upvote INTEGER NOT NULL,
    downvte INTEGER NOT NULL,
    date TIMESTAMP NOT NULL,
    heat FLOAT8
);