#[macro_use]
extern crate clap;
extern crate ansi_term;

#[macro_use]
extern crate vagment;

use clap::{Arg, App, SubCommand, ArgMatches};

use ansi_term::Colour::Yellow;

use std::io::{stdin, stdout, Write};

use vagment::app::vagrant;
use vagment::app::machine::Machine;

fn init_cli<'a>() -> ArgMatches<'a> {
    App::new("vagment")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(Arg::with_name("VAGRANT_COMMAND"))
        .arg(Arg::with_name("MACHINE_NUMBER"))
        .subcommand(SubCommand::with_name("list")
            .about("List available machines")
        )
        .subcommand(SubCommand::with_name("refresh")
            .about("Clear vagrant cache")
        )
        .get_matches()
}

pub fn prompt_machine_number(machines: &Vec<Machine>) -> String {
    print_list(machines);
    print!("{}", Yellow.paint("Please enter a machine number
-> "));

    let _ = stdout().flush();

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(bytes) => bytes,
        Err(error) => panic!("Could nout read input: {}", error)
    };
    input.trim().to_string()
}

fn process_command(machines: &Vec<Machine>, command: &str, num: u16) {

    let mut number = num;

    if machines.len() < 1 {
        panic!("Could not find any vagrant machines available");
    }

    if number == 0 {
        if machines.len() > 1 {
            let input = prompt_machine_number(&machines);
            number = input.parse().unwrap_or(0);
            // @TODO need to handle the bad input properly
        } else {
            number = 1;
        }
    }

    let index = (number - 1) as usize;

    if machines.get(index).is_none() {
        panic!("Please enter a valid number");
    }

    let machine = machines.get(index).unwrap();
    let commands = list_commands!();

    if commands.contains(&command) {
        vagrant::execute_command(command, machine.get_path());
    } else {
        let message = format!("`{}` is not a valid command! Available commands are {}", command, commands.join(", "));
        panic!(message);
    }
}

pub fn print_list(machines: &Vec<Machine>) {
    let output = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}", "Id", "Name", "Provider", "State", "Path");

    print!("\n");
    println!("{}", Yellow.paint(output));
    for machine in machines {
        machine.to_output();
    }
    print!("\n");
}

fn main() {
    let matches = init_cli();
    let machines = vagrant::get_machines();

    if matches.is_present("list") {
        print_list(&machines);
    } else if matches.is_present("refresh") {
        vagrant::refresh_list();
    } else {
        let cmd: &str = matches.value_of("VAGRANT_COMMAND")
            .unwrap_or("");

        let num: u16 = matches.value_of("MACHINE_NUMBER")
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);

        process_command(&machines, cmd, num);
    }
}
