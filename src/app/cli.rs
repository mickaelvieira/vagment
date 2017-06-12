
use clap::{Arg, App, SubCommand};

pub fn init<'a, 'b>() -> App<'a, 'b> {
    App::new("vagment")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("VAGRANT_COMMAND"))
        .arg(Arg::with_name("MACHINE_NUMBER"))
        .subcommand(SubCommand::with_name("dump")
                        .arg(Arg::with_name("MACHINE_NUMBER"))
                        .about("Dump Vagrant file"))
        .subcommand(SubCommand::with_name("edit")
                        .arg(Arg::with_name("MACHINE_NUMBER"))
                        .about("Edit Vagrant file"))
        .subcommand(SubCommand::with_name("list").about("List available machines"))
        .subcommand(SubCommand::with_name("refresh").about("Clear vagrant cache"))
}
