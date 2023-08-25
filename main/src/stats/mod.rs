#![allow(unused)]

use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

pub mod cpu_info;

pub struct InfoSettings {
    system: System,
    pub tracked_process: Option<Pid>,
}

impl InfoSettings {
    // Make this return a reference so we can lanuch a thread that has ownership and update the info.
    // The main thread can then read the data but not update it.
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        InfoSettings {
            system,
            tracked_process: None,
        }
    }

    pub fn tracked_process(&mut self, pid: usize) {
        let pid = Pid::from(2);
        self.tracked_process = Some(pid.to_owned());
    }

    pub fn display_process(&self) -> Vec<&Process> {
        if let Some(pid) = self.tracked_process {
            vec![self.system.process(pid).unwrap()]
        } else {
            let mut vec = Vec::new();
            for process in self.system.processes().values() {
                vec.push(process);
            }
            vec.sort_by_key(|p| !p.memory());
            vec
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_all();
    }
}
