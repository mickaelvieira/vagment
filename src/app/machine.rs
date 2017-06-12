use ansi_term::Colour::Green;

#[derive(Debug)]
pub struct Machine {
    id: String,
    name: String,
    provider: String,
    state: String,
    path: String,
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
            path: words[4].to_string(),
        }
    }

    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }

    pub fn get_vagrant_file_path(&self) -> String {
        let mut file = self.path.clone();
        file.push_str("/Vagrantfile");
        file
    }

    pub fn to_output(&self) {
        let output = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}",
                             self.id.as_str(),
                             self.name.as_str(),
                             self.provider.as_str(),
                             self.state.as_str(),
                             self.path.as_str());
        println!("{}", Green.paint(output));
    }

    pub fn is_running(&self) -> bool {
        self.state == "running"
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_retrieves_the_vm_path() {
        let m = Machine::from_output_line("00057e0 default virtualbox aborted /path/to/vm");
        assert_eq!(m.get_path(), "/path/to/vm");
    }

    #[test]
    fn it_retrieves_the_vagrant_file_path() {
        let m = Machine::from_output_line("00057e0 default virtualbox aborted /path/to/vm");
        assert_eq!(m.get_vagrant_file_path(), "/path/to/vm/Vagrantfile");
    }

    #[test]
    fn it_knows_when_the_vm_is_not_running() {
        let m = Machine::from_output_line("00057e0 default virtualbox aborted /path/to/vm");
        assert!(!m.is_running());
    }

    #[test]
    fn it_knows_when_the_vm_is_running() {
        let m = Machine::from_output_line("00057e0 default virtualbox running /path/to/vm");
        assert!(m.is_running());
    }
}
