use crate::error::*;
use std::collections::HashMap;
use std::path::PathBuf;

/// Struct to hold a collection of 3 views, according to miller's columns. First, being the
/// previous directory, then second directory, followed by preview window.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Tab {
    pub title: u32,

    // Views
    pub p_view: PathBuf,
    pub c_view: PathBuf,
    // pub preview: PathBuf,

    // Selected
    // TODO Currently, based on index, need to change to PathBuf
    pub focused: HashMap<PathBuf, usize>,
    // pub p_focused: usize,
    // pub c_focused: Option<usize>,
    // preview_selected: Vec<usize>,
}

impl Tab {
    /// Funtion to create a tab from given name and path
    pub fn from(title: u32, path: &PathBuf) -> Result<Self> {
        // TODO too much assumptions here. Need to clarify.
        debug!("Inside tab::from {:?}", path);
        let p_view: PathBuf = match path.to_str() {
            Some("/") => PathBuf::from("root"),
            Some(e) => PathBuf::from(e)
                .parent()
                .ok_or(ErrorKind::DirNotFound {
                    dirname: format!("Parent for {:?}", e),
                })?.to_path_buf(),
            None => PathBuf::new(),
        };
        // let parent_path = path.parent()
        //     .ok_or(ErrorKind::DirNotFound{dirname: format!("Parent for: {:?}", path.to_str())})?;
        let c_view: PathBuf = PathBuf::from(path);

        Ok(Self {
            title,
            p_view,
            c_view,

            focused: HashMap::new(),
            // p_focused: 0,
            // c_focused: None,
        })
    }

    pub fn go_back(&mut self) {
        let temp_path = PathBuf::from(&self.p_view);
        match temp_path.to_str() {
            Some("/") => {
                self.p_view = PathBuf::from("root");
                self.c_view = PathBuf::from("/")
            }
            Some("root") => {}
            Some(c) => {
                debug!("Getting other..... {:?}", c);
                let path = PathBuf::from(c);
                match path.parent() {
                    Some(d) => {
                        self.p_view = d.to_path_buf();
                        self.c_view = PathBuf::from(c);
                    }
                    None => {}
                }
            }
            None => {}
        }
        debug!("{:?}, {:?}", self.p_view, self.c_view);
    }

    pub fn go_forward(&mut self, path: PathBuf) {
        self.c_view = PathBuf::from(&path);
        self.p_view = path.parent().unwrap().to_path_buf();
    }
}
