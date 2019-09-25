use chrono::prelude::*;

use super::schema::users;
use super::schema::memes;
use super::schema::actions;

#[derive(Debug)]
#[derive(Queryable)]
pub struct Meme {
    pub memeid: i32,
    pub author: i32,
    pub image: String,
    pub upvote: i32,
    pub downvote: i32,
    pub posted_at: NaiveDateTime,
}

#[derive(Debug)]
#[derive(Queryable)]
pub struct User {
    pub userid: i32,
    pub username: String,
    pub userupvote: i32,
    pub userdownvote: i32,
}

pub enum ActionKind {
    Upvote,
    Downvote,
}

#[derive(Debug)]
#[derive(Queryable)]
#[derive(Insertable)]
pub struct Action {
    memeid: i32,
    userid: i32,
    is_upvote: bool,
    posted_at: NaiveDateTime,
}

impl Action {
    pub fn new((memeid, userid) : (i32, i32), action: ActionKind) -> Action {
        Action {
            memeid,
            userid,
            is_upvote: match action {
                ActionKind::Upvote => true,
                ActionKind::Downvote => false,
            },
            posted_at: Local::now().naive_local(),
        }
    }
    ///Returns (memeid, userid) tuple for this action
    pub fn getKey(&self) -> (i32,i32) {
        (self.memeid, self.userid)
    }
    //Get timestamp of when the action was exexuted
    pub fn getTimestamp(&self) -> NaiveDateTime {
        self.posted_at
    }
    ///Get the type of action
    pub fn getActionKind(&self) -> ActionKind {
        match self.is_upvote {
            true => ActionKind::Upvote,
            false => ActionKind::Downvote,
        }
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub userupvote: i32,
    pub userdownvote: i32,
}

#[derive(Insertable)]
#[table_name="memes"]
pub struct NewMeme {
    author: i32,
    image: String,
    upvote: i32,
    downvote: i32,
    posted_at: NaiveDateTime,
}

impl NewMeme {
    pub fn new((img, author): (&str, i32)) -> NewMeme {
        NewMeme {
            author,
            image: img.to_owned(),
            upvote: 0,
            downvote: 0,
            posted_at: Local::now().naive_local(),
        }
    }
}