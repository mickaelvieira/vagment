#[macro_use]
extern crate clap;
extern crate ansi_term;

#[macro_use]
extern crate vagment;

use clap::{Arg, App, SubCommand, ArgMatches};

use vagment::app::vagrant::*;
use vagment::app::machine::Machine;

fn init_cli<'a>() -> ArgMatches<'a> {
    App::new("vagmatchesent")
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

fn execute_vagrant_command(machines: &Vec<Machine>, command: &str, num: u16) {

    let mut number = num;

    if machines.len() < 1 {
        panic!("Could not find any vagrant machines available");
    }

    if number == 0 {
        if machines.len() > 1 {
            let input = prompt_machine_number(&machines);
            number = input.parse().unwrap_or(0);
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
        execute_command(command, machine.get_path());
    } else {
        let message = format!("`{}` is not a valid command! Available commands are {}", command, commands.join(", "));
        panic!(message);
    }
}

fn main() {
    let matches = init_cli();
    let machines = get_machines();

    if matches.is_present("list") {
        print_machine_list(&machines);
    } else if matches.is_present("refresh") {
        refresh_machine_list();
    } else {

        let cmd: &str = matches.value_of("VAGRANT_COMMAND")
            .unwrap_or("");

        let num: u16 = matches.value_of("MACHINE_NUMBER")
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);

        execute_vagrant_command(&machines, cmd, num);
    }
}
