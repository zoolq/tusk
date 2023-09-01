pub(super) mod draw;
pub(super) mod events;
mod modules;
mod tabs;

use std::collections::VecDeque;

use memu::units::{KiloByte, MegaByte};
use ratatui::widgets::ScrollbarState;
use sysinfo::{Pid, PidExt, Process as Proc, ProcessExt, ProcessStatus};

use crate::TABS;

#[derive(Default, Clone, Copy)]
pub enum Screen {
	/// The default screen, a bit of everything.
	#[default]
	Default,
	/// A special process screen with process information.
	Processes,
}

impl Screen {
	fn as_string(&self) -> &str {
		match *self {
			Self::Default => "Default",
			Self::Processes => "Process",
		}
	}
}

pub struct State {
	pub vertical_scroll_state: ScrollbarState,
	pub tabs_index: usize,
	pub tabs: Vec<Screen>,
}

impl State {
	pub fn new() -> Self {
		State {
			vertical_scroll_state: ScrollbarState::default(),
			tabs_index: 0,
			tabs: Vec::from(TABS),
		}
	}

	pub fn curret_tab(&self) -> Screen {
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
}

pub struct Data {
	/// Cpu name, static.
	pub cpu_name: String,
	/// Cpu frequency, updated.
	pub cpu_frequency: u64,
	/// Serial cpu usage in percent.
	pub cpu_usage: VecDeque<f32>,
	/// Serial network out.
	pub network_out: VecDeque<MegaByte>,
	/// Serial network in.
	pub network_in: VecDeque<MegaByte>,
	/// List of processes.
	pub processes: Vec<Process>,
	/// If a process is being tracked this is Some(Pid).
	pub tracked_pid: Option<Pid>,
	/// The tracked process info.
	pub tracked: Option<TrackedProcess>,
}

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
