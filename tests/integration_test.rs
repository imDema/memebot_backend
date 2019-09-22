use memebot_backend::*;
use memebot_backend::models::*;
use diesel::prelude::*;

#[test]
fn create_user_test() {
    let conn = establish_connection();
    let mut name = String::new();
    loop {
        println!("Insert name of user to create, \\end to end");
        std::io::stdin().read_line(&mut name).unwrap();
        if name.trim_end() == "\\end" {
            break;
        }
        create_user(&conn, &name[..name.len()-1]);
        name.truncate(0);
    }
}

#[test]
fn load_test() {
    use memebot_backend::schema::users::dsl::*;

    let conn = establish_connection();
    let results = users
        .load::<User>(&conn)
        .expect("Error loading users!");

    for user in results {
        println!("---------------------\nuserid:{}\nusername:{}\nupv:{}\ndwv:{}", user.userid, user.username, user.userupvote, user.userdownvote);
    }
}
