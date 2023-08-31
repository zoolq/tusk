use std::time::Duration;

use memu::units::KiloByte;
use sysinfo::{Cpu, CpuExt, NetworkExt, Networks, NetworksExt};

pub fn compute_in(networks: &Networks) -> KiloByte {
	KiloByte::new(networks.iter().map(|(_, n)| n.received()).sum())
}

pub fn compute_out(networks: &Networks) -> KiloByte {
	KiloByte::new(networks.iter().map(|(_, n)| n.transmitted()).sum())
}

pub fn per_second(data: KiloByte, elapsed: Duration) -> KiloByte {
	KiloByte::from(data.as_f64() / elapsed.as_secs_f64())
}

pub fn compute_frequency(vec: &[Cpu]) -> u64 {
	vec.iter().map(|core| core.frequency()).sum::<u64>() / vec.len() as u64
}

pub fn compute_usage(vec: &[Cpu]) -> f32 {
	vec.iter().map(|core| core.cpu_usage()).sum::<f32>() / vec.len() as f32
}
