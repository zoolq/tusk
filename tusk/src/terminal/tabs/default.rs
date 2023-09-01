use ratatui::prelude::*;

use crate::terminal::{
	modules::{cpu_usage::draw_usage, network::draw_network, stats::draw_stats},
	DrawingData,
};

pub fn window_default<B: Backend>(f: &mut Frame<B>, data: &DrawingData, area: Rect) {
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

	let network_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
		.split(chunks[2]);

	draw_usage(f, &data.cpu_usage, chunks[1]);
	draw_network(
		f,
		&data.network_in,
		&data.network_out,
		network_chunks[0],
		network_chunks[1],
	);
	draw_stats(f, data, chunks[0])
}
