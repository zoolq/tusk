use std::{collections::VecDeque, time::Duration};

use ratatui::{prelude::*, text::Span, widgets::*, Frame};

use crate::{
	datapoints::{NETWORK_DATAPOINTS, TICK_TIME},
	terminal::App,
	THEME,
};

/// Draws two graphs of the different tick components.
#[allow(clippy::too_many_arguments)]
pub fn draw_ticks<B: Backend>(
	f: &mut Frame<B>,
	app: &App,
	full_tick_area: Rect,
	split_tick_area: Rect,
) {
	let (min, max) = min_max(&app.real_tick);

	let working_data: Vec<(f64, f64)> = app
		.working_tick
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let working_dataset = Dataset::default()
		.marker(THEME.graph_style)
		.graph_type(GraphType::Line)
		.style(THEME.graph_1)
		.data(&working_data);

	let real_data: Vec<(f64, f64)> = app
		.real_tick
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let real_dataset = Dataset::default()
		.marker(THEME.graph_style)
		.graph_type(GraphType::Line)
		.style(THEME.graph_2)
		.data(&real_data);

	let chart = Chart::new(vec![working_dataset, real_dataset])
		.block(
			Block::default()
				.title(Line::from(vec![
					Span::styled("Tick Times", THEME.window),
					Span::styled(" (Real)", THEME.graph_1),
					Span::styled(" (Processing)", THEME.graph_2),
				]))
				.borders(Borders::ALL)
				.style(THEME.window),
		)
		.x_axis(
			Axis::default()
				.style(THEME.axis)
				.bounds([0.0, NETWORK_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(THEME.axis)
				.bounds([min, max])
				.labels(vec![
					Span::styled("0ms", THEME.text),
					Span::styled(format!("{:.0}ms", (min + max) / 2.0), THEME.text),
					Span::styled(format!("{:.0}ms", max), THEME.text),
				]),
		);

	f.render_widget(chart, full_tick_area);

	let refresh_data: Vec<(f64, f64)> = app
		.refresh_tick
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let refresh_dataset = Dataset::default()
		.marker(THEME.graph_style)
		.graph_type(GraphType::Line)
		.style(THEME.graph_1)
		.data(&refresh_data);

	let draw_data: Vec<(f64, f64)> = app
		.drawing_tick
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let draw_dataset = Dataset::default()
		.marker(THEME.graph_style)
		.graph_type(GraphType::Line)
		.style(THEME.graph_2)
		.data(&draw_data);

	let event_data: Vec<(f64, f64)> = app
		.event_tick
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_millis() as f64))
		.collect();

	let event_dataset = Dataset::default()
		.marker(THEME.graph_style)
		.graph_type(GraphType::Line)
		.style(THEME.graph_3)
		.data(&event_data);

	let chart = Chart::new(vec![refresh_dataset, draw_dataset, event_dataset])
		.block(
			Block::default()
				.title(Line::from(vec![
					Span::styled("Tick Times", THEME.window),
					Span::styled(" (Refresh)", THEME.graph_1),
					Span::styled(" (Draw)", THEME.graph_2),
					Span::styled(" (Event)", THEME.graph_3),
				]))
				.borders(Borders::ALL)
				.style(THEME.window),
		)
		.x_axis(
			Axis::default()
				.style(THEME.axis)
				.bounds([0.0, NETWORK_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(THEME.axis)
				.bounds([min, max])
				.labels(vec![
					Span::styled("0ms", THEME.text),
					Span::styled(format!("{:.0}ms", (min + max) / 2.0), THEME.text),
					Span::styled(format!("{:.0}ms", max), THEME.text),
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
