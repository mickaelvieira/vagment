#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate vagment;

use clap::{App, SubCommand};
use std::process::Command;
use vagment::machine::Machine;

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
    println!("Id Name Provider State Path");
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
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("refresh"))
        .get_matches();

    if m.is_present("list") {
        print_machine_list();
    } else if m.is_present("refresh") {
        refresh_machine_list();
    }
}
