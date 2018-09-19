//! Marcos is a command-line file manager in Rust with VIM-inspired key-bindings
//! Currently, it is a very early stage of development. Only Unix-like platforms are supported.
//!
//! # Install
//! Marcos requires the [Rust](https://www.rust-lang.org/en-US/) programming language installed on your system. Also, make sure `ncurses` is installed on your system.
//!
//! Fire up a terminal and run _one_ of the following:
//! ```bash
//! git clone https://github.com/TerminalWitchcraft/marcos && cd marcos
//! cargo build --release
//! sudo mv ./target/release/mcf /usr/bin
//! ```
//! or
//! ```bash
//! cargo install --git https://github.com/TerminalWitchcraft/marcos
//! ```
//!
//! # Configuration
//! Marcos is configured via a set of [toml](https://github.com/toml-lang/toml) files located at
//! `$XDG_CONFIG_HOME/marcos` or `~/.config/marcos`.
//!
//! Also run `mcf --help` to list out all possible options.
//!
//! # Key bindings
//!
//! Below is the list of default key bindings:
//!
//! | Key      | Action                                                                                |
//! |----------|---------------------------------------------------------------------------------------|
//! | q        | Exit marcos                                                                           |
//! | j        | Select item down                                                                      |
//! | k        | Select item up                                                                        |
//! | h        | Go previous (left)                                                                    |
//! | l        | Go next(right)                                                                        |
//! | :        | Activate command mode                                                                 |
//! | gg       | Go to the first selection                                                             |
//! | G        | Go to the last selection                                                              |
//! | [count]G | Go to the [count] item                                                                |
//! | za       | Toggle visibility of hidden items                                                     |
//! | y        | Yank(Copy) the selected file/folder(Similar to Ctrl-c)                                |
//! | x        | Cut the selected file/folder(similar to Ctrl-x)                                       |
//! | p        | Paste the Copied/Cut file/folder(Similar to Ctrl-v)                                   |
//! | r        | Rename selected file/folder                                                           |
//! | dd       | Delete selected file/folder(with confirmation)                                        |
//! | o        | Create new file(`touch filename`)                                                     |
//! | O        | Create new directory (`mkdir dirname`)                                                |
//! | P        | Paste the Copied/Cut file/folder replacing existing with same name(with Confirmation) |
//! | mX       | Create a bookmark with name X                                                         |
//! | `X       | Jump to bookmark with name X                                                          |
//! | n        | Move to next match                                                                    |
//! | N        | Move to previous match                                                                |
//! | /        | Search                                                                                |
//! | v        | Starts visual mode, selects all files until you press ESC                             |
//! | V        | Visual mode, select all                                                               |
//! | Ctrl+r   | Refresh(listings, data, cache, etc)                                                   |
//! | ESC      | Get me out!                                                                           |
//! |          |                                                                                       |

#[macro_use]
extern crate log;
extern crate alphanumeric_sort;
extern crate cursive;
extern crate dirs;
extern crate failure;
extern crate fern;
extern crate mime_guess;
extern crate systemstat;
extern crate uname;
extern crate users;
extern crate walkdir;
#[macro_use]
extern crate failure_derive;
extern crate unicode_width;

pub mod core;
pub mod error;
pub mod ui;
pub mod utils;
pub mod fs;
pub mod config;
