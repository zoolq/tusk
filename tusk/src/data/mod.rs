use std::collections::VecDeque;

use sysinfo::{Cpu, CpuExt, System, SystemExt};

use crate::{datapoints::CPU_USAGE_DATAPOINTS, terminal::DrawingData};

pub fn new_data(sys: &mut System) -> DrawingData {
	sys.refresh_all();

	let cpu_name = sys.cpus().first().unwrap().brand().to_owned();

	let cpu_usage = VecDeque::from([0.0; CPU_USAGE_DATAPOINTS]);

	DrawingData {
		cpu_name,
		cpu_usage,
	}
}

pub fn fetch_data(sys: &mut System, prior: &mut DrawingData) {
	sys.refresh_cpu();

	prior.cpu_usage.pop_front();
	prior.cpu_usage.push_back(compute_usage(sys.cpus()))
}

fn compute_usage(vec: &[Cpu]) -> f32 {
	vec.iter().map(|core| core.cpu_usage()).sum::<f32>() / vec.len() as f32
}
