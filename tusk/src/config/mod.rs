pub mod theme;

use std::{ffi::OsString, sync::Arc};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
	pub static ref CONFIG: Arc<Config> = Arc::new(Config::load());
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	theme: OsString,
}

impl Config {
	pub fn load() -> Self {
		Config {
			theme: OsString::from("../config.yaml"),
		}
	}
}
