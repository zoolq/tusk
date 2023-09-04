use ratatui::prelude::*;

use crate::terminal::{
	modules::{
		cpu_usage::draw_usage,
		error::{draw_error, FrameError::MissingTracked},
		tracked::memory::draw_tracked_memory,
	},
	App,
};

pub fn window_tracked<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	if let Some(tracked) = &app.tracked {
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

		draw_usage(f, &tracked.cpu_usage, chunks_middle_split[0]);
		draw_tracked_memory(f, &tracked.memory, chunks_middle_split[1]);
	} else {
		draw_error(f, MissingTracked, area);
	}
}
