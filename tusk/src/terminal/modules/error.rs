use std::{error::Error, fmt::Display};

use ratatui::{prelude::*, widgets::*};

use crate::config::theme::Theme;

#[derive(Debug)]
pub enum FrameError {
	MissingTracked,
	NoLog,
}

impl Display for FrameError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match *self {
			Self::MissingTracked => "no tracked process found",
			Self::NoLog => "logging isn not enabeled",
		};
		write!(f, "{}", s)
	}
}

impl Error for FrameError {}

pub fn draw_error<B: Backend>(f: &mut Frame<B>, error: FrameError, area: Rect, theme: &Theme) {
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
		.block(Block::default().borders(Borders::ALL).style(theme.error))
		.alignment(Alignment::Center);

	f.render_widget(paragraph, area)
}
