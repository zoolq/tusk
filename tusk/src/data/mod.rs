mod util;

use std::{collections::VecDeque, time::Instant};

use memu::units::MegaByte;
use sysinfo::{CpuExt, System, SystemExt};

use crate::{
	datapoints::{CPU_USAGE_DATAPOINTS, NETWORK_DATAPOINTS},
	terminal::{Data, Process},
};

use self::util::*;

pub struct DataStorage {
	pub last_snapshot: Instant,
	pub network_in_last_sec: MegaByte,
	pub network_out_last_sec: MegaByte,
}

impl DataStorage {
	pub fn new() -> Self {
		DataStorage {
			last_snapshot: Instant::now(),
			network_in_last_sec: MegaByte::default(),
			network_out_last_sec: MegaByte::default(),
		}
	}

	fn update_network(&mut self, sys: &mut System) {
		let networks = sys.networks();
		self.network_in_last_sec = compute_in(networks);
		self.network_out_last_sec = compute_out(networks);
	}

	fn update_time(&mut self) {
		self.last_snapshot = Instant::now();
	}
}

pub fn new_data(sys: &mut System) -> Data {
	sys.refresh_all();

	let cpu_name = sys.cpus().first().unwrap().brand().to_owned();

	let cpu_usage = VecDeque::new();

	let cpu_frequency = 0;

	let network_in = VecDeque::new();

	let network_out = VecDeque::new();

	let processes = Vec::new();

	let tracked_pid = None;

	let tracked = None;

	Data {
		cpu_name,
		cpu_frequency,
		cpu_usage,
		network_in,
		network_out,
		processes,
		tracked_pid,
		tracked,
	}
}

pub fn fetch_data(sys: &mut System, prior: &mut Data, storage: &mut DataStorage) {
	let elapsed = storage.last_snapshot.elapsed();
	sys.refresh_cpu();
	sys.refresh_networks();
	sys.refresh_processes();

	{
		let cpus = sys.cpus();

		prior.cpu_frequency = compute_frequency(cpus);

		if prior.cpu_usage.len() == CPU_USAGE_DATAPOINTS {
			prior.cpu_usage.pop_front();
		}

		prior.cpu_usage.push_back(compute_usage(cpus));
	}

	{
		storage.update_network(sys);

		if prior.network_in.len() == NETWORK_DATAPOINTS {
			prior.network_in.pop_front();
		}

		prior
			.network_in
			.push_back(per_second(storage.network_in_last_sec, elapsed));

		if prior.network_out.len() == NETWORK_DATAPOINTS {
			prior.network_out.pop_front();
		}

		prior
			.network_out
			.push_back(per_second(storage.network_out_last_sec, elapsed));

		storage.update_time();
	}

	{
		let processes = sys.processes();

		prior.processes = Vec::with_capacity(processes.len());

		for (pid, process) in processes {
			prior.processes.push(Process::from_pp(pid, process))
		}

		prior.processes.sort_by(|a, b| a.memory.cmp(&b.memory));
		prior.processes.reverse();

		if let Some(pid) = prior.tracked_pid {
			if let Some(tracked_process) = &mut prior.tracked {
				if let Some(process) = processes.get(&pid) {
					update_tracked(process, tracked_process, elapsed)
				} else {
					prior.tracked_pid = None;
				}
			} else {
				if let Some(process) = processes.get(&pid) {
					prior.tracked = Some(new_tracked(&pid, &process));
					if let Some(tracked_process) = &mut prior.tracked {
						update_tracked(process, tracked_process, elapsed);
					}
				} else {
					prior.tracked_pid = None;
				}
			}
		}
	}
}
