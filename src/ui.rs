use std::fmt::Write;

pub const C_RESET: &str = "\x1B[0m";
pub const C_CYAN: &str = "\x1B[36m";
pub const C_GREEN: &str = "\x1B[32m";
pub const C_YELLOW: &str = "\x1B[33m";
pub const C_MAGENTA: &str = "\x1B[35m";
pub const C_RED: &str = "\x1B[31m";
pub const C_BOLD: &str = "\x1B[1m";

pub fn format_uptime(seconds: u64) -> String {
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

pub fn push_bar(buffer: &mut String, label: &str, percent: usize, _default_color: &str) {
    let width = 40;
    let percent = if percent > 100 { 100 } else { percent };

    let color_code = if percent < 50 {
        C_GREEN
    } else if percent < 80 {
        C_YELLOW
    } else {
        C_RED
    };

    let filled = (percent * width) / 100;
    let empty = width - filled;

    let filled_str = "█".repeat(filled);
    let empty_str = "░".repeat(empty);

    writeln!(
        buffer,
        " {:<7}: [ {}{}{}{} ] {:>3}%",
        label, color_code, filled_str, C_RESET, empty_str, percent
    )
    .unwrap();
}

pub fn render_dashboard(buffer: &mut String, data: &crate::models::DashboardData) {
    let header_title = "RUST TOP DASHBOARD";
    let total_width = 70;
    let padding = (total_width - header_title.len()) / 2;

    writeln!(
        buffer,
        "{}{}╔{}╗{}",
        C_BOLD,
        C_CYAN,
        "═".repeat(total_width + 2),
        C_RESET
    )
    .unwrap();
    writeln!(
        buffer,
        "{}{}║ {}{}{} ║{}",
        C_BOLD,
        C_CYAN,
        " ".repeat(padding),
        header_title,
        " ".repeat(total_width - header_title.len() - padding),
        C_RESET
    )
    .unwrap();
    writeln!(
        buffer,
        "{}{}╚{}╝{}",
        C_BOLD,
        C_CYAN,
        "═".repeat(total_width + 2),
        C_RESET
    )
    .unwrap();

    writeln!(
        buffer,
        " Host: {}{:<15}{} | OS: {}{}{}",
        C_GREEN, data.host_name, C_RESET, C_GREEN, data.os_name, C_RESET
    )
    .unwrap();
    writeln!(
        buffer,
        " Proc: {}{:<15}{} | Up: {}{}{}",
        C_YELLOW,
        data.proc_count,
        C_RESET,
        C_GREEN,
        format_uptime(data.uptime),
        C_RESET
    )
    .unwrap();
    writeln!(buffer, "{}", "━".repeat(total_width + 4)).unwrap();

    push_bar(buffer, "CPU", data.cpu as usize, C_YELLOW);

    push_bar(buffer, "MEM", data.mem_pct as usize, C_MAGENTA);
    writeln!(
        buffer,
        "          {:.2} GB / {:.2} GB",
        data.mem_used, data.mem_total
    )
    .unwrap();

    writeln!(buffer, "{}", "━".repeat(total_width + 4)).unwrap();
    writeln!(
        buffer,
        " {}{}TOP PROCESSES (CPU){}{}",
        C_BOLD,
        C_RED,
        C_RESET,
        " ".repeat(total_width - 18)
    )
    .unwrap();
    writeln!(
        buffer,
        " {:<8} {:<25} {:<10} {:<10}",
        "PID", "NAME", "CPU %", "MEM (MB)"
    )
    .unwrap();
    for proc in data.top_procs {
        let name = if proc.name.len() > 25 {
            &proc.name[..25]
        } else {
            &proc.name
        };
        writeln!(
            buffer,
            " {:<8} {:<25} {:<10.1} {:<10.1}",
            proc.pid, name, proc.cpu_usage, proc.memory_mb
        )
        .unwrap();
    }

    writeln!(buffer, "{}", "━".repeat(total_width + 4)).unwrap();
    writeln!(buffer, " Press Ctrl+C to exit.").unwrap();
}
