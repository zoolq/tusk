use std::{collections::VecDeque, sync::Arc};

use memu::units::KiloByte;
use ratatui::{prelude::*, text::Span, widgets::*, Frame};

use crate::{
	config::theme::THEME,
	datapoints::{NETWORK_CUTOFF, NETWORK_DATAPOINTS, NETWORK_MAX, NETWORK_MIN},
	terminal::App,
};

/// Draws two network graphs in the given area
pub fn draw_network<B: Backend>(f: &mut Frame<B>, app: &App, area_in: Rect, area_out: Rect) {
	let theme = Arc::clone(&THEME);
	let (min, max) = min_max(&app.network_in, &app.network_out);

	let in_data: Vec<(f64, f64)> = app
		.network_in
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_f64()))
		.collect();

	let in_dataset = Dataset::default()
		.marker(theme.graph_style)
		.graph_type(GraphType::Line)
		.style(theme.graph_2)
		.data(&in_data);

	let chart = Chart::new(vec![in_dataset])
		.block(
			Block::default()
				.title("Network In (MB/s)".bold())
				.borders(Borders::ALL)
				.border_style(theme.window),
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
					Span::styled("0", theme.text),
					Span::styled(format!("{:.0}", (min + max) / 2.0), theme.text),
					Span::styled(format!("{:.0}", max), theme.text),
				]),
		);

	f.render_widget(chart, area_in);

	let out_data: Vec<(f64, f64)> = app
		.network_out
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_f64()))
		.collect();

	let out_dataset = Dataset::default()
		.marker(theme.graph_style)
		.graph_type(GraphType::Line)
		.style(theme.graph_1)
		.data(&out_data);

	let chart = Chart::new(vec![out_dataset])
		.block(
			Block::default()
				.title("Network Out (MB/s)".bold())
				.borders(Borders::ALL)
				.border_style(theme.window),
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
					Span::styled("0", theme.text),
					Span::styled(format!("{:.0}", (min + max) / 2.0), theme.text),
					Span::styled(format!("{:.0}", max), theme.text),
				]),
		);

	f.render_widget(chart, area_out);
}

fn min_max(data1: &VecDeque<KiloByte>, data2: &VecDeque<KiloByte>) -> (f64, f64) {
	let min = 0.0;
	let max1 = data1.iter().max().unwrap_or(&NETWORK_MIN);

	let max2 = data2.iter().max().unwrap_or(&NETWORK_MIN);

	let data_max = if max1 < max2 { max1 } else { max2 };
	let max = {
		match (
			data_max > &NETWORK_MIN,
			data_max > &NETWORK_CUTOFF,
			data_max > &NETWORK_MAX,
		) {
			// The value is higher than all other values.
			(true, true, true) => *data_max,
			// The value is between the max and the cutoff.
			(true, true, false) => NETWORK_MAX,
			// The value is between the min an the cutoff.
			(true, false, false) => *data_max,
			// The value is below the min.
			(false, false, false) => NETWORK_MIN,
			_ => unreachable!(concat!(
				"the equality ",
				stringify!(NETWORK_MIN),
				" < ",
				stringify!(NETWORK_CUTOFF),
				" < ",
				stringify!(NETWORK_MAX),
				" does not hold true"
			)),
		}
	};
	(min, max.as_f64())
}
