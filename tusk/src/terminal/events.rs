use crossterm::event::{
	Event::{self, Key},
	KeyCode::Char,
};

#[derive(Default)]
pub enum ControlFlow {
	// Indicates the program should continue.
	#[default]
	Continue,
	Reload,
	// Indicated the program should quit.
	Quit,
}

pub fn handle_event(event: Event) -> ControlFlow {
	if let Key(key) = event {
		match key.code {
			Char('q') => return ControlFlow::Quit,
			Char('r') => return ControlFlow::Reload,
			_ => return ControlFlow::default(),
		}
	}
	ControlFlow::Continue
}
