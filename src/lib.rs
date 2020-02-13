#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;

use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use schema::*;

pub mod models;
pub mod rating;
pub mod schema;
pub mod cli;
pub mod meme_query;
mod db;

/// Add a new user with username `username` to database
pub fn create_user(new_user: NewUser) -> QueryResult<()> {
    use schema::users::dsl::*;
    let conn = conn!();
    diesel::insert_into(users)
        .values((
            &new_user,
            userupvote.eq(0),
            userdownvote.eq(0),
            userscore.eq(rating::score(0, 0)))
        )
        .execute(&conn)?;
    Ok(())
}

/// Add a new meme to database
pub fn create_meme(meme: NewMeme) -> QueryResult<()>{
    use schema::memes::dsl::*;
    let conn = conn!();

    let now = Local::now().naive_local();

    diesel::insert_into(memes)
        .values((
            &meme,
            upvote.eq(0),
            downvote.eq(0),
            score.eq(rating::score(0, 0)),
            heat.eq(rating::HEAT_START),
            last_action.eq(&now),
            posted_at.eq(&now),
        ))
        .execute(&conn)?;
    Ok(())
}

fn create_action(conn: &PgConnection, action: NewAction) -> QueryResult<(i32, i32)> {
    use schema::actions::dsl::*;
    let change = match action.action_kind() {
        ActionKind::Upvote => (1, 0),
        ActionKind::Downvote => (0, 1),
    };

    diesel::insert_into(actions)
        .values((
            &action,
            is_active.eq(true),
            posted_at.eq(Local::now().naive_local()),
        ))
        .execute(conn)?;

    // TODO SUBSTITUTE WITH SQL FUNCTIONS AND TRIGGERS
    if let ActionKind::Upvote = &action.action_kind() {
        let (currheat, last_action) = memes::table
            .filter(memes::memeid.eq(action.memeid))
            .select((memes::heat, memes::last_action))
            .get_result::<(f32, NaiveDateTime)>(conn)?;

        let now = Local::now().naive_local();

        diesel::update(memes::table)
            .filter(memes::memeid.eq(action.memeid))
            .set((
                memes::heat
                    .eq(rating::heat_decay(currheat, last_action, now) + rating::HEAT_POS_INCREASE),
                memes::last_action.eq(now),
            ))
            .execute(conn)?;
    }
    // TODO SUBSTITUTE WITH SQL FUNCTIONS AND TRIGGERS
    Ok(change)
}

fn update_action(
    conn: &PgConnection,
    action: NewAction,
    existing_action: &Action,
) -> QueryResult<(i32, i32)> {
    use schema::actions::dsl::*;

    let select_query = diesel::update(actions)
        .filter(memeid.eq(action.memeid))
        .filter(userid.eq(action.userid));

    if existing_action.is_active() {
        if existing_action.action_kind() == action.action_kind() {
            select_query
                .set(is_active.eq(false))
                .execute(conn)?;
            match existing_action.action_kind() {
                ActionKind::Upvote => Ok((-1, 0)),
                ActionKind::Downvote => Ok((0, -1)),
            }
        } else {
            select_query
                .set(is_upvote.eq(action.is_upvote))
                .execute(conn)?;
            match action.action_kind() {
                ActionKind::Upvote => Ok((1, -1)),
                ActionKind::Downvote => Ok((-1, 1)),
            }
        }
    } else {
        select_query
            .set((is_active.eq(true), is_upvote.eq(action.is_upvote)))
            .execute(conn)?;
        match action.action_kind() {
            ActionKind::Upvote => Ok((1, 0)),
            ActionKind::Downvote => Ok((0, 1)),
        }
    }
}

fn apply_action(conn: &PgConnection, action: NewAction) -> QueryResult<(i32, i32)> {
    use schema::actions::dsl::*;
    let existing_action = actions
        .filter(memeid.eq(action.memeid))
        .filter(userid.eq(action.userid))
        .get_result::<Action>(conn)
        .optional()?;

    match existing_action {
        None => create_action(conn, action),
        Some(act) => update_action(conn, action, &act),
    }
}

