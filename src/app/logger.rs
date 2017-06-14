use ansi_term::Colour::Red;
use ansi_term::Colour::Green;

pub fn info<S>(message: S)
    where S: Into<String>
{
    println!("{}", Green.paint(message.into()));
}

pub fn error<S>(message: S)
    where S: Into<String>
{
    println!("{}", Red.paint(message.into()));
}
