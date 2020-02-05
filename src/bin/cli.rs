
use memebot_backend::cli::*;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    switcher(&mut stdin.lock(), &mut stdout);
}