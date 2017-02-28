use ansi_term::Colour::{Red,Green};

#[derive(Debug)]
pub struct Machine {
    id: String,
    name: String,
    provider: String,
    state: String,
    directory: String
}

impl Machine {
    pub fn from_output_line(line: &str) -> Machine {
        let line = line.to_string();
        let words:Vec<&str> = line.split_whitespace().collect();

        Machine{
            id: words[0].to_string(),
            name: words[1].to_string(),
            provider: words[2].to_string(),
            state: words[3].to_string(),
            directory: words[4].to_string()
        }
    }

    pub fn to_output(&self) {
        println!(
            "{} {} {} ({}) {}",
            Green.paint(self.id.as_str()),
            Green.paint(self.name.as_str()),
            Green.paint(self.provider.as_str()),
            Red.paint(self.state.as_str()),
            Green.paint(self.directory.as_str())
        );
    }
}
