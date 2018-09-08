//! Funtions to help in assisting filter of entries

use walkdir::DirEntry;

/// Returns true if entry is hidden, irrespective of type(file or directory)
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
