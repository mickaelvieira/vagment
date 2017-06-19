extern crate clap;
extern crate ansi_term;
extern crate vagment;

use std::io;
use std::io::Write;
use std::io::stdout;

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
use vagment::app::errors::CommandError;

fn main() {
    let mut cli = vagment::app::cli::init();
    let matches = cli.clone().get_matches();
    let machines = vagrant::get_machine_list();

    match parse(&matches, &machines) {
        Some((command, number)) => {
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
        None => {
            let _ = cli.print_help();
            println!("");
            std::process::exit(1);
        }
    }
}

fn parse(matches: &ArgMatches, machines: &[Machine]) -> Option<(String, u16)> {
    let mut number = 0;
    let mut command = String::from("");

    if let Some(subcommand) = matches.subcommand_name() {
        command = subcommand.to_string();
        if let Some(matches) = matches.subcommand_matches(subcommand) {
            number = matches.parse_machine_number();
        }
    }

    if command.needs_a_machine() && !number.is_valid() {
        number = if machines.len() > 1 {
            ask_for_machine_number(machines)
        } else {
            1
        };
    }

    if command.is_empty() {
        None
    } else {
        Some((command, number))
    }
}

fn ask_for_machine_number(machines: &[Machine]) -> u16 {
    println!("{}", formatter::format(machines));
    print!("{}", Yellow.paint("Please enter a machine number: "));

    let _ = stdout().flush();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(bytes) => bytes,
        Err(error) => panic!("Could not read input: {}", error),
    };
    input.trim().to_string().parse().unwrap_or(0)
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn run(command: String, number: u16, machines: Vec<Machine>) -> Result<String, CommandError> {
    if machines.len() < 1 {
        return Err(CommandError::NoMachinesFound);
    }

    if command.needs_a_machine() {
        let search = machines.get_machine_by_number(number);
        if search.is_none() {
            return Err(CommandError::InvalidNumber(number));
        }

        let machine = search.unwrap();

        if command.needs_machine_up() && !machine.is_running() {
            logger::info("VM is not running, we are going to boot it up");
            if vagrant::execute("up".to_string(), machine.get_path()).is_err() {
                return Err(CommandError::MachineNotBootable);
            }
        }

        match command.as_str() {
            "up" | "ssh" | "halt" | "status" | "resume" | "reload" | "suspend" | "destroy" => {
                vagrant::execute(command, machine.get_path())
            }
            "dump" => vagrant::dump(machine.get_path(), machine.get_vagrant_file_path()),
            "edit" => vagrant::edit(machine.get_path(), machine.get_vagrant_file_path()),
            _ => Err(CommandError::InvalidCommand(command)),
        }
    } else {
        match command.as_str() {
            "list" => vagrant::print_list(&machines),
            "refresh" => vagrant::refresh(),
            "shutdown" => vagrant::shutdown(machines.get_running_machines()),
            "bootup" => vagrant::bootup(machines.get_stopped_machines()),
            _ => Err(CommandError::InvalidCommand(command)),
        }
    }
}
