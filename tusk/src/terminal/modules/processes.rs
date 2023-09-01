use ratatui::{prelude::*, widgets::*};

use crate::terminal::Process;

pub fn draw_processes<B: Backend>(f: &mut Frame<B>, data: &[Process], area: Rect) {
	let items: Vec<ListItem> = data
		.iter()
		.map(|i| {
			ListItem::new(format!(
				"{}: {} {}",
				i.pid,
				i.name,
				i.memory.as_string_with_unit_and_precision(2),
			))
			.style(Style::default().fg(Color::Black))
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
