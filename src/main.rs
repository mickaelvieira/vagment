extern crate clap;
extern crate ansi_term;

extern crate vagment;

use std::process;

use ansi_term::Colour::Yellow;
use std::io::{stdin, stdout, Write};

use vagment::app;
use vagment::app::logger;
use vagment::app::vagrant;
// use vagment::app::machine::Machine;
use vagment::app::machine::Machines;
use vagment::app::command::AppCommand;
use vagment::app::number::AppNumber;
use vagment::app::args::AppArgs;

fn main() {
    let cli = vagment::app::cli::init();
    let matches = cli.get_matches();
    let machines = vagrant::list();

    if machines.len() < 1 {
        logger::error("Could not find any vagrant machines available".to_string());
        process::exit(1);
    }

    let result;
    let mut command = matches.vagrant_command();
    let mut number = matches.machine_number();

    if let Some(subcommand) = matches.subcommand_name() {
        if let Some(matches) = matches.subcommand_matches(subcommand) {
            command = subcommand;
            number = matches.machine_number();
        }
    }

    if command.needs_machine_number() && !number.is_valid() {
        if machines.len() > 1 {
            let list = machines.to_output();
            let input = ask_for_machine_number(list);
            number = input.parse().unwrap_or(0);
        } else {
            number = 1;
        }
    }

    println!("{:?}", command);
    println!("{:?}", number);

    if command.needs_machine_number() && !number.is_valid() {
        logger::error("Unexpected invalid machine number".to_string());
        process::exit(1);
    }

    if command.is_vagrant_command() {
        result = app::process_command(machines, command, number);
    } else {
        result = match command {
            "dump" => app::dump_vagrant_file(machines, number),
            "edit" => app::edit_vagrant_file(machines, number),
            "list" => app::print_list(machines),
            "refresh" => vagrant::refresh(),
            _ => Err(format!("Unexpected invalid command {:?}", command)),
        }
    }

    if result.is_err() {
        logger::error(result.unwrap_err());
    } else {
        logger::info(result.unwrap());
    }
}

fn ask_for_machine_number(list:String) -> String {
    println!("{}", list);
    print!("{}", Yellow.paint("Please enter a machine number\n-> "));

    let _ = stdout().flush();

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(bytes) => bytes,
        Err(error) => panic!("Could not read input: {}", error),
    };
    input.trim().to_string()
}
