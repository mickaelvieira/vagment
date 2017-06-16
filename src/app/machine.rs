#[derive(Debug, PartialEq, Clone)]
pub struct Machine {
    id: String,
    name: String,
    provider: String,
    state: String,
    path: String,
}

impl Machine {
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

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }

    pub fn get_state(&self) -> &str {
        self.state.as_str()
    }

    pub fn get_vagrant_file_path(&self) -> String {
        let mut file = self.path.clone();
        file.push_str("/Vagrantfile");
        file
    }

    pub fn is_running(&self) -> bool {
        self.state == "running"
    }
}

pub trait Machines {
    fn get_machine_by_number(&self, number: u16) -> Option<&Machine>;
    fn get_running_machines(&self) -> Vec<Machine>;
    fn get_stopped_machines(&self) -> Vec<Machine>;
}

impl Machines for Vec<Machine> {
    fn get_machine_by_number(&self, number: u16) -> Option<&Machine> {
        if number == 0 {
            return None;
        }

        let index = (number - 1) as usize;
        self.get(index)
    }

    fn get_running_machines(&self) -> Vec<Machine> {
        self.iter()
            .filter(|m| m.is_running())
            .map(|m| m.clone())
            .collect()
    }

    fn get_stopped_machines(&self) -> Vec<Machine> {
        self.iter()
            .filter(|m| !m.is_running())
            .map(|m| m.clone())
            .collect()
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
    fn it_retrieves_the_vm_name() {
        let m = Machine::from_output_line("00057e0 default virtualbox aborted /path/to/vm");
        assert_eq!(m.get_name(), "default");
    }

    #[test]
    fn it_retrieves_the_vm_state() {
        let m = Machine::from_output_line("00057e0 default virtualbox aborted /path/to/vm");
        assert_eq!(m.get_state(), "aborted");
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

    #[test]
    fn it_retrieves_a_vm_by_its_number() {
        let m1 = Machine::from_output_line("00057e0 default virtualbox running /path/to/vm1");
        let m2 = Machine::from_output_line("45457b5 default virtualbox poweroff /path/to/vm2");

        let mut m = Vec::new();
        m.push(m1);
        m.push(m2);

        assert_eq!(m.get_machine_by_number(1), Some(&m[0]));
        assert_eq!(m.get_machine_by_number(2), Some(&m[1]));
        assert_eq!(m.get_machine_by_number(0), None);
        assert_eq!(m.get_machine_by_number(3), None);
    }

    #[test]
    fn it_returns_a_vector_with_only_the_running_machines() {
        let m0 = Machine::from_output_line("00057e0 default virtualbox running /path/to/vm1");
        let m1 = Machine::from_output_line("00057e0 default virtualbox running /path/to/vm1");
        let m2 = Machine::from_output_line("45457b5 default virtualbox poweroff /path/to/vm2");

        let mut m = Vec::new();
        m.push(m1);
        m.push(m2);

        let r = m.get_running_machines();
        assert_eq!(r, vec![m0]);
    }

    #[test]
    fn it_returns_a_vector_with_only_the_non_running_machines() {
        let m0 = Machine::from_output_line("45457b5 default virtualbox poweroff /path/to/vm2");
        let m1 = Machine::from_output_line("00057e0 default virtualbox running /path/to/vm1");
        let m2 = Machine::from_output_line("45457b5 default virtualbox poweroff /path/to/vm2");

        let mut m = Vec::new();
        m.push(m1);
        m.push(m2);

        let r = m.get_stopped_machines();
        assert_eq!(r, vec![m0]);
    }
}
