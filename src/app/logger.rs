use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Style;

pub fn info<S>(message: S)
    where S: Into<String>
{
    let message = message.into();
    if !message.is_empty() {
        println!("{} {}", Style::new().bold().fg(Green).paint("Info:"), message);
    }
}

pub fn error<S>(message: S)
    where S: Into<String>
{
    let message = message.into();
    if !message.is_empty() {
        println!("{} {}", Style::new().bold().fg(Red).paint("Error:"), message);
    }
}
