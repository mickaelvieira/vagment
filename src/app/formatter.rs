use ansi_term::Colour::Green;
use ansi_term::Colour::Yellow;

use app::machine::Machine;

fn get_empty_line() -> String {
    String::from("")
}

fn get_header() -> String {
    let o = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10}",
                    "Number",
                    "Name",
                    "State",
                    "Path");
    format!("{}", Yellow.paint(o))
}

fn get_machine_line(index: usize, machine: &Machine) -> String {
    let line = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10}",
                       index + 1,
                       machine.get_name(),
                       machine.get_state(),
                       machine.get_path());
    format!("{}", Green.paint(line))
}

fn get_separator() -> String {
    let s = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10}",
                    "----------",
                    "----------",
                    "----------",
                    "----------");
    format!("{}", Yellow.paint(s))
}

pub fn format(machines: &Vec<Machine>) -> String {
    let mut lines = Vec::new();
    lines.push(get_empty_line());
    lines.push(get_header());
    lines.push(get_separator());
    for (index, machine) in machines.iter().enumerate() {
        lines.push(get_machine_line(index, machine));
    }
    lines.push(get_empty_line());
    lines.join("\n")
}
