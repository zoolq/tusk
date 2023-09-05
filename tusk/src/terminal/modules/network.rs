use std::collections::VecDeque;

use memu::units::MegaByte;
use ratatui::{prelude::*, symbols, text::Span, widgets::*, Frame};

use crate::{
	datapoints::{NETWORK_DATAPOINTS, NETWORK_MINIMUM_HIGHEST_THRUPUT},
	terminal::App,
};

/// Draws two network graphs in the given area
pub fn draw_network<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let (min, max) = min_max(&app.network_in, &app.network_out);

	let out_data: Vec<(f64, f64)> = app
		.network_out
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_f64()))
		.collect();

	let out_dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(app.theme.graph_1)
		.data(&out_data);

	let in_data: Vec<(f64, f64)> = app
		.network_in
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_f64()))
		.collect();

	let in_dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(app.theme.graph_2)
		.data(&in_data);

	let chart = Chart::new(vec![in_dataset, out_dataset])
		.block(
			Block::default()
				.title("Network (MB/s)".bold())
				.borders(Borders::ALL)
				.border_style(app.theme.window),
		)
		.x_axis(
			Axis::default()
				.style(app.theme.axis)
				.bounds([0.0, NETWORK_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(app.theme.axis)
				.bounds([min, max])
				.labels(vec![
					Span::styled("0", app.theme.text),
					Span::styled(format!("{:.0}", (min + max) / 2.0), app.theme.text),
					Span::styled(format!("{:.0}", max), app.theme.text),
				]),
		);

	f.render_widget(chart, area);
}

fn min_max(data1: &VecDeque<MegaByte>, data2: &VecDeque<MegaByte>) -> (f64, f64) {
	let min = 0.0;
	let max1 = data1
		.iter()
		.max()
		.unwrap_or(&NETWORK_MINIMUM_HIGHEST_THRUPUT);

	let max2 = data2
		.iter()
		.max()
		.unwrap_or(&NETWORK_MINIMUM_HIGHEST_THRUPUT);

	let max = if max1 < max2 { max1 } else { max2 };
	let max = if max > &NETWORK_MINIMUM_HIGHEST_THRUPUT {
		*max
	} else {
		NETWORK_MINIMUM_HIGHEST_THRUPUT
	};
	(min, max.as_f64())
}
