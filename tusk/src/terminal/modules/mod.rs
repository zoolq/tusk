pub mod cpu_usage;
pub mod network;
pub mod stats;

use std::io;

use ratatui::{
	prelude::{Backend, Constraint, Direction, Layout},
	Frame, Terminal,
};

use self::{cpu_usage::draw_usage, network::draw_network};

use super::DrawingData;

pub fn draw_default<B: Backend>(data: &DrawingData, terminal: &mut Terminal<B>) -> io::Result<()> {
	terminal.draw(|f| ui(f, data))?;
	Ok(())
}

pub fn ui<B: Backend>(f: &mut Frame<B>, data: &DrawingData) {
	let size = f.size();
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.margin(1)
		.constraints(
			[
				Constraint::Percentage(33),
				Constraint::Percentage(33),
				Constraint::Percentage(33),
			]
			.as_ref(),
		)
		.split(size);

	let network_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.margin(1)
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
}
