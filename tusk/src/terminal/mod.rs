pub(super) mod draw;
pub(super) mod events;
mod modules;
mod tabs;

use std::collections::VecDeque;

use memu::units::MegaByte;
use sysinfo::{Pid, PidExt, Process as Proc, ProcessExt};

pub struct DrawingData {
	/// Cpu name, static
	pub cpu_name: String,
	/// Cpu frequency, updated
	pub cpu_frequency: u64,
	/// Serial cpu usage in percent
	pub cpu_usage: VecDeque<f32>,
	/// Serial network out
	pub network_out: VecDeque<MegaByte>,
	/// Serial network in
	pub network_in: VecDeque<MegaByte>,
	/// List of processes
	pub processes: Vec<Process>,
}

pub struct Process {
	pub pid: u32,
	pub name: String,
	pub memory: MegaByte,
}

impl Process {
	pub fn from_pp(pid: &Pid, process: &Proc) -> Self {
		let pid = pid.as_u32();
		let name = process.name().to_owned();
		let memory = MegaByte::new(process.memory());

		Process { pid, name, memory }
	}
}
