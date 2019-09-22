use super::schema::users;

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