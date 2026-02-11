use sysinfo::System;
use crate::models::{SystemInfo, ProcessInfo};

pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
        host_name: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
    }
}

pub fn get_uptime() -> u64 {
    System::uptime()
}

pub struct SysCollector {
    sys: System,
}

impl SysCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_all();
        Self { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(200));
        self.sys.refresh_all();
    }

    pub fn cpu_load(&self) -> u64 {
        self.sys.global_cpu_info().cpu_usage() as u64
    }

    pub fn memory_usage(&self) -> (f64, f64, u64) {
        let total = self.sys.total_memory() as f64;
        let used = self.sys.used_memory() as f64;
        if total == 0.0 {
            return (0.0, 0.0, 0);
        }
        let percent = (used / total * 100.0) as u64;
        (used / 1_073_741_824.0, total / 1_073_741_824.0, percent)
    }

    pub fn process_count(&self) -> usize {
        self.sys.processes().len()
    }

    pub fn top_processes(&self, n: usize) -> Vec<ProcessInfo> {
        let mut processes: Vec<_> = self.sys.processes().values().collect();
        processes.sort_by(|a, b| {
            b.cpu_usage()
                .partial_cmp(&a.cpu_usage())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        processes
            .into_iter()
            .take(n)
            .map(|p| ProcessInfo {
                pid: p.pid().as_u32(),
                name: p.name().to_string(),
                cpu_usage: p.cpu_usage(),
                memory_mb: p.memory() as f64 / 1_048_576.0,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info() {
        let info = get_system_info();
        assert!(!info.os_name.is_empty());
    }

    #[test]
    fn test_collector() {
        let collector = SysCollector::new();
        let (used, total, _) = collector.memory_usage();
        assert!(total >= 0.0);
        assert!(used <= total);
        assert!(collector.process_count() > 0);
    }
}
