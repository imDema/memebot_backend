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
    pub testbool: bool,
}