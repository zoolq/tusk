use ratatui::{prelude::*, widgets::*};

use crate::terminal::DrawingData;

pub fn draw_processes<B: Backend>(f: &mut Frame<B>, data: &DrawingData, area: Rect) {
	let items: Vec<ListItem> = data
		.processes
		.iter()
		.map(|i| {
			ListItem::new(format!(
				"{}: {} {}",
				i.pid,
				i.name,
				i.memory.as_string_with_unit_and_precision(2),
			))
			.style(Style::default().fg(Color::Black).bg(Color::White))
		})
		.collect();

	let items = List::new(items)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.title("Processes")
				.style(Style::default().fg(Color::Green)),
		)
		.bg(Color::Reset);

	f.render_widget(items, area);
}
