pub mod draw;
pub mod events;
mod modules;

use std::collections::VecDeque;

use memu::units::KiloByte;

#[derive(Debug)]
pub struct DrawingData {
	pub cpu_name: String,
	pub cpu_usage: VecDeque<f32>,
	pub cpu_frequency: VecDeque<u64>,
	pub network_out: VecDeque<KiloByte>,
	pub network_in: VecDeque<KiloByte>,
}
