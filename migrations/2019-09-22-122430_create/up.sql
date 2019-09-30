-- Your SQL goes here
CREATE TABLE users (
    userid SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    userupvote INTEGER NOT NULL,
    userdownvote INTEGER NOT NULL,
    userscore FLOAT4 NOT NULL
);

CREATE TABLE memes (
    memeid SERIAL PRIMARY KEY,
    author INTEGER NOT NULL REFERENCES users(userid),
    image VARCHAR NOT NULL,
    --image_data BYTEA NOT NULL,
    upvote INTEGER NOT NULL,
    downvote INTEGER NOT NULL,
    score FLOAT4 NOT NULL,
    heat FLOAT4 NOT NULL,
    last_action TIMESTAMP NOT NULL,
    posted_at TIMESTAMP NOT NULL
);

CREATE TABLE actions (
    memeid INTEGER REFERENCES memes(memeid),
    userid INTEGER REFERENCES users(userid),
    is_upvote BOOLEAN NOT NULL,
    is_active BOOLEAN NOT NULL,
    posted_at TIMESTAMP NOT NULL,
    PRIMARY KEY (memeid, userid)
);

CREATE TABLE tags (
    tagid SERIAL PRIMARY KEY,
    tagname VARCHAR NOT NULL
);

CREATE TABLE meme_tags (
    tagid INTEGER NOT NULL REFERENCES tags(tagid),
    memeid INTEGER NOT NULL REFERENCES memes(memeid),
    PRIMARY KEY (tagid, memeid)
);