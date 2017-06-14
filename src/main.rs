extern crate clap;
extern crate ansi_term;

extern crate vagment;

use std::process;

use ansi_term::Colour::Yellow;
use std::io::{stdin, stdout, Write};

use vagment::app::logger;
use vagment::app::vagrant;
use vagment::app::machine::Machines;
use vagment::app::command::AppCommand;
use vagment::app::number::AppNumber;
use vagment::app::args::AppArgs;

fn main() {
    let cli = vagment::app::cli::init();
    let matches = cli.get_matches();
    let machines = vagrant::get_machine_list();

    if machines.len() < 1 {
        logger::error("Could not find any vagrant machines available".to_string());
        process::exit(1);
    }

    let result;
    let mut number = 0;
    let mut command = "";

    if let Some(subcommand) = matches.subcommand_name() {
        if let Some(matches) = matches.subcommand_matches(subcommand) {
            command = subcommand;
            number = matches.machine_number();
        }
    }

    if command.needs_a_machine() {
        if !number.is_valid() {
            if machines.len() > 1 {
                let list = machines.to_output();
                let input = ask_for_machine_number(list);
                number = input.parse().unwrap_or(0);
            } else {
                number = 1;
            }
        }

        let search = machines.get_machine_by_number(number);

        if search.is_none() {
            logger::error("Unexpected invalid machine number".to_string());
            process::exit(1);
        }

        let machine = search.unwrap();

        if command.needs_machine_up() && !machine.is_running() {
            logger::info("VM is not running, we are going to boot it up");
            let r = vagrant::boot(machine.get_path());
            if r.is_err() {
                logger::error("Could not boot the machine".to_string());
                process::exit(1);
            }
        }

        result = match command {
            "up" => vagrant::execute(command, machine.get_path()),
            "ssh" => vagrant::execute(command, machine.get_path()),
            "halt" => vagrant::execute(command, machine.get_path()),
            "status" => vagrant::execute(command, machine.get_path()),
            "resume" => vagrant::execute(command, machine.get_path()),
            "reload" => vagrant::execute(command, machine.get_path()),
            "suspend" => vagrant::execute(command, machine.get_path()),
            "destroy" => vagrant::execute(command, machine.get_path()),
            "dump" => vagrant::dump(machine.get_path(), machine.get_vagrant_file_path()),
            "edit" => vagrant::edit(machine.get_path(), machine.get_vagrant_file_path()),
            _ => Err(format!("Unexpected invalid command 1 {:?}", command)),
        }
    } else {
        result = match command {
            "list" => vagrant::print_list(machines),
            "refresh" => vagrant::refresh(),
            "shutdown" => vagrant::shutdown(machines.get_running_machines()),
            _ => Err(format!("Unexpected invalid command 2 {:?}", command)),
        }
    }

    if result.is_err() {
        logger::error(result.unwrap_err());
    } else {
        logger::info(result.unwrap());
    }
}

fn ask_for_machine_number(list: String) -> String {
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
