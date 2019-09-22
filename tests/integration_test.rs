use memebot_backend::*;
use memebot_backend::models::*;
use diesel::prelude::*;

#[test]
fn load_test() {
    use memebot_backend::schema::users::dsl::*;

    let conn = establish_connection();
    let results = users.filter(testbool.eq(true))
        .limit(5)
        .load::<User>(&conn)
        .expect("Error loading users!");
}