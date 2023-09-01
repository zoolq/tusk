use std::collections::VecDeque;

use memu::units::MegaByte;
use ratatui::{prelude::*, style::Style, symbols, text::Span, widgets::*, Frame};

use crate::datapoints::{NETWORK_DATAPOINTS, NETWORK_MINIMUM_HIGHEST_THRUPUT};

/// Draws two network graphs in the given areas
pub fn draw_network<B: Backend>(
	f: &mut Frame<B>,
	in_data: &VecDeque<MegaByte>,
	out_data: &VecDeque<MegaByte>,
	in_area: Rect,
	out_area: Rect,
) {
	draw_out(f, out_data, out_area);
	draw_in(f, in_data, in_area);
}

pub fn draw_out<B: Backend>(f: &mut Frame<B>, data: &VecDeque<MegaByte>, area: Rect) {
	let (min, max) = min_max(data);

	let data: Vec<(f64, f64)> = data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_f64()))
		.collect();

	let dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(Style::default().fg(Color::Red))
		.data(&data);

	let chart = Chart::new(vec![dataset])
		.block(
			Block::default()
				.title("Network Out (MB/s)".bold())
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Green)),
		)
		.x_axis(
			Axis::default()
				.style(Style::default().fg(Color::Cyan))
				.bounds([0.0, NETWORK_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(Style::default().fg(Color::Cyan))
				.bounds([min, max])
				.labels(vec![
					Span::styled("0", Style::default().fg(Color::Green)),
					Span::styled(
						format!("{:.0}", (min + max) / 2.0),
						Style::default().fg(Color::Green),
					),
					Span::styled(format!("{:.0}", max), Style::default().fg(Color::Green)),
				]),
		);

	f.render_widget(chart, area);
}

pub fn draw_in<B: Backend>(f: &mut Frame<B>, data: &VecDeque<MegaByte>, area: Rect) {
	let (min, max) = min_max(data);

	let data: Vec<(f64, f64)> = data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_f64()))
		.collect();

	let dataset = Dataset::default()
		.marker(symbols::Marker::Braille)
		.graph_type(GraphType::Line)
		.style(Style::default().fg(Color::Cyan))
		.data(&data);

	let chart = Chart::new(vec![dataset])
		.block(
			Block::default()
				.title("Network In (MB/s)".bold())
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Green)),
		)
		.x_axis(
			Axis::default()
				.style(Style::default().fg(Color::Cyan))
				.bounds([0.0, NETWORK_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(Style::default().fg(Color::Cyan))
				.bounds([min, max])
				.labels(vec![
					Span::styled("0", Style::default().fg(Color::Green)),
					Span::styled(
						format!("{:.0}", (min + max) / 2.0),
						Style::default().fg(Color::Green),
					),
					Span::styled(format!("{:.0}", max), Style::default().fg(Color::Green)),
				]),
		);

	f.render_widget(chart, area);
}

fn min_max(data: &VecDeque<MegaByte>) -> (f64, f64) {
	let min = 0.0;
	let max = data
		.iter()
		.max()
		.unwrap_or(&NETWORK_MINIMUM_HIGHEST_THRUPUT);
	let max = if max > &NETWORK_MINIMUM_HIGHEST_THRUPUT {
		*max
	} else {
		NETWORK_MINIMUM_HIGHEST_THRUPUT
	};
	(min, max.as_f64())
}
