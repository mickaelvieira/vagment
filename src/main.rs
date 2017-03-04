#[macro_use]
extern crate clap;
extern crate ansi_term;

#[macro_use]
extern crate vagment;

use clap::ArgMatches;

use std::process;

use vagment::app;
use vagment::app::logger;
use vagment::app::vagrant;

fn main() {
    let matches = app::init_cli();
    let machines = vagrant::list();

    let command = parse_vagrant_command(&matches);
    let number = parse_machine_number(&matches);

    if machines.len() < 1 {
        logger::error("Could not find any vagrant machines available".to_string());
        process::exit(1);
    }

    if matches.is_present("dump") {
        let result = app::validate_number(&machines, number);
        if result.is_ok() {
            let number = result.unwrap();
            let result = app::retrieve_machine_by_number(&machines, number);
            if result.is_ok() {
                let machine = result.unwrap();
                logger::log_result(app::dump_config(&machine));
            } else {
                logger::error(result.unwrap_err());
            }
        } else {
            logger::error(result.unwrap_err());
        }
    } else if matches.is_present("list") {
        app::print_list(&machines);
    } else if matches.is_present("refresh") {
        logger::info("Refreshing machine listing");
        logger::log_result(vagrant::refresh());
    } else {
        let result = app::validate_number(&machines, number);
        if result.is_ok() {
            let number = result.unwrap();
            let result = app::retrieve_machine_by_number(&machines, number);
            if result.is_ok() {
                let machine = result.unwrap();
                logger::log_result(app::process_command(&machine, command));
            } else {
                logger::error(result.unwrap_err());
            }
        } else {
            logger::error(result.unwrap_err());
        }
    }
}

fn parse_machine_number<'a>(matches: &'a ArgMatches) -> u16 {
    matches.value_of("MACHINE_NUMBER")
        .unwrap_or("0")
        .parse()
        .unwrap_or(0)
}

fn parse_vagrant_command<'a>(matches: &'a ArgMatches) -> &'a str {
    matches.value_of("VAGRANT_COMMAND").unwrap_or("")
}
