use memebot_backend::*;
use common::*;

pub mod common;

const HELPMESSAGE: &str = "COMMANDS:
adduser USERNAME\t\taddmeme IMAGE AUTHORID
upvote MEMEID FROMUSERID\tdownvote MEMEID FROMUSERID
delmeme MEMEID
print\tprintmeta\tprintuser
help";

#[test]
fn test_switcher() {
    let mut input = String::new();
    let conn = establish_connection();

    println!("{}", HELPMESSAGE);

    loop {
        print!("memebot_backend > ");
        std::io::Write::flush(&mut std::io::stdout()).expect("stdout flush failed");
        let line_read = std::io::stdin().read_line(&mut input).unwrap();
        if line_read == 0 {
            break;
        }
        let mut words = input[..input.len()-1].split(' ');

        let command_result = match words.next() {
            Some("adduser") => create_user_test(&conn, words),
            Some("addmeme") => create_meme_test(&conn, words),
            Some("upvote") => upvote_test(&conn, words),
            Some("downvote") => downvote_test(&conn, words),
            Some("delmeme") => delete_meme_test(&conn, words),
            Some("print") => Ok(print_test(&conn)),
            Some("printuser") => Ok(print_users_test(&conn)),
            Some("printmeta") => Ok(print_meta_test(&conn)),
            Some("help") => Ok(println!("{}", HELPMESSAGE)),
            Some("exit") => break,
            _ => Err("Invalid command\n"),
        };

        if let Err(why) = command_result {
            eprintln!("{}", why);
        }

        input.truncate(0);
    }
}
