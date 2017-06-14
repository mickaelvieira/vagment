pub trait AppCommand {
    fn needs_machine_up(&self) -> bool;
    fn needs_a_machine(&self) -> bool;
}

impl AppCommand for str {
    fn needs_machine_up(&self) -> bool {
        self == "ssh"
    }

    fn needs_a_machine(&self) -> bool {
        self == "up" ||
        self == "halt" ||
        self == "ssh" ||
        self == "destroy" ||
        self == "status" ||
        self == "suspend" ||
        self == "reload" ||
        self == "resume" ||
        self == "dump" ||
        self == "edit"
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_knows_when_it_needs_the_machine_up() {
        assert!("ssh".needs_machine_up());
        assert!(!"halt".needs_machine_up());
    }

    #[test]
    fn it_knows_when_it_needs_the_machine_number() {
        assert!("up".needs_a_machine());
        assert!("halt".needs_a_machine());
        assert!("ssh".needs_a_machine());
        assert!("destroy".needs_a_machine());
        assert!("status".needs_a_machine());
        assert!("suspend".needs_a_machine());
        assert!("reload".needs_a_machine());
        assert!("resume".needs_a_machine());
        assert!("dump".needs_a_machine());
        assert!("edit".needs_a_machine());
        assert!(!"refresh".needs_a_machine());
        assert!(!"list".needs_a_machine());
    }
}
