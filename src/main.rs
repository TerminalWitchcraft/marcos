extern crate clap;
extern crate cursive;
extern crate failure;
extern crate marcos;

use clap::{App, Arg};

use marcos::core;
use marcos::error;

fn main() {
    if let Err(err) = try_main() {
        println!("{}", error::failure_to_string(&err));
        let backtrace = err.backtrace().to_string();
        if !backtrace.trim().is_empty() {
            eprintln!("{}", backtrace);
        }
        ::std::process::exit(1);
    }
}

fn try_main() -> Result<(), failure::Error> {
    let matches = App::new("Marcos")
        .version("0.1.0")
        .author("Hitesh Paul")
        .about("Command line file-manager inspired by vim-like keybinding")
        .arg(
            Arg::with_name("path")
                .required(false)
                .help("Path to a directory where marcos should open")
                .value_name("PATH"),
        ).arg(
            Arg::with_name("log")
                .required(false)
                .help("Path to dump log file")
                .short("l")
                .long("log")
                .value_name("LOG"),
        ).arg(
            Arg::with_name("log_level")
                .requires("log")
                .help("Level of log files")
                .short("v")
                .long("level")
                .possible_values(&["debug", "info", "error"])
                .value_name("LOG_LEVEL"),
        ).get_matches();
    let mut app = core::app::init(
        match matches.value_of("path") {
            Some(c) => c,
            None => ".",
        },
        matches.value_of("log"),
        matches.value_of("log_level"),
    ).unwrap();
    app.run();
    // if let Some(c) = matches.value_of("path") {
    //     let mut app = core::app::init(c).unwrap();
    //     // println!("Got {} for path", c);
    // } else {
    //     println!("Wow");
    // }
    // let mut app = core::app::init().unwrap();
    // app.run();
    Ok(())
}
