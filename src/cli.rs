use super::*;
use std::io::{Write, BufRead};

pub const HELPMESSAGE: &str = "COMMANDS:
adduser USERID USERNAME\taddmeme IMAGE AUTHORID
addtag TAGNAME\taddmemetag MEMEID TAGID
upvote MEMEID FROMUSERID\tdownvote MEMEID FROMUSERID
print\tprintmeta\tprinttags\tprintuser\tprinthot
delmeme MEMEID
help";

pub fn create_user_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>) -> Result<(), String>{
    if let Some(id) = words.next() {
        if let Some(name) = words.next() {
            let userid = id.parse::<i32>().map_err(|_| "Error parsing userid".to_owned())?;
            let new_user = NewUser::new(userid, name);
            create_user(conn, new_user)
                .map_err(|err| format!("Error creating user: {}", err))
        }
        else {
            Err("Missing argument USERNAME".to_owned())
        }
    } else {
        Err("Missing arguments USERID USERNAME".to_owned())
    }
}

pub fn create_meme_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>) -> Result<(), String>{
    if let Some(image) = words.next() {
        if let Some(authorid) = words.next() {
            let id = authorid.parse::<i32>()
                .map_err(|_| "Cannot parse AUTHORID".to_owned())?;
            create_meme(conn, NewMeme::new(image, id))
                .map_err(|err| format!("{}", err))
        } else {
            Err("Invalid or missing arguments".to_owned())
        }
    } else {
        Err("Missing arguments".to_owned())
    }
}

pub fn print_test<T: Write>(conn: &PgConnection, out_str: &mut T) -> Result<(), String>{

    let results = users::table
        .order(users::userscore.desc())
        .load::<User>(conn)
        .map_err(|err| format!("Error loading users: {}", err))?;

    for user in results {
        writeln!(out_str, "{:?}", user).unwrap();
        let memeresults = memes::table
            .filter(memes::author.eq(user.userid))
            .order(memes::score.desc())
            .load::<Meme>(conn)
            .map_err(|err|  format!("Error loading memes: {}", err))?;

        for meme in memeresults {
            writeln!(out_str, "\t{:?}", meme).unwrap();
        }
    }
    Ok(())
}

pub fn print_tag_test<T: Write>(conn: &PgConnection, out_str: &mut T) -> Result<(), String> {
    let tgs = tags::table
        .select((tags::tagid, tags::tagname))
        .load::<(i32, String)>(conn)
        .map_err(|err|  format!("Error loading tags: {}", err))?;

    for t in tgs {
        writeln!(out_str, "{:?}", &t).unwrap();

        let mms = memes_by_tag_score_ordered(conn, t.0)
            .map_err(|err|  format!("Error loading memes: {}", err))?;
        for meme in mms.iter() {
            writeln!(out_str, "{:?}", meme).unwrap();
        }
    }
    Ok(())
}

pub fn print_users_test<T: Write>(conn: &PgConnection, out_str: &mut T)  -> Result<(), String> {
    let results = users::table
        .order(users::userscore.desc())
        .load::<User>(conn)
        .map_err(|err| format!("Error loading users: {}", err))?;

    for user in results {
        writeln!(out_str, "{:?}", user).unwrap();
    }
    Ok(())
}

pub fn print_hot<T: Write>(conn: &PgConnection, out_str: &mut T)  -> Result<(), String> {
    for mm in memes_by_heat(conn, 10).map_err(|e| format!("{}",e))? {
        writeln!(out_str, "{:?}", mm).unwrap();
    }
    Ok(())
}

pub fn print_meta_test<T: Write>(conn: &PgConnection, out_str: &mut T) -> Result<(), String> {
    let results = actions::table
        .load::<Action>(conn)
        .map_err(|err| format!("Error loading likes: {}", err))?;

    for action in results {
        writeln!(out_str, "{:?}", action).unwrap();
    }

    let results = tags::table
        .select((tags::tagid, tags::tagname))
        .get_results::<(i32, String)>(conn)
        .map_err(|err|  format!("Error loading tags: {}", err))?;

    for tag in results {
        writeln!(out_str, "Tag {:?}", tag).unwrap();
    }

    let res = meme_tags::table
        .load::<MemeTag>(conn)
        .map_err(|err|  format!("Error loading meme-tags: {}", err))?;

    for meme_tag in res {
        writeln!(out_str, "{:?}", meme_tag).unwrap();
    }
    Ok(())
}

pub fn upvote_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>)  -> Result<(), String> {
    let memeid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments".to_owned()),
    }.parse::<i32>();

    let userid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments".to_owned()),
    }.parse::<i32>();

    match (memeid, userid) {
        (Ok(meid), Ok(usid)) => new_action(conn, NewAction::new_upvote(meid, usid)).map_err(|err| format!("Error applying action: {}", err)),
        (Err(_), _) => Err("Error parsing memeid".to_owned()),
        (_, Err(_)) => Err("Error parsing userid".to_owned()),
    }
}

