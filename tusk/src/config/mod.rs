pub mod theme;

use std::{ffi::OsString, fs::File};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
	theme: OsString,
}
