use ansi_term::Colour::Yellow;
use std::io::{stdin, stdout, Write};
use std::result::Result;

use clap::{Arg, App, SubCommand, ArgMatches};

use app::machine::Machine;
use app::vagrant::CmdType;

pub mod logger;
pub mod machine;
pub mod vagrant;


type CmdResult<T> = Result<T, String>;

pub fn init_cli<'a>() -> ArgMatches<'a> {
    App::new("vagment")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("VAGRANT_COMMAND"))
        .arg(Arg::with_name("MACHINE_NUMBER"))
        .subcommand(SubCommand::with_name("dump")
            .arg(Arg::with_name("MACHINE_NUMBER"))
            .about("Dump Vagrant file"))
        .subcommand(SubCommand::with_name("list").about("List available machines"))
        .subcommand(SubCommand::with_name("refresh").about("Clear vagrant cache"))
        .get_matches()
}

pub fn print_list(machines: &[Machine]) {

    let output = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}",
                         "Id",
                         "Name",
                         "Provider",
                         "State",
                         "Path");

    println!("");
    println!("{}", Yellow.paint(output));
    for machine in machines {
        machine.to_output();
    }
    println!("");
}

pub fn dump_configuration(machines: &[Machine], number: u16) -> CmdResult<String> {

    let result = validate_number(machines, number)?;
    let machine = retrieve_machine_by_number(machines, result)?;
    vagrant::dump(machine.get_path())
}

pub fn process_command(machines: &[Machine],
                       command: &str,
                       number: u16)
                       -> CmdResult<String> {

    let result = validate_number(machines, number)?;
    let machine = retrieve_machine_by_number(machines, result)?;
    let commands = list_commands!();

    if !commands.contains(&command) {
        return Err(format!("`{}` is not a valid command! Available commands are {}",
                           command,
                           commands.join(", ")));
    }

    logger::info(format!("Executing command vagrant {} in {}",
                         Yellow.paint(command),
                         Yellow.paint(machine.get_path())));

    if command.needs_machine_up() && !machine.is_running() {
        logger::info("VM is not running, we are going to boot it up");
        let result = vagrant::boot(machine.get_path());
        if result.is_err() {
            return result;
        }
    }

    vagrant::execute(command, machine.get_path())
}

fn validate_number(machines: &[Machine], num: u16) -> CmdResult<u16> {

    let mut number: u16 = num;

    if number == 0 {
        if machines.len() > 1 {
            let input = ask_for_machine_number(machines);
            number = input.parse().unwrap_or(0);
        } else {
            number = 1;
        }
    }

    if number == 0 {
        return Err("Please enter a valid number".to_string());
    }

    Ok(number)
}

fn retrieve_machine_by_number(machines: &[Machine], number: u16) -> CmdResult<&Machine> {
    if number == 0 {
        return Err("Please enter a valid number".to_string());
    }

    let index = (number - 1) as usize;

    if machines.get(index).is_none() {
        return Err("Please enter a valid number".to_string());
    }

    Ok(machines.get(index).unwrap())
}

fn ask_for_machine_number(machines: &[Machine]) -> String {
    print_list(machines);
    print!("{}", Yellow.paint("Please enter a machine number\n-> "));

    let _ = stdout().flush();

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(bytes) => bytes,
        Err(error) => panic!("Could not read input: {}", error),
    };
    input.trim().to_string()
}
