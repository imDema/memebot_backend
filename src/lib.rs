#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

use std::error::Error;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use chrono::prelude::*;
use std::env;
use self::models::*;

pub mod schema;
pub mod models;
pub mod rating;

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

    let new_user = NewUser::new(username);
    
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

fn increase_upvote_counters(conn: &PgConnection, (memeid, userid) : (i32, i32)) -> Result<(), Box<dyn Error>> {
    use schema::{memes, users};
    let meme: Meme = diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
        .set(memes::upvote.eq(memes::upvote + 1))
        .get_result(conn)?;

    diesel::update(users::table.filter(users::userid.eq(meme.author)))
        .set(users::userupvote.eq(users::userupvote + 1))
        .execute(conn)?;
    Ok(())
}
fn decrease_upvote_counters(conn: &PgConnection, (memeid, userid) : (i32, i32)) -> Result<(), Box<dyn Error>> {
    use schema::{memes, users};
    let meme: Meme = diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
        .set(memes::upvote.eq(memes::upvote - 1))
        .get_result(conn)?;

    diesel::update(users::table.filter(users::userid.eq(meme.author)))
        .set(users::userupvote.eq(users::userupvote - 1))
        .execute(conn)?;
        Ok(())
}

fn increase_downvote_counters(conn: &PgConnection, (memeid, userid) : (i32, i32)) -> Result<(), Box<dyn Error>> {
    use schema::{memes, users};
    let meme: Meme = diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
        .set(memes::downvote.eq(memes::downvote + 1))
        .get_result(conn)?;

    diesel::update(users::table.filter(users::userid.eq(meme.author)))
        .set(users::userdownvote.eq(users::userdownvote + 1))
        .execute(conn)?;
    Ok(())
}
fn decrease_downvote_counters(conn: &PgConnection, (memeid, userid) : (i32, i32)) -> Result<(), Box<dyn Error>> {
    use schema::{memes, users};
    let meme: Meme = diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
        .set(memes::downvote.eq(memes::downvote - 1))
        .get_result(conn)?;

    diesel::update(users::table.filter(users::userid.eq(meme.author)))
        .set(users::userdownvote.eq(users::userdownvote - 1))
        .execute(conn)?;
        Ok(())
}

pub fn meme_action(conn: &PgConnection, memeid: i32, userid: i32, action: ActionKind) {
    use schema::{actions};

    let action_key = (memeid, userid);

    match action {
        ActionKind::Upvote => increase_upvote_counters(conn, action_key),
        ActionKind::Downvote => increase_downvote_counters(conn, action_key),
    }.expect("Error updating vote count: {}");

    diesel::insert_into(actions::table)
        .values(Action::new((memeid, userid), action))
        .execute(conn)
        .expect("Error tracking like data");
}

#[cfg(test)]
mod tests {
}
