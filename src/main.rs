extern crate clap;
extern crate ansi_term;
extern crate vagment;

use std::io::{stdin, stdout, Write};

use clap::ArgMatches;

use ansi_term::Colour::Yellow;

use vagment::app::logger;
use vagment::app::vagrant;
use vagment::app::formatter;
use vagment::app::machine::Machine;
use vagment::app::machine::Machines;
use vagment::app::command::AppCommand;
use vagment::app::number::AppNumber;
use vagment::app::args::AppArgs;

fn main() {
    let cli = vagment::app::cli::init();
    let matches = cli.get_matches();
    let machines = vagrant::get_machine_list();

    match parse(matches, &machines) {
        Ok((command, number)) => {
            match run(command, number, machines) {
                Ok(m) => {
                    logger::info(m);
                    std::process::exit(0);
                }
                Err(e) => {
                    logger::error(e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            logger::error(e);
            std::process::exit(1);
        }
    }
}

fn parse(matches: ArgMatches, machines: &Vec<Machine>) -> Result<(String, u16), String> {
    let mut number = 0;
    let mut command = String::from("");

    if let Some(subcommand) = matches.subcommand_name() {
        command = subcommand.to_string();
        if let Some(matches) = matches.subcommand_matches(subcommand) {
            number = matches.parse_machine_number();
        }
    }

    if command.needs_a_machine() && !number.is_valid() {
        if machines.len() > 1 {
            number = ask_for_machine_number(&machines);
        } else {
            number = 1;
        }
    }

    if command.is_empty() {
        Err("Please provide a command to run. For more information try --help".to_string())
    } else {
        Ok((command, number))
    }
}

fn ask_for_machine_number(machines: &Vec<Machine>) -> u16 {
    println!("{}", formatter::format(&machines));
    print!("{}", Yellow.paint("Please enter a machine number: "));

    let _ = stdout().flush();

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(bytes) => bytes,
        Err(error) => panic!("Could not read input: {}", error),
    };
    input.trim().to_string().parse().unwrap_or(0)
}

fn run(cmd: String, number: u16, machines: Vec<Machine>) -> Result<String, String> {
    let command = cmd.as_str();
    if machines.len() < 1 {
        return Err("Could not find any vagrant machines available".to_string());
    }

    if command.needs_a_machine() {
        let search = machines.get_machine_by_number(number);
        if search.is_none() {
            return Err("Unexpected invalid machine number".to_string());
        }

        let machine = search.unwrap();

        if command.needs_machine_up() && !machine.is_running() {
            logger::info("VM is not running, we are going to boot it up");
            if let Err(_) = vagrant::boot(machine.get_path()) {
                return Err("Could not boot the machine".to_string());
            }
        }

        match command {
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
            _ => Err(format!("Invalid command {:?}", command)),
        }
    } else {
        match command {
            "list" => vagrant::print_list(machines),
            "refresh" => vagrant::refresh(),
            "shutdown" => vagrant::shutdown(machines.get_running_machines()),
            _ => Err(format!("Invalid command {:?}", command)),
        }
    }
}
