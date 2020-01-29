use memebot_backend::*;
use memebot_backend::cli::*;

fn main() {
    let conn = establish_connection();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    switcher(&conn, &mut stdin.lock(), &mut stdout);
}