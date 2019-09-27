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

fn create_action(conn: &PgConnection, action_key: (i32, i32), action: ActionKind) -> (i32, i32) {
    use schema::actions::dsl::*;
    let change = match &action {
        ActionKind::Upvote => (1, 0),
        ActionKind::Downvote => (0, 1),
    };

    diesel::insert_into(actions)
        .values(Action::new(action_key, action))
        .execute(conn)
        .expect("Error creating new action!");
    
    change
}

fn cancel_action(conn: &PgConnection, action_key: (i32, i32), action: ActionKind) -> (i32, i32) {
    use schema::actions::dsl::*;
    diesel::delete(actions)
        .filter(memeid.eq(action_key.0))
        .filter(userid.eq(action_key.1))
        .execute(conn)
        .expect("Error trying to delete action!");
    
    match action {
        ActionKind::Upvote => (-1, 0),
        ActionKind::Downvote => (0, -1),
    }
}

fn update_action(conn: &PgConnection, action_key: (i32, i32), action: ActionKind) -> (i32, i32) {
    use schema::actions::dsl::*;

    diesel::update(actions)
        .filter(memeid.eq(action_key.0))
        .filter(userid.eq(action_key.1))
        .set((
            is_upvote.eq(match &action {
                ActionKind::Upvote => true,
                ActionKind::Downvote => false,
            }),
            posted_at.eq(Local::now().naive_local())
        ))
        .execute(conn)
        .expect("Error trying to delete action!");
    
    match action {
        ActionKind::Upvote => (1, -1),
        ActionKind::Downvote => (-1, 1),
    }
}

fn apply_action(conn: &PgConnection, action_key: (i32, i32), action: ActionKind) -> (i32, i32) {
    use schema::actions::dsl::*;
    let existing_action = actions
        .filter(memeid.eq(action_key.0))
        .filter(userid.eq(action_key.1))
        .load::<Action>(conn)
        .expect(&format!("Error retrieving existing actions for (memeid, userid): ({}, {})", action_key.0, action_key.1));

    match existing_action.len() {
        0 => create_action(conn, action_key, action),
        1 => {
            if existing_action[0].get_action_kind() == action {
                cancel_action(conn, action_key, action)
            } else {
                update_action(conn, action_key, action)
            }
        },
        _ => panic!(format!("Found multiple actions for (memeid, userid): ({}, {})!", action_key.0, action_key.1))
    }
}

/// Handle upvote or downvote event
/// upvoting an already upvoted post will cancel the vote (same for downvotes)
/// upvoting a downvoted post will cancel the downvote and add an upvote (same reversed)
/// 
/// # Arguments
/// * `memeid` id of the meme upvoted or downvoted
/// * `userid` id of the user which did the action
pub fn meme_action(conn: &PgConnection, memeid: i32, userid: i32, action: ActionKind) {
    use schema::{memes, users};

    let action_key = (memeid, userid);

    let (upchange, downchange) = apply_action(conn, action_key, action);

    let meme: Meme = diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
        .set((
            memes::upvote.eq(memes::upvote + upchange),
            memes::downvote.eq(memes::downvote + downchange),
            ))
        .get_result(conn)
        .expect("Error updating meme vote counters");

    diesel::update(users::table.filter(users::userid.eq(meme.author)))
        .set((
            users::userupvote.eq(users::userupvote + upchange),
            users::userdownvote.eq(users::userdownvote + downchange),
            ))
        .execute(conn)
        .expect("Error updating user vote counters");
}

#[cfg(test)]
mod tests {
}
