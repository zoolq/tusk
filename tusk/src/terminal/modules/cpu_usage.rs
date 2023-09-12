use std::sync::Arc;

use ratatui::{
	prelude::*,
	widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
	Frame,
};

use crate::{config::theme::THEME, datapoints::CPU_USAGE_DATAPOINTS, terminal::App};

pub fn draw_usage<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let theme = Arc::clone(&THEME);
	let data: Vec<(f64, f64)> = app
		.cpu_usage
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d as f64))
		.collect();

	let dataset = Dataset::default()
		.marker(theme.graph_style)
		.graph_type(GraphType::Line)
		.style(theme.graph_1)
		.data(&data);

	let chart = Chart::new(vec![dataset])
		.block(
			Block::default()
				.title("Cpu Usage".bold())
				.borders(Borders::ALL)
				.border_style(theme.window),
		)
		.x_axis(
			Axis::default()
				.style(theme.axis)
				.bounds([0.0, CPU_USAGE_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(theme.axis)
				.bounds([0.0, 100.0])
				.labels(vec![
					Span::styled("0%", theme.text),
					Span::styled("25%", theme.text),
					Span::styled("50%", theme.text),
					Span::styled("75%", theme.text),
					Span::styled("100%", theme.text),
				]),
		);

	f.render_widget(chart, area);
}
