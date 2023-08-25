use libc::sysconf;

use libc::_SC_NPROCESSORS_CONF;

/// Gets the amount of cpu cores.
pub fn cpu_cores() -> usize {
    let nprocs = unsafe { sysconf(_SC_NPROCESSORS_CONF) };
    if nprocs > 0 {
        nprocs as usize
    } else {
        1
    }
}

/// Gets the name of the cpu.
pub fn get_cpu_name() -> String {
    todo!("Parse the /proc/cpuinfo and sysfs")
}
