use std::process::Command;
use std::process::Stdio;
use std::result::Result;

use app::machine::Machine;

type CmdResult<T> = Result<T, String>;

pub trait CmdType {
    fn needs_machine_up(&self) -> bool;
}

impl CmdType for str {
    fn needs_machine_up(&self) -> bool {
        self == "ssh"
    }
}

pub fn list() -> Vec<Machine> {
    let child = Command::new("vagrant")
        .arg("global-status")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("vagrant global-status failed");

    let output = child.wait_with_output()
        .expect("failed to wait on child");

    let owned = String::from_utf8_lossy(&output.stdout).into_owned();
    let lines = owned.lines()
        .skip(2)
        .filter(|x| x.split_whitespace().count() == 5);

    lines.map(Machine::from_output_line).collect()
}

pub fn refresh() -> CmdResult<String> {
    let child = Command::new("vagrant")
        .arg("global-status")
        .arg("--prune")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("vagrant global-status --prune failed");

    let output = child.wait_with_output()
        .expect("failed to wait on child");

    if !output.status.success() {
        return Err("Command exited with errors".to_string());
    }

    Ok("Command was executed successfully".to_string())
}

pub fn boot(path: &str) -> CmdResult<String> {
    let mut child = Command::new("vagrant")
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("up")
        .spawn()
        .expect("failed to execute process");

    let status = child.wait()
        .expect("failed to wait on child");

    if !status.success() {
        return Err("Command exited with errors".to_string());
    }

    Ok("Command was executed successfully".to_string())
}

pub fn execute(command: &str, path: &str) -> CmdResult<String> {
    let mut child = Command::new("vagrant")
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg(command)
        .spawn()
        .expect("failed to execute process");

    let status = child.wait()
        .expect("failed to wait on child");

    if !status.success() {
        return Err("Command exited with errors".to_string());
    }

    Ok("Command was executed successfully".to_string())
}

pub fn dump(path: &str) -> CmdResult<String> {
    let mut file = path.to_string();
    file.push_str("/Vagrantfile");

    let child = Command::new("cat")
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg(file)
        .spawn()
        .expect("failed to execute process");

    let output = child.wait_with_output()
        .expect("failed to wait on child");

    if !output.status.success() {
        return Err("Command exited with errors".to_string());
    }

    Ok("Command was executed successfully".to_string())
}
