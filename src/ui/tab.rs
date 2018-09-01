use std::path::PathBuf;
use cursive::views::SelectView;

#[allow(dead_code)]
struct View {
    p_buff: PathBuf,
    count: usize,
    vec_entries: Vec<PathBuf>,
}

impl View {
    fn from(path: &PathBuf) -> Self {
        let entries = path.read_dir().unwrap();
        let paths = entries.into_iter().map(|e| {
            let dir = e.unwrap();
            let p = dir.path();
            p
        }).collect::<Vec<_>>();

        Self {
            p_buff: path.to_path_buf(),
            count: paths.len(),
            vec_entries: paths
        }
    }
}

#[allow(dead_code)]
pub struct Tab {
    pub title: String,
    
    // Views
    p_view: View,
    c_view: View,
    preview: View,

    // Focused 
    p_focused: usize,
    c_focused: usize,
    preview_focused: usize,

    // Selected
    p_selected: Vec<usize>,
    c_selected: Vec<usize>,
    preview_selected: Vec<usize>,
}


impl Tab {
    /// Funtion to create a tab from given name ana path
    pub fn from(title: &str, path: &PathBuf) -> Self {
        let current_path = path;
        let parent_path = path.parent().unwrap().to_path_buf();
        let preview_path = path.parent().unwrap().to_path_buf();

        let c_view = View::from(current_path);
        let p_view = View::from(&parent_path);
        let preview = View::from(&preview_path);
        let p_focused = Self::get_parent_index(&c_view);

        Self {
            title: String::from(title),
            p_view,
            c_view,
            preview,

            p_focused,
            c_focused: 0,
            preview_focused: 0,

            p_selected: Vec::new(),
            c_selected: Vec::new(),
            preview_selected: Vec::new(),
        }
    }

    pub fn p_widget(&self) -> SelectView<PathBuf> {
        let mut widget = SelectView::default();
        for item in self.p_view.vec_entries.iter() {
            let label: &str = match item.file_name() {
                Some(name) => match name.to_str() {
                    Some(data) => data,
                    None    => "",
                }
                None => ""
            };
        widget.add_item(label, PathBuf::from(item));
        }
        widget
    }

    pub fn c_widget(&self) -> SelectView<PathBuf> {
        let mut widget = SelectView::default();
        for item in self.c_view.vec_entries.iter() {
            let label: &str = match item.file_name() {
                Some(name) => match name.to_str() {
                    Some(data) => data,
                    None    => "",
                }
                None => ""
            };
        widget.add_item(label, PathBuf::from(item));
        }
        widget
    }

    fn get_parent_index(c_view: &View) -> usize {
        let mut i: usize = 0;
        for (index, name) in c_view.vec_entries.iter().enumerate() {
            if &c_view.p_buff == name {
                i = index;
                break;
            }
        }
        i
    }

}
