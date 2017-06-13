use ansi_term::Colour::Yellow;
use std::result::Result;

use app::machine::Machine;
use app::machine::Machines;
use app::command::AppCommand;

pub mod logger;
pub mod machine;
pub mod vagrant;
pub mod cli;
pub mod args;
pub mod command;
pub mod number;

type CmdResult<T> = Result<T, String>;

pub fn print_list(machines: Vec<Machine>) -> CmdResult<String> {
    println!("{}", machines.to_output());
    Ok(String::from(""))
}

pub fn edit_vagrant_file(machines: Vec<Machine>, number: u16) -> CmdResult<String> {
    let result = machines.get_machine_by_number(number);

    if result.is_none() {
        return Err("Nope".to_string());
    }

    let machine = result.unwrap();

    vagrant::edit(machine.get_path(), machine.get_vagrant_file_path())
}

pub fn dump_vagrant_file(machines: Vec<Machine>, number: u16) -> CmdResult<String> {
    let result = machines.get_machine_by_number(number);

    if result.is_none() {
        return Err("Nope".to_string());
    }

    let machine = result.unwrap();

    vagrant::dump(machine.get_path(), machine.get_vagrant_file_path())
}

pub fn process_command(machines: Vec<Machine>, command: &str, number: u16) -> CmdResult<String> {
    let result = machines.get_machine_by_number(number);

    if result.is_none() {
        return Err("Nope".to_string());
    }

    let machine = result.unwrap();
    let commands = list_commands!();

    if !command.is_vagrant_command() {
        return Err(format!("`{}` is not a valid command! Available commands are {}",
                           command,
                           commands.join(", ")));
    }

    logger::info(format!("Executing command vagrant {} in {}",
                         Yellow.paint(command),
                         Yellow.paint(machine.get_path())));

    if command.needs_machine_up() && !machine.is_running() {
        logger::info("VM is not running, we are going to boot it up");
        let result = vagrant::boot(machine.get_path());
        if result.is_err() {
            return result;
        }
    }

    vagrant::execute(command, machine.get_path())
}
