use ansi_term::Colour::Red;
use ansi_term::Colour::Green;

pub fn info(message: &str) {
    println!("{}", Green.paint(message));
}

pub fn error(message: &str) {
    println!("{}", Red.paint(message));
}
