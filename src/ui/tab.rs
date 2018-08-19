use std::path::PathBuf;

struct View {
    p_buff: PathBuf,
    count: usize,
    vec_entries: Vec<PathBuf>,
}

impl View {
    fn from(path: &PathBuf) -> View {
        let entries = path.read_dir().unwrap();
        let paths = entries.into_iter().map(|e| {
            let dir = e.unwrap();
            let p = dir.path();
            p
        }).collect::<Vec<_>>();

        View {
            p_buff: path.to_path_buf(),
            count: paths.len(),
            vec_entries: paths
        }
    }
}

pub struct Tab {
    pub title: String,
    p_view: View,
    c_view: View,
    preview: View,

    p_focused: usize,
    c_focused: usize,
    p_selected: Vec<usize>,
    c_selected: Vec<usize>,
    preview_selected: Vec<usize>,
}


impl Tab {
    pub fn from(title: String, path: &PathBuf) -> Tab {
        let current_path = path;
        let parent_path = path.parent().unwrap().to_path_buf();
        let preview_path = path.parent().unwrap().to_path_buf();

        let c_view = View::from(current_path);
        let p_view = View::from(&parent_path);
        let preview = View::from(&preview_path);
        let p_focused = Tab::get_parent_index(&c_view);

        Tab {
            title,
            p_view,
            c_view,
            preview,

            p_focused,
            c_focused: 0,

            p_selected: Vec::new(),
            c_selected: Vec::new(),
            preview_selected: Vec::new(),
        }
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
