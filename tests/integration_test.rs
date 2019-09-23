use memebot_backend::*;
use memebot_backend::models::*;
use diesel::prelude::*;

#[test]
fn test_switcher() {
    let mut input = String::new();
    loop {
        println!("\ncu: create user\tlu: load users\nau: add upvote\tdu: delete user\n\\end to end\n");
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim_end() {
            "cu" => create_user_test(),
            "lu" => load_test(),
            "au" => add_upv_test(),
            "du" => delete_user_test(),
            "\\end" => break,
            _ => println!("Invalid command"),
        }

        input.truncate(0);
    }
}

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

#[test]
fn add_upv_test() {
    let conn = establish_connection();
    let mut input = String::new();
    loop {
        println!("Insert name of user id to give upvote to, \\end to end");
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim_end() == "\\end" {
            break;
        }
        let id = input.trim_end().parse::<i32>().expect("Invalid id");

        user_increase_upvote(&conn, id);
        input.truncate(0);
    }
}

#[test]
fn delete_user_test() {
    use schema::users::dsl::*;
    let conn = establish_connection();
    let mut input = String::new();
    loop {
        println!("Insert name of to delete, \\end to end");
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim_end() == "\\end" {
            break;
        }

        diesel::delete(users.filter(username.like(&input.trim_end())))
            .execute(&conn)
            .expect(&format!("Error while deleting user {}", &input));
        
        input.truncate(0);
    }
}