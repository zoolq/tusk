pub mod draw;
pub mod events;
mod modules;
pub mod tabs;

use std::collections::VecDeque;

use memu::units::KiloByte;

#[derive(Debug)]
pub struct DrawingData {
	/// Cpu name, static
	pub cpu_name: String,
	/// Cpu frequency, updated
	pub cpu_frequency: u64,
	/// Serial cpu usage in percent
	pub cpu_usage: VecDeque<f32>,
	/// Serial network out
	pub network_out: VecDeque<KiloByte>,
	/// Serial network in
	pub network_in: VecDeque<KiloByte>,
}
