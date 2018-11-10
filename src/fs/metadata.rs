//! This module contains code to retrieve metadata about file/directory such as permissions,
//! owners, size, etc.
use std::os::unix::fs::*;
use std::path::PathBuf;

use users::{get_group_by_gid, get_user_by_uid};

use crate::error::*;

mod modes {
    pub type Mode = u32;

    pub const USER_READ: Mode = 256;
    pub const USER_WRITE: Mode = 128;
    pub const USER_EXECUTE: Mode = 64;

    pub const GROUP_READ: Mode = 32;
    pub const GROUP_WRITE: Mode = 16;
    pub const GROUP_EXECUTE: Mode = 8;

    pub const OTHER_READ: Mode = 4;
    pub const OTHER_WRITE: Mode = 2;
    pub const OTHER_EXECUTE: Mode = 1;
}

struct Permissions {
    pub user_read: bool,
    pub user_write: bool,
    pub user_execute: bool,

    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,

    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,
    // pub sticky:         bool,
    // pub setgid:         bool,
    // pub setuid:         bool,
}

impl From<u32> for Permissions {
    fn from(bits: u32) -> Self {
        let has_bit = |bit| bits & bit == bit;
        Self {
            user_read: has_bit(modes::USER_READ),
            user_write: has_bit(modes::USER_WRITE),
            user_execute: has_bit(modes::USER_EXECUTE),

            group_read: has_bit(modes::GROUP_READ),
            group_write: has_bit(modes::GROUP_WRITE),
            group_execute: has_bit(modes::GROUP_EXECUTE),

            other_read: has_bit(modes::OTHER_READ),
            other_write: has_bit(modes::OTHER_WRITE),
            other_execute: has_bit(modes::OTHER_EXECUTE),
            // sticky:         has_bit(modes::STICKY),
            // setgid:         has_bit(modes::SETGID),
            // setuid:         has_bit(modes::SETUID),
        }
    }
}

impl ToString for Permissions {
    fn to_string(&self) -> String {
        let mut repr = String::with_capacity(9);
        if self.user_read {
            repr.push('r')
        } else {
            repr.push('-')
        }
        if self.user_write {
            repr.push('w')
        } else {
            repr.push('-')
        }
        if self.user_execute {
            repr.push('x')
        } else {
            repr.push('-')
        }

        if self.group_read {
            repr.push('r')
        } else {
            repr.push('-')
        }
        if self.group_write {
            repr.push('w')
        } else {
            repr.push('-')
        }
        if self.group_execute {
            repr.push('x')
        } else {
            repr.push('-')
        }

        if self.other_read {
            repr.push('r')
        } else {
            repr.push('-')
        }
        if self.other_write {
            repr.push('w')
        } else {
            repr.push('-')
        }
        if self.other_execute {
            repr.push('x')
        } else {
            repr.push('-')
        }
        repr
    }
}

/// Represents an entry. It can be a file or a directory.
/// Contains information such as number of files(if its a directory), permissions,
/// groups, owners, size, etc.
pub struct Entry {
    path: PathBuf,
}

/// Initialize from PathBuf
impl From<PathBuf> for Entry {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Entry {
    /// Funtion which returns String representing permissions and owner of selected entry.
    pub fn permission_string(&self) -> Result<String> {
        let meta = self.path.metadata()?;
        let uid = meta.uid();
        let gid = meta.gid();
        let uid = get_user_by_uid(uid).unwrap();
        let gid = get_group_by_gid(gid).unwrap();
        let mut repr = String::with_capacity(10);
        if self.path.is_dir() {
            repr.push('d')
        } else {
            repr.push('-')
        }
        Ok(repr
            + &Permissions::from(meta.mode()).to_string()
            + &format!(" {}:{}", uid.name(), gid.name()))
    }
}
