use clap::ArgMatches;
use app::cli::ARG_NUMBER;

pub trait AppArgs {
    fn parse_machine_number(&self) -> u16;
}

impl<'a> AppArgs for ArgMatches<'a> {
    fn parse_machine_number(&self) -> u16 {
        self.value_of(ARG_NUMBER)
            .unwrap_or("0")
            .parse()
            .unwrap_or(0)
    }
}
