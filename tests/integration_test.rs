use memebot_backend::*;
use memebot_backend::models::*;
use diesel::prelude::*;

#[test]
fn test_switcher() {
    let mut input = String::new();
    let conn = establish_connection();
    loop {
        println!("
newuser: new user\tnewmeme: new meme
print: print all\tupv: upvote meme\tdwv: downvote meme
delmeme: delete meme
\\end to end\n");
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim_end() {
            "newuser" => create_user_test(&conn),
            "newmeme" => create_meme_test(&conn),
            "print" => load_test(&conn),
            "upv" => add_upv_test(&conn),
            "dwv" => add_dwv_test(&conn),
            "delmeme" => delete_meme_test(&conn),
            "\\end" => break,
            _ => println!("Invalid command"),
        }

        input.truncate(0);
    }
}

fn create_user_test(conn: &PgConnection) {
    let mut name = String::new();
    println!("Insert name of user to create");
    std::io::stdin().read_line(&mut name).unwrap();
    create_user(conn, &name[..name.len()-1]);
}

fn create_meme_test(conn: &PgConnection) {
    let mut input = String::new();
    println!("Insert in IMAGE AUTHORID");
    std::io::stdin().read_line(&mut input).unwrap();

    let procinp: Vec<&str> = input.split(' ').collect();
    if procinp.len() == 2 {
        let authid = procinp[1].trim_end().parse::<i32>().expect("Error parsing author id");
        let newmeme = NewMeme::new((procinp[0], authid));
        create_meme(conn, newmeme);
    }
    else {
        println!("Invalid format! use `IMAGE AUTHORID`");
    }
}

fn load_test(conn: &PgConnection) {
    use memebot_backend::schema::{users,memes,actions};

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

    let results = actions::table
        .load::<Action>(conn)
        .expect("Error loading likes");

    for like in results {
        println!("{:?}", like);
    }
}

fn add_upv_test(conn: &PgConnection) {
    let mut input = String::new();
    loop {
        println!("Insert id of user and meme upvoted (`MEMEID USERID`), \\end to end");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim_end() == "\\end" {
            break;
        }

        let mut initer = input.trim().split(' ');

        let memeid = initer.next()
            .unwrap_or_default()
            .parse::<i32>().expect("Invalid memeid");

        let userid = initer.next()
            .unwrap_or_default()
            .parse::<i32>().expect("Invalid userid");

        meme_action(conn, memeid, userid, ActionKind::Upvote);
        input.truncate(0);
    }
}

fn add_dwv_test(conn: &PgConnection) {
    let mut input = String::new();
    loop {
        println!("Insert id of user and meme downvoted (`MEMEID USERID`), \\end to end");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim_end() == "\\end" {
            break;
        }

        let mut initer = input.trim().split(' ');

        let memeid = initer.next()
            .unwrap_or_default()
            .parse::<i32>().expect("Invalid memeid");

        let userid = initer.next()
            .unwrap_or_default()
            .parse::<i32>().expect("Invalid userid");

        meme_action(conn, memeid, userid, ActionKind::Downvote);
        input.truncate(0);
    }
}

fn delete_meme_test(conn: &PgConnection) {
    use schema::memes::dsl::*;
    let mut input = String::new();
        println!("Insert id of meme to delete");
    std::io::stdin().read_line(&mut input).unwrap();

    let id = input.trim_end().parse::<i32>().expect("Invalid id");

    diesel::delete(memes.filter(memeid.eq(id)))
        .execute(conn)
        .expect(&format!("Error while deleting user {}", &input));
}