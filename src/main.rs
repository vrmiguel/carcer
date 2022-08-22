mod line;
mod parser;

use line::Line;

fn main() {
    let input = argv::iter()
        .nth(1)
        .expect("No expression supplied!")
        .to_string_lossy();
    match Line::parse(&input) {
        Ok(line) => {
            println!("\"{input}\" is equivalent to \"{line}\"")
        }
        Err(err) => eprintln!("{err}"),
    }
}
