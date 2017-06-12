pub trait AppCommand {
    fn needs_machine_up(&self) -> bool;
    fn is_valid(&self) -> bool;
}

impl AppCommand for str {
    fn needs_machine_up(&self) -> bool {
        self == "ssh"
    }

    fn is_valid(&self) -> bool {
        let commands = list_commands!();
        commands.contains(&self)
    }
}
