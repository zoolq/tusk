use ratatui::{prelude::*, widgets::*, Frame};

use crate::terminal::DrawingData;

pub fn draw_stats<B: Backend>(f: &mut Frame<B>, data: &DrawingData, area: Rect) {
	let text = vec![
		Line::from(format!("Name {}", data.cpu_name)),
		Line::from(format!("Frequency {}MHz", data.cpu_frequency)),
	];

	let paragraph = Paragraph::new(text.clone())
		.style(Style::default().fg(Color::Gray))
		.block(
			Block::default()
				.borders(Borders::ALL)
				.border_style(Style::default().fg(Color::Green)),
		);
	f.render_widget(paragraph, area);
}
