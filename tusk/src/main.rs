use std::{
	error::Error,
	io, thread,
	time::{Duration, Instant},
};

use terminal::draw::draw;

use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use data::{fetch_data, new_data};
use ratatui::prelude::*;
use sysinfo::{System, SystemExt};

use crate::terminal::events::{handle_event, ControlFlow};

mod data;
mod terminal;

pub const TICK_TIME: Duration = Duration::from_millis(100);

pub const EVENT_TIMEOUT: Duration = Duration::from_millis(1);

mod datapoints {
	pub const CPU_USAGE_DATAPOINTS: usize = 200;
}

fn main() -> Result<(), Box<dyn Error>> {
	enable_raw_mode()?;
	let mut stdout = io::stdout();

	execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	let app_output = run_app(&mut terminal);

	disable_raw_mode()?;
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture
	)?;
	terminal.show_cursor()?;

	if let Err(error) = app_output {
		println!("Exited with error: {}", error);
	}

	Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
	let mut sys = System::new_all();

	let mut data = new_data(&mut sys);

	loop {
		let tick_start = Instant::now();

		fetch_data(&mut sys, &mut data);

		draw(&data, terminal)?;

		if event::poll(EVENT_TIMEOUT)? {
			let event = event::read()?;
			let flow = handle_event(event);
			match flow {
				ControlFlow::Continue => (),
				ControlFlow::FunctionCalled => (),
				ControlFlow::Quit => break,
			}
		}

		if tick_start.elapsed() <= TICK_TIME {
			thread::sleep(TICK_TIME - tick_start.elapsed());
		}
	}
	Ok(())
}
