use ratatui::prelude::*;

use crate::terminal::{modules::processes::draw_processes, Data};

pub fn window_processes<B: Backend>(f: &mut Frame<B>, data: &Data, area: Rect) {
	draw_processes(f, &data.processes, area);
}
