//! Module which contains information related to system paths, files, folders, available space, etc

use std::fmt::Display;
use std::path::Path;

use systemstat::{Platform, System};
use uname::uname;
use users::get_current_username;

/// Funtion to get user information as a String in the form:
/// `username @ host`
pub fn user_info() -> String {
    let user_name = get_current_username().unwrap_or("NA".to_string());
    let user_data = uname().unwrap();
    format!("{}@{}", user_name, user_data.nodename)
}

/// Function to get mount point info as a std::String in the form:
/// `_path available_ of _total_`
pub fn disk_info<P: AsRef<Path> + Display>(path: P) -> String {
    let sys = System::new();
    let mount_info: String = match sys.mount_at(&path) {
        Ok(mount) => format!("{} {} available of {}", path, mount.avail, mount.total),
        // TODO Make error string less verbose
        Err(_e) => String::from("Failed to read mount info!"),
    };
    mount_info
}