/// Handle upvote or downvote event
/// upvoting an already upvoted post will cancel the vote (same for downvotes)
/// upvoting a downvoted post will cancel the downvote and add an upvote (same reversed)
///
/// # Arguments
/// * `memeid` id of the meme upvoted or downvoted
/// * `userid` id of the user which did the action
pub fn new_action(action: NewAction) -> QueryResult<()> {
    let conn = conn!();
    conn.transaction::<_,diesel::result::Error, _>(|| {
        let memeid = action.memeid.clone();
        let (upchange, downchange) = apply_action(&conn, action)?;

        let meme: Meme = diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
            .set((
                memes::upvote.eq(memes::upvote + upchange),
                memes::downvote.eq(memes::downvote + downchange),
            ))
            .get_result(&conn)?;

        let user: User = diesel::update(users::table.filter(users::userid.eq(meme.authorid)))
            .set((
                users::userupvote.eq(users::userupvote + upchange),
                users::userdownvote.eq(users::userdownvote + downchange),
            ))
            .get_result(&conn)?;

        //TODO REPLACE THIS WITH SQL FUNCTION / TRIGGER
        let new_meme_score = rating::score(meme.upvote, meme.downvote);
        let new_user_score = rating::score(user.userupvote, user.userdownvote);

        diesel::update(memes::table.filter(memes::memeid.eq(memeid)))
            .set(memes::score.eq(new_meme_score))
            .execute(&conn)?;
        diesel::update(users::table.filter(users::userid.eq(meme.authorid)))
            .set(users::userscore.eq(new_user_score))
            .execute(&conn)?;
        //TODO REPLACE THIS WITH SQL FUNCTION / TRIGGER

        Ok(())
    })?;
    Ok(())
}

/// Add a new tag with name `tagname` to database
pub fn create_tag(tagname: &str) -> QueryResult<Tag> {
    let conn = conn!();
    conn.transaction( || {
    let saved_tag = tags::table
        .filter(tags::tagname.like(tagname))
        .get_result::<Tag>(&conn)
        .optional()?;

    match saved_tag {
        None => {
            let new_tag = diesel::insert_into(tags::table)
                .values(tags::tagname.eq(tagname))
                .returning((tags::tagid, tags::tagname))
                .get_result::<Tag>(&conn)?;
            Ok(new_tag)
        }
        Some(saved_tag) => Ok(saved_tag)
    }
    })
}

/// Add tag `tagid` to meme `memeid`
pub fn add_meme_tag(memeid: i32, tagid: i32) -> QueryResult<()> {
    let conn = conn!();
    diesel::insert_into(meme_tags::table)
        .values(MemeTag::new(memeid, tagid))
        .on_conflict((meme_tags::memeid, meme_tags::tagid))
        .do_nothing()
        .execute(&conn)?;
    Ok(())
}

/// Returns all memes with tag `tagid`
pub fn memes_by_tagid(tagid: i32) -> QueryResult<Vec<Meme>> {
    let conn = conn!();
    memes::dsl::memes
        .inner_join(meme_tags::table.inner_join(tags::table))
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
            memes::posted_at,
        ))
        .load::<Meme>(&conn)
}

/// Returns all memes with tag `tagid` ordered by score
pub fn memes_by_tag_score_ordered(tagid: i32) -> QueryResult<Vec<Meme>> {
    let conn = conn!();
    memes::dsl::memes
        .inner_join(meme_tags::dsl::meme_tags.inner_join(tags::table))
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
            memes::posted_at,
        ))
        .order_by(memes::score.desc())
        .load::<Meme>(&conn)
}

pub fn memes_by_heat(quantity: usize) -> QueryResult<Vec<Meme>> {
    let conn = conn!();
    let mut allmemes: Vec<Meme> = memes::table
        .load::<Meme>(&conn)?;

    let now = Local::now().naive_local();
    
    allmemes.iter_mut()
        .for_each(|mut meme| meme.heat = rating::heat_decay(meme.heat, meme.last_action, now));

    allmemes.sort_unstable_by(|b, a| {
        a.heat
            .partial_cmp(&b.heat)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    allmemes.truncate(quantity);
    
    Ok(allmemes)
}

pub fn memes_by_userid(userid: i32) -> QueryResult<Vec<Meme>> {
    let conn = conn!();
    memes::table
        .filter(memes::author.eq(userid))
        .load::<Meme>(&conn)
}

pub fn user(userid: i32) -> QueryResult<User> {
    let conn = conn!();
    users::table
        .filter(users::userid.eq(userid))
        .get_result(&conn)
}

pub fn all_users() -> QueryResult<Vec<User>> {
    let conn = conn!();
    users::table
        .load::<User>(&conn)
}

pub fn all_memes() -> QueryResult<Vec<Meme>> {
    let conn = conn!();
    memes::table
        .load::<Meme>(&conn)
}

pub fn all_tags() -> QueryResult<Vec<Tag>> {
    let conn = conn!();
    tags::table
        .load::<Tag>(&conn)
}

#[cfg(test)]
mod tests {
    // use super::*;
}
