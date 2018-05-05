use std::path::PathBuf;


pub struct MyView {
    path_buf: PathBuf,
    pub selected: usize,
    pub count: usize,
    entries: Vec<PathBuf>,
}

impl MyView {
    pub fn new() -> MyView {
        MyView {
            path_buf: PathBuf::new(),
            selected: 0,
            count: 0,
            entries: Vec::new(),
        }
    }

    pub fn from(path: PathBuf) -> MyView {
        let entries = path.read_dir().unwrap();
        let paths = entries.into_iter().map(|e| {
            let dir = e.unwrap();
            let p = dir.path();
            p
        }).collect::<Vec<_>>();
        let count = paths.len();
        MyView {
            path_buf: path,
            selected: 0,
            count,
            entries: paths,

        }
    }

    pub fn get_entries(&self) -> Vec<String> {
        self.entries.iter().map(|e| {
            match e.file_name() {
                Some(data)  => match data.to_str() {
                    Some(value)     => value.to_string(),
                    None            => "".to_string(),
                },
                None        => "".to_string(),
            }
        }).collect::<Vec<_>>()
    }

    pub fn get_name(&self) -> String {
        //info!("Parent {:?}", self.path_buf.file_name());
        match self.path_buf.file_name() {
            Some(value)     => match value.to_str() {
                Some(val)   => val.to_string(),
                None        => "".to_string(),
            },
            None            => "".to_string(),
        }
    }

}
