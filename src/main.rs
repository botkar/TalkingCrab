use ferris_says::say;
use std::io::{stdout, BufWriter};
use ansi_term::Color;
use random::Source;

fn main() {
    let input_text = Color::Yellow.bold();
    println!("{}", input_text.paint("What would you like mr crab to say? "));
    let mut msg = String::new();
    let _input = std::io::stdin().read_line(&mut msg);
    let stdout = stdout();
    let width = msg.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&msg, width, &mut writer).unwrap();
}
