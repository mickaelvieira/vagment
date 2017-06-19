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

fn get_machine_line(machine: &Machine) -> String {
    let line = format!("{0: ^10} | {1: ^10} | {2: ^10} | {3: ^10}",
                       machine.get_number(),
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

pub fn format(machines: &[Machine]) -> String {
    let mut lines = Vec::new();
    lines.push(get_empty_line());
    lines.push(get_header());
    lines.push(get_separator());
    for machine in machines {
        lines.push(get_machine_line(machine));
    }
    lines.push(get_empty_line());
    lines.join("\n")
}
