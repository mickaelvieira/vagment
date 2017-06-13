pub trait AppCommand {
    fn needs_machine_up(&self) -> bool;
    fn needs_machine_number(&self) -> bool;
    fn is_vagrant_command(&self) -> bool;
}

impl AppCommand for str {
    fn needs_machine_up(&self) -> bool {
        self == "ssh"
    }

    fn needs_machine_number(&self) -> bool {
        self.is_vagrant_command() || self == "dump" || self == "edit"
    }

    fn is_vagrant_command(&self) -> bool {
        let commands = list_commands!();
        commands.contains(&self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_knows_when_it_is_a_vagrant_sub_command() {
        assert!("up".is_vagrant_command());
        assert!("halt".is_vagrant_command());
        assert!("ssh".is_vagrant_command());
        assert!("destroy".is_vagrant_command());
        assert!("status".is_vagrant_command());
        assert!("suspend".is_vagrant_command());
        assert!("reload".is_vagrant_command());
        assert!("resume".is_vagrant_command());
        assert!(!"version".is_vagrant_command());
    }

    #[test]
    fn it_knows_when_it_needs_the_machine_up() {
        assert!("ssh".needs_machine_up());
        assert!(!"halt".needs_machine_up());
    }

    #[test]
    fn it_knows_when_it_needs_the_machine_number() {
        assert!("up".needs_machine_number());
        assert!("halt".needs_machine_number());
        assert!("ssh".needs_machine_number());
        assert!("destroy".needs_machine_number());
        assert!("status".needs_machine_number());
        assert!("suspend".needs_machine_number());
        assert!("reload".needs_machine_number());
        assert!("resume".needs_machine_number());
        assert!("dump".needs_machine_number());
        assert!("edit".needs_machine_number());
        assert!(!"refresh".needs_machine_number());
        assert!(!"list".needs_machine_number());
    }
}
