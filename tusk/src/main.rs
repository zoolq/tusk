use std::{error::Error, io, thread, time::Instant};

use crossterm::{
	event::{self, DisableMouseCapture},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use data::{fetch_data, new_data, DataStorage};
use datapoints::{EVENT_TIMEOUT, TICK_TIME};
use ratatui::prelude::*;
use sysinfo::{System, SystemExt};
use terminal::draw::{draw, Tabs};

use crate::terminal::events::{handle_event, ControlFlow};

mod data;
mod terminal;

mod datapoints {
	use std::time::Duration;

	use memu::units::MegaByte;

	pub const TICK_TIME: Duration = Duration::from_millis(100);
	pub const EVENT_TIMEOUT: Duration = Duration::from_millis(0);
	pub const CPU_USAGE_DATAPOINTS: usize = 100;
	pub const NETWORK_DATAPOINTS: usize = 100;
	pub const NETWORK_MINIMUM_HIGHEST_THRUPUT: MegaByte = MegaByte::from_u8(3);
}

fn main() -> Result<(), Box<dyn Error>> {
	setup_logger().unwrap();

	enable_raw_mode()?;
	let mut stdout = io::stdout();

	execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	let mut app_output = run_app(&mut terminal);

	while let Ok(true) = app_output {
		app_output = run_app(&mut terminal);
	}

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

/// If this returns true a reaload should be initiated
///
/// The order of calls should always be this way:
///
/// Prior to looping:
///
/// - New system
/// - New data
/// - New storage
///
/// In each tick:
///
/// - fetch_data
/// - draw
/// - handle keys
///
fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<bool> {
	let mut tabs = Tabs::new();

	let mut sys = System::new_all();

	let mut data = new_data(&mut sys);

	let mut storage = DataStorage::new();

	loop {
		let tick_start = Instant::now();

		fetch_data(&mut sys, &mut data, &mut storage);

		draw(terminal, &data, &tabs)?;

		if event::poll(EVENT_TIMEOUT)? {
			let event = event::read()?;
			let flow = handle_event(event, &mut tabs);
			match flow {
				ControlFlow::Continue => (),
				ControlFlow::Quit => break,
				ControlFlow::Reload => return Ok(true),
			}
		}

		if tick_start.elapsed() <= TICK_TIME {
			thread::sleep(TICK_TIME - tick_start.elapsed());
		}
	}
	Ok(false)
}

fn setup_logger() -> Result<(), fern::InitError> {
	fern::Dispatch::new()
		.format(|out, message, _| out.finish(format_args!("{}", message)))
		.chain(fern::log_file("output.log")?)
		.apply()?;
	Ok(())
}
