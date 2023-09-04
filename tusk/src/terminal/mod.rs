/// Utilities `App` needs.
mod app_util;
/// Wrapper functions for drawing
pub(super) mod draw;
/// Wrapper functions for handling events.
pub(super) mod events;
/// This contains the windows drawn on the terminal.
mod modules;
/// This contains wrapper function for drawing tabs.
mod tabs;

use std::{
	collections::VecDeque,
	str::FromStr,
	time::{Duration, Instant},
};

use crossterm::event::KeyCode::{self, Down, Left, Right, Up};
use log::error;
use memu::{
	units::{KiloByte, MegaByte},
	MemoryUnit,
};
use ratatui::widgets::ScrollbarState;
use sysinfo::{CpuExt, Pid, PidExt, Process as Proc, ProcessExt, ProcessStatus, System, SystemExt};

use crate::{
	config::theme::Theme,
	datapoints::{
		CPU_USAGE_DATAPOINTS, DEBUG_TICK_DATAPOINTS, NETWORK_DATAPOINTS, TRACKED_PROCESS_DATAPOINTS,
	},
	TABS,
};

use self::app_util::{compute_frequency, compute_in, compute_out, compute_usage, per_second};

/// Defines which screen is drawn.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
	/// The default screen, a bit of everything.
	#[default]
	Default,
	/// A special process screen with process information.
	Processes,
	/// Tracked process screen
	Tracked,
	/// A debug screen only for developers. Enable by pressing
	Debug,
}

impl Screen {
	/// Gets the screens name, used for displaying.
	fn as_string(&self) -> &str {
		match *self {
			Self::Default => "Default",
			Self::Processes => "Process",
			Self::Tracked => "Tracked",
			Self::Debug => "Debug",
		}
	}
}

/// Defines what goes in the top bar.
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum TopBar {
	/// Display all tabs in the top bar.
	#[default]
	Tabs,
	/// Display input in the top bar.
	Input,
}

/// The main app struct handeling all app relevent information.
pub struct App {
	sys: System,
	pub theme: Theme,
	pub programs_scroll_state: ScrollbarState,
	tabs_index: usize,
	tabs: Vec<Screen>,
	top_bar: TopBar,
	input: String,
	input_position: usize,
	pub cpu_name: String,
	pub cpu_frequency: u64,
	pub cpu_usage: VecDeque<f32>,
	pub network_out: VecDeque<MegaByte>,
	pub network_in: VecDeque<MegaByte>,
	pub processes: Vec<Process>,
	tracked_pid: Option<Pid>,
	pub tracked: Option<TrackedProcess>,
	last_snapshot: Instant,
	/// Staches the current working tick time to ensure all ticks display the same refresh.
	working_prior: Duration,
	/// How long the data and drawing took to process.
	pub working_tick: VecDeque<Duration>,
	/// How long the tick really took
	pub real_tick: VecDeque<Duration>,
	/// Staches the current fetiching tick time to ensure all ticks display the same refresh.
	refresh_prior: Duration,
	/// Data refresh time per tick.
	pub refresh_tick: VecDeque<Duration>,
	/// Drawing time per tick.
	pub drawing_tick: VecDeque<Duration>,
	/// Time handeling events per tick.
	pub event_tick: VecDeque<Duration>,
}

impl App {
	/// Creates a new app.
	pub fn new() -> Self {
		let mut sys = System::new_all();

		sys.refresh_all();

		let mut app = App {
			theme: Theme::new(),
			programs_scroll_state: ScrollbarState::default(),
			tabs_index: 0,
			tabs: Vec::from(TABS),
			top_bar: TopBar::default(),
			input: String::new(),
			input_position: 0,
			cpu_name: sys.cpus().first().unwrap().brand().to_owned(),
			cpu_frequency: 0,
			cpu_usage: VecDeque::with_capacity(CPU_USAGE_DATAPOINTS),
			network_in: VecDeque::with_capacity(NETWORK_DATAPOINTS),
			network_out: VecDeque::with_capacity(NETWORK_DATAPOINTS),
			processes: Vec::new(),
			tracked_pid: None,
			tracked: None,
			last_snapshot: Instant::now(),
			working_prior: Duration::from_micros(0),
			working_tick: VecDeque::with_capacity(DEBUG_TICK_DATAPOINTS),
			real_tick: VecDeque::with_capacity(DEBUG_TICK_DATAPOINTS),
			refresh_prior: Duration::from_micros(0),
			refresh_tick: VecDeque::with_capacity(DEBUG_TICK_DATAPOINTS),
			drawing_tick: VecDeque::with_capacity(DEBUG_TICK_DATAPOINTS),
			event_tick: VecDeque::with_capacity(DEBUG_TICK_DATAPOINTS),
			sys,
		};

		app.refresh();

		app
	}

