use ansi_term::Colour::Yellow;
use std::io::{stdin, stdout, Write};
use clap::{Arg, App, SubCommand, ArgMatches};

use app::machine::Machine;

pub mod logger;
pub mod machine;
pub mod vagrant;

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

pub fn dump_config(machine: &Machine) -> Result<String, String> {
    vagrant::dump(machine.get_path())
}

pub fn validate_number(machines: &Vec<Machine>, num: u16) -> Result<u16, String> {

    let mut number: u16 = num;

    if number == 0 {
        if machines.len() > 1 {
            let input = ask_machine_number(&machines);
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

pub fn retrieve_machine_by_number(machines: &Vec<Machine>,
                                  number: u16)
                                  -> Result<&Machine, String> {

    if number == 0 {
        return Err("Please enter a valid number".to_string());
    }

    let index = (number - 1) as usize;

    if machines.get(index).is_none() {
        return Err("Please enter a valid number".to_string());
    }

    Ok(machines.get(index).unwrap())
}

pub fn process_command(machine: &Machine, command: &str) -> Result<String, String> {

    let commands = list_commands!();

    if !commands.contains(&command) {
        return Err(format!("`{}` is not a valid command! Available commands are {}",
                           command,
                           commands.join(", ")));
    }

    logger::info(format!("Executing command vagrant {} in {}",
                         Yellow.paint(command),
                         Yellow.paint(machine.get_path())));

    vagrant::execute(command, machine.get_path())
}

pub fn print_list(machines: &Vec<Machine>) {

    let output = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}",
                         "Id",
                         "Name",
                         "Provider",
                         "State",
                         "Path");

    print!("\n");
    println!("{}", Yellow.paint(output));
    for machine in machines {
        machine.to_output();
    }
    print!("\n");
}

fn ask_machine_number(machines: &Vec<Machine>) -> String {
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
