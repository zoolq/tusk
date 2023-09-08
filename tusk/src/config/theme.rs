use ratatui::{
	style::{Color, Style},
	symbols::Marker,
};
use serde::{Deserialize, Serialize};

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
	#[serde(alias = "borderColor")]
	#[serde(default = "borders")]
	borders: Color,
	#[serde(alias = "backgroundColor")]
	#[serde(default = "background")]
	background: Color,
	#[serde(alias = "graphColor")]
	#[serde(default = "graph")]
	graph: Color,
	#[serde(default = "tab_bg")]
	tab_bg: Color,
	#[serde(default = "tab_fg")]
	tab_fg: Color,
	#[serde(default = "selected_tab_fg")]
	selected_tab_fg: Color,
	#[serde(alias = "selectedTab")]
	#[serde(default = "selected_tab_bg")]
	selected_tab_bg: Color,
	#[serde(default = "selected_text_bg")]
	selected_text_bg: Color,
	#[serde(default = "selected_text_fg")]
	selected_text_fg: Color,
}

fn background() -> Color {
	Color::Reset
}

fn borders() -> Color {
	Color::DarkGray
}

fn graph() -> Color {
	Color::LightBlue
}

fn selected_tab_fg() -> Color {
	Color::DarkGray
}

fn selected_tab_bg() -> Color {
	Color::LightBlue
}

fn tab_fg() -> Color {
	Color::Reset
}

fn tab_bg() -> Color {
	Color::Blue
}

fn selected_text_bg() -> Color {
	Color::LightBlue
}

fn selected_text_fg() -> Color {
	Color::DarkGray
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
