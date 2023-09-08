use std::{error::Error, fmt::Display};

use ratatui::{prelude::*, widgets::*};

use crate::THEME;

#[derive(Debug)]
pub enum FrameError {
	MissingTracked,
}

impl Display for FrameError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match *self {
			Self::MissingTracked => "no tracked process found",
		};
		write!(f, "{}", s)
	}
}

impl Error for FrameError {}

pub fn draw_error<B: Backend>(f: &mut Frame<B>, error: FrameError, area: Rect) {
	let text = match error {
		FrameError::MissingTracked => vec![
			Line::from("Error".red().bold()),
			Line::from(""),
			Line::from("No process is being tracked."),
			Line::from("Please select a process by pressing `i`."),
		],
	};

	let paragraph = Paragraph::new(text)
		.block(Block::default().borders(Borders::ALL).style(THEME.error))
		.alignment(Alignment::Center);

	f.render_widget(paragraph, area)
}
