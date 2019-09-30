#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use chrono::prelude::*;
use std::env;
use models::*;
use schema::*;

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

/// Add a new user with username `username` to database
pub fn create_user(conn: &PgConnection, username: &str) {
    let new_user = NewUser::new(username);
    
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error creating user!");
}

/// Add a new meme to database
pub fn create_meme(conn: &PgConnection, meme: NewMeme) {
    diesel::insert_into(memes::table)
        .values(&meme)
        .execute(conn)
        .expect("Error creating meme!");
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

    // TODO SUBSTITUTE WITH SQL FUNCTIONS AND TRIGGERS
    let (currheat, last_action) = memes::table
        .filter(memes::memeid.eq(action_key.0))
        .select((memes::heat, memes::last_action))
        .get_result::<(f32, NaiveDateTime)>(conn)
        .expect("Error retrieving heat");

    let now = Local::now().naive_local();

    diesel::update(memes::table)
        .filter(memes::memeid.eq(action_key.0))
        .set((memes::heat.eq(rating::heat_decay(currheat, last_action, now)),
            memes::last_action.eq(now)))
        .execute(conn)
        .expect("Error updating heat");
    
    // TODO SUBSTITUTE WITH SQL FUNCTIONS AND TRIGGERS
    
    change
}

fn update_action(conn: &PgConnection, action_key: (i32, i32), action: ActionKind, existing_action: &Action) -> (i32, i32) {
    use schema::actions::dsl::*;

    let select_query = diesel::update(actions)
        .filter(memeid.eq(action_key.0))
        .filter(userid.eq(action_key.1));

    if existing_action.is_active() {
        if existing_action.get_action_kind() == action {
            select_query
                .set(is_active.eq(false))
                .execute(conn)
                .expect("Error deactivating action");
            match action {
                ActionKind::Upvote => (-1, 0),
                ActionKind::Downvote => (0, -1),
            }
        } else {
            select_query
                .set(is_upvote.eq(action.is_upvote()))
                .execute(conn)
                .expect("Error inverting action");
            match action {
                ActionKind::Upvote => (1, -1),
                ActionKind::Downvote => (-1, 1),
            }
        }
    } else {
        select_query
            .set((is_active.eq(true), is_upvote.eq(action.is_upvote())))
            .execute(conn)
            .expect("Error activating action");
        match action {
            ActionKind::Upvote => (1, 0),
            ActionKind::Downvote => (0, 1),
        }
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
        1 => update_action(conn, action_key, action, &existing_action[0]),
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
    let action_key = (memeid, userid);

    let (upchange, downchange) = apply_action(conn, action_key, action);

    let meme: Meme = diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
        .set((
            memes::upvote.eq(memes::upvote + upchange),
            memes::downvote.eq(memes::downvote + downchange),
            ))
        .get_result(conn)
        .expect("Error updating meme vote counters");

    let user: User = diesel::update(users::table.filter(users::userid.eq(meme.authorid)))
        .set((
            users::userupvote.eq(users::userupvote + upchange),
            users::userdownvote.eq(users::userdownvote + downchange),
            ))
        .get_result(conn)
        .expect("Error updating user vote counters");

    //TODO REPLACE THIS WITH SQL FUNCTION / TRIGGER
    let new_meme_score = rating::score(meme.upvote, meme.downvote);
    let new_user_score = rating::score(user.userupvote, user.userdownvote);

    diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
        .set(memes::score.eq(new_meme_score))
        .execute(conn)
        .expect("Error updating meme score");
    diesel::update(users::table.filter(users::userid.eq(meme.authorid)))
        .set(users::userscore.eq(new_user_score))
        .execute(conn)
        .expect("Error updating meme score");
    //TODO REPLACE THIS WITH SQL FUNCTION / TRIGGER    
}

/// Add a new tag with name `tagname` to database
pub fn create_tag(conn: &PgConnection, tagname: &str) {
    let saved_tag = tags::table
        .filter(tags::tagname.like(tagname))
        .select(tags::tagid)
        .get_result::<i32>(conn)
        .optional()
        .expect("Error checking tag existence");
    
    match saved_tag {
        None => {
            diesel::insert_into(tags::table)
                .values(tags::tagname.eq(tagname))
                .execute(conn)
                .expect("Error creating tag");()
            },
        Some(id) => eprintln!("Tag already exists with id {}!", id),
    };        
}

/// Add tag `tagid` to meme `memeid`
pub fn add_meme_tag(conn: &PgConnection, memeid: i32, tagid: i32) {
    diesel::insert_into(meme_tags::table)
        .values(MemeTag::new(memeid, tagid))
        .on_conflict((meme_tags::memeid, meme_tags::tagid))
        .do_nothing()
        .execute(conn)
        .expect("Error adding tag to meme");
}

/// Returns all memes with tag `tagid`
pub fn memes_by_tag(conn: &PgConnection, tagid: i32) -> Vec<Meme> {
    memes::dsl::memes
        .inner_join(meme_tags::dsl::meme_tags
            .inner_join(tags::dsl::tags))
        .filter(tags::tagid.eq(tagid))
        .select((
            memes::memeid,
            memes::author,
            memes::image,
            memes::upvote,
            memes::downvote,
            memes::score,
            memes::heat,
            memes::last_action,
            memes::posted_at,))
        .load::<Meme>(conn)
        .expect("Error retrieving memes by tag")
}

/// Returns all memes with tag `tagid` ordered by score
pub fn memes_by_tag_score_ordered(conn: &PgConnection, tagid: i32) -> Vec<Meme> {
    memes::dsl::memes
        .inner_join(meme_tags::dsl::meme_tags
            .inner_join(tags::dsl::tags))
        .filter(tags::tagid.eq(tagid))
        .select((
            memes::memeid,
            memes::author,
            memes::image,
            memes::upvote,
            memes::downvote,
            memes::score,
            memes::heat,
            memes::last_action,
            memes::posted_at,))
        .order_by(memes::score.desc())
        .load::<Meme>(conn)
        .expect("Error retrieving memes by tag")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_tag() {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        create_tag(&establish_connection(), &s[..s.len()-1]);
    }
}
