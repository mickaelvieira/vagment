pub trait AppCommand {
    fn needs_machine_up(&self) -> bool;
    fn is_vagrant_command(&self) -> bool;
}

impl AppCommand for str {
    fn needs_machine_up(&self) -> bool {
        self == "ssh"
    }

    fn is_vagrant_command(&self) -> bool {
        let commands = list_commands!();
        commands.contains(&self)
    }
}
