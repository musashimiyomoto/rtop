use std::process::Command;
use std::io;
use crate::models::{DiskInfo, SystemInfo};

pub fn get_system_info() -> SystemInfo {
    let mut info = SystemInfo { 
        os_name: "Unknown".to_string(), 
        host_name: "Unknown".to_string() 
    };
    
    if let Ok(s) = run_wmic(&["os", "get", "Caption,CSName", "/Value"]) {
        for line in s.lines() {
            let line = line.trim();
            if line.starts_with("Caption=") {
                info.os_name = line.split('=').nth(1).unwrap_or("").to_string();
            } else if line.starts_with("CSName=") {
                info.host_name = line.split('=').nth(1).unwrap_or("").to_string();
            }
        }
    }
    info
}

pub fn get_process_count() -> io::Result<usize> {
    let output = Command::new("tasklist").output()?;
    let s = String::from_utf8_lossy(&output.stdout);
    let count = s.lines().count();
    Ok(if count > 3 { count - 3 } else { 0 })
}

pub fn get_cpu_load() -> io::Result<u64> {
    let s = run_wmic(&["cpu", "get", "loadpercentage", "/Value"])?;
    Ok(parse_cpu_load(&s))
}

pub fn get_memory_usage() -> io::Result<(f64, f64, u64)> {
    let s = run_wmic(&["OS", "get", "FreePhysicalMemory,TotalVisibleMemorySize", "/Value"])?;
    Ok(parse_memory_usage(&s))
}

pub fn get_uptime() -> io::Result<u64> {
    let s = run_wmic(&["path", "Win32_PerfFormattedData_PerfOS_System", "get", "SystemUpTime", "/Value"])?;
    for line in s.lines() {
        if line.trim().starts_with("SystemUpTime=") {
            if let Some(val) = line.trim().split('=').nth(1) {
                return Ok(val.parse().unwrap_or(0));
            }
        }
    }
    Ok(0)
}

pub fn get_disks() -> io::Result<Vec<DiskInfo>> {
    let output = Command::new("wmic")
        .args(&["logicaldisk", "get", "DeviceID,FreeSpace,Size", "/Format:CSV"])
        .output()?;
        
    let s = String::from_utf8_lossy(&output.stdout).replace("\0", "");
    Ok(parse_disks(&s))
}

fn run_wmic(args: &[&str]) -> io::Result<String> {
    let output = Command::new("wmic")
        .args(args)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).replace("\0", ""))
}

// --- Parsing Functions (Private / Internal) ---

fn parse_cpu_load(output: &str) -> u64 {
    for line in output.lines() {
        if line.trim().starts_with("LoadPercentage=") {
            if let Some(val) = line.trim().split('=').nth(1) {
                return val.parse().unwrap_or(0);
            }
        }
    }
    0
}

fn parse_memory_usage(output: &str) -> (f64, f64, u64) {
    let mut total_kb = 0.0;
    let mut free_kb = 0.0;

    for line in output.lines() {
        let line = line.trim();
        if line.starts_with("TotalVisibleMemorySize=") {
             total_kb = line.split('=').nth(1).unwrap_or("0").parse().unwrap_or(0.0);
        } else if line.starts_with("FreePhysicalMemory=") {
             free_kb = line.split('=').nth(1).unwrap_or("0").parse().unwrap_or(0.0);
        }
    }

    if total_kb == 0.0 { return (0.0, 0.0, 0); }
    
    let used_kb = total_kb - free_kb;
    let percent = (used_kb / total_kb * 100.0) as u64;
    (used_kb / 1048576.0, total_kb / 1048576.0, percent)
}

fn parse_disks(output: &str) -> Vec<DiskInfo> {
    let mut disks = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() { continue; }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 4 && parts[1] != "DeviceID" {
            let name = parts[1].trim().to_string();
            let free_str = parts[2].trim();
            let size_str = parts[3].trim();
            
            if size_str.is_empty() || free_str.is_empty() { continue; }

            let free: f64 = free_str.parse().unwrap_or(0.0);
            let size: f64 = size_str.parse().unwrap_or(0.0);

            if size > 0.0 {
                let percent = ((size - free) / size * 100.0) as u64;
                disks.push(DiskInfo {
                    name,
                    total_gb: size / 1_073_741_824.0,
                    free_gb: free / 1_073_741_824.0,
                    percent,
                });
            }
        }
    }
    disks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cpu_load() {
        let input = "\r\nLoadPercentage=45\r\n\r\n";
        assert_eq!(parse_cpu_load(input), 45);
        
        let input_empty = "";
        assert_eq!(parse_cpu_load(input_empty), 0);
    }

    #[test]
    fn test_parse_memory_usage() {
        // Total: 16GB (16777216 KB), Free: 8GB (8388608 KB) => 50% usage
        let input = "FreePhysicalMemory=8388608\r\nTotalVisibleMemorySize=16777216\r\n";
        let (used, total, percent) = parse_memory_usage(input);
        
        assert_eq!(percent, 50);
        assert!((total - 16.0).abs() < 0.1); 
        assert!((used - 8.0).abs() < 0.1);
    }

    #[test]
    fn test_parse_disks() {
        let input = "\r\nNode,DeviceID,FreeSpace,Size\r\nMYPC,C:,53687091200,107374182400\r\n";
        let disks = parse_disks(input);
        
        assert_eq!(disks.len(), 1);
        assert_eq!(disks[0].name, "C:");
        assert_eq!(disks[0].percent, 50);
        assert!((disks[0].total_gb - 100.0).abs() < 0.1);
    }
}
