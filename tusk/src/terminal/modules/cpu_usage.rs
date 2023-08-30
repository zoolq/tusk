use std::collections::VecDeque;

use ratatui::{
	prelude::*,
	symbols::Marker,
	widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
	Frame,
};

pub fn draw_usage<B: Backend>(f: &mut Frame<B>, data: &VecDeque<f32>, area: Rect) {
	let data: Vec<(f64, f64)> = data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d as f64))
		.collect();

	let dataset = Dataset::default()
		.marker(Marker::Braille)
		.graph_type(GraphType::Line)
		.style(Style::default().fg(Color::Cyan))
		.data(&data);

	let chart = Chart::new(vec![dataset])
		.block(
			Block::default()
				.title("Cpu Usage")
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Green)),
		)
		.x_axis(
			Axis::default()
				.style(Style::default().fg(Color::Cyan))
				.bounds([0.0, data.len() as f64]),
		)
		.y_axis(
			Axis::default()
				.style(Style::default().fg(Color::Cyan))
				.bounds([0.0, 100.0])
				.labels(vec![
					Span::styled("0%", Style::default().fg(Color::Green)),
					Span::styled("50%", Style::default().fg(Color::Green)),
					Span::styled("100%", Style::default().fg(Color::Green)),
				]),
		);

	f.render_widget(chart, area);
}
