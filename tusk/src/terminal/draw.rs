use std::io;

use ratatui::prelude::*;

use super::DrawingData;

pub fn draw<B: Backend>(data: &DrawingData, terminal: &mut Terminal<B>) -> io::Result<()> {
	Ok(())
}
