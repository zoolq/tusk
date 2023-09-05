use std::collections::VecDeque;

use memu::units::MegaByte;
use ratatui::{prelude::*, symbols, text::Span, widgets::*, Frame};

use crate::{
	config::theme::Theme,
	datapoints::{NETWORK_DATAPOINTS, NETWORK_MINIMUM_HIGHEST_THRUPUT},
};

use super::legend::draw_legend;

/// Draws two network graphs in the given area
pub fn draw_network<B: Backend>(
	f: &mut Frame<B>,
	in_data: &VecDeque<MegaByte>,
	out_data: &VecDeque<MegaByte>,
	area: Rect,
	theme: &Theme,
) {
	let (min, max) = min_max(in_data, out_data);

	let out_data: Vec<(f64, f64)> = out_data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_f64()))
		.collect();

	let out_dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(theme.graph_1)
		.data(&out_data);

	let in_data: Vec<(f64, f64)> = in_data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_f64()))
		.collect();

	let in_dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(theme.graph_2)
		.data(&in_data);

	let chart = Chart::new(vec![in_dataset, out_dataset])
		.block(
			Block::default()
				.title("Network (MB/s)".bold())
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

	let chunks = Layout::default()
		.constraints([Constraint::Max(4), Constraint::Max(4), Constraint::Max(4)].as_ref())
		.direction(Direction::Vertical)
		.split(area);

	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(100), Constraint::Min(7)].as_ref())
		.split(chunks[0]);

	f.render_widget(chart, area);

	draw_legend(
		f,
		vec![
			("out".to_owned(), theme.graph_1),
			("in".to_owned(), theme.graph_2),
		],
		theme,
		chunks[1],
	)
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
