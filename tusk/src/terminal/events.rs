use crossterm::event::{
	Event::{self, Key},
	KeyCode::{BackTab, Backspace, Char, Enter, Esc, Left, Right, Tab, F},
};

use super::{App, TopBar};

/// Controls the flow of the app, based on user inputs.
pub enum ControlFlow {
	/// Indicates the program should continue.
	Continue,
	/// Indicates a reaload should happen.
	Reload,
	/// Indicated the program should quit.
	Quit,
}

/// Handeles user input events.
pub fn handle_event(event: Event, app: &mut App) -> ControlFlow {
	if let Key(key) = event {
		if app.top_bar == TopBar::Input {
			match key.code {
				Char(ch) => app.input_type(ch),
				Backspace => app.input_backspace(),
				Left | Right => app.arrow_event(key.code),
				Enter => app.input_enter(),
				Esc => app.wipe_input(),
				Tab => app.inc_tabs_index(),
				BackTab => app.dec_tabs_index(),
				_ => (),
			}
		} else {
			match key.code {
				Char('q') => return ControlFlow::Quit,
				Char('r') => return ControlFlow::Reload,
				Char('i') => app.top_bar = TopBar::Input,
				Tab => app.inc_tabs_index(),
				BackTab => app.dec_tabs_index(),
				#[cfg(debug_assertions)]
				Char('p') => panic!("intentional debug panic"),
				F(1) => app.switch_debug(),
				_ => return ControlFlow::Continue,
			}
		}
	}
	ControlFlow::Continue
}
