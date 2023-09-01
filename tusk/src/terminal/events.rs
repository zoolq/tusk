use crossterm::event::{
	Event::{self, Key},
	KeyCode::{BackTab, Char, Tab},
};

use super::State;

pub enum ControlFlow {
	/// Indicates the program should continue.
	Continue,
	/// Indicates a reaload should happen.
	Reload,
	/// Indicated the program should quit.
	Quit,
}

pub fn handle_event(event: Event, state: &mut State) -> ControlFlow {
	if let Key(key) = event {
		match key.code {
			Char('q') => return ControlFlow::Quit,
			Char('r') => return ControlFlow::Reload,
			Tab => state.inc_tabs_index(),
			BackTab => state.dec_tabs_index(),
			_ => return ControlFlow::Continue,
		}
	}
	ControlFlow::Continue
}
