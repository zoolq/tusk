use std::io;
use std::sync::Arc;

use namefn::namefn;
use ratatui::{prelude::*, widgets::*};

use ratatui::widgets::Tabs as TabWidget;

use crate::config::theme::THEME;

use super::tabs::debug::window_debug;
use super::tabs::default::window_default;
use super::tabs::processes::window_processes;
use super::tabs::tracked::window_tracked;
use super::{App, Screen, TopBar};

/// Wrapper function for drawing terminals
#[namefn]
pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
	app.log(format!("Drawing {:?}", app.current_tab()), NAME);
	terminal.draw(|f| ui(f, app))?;
	Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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
fn draw_input<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
	let theme = Arc::clone(&THEME);

	let input = Paragraph::new(app.input.as_str()).style(theme.text).block(
		Block::default()
			.borders(Borders::ALL)
			.title("Input")
			.style(theme.window),
	);
	f.render_widget(input, area);
	f.set_cursor(area.x + app.input_position as u16 + 1, area.y + 1)
}

/// Draws the tabs top bar.
fn draw_tabs<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
	let theme = Arc::clone(&THEME);

	let titles = app
		.tabs
		.iter()
		.map(|t| text::Line::from(Span::styled(t.as_string(), theme.tab)))
		.collect();

	let tabs = TabWidget::new(titles)
		.block(Block::default().borders(Borders::ALL).style(theme.window))
		.highlight_style(theme.selected_tab)
		.select(app.tabs_index());

	f.render_widget(tabs, area);
}
