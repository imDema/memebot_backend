#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{NewUser,User};

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
        .expect("Error saving post!")
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
