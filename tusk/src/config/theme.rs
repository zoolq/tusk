use lazy_static::lazy_static;
use ratatui::style::{Color, Style};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct Theme {
	pub window: Style,
	pub graph_1: Style,
	pub graph_2: Style,
	pub graph_3: Style,
	pub axis: Style,
	pub tab: Style,
	pub selected_tab: Style,
	pub text: Style,
	pub selected_text: Style,
	pub error: Style,
}

impl Theme {
	pub fn new() -> Self {
		Theme {
			window: Style::default().fg(Color::LightBlue),
			graph_1: Style::default().fg(Color::Blue),
			graph_2: Style::default().fg(Color::Red),
			graph_3: Style::default().fg(Color::Yellow),
			axis: Style::default().fg(Color::LightBlue),
			tab: Style::default().fg(Color::DarkGray),
			selected_tab: Style::default().fg(Color::LightBlue),
			text: Style::default().fg(Color::LightBlue),
			selected_text: Style::default().fg(Color::DarkGray).bg(Color::LightBlue),
			error: Style::default().fg(Color::Red),
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
