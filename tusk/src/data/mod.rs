use std::collections::VecDeque;

use memu::units::KiloByte;
use sysinfo::{Cpu, CpuExt, NetworkExt, Networks, NetworksExt, System, SystemExt};

use crate::{
	datapoints::{CPU_FREQUENCY_DATAPOINTS, CPU_USAGE_DATAPOINTS, NETWORK_DATAPOINTS},
	terminal::DrawingData,
};

pub fn new_data(sys: &mut System) -> DrawingData {
	sys.refresh_all();

	let cpu_name = sys.cpus().first().unwrap().brand().to_owned();

	let cpu_usage = VecDeque::from([0.0; CPU_USAGE_DATAPOINTS]);

	let cpu_frequency = VecDeque::from([0; CPU_FREQUENCY_DATAPOINTS]);

	let network_in = VecDeque::from([KiloByte::new(0); NETWORK_DATAPOINTS]);

	let network_out = VecDeque::from([KiloByte::new(0); NETWORK_DATAPOINTS]);

	DrawingData {
		cpu_name,
		cpu_usage,
		cpu_frequency,
		network_in,
		network_out,
	}
}

pub fn fetch_data(sys: &mut System, prior: &mut DrawingData) {
	sys.refresh_cpu();

	let cpus = sys.cpus();

	prior.cpu_frequency.pop_front();
	prior.cpu_frequency.push_back(compute_frequency(cpus));

	prior.cpu_usage.pop_front();
	prior.cpu_usage.push_back(compute_usage(cpus));

	sys.refresh_networks();

	let networks = sys.networks();

	prior.network_in.pop_front();
	prior.network_in.push_back(compute_in(networks));

	prior.network_out.pop_front();
	prior.network_out.push_back(compute_out(networks));
}

fn compute_in(networks: &Networks) -> KiloByte {
	KiloByte::new(networks.iter().map(|(_, n)| n.received()).sum())
}

fn compute_out(networks: &Networks) -> KiloByte {
	KiloByte::new(networks.iter().map(|(_, n)| n.transmitted()).sum())
}

fn compute_frequency(vec: &[Cpu]) -> u64 {
	vec.iter().map(|core| core.frequency()).sum::<u64>() / vec.len() as u64
}

fn compute_usage(vec: &[Cpu]) -> f32 {
	vec.iter().map(|core| core.cpu_usage()).sum::<f32>() / vec.len() as f32
}
