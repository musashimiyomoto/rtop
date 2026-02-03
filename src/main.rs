mod models;
mod sys;
mod ui;

use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::fmt::Write as FmtWrite;

use sys::{get_system_info, get_cpu_load, get_memory_usage, get_disks, get_process_count, get_uptime};
use ui::{push_bar, C_RESET, C_CYAN, C_GREEN, C_YELLOW, C_MAGENTA, C_BLUE, C_BOLD};

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else {
        format!("{}m {}s", minutes, secs)
    }
}

fn main() -> io::Result<()> {
    print!("\x1B[?1049h\x1B[?25l\x1B[2J");
    let mut stdout = io::stdout();

    let sys_info = get_system_info();

    loop {
        let cpu_res = get_cpu_load();
        let mem_res = get_memory_usage();
        let disks_res = get_disks();
        let proc_count = get_process_count();
        let uptime = get_uptime().unwrap_or(0);

        let mut buffer = String::new();
        
        buffer.push_str("\x1B[H");
        
        let header_title = "RUST TOP DASHBOARD";
        let total_width = 63;
        let padding = (total_width - header_title.len()) / 2;
        
        writeln!(buffer, "{}{}╔{}╗{}", C_BOLD, C_CYAN, "═".repeat(total_width + 2), C_RESET).unwrap();
        writeln!(buffer, "{}{}║ {}{}{} ║{}", C_BOLD, C_CYAN, " ".repeat(padding), header_title, " ".repeat(total_width - header_title.len() - padding), C_RESET).unwrap();
        writeln!(buffer, "{}{}╚{}╝{}", C_BOLD, C_CYAN, "═".repeat(total_width + 2), C_RESET).unwrap();

        writeln!(buffer, " Host: {}{:<15}{} | OS: {}{}{} ", 
            C_GREEN, sys_info.host_name, C_RESET,
            C_GREEN, sys_info.os_name, C_RESET
        ).unwrap();
        writeln!(buffer, " Proc: {}{:<15}{} | Up: {}{}{} ", 
            C_YELLOW, proc_count.unwrap_or(0), C_RESET,
            C_GREEN, format_uptime(uptime), C_RESET
        ).unwrap();
        writeln!(buffer, "{}", "━".repeat(67)).unwrap();

        match cpu_res {
            Ok(load) => {
                push_bar(&mut buffer, "CPU", load as usize, C_YELLOW);
            }
            Err(_) => writeln!(buffer, "CPU     : Error").unwrap(),
        }

        match mem_res {
            Ok((used, total, percent)) => {
                push_bar(&mut buffer, "MEM", percent as usize, C_MAGENTA);
                writeln!(buffer, "          {:.2} GB / {:.2} GB", used, total).unwrap();
            }
            Err(_) => writeln!(buffer, "MEM     : Error").unwrap(),
        }

        writeln!(buffer, "{}", "━".repeat(67)).unwrap();

        match disks_res {
            Ok(disks) => {
                if disks.is_empty() {
                    writeln!(buffer, " No disks found").unwrap();
                } else {
                    for disk in disks {
                        let label = format!("DSK {}", disk.name);
                        push_bar(&mut buffer, &label, disk.percent as usize, C_BLUE);
                        writeln!(buffer, "          {:.2} GB / {:.2} GB", disk.total_gb - disk.free_gb, disk.total_gb).unwrap();
                    }
                }
            }
            Err(_) => writeln!(buffer, "DISK    : Error").unwrap(),
        }

        writeln!(buffer, "{}", "━".repeat(67)).unwrap();
        writeln!(buffer, " Press Ctrl+C to exit.").unwrap();

        print!("{}", buffer);
        stdout.flush()?;

        thread::sleep(Duration::from_millis(2000));
    }
}
