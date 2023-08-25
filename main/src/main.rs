mod stats;

use libc::KEXEC_FILE_NO_INITRAMFS;
use memu::{kilo_bytes, units::KiloByte, MemoryUnit};
use stats::InfoSettings;
use sysinfo::ProcessExt;

#[allow(unused_imports)]
use self::stats::cpu_info::{cpu_cores, get_cpu_name};

fn main() {
    println!("{}", 1.23);
    let stats = InfoSettings::new();
    for proc in stats.display_process() {
        let memory = KiloByte::new(proc.memory());
        println!("{}", memory.as_f64());
        println!("{}", memory.as_giga_byte().as_f64());
        memory.as_f64();
    }
}
