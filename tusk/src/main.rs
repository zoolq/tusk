//! This is the source code for the Tusk application.
//! The Tusk application is a terminal based profiler for
//! linux based systems.

use std::{error::Error, io, thread, time::Instant};

use crossterm::{
	event::{self, DisableMouseCapture},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use datapoints::{EVENT_TIMEOUT, TICK_TIME};
use ratatui::prelude::*;
use terminal::{draw::draw, App, Screen};

use crate::terminal::events::{handle_event, ControlFlow};

/// The `terminal` module takes care of all the user interaction happening.
mod terminal;

/// All data handiling constants are defined here.
mod datapoints {
	#![allow(dead_code)]

	use std::time::Duration;

	use memu::units::MegaByte;

	/// How long a tick should be.
	/// This is 50ms in debug mode and 16 in release.
	#[cfg(debug_assertions)]
	pub const TICK_TIME: Duration = Duration::from_millis(50);
	/// How long a tick should be.
	/// This is 50ms in debug mode and 16 in release.
	#[cfg(not(debug_assertions))]
	pub const TICK_TIME: Duration = Duration::from_millis(16);
	/// How long `event::pool` should wait for events.
	pub const EVENT_TIMEOUT: Duration = Duration::from_millis(1);
	/// How many datapoints for cpu usage should be collected.
	pub const CPU_USAGE_DATAPOINTS: usize = 100;
	/// How many datapoints for the network should be collected.
	pub const NETWORK_DATAPOINTS: usize = 100;
	/// The amount of megabytes displayed at minimum by the networks graphs.
	pub const NETWORK_MINIMUM_HIGHEST_THRUPUT: MegaByte = MegaByte::from_u8(3);
	/// How many datapoints should be collected on the tracked process.
	pub const TRACKED_PROCESS_DATAPOINTS: usize = 100;
	/// The amount of megabytes displayed at minimum by the tracked process.
	pub const TRACKED_MINIMUM_HIGHEST_MEMORY: MegaByte = MegaByte::from_u8(0);
	/// How many log messages should be keept track of.
	pub const LOG_MESSAGES: usize = 100;
	/// How many ticks should be tracked in debug mode.
	pub const DEBUG_TICK_DATAPOINTS: usize = 100;
}

/// Currently hardcoded test tabs.
pub const TABS: [Screen; 3] = [Screen::Default, Screen::Processes, Screen::Tracked];

fn main() -> Result<(), Box<dyn Error>> {
	setup_logger().unwrap_or(());

	enable_raw_mode()?;
	let mut stdout = io::stdout();

	execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	let panic_hook = std::panic::take_hook();

	std::panic::set_hook(Box::new(move |panic| {
		disable_raw_mode().unwrap();
		execute!(io::stdout(), LeaveAlternateScreen).unwrap();
		panic_hook(panic);
	}));

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
	let mut app = App::new();

	loop {
		let tick_start = Instant::now();

		app.refresh();

		let draw_tick = Instant::now();
		draw(terminal, &app)?;
		app.draw_tick(draw_tick.elapsed());

		let event_tick = Instant::now();
		if event::poll(EVENT_TIMEOUT)? {
			let event = event::read()?;
			let flow = handle_event(event, &mut app);
			match flow {
				ControlFlow::Continue => (),
				ControlFlow::Quit => break,
				ControlFlow::Reload => return Ok(true),
			}
		}
		app.event_tick(event_tick.elapsed());

		app.working_tick(tick_start.elapsed());
		if tick_start.elapsed() <= TICK_TIME {
			thread::sleep(TICK_TIME - tick_start.elapsed());
		}
		app.real_tick(tick_start.elapsed());
	}
	Ok(false)
}

/// Function to set up fern logging.
fn setup_logger() -> Result<(), fern::InitError> {
	fern::Dispatch::new()
		.format(|out, message, record| {
			out.finish(format_args!(
				"[{} {}] {}",
				record.level(),
				record.target(),
				message
			))
		})
		.chain(fern::log_file("output.log")?)
		.apply()?;
	Ok(())
}
