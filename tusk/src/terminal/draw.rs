use std::io;

use ratatui::{prelude::*, widgets::*};

use ratatui::widgets::Tabs as TabWidget;

use super::tabs::debug::window_debug;
use super::tabs::default::window_default;
use super::tabs::processes::window_processes;
use super::tabs::tracked::window_tracked;
use super::{App, Screen, TopBar};

/// Wrapper function for drawing terminals
pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> io::Result<()> {
	terminal.draw(|f| ui(f, app))?;
	Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
	let size = f.size();
	let chunks = Layout::default()
		.constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
		.margin(1)
		.split(size);

	match app.top_bar {
		TopBar::Tabs => draw_tabs(f, app, chunks[0]),
		TopBar::Input => draw_input(f, app, chunks[0]),
	}

	match app.current_tab() {
		Screen::Default => window_default(f, app, chunks[1]),
		Screen::Processes => window_processes(f, app, chunks[1]),
		Screen::Tracked => window_tracked(f, app, chunks[1]),
		Screen::Debug => window_debug(f, app, chunks[1]),
	}
}

/// Draws the input top bar.
fn draw_input<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let input = Paragraph::new(app.input.as_str())
		.style(Style::default().fg(Color::Green))
		.block(
			Block::default()
				.borders(Borders::ALL)
				.title("Input")
				.style(Style::default().fg(Color::Green)),
		);
	f.render_widget(input, area);
	f.set_cursor(area.x + app.input_position as u16 + 1, area.y + 1)
}

/// Draws the tabs top bar.
fn draw_tabs<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
	let titles = app
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
		.select(app.tabs_index());

	f.render_widget(tabs, area);
}