pub fn downvote_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>)  -> Result<(), String> {
    let memeid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments".to_owned()),
    }.parse::<i32>();

    let userid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments".to_owned()),
    }.parse::<i32>();

    match (memeid, userid) {
        (Ok(meid), Ok(usid)) => new_action(conn, NewAction::new_downvote(meid, usid)).map_err(|err|  format!("Error applying action: {}", err)),
        (Err(_), _) => Err("Error parsing memeid".to_owned()),
        (_, Err(_)) => Err("Error parsing userid".to_owned()),
    }
}


pub fn create_tag_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>)  -> Result<(), String> {
    let tag = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments".to_owned()),
    };

    create_tag(conn, tag)
        .map(|_| ())
        .map_err(|err|  format!("Error creating tag: {}", err))
}

pub fn add_meme_tag_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>)  -> Result<(), String> {
    let memeid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments".to_owned()),
    }.parse::<i32>();

    let tagid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments".to_owned()),
    }.parse::<i32>();

    match (memeid, tagid) {
        (Ok(meid), Ok(tgid)) => add_meme_tag(conn, meid, tgid)
                                    .map_err(|err|  format!("Error adding tag: {}", err)),
        (Err(_), _) => Err("Error parsing memeid".to_owned()),
        (_, Err(_)) => Err("Error parsing userid".to_owned()),
    }
}

/// Trash code, very temporary
pub fn delete_meme_test<'a> (conn: &PgConnection, mut words: impl Iterator<Item = &'a str>)  -> Result<(), String> {
    let memeid = match words.next() {
        Some(w) => w,
        None => return Err("Not enough arguments".to_owned()),
    }.parse::<i32>();

    let id = match memeid {
        Ok(id) => id,
        Err(_) => return Err("Error parsing memeid".to_owned()),
    };
    
    let meem = memes::table.filter(memes::memeid.eq(id))
        .get_result::<Meme>(conn)
        .map_err(|err|  format!("Error loading meme: {}", err))?;
    
    let user: User = users::table.filter(users::userid.eq(meem.authorid))
        .get_result::<User>(conn)
        .map_err(|err|  format!("Error deleting meme: {}", err))?;

    diesel::delete(actions::table.filter(actions::memeid.eq(id)))
        .execute(conn)
        .map_err(|err|  format!("Error deleting actions: {}", err))?;
    
    diesel::update(users::table.filter(users::userid.eq(meem.authorid)))
        .set((
            users::userupvote.eq(user.userupvote - meem.upvote),
            users::userdownvote.eq(user.userdownvote - meem.downvote),
            users::userscore.eq(rating::score(user.userupvote - meem.upvote, user.userdownvote - meem.downvote)),
            ))
        .execute(conn)
        .map_err(|err|  format!("Error updating user score: {}", err))?;

    diesel::delete(memes::table.filter(memes::memeid.eq(id)))
        .execute(conn)
        .map_err(|err|  format!("Error deleting meme: {}", err))?;
    Ok(())
}

pub fn switcher<R: BufRead, W: Write>(conn: &PgConnection, in_str: &mut R, out_str: &mut W) {
    let mut input = String::new();

    writeln!(out_str, "{}", HELPMESSAGE).unwrap();

    loop {
        writeln!(out_str, "memebot_backend > ").unwrap();
        Write::flush(out_str).expect("stdout flush failed");
        let line_read = in_str.read_line(&mut input).unwrap();
        if line_read == 0 {
            break;
        }
        let mut words = input[..input.len()-1].split(' ');

        let command_result = match words.next() {
            Some("adduser") => create_user_test(&conn, words),
            Some("addmeme") => create_meme_test(&conn, words),
            Some("addtag") => create_tag_test(&conn, words),
            Some("addmemetag") => add_meme_tag_test(&conn, words),
            Some("upvote") => upvote_test(&conn, words),
            Some("downvote") => downvote_test(&conn, words),
            Some("delmeme") => delete_meme_test(&conn, words),
            Some("print") => print_test(&conn, out_str),
            Some("printuser") => print_users_test(&conn, out_str),
            Some("printtags") => print_tag_test(&conn, out_str),
            Some("printmeta") => print_meta_test(&conn, out_str),
            Some("printhot") => print_hot(&conn, out_str),
            Some("help") => writeln!(out_str, "{}", HELPMESSAGE).map_err(|err|  format!("Error printing help message: {}", err)),
            _ => Err("Invalid command".to_owned()),
        };

        if let Err(why) = command_result {
            eprintln!("{}\n", why);
        }

        input.truncate(0);
    }
}

pub struct Cli {
    conn: PgConnection,
}

impl Cli {
    pub fn new() -> Cli {
        let conn = establish_connection();
        Cli{
            conn
        }
    }
    pub fn connection(&self) -> &PgConnection {
        &self.conn
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}