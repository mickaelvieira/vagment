extern crate clap;
extern crate ansi_term;

extern crate vagment;

use clap::ArgMatches;

use std::process;

use vagment::app;
use vagment::app::logger;
use vagment::app::vagrant;
use vagment::app::cli;

fn main() {
    let library = cli::init();
    let matches = library.get_matches();
    let machines = vagrant::list();

    let command = parse_vagrant_command(&matches);
    let mut number = parse_machine_number(&matches);

    if machines.len() < 1 {
        logger::error("Could not find any vagrant machines available".to_string());
        process::exit(1);
    }

    if matches.is_present("dump") {
        if let Some(matches) = matches.subcommand_matches("dump") {
            number = parse_machine_number(&matches);
        }
        logger::log_result(app::dump_vagrant_file(&machines, number));
    } else if matches.is_present("edit") {
        if let Some(matches) = matches.subcommand_matches("dump") {
            number = parse_machine_number(&matches);
        }
        logger::log_result(app::edit_vagrant_file(&machines, number));
    } else if matches.is_present("list") {
        app::print_list(&machines);
    } else if matches.is_present("refresh") {
        logger::info("Refreshing machine listing");
        logger::log_result(vagrant::refresh());
    } else if command != "" {
        logger::log_result(app::process_command(&machines, command, number));
    }
}

fn parse_machine_number(matches: &ArgMatches) -> u16 {
    matches
        .value_of("MACHINE_NUMBER")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0)
}

fn parse_vagrant_command<'a>(matches: &'a ArgMatches) -> &'a str {
    matches.value_of("VAGRANT_COMMAND").unwrap_or("")
}
