use ratatui::{prelude::*, widgets::*};

pub enum FrameError {
	MissingTracked,
	NoLog,
}

pub fn draw_error<B: Backend>(f: &mut Frame<B>, error: FrameError, area: Rect) {
	let text = match error {
		FrameError::MissingTracked => vec![
			Line::from("Error".red().bold()),
			Line::from(""),
			Line::from("No process is being tracked."),
			Line::from("Please select a process by pressing `i`."),
		],
		FrameError::NoLog => vec![
			Line::from("**Error**".red().bold()),
			Line::from(""),
			Line::from("Logging is not enabeled."),
		],
	};

	let paragraph = Paragraph::new(text)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.style(Style::default().fg(Color::Green)),
		)
		.alignment(Alignment::Center);

	f.render_widget(paragraph, area)
}
