use std::sync::Arc;

use ratatui::{prelude::*, widgets::*};

use crate::{config::theme::THEME, terminal::App};

pub fn draw_processes<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let theme = Arc::clone(&THEME);
	let header_cells = ["Pid", "Name", "Memory", "Cpu", "Time", "Written", "Read"]
		.iter()
		.map(|h| Cell::from(*h).style(theme.header));

	let header = Row::new(header_cells)
		.style(theme.graph_1)
		.height(1)
		.bottom_margin(1);

	let rows = app.processes.iter().map(|i| {
		let cells = [
			format!("{}", i.pid),
			i.name.to_string(),
			i.memory.as_string_with_unit_and_precision(2),
			format!("{:.2}%", i.cpu_usage),
			format!("{}", i.time),
			format!("{}", i.total_written),
			format!("{}", i.total_read),
		];
		Row::new(cells).height(1).bottom_margin(1)
	});

	let table = Table::new(rows)
		.header(header)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.title("Processes".bold())
				.style(theme.window),
		)
		.widths(&[
			Constraint::Max(6),
			Constraint::Max(20),
			Constraint::Max(10),
			Constraint::Max(10),
			Constraint::Max(10),
			Constraint::Max(10),
		]);

	f.render_widget(table, area);
}
