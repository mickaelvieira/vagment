extern crate clap;
extern crate ansi_term;

extern crate vagment;

use std::process;

use vagment::app;
use vagment::app::logger;
use vagment::app::vagrant;
use vagment::app::command::AppCommand;
use vagment::app::args::AppArgs;

fn main() {
    let cli = vagment::app::cli::init();
    let matches = cli.get_matches();
    let machines = vagrant::list();

    if machines.len() < 1 {
        logger::error("Could not find any vagrant machines available".to_string());
        process::exit(1);
    }

    let command = matches.vagrant_command();
    let mut number = matches.machine_number();

    if let Some(subcommand) = matches.subcommand_name() {
        if let Some(matches) = matches.subcommand_matches(subcommand) {
            number = matches.machine_number();
        }
    }

    let result;

    if matches.is_present("dump") {
        result = app::dump_vagrant_file(&machines, number);
    } else if matches.is_present("edit") {
        result = app::edit_vagrant_file(&machines, number);
    } else if matches.is_present("list") {
        result = app::print_list(&machines);
    } else if matches.is_present("refresh") {
        result = vagrant::refresh();
    } else if command.is_vagrant_command() {
        result = app::process_command(&machines, command, number);
    } else {
        result = Ok(String::new());
    }

    if result.is_err() {
        logger::error(result.unwrap_err());
    } else {
        logger::info(result.unwrap());
    }
}
