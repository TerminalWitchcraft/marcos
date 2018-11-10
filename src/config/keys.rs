use cursive::event::Event;
use cursive::Cursive;

use serde_derive::{Serialize, Deserialize};

use crate::error::*;

enum Modifier {
    // Shift is not included, as Capital letter denotes shift key usage!
    Alt,
    Ctrl,
    Shift,
    AltCtrl,
    AltShift,
    CtrlShift,
    NoMod,
    // Currently Meta bindings not supported by Cursive
    // Meta
}

pub struct KeySequence {
    takes_count: bool,
    max_count: Option<i8>,
    modifier: Modifier,
    key: Vec<char>,
    operation: fn(&mut Cursive),
}

impl KeySequence {
    /// Funtion to emit a vector of `Event`s.
    fn emit_sequence(self) -> (bool, Vec<Event>) {
        let mut seq: Vec<Event> = Vec::with_capacity(2);
        match self {
            KeySequence {
                modifier: Modifier::Alt,
                key: c,
                ..
            } => {
                if !c.is_empty() {
                    seq.push(Event::AltChar(c[0]))
                }
            }
            KeySequence {
                modifier: Modifier::Ctrl,
                key: c,
                ..
            } => {
                if !c.is_empty() {
                    seq.push(Event::CtrlChar(c[0]))
                }
            }
            KeySequence {
                takes_count: _,
                modifier: Modifier::Shift,
                key: c,
                ..
            } => {
                if !c.is_empty() {
                    seq.push(Event::Char(c[0].to_ascii_uppercase()))
                }
            }
            // KeySequence{takes_count,
            //     modifier:Modifier::AltCtrl,
            //     key:c} => {seq.push(Event::CtrlAlt(_))},
            // KeySequence{takes_count,
            //     modifier:Modifier::AltShift,
            //     key:c} => {seq.push(Event::CtrlAlt(c))},
            // KeySequence{takes_count,
            //     modifier:Modifier::CtrlShift,
            //     key:c} => {seq.push(Event::AltChar(c))},
            KeySequence {
                takes_count: _,
                modifier: Modifier::NoMod,
                key: c,
                ..
            } => {
                for i in c {
                    seq.push(Event::Char(i))
                }
            }
            _ => {}
        }
        (self.takes_count, seq)
    }
}

pub enum KeyBindings {
    Quit(KeySequence),
    SelectUp(KeySequence),
    SelectDown(KeySequence),
    Back(KeySequence),
    Forward(KeySequence),
    Console(KeySequence),
    SelectFirst(KeySequence),
    SelectLast(KeySequence),
    SelectN(KeySequence),
    ShowHidden(KeySequence),
    Yank(KeySequence),
    Cut(KeySequence),
    Paste(KeySequence),
    PasteReplace(KeySequence),
    Rename(KeySequence),
    DeleteWithConfirm(KeySequence),
    NewFile(KeySequence),
    NewDir(KeySequence),
    CreateBookmark(KeySequence),
    JumpToBookmark(KeySequence),
    Search(KeySequence),
    NextMatch(KeySequence),
    PrevMatch(KeySequence),
    Visual(KeySequence),
    VisualAll(KeySequence),
    Refresh(KeySequence),
}

#[derive(Serialize, Deserialize)]
pub struct KeyMaps {
    quit: String,
    select_up: String,
    select_down: String,
    // select_first		= "gg"
    // select_last 		= "G"
    // select_n		= "*g"
    back: String,
    forward: String,
    prompt: String,
    show_hidden: String,
    yank: String,
    cut: String,
    paste: String,
    paste_replace: String,
    rename: String,
    delete_with_cfm: String,
    new_file: String,
    new_folder: String,
    // create bookmark
    // jumpto bookmark
    search: String,
    next_match: String,
    previous_match: String,
    visual: String,
    visual_all: String,
    refresh: String,
}

impl Default for KeyMaps {
    fn default() -> Self {
        Self {
            quit: "q".to_string(),
            select_up: "k".to_string(),
            select_down: "j".to_string(),
            // select_first		= "gg"
            // select_last 		= "G"
            // select_n		= "*g"
            back: "h".to_string(),
            forward: "l".to_string(),
            prompt: ":".to_string(),
            show_hidden: "za".to_string(),
            yank: "y".to_string(),
            cut: "x".to_string(),
            paste: "p".to_string(),
            paste_replace: "P".to_string(),
            rename: "r".to_string(),
            delete_with_cfm: "dd".to_string(),
            new_file: "o".to_string(),
            new_folder: "O".to_string(),
            // create bookmark
            // jumpto bookmark
            search: "/".to_string(),
            next_match: "n".to_string(),
            previous_match: "N".to_string(),
            visual: "v".to_string(),
            visual_all: "V".to_string(),
            refresh: "C-r".to_string(),
        }
    }
}
