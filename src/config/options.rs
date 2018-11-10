use log::*;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum Size {
    Bits,
    Bytes,
}

#[derive(Serialize, Deserialize)]
enum StatusPosition {
    Top,
    Bottom,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigOptions {
    size: Size,
    show_hidden: bool,
    confirm: bool,
    show_images: bool,
    status_position: StatusPosition,
    shorten_title: usize,
    preview_max_size: usize,
    delay_idle: usize,
    line_numbers: bool,
    show_popup: bool,
}

impl Default for ConfigOptions {
    fn default() -> ConfigOptions {
        debug!("Loading defaults for ConfigOptions");
        ConfigOptions {
            size: Size::Bytes,
            show_hidden: false,
            confirm: true,
            show_images: false,
            status_position: StatusPosition::Bottom,
            shorten_title: 0,
            preview_max_size: 102400,
            delay_idle: 2000,
            line_numbers: false,
            show_popup: false,
        }
    }
}
