// mod stats;

use std::time::Instant;

#[allow(unused_imports)]
use libc::KEXEC_FILE_NO_INITRAMFS;
use memu::{units::KiloByte, MemoryUnit};
use sysinfo::{Pid, ProcessExt, System, SystemExt};

fn main() {
	let mut sys = System::new_all();

	let mut tracked_pids: Vec<Pid> = vec![];

	for i in 0..100 {
		let st = Instant::now();
		sys.refresh_cpu();
		sys.refresh_processes();
		// Draw stuff
		println!("{:?}", st.elapsed());
	}
}
