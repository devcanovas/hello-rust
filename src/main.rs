use ferris_says::say;
use std::io::{stdout, BufWriter, Stdout, StdoutLock};

fn main() {
    let stdout: Stdout = stdout();
    let message: String = String::from("Hello fellow Rustceans!");
    let width: usize = message.chars().count();

    let mut writer: BufWriter<StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

