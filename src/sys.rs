use sysinfo::{System, Disks};
use std::io;
use crate::models::{DiskInfo, SystemInfo};

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

pub fn get_cpu_load() -> io::Result<u64> {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    // sysinfo needs some time between refreshes to calculate CPU usage
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu();
    
    let load = sys.global_cpu_info().cpu_usage();
    Ok(load as u64)
}

pub fn get_memory_usage() -> io::Result<(f64, f64, u64)> {
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    let total_kb = sys.total_memory() as f64;
    let used_kb = sys.used_memory() as f64;
    
    if total_kb == 0.0 { return Ok((0.0, 0.0, 0)); }
    
    let percent = (used_kb / total_kb * 100.0) as u64;
    Ok((used_kb / 1048576.0, total_kb / 1048576.0, percent)) // Converting to GB
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
