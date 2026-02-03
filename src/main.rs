use std::process::Command;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

struct SystemInfo {
    os_name: String,
    host_name: String,
}

struct DiskInfo {
    name: String,
    total_gb: f64,
    free_gb: f64,
    percent: u64,
}

fn main() -> io::Result<()> {
    print!("\x1B[2J");
    print!("\x1B[?25l");
    let mut stdout = io::stdout();

    let sys_info = get_system_info();

    loop {
        let cpu_res = get_cpu_load();
        let mem_res = get_memory_usage();
        let disks_res = get_disks();
        let proc_count = get_process_count();

        let mut buffer = String::new();
        
        buffer.push_str("\x1B[H");
        
        let c_reset = "\x1B[0m";
        let c_cyan = "\x1B[36m";
        let c_green = "\x1B[32m";
        let c_yellow = "\x1B[33m";
        let c_magenta = "\x1B[35m";
        let c_blue = "\x1B[34m";
        let c_bold = "\x1B[1m";

        use std::fmt::Write;

        writeln!(buffer, "{}{}=== RUST TOP (MULTI-DISK) ==={}", c_bold, c_cyan, c_reset).unwrap();
        writeln!(buffer, "Host: {}{}{} | OS: {}{}{}", 
            c_green, sys_info.host_name, c_reset,
            c_green, sys_info.os_name, c_reset
        ).unwrap();
        writeln!(buffer, "Processes: {}{}{}", c_yellow, proc_count.unwrap_or(0), c_reset).unwrap();
        writeln!(buffer, "{}", "-".repeat(65)).unwrap();

        match cpu_res {
            Ok(load) => {
                push_bar(&mut buffer, "CPU", load as usize, c_yellow);
            }
            Err(_) => writeln!(buffer, "CPU     : Error").unwrap(),
        }

        match mem_res {
            Ok((used, total, percent)) => {
                push_bar(&mut buffer, "MEM", percent as usize, c_magenta);
                writeln!(buffer, "          {:.2} GB / {:.2} GB", used, total).unwrap();
            }
            Err(_) => writeln!(buffer, "MEM     : Error").unwrap(),
        }

        writeln!(buffer, "{}", "-".repeat(65)).unwrap();

        match disks_res {
            Ok(disks) => {
                if disks.is_empty() {
                    writeln!(buffer, "No disks found").unwrap();
                } else {
                    for disk in disks {
                        let label = format!("DSK {}", disk.name);
                        push_bar(&mut buffer, &label, disk.percent as usize, c_blue);
                        writeln!(buffer, "          {:.2} GB / {:.2} GB", disk.total_gb - disk.free_gb, disk.total_gb).unwrap();
                    }
                }
            }
            Err(_) => writeln!(buffer, "DISK    : Error").unwrap(),
        }

        writeln!(buffer, "{}", "-".repeat(65)).unwrap();
        writeln!(buffer, "Press Ctrl+C to exit.").unwrap();

        print!("{}", buffer);
        stdout.flush()?;

        thread::sleep(Duration::from_millis(2000));
    }
}

fn push_bar(buffer: &mut String, label: &str, percent: usize, color_code: &str) {
    let width = 40;
    let percent = if percent > 100 { 100 } else { percent };
    
    let filled = (percent * width) / 100;
    let empty = width - filled;

    let filled_str = "#".repeat(filled);
    let empty_str = ".".repeat(empty);

    use std::fmt::Write;
    writeln!(buffer, "{:<7}: [ {}{}{}{} ] {:>3}%", 
        label, 
        color_code, 
        filled_str, 
        "\x1B[0m", 
        empty_str, 
        percent
    ).unwrap();
}

fn run_wmic(args: &[&str]) -> io::Result<String> {
    let output = Command::new("wmic")
        .args(args)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).replace("\0", ""))
}

fn get_disks() -> io::Result<Vec<DiskInfo>> {
    let output = Command::new("wmic")
        .args(&["logicaldisk", "get", "DeviceID,FreeSpace,Size", "/Format:CSV"])
        .output()?;
        
    let s = String::from_utf8_lossy(&output.stdout).replace("\0", "");
    let mut disks = Vec::new();
    
    for line in s.lines() {
        if line.trim().is_empty() { continue; }
        
        let parts: Vec<&str> = line.split(',').collect();
        
        if parts.len() >= 4 && parts[1] != "DeviceID" {
            let name = parts[1].trim().to_string();
            let free_str = parts[2].trim();
            let size_str = parts[3].trim();

            if size_str.is_empty() || free_str.is_empty() {
                continue;
            }

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
    
    Ok(disks)
}

fn get_cpu_load() -> io::Result<u64> {
    let s = run_wmic(&["cpu", "get", "loadpercentage", "/Value"])?;
    for line in s.lines() {
        if line.trim().starts_with("LoadPercentage=") {
            if let Some(val) = line.trim().split('=').nth(1) {
                return Ok(val.parse().unwrap_or(0));
            }
        }
    }
    Ok(0)
}

fn get_memory_usage() -> io::Result<(f64, f64, u64)> {
    let s = run_wmic(&["OS", "get", "FreePhysicalMemory,TotalVisibleMemorySize", "/Value"])?;
    
    let mut total_kb = 0.0;
    let mut free_kb = 0.0;

    for line in s.lines() {
        let line = line.trim();
        if line.starts_with("TotalVisibleMemorySize=") {
             total_kb = line.split('=').nth(1).unwrap_or("0").parse().unwrap_or(0.0);
        } else if line.starts_with("FreePhysicalMemory=") {
             free_kb = line.split('=').nth(1).unwrap_or("0").parse().unwrap_or(0.0);
        }
    }

    if total_kb == 0.0 { return Ok((0.0, 0.0, 0)); }
    
    let used_kb = total_kb - free_kb;
    let percent = (used_kb / total_kb * 100.0) as u64;
    Ok((used_kb / 1048576.0, total_kb / 1048576.0, percent))
}

fn get_process_count() -> io::Result<usize> {
    let output = Command::new("tasklist").output()?;
    let s = String::from_utf8_lossy(&output.stdout);
    let count = s.lines().count();
    Ok(if count > 3 { count - 3 } else { 0 })
}

fn get_system_info() -> SystemInfo {
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
