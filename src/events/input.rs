use std::io;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, RecvError};
use termion::input::TermRead;
use termion::event;


pub struct InputThread {
    tx: Sender<event::Key>,
    rx: Receiver<event::Key>,
    //handle: thread::JoinHandle<()>,
}

impl InputThread {
    pub fn new() -> InputThread {
        let (tx, rx) = mpsc::channel();
        InputThread {
            tx,
            rx,
        }
    }

    pub fn spawn(sender: Sender<event::Key>) {
        thread::spawn(move || {
            let stdin = io::stdin();
            for c in stdin.keys() {
                let evt = c.unwrap();
                sender.send(evt).unwrap();
                if evt == event::Key::Char('q') {
                    break;
                }
            }
            ()
        });
    }

    pub fn get_evt(&self) -> Result<event::Key, RecvError> {
        Ok(self.rx.recv()?)
    }

    pub fn clone_tx(&self) -> Sender<event::Key> {
        self.tx.clone()
    }
}
