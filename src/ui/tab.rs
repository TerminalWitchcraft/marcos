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
    p_selected: Vec<usize>,
    c_selected: Vec<usize>,
    // preview_selected: Vec<usize>,
}


impl Tab {
    /// Funtion to create a tab from given name and path
    pub fn from(title: u32, path: &PathBuf) -> Self {
        // TODO too much assumptions here. Need to clarify.
        let parent_path = path.parent().unwrap().to_path_buf();
        let current_path = PathBuf::from(path);

        let p_view = parent_path;
        let c_view = current_path;

        Self {
            title, 
            p_view,
            c_view,

            p_selected: Vec::new(),
            c_selected: Vec::new(),
        }
    }

    pub fn go_back(&mut self) {
        let flag = self.c_view.parent().is_some();
        let _curr_p = PathBuf::from(&self.c_view);
        let curr_p = PathBuf::from(&self.p_view);
        if flag {
            self.c_view = curr_p;
            self.p_view = self.p_view.parent().unwrap().to_path_buf();
        }
    }

}
