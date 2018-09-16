use std::path::PathBuf;

/// Struct for holding path information for each view such as parent, current, preview.
#[allow(dead_code)]
#[derive(Debug)]
pub struct View {
    pub p_buff: PathBuf,
}

impl View {
    fn from(path: &PathBuf) -> Self {
        Self {
            p_buff: path.to_path_buf(),
        }
    }
}


/// Struct to hold a collection of 3 views, according to miller's columns. First, being the
/// previous directory, then second directory, followed by preview window.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Tab {
    pub title: String,
    
    // Views
    pub p_view: View,
    pub c_view: View,
    pub preview: View,

    // Selected
    p_selected: Vec<usize>,
    c_selected: Vec<usize>,
    preview_selected: Vec<usize>,
}


impl Tab {
    /// Funtion to create a tab from given name and path
    pub fn from(title: &str, path: &PathBuf) -> Self {
        let current_path = path;
        let parent_path = path.parent().unwrap().to_path_buf();
        let preview_path = path.parent().unwrap().to_path_buf();

        let p_view = View::from(&parent_path);
        let c_view = View::from(current_path);
        let preview = View::from(&preview_path);

        Self {
            title: String::from(title),
            p_view,
            c_view,
            preview,

            p_selected: Vec::new(),
            c_selected: Vec::new(),
            preview_selected: Vec::new(),
        }
    }

    pub fn go_back(&mut self) {
        println!("hello");
    }

}
