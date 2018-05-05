use std::env;
use ui::view::MyView;


pub struct MyTab<'b> {
    pub title: &'b str,
    pub parent: MyView,
    pub current: MyView,
    pub preview: MyView,
}


impl<'b> MyTab <'b> {
    pub fn new(name:&str) -> MyTab {
        MyTab {
            title: name,
            parent: MyView::new(),
            current: MyView::new(),
            preview: MyView::new(),
        }
    }

    pub fn default() -> MyTab <'b> {
        // current directory
        let current_path = env::current_dir().unwrap();
        let parent_path = current_path.parent().unwrap().to_path_buf();
        let preview_path = current_path.parent().unwrap().to_path_buf();
        debug!("Setting current directory to... {:?}", &current_path);
        debug!("Setting parent directory to... {:?}", &parent_path);
        debug!("Setting preview directory to... {:?}", &preview_path);
        MyTab {
            title: "Default Void",
            parent: MyView::from(parent_path),
            current: MyView::from(current_path),
            preview: MyView::from(preview_path),
        }
    }

    pub fn get_current_items(&self) -> Vec<String> {
        self.current.get_entries()
    }

    pub fn get_parent_items(&self) -> Vec<String> {
        self.parent.get_entries()
    }

    pub fn get_preview_items(&self) -> Vec<String> {
        self.current.get_entries()
    }

    pub fn get_parent_index(&self) -> usize {
        let pwd = &self.current.get_name();
        let mut i: usize = 0;
        for (index, name) in self.get_parent_items().iter().enumerate() {
            if pwd == name {
                i = index
            } 
        }
        i
    }
}
