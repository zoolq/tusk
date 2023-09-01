use ratatui::prelude::*;

use crate::terminal::{modules::processes::draw_processes, DrawingData};

pub fn window_processes<B: Backend>(f: &mut Frame<B>, data: &DrawingData, area: Rect) {
	draw_processes(f, &data.processes, area);
}
