extern crate marcos;
extern crate cursive;
extern crate termion;

use marcos::{core};

fn main() {
    let mut app = core::app::init().unwrap();
    app.run();
}


