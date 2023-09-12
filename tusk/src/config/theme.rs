use std::sync::Arc;

use lazy_static::lazy_static;
use ratatui::{
	style::{Color, Style},
	symbols::Marker,
};
use serde::{Deserialize, Serialize};

lazy_static! {
	pub static ref THEME: Arc<Theme> = Arc::new(Theme::default());
}

#[derive(Clone, Copy)]
pub struct Theme {
	pub window: Style,
	pub graph_style: Marker,
	pub graph_1: Style,
	pub graph_2: Style,
	pub graph_3: Style,
	pub header: Style,
	pub axis: Style,
	pub tab: Style,
	pub selected_tab: Style,
	pub text: Style,
	pub selected_text: Style,
	pub error: Style,
}

impl Theme {
	pub const fn new() -> Self {
		Theme {
			window: Style::new().fg(Color::LightBlue).bg(Color::Reset),
			graph_style: Marker::Braille,
			graph_1: Style::new().fg(Color::Green).bg(Color::Reset),
			graph_2: Style::new().fg(Color::Red).bg(Color::Reset),
			graph_3: Style::new().fg(Color::Yellow).bg(Color::Reset),
			header: Style::new().fg(Color::LightBlue).bg(Color::Reset),
			axis: Style::new().fg(Color::LightBlue).bg(Color::Reset),
			tab: Style::new().fg(Color::DarkGray).bg(Color::Reset),
			selected_tab: Style::new().fg(Color::LightBlue).bg(Color::Reset),
			text: Style::new().fg(Color::LightBlue).bg(Color::Reset),
			selected_text: Style::new()
				.fg(Color::DarkGray)
				.bg(Color::LightBlue)
				.bg(Color::Reset),
			error: Style::new().fg(Color::Red).bg(Color::Reset),
		}
	}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WrapperTheme {
	borders: Color,
	background: Color,
	graph: Color,
	tab_bg: Color,
	tab_fg: Color,
	selected_tab_fg: Color,
	selected_tab_bg: Color,
	selected_text_bg: Color,
	selected_text_fg: Color,
}

impl Default for Theme {
	fn default() -> Self {
		Theme {
			window: Style::default().fg(Color::LightBlue).bg(Color::Reset),
			graph_style: Marker::Braille,
			graph_1: Style::default().fg(Color::Green).bg(Color::Reset),
			graph_2: Style::default().fg(Color::Red).bg(Color::Reset),
			graph_3: Style::default().fg(Color::Yellow).bg(Color::Reset),
			header: Style::default().fg(Color::LightBlue).bg(Color::Reset),
			axis: Style::default().fg(Color::LightBlue).bg(Color::Reset),
			tab: Style::default().fg(Color::DarkGray).bg(Color::Reset),
			selected_tab: Style::default().fg(Color::LightBlue).bg(Color::Reset),
			text: Style::default().fg(Color::LightBlue).bg(Color::Reset),
			selected_text: Style::default()
				.fg(Color::DarkGray)
				.bg(Color::LightBlue)
				.bg(Color::Reset),
			error: Style::default().fg(Color::Red).bg(Color::Reset),
		}
	}
}
