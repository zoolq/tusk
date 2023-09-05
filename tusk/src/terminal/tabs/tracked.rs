use ratatui::prelude::*;

use crate::terminal::{
	modules::{
		error::{draw_error, FrameError::MissingTracked},
		tracked::{cpu_usage::draw_tracked_usage, memory::draw_tracked_memory},
	},
	App,
};

pub fn window_tracked<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	if app.tracked.is_some() {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[
					Constraint::Percentage(33),
					Constraint::Percentage(33),
					Constraint::Percentage(33),
				]
				.as_ref(),
			)
			.split(area);

		let chunks_middle_split = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
			.split(chunks[1]);

		draw_tracked_usage(f, app, chunks_middle_split[0]);
		draw_tracked_memory(f, app, chunks_middle_split[1]);
	} else {
		draw_error(f, MissingTracked, area, &app.theme);
	}
}
