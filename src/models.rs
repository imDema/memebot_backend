use super::schema::users;
use super::schema::memes;

#[derive(Debug)]
#[derive(Queryable)]
pub struct Meme {
    pub memeid: i32,
    pub author: i32,
    pub image: String,
    pub upvote: i32,
    pub downvote: i32,
}

#[derive(Queryable)]
pub struct User {
    pub userid: i32,
    pub username: String,
    pub userupvote: i32,
    pub userdownvote: i32,
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
pub struct NewMeme<'a> {
    author: i32,
    image: &'a str,
    upvote: i32,
    downvote: i32,
}