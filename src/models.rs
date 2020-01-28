use chrono::prelude::*;

use super::schema::*;

use serde_derive::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Debug)]
pub struct Meme {
    pub memeid: i32,
    pub authorid: i32,
    pub image: String,
    pub upvote: i32,
    pub downvote: i32,
    pub score: f32,
    pub heat: f32,
    pub last_action: NaiveDateTime,
    pub posted_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Debug)]
pub struct User {
    pub userid: i32,
    pub username: String,
    pub userupvote: i32,
    pub userdownvote: i32,
    pub userscore: f32,
}

#[derive(Insertable, Queryable, Serialize, Debug)]
pub struct Action {
    memeid: i32,
    userid: i32,
    is_upvote: bool,
    is_active: bool,
    posted_at: NaiveDateTime,
}

#[derive(Queryable,Serialize)]
pub struct Tag {
    pub tagid: i32,
    pub tagname: String,
}


#[derive(Debug)]
#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="meme_tags"]
pub struct MemeTag {
    tagid: i32,
    memeid: i32,
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
    pub fn action_kind(&self) -> ActionKind {
        if self.is_upvote {
            ActionKind::Upvote
        } else {
            ActionKind::Downvote
        }
    }
}

impl MemeTag {
    pub fn new(memeid: i32, tagid: i32) -> MemeTag {
        MemeTag {memeid, tagid}
    }
}

// pub struct NewAction {
//     action_key: (i32, i32),
//     action: ActionKind,
// }

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    userid: i32,
    username: String,
}

impl NewUser {
    pub fn new(userid: i32, username: &str) -> NewUser {
        NewUser {
            userid,
            username: username.to_owned(),
        }
    }
}

#[derive(Insertable, Deserialize)]
#[table_name="memes"]
pub struct NewMeme {
    author: i32,
    image: String,
}

impl NewMeme {
    pub fn new(img: &str, author: i32) -> NewMeme {
        NewMeme {
            author,
            image: img.to_owned(),
        }
    }

    // pub fn to_meme(&self) -> Meme {
    //     Meme {
    //         memeid: 0,
    //         authorid: self.author,
    //         image: self.image,
    //         posted_at: self.posted_at,
    //         upvote: 0,
    //         downvote: 0,
    //         score: rating::score(0, 0),
    //         heat: 10.0,
    //         last_action: Local::now().naive_local(),
    //     }
    // }
}

#[derive(Deserialize, Insertable)]
#[table_name="actions"]
pub struct NewAction {
    pub memeid: i32,
    pub userid: i32,
    pub is_upvote: bool,
}

impl NewAction {
    pub fn action_kind(&self) -> ActionKind {
        if self.is_upvote {
            ActionKind::Upvote
        } else {
            ActionKind::Downvote
        }
    }

    pub fn new_upvote(memeid: i32, userid: i32) -> Self {
        Self{
            memeid,
            userid,
            is_upvote: true,
        }
    }
    pub fn new_downvote(memeid: i32, userid: i32) -> Self {
        Self {
            memeid,
            userid,
            is_upvote: false,
        }
    }
    pub fn new(memeid: i32, userid: i32, action: ActionKind) -> Self {
        match action {
            ActionKind::Upvote => Self::new_upvote(memeid, userid),
            ActionKind::Downvote => Self::new_downvote(memeid, userid),
        }
    }
}

//TODO DELETE
#[derive(Serialize)]
pub struct AllTest  {
    users: Vec<User>, 
    memes: Vec<Meme>,
}
impl AllTest {
    pub fn new (users: Vec<User>, memes: Vec<Meme>,) -> Self {
        Self{users, memes}
    }
}