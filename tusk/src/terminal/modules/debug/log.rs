use std::collections::VecDeque;

use ratatui::{prelude::*, widgets::*};

pub fn draw_log<B: Backend>(f: &mut Frame<B>, data: VecDeque<String>, area: Rect) {
	let items: Vec<ListItem> = data
		.iter()
		.map(|i| ListItem::new(format!("{:?}", i)).style(Style::default().fg(Color::Cyan)))
		.collect();

	let items = List::new(items)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.title("Logs".bold())
				.style(Style::default().fg(Color::Green)),
		)
		.bg(Color::Reset);

	f.render_widget(items, area);
}
