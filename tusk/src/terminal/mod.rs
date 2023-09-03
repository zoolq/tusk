mod app_util;
pub(super) mod draw;
pub(super) mod events;
mod modules;
mod tabs;

use std::{
	collections::VecDeque,
	str::FromStr,
	sync::{Arc, Mutex},
	time::{Duration, Instant},
};

use crossterm::event::KeyCode::{self, Left, Right};
use log::{error, Level, Log, Metadata, Record};
use memu::{
	units::{KiloByte, MegaByte},
	MemoryUnit,
};
use ratatui::widgets::ScrollbarState;
use sysinfo::{CpuExt, Pid, PidExt, Process as Proc, ProcessExt, ProcessStatus, System, SystemExt};

use crate::{
	datapoints::{
		CPU_USAGE_DATAPOINTS, LOG_MESSAGES, NETWORK_DATAPOINTS, TRACKED_PROCESS_DATAPOINTS,
	},
	TABS,
};

use self::app_util::{compute_frequency, compute_in, compute_out, compute_usage, per_second};

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
	fn as_string(&self) -> &str {
		match *self {
			Self::Default => "Default",
			Self::Processes => "Process",
			Self::Tracked => "Tracked",
			Self::Debug => "Debug",
		}
	}
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum TopBar {
	/// Display all tabs in the top bar.
	#[default]
	Tabs,
	/// Display input in the top bar.
	Input,
}
pub struct App {
	sys: System,
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
	network_in_last_sec: MegaByte,
	network_out_last_sec: MegaByte,
}

impl App {
	pub fn new() -> Self {
		let mut sys = System::new_all();

		sys.refresh_all();

		let mut app = App {
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
			network_in_last_sec: MegaByte::default(),
			network_out_last_sec: MegaByte::default(),
			sys,
		};

		app.refresh();

		app
	}

	pub fn refresh(&mut self) {
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
		self.network_in_last_sec = compute_in(networks);
		if self.network_in.len() == NETWORK_DATAPOINTS {
			self.network_in.pop_front();
		}
		self.network_in
			.push_back(per_second(self.network_in_last_sec, elapsed));

		self.network_out_last_sec = compute_out(networks);
		if self.network_out.len() == NETWORK_DATAPOINTS {
			self.network_out.pop_front();
		}

		self.network_out
			.push_back(per_second(self.network_out_last_sec, elapsed));

		let processes = self.sys.processes();
		self.processes = Vec::with_capacity(processes.len());
		for (pid, process) in processes {
			self.processes.push(Process::from_pp(pid, process))
		}

		self.processes.sort_by(|a, b| b.memory.cmp(&a.memory));

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
	}

	pub fn current_tab(&self) -> Screen {
		let tabs_index = self.tabs_index % self.tabs.len();
		self.tabs[tabs_index]
	}

	pub fn inc_tabs_index(&mut self) {
		self.tabs_index += 1;
		self.tabs_index %= self.tabs.len();
	}

	pub fn dec_tabs_index(&mut self) {
		if self.tabs_index != 0 {
			self.tabs_index -= 1;
		} else {
			self.tabs_index = self.tabs.len() - 1;
		}
		self.tabs_index %= self.tabs.len();
	}

	pub fn tabs_index(&self) -> usize {
		self.tabs_index % self.tabs.len()
	}

	pub fn input_type(&mut self, ch: char) {
		self.input.insert(self.input_position, ch);
		self.input_position += 1;
	}

	pub fn input_backspace(&mut self) {
		if self.input_position > 0 {
			self.input_position -= 1;

			let before_char_to_delete = self.input.chars().take(self.input_position);

			let after_char_to_delete = self.input.chars().skip(self.input_position + 1);

			self.input = before_char_to_delete.chain(after_char_to_delete).collect();
		}
	}

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
			_ => unreachable!("This function is only called when `Left` or `Right` occurs"),
		}
	}

	pub fn wipe_input(&mut self) {
		self.top_bar = TopBar::Tabs;
		self.input.clear();
		self.input_position = 0;
	}

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

	pub fn enable_debug(&mut self) {
		if !self.tabs.contains(&Screen::Debug) {
			self.tabs.push(Screen::Debug)
		}
	}

	pub fn disable_debug(&mut self) {
		self.tabs = self
			.tabs
			.iter()
			.filter(|&&a| a != Screen::Debug)
			.map(|a| a.to_owned())
			.collect();
	}

	pub fn switch_debug(&mut self) {
		if self.tabs.contains(&Screen::Debug) {
			self.disable_debug()
		} else {
			self.enable_debug()
		}
	}
}

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

pub struct Logger {
	log: Arc<Mutex<VecDeque<String>>>,
}

impl Logger {
	pub fn new() -> Self {
		Logger {
			log: Arc::new(Mutex::new(VecDeque::new())),
		}
	}

	pub fn logs(&self) -> Arc<Mutex<VecDeque<String>>> {
		Arc::clone(&self.log)
	}
}

impl Log for Logger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= Level::Info
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			let mut logs = self.log.lock().unwrap();

			if logs.len() >= LOG_MESSAGES {
				logs.pop_front();
			}

			logs.push_back(format!("{} - {}", record.level(), record.args()));
		}
	}

	fn flush(&self) {}
}
