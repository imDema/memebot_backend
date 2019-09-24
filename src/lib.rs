#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use chrono::prelude::*;
use std::env;
use self::models::{NewUser,User,NewMeme, Meme};

pub mod schema;
pub mod models;

///Read database url from .env and connect to it
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env !");

    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to database {}", db_url))
}

pub fn create_user(conn: &PgConnection, username: &str) -> User {
    use schema::users;

    let new_user = NewUser {
        username: username,
        userupvote: 0,
        userdownvote: 0,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error creating user!")
}

pub fn create_meme(conn: &PgConnection, meme: NewMeme) {
    use schema::memes;
    let mm : Meme = diesel::insert_into(memes::table)
        .values(&meme)
        .get_result(conn)
        .expect("Error creating meme!");
    println!("Created user: {:?}", mm);
}

pub fn like_meme(conn: &PgConnection, userid: i32, memeid: i32) {
    use schema::{users, memes,likes};
    println!("{}",diesel::debug_query::<diesel::pg::Pg,_>(&diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
        .set(memes::upvote.eq(memes::upvote + 1))));


    let meme: Meme = diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
        .set(memes::upvote.eq(memes::upvote + 1))
        .get_result(conn)
        .expect("Error increasing meme upvotes");

    diesel::update(users::table.filter(users::userid.eq(meme.author)))
        .set(users::userupvote.eq(users::userupvote + 1))
        .execute(conn)
        .expect(&format!("Error increasing user upvotes for meme {:?}", meme));

    diesel::insert_into(likes::table)
        .values((likes::memeid.eq(memeid), likes::userid.eq(userid), likes::liked_at.eq(Local::now().naive_local())))
        .execute(conn)
        .expect("Error tracking like data");
}

pub fn user_increase_upvote(conn: &PgConnection, id: i32) {
    use schema::users::dsl::{users, userupvote};
    diesel::update(users.find(id))
        .set(userupvote.eq(userupvote + 1))
        .execute(conn)
        .expect(&format!("Can't find user {}", id));
}

#[cfg(test)]
mod tests {
}
