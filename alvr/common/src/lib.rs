mod data;
mod logging;

use semver::{Prerelease, Version};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub use data::*;
pub use glam;
pub use lazy_static::lazy_static;
pub use log;
pub use logging::*;
pub use semver;

pub type StrResult<T = ()> = Result<T, String>;

pub const ALVR_NAME: &str = "ALVR";

pub mod prelude {
    pub use crate::{
        fmt_e, logging::*, trace_err, trace_err_dbg, trace_none, trace_str, StrResult,
    };
    pub use log::{debug, error, info, warn};
}

lazy_static! {
    pub static ref ALVR_VERSION: Version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
}

// accept semver-compatible versions
// Note: by not having to set the requirement manually, the major version is constrained to be
// bumped when the packet layouts or some critical behaviour has changed.
pub fn is_version_compatible(other_version: &Version) -> bool {
    if other_version.pre != Prerelease::EMPTY || ALVR_VERSION.pre != Prerelease::EMPTY {
        // Note: metadata (+) is always ignored in the version check
        *other_version == *ALVR_VERSION
    } else {
        other_version.major == ALVR_VERSION.major
    }
}

// Consistent across architectures, might not be consistent across different compiler versions.
pub fn hash_string(string: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish()
}

lazy_static! {
    pub static ref HEAD_ID: u64 = hash_string("/user/head");
    pub static ref LEFT_HAND_ID: u64 = hash_string("/user/hand/left");
    pub static ref RIGHT_HAND_ID: u64 = hash_string("/user/hand/right");
}