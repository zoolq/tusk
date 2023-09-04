use std::{collections::VecDeque, time::Duration};

use ratatui::{prelude::*, symbols, text::Span, widgets::*, Frame};

use crate::{
	config::theme::Theme,
	datapoints::{NETWORK_DATAPOINTS, TICK_TIME},
};

/// Draws two graphs of the different tick components.
#[allow(clippy::too_many_arguments)]
pub fn draw_ticks<B: Backend>(
	f: &mut Frame<B>,
	working_data: &VecDeque<Duration>,
	real_data: &VecDeque<Duration>,
	full_tick_area: Rect,
	refresh_data: &VecDeque<Duration>,
	draw_data: &VecDeque<Duration>,
	event_data: &VecDeque<Duration>,
	split_tick_area: Rect,
	theme: &Theme,
) {
	let (min, max) = min_max(real_data);

	let working_data: Vec<(f64, f64)> = working_data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let working_dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(theme.graph_1)
		.data(&working_data);

	let real_data: Vec<(f64, f64)> = real_data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let real_dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(theme.graph_2)
		.data(&real_data);

	let chart = Chart::new(vec![working_dataset, real_dataset])
		.block(
			Block::default()
				.title("Tick Times".bold())
				.borders(Borders::ALL)
				.style(theme.window),
		)
		.x_axis(
			Axis::default()
				.style(theme.axis)
				.bounds([0.0, NETWORK_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(theme.axis)
				.bounds([min, max])
				.labels(vec![
					Span::styled("0ms", theme.text),
					Span::styled(format!("{:.0}ms", (min + max) / 2.0), theme.text),
					Span::styled(format!("{:.0}ms", max), theme.text),
				]),
		);

	f.render_widget(chart, full_tick_area);

	let refresh_data: Vec<(f64, f64)> = refresh_data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let refresh_dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(theme.graph_1)
		.data(&refresh_data);

	let draw_data: Vec<(f64, f64)> = draw_data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let draw_dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(theme.graph_2)
		.data(&draw_data);

	let event_data: Vec<(f64, f64)> = event_data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let event_dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(theme.graph_3)
		.data(&event_data);

	let chart = Chart::new(vec![refresh_dataset, draw_dataset, event_dataset])
		.block(
			Block::default()
				.title("Tick Times".bold())
				.borders(Borders::ALL)
				.style(theme.window),
		)
		.x_axis(
			Axis::default()
				.style(theme.axis)
				.bounds([0.0, NETWORK_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(theme.axis)
				.bounds([min, max])
				.labels(vec![
					Span::styled("0ms", theme.text),
					Span::styled(format!("{:.0}ms", (min + max) / 2.0), theme.text),
					Span::styled(format!("{:.0}ms", max), theme.text),
				]),
		);

	f.render_widget(chart, split_tick_area);
}

fn min_max(data: &VecDeque<Duration>) -> (f64, f64) {
	let tick_time = TICK_TIME + Duration::from_millis(2);
	let min = 0.0;
	let max = data.iter().max().unwrap_or(&tick_time);
	let max = if max > &tick_time { *max } else { tick_time };
	(min, max.as_millis() as f64)
}
