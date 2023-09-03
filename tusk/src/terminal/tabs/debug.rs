use ratatui::prelude::*;

use crate::terminal::{modules::debug::tick::draw_ticks, App};

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

	draw_ticks(
		f,
		&app.working_tick,
		&app.real_tick,
		ticks[0],
		&app.refresh_tick,
		&app.drawing_tick,
		&app.event_tick,
		ticks[1],
	);
}
