pub mod draw;
pub mod events;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct DrawingData {
	pub cpu_name: String,
	pub cpu_usage: VecDeque<f32>,
}
