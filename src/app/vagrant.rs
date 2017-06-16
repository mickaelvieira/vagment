use std::env;
use std::process::Command;
use std::process::Stdio;
use std::result::Result;

use app::machine::Machine;
use app::formatter;
use app::errors::CommandError;

type CommandResult<T> = Result<T, CommandError>;

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

pub fn print_list(machines: Vec<Machine>) -> CommandResult<String> {
    println!("{}", formatter::format(&machines));
    Ok(String::from(""))
}

pub fn shutdown(machines: Vec<Machine>) -> CommandResult<String> {
    for machine in machines {
        let mut child = Command::new("vagrant")
            .current_dir(machine.get_path())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("halt")
            .spawn()
            .expect("failed");

        let _ = child.wait().expect("failed to wait on child");
    }

    Ok(String::from(""))
}

pub fn bootup(machines: Vec<Machine>) -> CommandResult<String> {
    for machine in machines {
        let mut child = Command::new("vagrant")
            .current_dir(machine.get_path())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("up")
            .spawn()
            .expect("failed");

        let _ = child.wait().expect("failed to wait on child");
    }

    Ok(String::from(""))
}

pub fn refresh() -> CommandResult<String> {
    let child = Command::new("vagrant")
        .arg("global-status")
        .arg("--prune")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("vagrant global-status --prune failed");

    let output = child.wait_with_output().expect("failed to wait on child");

    if !output.status.success() {
        Err(CommandError::ExitedWithError)
    } else {
        Ok("Command was executed successfully".to_string())
    }
}

pub fn execute(command: String, path: &str) -> CommandResult<String> {
    let mut child = Command::new("vagrant")
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg(command)
        .spawn()
        .expect("failed to execute process");

    let status = child.wait().expect("failed to wait on child");

    if !status.success() {
        Err(CommandError::ExitedWithError)
    } else {
        Ok("Command was executed successfully".to_string())
    }
}

pub fn dump(path: &str, file: String) -> CommandResult<String> {
    let child = Command::new("cat")
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg(file)
        .spawn()
        .expect("failed to execute process");

    let output = child.wait_with_output().expect("failed to wait on child");

    if !output.status.success() {
        Err(CommandError::ExitedWithError)
    } else {
        Ok("Command was executed successfully".to_string())
    }
}

pub fn edit(path: &str, file: String) -> Result<String, CommandError> {
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
            Err(CommandError::ExitedWithError)
        } else {
            Ok("Command was executed successfully".to_string())
        }
    } else {
        Err(CommandError::EnvNotFound("EDITOR".to_string()))
    }
}