	/// Refreshes the data.
	pub fn refresh(&mut self) {
		let tick = Instant::now();

		self.sys.refresh_cpu();
		let elapsed = self.last_snapshot.elapsed();
		self.sys.refresh_networks();
		self.last_snapshot = Instant::now();
		self.sys.refresh_processes();

		let cpus = self.sys.cpus();
		self.cpu_frequency = compute_frequency(cpus);
		if self.cpu_usage.len() == CPU_USAGE_DATAPOINTS {
			self.cpu_usage.pop_front();
		}
		self.cpu_usage.push_back(compute_usage(cpus));

		let networks = self.sys.networks();
		let network_in_last_sec = compute_in(networks);
		if self.network_in.len() == NETWORK_DATAPOINTS {
			self.network_in.pop_front();
		}
		self.network_in
			.push_back(per_second(network_in_last_sec, elapsed));

		let network_out_last_sec = compute_out(networks);
		if self.network_out.len() == NETWORK_DATAPOINTS {
			self.network_out.pop_front();
		}

		self.network_out
			.push_back(per_second(network_out_last_sec, elapsed));

		let processes = self.sys.processes();
		self.processes = Vec::with_capacity(processes.len());
		for (pid, process) in processes {
			self.processes.push(Process::from_pp(pid, process))
		}

		self.processes
			.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());

		if let Some(pid) = self.tracked_pid {
			if let Some(tracked_process) = &mut self.tracked {
				if let Some(process) = processes.get(&pid) {
					tracked_process.refresh(process, elapsed);
				} else {
					self.tracked_pid = None;
				}
			} else if let Some(process) = processes.get(&pid) {
				self.tracked = Some(TrackedProcess::new(&pid, process));
				if let Some(tracked_process) = &mut self.tracked {
					tracked_process.refresh(process, elapsed);
				}
			} else {
				self.tracked_pid = None;
			}
		}

		if self.refresh_tick.len() == DEBUG_TICK_DATAPOINTS {
			self.refresh_tick.pop_front();
		}
		self.refresh_tick.push_back(self.refresh_prior);
		self.refresh_prior = tick.elapsed();
	}

	/// Gets the current tab.
	pub fn current_tab(&self) -> Screen {
		let tabs_index = self.tabs_index % self.tabs.len();
		self.tabs[tabs_index]
	}

	/// Increments the tab index by 1.
	pub fn inc_tabs_index(&mut self) {
		self.tabs_index += 1;
		self.tabs_index %= self.tabs.len();
	}

	/// Decreases the tab index by 1.
	pub fn dec_tabs_index(&mut self) {
		if self.tabs_index != 0 {
			self.tabs_index -= 1;
		} else {
			self.tabs_index = self.tabs.len() - 1;
		}
		self.tabs_index %= self.tabs.len();
	}

	/// Gets the index of the current tab.
	pub fn tabs_index(&self) -> usize {
		self.tabs_index % self.tabs.len()
	}

	/// Inputs the given char.
	pub fn input_type(&mut self, ch: char) {
		self.input.insert(self.input_position, ch);
		self.input_position += 1;
	}

	/// Deletes the char prior to the cursor.
	pub fn input_backspace(&mut self) {
		if self.input_position > 0 {
			self.input_position -= 1;

			let before_char_to_delete = self.input.chars().take(self.input_position);

			let after_char_to_delete = self.input.chars().skip(self.input_position + 1);

			self.input = before_char_to_delete.chain(after_char_to_delete).collect();
		}
	}

	/// Moves the cursor in the given direction.
	/// Up and down move to the start and end of line respectively.
	pub fn arrow_event(&mut self, arrow: KeyCode) {
		match arrow {
			Left => {
				if self.input_position > 0 {
					self.input_position -= 1;
				}
			},
			Right => {
				if self.input_position > self.input.len() {
					self.input_position += 1;
				}
			},
			Up => self.input_position = 0,
			Down => self.input_position = self.input.len(),
			_ => (),
		}
	}

	/// Clears the input string.
	pub fn wipe_input(&mut self) {
		self.top_bar = TopBar::Tabs;
		self.input.clear();
		self.input_position = 0;
	}

	/// Enters the input as tracked pid.
	pub fn input_enter(&mut self) {
		let pid = match Pid::from_str(&self.input) {
			Ok(pid) => pid,
			Err(e) => {
				error!("{}", e);
				return;
			},
		};

		self.tracked_pid = Some(pid);

		self.wipe_input();
	}

	/// Makes the debug panel visible.
	pub fn enable_debug(&mut self) {
		if !self.tabs.contains(&Screen::Debug) {
			self.tabs.push(Screen::Debug)
		}
	}

	/// Makes the debug panel invisible.
	pub fn disable_debug(&mut self) {
		self.tabs = self
			.tabs
			.iter()
			.filter(|&&a| a != Screen::Debug)
			.map(|a| a.to_owned())
			.collect();
	}

	/// Switches the debug panel on or off depending on prior state.
	pub fn switch_debug(&mut self) {
		if self.tabs.contains(&Screen::Debug) {
			self.disable_debug()
		} else {
			self.enable_debug()
		}
	}

	/// Add the working tick time to app.
	pub fn working_tick(&mut self, tick: Duration) {
		if self.working_tick.len() == DEBUG_TICK_DATAPOINTS {
			self.working_tick.pop_front();
		}
		self.working_tick.push_back(self.working_prior);
		self.working_prior = tick;
	}

	/// Add the real tick time to app.
	pub fn real_tick(&mut self, tick: Duration) {
		if self.real_tick.len() == DEBUG_TICK_DATAPOINTS {
			self.real_tick.pop_front();
		}
		self.real_tick.push_back(tick);
	}

	/// Add the drawing tick time to app.
	pub fn draw_tick(&mut self, tick: Duration) {
		if self.drawing_tick.len() == DEBUG_TICK_DATAPOINTS {
			self.drawing_tick.pop_front();
		}
		self.drawing_tick.push_back(tick);
	}

	/// Add the event tick time to app.
	pub fn event_tick(&mut self, tick: Duration) {
		if self.event_tick.len() == DEBUG_TICK_DATAPOINTS {
			self.event_tick.pop_front();
		}
		self.event_tick.push_back(tick);
	}
}

