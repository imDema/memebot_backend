-- Your SQL goes here
CREATE TABLE users {
    userid SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    userupvote INTEGER NOT NULL,
    userdownvote INTEGER NOT NULL,
}

CREATE TABLE memes {
    memeid SERIAL PRIMARY KEY,
    FOREIGN KEY (author) INTEGER REFERENCES users(userid),
    image VARCHAR NOT NULL,
    image_data BLOB NOT NULL,
    upvote INTEGER NOT NULL,
    downvte INTEGER NOT NULL,
    date TIMESTAMP NOT NULL,
    heat FLOAT8 NOT NULL,
}