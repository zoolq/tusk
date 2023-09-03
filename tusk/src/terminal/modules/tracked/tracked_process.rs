use std::collections::VecDeque;

use memu::units::MegaByte;
use ratatui::{prelude::*, widgets::*};

use crate::datapoints::{TRACKED_MINIMUM_HIGHEST_MEMORY, TRACKED_PROCESS_DATAPOINTS};

pub fn draw_tracked_memory<B: Backend>(f: &mut Frame<B>, data: &VecDeque<MegaByte>, area: Rect) {
	let (min, max) = min_max(data);

	let data: Vec<(f64, f64)> = data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d.as_f64()))
		.collect();

	let dataset = Dataset::default()
		.marker(Marker::Braille)
		.graph_type(GraphType::Line)
		.style(Style::default().fg(Color::Cyan))
		.data(&data);

	let chart = Chart::new(vec![dataset])
		.block(
			Block::default()
				.title("Memory Usage".bold())
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Green)),
		)
		.x_axis(
			Axis::default()
				.style(Style::default().fg(Color::Cyan))
				.bounds([0.0, TRACKED_PROCESS_DATAPOINTS as f64]),
		)
		.y_axis(
			Axis::default()
				.style(Style::default().fg(Color::Cyan))
				.bounds([min, max])
				.labels(vec![
					Span::styled(format!("{:.0}", min), Style::default().fg(Color::Green)),
					Span::styled(format!("{:.0}", max), Style::default().fg(Color::Green)),
				]),
		);

	f.render_widget(chart, area);
}

fn min_max(data: &VecDeque<MegaByte>) -> (f64, f64) {
	let min = 0.0;
	let max = data.iter().max().unwrap_or(&TRACKED_MINIMUM_HIGHEST_MEMORY);
	let max = if max > &TRACKED_MINIMUM_HIGHEST_MEMORY {
		*max
	} else {
		TRACKED_MINIMUM_HIGHEST_MEMORY
	};
	(min, max.as_f64())
}