/// A tracked process.
#[derive(Debug)]
pub struct TrackedProcess {
	pub pid: u32,
	pub name: String,
	pub time: u64,
	pub status: ProcessStatus,
	pub total_written: MegaByte,
	pub total_read: MegaByte,
	pub cpu_usage: VecDeque<f32>,
	pub memory: VecDeque<MegaByte>,
	pub written: VecDeque<KiloByte>,
	pub read: VecDeque<KiloByte>,
}

impl TrackedProcess {
	/// Create a new tracked process from pid and process.
	pub fn new(pid: &Pid, process: &Proc) -> Self {
		let disk_usage = process.disk_usage();

		TrackedProcess {
			pid: pid.as_u32(),
			name: process.name().to_owned(),
			time: process.run_time(),
			status: process.status(),
			total_written: MegaByte::new(disk_usage.total_written_bytes),
			total_read: MegaByte::new(disk_usage.total_read_bytes),
			cpu_usage: VecDeque::from(vec![process.cpu_usage()]),
			memory: VecDeque::from(vec![MegaByte::new(process.memory())]),
			written: VecDeque::from(vec![KiloByte::new(disk_usage.written_bytes)]),
			read: VecDeque::from(vec![KiloByte::new(disk_usage.read_bytes)]),
		}
	}

	/// Refresh the tracked process.
	pub fn refresh(&mut self, process: &Proc, elapsed: Duration) {
		self.time = process.run_time();
		self.status = process.status();

		let disk_usage = process.disk_usage();
		self.total_written = MegaByte::new(disk_usage.total_written_bytes);
		self.total_read = MegaByte::new(disk_usage.total_read_bytes);

		if self.memory.len() == TRACKED_PROCESS_DATAPOINTS {
			self.memory.pop_front();
		}
		self.memory.push_back(MegaByte::new(process.memory()));

		if self.cpu_usage.len() == TRACKED_PROCESS_DATAPOINTS {
			self.cpu_usage.pop_front();
		}
		self.cpu_usage.push_back(process.cpu_usage());

		if self.written.len() == TRACKED_PROCESS_DATAPOINTS {
			self.written.pop_front();
		}
		self.written
			.push_back(per_second(MegaByte::new(disk_usage.written_bytes), elapsed).as_kilo_byte());

		if self.read.len() == TRACKED_PROCESS_DATAPOINTS {
			self.read.pop_front();
		}
		self.read
			.push_back(per_second(MegaByte::new(disk_usage.written_bytes), elapsed).as_kilo_byte());
	}
}

/// A generic process.
pub struct Process {
	pub pid: u32,
	pub name: String,
	pub time: u64,
	pub memory: MegaByte,
	pub status: ProcessStatus,
	pub total_written: MegaByte,
	pub total_read: MegaByte,
	pub cpu_usage: f32,
}

impl Process {
	/// Create the process from a pid and process.
	pub fn from_pp(pid: &Pid, process: &Proc) -> Self {
		let pid = pid.as_u32();
		let name = process.name().to_owned();
		let memory = MegaByte::new(process.memory());
		let time = process.run_time();
		let status = process.status();
		let cpu_usage = process.cpu_usage();
		let disk_usage = process.disk_usage();
		let total_read = MegaByte::new(disk_usage.total_read_bytes);
		let total_written = MegaByte::new(disk_usage.total_written_bytes);

		Process {
			pid,
			name,
			memory,
			time,
			status,
			cpu_usage,
			total_read,
			total_written,
		}
	}
}
