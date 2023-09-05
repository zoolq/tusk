use ratatui::{prelude::*, widgets::*};

use crate::config::theme::Theme;

pub fn draw_legend<B: Backend>(
	f: &mut Frame<B>,
	legend: Vec<(String, Style)>,
	theme: &Theme,
	area: Rect,
) {
	let text: Vec<Line> = legend
		.iter()
		.map(|(text, style)| Line::styled(text, *style))
		.collect();

	let paragraph = Paragraph::new(text)
		.block(Block::default().borders(Borders::ALL).style(theme.window))
		.alignment(Alignment::Right);

	f.render_widget(paragraph, area)
}
