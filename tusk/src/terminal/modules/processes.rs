use ratatui::{prelude::*, widgets::*};

use crate::{config::theme::Theme, terminal::Process};

pub fn draw_processes<B: Backend>(f: &mut Frame<B>, data: &[Process], area: Rect, theme: &Theme) {
	let items: Vec<ListItem> = data
		.iter()
		.map(|i| {
			ListItem::new(format!(
				"{:6}: {:10} {:10} {:10} {:10} {:10} {:10}",
				i.pid,
				i.name,
				i.memory.as_string_with_unit_and_precision(2),
				i.cpu_usage,
				i.time,
				i.total_written,
				i.total_read
			))
			.style(theme.text)
		})
		.collect();

	let items = List::new(items)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.title("Processes".bold())
				.style(theme.window),
		)
		.bg(Color::Reset);

	f.render_widget(items, area);
}
