#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate vagment;

use clap::{Arg, App, SubCommand};
use vagment::app::vagrant::*;

fn main() {
    let m = App::new("vagment")
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
        .get_matches();

    let machines = get_machines();

    println!("Number of machines {}", machines.len());

    if m.is_present("list") {
        print_machine_list(&machines);
    } else if m.is_present("refresh") {
        refresh_machine_list();
    } else {

        let mut num: u16 = m.value_of("MACHINE_NUMBER")
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);

        if machines.len() < 1 {
            panic!("Could not find any vagrant machines available");
        }

        if num == 0 {
            if machines.len() > 1 {
                let input = prompt_machine_number(&machines);
                num = input.parse().unwrap_or(0);
            } else {
                num = 1;
            }
        }

        let index = (num - 1) as usize;

        if machines.get(index).is_none() {
            panic!("Please enter a valid number");
        }

        let machine = machines.get(index).unwrap();
        let cmd = m.value_of("VAGRANT_COMMAND").unwrap_or("Unknown");
        let path = machine.get_path();

        execute_command(cmd, path);
    }
}
