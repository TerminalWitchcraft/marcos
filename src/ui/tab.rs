use std::path::PathBuf;
use std::fs::read_dir;


pub struct MyTab<'b> {
    pub title: &'b str,
    pub parent: Vec<PathBuf>,
    pub current: Vec<PathBuf>,
    pub preview: Vec<PathBuf>,
}


impl<'b> MyTab <'b> {
    pub fn new(name:&str) -> MyTab {
        MyTab {
            title: name,
            parent: Vec::new(),
            current: Vec::new(),
            preview: Vec::new(),
        }
    }

    pub fn default() -> MyTab <'b> {
        // current directory
        let current_paths = read_dir(".").unwrap();
        let current_list = current_paths.into_iter().map(|e| {
            let dir = e.unwrap();
            let p = dir.path();
            p
        }).collect::<Vec<_>>();

        //parent directory
        let parent_paths = read_dir("..").unwrap();
        let parent_list = parent_paths.into_iter().map(|e| {
            let dir = e.unwrap();
            let p = dir.path();
            p
        }).collect::<Vec<_>>();

        //Populate the MyTab instance
        MyTab {
            title: "Default Void",
            parent: parent_list,
            current: current_list,
            preview: Vec::new(),
        }
    }
}
