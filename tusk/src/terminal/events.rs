use std::{default, f64::consts::E, io};

use crossterm::event::{self, Event::Key, KeyCode::Char};

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

pub fn handle_event() -> io::Result<ControlFlow> {
	let event = event::read()?;
	if let Key(key) = event {
		match key.code {
			Char('q') => return Ok(ControlFlow::Quit),
			_ => return Ok(ControlFlow::default()),
		}
	}
	Ok(ControlFlow::Continue)
}
