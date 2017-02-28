#[macro_use]
extern crate clap;
extern crate vagment;

use clap::{App, SubCommand};
use std::process::Command;
use vagment::machine::{Machine};

fn get_vm_info() -> Vec<Machine> {
    let output = Command::new("vagrant")
        .arg("global-status")
        .output()
        .expect("ls command failed to start");

    let owned = String::from_utf8_lossy(&output.stdout).into_owned();
    let lines = owned.lines()
        .skip(2)
        .filter(|x| x.split_whitespace().count() == 5);

    lines.map(|line| Machine::from_output_line(line)).collect()
}

fn main() {
    let _m = App::new("vagment")
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("clear"))
        .get_matches();

    let _m = get_vm_info();

    println!("{:?}", _m.first());
}
