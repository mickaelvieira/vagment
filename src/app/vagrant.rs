use std::env;
use std::process::Command;
use std::process::Stdio;
use std::result::Result;

use app::machine::Machine;
use app::machine::Machines;

type CmdResult<T> = Result<T, String>;

pub fn get_machine_list() -> Vec<Machine> {
    let child = Command::new("vagrant")
        .arg("global-status")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("vagrant global-status failed");

    let output = child.wait_with_output().expect("failed to wait on child");
    let owned = String::from_utf8_lossy(&output.stdout).into_owned();
    let lines = owned.lines()
        .skip(2)
        .filter(|x| x.split_whitespace().count() == 5);

    lines.map(Machine::from_output_line).collect()
}

pub fn print_list(machines: Vec<Machine>) -> CmdResult<String> {
    println!("{}", machines.to_output());
    Ok(String::from(""))
}

pub fn shutdown(machines: Vec<Machine>) -> CmdResult<String> {
    for machine in machines {
        Command::new("vagrant")
            .current_dir(machine.get_path())
            .arg("halt")
            .spawn()
            .expect("failed");
    }

    Ok(String::from(""))
}

pub fn refresh() -> CmdResult<String> {
    let child = Command::new("vagrant")
        .arg("global-status")
        .arg("--prune")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("vagrant global-status --prune failed");

    let output = child.wait_with_output().expect("failed to wait on child");

    if !output.status.success() {
        Err("Command exited with errors".to_string())
    } else {
        Ok("Command was executed successfully".to_string())
    }
}

pub fn boot(path: &str) -> CmdResult<String> {
    let mut child = Command::new("vagrant")
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("up")
        .spawn()
        .expect("failed to execute process");

    let status = child.wait().expect("failed to wait on child");

    if !status.success() {
        Err("Command exited with errors".to_string())
    } else {
        Ok("Command was executed successfully".to_string())
    }
}

pub fn execute(command: &str, path: &str) -> CmdResult<String> {
    let mut child = Command::new("vagrant")
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg(command)
        .spawn()
        .expect("failed to execute process");

    let status = child.wait().expect("failed to wait on child");

    if !status.success() {
        Err("Command exited with errors".to_string())
    } else {
        Ok("Command was executed successfully".to_string())
    }
}

pub fn dump(path: &str, file: String) -> CmdResult<String> {
    let child = Command::new("cat")
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg(file)
        .spawn()
        .expect("failed to execute process");

    let output = child.wait_with_output().expect("failed to wait on child");

    if !output.status.success() {
        Err("Command exited with errors".to_string())
    } else {
        Ok("Command was executed successfully".to_string())
    }
}

pub fn edit(path: &str, file: String) -> CmdResult<String> {
    let editor = env::var_os("EDITOR");

    if editor.is_some() {
        let child = Command::new(editor.unwrap())
            .current_dir(path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg(file)
            .spawn()
            .expect("failed to execute process");

        let output = child.wait_with_output().expect("failed to wait on child");

        if !output.status.success() {
            Err("Command exited with errors".to_string())
        } else {
            Ok("Command was executed successfully".to_string())
        }
    } else {
        Err("The environment variable EDITOR does not appear to be set".to_string())
    }
}
