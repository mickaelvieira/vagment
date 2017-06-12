use clap::ArgMatches;
use app::cli::ARG_COMMAND;
use app::cli::ARG_NUMBER;

pub trait AppArgs {
    fn has_subcommand(&self) -> bool;
    fn machine_number(&self) -> u16;
    fn vagrant_command(&self) -> &str;
}

impl<'a> AppArgs for ArgMatches<'a> {
    fn has_subcommand(&self) -> bool {
        if self.is_present("dump") || self.is_present("edit") || self.is_present("list") ||
           self.is_present("refresh") {
            return true;
        }

        false
    }

    fn machine_number(&self) -> u16 {
        self.value_of(ARG_NUMBER)
            .unwrap_or("0")
            .parse()
            .unwrap_or(0)
    }

    fn vagrant_command(&self) -> &str {
        self.value_of(ARG_COMMAND).unwrap_or("")
    }
}
