use ratatui::{
	prelude::*,
	symbols::Marker,
	widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
	Frame,
};

use crate::{datapoints::CPU_USAGE_DATAPOINTS, terminal::App};

pub fn draw_tracked_usage<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let data: Vec<(f64, f64)> = app
		.tracked
		.as_ref()
		.unwrap()
		.cpu_usage
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d as f64))
		.collect();

	let dataset = Dataset::default()
		.marker(Marker::Braille)
		.graph_type(GraphType::Line)
		.style(app.theme.graph_1)
		.data(&data);

	let chart = Chart::new(vec![dataset])
		.block(
			Block::default()
				.title("Cpu Usage".bold())
				.borders(Borders::ALL)
				.border_style(app.theme.window),
		)
		.x_axis(
			Axis::default()
				.style(app.theme.axis)
				.bounds([0.0, CPU_USAGE_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(app.theme.axis)
				.bounds([0.0, 100.0])
				.labels(vec![
					Span::styled("0%", app.theme.text),
					Span::styled("25%", app.theme.text),
					Span::styled("50%", app.theme.text),
					Span::styled("75%", app.theme.text),
					Span::styled("100%", app.theme.text),
				]),
		);

	f.render_widget(chart, area);
}
