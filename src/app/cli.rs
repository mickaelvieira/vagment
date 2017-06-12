
use clap::{Arg, App, SubCommand};

pub const ARG_COMMAND: &str = "vagrant_command";
pub const ARG_NUMBER: &str = "machine_number";

pub fn init<'a, 'b>() -> App<'a, 'b> {
    App::new("vagment")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name(ARG_COMMAND))
        .arg(Arg::with_name(ARG_NUMBER))
        .subcommand(SubCommand::with_name("dump")
            .arg(Arg::with_name(ARG_NUMBER))
            .about("Dump Vagrant file"))
        .subcommand(SubCommand::with_name("edit")
            .arg(Arg::with_name(ARG_NUMBER))
            .about("Edit Vagrant file"))
        .subcommand(SubCommand::with_name("list").about("List available machines"))
        .subcommand(SubCommand::with_name("refresh").about("Clear vagrant cache"))
}
