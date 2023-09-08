use ratatui::{
	prelude::*,
	widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
	Frame,
};

use crate::{datapoints::CPU_USAGE_DATAPOINTS, terminal::App, THEME};

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
		.marker(THEME.graph_style)
		.graph_type(GraphType::Line)
		.style(THEME.graph_1)
		.data(&data);

	let chart = Chart::new(vec![dataset])
		.block(
			Block::default()
				.title("Cpu Usage".bold())
				.borders(Borders::ALL)
				.border_style(THEME.window),
		)
		.x_axis(
			Axis::default()
				.style(THEME.axis)
				.bounds([0.0, CPU_USAGE_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(THEME.axis)
				.bounds([0.0, 100.0])
				.labels(vec![
					Span::styled("0%", THEME.text),
					Span::styled("25%", THEME.text),
					Span::styled("50%", THEME.text),
					Span::styled("75%", THEME.text),
					Span::styled("100%", THEME.text),
				]),
		);

	f.render_widget(chart, area);
}
