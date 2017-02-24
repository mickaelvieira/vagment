extern crate vagment;

use std::env;
use std::process::Command;
use vagment::logger;
//
#[derive(Debug)]
struct Machine {
    id: String,
    name: String,
    provider: String,
    state: String,
    directory: String
}

// trait Const {
//     fn from_output_line(&self, &str: line) -> Machine;
// }
//
// impl Const for Machine {
//     fn from_output_line(&self, &str: line) -> Machine {
//         let mut words = line.to_string().split_whitespace();
//
//         let id = words.next().unwrap().to_string();
//         let name = words.next().unwrap().to_string();
//         let provider = words.next().unwrap().to_string();
//         let state = words.next().unwrap().to_string();
//         let directory = words.next().unwrap().to_string();
//
//         Machine{
//             id: id,
//             name: name,
//             provider: provider,
//             state: state,
//             directory: directory
//         }
//     }
// }

fn get_vm_info() {
    let output = Command::new("vagrant")
        .arg("global-status")
        .output()
        .expect("ls command failed to start");

    // println!("status: {}", output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    let owned = String::from_utf8_lossy(&output.stdout).into_owned();
    let lines = owned.lines()
        .skip(2)
        .filter(|x| x.split_whitespace().count() == 5);

    lines.map(|line| {
        let mut words = line.split_whitespace();

        let id = words.next().unwrap().to_string();
        let name = words.next().unwrap().to_string();
        let provider = words.next().unwrap().to_string();
        let state = words.next().unwrap().to_string();
        let directory = words.next().unwrap().to_string();

        Machine{
            id: id,
            name: name,
            provider: provider,
            state: state,
            directory: directory
        }
    })
}

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let command = env::args().nth(1);

    let o = get_vm_info();

    // println!("{:?}", o);

    if command.is_none() {
        logger::error("Please provide a command");
    } else {
        if command.unwrap() == "version" {
            logger::info(version);
        }
    }
}
