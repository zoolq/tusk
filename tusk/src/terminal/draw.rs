use std::{collections::VecDeque, io};

use crossterm::style::style;
use ratatui::{prelude::*, widgets::*};

use super::DrawingData;

pub fn draw<B: Backend>(data: &DrawingData, terminal: &mut Terminal<B>) -> io::Result<()> {
	terminal.draw(|f| ui(f, data))?;
	Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, data: &DrawingData) {
	let size = f.size();
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Ratio(1, 3), Constraint::Ratio(1, 2)].as_ref())
		.split(size);

	draw_chart(f, &data.cpu_usage, chunks[1]);
}

fn draw_chart<B: Backend>(f: &mut Frame<B>, data: &VecDeque<f32>, area: Rect) {
	let data: Vec<(f64, f64)> = data
		.iter()
		.enumerate()
		.map(|(i, &d)| (i as f64, d as f64))
		.collect();

	let dataset = Dataset::default()
		.name("cpu_usage")
		.marker(symbols::Marker::Dot)
		.graph_type(GraphType::Line)
		.style(Style::default().fg(Color::Cyan))
		.data(&data);

	let chart = Chart::new(vec![dataset])
		.block(
			Block::default()
				.title("")
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Green)),
		)
		.x_axis(
			Axis::default()
				.title(Span::styled("Time", Style::default().fg(Color::Green)))
				.style(Style::default().fg(Color::Cyan))
				.bounds([0.0, data.len() as f64]),
		)
		.y_axis(
			Axis::default()
				.title(Span::styled("Usage", Style::default().fg(Color::Green)))
				.style(Style::default().fg(Color::Cyan))
				.bounds([0.0, 100.0])
				.labels(vec![
					Span::styled("0%", Style::default().fg(Color::Green)),
					Span::styled("50%", Style::default().fg(Color::Green)),
					Span::styled("100%", Style::default().fg(Color::Green)),
				]),
		);

	f.render_widget(chart, area);
}
