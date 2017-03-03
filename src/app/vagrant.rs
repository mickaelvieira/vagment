use std::process::Command;
use std::process::Stdio;

use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;

use app::machine::Machine;

pub fn get_machines() -> Vec<Machine> {
    let child =
        Command::new("vagrant")
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

pub fn refresh_machine_list() {
    Command::new("vagrant")
        .arg("global-status")
        .arg("--prune")
        .spawn()
        .expect("vagrant global-status --prune failed");
}

pub fn execute_command(cmd: &str, path: &str) {
    println!("{}", Green.paint(format!(
        "Executing command vagrant {} in {}",
        Yellow.paint(cmd),
        Yellow.paint(path)
    )));

    let mut child =
        Command::new("vagrant")
            .current_dir(path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg(cmd)
            .spawn()
            .expect("failed to execute process");

    let status = child.wait().expect("failed to wait on child");
    if status.success() {
        println!("Command was executed successfully");
    } else {
        println!("Command exited with errors");
    }
}

pub fn print_machine_list(machines: &Vec<Machine>) {
    let output = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10} | {4: ^10}", "Id", "Name", "Provider", "State", "Path");

    println!("{}", Yellow.paint(output));

    for machine in machines {
        machine.to_output();
    }
}
