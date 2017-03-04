use ansi_term::Colour::Green;

#[derive(Debug)]
pub struct Machine {
    id: String,
    name: String,
    provider: String,
    state: String,
    directory: String,
}

impl Machine {
    // Use of the Into Trait https://doc.rust-lang.org/nightly/core/convert/trait.Into.html
    // Let the caller to pass either a String or &str type
    // pub fn from_output_line<S: Into<String>>(line: S) -> Machine {
    pub fn from_output_line<S>(line: S) -> Machine
        where S: Into<String>
    {
        let line = line.into();
        let words: Vec<&str> = line.split_whitespace().collect();

        Machine {
            id: words[0].to_string(),
            name: words[1].to_string(),
            provider: words[2].to_string(),
            state: words[3].to_string(),
            directory: words[4].to_string(),
        }
    }

    pub fn get_path(&self) -> &str {
        self.directory.as_str()
    }

    pub fn to_output(&self) {
        let output = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}",
                             self.id.as_str(),
                             self.name.as_str(),
                             self.provider.as_str(),
                             self.state.as_str(),
                             self.directory.as_str());
        println!("{}", Green.paint(output));
    }
}
