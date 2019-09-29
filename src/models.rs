use chrono::prelude::*;

use super::schema::*;
use super::rating;

#[derive(Debug)]
#[derive(Queryable)]
pub struct Meme {
    pub memeid: i32,
    pub authorid: i32,
    pub image: String,
    pub upvote: i32,
    pub downvote: i32,
    pub score: f32,
    pub posted_at: NaiveDateTime,
}

#[derive(Debug)]
#[derive(Queryable)]
pub struct User {
    pub userid: i32,
    pub username: String,
    pub userupvote: i32,
    pub userdownvote: i32,
    pub userscore: f32,
}

#[derive(Debug)]
#[derive(Queryable)]
#[derive(Insertable)]
pub struct Action {
    memeid: i32,
    userid: i32,
    is_upvote: bool,
    is_active: bool,
    posted_at: NaiveDateTime,
}

#[derive(Debug)]
#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="meme_tags"]
pub struct MemeTag {
    memeid: i32,
    tagid: i32,
}

#[derive(PartialEq)]
pub enum ActionKind {
    Upvote,
    Downvote,
}

impl ActionKind {
    pub fn is_upvote(&self) -> bool {
        match self {
            ActionKind::Upvote => true,
            ActionKind::Downvote => false,
        }
    } 
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
            is_active: true,
            posted_at: Local::now().naive_local(),
        }
    }
    ///Returns (memeid, userid) tuple for this action
    pub fn get_key(&self) -> (i32,i32) {
        (self.memeid, self.userid)
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    //Get timestamp of when the action was exexuted
    pub fn get_timestamp(&self) -> NaiveDateTime {
        self.posted_at
    }
    ///Get the type of action
    pub fn get_action_kind(&self) -> ActionKind {
        match self.is_upvote {
            true => ActionKind::Upvote,
            false => ActionKind::Downvote,
        }
    }
}

impl MemeTag {
    pub fn new(memeid: i32, tagid: i32) -> MemeTag {
        MemeTag {memeid, tagid}
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    username: String,
    userupvote: i32,
    userdownvote: i32,
    userscore: f32,
}

impl NewUser {
    pub fn new(username: &str) -> NewUser {
        NewUser {
            username: username.to_owned(),
            userupvote: 0,
            userdownvote: 0,
            userscore: rating::score(0, 0),
        }
    }
}

#[derive(Insertable)]
#[table_name="memes"]
pub struct NewMeme {
    author: i32,
    image: String,
    upvote: i32,
    downvote: i32,
    score: f32,
    posted_at: NaiveDateTime,
}

impl NewMeme {
    pub fn new(img: &str, author: i32) -> NewMeme {
        NewMeme {
            author,
            image: img.to_owned(),
            upvote: 0,
            downvote: 0,
            score: rating::score(0, 0),
            posted_at: Local::now().naive_local(),
        }
    }
}