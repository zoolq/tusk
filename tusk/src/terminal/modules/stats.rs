use ratatui::{prelude::*, widgets::*, Frame};

use crate::{terminal::App, THEME};

pub fn draw_stats<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let text = vec![
		Line::from(format!("Name {}", app.cpu_name)),
		Line::from(format!("Frequency {}MHz", app.cpu_frequency)),
	];

	let paragraph = Paragraph::new(text.clone()).style(THEME.text).block(
		Block::default()
			.borders(Borders::ALL)
			.border_style(THEME.window),
	);
	f.render_widget(paragraph, area);
}
