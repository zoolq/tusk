use ratatui::{prelude::*, widgets::*};

use crate::{terminal::App, THEME};

pub fn draw_log<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let header_cells = ["Caller", "Message"]
		.iter()
		.map(|h| Cell::from(*h).style(THEME.header));

	let header = Row::new(header_cells)
		.style(THEME.graph_1)
		.height(1)
		.bottom_margin(1);

	let rows = app.log.iter().rev().map(|i| {
		let cells = [i.caller.to_string(), i.message.to_string()];
		Row::new(cells).height(1).bottom_margin(1)
	});

	let title = format!("Log Messages {}", app.log.len());

	let table = Table::new(rows)
		.header(header)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.title(title.bold())
				.style(THEME.window),
		)
		.widths(&[Constraint::Percentage(33), Constraint::Percentage(67)]);

	f.render_widget(table, area);
}
