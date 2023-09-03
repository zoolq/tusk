use ratatui::prelude::*;

use crate::terminal::{modules::processes::draw_processes, App};

pub fn window_processes<B: Backend>(f: &mut Frame<B>, data: &App, area: Rect) {
	draw_processes(f, &data.processes, area);
}
