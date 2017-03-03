use std::process::Command;
use std::process::Stdio;

use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;

use app::machine::Machine;

pub fn get_machines() -> Vec<Machine> {
    let child =
        Command::new("vagrant")
            .arg("global-status")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("vagrant global-status failed");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    let owned = String::from_utf8_lossy(&output.stdout).into_owned();
    let lines = owned.lines()
        .skip(2)
        .filter(|x| x.split_whitespace().count() == 5);

    lines.map(|line| Machine::from_output_line(line)).collect()
}

pub fn refresh_list() {
    println!("{}", Green.paint("Refreshing machine listing"));

    let child =
        Command::new("vagrant")
            .arg("global-status")
            .arg("--prune")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("vagrant global-status --prune failed");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    if output.status.success() {
        println!("{}", Green.paint("Command was executed successfully"));
    } else {
        println!("{}", Red.paint("Command exited with errors"));
    }
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
        println!("{}", Green.paint("Command was executed successfully"));
    } else {
        println!("{}", Red.paint("Command exited with errors"));
    }
}
