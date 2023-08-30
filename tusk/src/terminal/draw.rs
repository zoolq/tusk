use std::{io, path::PathBuf};

use ratatui::prelude::*;

use super::{modules::draw_default, DrawingData};

#[derive(Default)]
pub enum Screen {
	/// The default screen, a bit of everything.
	#[default]
	Default,
	/// A custom screen, found in a .yaml file.
	Custom(PathBuf),
	/// A special cpu screen with lots of cpu information.
	Cpu,
	/// A special network screen with lots of network information.
	Network,
	/// A special memory screen with lots of memory information.
	Memory,
	/// A special process screen with information on the running processes.
	Process,
}

/// Wrapper function for drawing terminals
pub fn draw<B: Backend>(
	screen: Screen,
	data: &DrawingData,
	terminal: &mut Terminal<B>,
) -> io::Result<()> {
	match screen {
		_ => draw_default(data, terminal)?,
	}
	Ok(())
}
