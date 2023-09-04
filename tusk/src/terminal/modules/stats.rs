use ratatui::{prelude::*, widgets::*, Frame};

use crate::terminal::App;

pub fn draw_stats<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let text = vec![
		Line::from(format!("Name {}", app.cpu_name)),
		Line::from(format!("Frequency {}MHz", app.cpu_frequency)),
	];

	let paragraph = Paragraph::new(text.clone()).style(app.theme.text).block(
		Block::default()
			.borders(Borders::ALL)
			.border_style(app.theme.window),
	);
	f.render_widget(paragraph, area);
}
