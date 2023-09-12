use std::sync::Arc;

use ratatui::{prelude::*, widgets::*, Frame};

use crate::{config::theme::THEME, terminal::App};

pub fn draw_stats<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let theme = Arc::clone(&THEME);
	let text = vec![
		Line::from(format!("Name {}", app.cpu_name)),
		Line::from(format!("Frequency {}MHz", app.cpu_frequency)),
	];

	let paragraph = Paragraph::new(text.clone()).style(theme.text).block(
		Block::default()
			.borders(Borders::ALL)
			.border_style(theme.window),
	);
	f.render_widget(paragraph, area);
}
