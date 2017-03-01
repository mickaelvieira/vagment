#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate vagment;

use ansi_term::Colour::Yellow;

use clap::{Arg, App, SubCommand};
use std::process::Command;
use vagment::app::Machine;

fn get_vm_info() -> Vec<Machine> {
    let output = Command::new("vagrant")
        .arg("global-status")
        .output()
        .expect("vagrant global-status failed");

    let owned = String::from_utf8_lossy(&output.stdout).into_owned();
    let lines = owned.lines()
        .skip(2)
        .filter(|x| x.split_whitespace().count() == 5);

    lines.map(|line| Machine::from_output_line(line)).collect()
}

fn print_machine_list() {
    let machines = get_vm_info();
    let output = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}", "Id", "Name", "Provider", "State", "Path");

    println!("{}", Yellow.paint(output));
    for machine in &machines {
        machine.to_output();
    }
}

fn refresh_machine_list() {
    Command::new("vagrant")
        .arg("global-status")
        .arg("--prune")
        .output()
        .expect("vagrant global-status --prune failed");
}

fn main() {
    let m = App::new("vagment")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("list")
            .about("List available machines")
        )
        .subcommand(SubCommand::with_name("refresh")
            .about("Clear vagrant cache")
        )
        .subcommand(SubCommand::with_name("up")
            .about("Bring up the specified machine")
            .arg(Arg::with_name("index")
                .short("i")
                .help("Machine index")
                .required(true)
            )
        )
        .get_matches();

    if m.is_present("list") {
        print_machine_list();
    } else if m.is_present("refresh") {
        refresh_machine_list();
    }
}
