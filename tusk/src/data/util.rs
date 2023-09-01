use std::{collections::VecDeque, time::Duration};

use memu::{
	units::{KiloByte, MegaByte},
	MemoryUnit,
};
use sysinfo::{Cpu, CpuExt, NetworkExt, Networks, NetworksExt, Pid, PidExt, Process, ProcessExt};

use crate::{datapoints::TRACKED_PROCESS_DATAPOINTS, terminal::TrackedProcess};

pub fn compute_in(networks: &Networks) -> MegaByte {
	MegaByte::new(networks.iter().map(|(_, n)| n.received()).sum())
}

pub fn compute_out(networks: &Networks) -> MegaByte {
	MegaByte::new(networks.iter().map(|(_, n)| n.transmitted()).sum())
}

pub fn per_second(data: MegaByte, elapsed: Duration) -> MegaByte {
	MegaByte::from(data.as_f64() / elapsed.as_secs_f64())
}

pub fn compute_frequency(vec: &[Cpu]) -> u64 {
	vec.iter().map(|core| core.frequency()).sum::<u64>() / vec.len() as u64
}

pub fn compute_usage(vec: &[Cpu]) -> f32 {
	vec.iter().map(|core| core.cpu_usage()).sum::<f32>() / vec.len() as f32
}

pub fn new_tracked(pid: &Pid, process: &Process) -> TrackedProcess {
	let pid = pid.as_u32();
	let name = process.name().to_owned();
	let time = process.run_time();
	let status = process.status();
	let disk_usage = process.disk_usage();
	let total_written = MegaByte::new(disk_usage.total_written_bytes);
	let total_read = MegaByte::new(disk_usage.total_read_bytes);
	let memory = VecDeque::from(vec![MegaByte::new(process.memory())]);
	let cpu_usage = VecDeque::from(vec![process.cpu_usage()]);
	let written = VecDeque::from(vec![KiloByte::new(disk_usage.written_bytes)]);
	let read = VecDeque::from(vec![KiloByte::new(disk_usage.read_bytes)]);

	TrackedProcess {
		pid,
		name,
		time,
		status,
		total_written,
		total_read,
		memory,
		cpu_usage,
		written,
		read,
	}
}

pub fn update_tracked(process: &Process, prior: &mut TrackedProcess, elapsed: Duration) {
	prior.time = process.run_time();
	prior.status = process.status();

	let disk_usage = process.disk_usage();
	prior.total_written = MegaByte::new(disk_usage.total_written_bytes);
	prior.total_read = MegaByte::new(disk_usage.total_read_bytes);

	if prior.memory.len() == TRACKED_PROCESS_DATAPOINTS {
		prior.memory.pop_front();
	}
	prior.memory.push_back(MegaByte::new(process.memory()));

	if prior.cpu_usage.len() == TRACKED_PROCESS_DATAPOINTS {
		prior.cpu_usage.pop_front();
	}
	prior.cpu_usage.push_back(process.cpu_usage());

	if prior.written.len() == TRACKED_PROCESS_DATAPOINTS {
		prior.written.pop_front();
	}
	prior
		.written
		.push_back(per_second(MegaByte::new(disk_usage.written_bytes), elapsed).as_kilo_byte());

	if prior.read.len() == TRACKED_PROCESS_DATAPOINTS {
		prior.read.pop_front();
	}
	prior
		.read
		.push_back(per_second(MegaByte::new(disk_usage.written_bytes), elapsed).as_kilo_byte());
}
