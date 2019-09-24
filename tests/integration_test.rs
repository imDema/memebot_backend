use memebot_backend::*;
use memebot_backend::models::*;
use diesel::prelude::*;

#[test]
fn test_switcher() {
    let mut input = String::new();
    let conn = establish_connection();
    loop {
        println!("
        newuser: new meme\tnewmeme: new meme
        print: print all\tupv: upvote meme
        delmeme: delete meme
        \\end to end\n");
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim_end() {
            "newusr" => create_user_test(&conn),
            "newmeme" => load_test(&conn),
            "newuser" => create_meme_test(&conn),
            "upv" => add_upv_test(&conn),
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
    let mut name = String::new();
    println!("Insert in IMAGE AUTHORID");
    std::io::stdin().read_line(&mut name).unwrap();


    create_user(conn, &name[..name.len()-1]);
}

fn load_test(conn: &PgConnection) {
    use memebot_backend::schema::{users,memes};

    let results = users::table
        .load::<User>(conn)
        .expect("Error loading users!");

    for user in results {
        println!("---------------------\nuserid:{}\tusername:{}\nupv:{}\tdwv:{}", user.userid, user.username, user.userupvote, user.userdownvote);
        let memeresults = memes::table
            .filter(memes::dsl::author.eq(user.userid))
            .load::<Meme>(conn)
            .expect("Error loading memes!");

        for meme in memeresults {
            println!("++++++++++++++++\tmemeid:{}\timage:{}\n\tupv:{}\tdwv:{}", meme.memeid, meme.image, meme.upvote, meme.downvote);
        }
    }
}

fn add_upv_test(conn: &PgConnection) {
    let mut input = String::new();
    loop {
        println!("Insert id of meme to upvote, \\end to end");
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim_end() == "\\end" {
            break;
        }
        let id = input.trim_end().parse::<i32>().expect("Invalid id");

        user_increase_upvote(conn, id);
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