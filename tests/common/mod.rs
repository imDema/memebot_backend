use memebot_backend::*;
use memebot_backend::schema::*;
use memebot_backend::models::*;
use diesel::prelude::*;

pub fn create_user_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>) -> Result<(), &'static str>{
    match words.next() {
        Some(w) => Ok(create_user(conn, w)),
        None => Err("Not enough arguments\n")
    }
}

pub fn create_meme_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>) -> Result<(), &'static str>{
    let image = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    };

    let authorid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    }.parse::<i32>();
    
    match authorid {
        Ok(id) => Ok(create_meme(conn, NewMeme::new(image, id))),
        Err(_) => Err("Error parsing author id"),
    }
}

pub fn print_test(conn: &PgConnection) {

    let results = users::table
        .order(users::userscore.desc())
        .load::<User>(conn)
        .expect("Error loading users!");

    for user in results {
        println!("{:?}", user);
        let memeresults = memes::table
            .filter(memes::author.eq(user.userid))
            .order(memes::score.desc())
            .load::<Meme>(conn)
            .expect("Error loading memes!");

        for meme in memeresults {
            println!("\t{:?}", meme);
        }
    }
}

pub fn print_tag_test(conn: &PgConnection) {
    let tgs = tags::table
        .select((tags::tagid, tags::tagname))
        .get_results::<(i32, String)>(conn)
        .expect("Error retrieving tags");

    for t in tgs {
        println!("{:?}", &t);

        let mms = memes_by_tag_score_ordered(conn, t.0);
        for meme in mms.iter() {
            println!("{:?}", meme);
        }
    }
}

pub fn print_users_test(conn: &PgConnection) {
    let results = users::table
        .order(users::userscore.desc())
        .load::<User>(conn)
        .expect("Error loading users!");

    for user in results {
        println!("{:?}", user);
    }
}

pub fn print_meta_test(conn: &PgConnection) {
    let results = actions::table
        .load::<Action>(conn)
        .expect("Error loading likes");

    for action in results {
        println!("{:?}", action);
    }

    let results = tags::table
        .select((tags::tagid, tags::tagname))
        .get_results::<(i32, String)>(conn)
        .expect("Error loading tags");

    for tag in results {
        println!("Tag {:?}", tag);
    }

    let res = meme_tags::table
        .load::<MemeTag>(conn)
        .expect("Error loading MemeTags");

    for meme_tag in res {
        println!("{:?}", meme_tag);
    }
}

pub fn upvote_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>) -> Result<(), &'static str> {
    let memeid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    }.parse::<i32>();

    let userid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    }.parse::<i32>();

    match (memeid, userid) {
        (Ok(meid), Ok(usid)) => Ok(meme_action(conn, meid, usid, ActionKind::Upvote)),
        (Err(_), _) => Err("Error parsing memeid\n"),
        (_, Err(_)) => Err("Error parsing userid\n"),
    }
}

pub fn downvote_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>) -> Result<(), &'static str> {
    let memeid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    }.parse::<i32>();

    let userid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    }.parse::<i32>();

    match (memeid, userid) {
        (Ok(meid), Ok(usid)) => Ok(meme_action(conn, meid, usid, ActionKind::Downvote)),
        (Err(_), _) => Err("Error parsing memeid\n"),
        (_, Err(_)) => Err("Error parsing userid\n"),
    }
}


pub fn create_tag_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>) -> Result<(), &'static str> {
    let tag = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    };

    Ok(create_tag(conn, tag))
}

pub fn add_meme_tag_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>) -> Result<(), &'static str> {
    let memeid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    }.parse::<i32>();

    let tagid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    }.parse::<i32>();

    match (memeid, tagid) {
        (Ok(meid), Ok(tgid)) => Ok(add_meme_tag(conn, meid, tgid)),
        (Err(_), _) => Err("Error parsing memeid\n"),
        (_, Err(_)) => Err("Error parsing userid\n"),
    }
}

/// Trash code, very temporary
pub fn delete_meme_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>) -> Result<(), &'static str> {
    let memeid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments\n"),
    }.parse::<i32>();

    let id = match memeid {
        Ok(id) => id,
        Err(_) => return Err("Error parsing memeid\n"),
    };

    diesel::delete(actions::table.filter(actions::memeid.eq(id)))
        .execute(conn)
        .expect("Error deleting actions for selected meme");

    let meem = memes::table.filter(memes::memeid.eq(id))
        .get_result::<Meme>(conn)
        .expect("Error retrieving meme");

    let user: User = diesel::update(users::table.filter(users::userid.eq(meem.authorid)))
        .set((
            users::userupvote.eq(users::userupvote - meem.upvote),
            users::userdownvote.eq(users::userdownvote - meem.downvote),
            ))
        .get_result(conn)
        .expect("Error updating user vote counters");

    diesel::update(users::table.filter(users::userid.eq(meem.authorid)))
        .set(users::userscore.eq(rating::score(user.userupvote, user.userdownvote)))
        .execute(conn)
        .expect("Error updating user score");

    diesel::delete(memes::table.filter(memes::memeid.eq(id)))
        .execute(conn)
        .expect(&format!("Error while deleting user {}", id));
    Ok(())
}
