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

    let command = matches.vagrant_command();
    let mut number = matches.machine_number();

    if machines.len() < 1 {
        logger::error("Could not find any vagrant machines available".to_string());
        process::exit(1);
    }

    if let Some(subcommand) = matches.subcommand_name() {
        if let Some(matches) = matches.subcommand_matches(subcommand) {
            number = matches.machine_number();
        }
    }

    if matches.is_present("dump") {
        logger::log_result(app::dump_vagrant_file(&machines, number));
    } else if matches.is_present("edit") {
        logger::log_result(app::edit_vagrant_file(&machines, number));
    } else if matches.is_present("list") {
        app::print_list(&machines);
    } else if matches.is_present("refresh") {
        logger::info("Refreshing machine listing");
        logger::log_result(vagrant::refresh());
    } else if command.is_valid() {
        logger::log_result(app::process_command(&machines, command, number));
    }
}
