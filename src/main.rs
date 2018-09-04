extern crate marcos;
extern crate cursive;
extern crate termion;
extern crate clap;

use clap::{Arg, App, SubCommand};
use marcos::{core};

fn main() {
    let matches = App::new("Marcos")
        .version("0.1.0")
        .author("Hitesh Paul")
        .about("Command line file-manager inspired by vim-like keybinding")
        .arg(Arg::with_name("path")
             .required(false)
             .help("Path to a directory where marcos should open")
             .value_name("PATH"))
        .arg(Arg::with_name("log")
             .required(false)
             .help("Path to dump log file")
             .short("l")
             .long("log")
             .value_name("LOG"))
        .arg(Arg::with_name("log_level")
             .requires("log")
             .help("Level of log files")
             .short("v")
             .long("level")
             .possible_values(&["debug", "info", "error"])
             .value_name("LOG_LEVEL"))
        .get_matches();
    let mut app = core::app::init(
        match matches.value_of("path") {
            Some(c) => c,
            None => ".",
        },
        matches.value_of("log"),
        matches.value_of("log_level")).unwrap();
    // if let Some(c) = matches.value_of("path") {
    //     let mut app = core::app::init(c).unwrap();
    //     // println!("Got {} for path", c);
    // } else {
    //     println!("Wow");
    // }
    // let mut app = core::app::init().unwrap();
    // app.run();
}


