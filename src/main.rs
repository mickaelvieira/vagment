#[macro_use]
extern crate clap;
extern crate ansi_term;

#[macro_use]
extern crate vagment;

use vagment::app::{self};
use vagment::app::vagrant;

fn main() {
    let matches = app::init_cli();
    let machines = vagrant::get_machines();

    if matches.is_present("list") {
        app::print_list(&machines);
    } else if matches.is_present("refresh") {
        vagrant::refresh_list();
    } else {
        let cmd: &str = matches.value_of("VAGRANT_COMMAND")
            .unwrap_or("");

        let num: u16 = matches.value_of("MACHINE_NUMBER")
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);

        app::process_command(&machines, cmd, num);
    }
}
