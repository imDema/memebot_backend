use memebot_backend::cli::*;

use std::io::{BufReader};

#[test]
fn cli_test() {
    let file_in = std::fs::File::open("tests/testinput.txt").unwrap();
    let mut buf_in = BufReader::new(file_in);

    let mut stdout = std::io::stdout();

    switcher(&mut buf_in, &mut stdout);
}