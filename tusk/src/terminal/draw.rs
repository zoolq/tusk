use std::io;

use ratatui::{prelude::*, widgets::*};

use ratatui::widgets::Tabs as TabWidget;

use super::tabs::processes::draw_processes;
use super::{tabs::default::draw_default, DrawingData};

#[derive(Default, Clone, Copy)]
pub enum Screen {
	/// The default screen, a bit of everything.
	#[default]
	Default,
	/// A special process screen with process information.
	Processes,
}

impl Screen {
	fn as_string(&self) -> &str {
		match *self {
			Self::Default => "Default",
			Self::Processes => "Process",
		}
	}
}

pub struct Tabs {
	index: usize,
	tabs: Vec<Screen>,
}

impl Tabs {
	pub fn new() -> Self {
		Tabs {
			index: 0,
			tabs: vec![Screen::Default, Screen::Processes],
		}
	}

	pub fn current(&self) -> Screen {
		let index = self.index % self.tabs.len();
		self.tabs[index]
	}

	pub fn inc_index(&mut self) {
		self.index += 1;
		self.index %= self.tabs.len();
	}

	pub fn dec_index(&mut self) {
		if self.index != 0 {
			self.index -= 1;
		} else {
			self.index = self.tabs.len() - 1;
		}
		self.index %= self.tabs.len();
	}

	pub fn index(&self) -> usize {
		self.index % self.tabs.len()
	}
}

/// Wrapper function for drawing terminals
pub fn draw<B: Backend>(
	terminal: &mut Terminal<B>,
	data: &DrawingData,
	tabs: &Tabs,
) -> io::Result<()> {
	terminal.draw(|f| ui(f, data, tabs))?;
	Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, data: &DrawingData, tabs: &Tabs) {
	let size = f.size();
	let chunks = Layout::default()
		.constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
		.margin(1)
		.split(size);

	draw_tabs(f, tabs, chunks[0]);

	match tabs.current() {
		Screen::Default => draw_default(f, data, chunks[1]),
		Screen::Processes => draw_processes(f, data, chunks[1]),
	}
}

fn draw_tabs<B: Backend>(f: &mut Frame<B>, tabs: &Tabs, area: Rect) {
	let titles = tabs
		.tabs
		.iter()
		.map(|t| {
			text::Line::from(Span::styled(
				t.as_string(),
				Style::default().fg(Color::Green),
			))
		})
		.collect();

	let tabs = TabWidget::new(titles)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.style(Style::default().fg(Color::Green)),
		)
		.highlight_style(Style::default().fg(Color::Yellow))
		.select(tabs.index());

	f.render_widget(tabs, area);
}
