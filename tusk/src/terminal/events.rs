use crossterm::event::{
	Event::{self, Key},
	KeyCode::{BackTab, Char, Tab},
};

use super::draw::Tabs;

pub enum ControlFlow {
	/// Indicates the program should continue.
	Continue,
	/// Indicates a reaload should happen.
	Reload,
	/// Indicated the program should quit.
	Quit,
}

pub fn handle_event(event: Event, tabs: &mut Tabs) -> ControlFlow {
	if let Key(key) = event {
		match key.code {
			Char('q') => return ControlFlow::Quit,
			Char('r') => return ControlFlow::Reload,
			Tab => tabs.inc_index(),
			BackTab => tabs.dec_index(),
			_ => return ControlFlow::Continue,
		}
	}
	ControlFlow::Continue
}
