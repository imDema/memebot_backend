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

#[derive(Debug)]
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
pub struct NewMeme {
    author: i32,
    image: String,
    upvote: i32,
    downvote: i32,
}

impl NewMeme {
    pub fn new((img, author): (&str, i32)) -> NewMeme {
        NewMeme {
            author,
            image: img.to_owned(),
            upvote: 0,
            downvote: 0,
        }
    }
}