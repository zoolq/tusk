use ratatui::prelude::*;

use crate::terminal::{
	modules::debug::{log::draw_log, tick::draw_ticks},
	App,
};

pub fn window_debug<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
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

	let ticks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
		.split(chunks[1]);

	let logs = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
		.split(chunks[0]);

	draw_ticks(f, app, ticks[0], ticks[1]);
	draw_log(f, app, logs[1])
}
