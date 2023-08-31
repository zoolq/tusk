mod util;

use std::{collections::VecDeque, time::Instant};

use memu::units::KiloByte;
use sysinfo::{CpuExt, System, SystemExt};

use crate::{
	datapoints::{CPU_USAGE_DATAPOINTS, NETWORK_DATAPOINTS},
	terminal::DrawingData,
};

use self::util::*;

pub struct DataStorage {
	pub last_snapshot: Instant,
	pub network_in_last_sec: KiloByte,
	pub network_out_last_sec: KiloByte,
}

impl DataStorage {
	pub fn new() -> Self {
		DataStorage {
			last_snapshot: Instant::now(),
			network_in_last_sec: KiloByte::default(),
			network_out_last_sec: KiloByte::default(),
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

pub fn new_data(sys: &mut System) -> DrawingData {
	sys.refresh_all();

	let cpu_name = sys.cpus().first().unwrap().brand().to_owned();

	let cpu_usage = VecDeque::from([0.0; CPU_USAGE_DATAPOINTS]);

	let cpu_frequency = 0;

	let network_in = VecDeque::from([KiloByte::new(0); NETWORK_DATAPOINTS]);

	let network_out = VecDeque::from([KiloByte::new(0); NETWORK_DATAPOINTS]);

	DrawingData {
		cpu_name,
		cpu_frequency,
		cpu_usage,
		network_in,
		network_out,
	}
}

pub fn fetch_data(sys: &mut System, prior: &mut DrawingData, storage: &mut DataStorage) {
	sys.refresh_cpu();

	let cpus = sys.cpus();

	prior.cpu_frequency = compute_frequency(cpus);

	prior.cpu_usage.pop_front();
	prior.cpu_usage.push_back(compute_usage(cpus));

	sys.refresh_networks();
	storage.update_network(sys);
	let time = storage.last_snapshot.elapsed();

	prior.network_in.pop_front();
	prior
		.network_in
		.push_back(per_second(storage.network_in_last_sec, time));

	prior.network_out.pop_front();
	prior
		.network_out
		.push_back(per_second(storage.network_out_last_sec, time));

	storage.update_time();
}
