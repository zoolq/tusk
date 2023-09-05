pub mod theme;

use std::ffi::OsString;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
	theme: OsString,
}
