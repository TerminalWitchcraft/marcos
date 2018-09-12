//! marcos is a terminal file manager
#[macro_use]extern crate log;
extern crate cursive;
extern crate fern;
extern crate walkdir;
extern crate alphanumeric_sort;
extern crate mime_guess;
extern crate uname;
extern crate users;

pub mod core;
pub mod utils;
pub mod ui;
