use sysinfo::{System, Disks, Networks};
use std::io;
use crate::models::{DiskInfo, SystemInfo, ProcessInfo, NetworkInfo};

pub fn get_system_info() -> SystemInfo {
    SystemInfo { 
        os_name: System::name().unwrap_or_else(|| "Unknown".to_string()), 
        host_name: System::host_name().unwrap_or_else(|| "Unknown".to_string()) 
    }
}

pub fn get_process_count() -> io::Result<usize> {
    let mut sys = System::new_all();
    sys.refresh_processes();
    Ok(sys.processes().len())
}

pub fn get_top_processes(n: usize) -> io::Result<Vec<ProcessInfo>> {
    let mut sys = System::new_all();
    sys.refresh_processes();
    sys.refresh_cpu(); // Need for CPU usage
    std::thread::sleep(std::time::Duration::from_millis(100));
    sys.refresh_processes(); // Refresh again to get CPU diff

    let mut processes: Vec<_> = sys.processes().values().collect();
    // Sort by CPU usage descending
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));

    let top_procs = processes.into_iter().take(n).map(|p| {
        ProcessInfo {
            pid: p.pid().as_u32(),
            name: p.name().to_string(),
            cpu_usage: p.cpu_usage(),
            memory_mb: p.memory() as f64 / 1_048_576.0,
        }
    }).collect();

    Ok(top_procs)
}

pub fn get_network_info() -> io::Result<Vec<NetworkInfo>> {
    let networks = Networks::new_with_refreshed_list();
    let mut net_list = Vec::new();
    
    for (name, data) in &networks {
        net_list.push(NetworkInfo {
            name: name.clone(),
            tx_bytes: data.transmitted(),
            rx_bytes: data.received(),
        });
    }
    Ok(net_list)
}

pub fn get_cpu_load() -> io::Result<u64> {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu();
    
    let load = sys.global_cpu_info().cpu_usage();
    Ok(load as u64)
}

pub fn get_memory_usage() -> io::Result<(f64, f64, u64)> {
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    let total_bytes = sys.total_memory() as f64;
    let used_bytes = sys.used_memory() as f64;
    
    if total_bytes == 0.0 { return Ok((0.0, 0.0, 0)); }
    
    let percent = (used_bytes / total_bytes * 100.0) as u64;
    // Fix: sysinfo returns bytes, so we divide by 1024^3 for GB
    Ok((used_bytes / 1_073_741_824.0, total_bytes / 1_073_741_824.0, percent))
}

pub fn get_uptime() -> io::Result<u64> {
    Ok(System::uptime())
}

pub fn get_disks() -> io::Result<Vec<DiskInfo>> {
    let disks = Disks::new_with_refreshed_list();
    
    let mut disk_list = Vec::new();
    for disk in &disks {
        let total = disk.total_space() as f64;
        let free = disk.available_space() as f64;
        
        if total > 0.0 {
            let percent = ((total - free) / total * 100.0) as u64;
            disk_list.push(DiskInfo {
                name: disk.mount_point().to_string_lossy().to_string(),
                total_gb: total / 1_073_741_824.0,
                free_gb: free / 1_073_741_824.0,
                percent,
            });
        }
    }
    Ok(disk_list)
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
    fn test_memory_usage() {
        let res = get_memory_usage();
        assert!(res.is_ok());
        let (used, total, _) = res.unwrap();
        assert!(total >= 0.0);
        assert!(used <= total);
    }
}
