use clap::{App, Arg, SubCommand};

pub const ARG_NUMBER: &str = "machine_number";

pub fn init<'a, 'b>() -> App<'a, 'b> {
    App::new("vagment")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("up")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Run up command against the given machine"),
        )
        .subcommand(
            SubCommand::with_name("halt")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Run halt command against the given machine"),
        )
        .subcommand(
            SubCommand::with_name("ssh")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Run ssh command against the given machine"),
        )
        .subcommand(
            SubCommand::with_name("destroy")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Run destroy command against the given machine"),
        )
        .subcommand(
            SubCommand::with_name("status")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Run status command against the given machine"),
        )
        .subcommand(
            SubCommand::with_name("suspend")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Run suspend command against the given machine"),
        )
        .subcommand(
            SubCommand::with_name("reload")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Run reload command against the given machine"),
        )
        .subcommand(
            SubCommand::with_name("resume")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Run resume command against the given machine"),
        )
        .subcommand(
            SubCommand::with_name("dump")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Dump Vagrant file"),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .arg(Arg::with_name(ARG_NUMBER))
                .about("Edit Vagrant file"),
        )
        .subcommand(SubCommand::with_name("list").about("List available machines"))
        .subcommand(SubCommand::with_name("refresh").about("Clear vagrant cache"))
        .subcommand(SubCommand::with_name("shutdown").about("Shutdown all running machines"))
        .subcommand(SubCommand::with_name("bootup").about("Boot up all stopped machines"))
}
