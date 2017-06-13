use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;

#[derive(Debug, PartialEq)]
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

    pub fn to_output(&self) -> String {
        let output = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}",
                             self.id.as_str(),
                             self.name.as_str(),
                             self.provider.as_str(),
                             self.state.as_str(),
                             self.path.as_str());
        format!("{}", Green.paint(output))
    }

    pub fn is_running(&self) -> bool {
        self.state == "running"
    }
}

pub trait Machines {
    fn get_machine_by_number(&self, number: u16) -> Option<&Machine>;
    fn to_output(&self) -> String;
}

impl Machines for Vec<Machine> {
    fn get_machine_by_number(&self, number: u16) -> Option<&Machine> {
        if number == 0 {
            return None;
        }

        let index = (number - 1) as usize;
        let machine = self.get(index);

        if machine.is_none() {
            None
        } else {
            Some(machine.unwrap())
        }
    }

    fn to_output(&self) -> String {
        let o = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}",
                             "Id",
                             "Name",
                             "Provider",
                             "State",
                             "Path");

        let mut lines = Vec::new();
        let header = format!("{}", Yellow.paint(o));

        lines.push(header);
        for machine in self {
            lines.push(machine.to_output());
        }

        lines.join("\n")
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
    //
    // #[test]
    // fn it_retrieves_a_vm_by_its_number() {
    //     let m1 = Machine::from_output_line("00057e0 default virtualbox running /path/to/vm1");
    //     let m2 = Machine::from_output_line("45457b5 default virtualbox poweroff /path/to/vm2");
    //
    //     let m: Vec<Machine> = vec![m1, m2];
    //
    //     assert_eq!(m.get_machine_by_number(1), Some(&m1));
    //     // assert_eq!(m.get_machine_by_number(2), Some(&m2));
    //     // assert_eq!(m.get_machine_by_number(0), None);
    //     // assert_eq!(m.get_machine_by_number(3), None);
    //     // assert_eq!(m, vec![&m1, &m2]);
    // }
}
