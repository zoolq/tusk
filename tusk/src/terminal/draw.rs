use std::io;

use ratatui::{prelude::*, widgets::*};

use ratatui::widgets::Tabs as TabWidget;

use super::tabs::default::window_default;
use super::tabs::processes::window_processes;
use super::{Data, Screen, State};

/// Wrapper function for drawing terminals
pub fn draw<B: Backend>(terminal: &mut Terminal<B>, data: &Data, state: &State) -> io::Result<()> {
	terminal.draw(|f| ui(f, data, state))?;
	Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, data: &Data, state: &State) {
	let size = f.size();
	let chunks = Layout::default()
		.constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
		.margin(1)
		.split(size);

	draw_tabs(f, state, chunks[0]);

	match state.curret_tab() {
		Screen::Default => window_default(f, data, chunks[1]),
		Screen::Processes => window_processes(f, data, chunks[1]),
	}
}

fn draw_tabs<B: Backend>(f: &mut Frame<B>, state: &State, area: Rect) {
	let titles = state
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
		.select(state.tabs_index());

	f.render_widget(tabs, area);
}
