#[macro_use]
extern crate clap;
extern crate ansi_term;
extern crate vagment;

use ansi_term::Colour::Yellow;

use clap::{Arg, App, SubCommand};
use std::io;
use std::process::Command;
use std::process::Stdio;
use vagment::app::Machine;

fn get_machines() -> Vec<Machine> {
    let child = Command::new("vagrant")
        .arg("global-status")
        .stdout(Stdio::piped())
        .spawn()
        .expect("vagrant global-status failed");

    let output = child.wait_with_output().unwrap();
    let owned = String::from_utf8_lossy(&output.stdout).into_owned();
    let lines = owned.lines()
        .skip(2)
        .filter(|x| x.split_whitespace().count() == 5);

    lines.map(|line| Machine::from_output_line(line)).collect()
}

fn print_machine_list(machines: &Vec<Machine>) {
    let output = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}", "Id", "Name", "Provider", "State", "Path");

    println!("{}", Yellow.paint(output));
    for machine in machines {
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

fn prompt_machine_number(machines: &Vec<Machine>) -> String {

    print_machine_list(machines);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(bytes) => bytes,
        Err(error) => panic!("Could nout read input: {}", error)
    };
    input.trim().to_string()
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

    let machines = get_machines();
    let input = prompt_machine_number(&machines);
    let num: i32 = input.parse().unwrap_or(-1);

    println!("{:?}", input);
    println!("{:?}", num);

    if num < 0 {
        panic!("Please enter a valid number");
    }

    if m.is_present("list") {
        print_machine_list(&machines);
    } else if m.is_present("refresh") {
        refresh_machine_list();
    }

    println!("Number of machines {}", machines.len());
}
