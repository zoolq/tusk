use crossterm::event::{
	Event::{self, Key},
	KeyCode::Char,
};

#[derive(Default)]
pub enum ControlFlow {
	// Indicates the `handle_event` function called another function.
	FunctionCalled,
	// Indicates the program should continue.
	#[default]
	Continue,
	// Indicated the program should quit.
	Quit,
}

pub fn handle_event(event: Event) -> ControlFlow {
	if let Key(key) = event {
		match key.code {
			Char('q') => return ControlFlow::Quit,
			_ => return ControlFlow::default(),
		}
	}
	ControlFlow::Continue
}
