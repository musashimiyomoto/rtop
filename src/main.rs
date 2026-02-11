mod models;
mod sys;
mod ui;

use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use std::fmt::Write as FmtWrite;

use sys::{get_system_info, get_uptime, SysCollector};
use ui::{push_bar, C_RESET, C_CYAN, C_GREEN, C_YELLOW, C_MAGENTA, C_BOLD, C_RED};


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
    let mut collector = SysCollector::new();

    loop {
        collector.refresh();

        let cpu = collector.cpu_load();
        let (mem_used, mem_total, mem_pct) = collector.memory_usage();
        let proc_count = collector.process_count();
        let uptime = get_uptime();
        let top_procs = collector.top_processes(5);

        let mut buffer = String::new();
        buffer.push_str("\x1B[2J\x1B[H");

        let header_title = "RUST TOP DASHBOARD";
        let total_width = 70;
        let padding = (total_width - header_title.len()) / 2;

        writeln!(buffer, "{}{}╔{}╗{}", C_BOLD, C_CYAN, "═".repeat(total_width + 2), C_RESET).unwrap();
        writeln!(buffer, "{}{}║ {}{}{} ║{}", C_BOLD, C_CYAN, " ".repeat(padding), header_title, " ".repeat(total_width - header_title.len() - padding), C_RESET).unwrap();
        writeln!(buffer, "{}{}╚{}╝{}", C_BOLD, C_CYAN, "═".repeat(total_width + 2), C_RESET).unwrap();

        writeln!(buffer, " Host: {}{:<15}{} | OS: {}{}{}",
            C_GREEN, sys_info.host_name, C_RESET,
            C_GREEN, sys_info.os_name, C_RESET
        ).unwrap();
        writeln!(buffer, " Proc: {}{:<15}{} | Up: {}{}{}",
            C_YELLOW, proc_count, C_RESET,
            C_GREEN, format_uptime(uptime), C_RESET
        ).unwrap();
        writeln!(buffer, "{}", "━".repeat(total_width + 4)).unwrap();

        push_bar(&mut buffer, "CPU", cpu as usize, C_YELLOW);

        push_bar(&mut buffer, "MEM", mem_pct as usize, C_MAGENTA);
        writeln!(buffer, "          {:.2} GB / {:.2} GB", mem_used, mem_total).unwrap();

        writeln!(buffer, "{}", "━".repeat(total_width + 4)).unwrap();
        writeln!(buffer, " {}{}TOP PROCESSES (CPU){}{}", C_BOLD, C_RED, C_RESET, " ".repeat(total_width - 18)).unwrap();
        writeln!(buffer, " {:<8} {:<25} {:<10} {:<10}", "PID", "NAME", "CPU %", "MEM (MB)").unwrap();
        for proc in &top_procs {
            let name = if proc.name.len() > 25 { &proc.name[..25] } else { &proc.name };
            writeln!(buffer, " {:<8} {:<25} {:<10.1} {:<10.1}",
                proc.pid, name, proc.cpu_usage, proc.memory_mb
            ).unwrap();
        }

        writeln!(buffer, "{}", "━".repeat(total_width + 4)).unwrap();
        writeln!(buffer, " Press Ctrl+C to exit.").unwrap();

        print!("{}", buffer);
        stdout.flush()?;

        thread::sleep(Duration::from_millis(2000));
    }
}
